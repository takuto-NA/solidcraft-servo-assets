#!/usr/bin/env python3
"""Build detached horn source references in isolated solidcraft-cli sessions.

Dimensions are the default; source-solid includes only supported partial solids.
Neither mode infers an installed servo position or certifies a mating fit.
"""

import argparse
import copy
import importlib.util
import json
import math
import re
import sys
from pathlib import Path


sys.dont_write_bytecode = True
_spec = importlib.util.spec_from_file_location(
    "horn_lineup_helpers", Path(__file__).with_name("build-horn-lineup.py"))
if _spec is None or _spec.loader is None:
    raise RuntimeError("Cannot load the adjacent horn lineup helpers")
helpers = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(helpers)
invoke = helpers.invoke
read_catalog = helpers.read_catalog
part_bounds = helpers.part_bounds
matches = helpers.matches
package_fingerprint = helpers.package_fingerprint
write_json = helpers.write_json


def select_horns(horns, mode, selected_ids):
    ids = selected_ids if selected_ids is not None else [
        key for key, horn in horns.items() if mode == "dimensions"
        or mode == "source-solid" and horn["parts"] and horn["status"] != "photo_estimate"
        or mode == "photo-solid" and horn["status"] == "photo_estimate"]
    if not 1 <= len(ids) <= 128 or len(set(ids)) != len(ids):
        raise ValueError("Choose 1..128 distinct horn source IDs")
    selected = []
    for key in ids:
        if key not in horns:
            raise ValueError(f"Unknown horn source ID: {key}")
        horn = horns[key]
        if mode == "source-solid" and not horn["parts"]:
            raise ValueError(f"No supported partial solid for {key}; use --mode dimensions")
        if mode == "source-solid" and horn["status"] == "photo_estimate":
            raise ValueError(f"{key} includes photo inference; use --mode photo-solid")
        if mode == "photo-solid" and horn["status"] != "photo_estimate":
            raise ValueError(f"{key} is not a photo-estimated variant")
        selected.append(horn)
    return selected


def source_ids(horn):
    ids = {value["evidence"]["source"] for value in horn["dimensions"].values()}
    ids.update(value["evidence"]["source"] for value in horn.get("estimates", {}).values())
    ids.update(observation["source"] for observation in horn["observations"])
    for hole in horn["holes"]:
        ids.update((hole["evidence"]["source"], hole["diameter"]["evidence"]["source"]))
        for key in ("pitch_mm", "specified_depth_mm"):
            value = (hole.get("thread") or {}).get(key)
            if value is not None:
                ids.add(value["evidence"]["source"])
    return ids


def normalized_horn(horn):
    horn = copy.deepcopy(horn)
    for hole in horn["holes"]:
        hole.setdefault("thread", None)
        if hole["thread"] is not None:
            hole["thread"].setdefault("pitch_mm", None)
            hole["thread"].setdefault("specified_depth_mm", None)
    return horn


def check_snapshot(result, horn, params, sources):
    snapshot = result.get("source_snapshot")
    if snapshot is None:
        if result.get("metadata_saved_in_design") is not True or not result.get("snapshot"):
            raise ValueError("Neither a persisted snapshot nor returned source snapshot is available")
        return "persisted_in_design_not_read_by_helper"
    if (snapshot.get("asset_kind") != "servo_horn"
            or snapshot.get("representation") != "horn-reference"
            or snapshot.get("accessories") != []
            or not helpers.equivalent(snapshot.get("asset"), normalized_horn(horn))
            or not helpers.equivalent(snapshot.get("parameters"), params)):
        raise ValueError(f"Detached source snapshot differs from the selected horn facts: {horn['id']}")
    expected_basis = ("source_dimension_references_only" if params["mode"] == "dimensions"
                      else "photo_reconstructed_estimated_geometry" if params["mode"] == "photo-solid"
                      else "source_partial_outer_profile")
    if snapshot.get("geometry_basis") != expected_basis:
        raise ValueError("Source snapshot overstates or misidentifies the reference geometry")
    used_sources = {source["id"]: source for source in snapshot.get("sources", [])}
    if set(used_sources) != source_ids(horn) or any(
            not helpers.equivalent(source, sources.get(key)) for key, source in used_sources.items()):
        raise ValueError("Source snapshot citations differ from the selected package sources")
    return "returned_snapshot_facts_verified"


