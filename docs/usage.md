# Servo Assets for SolidCraft

For the standard two-button interface, see
[Insert a 3D servo or horn](insertion-workflow.md). This guide also covers the
tooling package for detailed geometry and existing automation calls.

An offline Rust/WebAssembly plugin for placing source-labelled servo references
through ordinary SolidCraft commands. It supplies schema-drawn forms and CLI/agent
entries, and creates editable features with one undo per call.

Version **0.13.0**, catalog **`2026-11-19.10`**, includes **28 integrated models and
29 source profiles** across eight catalog packs. The additional SG90 TA0132 profile
provides separate envelopes and an output-axis reference. These servo packs contain
78 source records. Four separate horn packs add **18 horn source variants and 28
compatibility records**, backed by 28 horn source records. Retrieval dates and
evidence stay attached to individual facts.
Unknown physical product revisions stay `null`; a data revision is not a factory
revision.

| Integrated model | Evidence available and limits |
|---|---|
| TowerPro SG90 Analog | Sourced A-F envelopes; PREVIEW mounts and shaft pose |
| TowerPro SG92R | Sourced A-F envelopes; PREVIEW mounts and shaft pose |
| TowerPro MG90S | Sourced A-F envelopes; PREVIEW mounts and shaft pose |
| TowerPro MG90D | Sourced A-F envelopes; PREVIEW mounts and shaft pose |
| TowerPro MG92B | Sourced A-F envelopes; PREVIEW mounts and shaft pose |
| TowerPro SG5010 | Sourced A-F envelopes; PREVIEW four-slot ears and shaft pose |
| TowerPro MG996R | A-F body profile plus a separate documented four-slot XY pattern/axis; source frames remain **UNREGISTERED**, integrated placement is PREVIEW |
| FEETECH FS90 | Documented mounting/output datums; assumed symmetric case alignment and simplified contours |
| FEETECH FS90R | Its own updated drawing establishes mounting/output datums; assumed case alignment and simplified contours |
| FEETECH FT90B | Its own FT90B-C001 drawing establishes mounting/output datums; assumed case alignment and simplified contours |
| FEETECH FS0307 | Sourced housing spans; PREVIEW ears, slots, shaft and cover |
| FEETECH FS5103B | Sourced housing spans, flange underside and four-slot pitches; slot diameter, registration and shaft pose remain unknown |
| FEETECH FT1117M | Sourced housing spans, ear length/underside and spline outside diameter; PREVIEW slot centers and shaft pose |
| Hitec HS-311 | Documented four-pocket pattern and shaft XY; assumed case alignment; bare-shaft tip height unknown |
| Hitec HS-55 | Documented two-hole pattern and shaft XY; assumed case alignment; bare-shaft tip height unknown |
| FEETECH STS3215-C001 old case | A/0 drawing-defined case; PREVIEW main/idler and rear-socket registration |
| FEETECH STS3215-C001 lightguide | Notice-photo PREVIEW; seller nominal envelope, unverified new-mold dimensions/details |
| FEETECH STS3215-C018 | 12 V /1:345 document profile; current mold mapping and case-frame output poses unknown |
| FEETECH STS3215-C044 | 7.4 V /1:191 document profile; current mold mapping and case-frame output poses unknown |
| FEETECH STS3215-C046 | 7.4 V /1:147 document profile; current mold mapping and case-frame output poses unknown |
| ROBOTIS XL330-M077-T / M288-T | Shared dimensioned case, front/back M2 tapping-screw references, main/idler axes and two three-contact ports |
| ROBOTIS XC330-T288-T / M288-T | Separate source identities using the documented common X330 case; no generic XL/XC electrical interchangeability claim |
| ROBOTIS XM430-W350-T / R | Four source-defined mounting faces, 14 M2.5 x 0.45 case references, main/idler axes and two three/four-contact bus sockets |
| ROBOTIS XM540-W270-T / R | Five mounting faces, 16 M2.5 x 0.45 case references, main/idler axes and two three/four-contact bus sockets |

**Start with the Servo button or `insert-servo` for a recognizable model.** The 15 fixed-pigtail
profiles have a housing, two mounting ears, an upper cover, a plain output shaft
and an optional cable outlet marker. Some ears have four slots; HS-55 has two
round mounting holes. STS3215 profiles instead show the serial-servo case, main
output, rear idler and two detachable bus sockets, without attached cables.
Named and colored parts make these interfaces visible. Amber parts and APPROX
labels identify unverified geometry. **PREVIEW geometry is not a verified drilling
template or conservative interference bound.** Read the source profile and measure
your specimen before relying on an assumed datum. A preview shaft does not establish
a spline fit; selected horn bindings carry separate compatibility evidence.

