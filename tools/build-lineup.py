#!/usr/bin/env python3
"""Build a same-scale comparison and verify exact host bounds for all selected servos."""

import argparse
import json
import math
import subprocess
from pathlib import Path


def invoke(cli, *args):
    result = subprocess.run([str(cli), *map(str, args)], capture_output=True, text=True, check=True)
    return json.loads(result.stdout) if result.stdout.strip() else None


def union(bounds):
    return {key: [op(box[key][i] for box in bounds) for i in range(3)] for key, op in [("min", min), ("max", max)]}


def primitive_bounds(primitive):
    if primitive["kind"] == "box":
        corner, size = primitive["corner_mm"], primitive["size_mm"]
        return {"min": corner, "max": [a + b for a, b in zip(corner, size)]}
    base, axis = primitive["base_mm"], primitive["axis"]
    norm = math.hypot(*axis)
    direction = [value / norm for value in axis]
    end = [a + b * primitive["height_mm"] for a, b in zip(base, direction)]
    radial = [primitive["diameter_mm"] / 2 * math.sqrt(max(0, 1 - value * value)) for value in direction]
    return {"min": [min(a, b) - r for a, b, r in zip(base, end, radial)], "max": [max(a, b) + r for a, b, r in zip(base, end, radial)]}


def part_bounds(part):
    return union([primitive_bounds(part["geometry"])] + [primitive_bounds(modifier["geometry"]) for modifier in part["modifiers"] if modifier["operation"] == "join"])


def translated(box, offset):
    return {key: [value + delta for value, delta in zip(box[key], offset)] for key in box}


def matches(actual, expected):
    return all(abs(actual[key][i] - expected[key][i]) < 1e-6 for key in expected for i in range(3))


def basis(asset):
    frame = asset["reference_model"]["frame"]
    mounts = any(group["frame"] == frame for group in asset["mount_groups"])
    axis = any(output.get("axis") and output["axis"]["frame"] == frame for output in asset["output_interfaces"])
    if mounts and axis:
        return "MOUNT + AXIS / SIMPLIFIED"
    if mounts or axis:
        return "PARTIAL DATUMS"
    return "PREVIEW POSITIONS"


def text(value, at, height):
    return {"command": "sketch.text", "params": {"text": value, "at": at, "height": height, "angle": 0}}


