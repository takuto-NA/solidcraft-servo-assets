# ROBOTIS X330 and X430 horn source audit

Re-audited 2026-11-19 against the actual rendered pages. All dimensions are in
millimeters. Only published manufacturer 2D drawings and official product
information were consulted. No commercial CAD, STEP, IGES or DWG data was used.
Manufacturer PDFs and rendered pages remain in `/tmp`, outside the distributed
plugin. The numeric catalog is an original transcription.

## Primary sources

| Reference | Official download | Drawing identification |
| --- | --- | --- |
| HNX330-N101 metal normal horn | [2163](https://en.robotis.com/service/download.php?no=2163) | `HNX330-N101.pdf`, 21-Dec-22, sheet 5 of 5 |
| X330 plastic normal horn | [2129](https://en.robotis.com/service/download.php?no=2129) | `DC15_A01_HORN2.pdf`, XL330/XC330 Horn, 2022.08.22 |
| HN12-N101 normal horn | [1735](https://en.robotis.com/service/download.php?no=1735) | `HN12_N101.pdf`, 22-May-19 |
| HN12-I101 rear idler | [1757](https://en.robotis.com/service/download.php?no=1757) | `HN12_I101.pdf`, 22-May-19 |
| X330 legacy assembly/idler reference | [1986](https://en.robotis.com/service/download.php?no=1986) | `XL,XC-330.pdf`, X330, 28-May-20 |
| X430 installed normal/idler reference | [157](https://www.robotis.com/service/download.php?no=157) | `XM,H-430_idler.pdf`, XM430/XH430 (HINGE), 2019/03/19 |

The official [horn drawing index](https://en.robotis.com/service/downloadpage.php?ca_id=7020)
and [actuator drawing index](https://en.robotis.com/service/downloadpage.php?ca_id=7010)
identify the part/family drawings. The official endpoints redirect to
ROBOTIS-linked PDF files. Full pages, individual sections and the assembled
side view were inspected. Callouts and witness lines supply dimensions; no
numeric geometry was inferred from pixel scale.

## Individual dimensions and current references

The local origin is the center of the **outer wide-flange face**, Z0. Positive Z
points away from the actuator. This convention distinguishes a front boss from a
rear hub; it does not establish an installed pose by itself.

| Part | Source outer profile | Source frame holes | Current output |
| --- | --- | --- | --- |
| HNX330-N101 metal | Diameter 16, plate 2.7, rear hub diameter 7.4, rear span 5.6 | 4 x M2 x 0.4 TAP THRU, PCD 12 | Partial source outer volume; mating cavity omitted |
| X330 plastic normal, DC15_A01 | Diameter 16 +0/-0.2, plate 2.9, hub reference diameter (7.45), rear span 5.6 +0/-0.15 | 4 x diameter 1.6 HOLE THRU (Self Tap), PCD 12 | Partial source outer volume; ribs/key/serration omitted |
| HN12-N101 normal | Diameter 19.5, plate 2, outward boss diameter 8, axial reference span 4.2 | 8 x M2 x 0.4 TAP THRU, PCD 16 | Partial source outer volume; central passage and mating cavity omitted |
| HN12-I101 rear idler | Diameter 19.5, flange 2; section diameters 9.5, 12.2 and 13; sleeve length absent | 8 x M2 x 0.4 TAP DP.2, PCD 16 | Dimensions and hole references only; no complete solid |

Four-hole centers are `(6,0)`, `(0,6)`, `(-6,0)` and `(0,-6)`. Eight-hole
centers use radius 8 at 45 degree increments, including cardinal directions.
These are local drawing patterns, not calibrated actuator-neutral angles.
Machine-tapped holes use nominal thread-major references, not actual factory
tap-drill diameters or helical thread geometry. Plastic pilots retain their
published diameter 1.6.

### HNX330-N101 metal normal horn

Section A-A directly dimensions plate 2.7, total rear span 5.6 and hub diameter
7.4. The diameter 2.6 axial opening and diameter 5.6 counterbore, depth 1.65,
remain as source-dimensioned openings. The offset diameter 1.5 relief and offset
1.53 remain metadata; its unregistered axial extent is no longer cut through the
whole part.

The female serration has 20 repetitions, a 92 degree detail angle and diameters
4.95/4.5, but its axial transition is not dimensioned. The former smooth cavity
with assumed depth 2.9 is removed. Published chamfer C0.5 is not representable by
the current box/cylinder recipe and is omitted. The remaining solid fills
omitted cavities and edge details; it is a partial outer volume, not a mating or
manufacturing solid.

The [HNX330-N101 item](https://en.robotis.com/shop_en/item.php?it_id=903-0314-000)
documents XL330/XC330 compatibility. The exploded drawing distinguishes the
XC330 M2.5 retaining screw from the XL330 M2.6 tapping screw. It does not locate
the installed flange relative to the case. Retaining screws are omitted.

### X330 plastic normal horn

Download 2129 depicts a normal serrated output horn, not the rear idler.
Parenthesized diameters (7.45), (5) and (4.52) are source reference dimensions.
The drawing also gives a diameter 2.6 HOLE THRU and diameter 5.6 +/-0.2
counterbore, depth 1.6.

The rear cavity/serration depth **2.6 is directly dimensioned** in Section A-A.
The old preview's 2.75 depth was incorrect. The catalog now records 2.6 but
creates no smooth surrogate cavity. Rear rib pockets, keyed relief, serrations
and undimensioned edge details are omitted. Published outer cylinders, confirmed
central opening/counterbore and four pilot holes remain.

The drawing does not establish a current sales SKU or the exact factory-fitted
part in current actuator profiles. It is not renamed HNX330-N102 by inference.
Asymmetric tolerances stay in source evidence notes because `tolerance_mm`
represents symmetric tolerances only.

### HN12-N101 normal horn

The diameter 8 boss is in front of the wide flange, away from the actuator.
With the outer wide-flange face Z0, the plate occupies Z-2..0 and the boss
reaches Z+2.2, because the section span is `4.2 = 2 + 2.2`.

The central retaining-bolt passage diameter is not dimensioned. Its former
2.8 mm cut is removed. The DC12 serration detail gives 20 repetitions, 92 degrees
and diameters 5.8/5.2; the section step is 3.45. These values do not define a
smooth cylindrical mating bore, so that surrogate is also removed. The rear lip
and undimensioned edge details are omitted. The resulting outer-volume reference
has a filled center where the real passage/cavity is unknown and is unsuitable
for a mating-fit check.

The [HN12-N101 item](https://en.robotis.com/shop_en/item.php?it_id=903-0238-000)
documents XH/XM430 compatibility and includes a thrust washer and retaining
hardware. Neither hardware nor a substitute washer thickness is modeled.

### HN12-I101 rear idler

The source distinguishes idler and plastic-bearing components. Radial dimensions
and flange thickness are published, but the overall sleeve length and complete
axial step distribution are absent. The old combined 8 mm sleeve was fabricated
and is removed. The record is now `status: dimensions_only`, `parts: []`; known
dimensions and the hole pattern remain without an invented complete body.

The literal `TAP DP.2` callout is corroborated by the installed sheet's
`DP2.0(MAX.)`. This is not an engagement or screw-length guarantee. The
[HN12-I101 item](https://en.robotis.com/shop_en/item.php?it_id=903-0240-000)
describes the opposite-side hook-retained accessory set, including a plastic
bearing and cap. It is not a second driven normal horn.

## Installed X430 registration

The XM430/XH430 (HINGE) side view was rendered again at 450 dpi to separate the
witness lines. Its intervals are distinct:

- **2** is from the main case face to the normal horn's outer wide-flange face.
- **2.2** is from that outer flange face to the outward diameter 8 boss tip.
- The rear **2** is from the rear case face to the idler's outer flange face.

These agree with the standalone HN12-N101 section. The previous pose rule
`preview coupling tip 35.2 + plate 2 = 37.2` had no source basis and is removed.
The installed sheet also defines the current XM430 frame:

| Feature | Existing XM430 case-frame coordinate | Cross-check |
| --- | --- | --- |
| Shared main/idler centerline | X11.25, Y14.25 | Source 11.25; centered width `28.5 / 2` |
| Main case face | Z34 | Source depth 34 from rear case Z0 |
| HN12-N101 outer flange face | Z36, direction +Z | `34 + 2` |
| HN12-N101 diameter 8 boss tip | Z38.2 | `34 + 2 + 2.2` |
| HN12-I101 outer flange face | Z-2, direction -Z | `0 - 2` |

All four XM430 T/R bindings now have documented nominal placement using this
sheet. The idler remains unavailable as an automatically attached solid because
its full geometry is unknown. These poses do not specify production tolerances,
omitted mating profiles, bolt-head extent, washer thickness or neutral-angle
calibration. No preview body is removed by these bindings.

## X330 assembly limits

The 2020 X330 family sheet shows legacy case span 23, output projection 3 and
front extent 26. Its optional rear idler adds another 3, for total span 29.
It does not identify the depicted normal part as either exact 2022 individual
horn revision or register that revision to each current case. Matching diameter
16, PCD 12 or appearance cannot establish a seat or factory part identity.

All eight X330 bindings retain documented series compatibility but now use
`placement_basis: unknown`, `origin_mm: null` and empty `replace_model_parts`.
The prior Z26 attachment and generic stock-face replacement claim are removed.
Automatic attachment requires a source registering the accessory revision to the
case.

## Remaining representation limits

Box/cylinder recipes cannot reproduce a complete serration, chamfer, rounded
edge or rib pocket. No guessed dimensions are introduced to fill those gaps.
A partial source outer volume may occupy space removed by an actual cavity and
must not be treated as a guaranteed clearance volume. Component-level idler
structure, asymmetric tolerances and calibrated angular phase also remain
outside the current solid definition. See the correction record for numeric
cross-checks and the removed constructions.