The 15 fixed-pigtail models default to five structural bodies and a **3 mm marker
made of three conductor bodies, with no connector**: eight bodies total. `include_cable=false`
leaves five. `include_connector=true` with cable enabled adds an illustrative
connector box, for nine. Set `wire_length_mm` explicitly for a longer straight
segment, from 2 through 250 mm. Outlet height, insulated-wire diameter and pitch
are also editable planning values. Connector dimensions are catalog-specific
planning defaults; use `cable-space` for caller-sized connector envelopes.
The marker is separate from the published full lead length.

`labels=notes` adds native workspace callouts anchored to fixed design coordinates.
Use `labels=front` for CAD text and leaders in a sketch that moves with the servo.
Cable leaders follow the selected outlet height.
`labels=none` suppresses annotations. Labels and construction references do not
add solid bodies.

**Separate datum tools expose source facts independently of preview geometry.**
Fourteen profiles have mounting-pattern references and fifteen have located output-axis
references, including the TA0132 profile. `mount-pattern` uses documented reference
circles; a circle for an open slot does not reproduce its mouth or specify a screw
fit. `output-reference` refuses an unknown pose and creates a shaft solid only
when its diameter and both vertical limits are established. Hitec's located XY
axes have unknown bare-shaft top heights. MG996R's drawing locates XY but supplies
no vertical extents or registration to the configure-table body. These partial
references remain usable as axes without an invented shaft cylinder.

Independent envelope boxes use their own minimum-corner frames. Mounting frames
and case frames are explicit; a mounting or shaft datum must not silently become
a box corner. In particular, the MG996R preview's assumed case registration does
not register the two source observations. FS90R and FT90B use their own inspected
PDFs, rather than inheriting FS90 geometry because of a similar name.

**Cable clearance is a separate reference.** Each fixed-pigtail profile has evidence
for one harness with three conductors. A harness is one bundle, not three cable
exits. Exact outlet coordinates, conductor/bundle cross-sections, bend radius and
complete connector envelopes remain unknown in the catalog. Published fixed-lead lengths
range from 150 to 320 mm; their endpoint conventions are not established.

`cable-space` creates a user-sized straight harness corridor, optional space for
flexing at the outlet and an optional connector box. Enter outlet XYZ, exit
direction, bundle width/thickness and routing length from your measurement or
layout plan. Flex space is a design allowance, not a factory minimum bend radius.
The route is a straight segment along a coordinate axis. To move a servo and its
planned cable spaces together, activate the servo component before adding the
cable spaces.

**STS3215 ports are separate from attached harnesses.** Each STS profile records two
receptacles, each with `contact_count: 3` and `connector_count: 1`. Bundle and
conductors-per-bundle counts remain null: a socket does not imply a connected
cable. The models show the two sockets and no attached wires by default. The
150/200 mm detachable-cable specifications do not become fixed-pigtail lengths.

## STS3215 source and tooling variants

The collection distinguishes C001 7.4 V /1:345, C018 12 V /1:345, C044 7.4 V /1:191
and C046 7.4 V /1:147. C001 has separate drawing-defined old-case and light-guide
PREVIEW profiles. Only its Akizuki listing explicitly documents the 2025 mold
change and random old/new shipment. The notice does not map both molds onto the
other three electrical variants or establish the current stock's geometry.

The two three-contact sockets are on the same rear/idler face, side by side across
the case width. Their exact positions, socket/plug envelopes and insertion
clearances remain unverified. Front-only images hide these interfaces: inspect
both front and rear to see the main output, rear idler, sockets and C001 light
guide difference. These are original simplified CAD references, not vendor CAD.

The four drawing-defined STS models have 11 solid bodies each: housing, main,
idler, two socket shells and six contacts. The C001 light-guide PREVIEW adds one
window body, for 12. Their fixed-lead recipes are empty, so cable/connector toggles
do not remove the sockets or attach wires. The new-mold notice supplies no full
dimensional drawing: pre-change step/shaft values used in that preview remain
planning references, not verified new-mold facts.

The integrated main/idler point along +Z/-Z, but their case XY registration is
explicitly approximate. The source's 12.5 mm internal reference is not a verified
case-end offset or proved case-midplane datum, so it does not supply a factual
case-frame output pose. The axial relation 29 +3.4 +4.1 =36.5 mm remains separate
from 32/35 mm cover extents. Case-fastener topology is not a documented bracket
mounting template. See the source audit before using a preview to design a fit.