def make_labels(placements, row_count, row_spacing, column_spacing, columns, max_height, top=False):
    front_y = -max(placement["width"] / 2 for placement in placements) - 3
    commands = []
    if top:
        # Keep text outlines below the host's ordinary per-sketch point/curve limit.
        for start in range(0, len(placements), 4):
            commands.append({"command": "sketch.create", "params": {"name": f"Servo collection top labels group {start // 4 + 1}", "frame": "world", "plane": {"origin": [0, 0, max_height + 2], "x_dir": [1, 0, 0], "y_dir": [0, 1, 0]}}})
            for placement in placements[start:start + 4]:
                x, y = placement["case_center_x"], placement["row_y"]
                name = placement["model"]
                name_height = min(4.0, 60.0 / max(1, len(name)) / 0.65)
                label_y = y - max(placement["width"] / 2, 11) - 6
                commands.append(text(name, [x - len(name) * name_height * 0.3, label_y], name_height))
                commands.append(text(placement["basis"], [x - 16, label_y - 5], 1.8))
            commands.append({"command": "sketch.finish", "params": {}})
        commands.append({"command": "sketch.create", "params": {"name": "Servo collection top heading", "frame": "world", "plane": {"origin": [0, 0, max_height + 2], "x_dir": [1, 0, 0], "y_dir": [0, 1, 0]}}})
        commands.append(text("TOP VIEW - MOUNTS AND SHAFT CENTERS", [25, (row_count - 1) * row_spacing + 24], 3.5))
        caption = "Cable markers hidden; amber parts contain planning assumptions" if any(placement.get("has_fixed_lead", True) for placement in placements) else "Amber: planning assumptions / sockets have no attached cables"
        commands.append(text(caption, [25, -32], 2.5))
        commands.append({"command": "sketch.finish", "params": {}})
        return commands
    for row in range(row_count):
        commands.append({"command": "sketch.create", "params": {"name": f"Servo collection labels row {row + 1}", "frame": "world", "plane": {"origin": [0, row * row_spacing + front_y, 0], "x_dir": [1, 0, 0], "y_dir": [0, 0, 1]}}})
        for placement in (placement for placement in placements if placement["row"] == row):
            x = placement["case_center_x"]
            name = placement["model"]
            name_height = min(5.0, 60.0 / max(1, len(name)) / 0.65)
            commands.append(text(name, [x - len(name) * name_height * 0.3, -7], name_height))
            commands.append(text(placement["basis"], [x - 17, -12], 2.1))
        if row == 0:
            caption = "Short lead markers / no loose plugs" if any(placement.get("has_fixed_lead", True) for placement in placements) else "Detachable bus sockets / no attached harnesses"
            commands.append(text(caption, [25, -22], 3))
            commands.append(text("Amber: planning geometry. Contours and cable exits are simplified.", [25, -28], 2.2))
        commands.append({"command": "sketch.finish", "params": {}})
    commands.append({"command": "sketch.create", "params": {"name": "Servo collection heading", "frame": "world", "plane": {"origin": [0, (row_count - 1) * row_spacing + 30, 0], "x_dir": [1, 0, 0], "y_dir": [0, 0, 1]}}})
    title = f"{len(placements)} SERVO REFERENCES - SAME SCALE"
    commands.append(text(title, [(min(columns, len(placements)) - 1) * column_spacing / 2 - 50, max_height + 5], 4))
    commands.append({"command": "sketch.finish", "params": {}})
    return commands


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cli", type=Path, required=True)
    parser.add_argument("--package", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--asset-id", action="append", help="Repeat to choose profiles; default: all integrated models sorted by case length")
    parser.add_argument("--columns", type=int, default=5)
    parser.add_argument("--column-spacing", type=float, default=75)
    parser.add_argument("--row-spacing", type=float, default=85)
    args = parser.parse_args()
    if not 1 <= args.columns <= 16 or not all(math.isfinite(value) and value >= 40 for value in [args.column_spacing, args.row_spacing]):
        parser.error("Use 1..16 columns and finite spacing of at least 40 mm")
    cli, package, out = args.cli.resolve(), args.package.resolve(), args.out.resolve()
    out.mkdir(parents=True, exist_ok=True)
    # Read the packaged facts, so the verification always matches the selected build.
    assets = [asset for path in sorted((package / "catalog").glob("*.json")) for asset in json.loads(path.read_text()).get("assets", [])]
    if args.asset_id:
        selected = [next(asset for asset in assets if asset["id"] == asset_id) for asset_id in args.asset_id]
    else:
        selected = [asset for asset in assets if asset.get("reference_model")]
        selected.sort(key=lambda asset: (part_bounds(next(part for part in asset["reference_model"]["parts"] if part["id"] == "housing"))["max"][0] - part_bounds(next(part for part in asset["reference_model"]["parts"] if part["id"] == "housing"))["min"][0], asset["model"]))
    if not selected or len(selected) > 64:
        parser.error("Choose 1..64 integrated profiles")
    approval = invoke(cli, "exec", "plugin.inspect", json.dumps({"path": str(package)}))["approval"]
    placements, commands, expected_parts = [], [], []
    for index, asset in enumerate(selected):
        model = asset["reference_model"]
        housing = part_bounds(next(part for part in model["parts"] if part["id"] == "housing"))
        width = housing["max"][1] - housing["min"][1]
        row, column = divmod(index, args.columns)
        case_x, row_y = column * args.column_spacing, row * args.row_spacing
        offset = [case_x - housing["min"][0], row_y - width / 2 - housing["min"][1], -housing["min"][2]]
        params = {"asset_id": asset["id"], "x_mm": offset[0], "y_mm": offset[1], "z_mm": offset[2], "labels": "none"}
        commands.append({"command": "plugin.run", "params": {"path": str(package), "approval": approval, "entry": "servo-reference", "params": params}})
        expected = {part["id"]: translated(part_bounds(part), offset) for part in model["parts"]}
        for cable in model["cables"]:
            count = len(cable["conductor_colors"])
            for wire in range(count):
                base = list(cable["face_base_mm"])
                base[1] += (wire - (count - 1) / 2) * cable["default_wire_pitch_mm"]
                base[2] += cable["default_outlet_height_mm"]
                primitive = {"kind": "cylinder", "base_mm": base, "axis": [-1, 0, 0], "diameter_mm": cable["default_wire_diameter_mm"], "height_mm": cable["default_wire_length_mm"]}
                expected[f"{cable['port_id']}-wire-{wire + 1}"] = translated(primitive_bounds(primitive), offset)
        expected_parts.append(expected)
        placements.append({"asset_id": asset["id"], "model": asset["model"], "row": row, "column": column, "case_x": case_x, "case_center_x": case_x + (housing["max"][0] - housing["min"][0]) / 2, "row_y": row_y, "width": width, "offset_mm": offset, "basis": basis(asset), "has_fixed_lead": bool(model["cables"])})
    expected_bounds = union([box for parts in expected_parts for box in parts.values()])
    row_count = math.ceil(len(selected) / args.columns)
    max_height = expected_bounds["max"][2]
    commands.extend(make_labels(placements, row_count, args.row_spacing, args.column_spacing, args.columns, max_height))
    script, design, results = (out / name for name in ["servo-lineup.json", "servo-lineup.sc", "servo-lineup-results.json"])
    script.write_text(json.dumps(commands, indent=2) + "\n")
    subprocess.run([str(cli), "run", str(script), "--save", str(design), "--results", "--json-out", str(results)], capture_output=True, text=True, check=True)
    replies = json.loads(results.read_text())
    measured = invoke(cli, "eval", design)
    expected_count = sum(len(parts) for parts in expected_parts)
    if measured["body_count"] != expected_count or not all(body["closed"] for body in measured["bodies"]):
        raise ValueError("Unexpected body count or open reference body")
    bodies = {body["name"]: body for body in measured["bodies"]}
    wire_names = []
    for placement, expected, reply in zip(placements, expected_parts, replies):
        roles = reply["result"]["body_roles"]
        if set(roles) != set(expected):
            raise ValueError("Missing part role or an unexpected connector")
        for role, bounds in expected.items():
            if not matches(bodies[roles[role]]["bbox"], bounds):
                raise ValueError(f"Unexpected exact bounds: {placement['model']} / {role}")
            if "-wire-" in role:
                wire_names.append(roles[role])
                case = bodies[roles["housing"]]["bbox"]
                if abs(bounds["max"][0] - case["min"][0]) > 1e-6 or bounds["max"][2] >= bodies[roles["near-ear"]]["bbox"]["min"][2]:
                    raise ValueError("A cable marker enters its case or mounting ear")
    bounds = union([body["bbox"] for body in measured["bodies"]])
    if not matches(bounds, expected_bounds):
        raise ValueError("Collection bounds disagree with the analytical primitive bounds")
    hidden_labels = [f"Servo collection labels row {row + 1}" for row in range(row_count)] + ["Servo collection heading"]
    top_commands = [{"command": "browser.visibility", "params": {"items": hidden_labels + wire_names, "visible": False}}]
    top_commands.extend(make_labels(placements, row_count, args.row_spacing, args.column_spacing, args.columns, max_height, top=True))
    (out / "top-view.json").write_text(json.dumps(top_commands, indent=2) + "\n")
    ports = [(port, asset["reference_model"]["parts"]) for asset in selected for port in asset["cable_ports"] if port["attachment"] in {"detachable", "detachable_socket"}]
    sockets = sum(any(part["id"] == port["id"] + "-socket" for part in parts) for port, parts in ports)
    receipt = {"plugin_version": json.loads((package / "plugin.json").read_text())["version"], "model_count": len(selected), "body_count": measured["body_count"], "all_closed": True, "loose_plug_count": 0, "detachable_socket_count": sockets, "source_only_port_count": len(ports) - sockets, "outlet_count": len(wire_names), "outlets_stop_at_case_face": True, "outlets_below_mounts": True, "case_base_z_mm": 0, "columns": args.columns, "column_spacing_mm": args.column_spacing, "row_spacing_mm": args.row_spacing, "bbox_mm": bounds, "placements": placements}
    (out / "verification.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({"design": str(design), "model_count": len(selected), "body_count": measured["body_count"], "bbox_mm": bounds, "verified": True}, indent=2))


if __name__ == "__main__":
    main()