def verify_geometry(measured, placements, mode):
    bodies = {body["name"]: body for body in measured["bodies"]}
    expected_count = sum(len(placement["expected_parts"]) for placement in placements)
    owned = [name for placement in placements for name in placement["body_roles"].values()]
    if (measured["body_count"] != expected_count or len(bodies) != expected_count
            or len(set(owned)) != expected_count or set(bodies) != set(owned)):
        raise ValueError("Detached gallery body ownership or names changed after saving/reopening")
    if mode == "dimensions" and (bodies or owned):
        raise ValueError("Dimension references unexpectedly created solid bodies")
    if any(re.fullmatch(r"Body\d+", name) for name in owned):
        raise ValueError("An owned body still uses an unpersisted automatic BodyN alias")
    if any(not body["closed"] for body in bodies.values()):
        raise ValueError("A partial source body is not closed")
    if any(feature.get("error") for feature in measured.get("timeline", [])):
        raise ValueError("A gallery timeline feature failed during reopening")
    for placement in placements:
        for role, expected in placement["expected_parts"].items():
            actual = bodies[placement["body_roles"][role]]["bbox"]
            if not matches(actual, expected):
                raise ValueError(f"Exact body bounds differ from source primitives: {placement['horn_id']} / {role}")
    bounds = helpers.union(body["bbox"] for body in bodies.values()) if bodies else None
    if bounds is not None and not matches(bounds, helpers.union(
            box for placement in placements for box in placement["expected_parts"].values())):
        raise ValueError("Gallery bounds differ from the analytic primitive bounds")
    return bounds


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cli", type=Path, required=True)
    parser.add_argument("--package", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--mode", choices=("dimensions", "source-solid", "photo-solid"), default="dimensions")
    parser.add_argument("--horn-id", action="append", help="Repeat to choose source IDs; default: all supported by the mode")
    parser.add_argument("--columns", type=int, default=4)
    parser.add_argument("--column-spacing", type=float, default=80)
    parser.add_argument("--row-spacing", type=float, default=80)
    args = parser.parse_args()
    if (not 1 <= args.columns <= 16 or not all(math.isfinite(value) and 40 <= value <= 1000
            for value in (args.column_spacing, args.row_spacing))):
        parser.error("Use 1..16 columns and finite spacing within 40..1000 mm")
    cli, package, out = args.cli.resolve(), args.package.resolve(), args.out.resolve()
    if out.is_relative_to(package):
        parser.error("Keep --out outside the plugin package")
    _, horns, _ = read_catalog(package)
    selected = select_horns(horns, args.mode, args.horn_id)
    fingerprint = package_fingerprint(package)
    sources = {}
    for path in sorted((package / "catalog").glob("horns-*.json")):
        for source in json.loads(path.read_text(encoding="utf-8"))["sources"]:
            if source["id"] in sources:
                raise ValueError(f"Duplicate horn source identity: {source['id']}")
            sources[source["id"]] = source
    placements = []
    for index, horn in enumerate(selected):
        row, column = divmod(index, args.columns)
        x, y = column * args.column_spacing, row * args.row_spacing
        if max(abs(x), abs(y)) > 100000:
            parser.error("Gallery exceeds the horn-reference coordinate limit; reduce spacing or rows")
        params = {"horn_id": horn["id"], "mode": args.mode, "x_mm": x, "y_mm": y,
                  "z_mm": 0, "angle_deg": 0}
        matrix = [1, 0, 0, x, 0, 1, 0, y, 0, 0, 1, 0]
        parts = ({f"horn-{part['id']}": part_bounds(part, matrix) for part in horn["parts"]}
                 if args.mode in ("source-solid", "photo-solid") else {})
        placements.append({"horn_id": horn["id"], "model": horn["model"], "row": row,
                           "column": column, "parameters": params, "expected_parts": parts})
    out.mkdir(parents=True, exist_ok=True)
    receipt_path = out / "verification.json"
    write_json(receipt_path, {"verified": False, "state": "building", "mode": args.mode,
                              "horn_count": len(selected)})
    approval = invoke(cli, "exec", "plugin.inspect", json.dumps({"path": str(package)}))["approval"]
    commands = [{"command": "plugin.run", "params": {"path": str(package), "approval": approval,
                "entry": "horn-reference", "params": placement["parameters"]}} for placement in placements]
    script, design = out / "horn-references.json", out / "horn-references.sc"
    results_path = out / "horn-references-results.json"
    write_json(script, commands)
    invoke(cli, "run", script, "--save", design, "--results", "--json-out", results_path)
    if package_fingerprint(package) != fingerprint:
        raise ValueError("Plugin package changed during the build; rerun with a stable package")
    replies = json.loads(results_path.read_text(encoding="utf-8"))
    if len(replies) != len(selected):
        raise ValueError("Host returned a different number of horn reference results")
    for horn, placement, reply in zip(selected, placements, replies):
        result = reply["result"]
        if (result.get("asset_id") != horn["id"] or result.get("data_revision") != horn["data_revision"]
                or result.get("representation") != "horn-reference" or result.get("accessories") != []):
            raise ValueError("Host returned an attached accessory or a different canonical source identity")
        roles = result["body_roles"]
        if (set(roles) != set(placement["expected_parts"])
                or set(result["bodies"]) != set(roles.values()) or len(result["bodies"]) != len(roles)):
            raise ValueError("Source reference created missing, extra or unowned bodies")
        placement["body_roles"] = roles
        placement["snapshot_check"] = check_snapshot(result, horn, placement["parameters"], sources)
        placement["snapshot"] = result.get("snapshot")
    measured = invoke(cli, "eval", design)
    write_json(out / "measurements.json", measured)
    bounds = verify_geometry(measured, placements, args.mode)
    if package_fingerprint(package) != fingerprint:
        raise ValueError("Plugin package changed during verification")
    receipt = {"verified": True, "kind": "detached_horn_references", "mode": args.mode,
               "design": str(design), "horn_count": len(selected), "body_count": measured["body_count"],
               "automatic_assembly": False, "owned_names_persisted": True,
               "partial_solids_closed": True, "analytic_bounds_match": True, "bbox_mm": bounds,
               "bounds_method": "Analytic cylinder/box extents and additive union; current internal cuts preserve extrema. No Boolean-volume or fit claim.",
               "snapshot_checks": sorted({placement["snapshot_check"] for placement in placements}),
               "columns": args.columns, "column_spacing_mm": args.column_spacing,
               "row_spacing_mm": args.row_spacing, "placements": placements, "package_inputs": fingerprint}
    write_json(receipt_path, receipt)
    print(json.dumps({key: receipt[key] for key in ("design", "mode", "horn_count", "body_count", "bbox_mm", "verified")}, indent=2))


if __name__ == "__main__":
    try:
        main()
    except (OSError, RuntimeError, ValueError, KeyError, TypeError) as error:
        raise SystemExit(str(error)) from error
