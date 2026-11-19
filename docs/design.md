# Servo reference assets

## Working prototype

Version **0.9.1** uses merged catalog release **`2026-11-19.10`**. The standalone
Rust/Wasm plugin runs offline, uses schema-drawn host forms, and inserts ordinary
SolidCraft features through commands. It has 28 integrated models and 29 source
profiles, with 78 source records across eight independently validated packs:

| Pack | Source profiles | Integrated models |
|---|---:|---:|
| `towerpro-micro.json` | 5 | 4 |
| `towerpro-standard.json` | 4 | 4 |
| `feetech-expansion.json` | 5 | 5 |
| `classic-servos.json` | 2 | 2 |
| `feetech-serial.json` | 5 | 5 |
| `robotis-330.json` | 4 | 4 |
| `robotis-xm430.json` | 2 | 2 |
| `robotis-xm540.json` | 2 | 2 |

Four independent accessory packs add 18 horn source variants, 28 compatibility
records and 28 source records without multiplying the 28 integrated servo models:

| Horn pack | Horns | Bindings | Sources |
|---|---:|---:|---:|
| `horns-micro.json` | 8 | 4 | 7 |
| `horns-robotis-430.json` | 4 | 12 | 8 |
| `horns-robotis-540-sts.json` | 4 | 12 | 12 |
| `horns-user-reference.json` | 2 | 0 | 1 |

Nine variants retain dimensions without solids, seven provide partial source
solids and two provide photo-estimated solids. The latter use printed major
dimensions plus calibrated photo traces, with per-feature provenance separate
from verified facts. The existing servo previews retain their separate
source/assumption boundaries.

The integrated collection covers SG90 Analog, SG92R, MG90S, MG90D, MG92B, SG5010,
MG996R, FS90, FS90R, FT90B, FS0307, FS5103B, FT1117M, HS-311 and HS-55, plus five
STS3215 source/case profiles and eight ROBOTIS 330/XM430/XM540 representatives.
The separate Aus Electronics Direct TA0132 SG90 profile provides envelopes and an output axis.
A model name identifies a product family, not a guaranteed interchangeable interface.

Each record distinguishes physical revision, data revision, seller SKU, source
observations and representation assumptions. Physical revisions remain unknown
unless evidence identifies them. New source facts or previews do not silently
update saved instances.

| Representation | Current behavior | Evidence boundary |
|---|---|---|
| Integrated reference | Named parts from a profile recipe, including conventional ears/shaft or serial-servo main/idler and sockets | A mixture of published dimensions, explicit alignment assumptions and preview parts |
| Envelope | Ordinary box from three selected source dimensions | A reference box with explicit omissions and its own minimum-corner frame |
| Mounting pattern | Fixed circles from a complete documented center/diameter record | Screw references, not complete slot contours or a screw-fit guarantee |
| Output reference | Fixed markers and a documented axis; shaft envelope only with known diameter and both height limits | A partial located axis is useful even when bare-shaft tip height remains unknown |
| Detached horn reference | Known circles/annotations by default; source-solid or explicit photo-solid modes provide their respective 3D references | Photo inference remains a separate evidence class; a user layout does not establish compatibility or an installed seat |
| Attached horn | Supported partial solid transformed onto a documented profile/output pose | Requires documented part-pair compatibility and documented placement before any host commands |
| Cable space | User-sized straight harness corridor, outlet-flex allowance and optional connector box | Explicit planning keepouts, separate from manufacturer geometry |

The TowerPro A-F key distinguishes housing top from rectangular case shoulder and
shaft top. FEETECH C labels also require their individual drawings: FS0307 C is a
housing span, while FS5103B and FT1117M drawings terminate at an upper boss/collar,
below the exposed spline. A headline length/width/height triple does not resolve
these meanings. See [sources.md](sources.md).

An outer reference box fills empty space around ears and upper features. It is not
an exact contact surface. Simplified integrated outlines are not conservative
interference bounds. Clearance parameters remain explicit user allowances, not
changes to nominal facts or invented manufacturing tolerances.

## Integrated geometry and uncertainty

The 15 conventional fixed-pigtail references have five structural bodies: housing,
near ear, far ear, cover and shaft. Ears can have one or two mounting pockets.
HS-55 has two round holes; the other conventional recipes depict open slots.
Serial-servo recipes use their own case, main/idler and socket parts instead of
forcing an ear shape or a fixed part count. Ordinary primitive features, owned
cut/join operations and part colors remain editable after insertion.

