# Micro-horn correction audit

This audit records the pre-correction `catalog/horns-micro.json` inspected on
2026-11-19. It supersedes the earlier acceptance of arbitrary arm strips, central
openings and installed poses. Code, catalogs, packages and builds were not changed
by this audit. The recommended status for an underconstrained horn is
`dimensions_only`, without an invented solid or fitted pose.

## Sources and interpretation

- [User-supplied annotated photograph](https://ae-pic-a1.aliexpress-media.com/kf/Se22ca210d021427aafa90dafb6c79844N.jpg?has_lang=1&ver=3),
  inspected from the original 1600 x 2706 image. Manufacturer, part number, product
  listing, revision and servo compatibility are unknown. Its caption describes
  hand measurement and possible error; no numeric tolerance is supplied.
- [Aus Electronics Direct TA0132, page 5](https://www.auselectronicsdirect.com.au/assets/brochures/TA0132.pdf#page=5),
  inspected as a rendered dimension drawing, including enlarged side and plan
  views. This is a different source specimen, not an identification of the user's
  photographed parts.
- [HITEC ALL SERVO HORN, Revision D, 2023-04-28](https://shop.multiplex-rc.de/userdata/files/HITEC%20ALL%20SERVO%20HORN_Rev%20D_230428.pdf),
  inspected on pages 2, 3, 5, 12 and 14, including enlarged contour views.

In the following tables, **annotation fact** means the number is printed in the
image. It does not certify the feature's manufacturing dimension. **Derived**
means an explicit calculation from identified source dimensions and relations.
**Unknown** includes ambiguous leader targets and geometry that is visible but
not dimensioned. No pixel scaling, ellipse fitting, perspective correction or
counting of pictured spline teeth is used to create dimensions.

## User image: double arm

All numbers are millimetres. The photographs are oblique, including the apparent
plan views. Openings at different depths project to different centers and sizes;
the image is not an orthographic section.

| Printed number | What the leaders show | Permitted reading and remaining uncertainty |
| --- | --- | --- |
| 31.2 | End-to-end span of the double arm | Annotation fact for the pictured length. Equal 15.6 reaches would require an unproved centering/symmetry relation. |
| 1.45 | A bracket across a small upper linkage opening | Annotation fact, apparently a hole-size measurement. No diameter symbol, tolerance or full repeated-hole specification is given. |
| 4.4 | Offset between the nearest upper linkage-hole leader and a central-opening leader | Annotation fact. The central datum/surface is not identified well enough to declare an output-axis-to-hole coordinate. |
| 4.65 | Vertical bracket inside the hub/opening region | Annotation fact only. It is not established as a cylindrical bore, spline major diameter, minor diameter, pitch diameter or seat. |
| 6.74 | Bracket around the outer hub region | Annotation fact, apparently an outer hub span. A true cylindrical outside diameter and its datum are not independently specified. |
| 2.36, central diagonal leaders | Width associated with the central clear opening | Annotation fact. The exact section, diameter convention and relation to the toothed cavity are unknown. It is not a sourced 5.2 mm rear bore. |
| 2.43 | Two adjacent lower linkage-hole center leaders | Annotation fact for that local center spacing. It does not dimension every hole pitch, hole count or the first hole's axis offset. |
| 2.36, side view | Bracket at the arm plate edge | Annotation fact, apparently plate thickness. The photograph does not provide an orthographic section or tolerance. |
| 4.72, side view | Larger axial-height bracket beside the hub and plate | Annotation fact, apparently an overall-height measurement. Its support surfaces are not explicitly named; the oblique leaders cannot establish a horn seating plane. |

The contour visibly has tapered flanks, rounded tips, a raised annular hub,
linkage openings and small molded recesses. None of the taper angles, flank
widths, tip radii, fillets, recess depths or complete hole coordinates is
dimensioned. The two arms must not be replaced with AU #1 dimensions solely
because both have double-arm topology.

## User image: single arm

| Printed number | What the leaders show | Permitted reading and remaining uncertainty |
| --- | --- | --- |
| 21.3 | Overall span from the hub-side extremity to the arm tip | Annotation fact. It does not supply the output-axis-to-tip reach. |
| 14.3 | Span from a lower hub-region extension to the arm tip | Annotation fact. The starting line is not the shaft axis, so this is not a 14.3 mm axis reach. |
| 6.76 | Bracket around the hub region | Annotation fact, apparently an outer hub span; the precise cylinder/datum is not identified independently. |
| 4.54 | Bracket inside the hub region | Annotation fact only. No spline major/minor/pitch-diameter or seating interpretation is justified. |
| 2.36, upper horizontal leaders | Central clear-opening width | Annotation fact. Its section and relation to the toothed cavity remain unknown. |
| 1.45 | Bracket across the nearest small linkage opening | Annotation fact, apparently hole size; repeated-hole diameter/tolerance remains unspecified. |
| 2.43 | Two adjacent tip-side hole-center leaders | Annotation fact for one local spacing; do not extrapolate a complete equal-pitch pattern. |
| 4.82, side view | Axial-height bracket beside the hub | Annotation fact, apparently overall height; no installed support or seating datum is named. |
| 2.08, side view | Arm plate-edge bracket | Annotation fact, apparently plate thickness; its numeric tolerance is unknown. |

`21.3 - 14.3 = 7.0` is arithmetic, not a new verified hub diameter. It differs
from the separate 6.76 annotation and the leader datums are not established as
the same physical planes. Likewise, subtracting the side-view thickness from
the larger height would require verified common support planes before calling
the difference a boss projection. Do not use either subtraction to fill the
missing 3D geometry.

The single arm is tapered and rounded with a raised toothed hub and molded
recesses. Its width law, tip radius, center-to-tip distance, hole-array origin,
recess dimensions and spline remain unknown. It is not AU #4, whose printed
overall length is 19.5 and whose plate is 1.5.

## User image: screws

| Pictured screw | Printed dimensions | What is still unknown |
| --- | --- | --- |
| Larger screw | 9.1 tip-to-head extremity; 7.74 tip-to-under-head-region span; 4.84 head-width annotation | Shank/thread diameter, pitch, standard, usable engagement, exact bearing plane and intended fixing location |
| Smaller screw | 5.36 total span; 3.7 tip-to-under-head-region span; 4.26 head-width annotation | Shank/thread diameter, pitch, standard, usable engagement, exact bearing plane and intended fixing location |

The differences `9.1 - 7.74 = 1.36` and `5.36 - 3.7 = 1.66` are conditional head
height calculations only if the extension lines use identical physical datums.
They do not establish screw standards or assign one screw to a horn and the
other to a case. No screw geometry or required hole diameter should be invented.

## AU page 5: retain its own facts

Each AU side view explicitly gives plate thickness 1.5, boss diameter 6.9 and
boss projection 2.5 beyond the plate. Thus 4.0 is a valid derived loose-part
axial extent, `1.5 + 2.5`, for this source drawing. No tolerance is given.
The image's different 2.36/2.08 and 4.72/4.82 labels do not revise these AU facts.
The page's diameter-1.0 typical note applies to its peripheral linkage holes,
not to a central retaining hole or spline cavity.

| AU horn | Direct plan/span facts | Hole-center facts in a source-local frame | Contour limits |
| --- | --- | --- | --- |
| #1 double | 32.0 total; 16.0 each side; 6.3 degree included taper | Long-axis radii 4, 6, 8, 10, 12, 14 on both sides; diameter 1.0 typical | End radii/centers and absolute tapered-edge offset are not explicitly dimensioned |
| #2 cross | 20.3 totals, separately labelled 10.2 halves | Radii 4.3, 6.3, 8.3 on all four centerlines; diameter 1.0 typical | No width/end-radius callout; `2 x 10.2 = 20.4`, not 20.3 |
| #3 asymmetric cross | 34.5 = 16.0 + 18.5 long span; 16.3 short span; 6.3 and 9.8 degree long-arm included tapers | Shorter long arm radii 4 through 14 in steps of 2; longer arm 4.5 through 16.5 in steps of 2; short arms 4.3 and 6.3 | Short-span registration, tip radii, short-arm width and long-flank offsets are not fully dimensioned |
| #4 single | 19.5 total; 16.0 arm reach; 3.5 back reach; boss diameter 6.9; 6.3 degree included taper | Radii 4, 6, 8, 10, 12, 14 on the arm centerline; diameter 1.0 typical | The 3.5 back reach differs from `6.9 / 2 = 3.45`; do not select an invented reconciliation |

### What an analytic AU taper can establish

The 6.3 and 9.8 degree labels span the two flank extensions; they are **included**
angles. For a symmetric arm centerline, the corresponding flank directions are
plus/minus 3.15 or 4.9 degrees. With X along the arm, the two straight flanks can
be written `y = +/- (b - x tan(alpha))`. Here `alpha` is the half-angle and `b`
is the still-undimensioned intercept. The source fixes the slope; it does not
independently fix `b`.

A tangent circular tip with center `(c,0)` and radius `r` would satisfy
`b = c tan(alpha) + r / cos(alpha)` and tip reach `L = c + r`. Those equations
still leave a degree of freedom when only `L` and `alpha` are known. An outer
linkage hole at radius 14 does not by itself prove that the enclosing end arc is
concentric with that hole. Assuming `c=14` and `L=16` would yield `r=2`, but the
concentricity is an additional relation, not an independently printed radius.
It must not be promoted into a verified 2 mm end radius.

The diameter-6.9 boss cannot supply the missing plate-flank tangent: the plan view
shows the circular boss overlapping the arm, with the arm flanks continuing
inside its outline. A line tangent to a 3.45-radius circle is not the depicted
arm-root relation. Nor may the already fabricated 3.2 width/R1.6 capsule supply
the missing intercept. There is no unique dimensionally established AU solid
contour. Retain the spans, slopes and formal hole references as `dimensions_only`.

## Hitec contours: source facts versus the present strips

The user image does not identify a Hitec part. The following facts belong only to
the named Hitec Revision D sheets; they cannot fill gaps in the user's image or
an AU variant.

| Horn; PDF page | Reliable numeric contour/hole evidence | Is a complete contour established? |
| --- | --- | --- |
| FS-IL15; 3 | End R1.8; linkage-pair spans 27/21/15, giving centers +/-13.5, +/-10.5, +/-7.5; diameter 1.0 holes; rear hub diameter 6.0 | The flanks taper. Their angle and independently registered plate-root width are not called out. The rear-hub diameter does not prove a constant 3.6 plate width. Recommend `dimensions_only`. |
| FS-X15; 5 | End R1.6; horizontal/vertical hole spans 13/14, giving (+/-6.5,0) and (0,+/-7); diameter 1.0 holes; R0.5 root transitions; rear hub diameter 5.6 | End width 3.2 follows the semicircular tip, but the center has short diagonal transitions and R0.5 blends with unlabelled registration. Two crossing constant-width capsules omit that contour. Recommend `dimensions_only`. |
| M-I24; 12 | End R2; hole spans 25/20/15, giving centers +/-12.5, +/-10, +/-7.5; diameter 1.2 holes; R3 root blends; rear hub diameter 8.0 | Flank angle and R3 blend locations remain underconstrained. A constant 4.0 plate strip is not its tapered outline. Recommend `dimensions_only`. |
| M-O24; 14 | Circular outside diameter 17; rear hub diameter 8.0; front annulus diameter 7.0; diameter 1.2 peripheral holes; PCD 12/13 | The circular outer contour and axial section are dimensioned. Only four cardinal linkage centers have unambiguous numeric registration; four oblique angles and the complete spline remain missing. No complete fitted part is established. |

The Hitec end circles and pair spans support derived plan extents where the
drawn tip centerline relations are used: FS-IL15 `27 + 2 x 1.8 = 30.6`, FS-X15
`13 + 2 x 1.6 = 16.2` by `14 + 2 x 1.6 = 17.2`, and M-I24
`25 + 2 x 2 = 29`. These extents do not determine intermediate flank geometry.
R0.5 or R3 alone does not determine an undimensioned transition's position.

Hitec axial facts can remain in the dimension records: FS-IL15 plate 1.5 and
rear-face-to-plate-face 3.5; FS-X15 plate 1.0 and corresponding span 3.2; M-I24
and M-O24 plate 1.6 and corresponding span 4.1. Their H24T front annulus projects
0.9, derived as 5.0 minus 4.1, with diameter-6 front recess depth 1.3 and central
diameter-2.8 through passage. The F15T central through passage is diameter 1.8.
Rear serration-pocket depth labels are 2.5 for the selected F15T sheets and 3.0
for the selected H24T sheets. These are loose-part dimensions, not servo seats.

F15T nominal diameter 4.0 and H24T nominal diameter 6.0 are named compatibility
classes. They do not dimension a smooth cylindrical bore. In particular, FS-X15
also has a literal section callout 3.2; this audit does not rename that callout a
major or minor spline diameter. The full mating tooth definition is absent.

M-O24's source-located cardinal hole centers are `(-6,0)`, `(6,0)`, `(0,-6.5)`
and `(0,6.5)`. The current additional centers `(+/-2.223131,+/-6.108002)` come
from an invented 20 degree angle, not a source dimension. Those four cuts must
be removed. The circular body and section could support a detached partial
dimension reference, but not an assertion of a complete manufactured or mounted
horn. Under the current strict insertion policy, retain `dimensions_only` until
the incomplete geometry has an explicit supported representation.

## Required corrections to the pre-correction catalog

| Current element | Required treatment |
| --- | --- |
| All four AU 3.2-wide arm strips and R1.6 caps | Remove from offered solid geometry; no AU callout establishes those widths/radii. Retain source taper facts without filling their missing offsets. |
| AU central diameter-2 through cuts | Remove; they have no source dimension. The user-image 2.36 annotation does not identify these AU parts. |
| AU diameter-5.2 rear openings and selected pocket depth | Remove; neither the AU sheet nor the user image establishes that cavity. |
| AU #2 symmetric 10.15 outline halves | Remove as a resolution of the unresolved 20.3/10.2 conflict. Preserve the conflicting source numbers separately. |
| AU #3 +/-8.15 outline registration | Remove from factual/insertable geometry; source total 16.3 alone does not independently locate both ends. |
| AU #4 constructed 19.45 contour | Do not offer it as the source's 19.5 horn. Preserve the 19.5/16/3.5/6.9 discrepancy without manufacturing a different contour. |
| FS-IL15 3.6 and M-I24 4.0 constant strips | Remove as complete contour substitutes. Their end diameters do not describe a constant width over the arms. |
| FS-X15 crossed 3.2 capsules | Remove as a complete center outline; its source transition geometry is not a simple capsule union. |
| Hitec nominal-class round rear bores | Do not offer them as source-recovered mating cavities. Retain named class facts without inventing tooth geometry. |
| M-O24 four assumed oblique-hole cuts | Remove; retain only the four numerically registered cardinal centers in the formal pattern. |
| AU-to-TowerPro and Hitec preview-tip-plus-plate poses | Disable fitted insertion while pose is unsupported. A visible warning does not supply the missing seat or axis registration. |

The named Hitec manufacturer part-pair compatibility statements may remain as
metadata. They establish physical accessory applicability, not a printable CAD
fit. AU-to-TowerPro compatibility remains unestablished; the supplied image has
no identified manufacturer/model to bind at all.

## Useful references without fabricated mounting

Present the source-specific identities and dimension tables independently. A
dimension reference may show known hole circles/centers, dimensioned boss circles,
axis centerlines, spans and taper-angle directions. Leave an unknown silhouette,
tooth profile, seat or full hole array absent rather than closing it with a
round-number choice. Do not insert a fitted horn at a guessed preview-shaft tip.

The user image can have its own unidentified double/single measurement record,
with each ambiguous annotation retained verbatim as a source observation. It
must not overwrite AU, TowerPro or Hitec identities. A future insertable contour
needs a dimensioned flank/arc construction or a measured, identified specimen;
a fitted pose additionally needs an actual shaft/seat datum. These are separate
evidence requirements.