## ROBOTIS 330, XM430 and XM540

The eight ROBOTIS profiles show separate main and rear/idler interfaces, case
mounting recesses and two opposed side bus sockets, with no attached harnesses.
The 330 profiles use 11 bodies each; XM430 T/R use 12/14 and XM540 T/R use 11/13.
Their case frames follow long X, width Y, main +Z and rear -Z. The manufacturer
drawings locate mounting centers and axis centerlines. Socket envelopes/poses,
bare XM coupling shapes and shallow recess depths remain planning geometry.
Rear marker contours, including XM540's cross-slotted idler-axis marker, are
provisional and do not establish fitted retention hardware.

`mount-pattern` now offers a mounting face/group selector. Thread metadata keeps
nominal screw size, documented pitch and the raw published maximum-depth callout.
Reference circles use nominal major diameter, not a tap drill or a clearance fit.
M2 tapping screws on 330 plastic cases remain distinct from M2.5 x 0.45 XM threads.
The XM430 near-end pair remains preview-only because its registration is not
fully established in the selected source. No maximum-depth callout establishes
a safe screw length. Optional horns and their bolt circles are separate assets;
the X330 base depiction is retained when no additional horn is selected.

XM540 also records the documented five-contact external-I/O and three-contact
dual-joint interfaces. Their exact accessible poses are unknown, so they have no
invented exterior parts. Three GPIO channels do not mean three sockets.

See [ROBOTIS sources](robotis-sources.md) and [other servo sources](sources.md)
for dimensions, conflicts and unknowns, and the [extension design](design.md)
for variants, horns and QDD.

## Horn dimensions and photo reconstructions

Version 0.9 includes two tangible 3D reconstructions of the supplied photograph:
a 31.2 mm double arm with twelve linkage holes and a 21.3 mm single arm with
seven. Printed dimensions constrain their heights and hub sizes; calibrated
photo traces supply their distinct tapered contours, rounded ends and hole
positions. Each inferred value retains its method and source evidence. These
are explicit photo estimates, with no claimed spline fit or installed seat.

The 18 variants comprise nine `dimensions_only` records, seven supported partial
source solids and two `photo_estimate` solids. The 28 existing servo previews
retain their individually documented limitations. Shape inference is supported
when it is grounded in an identified image and recorded separately from facts.

| Horn pack | Variants | Compatibility records | Source audit |
|---|---:|---:|---|
| `horns-micro.json` | 8 | 4 | [AU and Hitec correction audit](horn-micro-correction-audit.md) |
| `horns-robotis-430.json` | 4 | 12 | [X330/X430 corrections](horn-robotis-430-corrections.md) |
| `horns-robotis-540-sts.json` | 4 | 12 | [XM540 normal/idler and STS3215 discs](horn-robotis-540-sts-audit.md) |
| `horns-user-reference.json` | 2 | 0 | [Photo reconstruction and numerical calibration](sg90-photo-reconstruction.md) |

`horn-reference` inserts a detached reference with an explicit mode:

| Mode | Result |
|---|---|
| `dimensions` (default) | Known circular references and annotations, with zero solid bodies |
| `source-solid` | An available partial source solid; rejects photo estimates and dimensions-only records |
| `photo-solid` | A photo-estimated 3D horn with estimated contour/hole labels; accepts only photo-estimate records |

Unsupported modes are refused before host commands. Printed photograph labels
remain `reported_*` values with `photo_annotation` evidence. Their geometric
interpretations live in a separate `estimates` map with units, `photo_trace` or
`photo_inference` evidence and pixel anchors where available. Unknown numerical
uncertainty remains null. Estimated hole cuts do not become verified mounting
datums. See the [reconstruction](sg90-photo-reconstruction.md) and
[source review](sg90-photo-review.md) for the measurements, inference and
conflicting 2.43 mm pitch annotation.

Dimensions, manufacturer part-pair compatibility and installed pose are separate
claims. A `FIT DOCUMENTED` label concerns the physical parts named by the source;
it does not establish printable geometry, a complete spline or a verified CAD
mating fit. Unknown placement has no coordinate and cannot replace a servo part.
The twelve AU-to-TowerPro proposals and two changed-case STS light-guide proposals
were removed. No arbitrary sleeve is supplied for either ROBOTIS idler.

