# ROBOTIS actuator reference sources

The ROBOTIS servo packs in catalog release `2026-11-19.8` retain eight named actuator
profiles. Separate selectable horns are covered by the linked accessory audits.
Sources were checked on 2026-11-19. Numeric drawing facts, derived datum
coordinates and visual planning geometry remain separate in each record.
Document dates and catalog revisions do not identify a factory production revision.

| Family | Implemented profiles | Main case in CAD X x Y x Z |
| --- | --- | --- |
| X330 | XL330-M077-T, XL330-M288-T, XC330-T288-T, XC330-M288-T | 34 x 20 x 23 mm; source-depicted output flange projects another 3 mm |
| XM430 | XM430-W350-T, XM430-W350-R | 46.5 x 28.5 x 34 mm |
| XM540 | XM540-W270-T, XM540-W270-R | 58.5 x 33.5 x 44 mm |

The CAD convention places the long case direction along +X, the case width along
+Y, the main output toward +Z and the rear idler toward -Z. The case minimum
corner is `(0,0,0)`. This rotates the manufacturer's usual W x H x D presentation;
it does not change the dimensions. Main and idler datum lines are distinct named
interfaces even when they share a coaxial position.

The default adds no connected cable, mating plug, mounting screw, external frame
or separately selected horn/idler assembly. It preserves each existing source
depiction; the X330 output face may include a plastic normal horn. Small interface markers and socket shells
can still contain explicitly approximate contours. They are editable references,
not manufacturing models or guaranteed interference envelopes.

## XL330 and XC330

