# XM540 and STS3215 horn source audit

Strict correction audit, retrieved and visually inspected on 2026-11-19. All
lengths are millimetres. The four horn records in
`catalog/horns-robotis-540-sts.json` now have `data_revision: 2`. This audit does
not claim a new physical hardware revision. The legacy ROBOTIS catalog IDs keep
their original `reference-20190318` suffix for identity continuity; individual
part dimensions now explicitly cite the later 22-May-2019 drawings.

The current data contains three partial major-profile solids and one
`dimensions_only` record. No estimated bore, hub length, polygon vertex or
installed STS coordinate is retained. Small omitted features and unavailable
cavities remain explicit. These references are not complete manufacturing or
mating models. Vendor drawings and photographs are not distributed in the pack.

The earlier v0.7 horn galleries predate this correction. Their closed-body and
bounding-box checks do not validate the corrected source interpretation; fresh
evidence is needed after the runtime gates are updated.

## Official individual HN13 sources

The [ROBOTIS Horn index](https://www.robotis.com/service/downloadpage.php?ca_id=7020)
links the following individual drawings. Its download endpoints return HTML
wrappers containing the manufacturer-selected PDF links; the wrappers are not
PDF files.

| Part | Official download wrapper | Published PDF, rendered for this audit |
| --- | --- | --- |
| HN13-N101 | [Download 1737](https://www.robotis.com/service/download.php?no=1737) | [HN13_N101.pdf](https://www.dropbox.com/s/ry89g6jkfavwr6l/HN13_N101.pdf?dl=1), 22-May-2019, sheet 1/1 |
| HN13-I101 | [Download 1753](https://www.robotis.com/service/download.php?no=1753) | [HN13_I101.pdf](https://www.dropbox.com/s/v6cza08i9oxrv5h/HN13_I101.pdf?dl=1), 22-May-2019, sheet 1/1 |

The inspected PDF SHA256 values are
`761a049309bce2242ab295332a488d024c0a68b5af8560c892f016db3ba93f44`
for N101 and
`5f4aed8ba809107aa9fca20ece692095d3304f322f11548d387f2ac966c6a1aa`
for I101. Research copies stay outside the repository.

Named compatibility is independently supported by the official
[HN13-N101 product](https://e-shop.robotis.co.jp/product.php?id=476),
[HN13-I101 product](https://e-shop.robotis.co.jp/product.php?id=146), and
[XM540 assembly manual](https://emanual.robotis.com/docs/en/dxl/x/xm540-w270/).
The idler is a bearing/routing interface on the opposite side, not another
powered spline. Bearing, cap, washer and retaining fasteners are separate parts.

## HN13-N101: published major outer profile

The individual drawing explicitly gives flange diameter 26, flange thickness
2.6, axial extent 4.9 and boss diameter 10. The boss projection is
`4.9 - 2.6 = 2.3`. It also specifies eight **M2.5 × 0.45 TAP THRU** holes on
pitch circle 22 and four **diameter 2 HOLE THRU** holes on pitch circle 16.
These callouts replace the former guessed blind holes in the current reference.
The DC13 inset gives 32 teeth, a 90-degree angle and reference diameters 8 and
7.4. Section A-A gives pocket depth 3.7. No smooth substitute bore or tooth
surface is constructed from these incomplete mating details.

The partial solid uses local outer flange face Z0, flange Z−2.6 to 0 and
outward boss Z0 to 2.3. Hole centers are the four cardinal points and four
45-degree points at radius 11; the plain inner holes are cardinal at radius 8.
All cylinders use published values or these direct arithmetic derivations.
Tapped holes are shown as nominal-major-diameter reference cuts, with no
helical or tap-drill surface claim.

The unlabelled retaining-bore diameter, rear lip/recess details, small edge
features and complete mating cavity are omitted. The reference therefore shows
the supported major exterior and hole pattern, not the complete manufactured
horn. In particular, a missing central cavity must not be interpreted as a
solid center in the real accessory.

## HN13-I101: dimensions only

The individual section gives flange diameter 26 and thickness 2.5; eight
M2.5 × 0.45 holes on pitch circle 22 have specified depth 2.5. Section A-A
explicitly shows a **diameter 10 passage** inside a **diameter 12 tube**, plus
a diameter 18.2 callout in the annular region. It does not dimension the tube's
axial length or all shoulder and hook positions.

The earlier diameter 12 through opening was an unsupported estimate and has
been removed. The record now has `status: dimensions_only` and `parts: []`.
Its published dimensions and hole centers remain available as dimensional
references. Neither bearing fit nor missing tube length is inferred from a
bearing part number or from the illustration's pixel scale.

## Assembly source and conflicting callouts

The older [ROBOTIS-authored HINGE drawing, 2019-03-18, mirrored by BestTechnology](https://www.besttechnology.co.jp/modules/knowledge/gate.php/XM%2CH%2CD-540.N101.I101.pdf?_noumb=&openfile=X540_dimension.pdf&refer=Dynamixel+XM540-W150&way=attach)
remains the case-frame source. The shared axis is X13.75, Y16.75, with rear
case Z0 and front case Z44. Its installed outer-flange projections establish
normal `[13.75, 16.75, 46.6]` in +Z and idler `[13.75, 16.75, -2.5]` in −Z.
These outer-face poses do not establish every inner contact plane or neutral
angular phase. The dimensions-only idler has no automatic 3D attachment.

The March assembly calls N101 holes depth 2.5 maximum, whereas its May
individual drawing calls them through. Historical limits remain in explicitly
dated dimension fields; they are not used as the May part's thread depth. This
is a source-document discrepancy, not proof of a changed physical revision.
The current normal model follows the individual-part through callouts.

## FEETECH sources and literal diameter references

Each of the following accessory pages was separately inspected as a rendered
image. Their shared dimensions are not inferred from a matching tooth count.

| A/0 profile | Manufacturer specification hosted by Akizuki | Inspected accessory page |
| --- | --- | --- |
| C001 | [STS3215-C001 SEPC](https://akizukidenshi.com/goodsaffix/STS3215-C001%20SEPC.pdf) | Physical 7, printed 7/8, section 10, dated 2020-03-28 |
| C018 | [STS3215-C018](https://akizukidenshi.com/goodsaffix/STS3215-C018.pdf) | Physical 7, printed 7/8, section 11, dated 2023-07-20 |
| C044 | [ST-3215-C044 SEPC](https://akizukidenshi.com/goodsaffix/ST-3215-C044%20SEPC.pdf) | Physical 11, printed 7/8, section 10, dated 2025-04-15 |
| C046 | [ST-3215-C046 SEPC](https://akizukidenshi.com/goodsaffix/ST-3215-C046%20SEPC.pdf) | Physical 12, printed 7/8, section 10, dated 2025-04-15 |

| Explicit source dimension | Main disc | Rear disc |
| --- | --- | --- |
| Outside diameter | 19.95 | 19.95 |
| Hub diameter | 9 | 9 |
| Plate / total height | 2.5 / 4.5 | 2.1 / 3.1 |
| Frame pattern | Four M3 on pitch circle 14 | Four M3 on pitch circle 14 |
| Center information | Clearance 3.2, pocket depth 3.6, nominal `25T (5.9)` | Plain through passage 6.05 |

The source raster depicts polygonized outside edges, pitch circles and even
nominally round holes. The explicit diameter symbols support literal circular
major-profile references; the raster does not define physical facet counts,
flat angles or polygon vertices. No vertices are traced or estimated. If a
future manufacturer source establishes actual flats, it needs its own profile.

General notes specify unspecified dimensional tolerance ±0.03, unmarked
chamfers 0.3 and 6061-T6 with natural anodizing. The 5.9 serration size is kept
as a nominal size reference, with no bore or tooth-fit tolerance assigned.

## FEETECH source-to-model mapping and assembly boundary

Both local references put the outer plate face at Z0. The main plate occupies
Z−2.5 to 0 and its diameter 9 hub extends to Z−4.5; its hub projection is
`4.5 - 2.5 = 2`. The rear plate occupies Z−2.1 to 0 and its diameter 9 hub
extends to Z−3.1; its projection is `3.1 - 2.1 = 1`. No numerical overlap or
extra axial margin is added. The rear diameter 6.05 plain passage is retained.

The main center cavities are omitted entirely. In particular, `25T (5.9)` is
not turned into an invented smooth diameter 5.9 pocket. Its source-backed 3.2
clearance and 3.6 depth remain facts but are not silently expanded into a complete
central profile. Section X-X supports the four through-plate threaded
penetrations. Their displayed diameter 3 cylinders are nominal M3-major
references. Pitch, class, helical surfaces, usable engagement and chamfers
remain omitted.

Each A/0 specification documents compatibility with the matching discs, but
none locates the installed outer face completely in the case frame. All eight
A/0 mounts therefore have `placement_basis: unknown`, `origin_mm: null` and an
empty replacement list. The previous case-midplane and tip-plus-plate placement
coordinates have been removed. No automatic STS assembly is authorized by these
records.

The [C001 lightguide change notice](https://akizukidenshi.com/goodsaffix/STS3215_design_change_20251229.pdf)
does not establish the changed case's horn fit or seat. Its two unsupported
compatibility proposals have been removed, rather than left as selectable
bindings.

## Data boundaries and schema needs

The corrected pack contains four horns, twelve documented compatibility
bindings and twelve sources. Three horn records contain a partial published
major profile; I101 contains no 3D parts. HN13 outer-face placement is documented;
all STS installed poses are unknown.

The runtime must distinguish a dimensional sketch, a partial major profile and
a complete part. A known diameter may be drawn as a labelled dimension circle
without claiming that every line is a finished part outline. Missing geometry
must not be replaced by a dummy solid. Unknown installed poses remain nullable
and cannot trigger attachment or replacement of actuator parts.

Further useful schema fields are explicit hole termination (`through`, `blind`
or `maximum depth`), separate nominal thread and clearance meanings, typed
counts/angles for serration references, per-primitive links to dimensions or
arithmetic derivations, omitted-feature scope, and source-content hashes. Those
would make the distinction between a source fact and a constructed feature
machine-checkable. A tooth count or size label alone still cannot establish
interchangeability, mating fit or a neutral phase.