For a fixed-pigtail model, the default adds three short conductor markers, for
eight bodies. Disabling cable
leaves five. The connector is omitted by default; explicitly enabling it with the
cable adds one planning box, for nine. Annotation sketches and construction
references do not add solid bodies.

Part basis, notes, colors and visible labels preserve evidence status. Amber
identifies unverified geometry such as a preview shaft position or uncertain
height registration. All recipes retain explicit assumptions in the instance
snapshot. A recognizable feature does not promote its coordinates into factual
`dimensions`, `mount_groups` or `output_interfaces`.

### Documented FEETECH datums

FS90, FS90R and FT90B each have independently inspected drawings. Each gives
27.7 mm slot pitch, diameter 2.2 circular slot pockets, an output axis 8.7 mm from
the near mounting center, diameter 4.86 output spline, and base-referenced heights.
The XY equality is an observation from three sources, not inheritance by model name.

Their `near_mount_center` frame places the near slot at `(0,0)`, the far slot at
`(27.7,0)` and the ear underside at Z=0, with +Z toward the output. FS90 and FT90B
have base Z=-15.7; FS90R has base Z=-16.7. All three have nominal shoulder Z=6.7,
upper housing top Z=11, shaft top Z=14 and ear top Z=2.5 in their mounting frames.
Subtraction-derived height bounds add input tolerances. Linear callouts carry
+/-0.1 mm; diameter tolerances are not supplied.

Case placement assumes symmetry between the mounting centers. Its X extent is
2.6 through 25.1 and Y extent -6.05 through 6.05. The drawing does not explicitly
dimension that registration. Ear perimeters, 1 mm slot mouths and the two-lobe
cover are approximate. A plain shaft cylinder represents an outside envelope,
without teeth, seating plane or electrical-neutral phase.

### Preview geometry with limited source dimensions

The seven TowerPro integrated profiles use published A-F extents. SG90 Analog,
SG92R, MG90S, MG90D, MG92B and SG5010 still lack factual slot patterns and output
poses. Their round-number shaft positions, ear thicknesses, slot diameters,
shoulder partitions and cover contours remain explicit planning values.

MG996R has an additional Akizuki drawing with a four-slot XY pattern and shaft
axis. The configure-table body and drawing pattern are **UNREGISTERED source
frames**: there is no documented case-edge registration, height datum linking the
drawing to configure F, or proof of the same physical revision. The integrated
preview centers that pattern on the configure body as an assumption. Its drawing
axis and mounting pattern remain independent references; the preview does not
establish a factual combined case/mount/shaft placement.

FS0307 provides only 20 x 8.3 x 17.3 mm housing spans. Its ears, four slots, turret and
bare shaft are planning. FS5103B supplies 40.8 x 20.1, upper-boss datum 38, flange
underside 26.8 and four-slot pitches 49.5 x 10 mm, but no slot diameter, complete case
registration or output pose. FT1117M supplies 30 x 12, upper-collar datum 31.7, flange
underside 20, ear overall 40.5 and spline outside diameter 5.8, while slot centers,
slot diameter, shaft axis and bare-shaft tip height remain unknown. Source scalar
facts stay available without manufacturing a complete mounting group or axis.

HS-311 and HS-55 have documented mounting pockets, shaft XY, case size and flange
heights. Their case registration assumes symmetry and outlines are simplified.
The Hitec drawings include fitted horns, so those horn heights do not establish
bare-shaft tips. The integrated shaft uses a clearly marked illustrative Z
registration; the factual output record keeps `shaft_top_dimension: null`.

All PREVIEW geometry supports comparison and identifies what to measure next.
It is not a verified drilling template. Promote an assumed datum only through a
new dimensioned or measured record with specimen identity, method and uncertainty.
A source disagreement is not proof of a physical revision.

## STS3215: source-defined cases and detachable ports