The [ROBOTIS X330 brochure, page 5](https://www.hannovermesse.de/apollo/hannover_messe_2025/obs/Binary/A1424108/1424108_05023972.pdf)
contains one mechanical drawing and a table explicitly listing all four selected
models, plus XC330-M181-T and XC330-T181-T. Only the four implemented names are
offered as assets. Their common drawing does not merge their motor, gear, voltage
or protocol specifications.

The drawing establishes a 20 mm width, 34 mm length, 23 mm main-case depth,
3 mm output-face projection and 16 mm output-flange diameter. The output axis is
9.5 mm from the shaft-near short end and follows the drawn transverse centerline.
In the CAD frame, the main datum is `(9.5,10,23)` toward +Z and the rear datum is
`(9.5,10,0)` toward -Z. The 26 mm overall depth includes the output wheel face.

Front and rear frame patterns have 16 x 30 mm center spacing. Each pattern has
four reference centers. Their case coordinates `(2,2)`, `(2,18)`, `(32,2)` and
`(32,18)` use symmetric registration of that pattern on the 20 x 34 mm outline.
The brochure does not independently tolerance the resulting 2 mm edge offsets.
The two planes preserve that derivation and its limitation in their evidence.

The [official frame assembly illustration](https://emanual.robotis.com/assets/images/dxl/x/x330/xl330_assembly_integrated.png)
specifies **M2 tapping screws** for front and back frames. This describes the
fastener, not a recovered machined metric thread or a 2 mm pilot drill. The pack
records the nominal screw diameter, with unknown pitch and engagement depth.
Visible 1.6 mm pilot recesses and their short depths are planning geometry.
The small wheel-face mating pattern is also approximate; no sourced bolt circle
or manufacturing hole depth is asserted for it.

The [HNX330-N101 accessory description](https://robotis.com/shop/item_export_only.php?it_id=903-0314-000)
and its exploded illustration distinguish the selected metal horn from the
actuator underneath. They do not prove the exact physical identity or factory
SKU of the brochure's plastic output-face depiction. That original diameter 16,
3 mm projection remains when `horn_id` is omitted or `none`. Selecting either
X330 normal horn replaces only the generated `output-flange` display role,
avoiding a duplicated plate. This is not a physical gear-removal instruction or
a claim about factory packing. The [X330/X430 horn audit](horn-robotis-430-audit.md)
records both source variants and the remaining identity/seat limits.

The rear ring's 12/6 mm diameters and 1 mm recess are visual planning choices
because the brochure does not call them out. The optional X330 idler/cap remains
a separate assembly reference, without a selected idler horn asset in this release.
A rear-axis reference does not imply an installed second horn.

The [XL330-M288 manual](https://emanual.robotis.com/docs/en/dxl/x/xl330-m288/)
and [XC330-T288 manual](https://emanual.robotis.com/docs/en/dxl/x/xc330-t288/)
identify the three-contact TTL interface: GND, VDD and DATA, using JST EH headers
and mating housings. Product and rear views show two opposite-side sockets.
Socket count and contact count are recorded separately from cable bundles;
neither socket acquires a permanently attached harness. Socket dimensions,
positions, pin dimensions and ordering, latch shape and insertion clearances remain
planning geometry. Contact-marker indices count the visible contacts; they do not
assign electrical pin numbers. The other selected model manuals are linked in
their own records.

The manual's shared full-drawing endpoint, `download.php?no=1986`, returned a
404 page during the original actuator audit, so its source record uses the
accessible manufacturer brochure. The later horn audit reached download 1986
through the official `en.robotis.com` drawing index and inspected its public PDF.
That assembly reference does not silently replace the selected actuator source
profile or supply unknown mating geometry from a CAD model.

## XM430

The [official XM430-W350 manual](https://emanual.robotis.com/docs/en/dxl/x/xm430-w350/)
links [ROBOTIS drawing download 157](https://www.robotis.com/service/download.php?no=157).
The inspected manufacturer sheet is titled **XM430/XH430 (HINGE)** and dated
2019-03-19. The W210 manual links the same mechanical reference; this is recorded
as a cross-reference, not an additional W210 asset or an electrical alias.

The main and rear datum axes share `(X,Y)=(11.25,14.25)`. Their reference planes
are Z=34 and Z=0. A reference point on this line is not a recovered spline root
or horn seating plane. Bare coupling diameter, tooth form and tip height remain
unknown, so the formal output records contain no invented shaft-height interval.

Four mounting planes expose fourteen source-located case taps. The following
table gives world-coordinate sets; repeated values form all listed combinations.

| Face | World coordinates in mm | Count |
| --- | --- | ---: |
| Main face, +Z | X=19.25; Y=3.25 or 25.25; Z=34 | 2 |
| Side, -Y | X=15.25 or 39.25; Y=0; Z=11 or 23 | 4 |
| Side, +Y | X=15.25 or 39.25; Y=28.5; Z=11 or 23 | 4 |
| Far end, +X | X=46.5; Y=6.25 or 22.25; Z=11 or 23 | 4 |

Case tap callouts are **M2.5 x 0.45, DP 3.0 (MAX.)**. The maximum is not a minimum
usable thread engagement or a screw-length recommendation. Reference circles
use nominal thread-major diameter; they are not clearance or tap-drill holes.
Two additional near-end pockets are visible in the model as previews, but their
transverse registration is not separately dimensioned in the selected view.
They do not appear in a formal mounting group.

The HINGE sheet depicts fitted accessory horns. Its eight M2 x 0.4 holes on
PCD 16 belong to those accessories and are excluded from bare-case groups and
geometry. The model's small main coupling and rear annulus are explicitly
provisional shape markers. Selecting HN12-N101 or HN12-I101 adds the separate
source-defined accessory reference with an assumed pose. HN12-I101 replaces the
generated `idler-seat` marker; this display policy does not prescribe removal of
physical bearings or gears. See [the horn audit](horn-robotis-430-audit.md).

The T profile has two three-contact TTL bus sockets; the R profile has two
four-contact RS-485 bus sockets. The [back-case routing illustration](https://emanual.robotis.com/assets/images/dxl/x/wiring_through_back_case.jpg)
and [internal-header photograph](https://emanual.robotis.com/assets/images/dxl/x/assembly/common/x-series_cable_assembly.png)
show the detachable connections and the alternate hollow-back routing path.
That mechanical route is not a third electrical port. Shell position, contact
placement and insertion space are approximate; the models contain no pigtails
or automatically routed cables.

## XM540

The [current XM540-W270 model reference](https://docs.robotis.com/docs/dxl/model_reference/x_series/xm_series/xm540-w270/)
corroborates the 33.5 W x 58.5 H x 44 D mm case. The inspected
[manufacturer drawing hosted by Best Technology](https://www.besttechnology.co.jp/modules/knowledge/gate.php/XM%2CH%2CD-540.N101.I101.pdf?_noumb=&openfile=X540_dimension.pdf&refer=Dynamixel+XM540-W150&way=attach)
is titled **XM540/XH540/XD540 HINGE**, dated 2019-03-18 and marked sheet 1 of 2.
The mirror served a one-page PDF. The current official download endpoint returned
404 during this audit; mirror provenance and the incomplete sheet set remain
explicit. The W150 manual's common drawing link is a mechanical cross-reference,
not a silently added W150 profile.

The main and rear datum axes share `(X,Y)=(13.75,16.75)` on reference planes Z=44
and Z=0. Bare output diameter, tooth form, tip height and fitted-horn seating
geometry remain unknown. Flush main-interface and rear-routing markers are
planning shapes, not recovered spline surfaces.

Five mounting planes expose sixteen source-located case taps.

| Face | World coordinates in mm | Count |
| --- | --- | ---: |
| Side, -Y | X=19.75 or 51.75; Y=0; Z=14 or 30 | 4 |
| Side, +Y | X=19.75 or 51.75; Y=33.5; Z=14 or 30 | 4 |
| Near end, -X | X=0; Y=6.75 or 26.75; Z=30 | 2 |
| Far end, +X | X=58.5; Y=6.75 or 26.75; Z=14 or 30 | 4 |
| Main face, +Z | X=24.25; Y=3.25 or 30.25; Z=44 | 2 |

The case thread callout is **M2.5 x 0.45, DP 4.0 (MAX.)**. Maximum depth is not
guaranteed usable engagement. The case taps are distinct from the four long
assembly fasteners and from the illustrated installed-horn patterns. Neither the
horn's PCD 22 thread pattern nor its PCD 16 small-hole pattern enters a bare-case
mount group. Visible major-diameter cylinders mark positions without reproducing
thread geometry.

The T and R profiles each model two detachable bus sockets, with three TTL or
four RS-485 contacts per socket. Additional source-only interfaces remain separate:

| Interface | Documented contacts | Treatment |
| --- | ---: | --- |
| External I/O | 5 | One header is shown in the source diagram; its case pose is unknown, so no exterior socket is invented |
| Dual Joint | 3 | Contact specification is known; in-product quantity and case pose remain unknown |

The separate rear idler, bearing and cap are assembled around the documented
routing path. An available mechanical pass-through does not establish another
electrical port, an always-open through-hole across the actuator, or a permanently
installed rear horn.

The selectable HN13-N101 and HN13-I101 references use the assembled drawing's
documented outer flange origins at `(13.75,16.75,46.6)` and
`(13.75,16.75,-2.5)`. Their normal/idler compatibility is documented separately.
Selecting one replaces the corresponding generated output/routing marker.
Complete loose-part back profiles, mating seats and retaining hardware remain
incomplete; source-located outer faces do not establish a verified contact stack.
See [the XM540/STS horn audit](horn-robotis-540-sts-audit.md).

## Extending these records

Keep fastener type, nominal thread-major diameter, pilot diameter, thread pitch,
depth limit and usable engagement separate. Likewise, distinguish a physical
socket, contacts in that socket, connected bundles and conductors in a selected
harness. Data that remains unknown should not become a zero value or inherit a
nearby model's dimensions.

New measured variants need specimen identity, method, uncertainty and an explicit
relationship to the source profile. Saved instances retain their own catalog
snapshot. Vendor PDFs, images and CAD files are not redistributed in these packs;
their geometry instructions and documentation are original.
