# Servo dimensional sources

Retrieved and visually checked on **2026-11-19**. Dimensions below are millimeters. This is a transcription of published facts and their limits, not a claim that every unit sold under a model name matches them.

Version **0.9.1** uses merged catalog release **`2026-11-19.10`** with 29 servo source profiles, 28 integrated models and 78 source records across eight servo packs. Four separate horn packs add 18 horn variants, 28 compatibility records and 28 horn source records. Each asset/accessory profile has its own data revision. No physical product revision has been established for these records; a preview recipe or data correction does not establish one.

Eight ROBOTIS profiles add source-defined case mounting faces, main/idler axes and detachable bus sockets. Their dimensions, thread callouts, connector topology and remaining approximations are documented in the [ROBOTIS source audit](robotis-sources.md).

Original PDFs and images are research references. They are not distributed with the plugin. Geometry recipes and catalog text are original. No third-party CAD model supplies these dimensions.

The integrated collection adds MG996R, SG5010, MG90D, MG92B, FS90R, FT90B, FS0307, FS5103B, FT1117M, HS-311 and HS-55 to the original SG90 Analog, SG92R, MG90S and FS90 models. PREVIEW parts remain separate from engineering facts. FS90R and FT90B use independently inspected model-specific drawings. Hitec output XY is located, while bare-shaft tip heights remain unknown. MG996R's configure body and independent drawing XY remain UNREGISTERED source frames. Five STS3215 case/source profiles add opposed main/idler visualization and two detachable three-contact sockets, with factual case-relative axes still unknown. The separate Aus Electronics Direct SG90 provides envelopes and an output axis, without a shaft solid, mounting pattern or integrated model.

## TowerPro configure tables

The TowerPro configure profiles use these manufacturer pages:

| Model | Manufacturer page | Akizuki seller item |
| --- | --- | --- |
| SG90 Analog | [SG90 Analog](https://towerpro.com.tw/product/sg90-analog/) | [108761](https://akizukidenshi.com/catalog/g/g108761/) |
| SG92R | [SG92R](https://towerpro.com.tw/product/sg92r-7/) | [108914](https://akizukidenshi.com/catalog/g/g108914/) |
| MG90S | [MG90S](https://towerpro.com.tw/product/mg90s-3/) | [113227](https://akizukidenshi.com/catalog/g/g113227/) |
| MG90D | [MG90D](https://towerpro.com.tw/product/mg90d-2/) | [113226](https://akizukidenshi.com/catalog/g/g113226/) |
| MG92B | [MG92B](https://towerpro.com.tw/?product=mg92b) | [113228](https://akizukidenshi.com/catalog/g/g113228/) |
| SG5010 | [SG5010](https://towerpro.com.tw/product/sg5010-4/) | [108913](https://akizukidenshi.com/catalog/g/g108913/) |
| MG996R | [MG996R](https://towerpro.com.tw/product/mg996r/) | [112534](https://akizukidenshi.com/catalog/g/g112534/) |

Their shared [manufacturer dimensional key](https://towerpro.com.tw/wp-content/uploads/2014/07/%E5%B0%8F%E9%A6%AC%E9%81%94%E5%B0%BA%E5%AF%B8%E6%A8%99%E7%A4%BA%E5%9C%96B.jpg) defines the table letters:

| Letter | Drawing interpretation |
| --- | --- |
| A | Base to output shaft top |
| B | Housing length, excluding ears |
| C | Base to gear-tower housing top, excluding output shaft |
| D | Housing width |
| E | Overall length including mounting ears |
| F | Base to underside of mounting ears |

The drawing does not make C the height of the rectangular lower case. No tolerance is printed for these table dimensions.

| Model | A | B | C | D | E | F |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| SG90 Analog | 30.3 | 22.7 | 27.0 | 12.2 | 32.3 | 17.0 |
| SG92R | 34.5 | 22.8 | 26.7 | 12.6 | 32.5 | 16.0 |
| MG90S | 32.5 | 22.8 | 28.4 | 12.4 | 32.1 | 18.5 |
| MG90D | 32.5 | 22.6 | 28.5 | 12.0 | 31.5 | 19.8 |
| MG92B | 35.0 | 22.6 | 31.0 | 12.0 | 31.5 | 22.8 |
| SG5010 | 44.1 | 40.7 | 38.0 | 20.0 | 55.0 | 27.8 |
| MG996R | 42.7 | 40.9 | 37.0 | 20.0 | 54.0 | 26.8 |

These configure tables support nominal housing and overall reference boxes. The tables alone do not establish mounting-hole pitch or diameter, ear thickness, shaft-axis location, shaft diameter, rectangular shoulder height or horn geometry. MG996R has additional drawing-relative XY facts from an independent source, described below; these do not register its configure body to that drawing.

Six configure-only TowerPro `main` output records have no established axis pose. A known overall shaft-top height does not locate the shaft in X and Y. The factual output-reference command does not substitute a housing-box center or another seller's SG90 or MG90S offset for the missing pose.

### TowerPro PREVIEW geometry

The seven TowerPro references combine A-F-backed envelope sizes and height extents with recognizable approximate parts. The original SG90 Analog, SG92R and MG90S recipes use the rules in the table below; the four additional profiles keep their own explicit planning values in the catalog. Their visible mounting slots, shaft locations, lower-case shoulder, upper-cover contours and cable exits use declared round-number planning assumptions. These positions are not extracted from unscaled photographs and are not measurements of a factory variant.

The PREVIEW geometry is marked APPROX and stored separately from factual dimensions, mounting groups and output interfaces. The visual models do not provide verified screw templates or establish shaft placement. The source conflicts above remain unresolved. A future measured record must identify the actual specimen, method and uncertainty before an assumed preview datum can become documented geometry.

The preview origin is the case-minimum corner. B, C, D, E and F below are the published table letters; combining them into these placements does not make the placements source dimensions.

| Preview detail | Declared planning rule |
| --- | --- |
| Mounting ears | Symmetric extension `(E-B)/2` at height F, with assumed 2 mm thickness |
| Slot centers | Midpoints of the ear rectangles, with assumed diameter 2 mm and mouth width 1 mm |
| Shaft center and diameter | `(D/2, D/2)` in the case frame; assumed diameter 5 mm |
| Plain lower-case shoulder | Assumed 4 mm below C; the exposed shaft interval follows A-C |
| Upper cover | Simplified two-lobe contour; secondary diameter 6 mm and X offset 5 mm are assumptions |
| Cable outlet | X=0 face, Y=D/2, assumed height 2 mm above the case base |

These round-number choices distinguish approximate parts from documented datums. Their tolerances are unknown, not zero. They do not provide a conservative collision envelope or mechanical compatibility between profiles. MG90D, MG92B and SG5010 also have unlocated factual axes; MG996R's preview assumes a registration between two unregistered source frames, as described below.

The 15 conventional fixed-pigtail references default to five structural bodies and a 3 mm marker made of three conductor bodies, with no connector. This yields eight bodies, or five with `include_cable: false`. `include_connector: true` with the cable enabled adds one illustrative box, yielding nine. These are display choices, not evidence about the supplied lead length or connector dimensions.

## Conflicting descriptions

The same pages contain different headline dimensions:

| Model | TowerPro headline dimension | Akizuki structured dimension |
| --- | --- | --- |
| SG90 Analog | 23 x 12.2 x 29 | 22.8 x 12.6 x 26.7 |
| SG92R | 23 x 12.2 x 27 | 22.8 x 12.6 x 26.7 |
| MG90S | 22.8 x 12.2 x 28.5 | 22.8 x 12.4 x 28.4 |

Akizuki's MG90S prose repeats 22.8 x 12.2 x 28.5, while its structured fields differ. These disagreements are recorded as source observations. They do not prove separate tooling, batches, or physical revisions. The headline length/width/height text alone does not establish which protrusions its height includes.

The [Akizuki SG90 reference sheet](https://akizukidenshi.com/goodsaffix/SG90_a.pdf), page 1, gives another set: housing length 22.2, width 11.8, overall ear length 32.2, base to ear underside 16, and height 31 to the fitted horn. It describes these dimensions as approximate. The drawing does not provide a mounting-hole center distance or diameter. In particular, 22.2 is the case length, not the screw pitch.

The [SG92R sheet distributed by Adafruit](https://cdn-shop.adafruit.com/product-files/5592/C17481_SG92R_datasheet.pdf) contains a single specification page with 23 x 12.2 x 27 and no dimensional drawing. It does not fill the missing mounting geometry. The [Adafruit product page](https://www.adafruit.com/product/169) separately lists 23 x 11 x 29; this is another unresolved description, not permission to combine sources into a fabricated interface.

The additional manufacturer headlines also differ from their configure tables: MG90D 22.8 x 12.2 x 28.5, MG92B 22.8 x 12 x 31, SG5010 40.6 x 20.5 x 38, and MG996R 40.7 x 19.7 x 42.9. The selected A-F values remain explicit; headline disagreement does not establish a factory revision.

Electrical JR/Futaba connector wording does not establish spline compatibility. The TowerPro SG92R page mentions fitting a Futaba servo arm without supplying the spline dimensions needed for a complete interface. The catalog does not infer a tooth count or documented compatibility edge from that statement. The separate AU-derived horn proposals have explicitly unverified fit; see [the micro-horn audit](horn-micro-audit.md).

## MG996R: independent drawing XY, unregistered body

[Akizuki's linked MG996R sheet](https://akizukidenshi.com/goodsaffix/mg996r.pdf), page 2, independently supplies four circular slot pockets of diameter 3.90 on 48.60 x 10.00 spacing. An origin at the midpoint of the output-near column gives centers `(0,+/-5)` and `(48.6,+/-5)`. The shaft axis is `(14.25,0)` along +Z, derived as `48.60 - 34.35`. No dimensional tolerances are printed.

That drawing supplies no case-edge registration, housing/shaft-top heights or flange-height datum. Its page 1 headline is 40.7 x 19.7 x 42.9, which differs from the TowerPro configure profile B40.9/D20/C37/A42.7. The source does not prove the same physical revision. **The drawing's XY frame and configure-table body remain UNREGISTERED.** Z=0 is a drawing-reference plane, not an established shaft foot or a transferred configure F26.8 datum.

The factual mounting pattern and axis can therefore be used independently. The factual output path creates no shaft solid. The integrated PREVIEW centers the drawing pattern on the configure body by assumption, giving case-relative slot rows X=-3.85/44.75 and Y=5/15, with axis X=10.4/Y=10. Its vertical placements, case alignment, ear outline/thickness, cover, shaft diameter and cable exit remain planning geometry. These assumed coordinates do not complete the factual registration.

## FEETECH FS90: documented mounting pattern

The initial FS90 record follows [Akizuki's manufacturer datasheet](https://akizukidenshi.com/goodsaffix/fs90.pdf), page 2, section 6, supplied for [seller item 114806](https://akizukidenshi.com/catalog/g/g114806/).

| Feature | Nominal | Published tolerance |
| --- | ---: | --- |
| Housing length | 22.5 | +/-0.1 |
| Housing width | 12.1 | +/-0.1 |
| Base to rectangular case shoulder | 22.4 | +/-0.1 |
| Base to gear-tower top | 26.7 | +/-0.1 |
| Base to output shaft top | 29.7 | +/-0.1 |
| Base to ear underside | 15.7 | +/-0.1 |
| Base to ear top | 18.2 | +/-0.1 |
| Overall length including ears | 32.3 | +/-0.1 |
| Mounting-hole center distance | 27.7 | +/-0.1 |
| Mounting-hole diameter | 2.2 | Not stated |
| Shaft axis to near mounting-hole center, along length | 8.7 | +/-0.1 |
| Output spline outside diameter | 4.86 | Not stated |
| Output spline tooth count | 21 teeth | Not stated |

The pattern sketch uses mounting centers `(0, 0)` and `(27.7, 0)`, each with a reference circle of diameter 2.2. The servo drawing depicts open slots. The circles do not reproduce the slot perimeter or prescribe a drilling allowance or self-tapping screw. Both reference axes point along local +Z.

### FS90 output datum and height derivations

The FS90 output reference uses the same `near_mount_center` frame as the mounting pattern. The near mounting-slot center at the ear underside defines `(0, 0, 0)`. +X points to the far mounting center, and +Z points toward the output shaft. The output-axis line passes through `(8.7, 0, 0)` along +Z. The 8.7 offset is directly dimensioned; the shared centerline and chosen reference plane define the zero coordinates. Those zeros are datum definitions, not claims of zero manufacturing error.

| Output quantity | Nominal | Evidence and tolerance |
| --- | ---: | --- |
| Axis X from near mounting center | 8.7 | Direct drawing dimension, +/-0.1 |
| Axis Y in the mounting frame | 0 | Shared drawn centerline; transverse alignment tolerance not specified |
| Housing top above ear underside | 11.0 | Derived as 26.7 - 15.7; conservative independent worst-case bound +/-0.2 |
| Shaft top above ear underside | 14.0 | Derived as 29.7 - 15.7; conservative independent worst-case bound +/-0.2 |
| Housing-top-to-shaft-top interval | 3.0 | Derived as 29.7 - 26.7; conservative independent worst-case bound +/-0.2 |
| Shaft outside diameter | 4.86 | Direct diameter callout; tolerance not stated |

The 11.0 and 14.0 values are expressed in the ear-underside frame, not measured directly from that surface. Their +/-0.2 bounds add the two input limits; they are not separately published tolerances. The construction axis includes the portion of its line inside the housing. Its intersection with the datum plane is not the physical start of the shaft.

The output-reference command creates fixed construction markers, a datum axis, and a nominal diameter-4.86 shaft envelope over the 3.0 height interval. The envelope does not reproduce spline teeth, undercuts, or a horn seating surface. The drawing supplies neither a fitted-horn seating datum nor an angular phase for the spline at electrical neutral.

Subtracting ear underside from ear top yields nominal thickness 2.5. This is a derivation, with input tolerances, not a separately specified 2.5 +/-0.1 dimension. Assuming the symmetric case positioning shown in the drawing yields case length coordinates 2.6 through 25.1 and outer-ear length coordinates -2.3 through 30.0 from the first-hole origin. The integrated FS90 model uses this declared assumption for registration. The independent FS90 housing box remains in its own minimum-corner frame.

The spline outside diameter and tooth count are not a complete tooth profile or proof of compatibility with another manufacturer's horn. The initial asset does not generate spline teeth or a horn. Unspecified details include the exact ear perimeter, slot geometry, screw engagement, spline tooth form, and supplied horn geometry.

### Integrated FS90 model: facts and approximations

The integrated model uses the mounting frame defined above. Its visible parts do not have one uniform evidence status:

| Visible feature | Evidence or declared assumption |
| --- | --- |
| Mounting centers X=0 and X=27.7, Y=0 | Published pitch and common drawn centerline |
| Circular slot pockets, diameter 2.2 | Published diameter; no diameter tolerance specified |
| Shaft axis X=8.7, Y=0, along +Z | Direct axis offset and drawn centerline; datum-plane zero is a frame definition |
| Shaft envelope, diameter 4.86 and Z=11 through 14 | Published outside diameter and subtraction-derived heights; no spline teeth or horn seat |
| Rectangular body, 22.5 x 12.1 x 22.4 | Published sizes; placement between the mounting centers assumes symmetry |
| Body X=2.6 through 25.1, Y=-6.05 through 6.05 | Assumed symmetric registration, not directly dimensioned coordinates |
| Ear top Z=2.5 and body base Z=-15.7 | Ear thickness and datum conversion derived from published height callouts |
| Rectangular ear perimeter and 1 mm open mouth | Simplified visual geometry; mouth width and exact perimeter are not specified |
| Upper cover's two-lobe contour | Approximate two-cylinder construction; heights follow the drawing but the contour is not fully dimensioned |
| Part colors | Original role-identification colors, not material or finish specifications |

The nominal shoulder is Z=6.7, derived as 22.4 - 15.7, with a conservative independent worst-case +/-0.2 bound. The upper-cover top Z=11 and shaft top Z=14 use the derivations already listed. The approximate cover and slot contours are not conservative clearance bounds. Adding recognizable contours does not make their dimensions measured facts.

The model has no fitted horn. Its output-axis reference must not be read as a spline angular phase, neutral horn angle, or seating datum.

### Labels and evidence screenshots

`labels: "notes"` saves native model-space note callouts. The GUI displays them, but CLI snapshots omit these overlays. `labels: "front"` instead creates real text-outline curves and construction leaders in a separate front-view annotation sketch. A front snapshot can therefore show the labels without adding marks to the screenshot afterward. Annotation text distinguishes sourced shaft and slot dimensions from approximate alignment, cover, slot-mouth, and cable geometry. Label layout is illustration, not dimensional evidence.

Do not substitute the [older FS90-C datasheet distributed by Pololu](https://www.pololu.com/file/0J1435/FS90-specs.pdf), which specifies 23.2 x 12.5 x 22. Its model and dimensional record differ. The [Pololu product page](https://www.pololu.com/product/2818) is a separate seller observation.

## FEETECH expansion: model-specific source profiles

Each entry uses the exact Akizuki SKU, its linked manufacturer sheet and its own photograph. The first two drawings were checked independently; a matching FS90-like XY pattern is not inferred from their names.

| Model | Seller item | Manufacturer sheet | Photograph |
|---|---|---|---|
| FS90R | [113206](https://akizukidenshi.com/catalog/g/g113206/) | [Updated drawing](https://akizukidenshi.com/goodsaffix/fs90r_20201214.pdf) | [113206 photo](https://akizukidenshi.com/img/goods/L/113206.jpg) |
| FT90B | [114693](https://akizukidenshi.com/catalog/g/g114693/) | [FT90B-C001 V1.0](https://akizukidenshi.com/goodsaffix/ft90b.pdf) | [114693 photo](https://akizukidenshi.com/img/goods/L/114693.jpg) |
| FS0307 | [114807](https://akizukidenshi.com/catalog/g/g114807/) | [FS0307 V1.0](https://akizukidenshi.com/goodsaffix/fs0307.pdf) | [114807 photo](https://akizukidenshi.com/img/goods/L/114807.jpg) |
| FS5103B | [114802](https://akizukidenshi.com/catalog/g/g114802/) | [FS5103B V1.0](https://akizukidenshi.com/goodsaffix/fs5103b.pdf) | [114802 photo](https://akizukidenshi.com/img/goods/L/114802.jpg) |
| FT1117M | [114803](https://akizukidenshi.com/catalog/g/g114803/) | [FT1117M V1.0](https://akizukidenshi.com/goodsaffix/ft1117m.pdf) | [114803 photo](https://akizukidenshi.com/img/goods/L/114803.jpg) |

### FS90R and FT90B

The following values come from each sheet's page 2, section 6. Linear callouts explicitly carry +/-0.1 mm; slot and shaft diameters have no printed tolerance.

| Drawing quantity | FS90R | FT90B |
|---|---:|---:|
| Case length x width | 22.5 x 12.1 | 22.5 x 12.1 |
| Base to shoulder | 23.4 | 22.4 |
| Base to upper housing top | 27.7 | 26.7 |
| Base to shaft top | 30.7 | 29.7 |
| Base to ear underside / ear top | 16.7 / 19.2 | 15.7 / 18.2 |
| Ear overall length | 32.3 | 32.3 |
| Slot-center pitch | 27.7 | 27.7 |
| Circular slot-pocket diameter | 2.2 | 2.2 |
| Axis from near slot center | 8.7 | 8.7 |
| Spline outside diameter | 4.86 | 4.86 |

Both patterns use centers `(0,0)` and `(27.7,0)`, with the ear underside at Z=0 and the output axis `(8.7,0,0)` toward +Z. Subtracting each source's underside height gives shoulder Z=6.7, housing top Z=11, shaft top Z=14 and ear top Z=2.5. Each difference between independently toleranced +/-0.1 inputs has a conservative +/-0.2 bound. Neither source directly dimensions the symmetric case registration used by the integrated model; ear perimeters, narrow slot mouths and the two-lobe cover remain approximate.

FS90R page 1 specifies 21T(4.8 mm), while its updated page 2 drawing labels diameter 4.86. The catalog records this disagreement and uses the drawing value, without inventing a physical revision. FT90B's linked sheet identifies FT90B-C001 and document V1.0; those identifiers do not prove a universal FT90B production revision. Neither 21T specification establishes a mating tooth form or interchangeable horn.

### FS0307, FS5103B and FT1117M

| Model | Source-backed scalar dimensions | Missing registration and model limits |
|---|---|---|
| FS0307 | A20 x B8.3 x C17.3 housing spans | Four open ear slots are depicted, but no pitches, slot diameters, ear heights, shaft pose or bare-shaft dimensions are supplied |
| FS5103B | A40.8, B20.1, C38 upper-boss height, ear underside 26.8, four-slot pitches 49.5 x 10.0 | Slot diameter, ear thickness/overall length, case registration and shaft pose/dimensions are unknown; D49.5 is mounting pitch, not overall ear length |
| FT1117M | Case 30 x 12, upper-collar height 31.7, ear underside 20, overall ear length 40.5, spline outside diameter 5.8 | Hole pitch/diameter, ear thickness, shaft XY and bare-shaft top remain unknown;40.5 is ear length, not center pitch |

FS0307 page 1 uses FS0307, while the page 2 model line reads FS307. This is a recorded source label difference, not an inferred physical variant. The FS5103B and FT1117M vertical callouts terminate below the exposed spline. Do not use C38 or 31.7 as shaft-top or fitted-horn height. None of these three profiles emits a factual mounting group or located output axis.

Their integrated amber parts are original planning silhouettes. FS0307 uses planning ears with 23 mm pitch and diameter 1.6 pockets, shaft axis X5/Y4.15 and diameter 3/height 2. FS5103B retains source 49.5 x 10 pitch but assumes symmetric case registration, diameter 4 pockets, 57.5 overall ears and a shaft at X10/Y10.05 with diameter 6/height 4. FT1117M retains source 40.5 ear length and diameter 5.8 spline, but assumes 36 pitch, diameter 3 pockets and shaft X8/Y6/height 4. These numbers do not enter factual interface fields. The catalog records all other assumptions, including approximate shoulder partitions, covers and cable exits.

All five expansion photographs support one harness, three conductors and one lead-end connector. They do not establish numeric exit poses, wire sizes, connector envelopes or bend radii. Page 1 row 3-11 gives the model-specific lead lengths listed below.

## Hitec HS-311 and HS-55: documented XY, partial shaft extents

The selected sources are Hitec's [HS-311 Ver 2.2 sheet](https://www.hiteccs.com/public/uploads/data_sheet/HCS_HS-311_Specsheetv2.2_102-1729889658.pdf) and [HS-55 Ver 2.2 sheet](https://www.hiteccs.com/public/uploads/data_sheet/HCS_HS-55_Specsheetv2.2_1026-1729887855.pdf), linked from the [HS-311](https://www.hiteccs.com/actuators/product-details/HS-311) and [HS-55](https://www.hiteccs.com/actuators/product-details/HS-55) product pages. Ver 2.2 identifies the documents, not a proved physical revision.

| Source quantity | HS-311 | HS-55 |
|---|---:|---:|
| Case length x width x housing-top height | 40 x 20 x 36.5 | 22.8 x 11.6 x 24 |
| Overall ear length | 52.8 | 32.6 |
| Ear underside / thickness | 26.5 /2.5 | 16.7 /1.0 |
| Mounting columns | 47.8 apart | 28.6 apart |
| Transverse center spacing | 10.2, four pockets | One centerline, two round holes |
| Circular pocket diameter | 4.5 | 2.0 |
| Shaft axis from near column | 13.9 | 9.0 |
| Spline outside diameter / tooth count | 6 /24T | 4 /15T |
| Local end-view spline protrusion | 3.2 | 2.7 |
| Absolute bare-shaft top from case base | Unknown | Unknown |

The physical-dimensions row supplies +/-0.1 mm for the quoted three case dimensions. The catalog leaves other callout tolerances unknown. HS-311 centers are `(0,+/-5.1)` and `(47.8,+/-5.1)`. HS-55 centers are `(0,0)` and `(28.6,0)`. Origins are shaft-near column midpoints on the ear underside, with +X toward the far column. Axes are respectively `(13.9,0,0)` and `(9,0,0)` toward +Z. Case placement assumes symmetry between columns; ear/cover contours remain simplified.

The drawings include fitted horns. Local spline protrusion does not establish the absolute support-collar plane. Adding 3.2 to 36.5 or 2.7 to 24 would invent a bare-shaft top. Accordingly, `shaft_top_dimension` stays null, the factual output path draws no shaft solid, and integrated shaft Z is explicitly amber planning geometry. The HS-55 source does give shoulder 20.8, so its cover's vertical interval is 24-20.8=3.2; that interval does not locate the bare shaft.

The three-contact connector diagrams identify SIGNAL/VCC/GND and depict one attached harness with three insulated yellow/red/black conductors and one connector. Their selected lead lengths are 300 mm and 250 mm, without a declared length tolerance. The [older Hitec Japan HS-55 sheet](https://hitecrcd.co.jp/material/spec_sheet/servo/31055.pdf) instead gives 160 mm. This source conflict is preserved, not merged into a claimed production revision. None of these sources establishes complete cable-exit XYZ, insulated-wire cross-section or minimum bend radius for the catalog.

## STS3215 serial-bus variants and tooling observations

Four model-specific manufacturer sheets are linked by the corresponding Akizuki listings. Their document editions and electrical variants do not establish current production tooling or a physical revision code.

The five implemented records have the following stable identities and default body counts. Counts describe the original preview recipes, not manufactured part counts.

| Asset ID | Case/source interpretation | Default solid bodies |
|---|---|---:|
| `feetech.sts3215-c001.a0` | C001 A/0 drawing-defined old case | 11 |
| `feetech.sts3215-c001.lightguide` | C001 light-guide notice PREVIEW | 12 |
| `feetech.sts3215-c018.a0` | C018 A/0 drawing profile; current mold unknown | 11 |
| `feetech.sts3215-c044.a0` | C044 A/0 drawing profile; current mold unknown | 11 |
| `feetech.sts3215-c046.a0` | C046 A/0 drawing profile; current mold unknown | 11 |

Each has a housing, main, rear idler, two socket shells and six contact bodies. Only the light-guide PREVIEW adds a window body. None includes an attached harness or plug.

| Variant | Seller item | Nominal voltage / ratio | Manufacturer document |
|---|---|---|---|
| STS3215-C001 | [116312](https://akizukidenshi.com/catalog/g/g116312/) | 7.4 V /1:345 | [A/0, 2020-04-10](https://akizukidenshi.com/goodsaffix/STS3215-C001%20SEPC.pdf) |
| STS3215-C018 | [130969](https://akizukidenshi.com/catalog/g/g130969/) | 12 V /1:345 | [A/0, 2023-07-20](https://akizukidenshi.com/goodsaffix/STS3215-C018.pdf) |
| STS3215-C044 | [131131](https://akizukidenshi.com/catalog/g/g131131/) | 7.4 V /1:191 | [A/0, 2025-04-15](https://akizukidenshi.com/goodsaffix/ST-3215-C044%20SEPC.pdf) |
| STS3215-C046 | [131132](https://akizukidenshi.com/catalog/g/g131132/) | 7.4 V /1:147 | [A/0, 2025-04-15](https://akizukidenshi.com/goodsaffix/ST-3215-C046%20SEPC.pdf) |

The selected C001 sheet gives input range 4..7.4 V; C018 gives 4..14 V. C044/C046 sheets give 5..8.4 V, while the seller prose describes 6..7.4 V operation. Keep these source observations distinct. These electrical labels identify source profiles; they do not establish mechanical interchangeability or a new factory revision.

### C001 old case and light-guide PREVIEW

Only the C001 seller listing explicitly links the [2025-12-29 mold-design change notice](https://akizukidenshi.com/goodsaffix/STS3215_design_change_20251229.pdf) and says old/new stock ships randomly without selection. The notice's rear-face photos show the old open rectangular groove toward the idler replaced by a circular translucent light-guide window between the idler and near short edge. It supplies no dimensional callouts for that change. The blue rings are highlights, not physical circular ribs.

The catalog therefore distinguishes the C001 drawing-defined old-case observation from a C001 light-guide PREVIEW. The new profile keeps only the seller SKU's nominal 45.23 x 24.73 x 35 envelope in its factual dimensions. The notice does not verify detailed new-mold geometry: pre-change step, shaft and contact-pitch choices used by the preview remain planning references. Its light-guide diameter 2.5, X3.3/Y12.365 and thickness 0.4 are explicit planning values. Feature dimensions and exact placement in the new preview remain assumptions. C018, C044 and C046 retain their own document-defined profiles with current mold mapping unknown. Their sheets depict the old-style groove, but this illustration does not prove that all current stock uses that mold. The C001 notice is not evidence that each of the four electrical variants is available in both molds.

### Source axial and case-frame boundaries

Each outside-dimension drawing labels 45.23 x 24.73 in plan, with a 45.2 x 24.7 x 35 mechanical-summary triple. The drawings separately show 32/35 cover extents, a 29 mm separation between output-side and rear-side support planes, 3.4 mm exposed main spline, 4.1 mm rear projection and 36.5 mm tip-to-tip relation: `29 +3.4 +4.1 =36.5`. These heights describe different features. Do not add 3.4 to 35 or treat 35 as the support-plane separation. No dimensional tolerances are printed for these callouts.

The main spline is specified 25T/OD5.9 mm; the rear view calls out diameter 6 with a PA3.0 x 5 self-tapping center fastener. The paired views place the main and idler on opposite faces, but their 12.5 mm longitudinal callout terminates on an internal reference line. That line is not proved to be the case midplane or a dimensioned case end. The preview assumes a midplane, yielding `X=45.23/2-12.5=10.115` and transverse `Y=24.73/2=12.365`. Those assumed case coordinates do not populate factual output poses.

The 8-PA2.0 self-tapping-screw callout identifies case-fastener topology. It supplies no complete bracket-mount center pattern, pilot diameter, engagement depth or load interface. There is no verified bracket drilling template. Main/idler visualization along +Z/-Z remains distinct from a supported factual case-frame axis reference.

### Two detachable three-contact bus sockets

Each model's own drawing independently shows two adjacent receptacles on the same rear/idler face, below the idler along the case's long direction, side by side across its width. Their mating direction faces away from the main output. The four A/0 drawings label `5264/2.54 3P`: 3 contacts per socket with 2.54 mm contact pitch. The separate light-guide profile derives its 3-contact counts from the notice photograph; it does not inherit a factual contact-pitch dimension from the old drawing. The signals are GND, supply and TTL data. The C001 connector table and example interface schematic present opposite GND/data numbering, with the viewing convention unresolved; the catalog does not assign numbered polarity to the preview pins.

| Evidence | C001 | C018 | C044 | C046 |
|---|---|---|---|---|
| Outside drawing | PDF page 6 | PDF page 6 | Physical PDF page 9, printed 6/8 | Physical PDF pages 10-11, printed 6/8 |
| Mechanical connector/cable row | Page 4 | Page 4 | Physical page 7, printed 4/8 | Physical page 7, printed 4/8 |
| Specified detachable cable length | 150 mm | 150 mm | 200 mm | 200 mm |
| Port contact count | 3 each | 3 each | 3 each | 3 each |

The STS records have two port records, `bus-a` and `bus-b`, each `attachment: detachable`, `contact_count: 3` and `connector_count: 1`. `bundle_count` and `conductors_per_bundle` remain null; the model's fixed-lead `cables` list is empty. Two ports do not mean two attached harnesses or six permanent wires. The 15/20 cm cable specifications describe detachable cables, with unspecified endpoints and supplied quantity; they are not fixed-pigtail lengths or default routing paths.

Exact socket centers, complete receptacle/plug envelopes, strain relief, keying, insertion/removal clearance and bend radius remain unknown. Contact pitch alone does not establish those dimensions. The original simplified socket/pin geometry is a planning aid with explicit assumptions. A front view cannot show all interfaces; inspect the rear view for both sockets, idler and C001 tooling difference. Use explicit user-planned cable/plug space when reserving interference clearances.

## Fixed-pigtail cable interfaces and interference evidence

The following counts describe the selected product photographs or manufacturer wiring diagrams. Counts are qualitative observations or specification facts, not dimensional measurements. A harness is one cable bundle containing multiple insulated conductors. They apply to the selected source observations, not every item sold under a shared model name.

| Record | Observed bundles | Conductors per bundle | Published lead length | Count evidence |
| --- | ---: | ---: | --- | --- |
| TowerPro SG90 Analog | 1 | 3 | 250 mm, tolerance unspecified | [Manufacturer photo](https://towerpro.com.tw/wp-content/uploads/2014/08/SG90-1B.jpg) |
| TowerPro SG92R | 1 | 3 | 250 mm, tolerance unspecified | [Manufacturer photo](https://towerpro.com.tw/wp-content/uploads/2014/08/SG92R-1B.jpg) |
| TowerPro MG90S | 1 | 3 | 250 mm, tolerance unspecified | [Manufacturer photo](https://towerpro.com.tw/wp-content/uploads/2014/08/MG90S-1B.jpg) |
| FEETECH FS90, Akizuki 114806 | 1 | 3 | 250 +/-5 mm | [Seller photo](https://akizukidenshi.com/img/goods/L/114806.jpg) |
| TowerPro MG90D | 1 | 3 | 250 mm, tolerance unspecified | [Manufacturer photo](https://towerpro.com.tw/wp-content/uploads/2014/08/MG90D-1B.jpg) |
| TowerPro MG92B | 1 | 3 | 250 mm, tolerance unspecified | [Manufacturer photo](https://towerpro.com.tw/wp-content/uploads/2014/08/MG92B-1B.jpg) |
| TowerPro SG5010 | 1 | 3 | 320 mm, tolerance unspecified | [Manufacturer photo](https://towerpro.com.tw/wp-content/uploads/2014/08/SG5010-1B.jpg) |
| TowerPro MG996R | 1 | 3 | 320 mm, tolerance unspecified | [Manufacturer photo](https://towerpro.com.tw/wp-content/uploads/2014/08/MG996R-3b.jpg) |
| FEETECH FS90R | 1 | 3 | 250 +/-5 mm | [Seller photo](https://akizukidenshi.com/img/goods/L/113206.jpg) |
| FEETECH FT90B | 1 | 3 | 250 +/-5 mm | [Seller photo](https://akizukidenshi.com/img/goods/L/114693.jpg) |
| FEETECH FS0307 | 1 | 3 | 150 +/-5 mm | [Seller photo](https://akizukidenshi.com/img/goods/L/114807.jpg) |
| FEETECH FS5103B | 1 | 3 | 300 +/-5 mm | [Seller photo](https://akizukidenshi.com/img/goods/L/114802.jpg) |
| FEETECH FT1117M | 1 | 3 | 180 +/-5 mm | [Seller photo](https://akizukidenshi.com/img/goods/L/114803.jpg) |
| Hitec HS-311 | 1 | 3 | 300 mm, tolerance unspecified | Manufacturer connector diagram in the HS-311 sheet above |
| Hitec HS-55 | 1 | 3 | 250 mm, tolerance unspecified | Manufacturer connector diagram in the HS-55 sheet above |

The TowerPro product pages specify 25 cm or 32 cm leads. FEETECH lengths come from each sheet's page 1 row 3-11; Hitec lengths come from the selected Ver 2.2 physical specifications. Endpoint conventions and inclusion of connector or strain relief are not established. Do not treat a published lead length as guaranteed straight routable space.

The original four photographs show brown/red/orange conductors; the expansion includes black/red/white FEETECH leads and the Hitec yellow/red/black wiring diagrams. Colors and harness counts remain source-specific. The FEETECH photographs and Hitec diagrams establish one lead-end connector in each selected profile; the seven selected TowerPro photos do not establish connector count. Low short-end exits are visible in some sources, but concealment and missing dimensions prevent exact outlet coordinates. The generic key cannot register an exit for every model.

| Interference property | Evidence boundary for the current records |
| --- | --- |
| Exact exit XYZ and direction | No dimensioned exit datum; any integrated-model placement is a labeled planning assumption |
| Individual insulated wire outside diameter | Not established in the catalog; conductor gauge alone would not specify the insulated envelope |
| Bundle width, thickness, or jacket diameter | No dimensioned cross-section |
| Complete connector envelope and strain relief | No fully dimensioned, registered envelope is implemented |
| Minimum static or dynamic bend radius | Not specified |
| Connector insertion/removal and flex space | Requires a separate design allowance |

The working cable-space representation uses explicit user planning values for these unknowns. Its straight corridor, flex allowance, and optional connector box are keepouts, not a reconstruction of the photographed wire. Snapshots mark these inputs `user_specified_planning_envelopes`; no default numeric exit position or bend radius is promoted into manufacturer data.

The integrated references draw three short illustrative conductor cylinders by default. A connector box appears only when explicitly requested. FS90's planning defaults remain separate from its cable-port facts:

| Planning quantity | Integrated default | Status |
| --- | ---: | --- |
| Outlet height above the body base | 2 | Editable insertion value; exact exit height is not specified |
| Conductor outside diameter | 1 | Editable insertion value; includes an assumed insulated-wire envelope |
| Conductor center pitch | 1.1 | Editable insertion value; not obtained by photo scaling |
| Displayed straight segment length | 3 | Short outlet marker; editable from 2 through 250, not the published lead length |
| Connector box length x width x height | 10 x 8 x 4 | Hidden by default; approximate catalog geometry when requested |

The default FS90 exit is on the assumed near case face at `(2.6, 0, -13.7)` in the mounting frame, extending along -X. Its height follows the 2 mm planning default and the derived case-base height. The FS90 photograph supports a low exit on that short end qualitatively, but supplies none of these numeric coordinates, wire sizes or connector dimensions. TowerPro previews use their own explicit planning placements, not the FS90 datum. A caller can request a longer segment, for example 20 mm, and opt into the connector; use `cable-space` for caller-sized connector envelopes. The integrated snapshot records basis `mixed_dimensioned_assumed_and_approximate`; changing a planning parameter does not change the source's nominal dimensions or create a known factory revision.

The older Pololu FS90-C sheet specifies 200 +/-5 mm lead length. That value belongs to its separate source profile and must not replace the selected Akizuki record's 250 +/-5 mm.

## Aus Electronics Direct SG90: source-specific output reference

The [Aus Electronics Direct SG90 guide](https://www.auselectronicsdirect.com.au/assets/brochures/TA0132.pdf), page 4, contains a detailed drawing. It gives mounting-hole radius 1.1 and a direct pitch of 29. However, its 32.4 overall length minus two 1.9 end offsets gives 28.6. The separate chain 8.75 + 6.0 + 13.85 also gives 28.6. This internal conflict remains unresolved. Do not publish 29 as verified pitch or silently change it to 28.6. Page 5 shows four dimensioned horns, but their hub geometry does not establish a universal spline interface.

The catalog includes `aus-electronics-direct.sg90.ta0132` as a separate source record. The manufacturer is unspecified. The output reference uses the housing envelope's minimum corner as its origin, with X along its length, Y along its width, and +Z toward the shaft. It does not reuse the TowerPro SG90 Analog record's dimensions or cable facts.

| Output-reference quantity | Nominal | Evidence |
| --- | ---: | --- |
| Housing envelope length | 22.7 | Side-view length callout |
| Housing envelope width | 12.1 | Top-view width callout |
| Housing/tower-top height from base | 27.1 | Side-view height callout; excludes exposed output shaft |
| Shaft-top height from base | 30.7 | Side-view overall height callout |
| Axis X from the left case end | 5.8 | Derived as 1.9 + 8.75 - 4.85 |
| Axis Y from either case-width edge | 6.05 | Direct top-view dimensions to the axis centerline |
| Exposed shaft height | 3.6 | Direct side-view callout, also 30.7 - 27.1 |
| Shaft outside diameter | Unknown | No applicable output-shaft diameter callout |

The X derivation uses three published distances: outer left flange to near mounting center 1.9, near mounting center to output axis 8.75, and outer left flange to case end 4.85. It does not use the conflicting 29 mounting-pitch callout. No dimensional tolerances are printed, so none is assigned to the derived 5.8 coordinate.

The drawing's 6.0 dimension is between the main output-axis center and the secondary top-boss center. It is not the distance from the output axis to the case end. Likewise, the diameter-5.4 callout points to the secondary boss, not the output shaft. Neither value supplies a missing output-shaft dimension.

The output-reference command can therefore create the case envelope, a datum axis through `(5.8, 6.05, 0)`, and a fixed construction cross at the shaft-top height. It creates no shaft solid. The unresolved pitch conflict prevents an offered mounting pattern, and no fitted-horn seating plane, spline angular phase, or cable geometry is established for this profile.

## Additional SG90 drawings: separate, incomplete profiles

The ADDICORE-labelled drawing on [C&D Elettronica's SG90 product page](https://www.cedelettronica.com/it/prodotti/automazione/mini-servo-sg90/4589) gives case dimensions 22.5 x 11.8, output-axis offset 5.9 from the case end, shaft diameter 4.6, and a slot pocket diameter 2.0 with a 1.3 mouth. It also depicts a 3.6 x 1.2 cable opening. It does not explicitly locate both mounting centers; deriving a pitch would require additional alignment and symmetry assumptions.

[Elcoteam's SG90 drawing](https://www.elcoteam.com/media/wysiwyg/image/servo_sg90_size.jpg) gives mounting pitch 27.7 and case dimensions 22.4 x 12.5, but no longitudinal output-axis offset. These dimensions differ from the ADDICORE-labelled and Aus Electronics Direct profiles. Do not combine their pitch and axis dimensions into a purported verified SG90 model. Neither drawing is implemented as a complete placement profile in this release.

## Additional MG90S drawings: not implemented

[AIFITLAB's DSSERVO MG90S page](https://aifitlab.com/products/dsservo-mg90s-servo-motor) provides separate drawings:

| Variant | Published facts | Missing facts |
| --- | --- | --- |
| [All-metal MG90S](https://cdn.shopify.com/s/files/1/0673/6848/5000/files/DSSERVO-MG90S-Drawing_All-metal.pdf?v=1784532330) | Four mounting centers on 28 x 7 spacing; housing length 23; outer ear length 32; shaft offsets are dimensioned | Mounting-hole diameter is not called out |
| [Non-all-metal MG90S](https://cdn.shopify.com/s/files/1/0673/6848/5000/files/DSSERVO-MG90S-Drawing_Not_all-metal.pdf?v=1784532330) | Two diameter-2 holes; housing length 24; width 11.9; overall ear length 32.3 | Mounting-hole center distance and shaft-axis position are not dimensioned |

These DSSERVO records must remain separate from TowerPro MG90S. Their existence also demonstrates why a model name cannot be the only catalog key.

For the all-metal drawing, an origin halfway between the near hole pair gives centers `(0,+/-3.5)` and `(28,+/-3.5)`, and the output axis at `(8.3,0)`. The longitudinal chain is consistent: the shaft is 10.3 from the flange end, the near hole column is 2.0 from that end, and the case starts 4.5 from it. These establish positions, but do not supply a clearance-hole diameter or establish compatibility with the TowerPro case.

## Further common-servo source work

MG995 is not included: no suitable primary dimensional source was established during this release's audit. DS3218 and Futaba S3003 also remain future source-audit work, not aliases of an existing model.

Additional STS3215 physical specimens, tooling mappings for other suffixes, exact port poses and bracket interfaces require further evidence. The current source-defined profiles and their limits are documented above.

## Further actuator sources

The implemented ROBOTIS 330/XM430/XM540 representatives are audited separately in
[robotis-sources.md](robotis-sources.md). Additional ratios, further horns, frames and
measured connector poses require their own compatibility evidence. The manual
advertises its migration to [ROBOTIS Docs](https://docs.robotis.com/); verify current
manufacturer sources when adding records. An updated link does not automatically
update a saved instance.

No generic QDD dimensions or compatibility are asserted. A future QDD entry requires a manufacturer and exact model, its own drawing revision, and documented case and output interfaces.

## Separate horn source records

The v0.7.0 accessory catalogs keep horn identity, documented dimensions, proposed
compatibility and installed pose independent of the servo records. The audits are
[micro horns](horn-micro-audit.md), [X330/X430 horns](horn-robotis-430-audit.md) and
[XM540/STS3215 horns](horn-robotis-540-sts-audit.md). The four AU drawings support
partial geometry but unverified TowerPro fit; named Hitec/ROBOTIS combinations and
each inspected STS A/0 accessory page supply their own compatibility evidence.
The light-guide case proposals retain unverified fit. An assumed pose remains
assumed even when compatibility is documented.

No additional horn is the default. Selecting one stores its facts, sources,
fit/pose status and transform with the instance. Primitive contours, plain spline
cavities, omitted retention hardware and explicit planning details do not form
complete manufacturing geometry or a conservative interference bound.