The serial pack has five integrated profiles: `feetech.sts3215-c001.a0`,
`feetech.sts3215-c001.lightguide`, `feetech.sts3215-c018.a0`,
`feetech.sts3215-c044.a0` and `feetech.sts3215-c046.a0`. It distinguishes four
electrical/model suffixes and the documented C001 old-case/light-guide observations.
A source drawing, a seller item and a physical production revision remain separate identities. An A/0 document date is
not a proved production revision, and a photograph is not a current-stock promise.
Only the C001 listing explicitly connects its seller item to the 2025 mold notice;
the notice does not map old/new tooling onto C018, C044 or C046.

The old rear case has an open groove toward the idler; the new notice image shows
a circular light-guide window. The notice provides no dimensions for that change.
The light-guide profile is a PREVIEW with seller-SKU nominal envelope dimensions
and explicitly assumed feature sizes/placement. Its pre-change drawing-based
step, shaft and contact-pitch choices do not become verified new-mold facts.
Blue circles in the notice are editorial highlights, not case geometry.

Each drawing shows main and rear idler/support interfaces on opposite faces. The
source axial relation is 29 mm between their support planes, 3.4 mm exposed main
spline and 4.1 mm rear projection, totaling 36.5 mm across their tips. Separate 32/35
cover/bulge extents must not replace the support-plane relation. The 25T/OD5.9 main
spline specification and rear diameter 6 callout alone do not establish horn compatibility.
The separate accessory-page drawings now support named main/rear disc bindings
to each inspected A/0 profile; installed poses remain unknown and automatic
attachment is unavailable.

The 12.5 mm callout terminates on an internal reference line. Registering that line
as the case midplane is a preview assumption, not an established case-end datum.
The visible main/idler XY uses that assumption; factual output interfaces retain
unknown case-frame poses. Showing a plain shaft and idler does not create a
verified `output-reference` choice. The 8-PA2.0 case-fastener callout supplies no
complete bracket-hole coordinates, pilot diameters, engagement depths or load
interface. It is not a factual bracket mounting template.

Both detachable bus receptacles are on the rear/idler face, side by side across
the case width, with mating direction opposite the main output. Each port records
one connector with three contacts. The four A/0 document profiles specify 2.54 mm
contact pitch; the light-guide photo profile establishes counts without a new
pitch specification. Exact socket centers, envelope sizes, keying, insertion space
and contact orientation in the model frame remain unknown. The integrated socket/pin geometry
is labelled planning geometry. A contact is not an attached insulated conductor.

The STS recipes have an empty fixed-lead `cables` list. They show two sockets but
create no attached harnesses or three-wire markers, even though a detachable
150/200 mm cable is described in the specifications. The four A/0 models have
11 solid bodies each; the light-guide PREVIEW has 12. Cable/connector switches do
not hide socket shells/contacts or synthesize a harness. Connected harness count
is a layout state, not inferred from available ports. `cable-space` can reserve explicit
user-planned routing/plug space for either selected port without claiming a
manufacturer cable path. Both front and rear views are needed to inspect the
main output, rear idler, two sockets and the C001 tooling distinction.

## Frames, mounting patterns and partial outputs

Every representation names its frame. Independent envelope boxes use
`envelope_minimum`, meaning their own minimum corner. Integrated models may use a
case frame or `near_mount_center`. The latter is defined per source: it can be the
midpoint of a four-hole column, rather than one actual screw center. +X follows
the source pattern and +Z points toward the output.

Mounting groups support both frames. A case-frame group has either a
`plane_height_dimension` for the legacy XY plane or an explicit `plane` with
origin, orthonormal X/Y directions and evidence. Hole centers are local 2D
coordinates on that plane. This supports opposite covers, long sides and short
ends without flattening their hole patterns into one plane. `mount_group_id`
selects a source-defined group; omission or `profile-default` selects the
representation's group. A group from another asset is rejected before editing.
Mount-relative groups keep both plane fields absent/null and define their own
local XY zero plane. A known pitch without a
verified diameter or case registration can remain scalar evidence without a
complete mounting group. This does not imply that all current profiles have a
case-registered mounting pattern.

Each hole may store a thread designation, documented pitch and raw source depth
callout. Unknown pitch/depth stay null. Nominal fastener major-diameter circles
are references, not pilot holes or modeled internal threads. ROBOTIS `DP MAX`
callouts remain maximum values; they do not establish a safe screw length or a
minimum available engagement. The 330 assembly's M2 tapping screw is distinct
from the XM case's M2.5 x 0.45 machined thread. Integrated shallow recesses are
visual markers with explicit planning depths. See [ROBOTIS source audit](robotis-sources.md).