Automatic attachment through `servo-reference` requires documented compatibility,
a documented installed position and an available partial solid. Four combinations
currently qualify: HN12-N101 on XM430-W350-T/R and HN13-N101 on XM540-W270-T/R.
Photo-estimated horns cannot attach automatically, even if compatibility records
are added later; `photo-solid` places them as detached user layouts.
Other records remain available through detached `horn-reference`. X330 seating
is unknown, so no X330 attachment or `output-flange` replacement is offered.

Omit `horn_id` or use `none` in `servo-reference` to preserve its base-profile
geometry with no additional accessory. This does not certify a bare factory
assembly: the existing X330 depiction may include a plastic normal horn.
Body names, annotations and source snapshots retain the evidence status and
omissions. Visual angles are user layout choices, not electrical neutral or
verified spline phase; an axial adjustment is a recorded offset from a documented
assembly baseline. Partial outer volumes do not provide complete contact geometry
or conservative clearance bounds.

## Build and check

The source is a standalone crate, outside SolidCraft's workspace. Rust with the
`wasm32-unknown-unknown` target is required. Dependencies must be cached for the
offline build; the fetch commands in [README](../README.md) prepare a fresh
development machine. Run `cargo xtask ci` for the full repository gate.

```sh
bash build.sh
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings
SOLIDCRAFT_CLI=/path/to/current/solidcraft-cli
"$SOLIDCRAFT_CLI" plugin check dist/servo-assets
"$SOLIDCRAFT_CLI" plugin test dist/servo-assets --out evidence/plugin-tests
```

The component wrapper uses the permissively licensed `wit-component` crate; no
global `wasm-tools` installation is required. The current archive is
`dist/servo-assets-0.13.0.zip`. It uses the command, storage and dialog interfaces
in SolidCraft 0.47. Host fixtures are included under `tests/` in the archive.

For an older host, `bash build-legacy.sh` creates
`dist/servo-assets-legacy-0.13.0.zip` using `commands@0.1`. It creates the same
geometry and returns the source snapshot in its command reply. That host cannot
save plugin storage in the design: `metadata_saved_in_design` is false. Keep the
returned snapshot or source package alongside the design. Both editions have the
same plugin ID; install the edition appropriate to the host.

Create a same-scale comparison through the real host:

```sh
bash build.sh --tools
python3 tools/build-lineup.py --cli "$SOLIDCRAFT_CLI" --package target/tooling/servo-assets --out evidence/lineup
```

The helper selects all 28 integrated profiles by default, in five columns with
75 mm column spacing and 85 mm row spacing. Each case base is at Z=0 and each case
width is centered on its row. It creates a saved design, command script and
verification receipt, with CAD text identifying each model and its evidence
limits. Expected part roles and body counts come from the selected recipes,
including socket bodies where present; the current all-model expectation is
270 closed bodies. The 15 fixed-pigtail profiles use short
three-wire markers and omit optional lead-end connector boxes; STS profiles add
no attached wires. The receipt records the host result; these expectations alone
are not a report that a particular package passed.

Repeat `--asset-id` to select profiles. Adjust `--columns`, `--column-spacing` and
`--row-spacing` to suit the comparison, for example:

```sh
python3 tools/build-lineup.py --cli "$SOLIDCRAFT_CLI" --package target/tooling/servo-assets --out evidence/micro-pair --asset-id towerpro.sg90-analog.configure --asset-id feetech.fs90r.akizuki-drawing --columns 2 --column-spacing 75 --row-spacing 85
```

Use the legacy package path with an older installed CLI. If the app has no zip
installer, extract the legacy archive into its plugins folder; it contains the
required `servo-assets/` folder. The Linux default is
`~/.local/share/solidcraft/plugins/`. Listing a package does not run it.

## Use

Install the ZIP through **Preferences → Plugins → Install plugin…**. The
**Model** tab exposes **Servo model** (`insert-servo`) and **Horn model**
(`insert-horn-model`). Both create 3D bodies from the selected catalog model.

The commands below are available in the tooling build (`./build.sh --tools`).
Use that package when calling the detailed commands through the CLI or MCP.

