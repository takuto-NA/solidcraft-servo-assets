# Instructions for Servo Assets agents

This repository is the standalone Servo Assets plugin for SolidCraft. Read
[CONTRIBUTING.md](CONTRIBUTING.md), [README.md](README.md) and the relevant source
audit before editing. It is not the SolidCraft application workspace. Keep the
work focused and preserve unrelated changes.

## Runtime and checks

- Runtime application logic is Rust, builds for WebAssembly and uses documented
  SolidCraft commands. Python is allowed for development/data/evidence helpers.
- Validate input before issuing commands. Non-test Rust must not add `unsafe`,
  `unwrap`, `expect` or panics. Return an error for unsupported input.
- Only modify bodies and sketches created by the current operation. Profile
  extrusion must use its owned sketch ID; keep target roles and saved body names.
- Run `cargo xtask ci` before committing implementation or catalog changes.
  Set `SOLIDCRAFT_CLI` to a compatible CLI to include host validation when
  available. Report omitted checks honestly; do not invent a passing result.
- Put Rust regressions in existing test modules and host fixtures in `tests/`.
  Use numeric/source checks plus actual host insertion for geometry changes.
- Capture and inspect actual CAD screenshots for observable model/UI changes.
  Include the parameters and necessary front/rear/output/cable views.
- Do not commit build directories, generated archives, downloaded vendor media,
  credentials or private evidence. Publish or tag a release only when requested.

## Catalog evidence

- Stable IDs identify a source/model/variant. Keep manufacturer, model, seller
  SKU, document revision and physical revision separate. Do not silently replace
  another variant's facts. Increment the relevant data revision for corrections.
- Every factual dimension and formal hole/axis needs per-field evidence. Preserve
  conflicts. Unknown is null or an explicit missing field, never an invented zero.
- Grounded photo estimates are allowed and useful. Separate `reported_*`
  annotations from `estimates`, record method, calibration, pixel anchors where
  available and unknown uncertainty. Do not promote inferred geometry to facts.
- Keep `dimensions_only`, source partial solids and `photo_estimate` distinct.
  Use `photo-solid` explicitly for inferred shapes. Read
  [docs/horn-data-schema.md](docs/horn-data-schema.md) before editing horns.
- Formal `holes` are source-located datums. Photo-inferred cuts belong in the
  estimate-backed recipe, not the formal mounting pattern.
- Frames and axes are explicit. A box corner, axis reference plane, shaft tip and
  horn seat are different datums. Never register them silently.
- Distinguish nominal thread size, pilot/clearance diameter, pitch, maximum depth
  and usable engagement. A maximum-depth label does not establish screw length.
- Cable entrance, bundle/conductor count, socket/contact count and connected
  harness count are separate facts. Do not add a pigtail to a detachable socket.
- Horn compatibility and installed pose have separate evidence. Unknown pose has
  null origin and no replacement roles. Photo estimates do not auto-assemble.
- Keep source snapshots reproducible. Updates must not rewrite saved instances.
  Do not describe partial geometry as printable or a guaranteed clearance bound.

## Independence and contribution flow

- All code and documentation are English. Original contributions use
  MIT OR Apache-2.0; preserve attribution and existing license notices.
- Do not read, copy or use GPL/LGPL/AGPL implementations or FreeCAD-derived
  geometry. Do not drive or extract code, geometry or measurements from
  commercial CAD products. Public manufacturer facts and original analytic
  construction are the basis for this catalog.
- Keep vendor PDFs, photographs, artwork and CAD files external. Store source
  URLs, locators, numeric facts and original explanations. Do not copy product
  prose or another project's templates.
- Check related issues/PRs, then use a normal fork, branch and focused PR. No
  draft claim, CLA or additional public approval process is required to start.
- Do not add dependencies, runtime permissions or distribution changes as a
  side effect of a data-only addition. Explain any required scope change.

For new data, start with [docs/adding-assets.md](docs/adding-assets.md).