A named output interface stores frame, axis line, direction, coordinate
uncertainties, evidence and unknowns. A missing pose stays unavailable; the plugin
does not choose the box center. A located axis need not have complete vertical
extents. Missing housing/shaft-top values remain null in summaries. A drawn axis
is a line, and its intersection with a reference plane is not a physical shaft foot.

| Source profile | Available output reference |
|---|---|
| FS90, FS90R, FT90B | Mount-relative axis `(8.7,0,0)`; known top heights and diameter 4.86 allow a nominal 3 mm shaft envelope |
| MG996R drawing | Mount-pattern-relative XY axis `(14.25,0,0)`; vertical extents and registration to configure body unknown; no factual shaft solid |
| HS-311 | Mount-relative XY axis `(13.9,0,0)`; housing top known but bare-shaft top unknown; no factual shaft solid |
| HS-55 | Mount-relative XY axis `(9,0,0)`; housing top known but bare-shaft top unknown; no factual shaft solid |
| TA0132 SG90 | Case-frame axis `(5.8,6.05,0)`, shaft top 30.7; diameter unknown, so no shaft solid |
| XL330 / XC330 representatives | Case-frame main/idler centerline `(9.5,10)`; output-face extent is specific to the source depiction; no assumed accessory horn pattern |
| XM430-W350 T / R | Case-frame main/idler centerline `(11.25,14.25)`; bare shaft diameter and heights unknown |
| XM540-W270 T / R | Case-frame main/idler centerline `(13.75,16.75)`; bare shaft diameter and heights unknown |

MG996R's Z=0 is a drawing-reference plane only; no configure-table flange height
is silently attached to it. Hitec's mounting frames have sourced underside heights,
but local spline protrusion dimensions still do not establish absolute collar
support planes or bare tips. The AU SG90 source has conflicting mounting pitches
of 29 and 28.6 mm, so it offers no screw pattern and inherits no TowerPro cable data.

The factual `output-reference` path creates a +Z shaft envelope only when outside
diameter and both height limits are known. Located axis-only references may point
along +Z or -Z; the reverse-height shaft adapter is not implemented. Missing poses
remain unavailable in either direction. A preview shaft inside
`servo-reference` is a separately labelled planning part. Neither establishes
spline teeth, a seating plane or neutral angular phase. Horn compatibility comes
from a separate explicit accessory binding, rather than the appearance of a shaft.

## Horn identity, compatibility and assembly pose

Horn catalogs use stable source identities, revisions, dimensions, formal hole
patterns and explicit unknowns. Version 0.9 supports the requested photo-based
shape reconstruction with per-feature provenance. It distinguishes published
facts, printed annotations and inferred geometry; a warning alone cannot turn an
unsupported number into a verified source dimension.

- `dimensions_only` requires an empty `parts` list. Numeric source facts and
  located hole references remain available without an invented silhouette.
- `partial_dimensions` requires supported outer-profile parts, with omitted
  geometry stated explicitly. It does not establish a complete manufactured part.
- `photo_estimate` requires geometry and a nonempty `estimates` map. Each numeric
  estimate records value, unit, source/locator/method, interpretation, original
  pixel anchors where available and nullable uncertainty. Parts retain the
  `photo_estimate` basis and explicit reconstruction labels.
- Photograph labels use `reported_*` dimension keys and `photo_annotation`
  evidence in dimensions-only or photo-estimate records. They do not become
  verified physical dimensions or formal hole datums. A geometric interpretation
  belongs in `estimates`, using `photo_trace` or `photo_inference` evidence.

The dimensions-only records are four AU horns, three Hitec arms and two ROBOTIS
idlers. The seven partial outer solids are the
Hitec M-O24 disc, two X330 normal horns, HN12-N101, HN13-N101 and the two STS3215
discs. Their unsupported geometry remains absent. The two photo reconstructions
are different: the printed 31.2/21.3 lengths, 2.36/2.08 plates and 6.74/6.76 hub
spans constrain calibrated tapered line/arc profiles, twelve/seven linkage cuts
and smooth mating-cup approximations. Their tip radii, hole centers and cavity
interpretations are source-linked estimates, not inherited AU/Hitec dimensions.
Formal `holes` stays empty for these photo records. Spline teeth and cosmetic
molding dimples are omitted. The conflicting 2.43 pitch label remains a reported
observation rather than overriding the clearly pictured aperture counts.