| Entry | Effect | Input |
|---|---|---|
| `servo-assets.catalog` | Read | Optional `query` |
| `servo-assets.describe` | Read | `asset_id` |
| `servo-assets.place-servo` | Edit | `asset_id`; optional placement XYZ and `show_callouts` |
| `servo-assets.servo-reference` | Edit | `asset_id`; optional `horn_id`, horn angle/axial adjustment, placement XYZ, cable/connector inclusion, outlet height, wire diameter/pitch/length and `labels` |
| `servo-assets.horn-reference` | Edit | `horn_id`; optional `mode` (`dimensions`, `source-solid` or `photo-solid`), placement XYZ and `angle_deg` |
| `servo-assets.insert` | Edit | `asset_id`; optional `representation`, `mount_group_id`, `clearance_mm`, placement XYZ |
| `servo-assets.mount-pattern` | Edit | `asset_id`; optional `mount_group_id`, `clearance_mm`, placement XYZ |
| `servo-assets.output-reference` | Edit | `asset_id`; optional `output_id` (default `main`), `clearance_mm`, placement XYZ |
| `servo-assets.cable-space` | Edit | `asset_id`, `port_id`, `exit_x_mm`, `exit_y_mm`, `exit_z_mm`, `direction`, `bundle_width_mm`, `bundle_height_mm`, `straight_length_mm`; optional `flex_space_mm`, three `connector_*_mm` sizes |

Placement parameters are `x_mm`, `y_mm`, `z_mm`. Integrated cable parameters are
`include_cable`, `include_connector`, `outlet_height_mm`, `wire_diameter_mm`,
`wire_pitch_mm`, `wire_length_mm`. Labels accept `notes`, `front` or `none`.
The two integrated servo entries return `component` with the component ID,
name and occurrence ID. Use the occurrence ID with SolidCraft's
`occurrence.move` command to move the whole servo.
Attached-horn parameters are `horn_id`, `horn_angle_deg` and
`horn_axial_offset_mm`; `describe` reports whether a compatibility record permits
automatic assembly. Unsupported attachments are rejected. For a separate source
reference, pass these parameters to `servo-assets.horn-reference`:

```json
{"horn_id":"unidentified.photo-se22ca210.double-arm","mode":"photo-solid","x_mm":0,"y_mm":0,"z_mm":0,"angle_deg":0}
```

`insert` defaults to `case-envelope`. `overall-envelope` is available only where
the selected profile documents the required ear length and shaft-top height;
it excludes the cable and horn. A clearance parameter is an explicit design
allowance, separate from published tolerances.

Inspect facts or generate an ordinary host script without installing the plugin:

```sh
cargo run --offline -- catalog FEETECH
cargo run --offline -- describe towerpro.mg996r.configure
cargo run --offline -- horn-describe hitec.fs-x15.rev-d-20230428
cargo run --offline -- recipe feetech.fs90r.akizuki-drawing mounting-pattern > fs90r-pattern.json
solidcraft-cli run fs90r-pattern.json --save fs90r-pattern.sc
cargo run --offline -- recipe hitec.hs55.hcs-v2-2 output-reference > hs55-output.json
solidcraft-cli run hs55-output.json --save hs55-output.sc
```

For `cable-space`, provide all three connector sizes or leave all three zero.
The values in `tests/cable-space.json` are synthetic tests, not measurements.
The native `cable-recipe PARAMS.json` command accepts that case's `params` object.
The native `horn-recipe PARAMS.json` command accepts `horn-reference` parameters.
Plain command scripts create geometry without plugin source metadata.
The calibrated photo recipes are reproduced by `tools/update-photo-horns.py`;
the [reconstruction notes](sg90-photo-reconstruction.md) explain its inputs.

The plugin validates eight bounded servo packs and four independent horn packs,
with distinct source and asset/accessory IDs. Each successful insertion stores the selected source record, its
referenced sources, parameters and command result under
`plugins/servo-assets/blobs/instances/` in the design, plus a small instance index.
Attached horns add an `accessories` snapshot containing their facts, compatibility,
documented placement, source references and applied transform. Detached references
save their horn record, per-feature estimates and parameters; dimension mode saves
annotations and no bodies. With `labels=none` on a servo, its status remains in
body names and metadata.
Owned `body.rename` calls replace fallback `BodyN` names with profile/role names
and a creating-feature-ID suffix. Repeated profiles and horn variants retain
distinguishable roles after saving and reopening the design.
It does not embed vendor PDFs or photographs. Updates do not resize saved
instances. Ordinary features and saved snapshots remain when the plugin is
missing. The legacy edition returns its snapshot instead of embedding it.

To add a source profile, extend a catalog pack and rebuild. Missing geometry needs
new measurements or source evidence, not another model's pattern. Horn variants
extend their own catalog packs with separate fit and pose records; see the
[horn data schema](horn-data-schema.md). New geometry kinds need validated
adapters before they can be offered as usable interfaces. Oriented case faces and source thread callouts are already
supported; safe screw engagement and full thread geometry remain separate work.
MG995, DS3218, Futaba S3003, additional STS3215 tooling/specimen variants and
maker-specific QDD actuators remain future source-audit work.
