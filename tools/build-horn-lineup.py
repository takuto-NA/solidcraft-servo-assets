#!/usr/bin/env python3
"""Build selected horn/actuator references and verify the real host's closed bodies.

Runs isolated solidcraft-cli sessions, never the user's live app. Catalog source
facts and documented mounts are retained in the plugin's per-instance metadata.
The helper checks bounds and ownership, not mechanical fit or Boolean volumes.
"""

import argparse
import hashlib
import itertools
import json
import math
import subprocess
import textwrap
from pathlib import Path


def invoke(cli, *args):
    result = subprocess.run([str(cli), *map(str, args)], capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(f"solidcraft-cli {args[0]} failed:\n{result.stderr[-8000:]}")
    return json.loads(result.stdout) if result.stdout.strip() else None


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def union(boxes):
    boxes = list(boxes)
    if not boxes:
        raise ValueError("Cannot bound an empty collection")
    return {key: [op(box[key][i] for box in boxes) for i in range(3)]
            for key, op in [("min", min), ("max", max)]}


def point(matrix, value, vector=False):
    return [sum(matrix[row * 4 + col] * value[col] for col in range(3))
            + (0 if vector else matrix[row * 4 + 3]) for row in range(3)]


def primitive_bounds(primitive, matrix=None):
    matrix = matrix or [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0]
    if primitive["kind"] == "box":
        corner, size = primitive["corner_mm"], primitive["size_mm"]
        corners = [point(matrix, [corner[i] + size[i] * side[i] for i in range(3)])
                   for side in itertools.product((0, 1), repeat=3)]
        return {key: [op(corner[i] for corner in corners) for i in range(3)]
                for key, op in [("min", min), ("max", max)]}
    if primitive["kind"] == "profile":
        candidates = []
        for segment in primitive["segments"]:
            points = [segment["start_mm"], segment["end_mm"]]
            if segment["kind"] == "arc":
                cx, cy = segment["center_mm"]
                sx, sy = segment["start_mm"]
                ex, ey = segment["end_mm"]
                radius = math.hypot(sx - cx, sy - cy)
                start, end = math.atan2(sy - cy, sx - cx), math.atan2(ey - cy, ex - cx)
                direction = 1 if segment["ccw"] else -1
                sweep = (direction * (end - start)) % math.tau
                for row in range(3):
                    theta = math.atan2(matrix[row * 4 + 1], matrix[row * 4])
                    for angle in (theta, theta + math.pi):
                        if (direction * (angle - start)) % math.tau <= sweep + 1e-12:
                            points.append([cx + radius * math.cos(angle), cy + radius * math.sin(angle)])
            for x, y in points:
                for z in (primitive["z_mm"], primitive["z_mm"] + primitive["height_mm"]):
                    candidates.append(point(matrix, [x, y, z]))
        return {key: [op(p[i] for p in candidates) for i in range(3)]
                for key, op in [("min", min), ("max", max)]}
    if primitive["kind"] != "cylinder":
        raise ValueError(f"Unsupported primitive: {primitive['kind']}")
    base = point(matrix, primitive["base_mm"])
    axis = point(matrix, primitive["axis"], vector=True)
    norm = math.hypot(*axis)
    if norm <= 0:
        raise ValueError("Cylinder axis is zero")
    direction = [value / norm for value in axis]
    end = [a + b * primitive["height_mm"] for a, b in zip(base, direction)]
    radial = [primitive["diameter_mm"] / 2 * math.sqrt(max(0, 1 - value * value))
              for value in direction]
    return {"min": [min(a, b) - r for a, b, r in zip(base, end, radial)],
            "max": [max(a, b) + r for a, b, r in zip(base, end, radial)]}


def part_bounds(part, matrix=None):
    # Current catalog cuts are internal holes/slots and preserve bounding extrema.
    # Transform each primitive first: rotating a cylinder's AABB overestimates it.
    return union(primitive_bounds(geometry, matrix) for geometry in
                 [part["geometry"]] + [modifier["geometry"]
                 for modifier in part.get("modifiers", []) if modifier["operation"] == "join"])


def translated(box, offset):
    return {key: [value + delta for value, delta in zip(box[key], offset)] for key in box}


def matches(actual, expected):
    return all(math.isfinite(actual[key][i]) and abs(actual[key][i] - expected[key][i]) < 1e-6
               for key in ("min", "max") for i in range(3))


def equivalent(actual, expected):
    """JSON number round trips can differ in their final binary decimal digit."""
    if isinstance(actual, dict) and isinstance(expected, dict):
        return actual.keys() == expected.keys() and all(equivalent(actual[key], expected[key]) for key in actual)
    if isinstance(actual, list) and isinstance(expected, list):
        return len(actual) == len(expected) and all(equivalent(a, b) for a, b in zip(actual, expected))
    if (isinstance(actual, (int, float)) and not isinstance(actual, bool)
            and isinstance(expected, (int, float)) and not isinstance(expected, bool)):
        return math.isclose(actual, expected, rel_tol=1e-14, abs_tol=1e-12)
    return actual == expected


def read_catalog(package):
    assets, horns, mounts = {}, {}, []
    for path in sorted((package / "catalog").glob("*.json")):
        pack = json.loads(path.read_text(encoding="utf-8"))
        for key, records in [("assets", assets), ("horns", horns)]:
            for record in pack.get(key, []):
                if record["id"] in records:
                    raise ValueError(f"Duplicate catalog identity: {record['id']}")
                records[record["id"]] = record
        mounts.extend(pack.get("mounts", []))
    return assets, horns, mounts


def package_fingerprint(package):
    manifest = json.loads((package / "plugin.json").read_text(encoding="utf-8"))
    module = (package / manifest["runtime"]["module"]).resolve()
    if not module.is_relative_to(package):
        raise ValueError("Plugin module must stay inside the package directory")
    paths = [package / "plugin.json", module] + sorted((package / "catalog").glob("*.json"))
    return {str(path.relative_to(package)): hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}


def select_horns(assets, horns, mounts, selected_ids):
    first = {}
    for mount in mounts:
        if (mount["compatibility"] == "documented" and mount["placement_basis"] == "documented"
                and mount.get("origin_mm") is not None and horns[mount["horn_id"]].get("parts")):
            first.setdefault(mount["horn_id"], mount)
    choices = []
    for horn_id in selected_ids or first:
        if horn_id not in horns or horn_id not in first:
            raise ValueError(f"No supported documented assembly for {horn_id}; use build-horn-references.py for detached dimensions or partial solids")
        mount = first[horn_id]
        if mount["asset_id"] not in assets:
            raise ValueError(f"Unknown bound actuator: {mount['asset_id']}")
        asset = assets[mount["asset_id"]]
        if not asset.get("reference_model"):
            raise ValueError(f"No integrated actuator model: {asset['id']}")
        choices.append((asset, horns[horn_id], mount))
    if not 1 <= len(choices) <= 64:
        raise ValueError("Choose 1..64 horn variants")
    return choices


def base_parts(asset, mount, offset):
    model = asset["reference_model"]
    replaced = set(mount.get("replace_model_parts", []))
    if not replaced.issubset({part["id"] for part in model["parts"]}):
        raise ValueError(f"Binding replaces an unknown part: {asset['id']}")
    parts = {part["id"]: translated(part_bounds(part), offset)
             for part in model["parts"] if part["id"] not in replaced}
    for cable in model.get("cables", []):
        count = len(cable["conductor_colors"])
        for index in range(count):
            base = list(cable["face_base_mm"])
            base[1] += (index - (count - 1) / 2) * cable["default_wire_pitch_mm"]
            base[2] += cable["default_outlet_height_mm"]
            wire = {"kind": "cylinder", "base_mm": base, "axis": [-1, 0, 0],
                    "diameter_mm": cable["default_wire_diameter_mm"],
                    "height_mm": cable["default_wire_length_mm"]}
            role = f"{cable['port_id']}-wire-{index + 1}"
            if role in parts:
                raise ValueError(f"Duplicate expected part role: {role}")
            parts[role] = translated(primitive_bounds(wire), offset)
    return parts


def validate_accessory(accessory, horn, mount, params):
    expected_horn = json.loads(json.dumps(horn))
    for hole in expected_horn["holes"]:
        hole.setdefault("thread", None)
        if hole["thread"] is not None:
            hole["thread"].setdefault("pitch_mm", None)
            hole["thread"].setdefault("specified_depth_mm", None)
    if accessory.get("kind") != "servo_horn" or not equivalent(accessory.get("horn"), expected_horn):
        raise ValueError("Built horn facts differ from the selected package catalog")
    expected_mount = dict(mount)
    expected_mount.setdefault("replace_model_parts", [])
    if not equivalent(accessory.get("mount"), expected_mount):
        raise ValueError("Built horn binding differs from the selected package catalog")
    if (not equivalent(accessory.get("angle_deg"), params["horn_angle_deg"])
            or not equivalent(accessory.get("axial_adjustment_mm"), params["horn_axial_offset_mm"])):
        raise ValueError("Host metadata disagrees with the requested user adjustment")
    matrix = accessory.get("world_from_horn", [])
    if len(matrix) != 12 or not all(math.isfinite(value) for value in matrix):
        raise ValueError("Missing or invalid horn placement matrix")
    axes = [point(matrix, [int(i == j) for i in range(3)], vector=True) for j in range(3)]
    dot = lambda a, b: sum(x * y for x, y in zip(a, b))
    if any(abs(dot(axes[i], axes[j]) - int(i == j)) > 1e-8 for i in range(3) for j in range(3)):
        raise ValueError("Horn placement matrix is not rigid")
    cross = [axes[0][1] * axes[1][2] - axes[0][2] * axes[1][1],
             axes[0][2] * axes[1][0] - axes[0][0] * axes[1][2],
             axes[0][0] * axes[1][1] - axes[0][1] * axes[1][0]]
    if abs(dot(cross, axes[2]) - 1) > 1e-8 or any(abs(a - b) > 1e-8 for a, b in zip(axes[2], mount["direction"])):
        raise ValueError("Horn orientation does not preserve the documented outward direction")
    offset = [params[key] for key in ("x_mm", "y_mm", "z_mm")]
    origin = [a + b + direction * params["horn_axial_offset_mm"]
              for a, b, direction in zip(mount["origin_mm"], offset, mount["direction"])]
    if any(abs(matrix[i * 4 + 3] - origin[i]) > 1e-8 for i in range(3)):
        raise ValueError("Horn origin does not follow its binding and outward adjustment")
    return matrix


def text_command(value, at, height):
    return {"command": "sketch.text", "params": {"text": value, "at": at, "height": height, "angle": 0}}


def label_lines(placement):
    accessory = placement["accessory"]
    mount = accessory["mount"]
    return [f"ACTUATOR {placement['model']}",
            f"HORN {accessory['horn']['model']} / {accessory['horn']['status'].upper().replace('_', ' ')}",
            f"FIT {mount['compatibility'].upper()} / POSE {mount['placement_basis'].upper()}",
            f"ANGLE {accessory['angle_deg']:g} deg / AXIAL {accessory['axial_adjustment_mm']:+g} mm"]


def make_labels(placements, bounds, view="front"):
    commands, names = [], []
    rows = sorted({placement["row"] for placement in placements})
    for row in rows:
        row_items = [placement for placement in placements if placement["row"] == row]
        # Two models per sketch leaves headroom for long manufacturer variant names.
        for start in range(0, len(row_items), 2):
            group = row_items[start:start + 2]
            name = f"Horn collection {view} labels row {row + 1} group {start // 2 + 1}"
            names.append(name)
            front_y = min(item["bbox_mm"]["min"][1] for item in row_items) - 4
            if view == "top":
                plane = {"origin": [0, 0, bounds["max"][2] + 2], "x_dir": [1, 0, 0], "y_dir": [0, 1, 0]}
            elif view == "bottom":
                # SolidCraft standard bottom view keeps screen-right +X, up -Y.
                plane = {"origin": [0, 0, bounds["min"][2] - 2], "x_dir": [1, 0, 0], "y_dir": [0, -1, 0]}
            else:
                plane = {"origin": [0, front_y, 0], "x_dir": [1, 0, 0], "y_dir": [0, 0, 1]}
            commands.append({"command": "sketch.create", "params": {"name": name, "frame": "world", "plane": plane}})
            for placement in group:
                x = placement["case_center_x"]
                first_y = (-(placement["bbox_mm"]["max"][1] + 7) if view == "bottom" else
                           placement["bbox_mm"]["min"][1] - 7 if view == "top" else -7)
                for index, line in enumerate(label_lines(placement)):
                    lines = textwrap.wrap(line, width=48, break_long_words=False) or [line]
                    height = 2.3 if index == 0 else 1.65
                    at_y = first_y - [0, 6, 15, 20][index]
                    for wrapped_index, wrapped in enumerate(lines):
                        commands.append(text_command(wrapped,
                            [x - len(wrapped) * height * .3, at_y - wrapped_index * height * 1.4], height))
            commands.append({"command": "sketch.finish", "params": {}})
    return commands, names


def verify_bodies(measured, expected, all_roles):
    bodies = {body["name"]: body for body in measured["bodies"]}
    owned = [body for roles in all_roles for body in roles.values()]
    count = sum(len(parts) for parts in expected)
    if measured["body_count"] != count or len(bodies) != count:
        raise ValueError(f"Expected {count} uniquely named bodies; host reported {measured['body_count']} bodies / {len(bodies)} names")
    if len(set(owned)) != count:
        raise ValueError("Different instances claim the same owned body name")
    if set(bodies) != set(owned):
        raise ValueError(f"Owned body names do not survive reopening: missing {sorted(set(owned) - set(bodies))[:5]}, unexpected {sorted(set(bodies) - set(owned))[:5]}")
    open_names = [name for name, body in bodies.items() if not body["closed"]]
    if open_names:
        raise ValueError(f"Open reference bodies: {open_names[:5]}")
    failed_features = [feature["name"] for feature in measured.get("timeline", []) if feature.get("error")]
    if failed_features:
        raise ValueError(f"Failed timeline features: {failed_features[:5]}")
    for parts, roles in zip(expected, all_roles):
        for role, box in parts.items():
            if not matches(bodies[roles[role]]["bbox"], box):
                raise ValueError(f"Unexpected exact bounds for role {role}: {bodies[roles[role]]['bbox']} versus {box}")
    bounds = union(body["bbox"] for body in bodies.values())
    if not matches(bounds, union(box for parts in expected for box in parts.values())):
        raise ValueError("Gallery bounds disagree with the analytic part bounds")
    return bodies, bounds


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cli", type=Path, required=True)
    parser.add_argument("--package", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--horn-id", action="append", help="Repeat to choose horns; default: one documented assembly per supported solid horn")
    parser.add_argument("--columns", type=int, default=4)
    parser.add_argument("--column-spacing", type=float, default=90)
    parser.add_argument("--row-spacing", type=float, default=90)
    parser.add_argument("--horn-angle-deg", type=float, default=0)
    parser.add_argument("--horn-axial-offset-mm", type=float, default=0)
    parser.add_argument("--reuse-build", action="store_true", help="Reuse saved base geometry only for an identical package fingerprint and parameters; ownership and bounds are still verified")
    args = parser.parse_args()
    if (not 1 <= args.columns <= 16 or not all(math.isfinite(value) and 40 <= value <= 1000
            for value in (args.column_spacing, args.row_spacing))):
        parser.error("Use 1..16 columns and finite spacing within 40..1000 mm")
    if (not math.isfinite(args.horn_angle_deg) or not -360 <= args.horn_angle_deg <= 360
            or not math.isfinite(args.horn_axial_offset_mm) or not -100 <= args.horn_axial_offset_mm <= 100):
        parser.error("Use an angle within +/-360 deg and axial adjustment within +/-100 mm")
    cli, package, out = args.cli.resolve(), args.package.resolve(), args.out.resolve()
    try:
        choices = select_horns(*read_catalog(package), args.horn_id)
        fingerprint = package_fingerprint(package)
    except (ValueError, KeyError) as error:
        parser.error(str(error))
    out.mkdir(parents=True, exist_ok=True)
    receipt_path = out / "verification.json"
    write_json(receipt_path, {"verified": False, "state": "building", "horn_count": len(choices)})
    approval = invoke(cli, "exec", "plugin.inspect", json.dumps({"path": str(package)}))["approval"]
    commands, placements, expected = [], [], []
    for index, (asset, horn, mount) in enumerate(choices):
        housing = part_bounds(next(part for part in asset["reference_model"]["parts"] if part["id"] == "housing"))
        row, column = divmod(index, args.columns)
        width = housing["max"][1] - housing["min"][1]
        case_x, row_y = column * args.column_spacing, row * args.row_spacing
        offset = [case_x - housing["min"][0], row_y - width / 2 - housing["min"][1], -housing["min"][2]]
        if any(abs(value) > 10000 for value in offset):
            parser.error("Gallery offset exceeds the plugin's 10000 mm coordinate limit; reduce spacing or rows")
        params = {"asset_id": asset["id"], "horn_id": horn["id"], "labels": "none",
                  "x_mm": offset[0], "y_mm": offset[1], "z_mm": offset[2],
                  "horn_angle_deg": args.horn_angle_deg, "horn_axial_offset_mm": args.horn_axial_offset_mm,
                  "include_cable": True, "include_connector": False}
        commands.append({"command": "plugin.run", "params": {"path": str(package), "approval": approval,
                         "entry": "servo-reference", "params": params}})
        expected.append(base_parts(asset, mount, offset))
        placements.append({"asset_id": asset["id"], "horn_id": horn["id"], "model": asset["model"],
                           "row": row, "column": column, "case_center_x": case_x + (housing["max"][0] - housing["min"][0]) / 2,
                           "row_y": row_y, "offset_mm": offset, "parameters": params,
                           "source_counts": {"base_parts": len(asset["reference_model"]["parts"]),
                                             "replaced_base_parts": len(mount.get("replace_model_parts", [])),
                                             "horn_parts": len(horn["parts"]),
                                             "fixed_wire_bodies": sum(len(cable["conductor_colors"])
                                                 for cable in asset["reference_model"].get("cables", []))}})
    build_script, base_design = out / "horn-lineup-build.json", out / "horn-lineup-base.sc"
    results_path = out / "horn-lineup-results.json"
    build_info_path = out / "horn-lineup-build-info.json"
    build_info = {"package_inputs": fingerprint,
                  "parameters": [command["params"]["params"] for command in commands]}
    write_json(build_script, commands)
    if args.reuse_build:
        if not all(path.is_file() for path in (base_design, results_path, build_info_path)):
            raise ValueError("--reuse-build needs the saved base design, results and build-info fingerprint in --out")
        if json.loads(build_info_path.read_text(encoding="utf-8")) != build_info:
            raise ValueError("--reuse-build refuses a different package or geometry parameters; rerun without --reuse-build")
    else:
        invoke(cli, "run", build_script, "--save", base_design, "--results", "--json-out", results_path)
        if package_fingerprint(package) != fingerprint:
            raise ValueError("Package changed during the host build; retain the evidence and rerun with a stable package")
        write_json(build_info_path, build_info)
    replies = json.loads(results_path.read_text(encoding="utf-8"))
    if len(replies) != len(choices):
        raise ValueError("Host returned a different number of plugin results")
    all_roles = []
    for (asset, horn, mount), placement, parts, reply in zip(choices, placements, expected, replies):
        result = reply["result"]
        accessories = result.get("accessories", [])
        if len(accessories) != 1 or result.get("asset_id") != asset["id"] or result.get("data_revision") != asset["data_revision"]:
            raise ValueError("Missing horn metadata or wrong built actuator profile")
        accessory = accessories[0]
        matrix = validate_accessory(accessory, horn, mount, placement["parameters"])
        base_bbox = union(parts.values())
        for part in horn["parts"]:
            role = f"horn-{part['id']}"
            if role in parts:
                raise ValueError(f"Horn role collides with an actuator part: {role}")
            parts[role] = part_bounds(part, matrix)
        roles = result["body_roles"]
        if (set(roles) != set(parts) or len(result["bodies"]) != len(parts)
                or set(result["bodies"]) != set(roles.values())):
            raise ValueError(f"Missing owned role or unexpected connector: {horn['id']}")
        all_roles.append(roles)
        placement.update({"accessory": accessory, "body_roles": roles, "body_count": len(parts),
                          "base_bbox_mm": base_bbox, "bbox_mm": union(parts.values()),
                          "horn_bbox_mm": union(box for role, box in parts.items() if role.startswith("horn-"))})
    measured = invoke(cli, "eval", base_design)
    write_json(out / "base-measurements.json", measured)
    _, bounds = verify_bodies(measured, expected, all_roles)
    labels, label_names = make_labels(placements, bounds)
    label_script, design = out / "front-labels.json", out / "horn-lineup.sc"
    write_json(label_script, labels)
    write_json(out / "horn-lineup.json", commands + labels)
    invoke(cli, "run", label_script, "--in", base_design, "--save", design,
           "--results", "--json-out", out / "horn-lineup-label-results.json")
    # Labels are actual CAD sketches; verify adding them did not alter solid ownership.
    final_measurements = invoke(cli, "eval", design)
    write_json(out / "measurements.json", final_measurements)
    _, bounds = verify_bodies(final_measurements, expected, all_roles)
    top_labels, top_label_names = make_labels(placements, bounds, view="top")
    top_commands = [{"command": "browser.visibility", "params": {"items": label_names, "visible": False}}] + top_labels
    write_json(out / "top-view.json", top_commands)
    bottom_labels, bottom_label_names = make_labels(placements, bounds, view="bottom")
    bottom_commands = [{"command": "browser.visibility", "params": {"items": label_names, "visible": False}}] + bottom_labels
    write_json(out / "bottom-view.json", bottom_commands)
    receipt = {"verified": True, "plugin_version": json.loads((package / "plugin.json").read_text(encoding="utf-8"))["version"],
               "horn_count": len(choices), "body_count": measured["body_count"], "all_closed": True,
               "owned_role_sets_match": True, "base_bounds_match": True, "transformed_horn_bounds_match": True,
               "bounds_method": "Analytic box corners and cylinder radial extents; union additive primitives; current internal cuts preserve extrema. No exact Boolean volume is asserted.",
               "loose_connector_count": 0, "case_base_z_mm": 0,
               "columns": args.columns, "column_spacing_mm": args.column_spacing, "row_spacing_mm": args.row_spacing,
               "bbox_mm": bounds, "front_label_sketches": label_names,
               "top_label_sketches": top_label_names, "bottom_label_sketches": bottom_label_names,
               "view_scripts_input": str(design), "placements": placements}
    receipt["package_inputs"] = fingerprint
    write_json(receipt_path, receipt)
    print(json.dumps({"design": str(design), "horn_count": len(choices), "body_count": measured["body_count"],
                      "bbox_mm": bounds, "verified": True}, indent=2))


if __name__ == "__main__":
    try:
        main()
    except (OSError, RuntimeError, ValueError) as error:
        raise SystemExit(str(error)) from error