`tools/update-photo-horns.py` reproduces the photo recipes from the recorded
calibration. The double's fitted half-taper is 3.24 degrees; the single uses a
smooth circular-root/end tangent regularized to 5.10449 degrees from the 4.89-degree
photo fit. Each estimate retains its own derivation and unknown physical error.
See [reconstruction](sg90-photo-reconstruction.md) and
[source review](sg90-photo-review.md).

The `Profile` primitive stores `z_mm`, `height_mm` and a closed sequence of
analytic `Line`/`Arc` segments. It creates an owned sketch and extrudes that
insertion's sketch ID, preserving true tapered flanks and circular tips instead
of a stack of rectangular strips or a display mesh. Subsequent joins/cuts target
the newly created body role. No unrelated sketch or body is selected implicitly.

Compatibility and placement remain independent. `FIT DOCUMENTED` identifies a
manufacturer-supported physical part pair. It supplies no claim that the partial
outer CAD is printable, reproduces mating teeth or fits a real shaft. A mount's
`placement_basis` is `documented` or `unknown`; an unknown pose requires
`origin_mm: null`, no placement evidence and no replacement roles. A loose-part
thickness or a preview shaft tip does not locate an installed outer plate face.
The twelve AU/TowerPro proposals and two changed-case STS light-guide proposals
were removed. Named Hitec, X330 and A/0 STS compatibility can remain recorded
while their seating positions remain unknown.

`horn-reference` accepts `horn_id`, `mode`, `x_mm`, `y_mm`, `z_mm` and
`angle_deg`. Default mode `dimensions` inserts known circles and source
annotations with zero solid bodies. Unsupported outlines remain absent. Mode
`source-solid` creates an available partial outer solid and source annotations;
a dimensions-only record or photo-estimated record is rejected before host
commands. Explicit `photo-solid` accepts photo estimates, creates their analytic
3D reconstruction and labels the estimated contour and holes. All three modes
are detached user layouts and infer no servo compatibility or seating plane.

Automatic assembly through `servo-reference` requires documented compatibility,
documented placement, nonempty supported parts and a non-photo status. Four combinations currently
qualify: HN12-N101 on XM430-W350-T/R and HN13-N101 on XM540-W270-T/R. A documented
idler flange pose alone does not enable assembly when that idler has no solid.
Photo-estimated horns never qualify, even if a future record proposes a binding.
X330 normal-horn seats are unknown, so no automatic X330 attachment or
`output-flange` replacement is offered. Unsupported combinations fail before
any host edits.

`servo-reference` still accepts `horn_id`, `horn_angle_deg` and
`horn_axial_offset_mm` for the enabled combinations. Omission or `horn_id: "none"`
preserves the existing base-profile geometry without an extra accessory. That
default does not certify a bare factory assembly: the X330 source depiction may
include a plastic normal horn, without proof of its exact factory SKU/revision.
Existing source snapshots remain historical records; the new catalog does not
silently resize or reinterpret earlier geometry.

For supported solids, local origin is the shaft center on the outer plate face,
Z0, with +Z outward. Source-defined bosses may extend above the plate and rear
hubs below it. Reverse outputs use a rigid Y/Z reversal. Visual rotation is not
electrical neutral or a verified spline index; axial adjustment is a recorded
user offset from the documented baseline and does not establish contact geometry.

Body names and source annotations state the reference status and omissions.
Attached horns add an `accessories` snapshot containing the record, compatibility,
documented pose, sources and applied transform. Detached references save the
source record, separate per-feature estimates and parameters; dimension mode
saves an annotation sketch and no bodies. Unknown estimate uncertainty stays
null, rather than becoming a claimed manufacturing tolerance. Owned `body.rename` calls persist profile/role names with the creating
feature ID as a suffix, preserving distinct roles after save/reopen.

Unknown details do not become printable geometry, a recovered spline fit or a
conservative interference bound. See [horn schema](horn-data-schema.md),
[micro correction audit](horn-micro-correction-audit.md),
[X330/X430 corrections](horn-robotis-430-corrections.md) and
[XM540/STS audit](horn-robotis-540-sts-audit.md).

## Cable interfaces and planning space

