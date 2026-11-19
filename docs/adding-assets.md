# Adding a servo or horn

You can add many catalog variants by editing JSON and a source note. Start in an
existing pack with a nearby record's structure; do not inherit that record's
dimensions or compatibility. You can also open an asset issue with a source link
and ask for help encoding it.

## 1. Identify the hardware and source

Record the manufacturer and exact printed model. Include seller SKU, document
title/revision/date and specimen markings when available. Unknown manufacturer,
factory revision or batch must remain unknown. Keep different vendors, tooling
versions, horns and observed specimens separate even when they share a model name.

Add a unique source record with publisher, title, HTTPS URL and retrieval date.
For each fact, include the source ID, page/figure/callout, method and an original
interpretation note. Link vendor media rather than adding the PDF or image to
the repository. For your own measurements, describe specimen identity, datum,
instrument/method and uncertainty. A source date is not a physical revision.

| Evidence | How to record it |
| --- | --- |
| Explicit drawing/specification value | Factual dimension with its exact meaning and supplied tolerance |
| Derived coordinate or span | Factual derivation with inputs, frame and assumptions explained |
| Printed photo annotation | `reported_*` value with `photo_annotation` evidence; preserve ambiguity |
| Traced or interpreted photo geometry | Separate `estimates` entry with `photo_trace`/`photo_inference`, calibration and anchors |
| Missing or contradictory value | Explicit unknown or separate observations; do not average unrelated sources |

Grounded inference is welcome when it produces a useful recognizable model.
Explain why a contour, hole count or cavity interpretation follows the image.
Record unresolved conflicts, such as a pitch label that cannot fit the pictured
hole count. Do not choose a convenient round value without evidence, and do not
discard an otherwise useful photo reconstruction merely because it lacks a full
manufacturing definition. See [the photo example](sg90-photo-reconstruction.md).

## 2. Choose the catalog and representation

Servo packs contain `assets`; horn packs contain independent `horns` and
compatibility/placement `mounts`. New records in an existing pack usually need no
Rust. A new pack needs explicit loader registration and validation coverage.
A new geometry type or command requires Rust work; discuss that scope in the PR.

Use a unique stable canonical ID. Increase `data_revision` when correcting an
existing record. Do not change a physical revision or reuse an ID to merge a
different vendor's hardware. Catalog release values identify the data
distribution, not the factory batch.

Use the narrowest useful representation. Known dimensions or hole references
can exist without an integrated model. For horns:

- `dimensions_only` stores facts/references with no solid parts.
- `partial_dimensions` stores supported source geometry and lists omitted details.
- `photo_estimate` stores a reconstruction with separate per-value estimates.

`horn-reference` offers `dimensions`, `source-solid` and `photo-solid` explicitly.
Photo-estimated cuts stay out of formal `holes`. Read the complete
[horn schema](horn-data-schema.md) before adding a horn recipe.

## 3. Describe the important interfaces

| Interface | What to establish or mark unknown |
| --- | --- |
| Body/envelope | Length, width, height and what they include: ears, upper tower, bare shaft or fitted horn |
| Mounting | Plane/frame, centers, hole or slot diameter, thread meaning and any source depth limits |
| Output | Axis position/direction in a named frame; separate bare-shaft extents, outside diameter and horn seat |
| Fixed cable | Entrance position/direction, number of bundles and conductors; source lead length separately from the displayed short marker |
| Detachable port | Physical socket quantity, contacts per socket, source pin convention and any known pose; connected cables remain separate |
| Horn | Source identity, local contour/pattern, named part-pair compatibility and installed pose as independent claims |

Declare the coordinate convention before converting dimensions. For example,
`envelope_minimum` uses a particular envelope's minimum corner; it is not
automatically a mounting datum. A located axis is a line, not a proved shaft
foot or seating plane. Preserve the origin and orientation of every mounting
face rather than flattening all holes into a single XY pattern.

For a threaded feature, keep nominal major diameter, pitch and specified depth
separate. A reference circle is not a tap drill or clearance-hole specification.
Keep `MAX` limits in the evidence; they do not prove usable engagement.

Cable information is especially useful for interference checks. One harness
with three conductors is one cable entrance, not three exits. Two three-contact
bus sockets do not imply two attached cables. Exact entrance dimensions, bend
radius and plug clearance can remain unknown while contact count is known.
Do not route long wires or add an end connector unless the selected recipe
explicitly calls for them.

For a horn mount, named compatibility does not establish a seat. Unknown
placement keeps `origin_mm: null`, no placement evidence and no replacement
roles. Automatic assembly requires documented compatibility and pose plus a
supported non-photo solid. A detached photo reconstruction remains useful
without inventing that relationship.

## 4. Check the data and actual CAD result

Inspect the catalog output first:

```sh
cargo run -- catalog MODEL-NAME
cargo run -- describe CANONICAL-SERVO-ID
cargo run -- horn-describe CANONICAL-HORN-ID
cargo xtask ci
```

The placeholder IDs above must be replaced by the record you added. For a horn,
save its `horn-reference` parameters in a local JSON file and use
`cargo run -- horn-recipe PARAMS.json` to inspect its ordinary command recipe.
The [README](../README.md) describes entry parameters and compatible host editions.

Add or adjust a focused fixture in `tests/` for observable insertion/refusal
behavior. Check known numeric dimensions, source links, units, circle/axis
locations, closed-body counts where applicable and uncertainty labels. Do not
change expected values merely to match an output that conflicts with the source.
If you add code, use the existing Rust test module for meaningful regressions.

Some inventory tests assert the catalog's total record counts. Update those
expectations when a legitimate addition changes the totals. If you are making a
JSON-only contribution, mention that count change in the PR and ask for help
with the small test update; no new runtime implementation is needed for it.

With a compatible SolidCraft CLI, run:

```sh
SOLIDCRAFT_CLI=/absolute/path/to/solidcraft-cli cargo xtask ci
```

Insert the model/reference in the real host and save/reopen it. Inspect geometry,
body roles, source metadata and labels. Capture an actual CAD screenshot with
the important interfaces visible; use multiple views for rear sockets, idlers or
hidden cable entrances. For a family comparison, use the lineup helpers in
`tools/` at a common scale. Those Python scripts are development tools, not the
plugin runtime.

Keep source-derived dimensions distinct from visual fit. A successful Boolean
or closed body does not prove a real spline fit or complete interference envelope.
Report checks that could not run and attach the script/parameters needed to
repeat the views. Vendor photos and drawn mockups do not replace host evidence.

## 5. Send the contribution

Open a focused PR with the hardware/source identity, changes, inference decisions,
remaining unknowns, validation results and your CAD screenshots. An issue is
optional; no draft claim or separate contributor agreement is needed.

Write original explanations. Keep vendor media, commercial CAD data and
GPL/LGPL/AGPL-derived code or geometry out of the repository. The project and
original contributions use MIT OR Apache-2.0. See
[CONTRIBUTING](../CONTRIBUTING.md) and [ATTRIBUTION](../ATTRIBUTION.md).