Cable data is a first-class interface. Separate contact count, connector count,
attachment type, connected bundle count, conductors per bundle and published cable
length. Each of the 15 fixed-pigtail profiles has source evidence for one attached
harness with three conductors. The AU SG90 record has no inherited cable facts. Count evidence can
come from an exact product photograph or a documented wiring diagram; it is not
a dimensional measurement or a guarantee for every product sold under the name.

For detachable STS ports, `contact_count` is 3 and `connector_count` is 1 per port,
while `bundle_count` and `conductors_per_bundle` remain null. Two available ports
do not imply two connected cables, nor six permanently attached wires. Their
specified accessory-cable lengths stay separate from fixed-lead metadata. The
AU SG90 has neither inherited port counts nor inherited attached-lead facts.

Exact exit XYZ/direction, insulated-wire size, bundle cross-section, strain-relief
shape, minimum bend radius and complete connector envelope remain unknown. Lead
length does not establish a straight route or usable free length. Endpoint
conventions are unspecified, and a wire marker does not represent the full lead.

Integrated cable defaults are planning values: outlet 2 mm above the case base,
wire diameter 1 mm, pitch 1.1 mm and displayed length 3 mm. Height, diameter, pitch
and segment length are editable; pitch must be at least diameter and length is
bounded 2..250 mm. Connector size comes from each recipe and remains illustrative;
Hitec defaults differ from the 10 x 8 x 4 mm defaults in the other packs. The
connector is opt-in. These planning values never overwrite cable-port facts.

`cable-space` requires exit coordinates, one of six axis directions, bundle width
and height, and straight corridor length. Flex space and a caller-sized connector
box are optional. A corridor represents the whole harness, not three separate
exits. Snapshots mark these bodies `user_specified_planning_envelopes`, with the
selected source facts alongside the user's inputs.

The current route is a single axis-aligned straight segment. There is no flexible
simulation, automatic routing, motion-clearance guarantee or automatic following
of a moved servo. Shared placement in a component is the caller's assembly choice.
Future measured ports can add exact exit datums through explicit catalog records.

## Labels and observable evidence

`labels: "notes"` creates native `note.add` callouts saved in the design and visible
in the GUI. Their anchors are design coordinates, not associative body links.
CLI snapshots omit note overlays. `labels: "front"` creates actual `sketch.text`
outline curves and construction leaders in a separate front-view sketch. These
curves can be transformed or occluded like other sketch geometry; they are not
marks painted onto an exported screenshot. Cable leaders use the computed outlet
position, including insertion overrides and their bound port. `labels: "none"`
creates neither form. Body names alone are not automatic CLI image labels.

The comparison helper selects all 28 integrated profiles by default. It aligns
case bottoms to Z=0, centers their case widths on rows and arranges five columns
with 75 mm column spacing and 85 mm row spacing. Repeated `--asset-id` selects a
subset; `--columns`, `--column-spacing` and `--row-spacing` change the layout.
It saves a real design, its command script and a host verification receipt.
Expected roles and body counts come from the selected profile recipes; the current
all-model expectation is 270 closed bodies. Fixed-lead
models omit optional lead-end connector boxes; serial models retain their socket
bodies and add no attached wires. The receipt, not the expectation, records
observed success. Screenshot evidence must be
captured from that CAD model; annotations are actual design geometry.

## Commands and saved instances

Commands expose lookup, inspection, recipes and insertion. The host draws forms
from schemas; programmatic calls never open dialogs. Geometry comes from host
commands rather than a plugin mesh. Owned cut/join targets and colors affect
only bodies created by that insertion.

The current edition writes a per-instance JSON snapshot with `storage.put_blob`,
then references it from an index through `storage.patch`, within the host command
transaction. The snapshot contains the selected asset, only its referenced source
records, data revision, parameters, executed recipe and created feature/body IDs.
Attached horns add independent accessory facts, documented compatibility and
placement with their sources and transform. Detached horn references store their
own record, parameters and source annotations without inventing an installed pose.
It does not include unrelated catalog sources or download vendor media.

Snapshot blobs are indexed under `plugins/servo-assets/blobs/instances/` in the
design. Plugin updates do not resize earlier instances. Removing the plugin
leaves ordinary geometry and saved snapshots; features are not dependent on a
runtime catalog lookup. This uses existing plugin storage, without a new core
design-format field. The legacy `commands@0.1` edition returns the snapshot and
sets `metadata_saved_in_design: false`, rather than embedding storage in the design.

## Catalog identity, evidence and pack limits

| Field | Meaning |
|---|---|
| Stable asset ID | Identity of the selected source record or physical variant |
| Manufacturer/model | Product identity, insufficient to establish interchangeability |
| `product_revision` | Evidenced physical revision, or null when unknown |
| `data_revision` | Revision of catalog transcription and interpretation |
| Seller item | Seller, SKU and URL, independent of manufacturer revision |
| Catalog release | Versioned distribution of merged records |
| Representation/frame | Geometry meaning, coordinate convention and explicit omissions |

Each dimension stores nominal millimeters, optional tolerance and per-field
source/locator/method/interpretation. Sources store publisher, title, URL and
retrieval date. Direct dimensions, specification text, measurements, derived
coordinates and qualitative observations stay distinct. Missing tolerance is
unknown, not zero. Derived bounds record their inputs; assumed symmetry is not
an additional measurement. Source conflicts stay attached to their observations.

Eight servo packs and four separate horn packs use bounded schema 1 records.
Each catalog validates packs independently, then merges with duplicate identity
and source-ID rejection. A pack is at most 512 KiB; each merged
input is bounded to 16 packs/2 MiB, with record-count limits. Pack identities and
releases remain data, while command IDs remain stable. Each per-asset description
and insertion snapshot uses only referenced sources.

The manifest/schema index is generated at build time, with the host's 16 KiB
budget enforced; the current compact manifest is 16,260 bytes against 16,384.
Dropdowns show readable model names and use compact deterministic
`ref-` selector keys. The catalog rejects key collisions and reserves that prefix.
Canonical asset IDs remain accepted by direct calls and are always used in saved
snapshots, instance indexes and replies. Selector keys do not imply mechanical
compatibility. Larger future catalogs still need separate bounded UI catalogs or
query-backed selectors before the manifest limit is reached. Separate schema
resources and paginated installation are design directions, not current features.
New geometry adapters need validation; adding data must not enable arbitrary
external code or silently infer an undocumented interface.

## Future assets and interfaces

The following are extension work, not completed features:

- MG995 after a suitable primary dimensional source is available; DS3218 and Futaba S3003 with exact manufacturer/revision evidence.
- Additional STS3215 tooling/specimen variants and measured port/case registration; current C001 light-guide evidence must not be generalized to other suffixes.
- General through/blind hole semantics and validated screw-engagement allowances; current oriented mounting groups preserve source thread/depth callouts without inferring screw lengths.
- Full output interfaces including locating diameters, axial contact surfaces, flange patterns and additional outputs.
- Further horn variants, complete source-defined contours/mating geometry and motion envelopes. Current references preserve source identity, known holes and explicit fit/pose status; missing geometry remains absent. Tooth count or JR/Futaba electrical wording alone remains insufficient.
- Full detachable-port envelopes and measured poses, routed harnesses, pass-through openings, multi-segment cable planning and motion/tool-access envelopes; current STS port counts are only the initial interface data.

The first ROBOTIS 330, XM430 and XM540 representatives use several case mounting
faces and separate main/idler poses. Normal and idler source records are separate
accessories; only supported solids with documented poses attach automatically.
Additional horns, frames, retention hardware and exact insertion
envelopes remain future work. Detachable connector ports
are separate from connected harnesses and mechanical routing through the hollow
idler. Port count is not connected bundle count. TTL and RS-485 variants retain
their distinct contact counts and independently selected profiles.

QDD is an actuator approach, not a shared geometry category. Future entries require
manufacturer, exact model, source revision and supported physical revision.
Cylindrical cases, hollow outputs, bolt circles and multiple interfaces should fit
the same datum model without inheriting micro-servo ears. Primary source links and
variant boundaries are recorded in [sources.md](sources.md).

## Contribution boundary

Add documented facts and original geometry, with seller/specimen/batch identity,
measurement method and uncertainty where applicable. Keep vendor and revision
variants explicit. Never copy commercial CAD models or obtain dimensions through
another CAD product. Vendor PDFs/images remain research references, outside the
package unless appropriately licensed. Source links, numeric facts and original
plugin geometry are sufficient for this catalog.
