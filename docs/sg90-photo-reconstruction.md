# SG90-style horns reconstructed from the supplied photograph

The user explicitly permits and expects photo-based shape inference. These
recommendations support tangible double-arm and single-arm CAD references,
constrained by the image's printed major dimensions and calibrated image
features. The earlier dimensions-only response was too restrictive for that
requested visual reconstruction.

The source is the [supplied annotated photograph](https://ae-pic-a1.aliexpress-media.com/kf/Se22ca210d021427aafa90dafb6c79844N.jpg?has_lang=1&ver=3),
inspected at its original 1600 x 2706 pixels on 2026-11-19. Pixel coordinates
below use the original image's upper-left origin. Manufacturer, part number,
factory revision and mating servo remain unidentified. Its hand-measurement
caption supplies no numeric tolerance. Preserve the annotation records separately
from reconstructed dimensions and relations.

## Through holes and molded dimples

**Double arm: six linkage through holes per arm, twelve total. Single arm:
seven linkage through holes.** The center retaining opening is additional and
does not enter those counts.

In the double-arm plan photograph, the lower half has six distinct apertures
with bright background visible. The upper half has five unobscured apertures and
a sixth partly hidden by the raised hub and overlaid dimension line. Crucially,
the separate side photograph shows six centerline openings independently on
each arm. The six-plus-six count is therefore not merely a symmetry guess.

The single-arm plan shows seven separate background-visible centerline openings.
Its side photograph also shows seven small openings. The larger circles beside
the centerline occur in paired groups and have dark shallow bottoms; they are
molded dimples/recesses, not additional through holes. Do not drill those paired
circles through the reconstructed plate. Their depth is not recoverable from
the image; omitting them is preferable to using an arbitrary deep cut.

The printed 1.45 measurement is interpreted as the small through-opening size
for each pictured horn. Applying it to the visually repeated apertures is an
explicit reconstruction assumption, not a complete manufacturer hole note.

## Major dimensions adopted as reconstruction constraints

All values in this table are printed image labels in millimetres. The feature
interpretations are modeling choices, not new verified measurements.

| Constraint | Double arm | Single arm | Interpretation |
| --- | ---: | ---: | --- |
| Overall length | 31.2 | 21.3 | End-to-end target for the reconstructed planform |
| Plate thickness | 2.36 | 2.08 | Side-view plate-span label used as nominal model thickness |
| Total axial height | 4.72 | 4.82 | Larger side-view span used as nominal loose-part model height |
| Hub outside diameter | 6.74 | 6.76 | Outer-hub span interpreted as a circular cylinder |
| Small linkage-opening size | 1.45 | 1.45 | Interpreted as repeated round through-hole diameter |
| Central passage | 2.36 | 2.36 | Central-opening label interpreted as the retaining passage |
| Visible toothed-cup span | 4.65 | 4.54 | Smooth visual recess approximation only; no spline-diameter classification |

The 4.65/4.54 labels do not establish tooth major, minor or pitch diameter. The
2.36 central label does not define a retaining screw specification. Using these
values for a visual recess and passage preserves the pictured appearance without
claiming a production interface.

An explicit section convention can place the outside plate face at local Z0,
plate and raised hub extending toward negative Z. The proposed smooth cup is
cut only through the reconstructed raised boss: `H - t = 2.36` for the double
and `2.74` for the single, with the central 2.36 passage continuing through the
plate. That pocket depth is inferred from the chosen section, not measured
blind depth, engagement, seating height or spline fit. Omit the teeth rather
than invent their count or form. A reversed detached display orientation is
equivalent; neither orientation establishes an installed pose.

## Pixel anchors and calibration

The nominal plan photographs are oblique. Raised hub rims and openings at the
plate project to different image centers. A single isotropic scale applied to
every feature would hide this effect. Coordinates are approximate image anchors;
their decimal places are computational centroids, not physical accuracy.

| Feature | Double-arm image anchor | Single-arm image anchor |
| --- | --- | --- |
| Overall projected end limits | Y approximately 344 and 1052 | Y approximately 1385 and 1953 |
| Central clear-passage centroid | approximately (398, 689.5) | approximately (413, 1497.7) |
| Outer hub transverse span | approximately 165 pixels | approximately 185 pixels |
| Hub-derived transverse scale | `165 / 6.74 = 24.48` pixels/mm | `185 / 6.76 = 27.37` pixels/mm |

For the double, symmetric reaches of 15.6 follow the chosen equal-arm
reconstruction and printed total 31.2. Separate image-to-arm scales compensate
for the unequal projected upper and lower distances from the passage anchor:

```text
upper radius = 15.6 * (689.52 - image_y) / (689.52 - 344)
lower radius = 15.6 * (image_y - 689.52) / (1052 - 689.52)
```

Mirroring the average of corresponding upper/lower apertures reduces the
perspective asymmetry. It does not assert an exact factory-symmetric specimen.

For the single, the circular-hub interpretation gives rear reach `6.76/2 = 3.38`.
The printed overall 21.3 then gives a tip reach of `21.3 - 3.38 = 17.92` from the
axis. This relation is inferred from the pictured concentric circular hub and
the length label. Calibrate the plate's passage anchor to the tip independently
of the elevated rim:

```text
radius = 17.92 * (image_y - 1497.74) / (1953 - 1497.74)
```

This uses approximately 25.41 pixels/mm longitudinally, rather than treating the
raised hub's projected rear edge as a coplanar plate reference. The separate
14.3 annotation remains a source observation: it starts near the projected hub
edge, not at the axis. It is not silently substituted for the axis-to-tip reach.

## Linkage-hole locations

Bright-aperture centroids were sampled near the horn centerline. They can differ
slightly from the mouth center because the thick plate occludes part of an
opening. The partly hidden upper double aperture was checked against the side
photograph. Approximate positions are appropriate for a visual reference; an
identified specimen is still needed for a drilling template.

| Double pair, near to far | Upper image Y | Lower image Y | Mirror-averaged radius, mm | Radius / 31.2 |
| --- | ---: | ---: | ---: | ---: |
| 1 | approximately 588, partly occluded | 788.04 | 4.412 | 0.14140 |
| 2 | 551.68 | 832.02 | 6.178 | 0.19802 |
| 3 | 509.95 | 875.04 | 8.046 | 0.25788 |
| 4 | 468.41 | 920.04 | 9.952 | 0.31897 |
| 5 | 426.5 | 963.12 | 11.825 | 0.37901 |
| 6 | approximately 385 | 1007.75 | 13.722 | 0.43982 |

Use local centers `(+/-r,0)` for these six radii, twelve holes total. A visually
regularized alternative is approximately `4.4 + i*1.865`, for `i=0..5`. The
individual image-derived radii above retain more of the observed spacing and
are the recommended source-specific array.

| Single hole, near to far | Image Y | Reconstructed radius, mm | Radius / 21.3 |
| --- | ---: | ---: | ---: |
| 1 | 1615.85 | 4.649 | 0.21827 |
| 2 | 1664.33 | 6.557 | 0.30786 |
| 3 | 1712.21 | 8.442 | 0.39634 |
| 4 | 1759.94 | 10.321 | 0.48454 |
| 5 | 1809.15 | 12.258 | 0.57548 |
| 6 | 1858.62 | 14.205 | 0.66690 |
| 7 | 1907.18 | 16.116 | 0.75664 |

Use seven local centers `(r,0)`. Their mean reconstructed pitch is approximately
1.911 mm. Neither array inherits an AU or Hitec hole pattern.

### The printed 2.43 spacing is inconsistent with a global pitch

The image labels 2.43 between two adjacent tip-side holes on each part. It must
remain recorded, but applying it to every visible interval does not agree with
the pictured counts and available reach:

- Double: six holes starting at the separately labelled 4.4 offset and spaced
  2.43 would end at `4.4 + 5*2.43 = 16.55`, beyond the 15.6 tip reach. Five holes
  would fit that arithmetic, but the independent side photograph visibly has six.
- Single: seven holes have six gaps, so a 2.43 pitch consumes 14.58 before adding
  the nearest-hole offset. The photo-inferred nearest radius is about 4.65;
  their sum 19.23 exceeds the reconstructed 17.92 reach.

Perspective alone does not turn inconsistent printed constraints into exact
geometry. The chosen reconstruction prioritizes the printed major extents and
the independently visible aperture topology, using calibrated photo positions.
It preserves 2.43 and 4.4 as source observations with their unresolved datum/pitch
conflict. This is a stated modeling decision, not an undisclosed correction.

## Tapered silhouettes and rounded ends

Widths below are silhouette spans in original pixels at rows away from most
dimension leaders. Hub shadows and anti-aliasing make individual edge locations
uncertain by several pixels. The midpoint of left/right flanks is used to remove
the single photograph's slight centerline slant.

| Part/arm region | Image Y samples | Measured silhouette widths, pixels |
| --- | --- | --- |
| Double upper | 395, 440, 480, 525, 565 | 92, 97, 102, 108, 113 |
| Double lower | 850, 895, 940, 1015 | 111, 106, 100, 91 |
| Single | 1580, 1630, 1685, 1735, 1785, 1835 | 167, 158, 148, 138, 128, 120 |

Combining transverse hub calibration with the longitudinal calibration gives a
double half-taper around 3.24 degrees, approximately 6.5 degrees included. The
single straight-flank fit gives about 4.89 degrees per side, approximately 9.8
degrees included. These are separate image-derived fits; the single must not
inherit the double taper or AU's 6.3-degree callout.

The rounded end spans support radii around 1.9 mm for the double and 2.0 mm for
the single. The selected nominal reconstruction radii are 1.90 and 1.96. They are
calibrated silhouette choices, not unlabelled manufacturer radius callouts.

### Immediate double-arm construction

Use hub radius 3.37, symmetric tip reach 15.6, end-circle radius 1.90 and centers
at `(+/-13.70,0)`. Use half-angle `alpha=3.24 degrees`. For the positive arm the
upper straight flank is `y=b-x*tan(alpha)`, with the lower flank reflected.
Analytic end-circle tangency gives:

```text
b = 13.70*tan(alpha) + 1.90/cos(alpha) = 2.67859
upper tangent point = (13.80739, 1.89696)
outer tip = (15.60, 0)
```

Mirror for the negative arm. Join the tapered plate with the concentric hub; the
hub circle overlaps the narrower arm root rather than forcing the arm edges to
be tangents to the larger hub. This reproduces the pictured broad central ring
and slim tapered arms. It is not a constant-width capsule.

### Immediate single-arm construction

Use circular root/hub radius 3.38, end radius 1.96 and end-circle center
`(15.96,0)`. The tip is at 17.92; the root extends to -3.38, giving total 21.3.
A common external tangent between these circles produces a smooth inferred
planform:

```text
alpha = asin((3.38-1.96)/15.96) = 5.10449 degrees
upper flank intercept = 3.38/cos(alpha) = 3.39346
upper end tangent = (16.13439, 1.95223)
```

This regularizes the measured 4.89-degree flank by about 0.21 degrees so the root
and tip meet smoothly. The largest total-width difference from the fitted line
is about 0.12 mm near the root and decreases toward the tip. Record this
regularization explicitly; it is grounded in the measured silhouette and the
adopted circular hub/end geometry.

The proposed contour/radii and centers should be tagged as photo-inferred.
Several-pixel edge selection, aperture occlusion and residual perspective can
move reconstructed positions by roughly tenths of a millimetre. That is a
reconstruction spread, not a manufacturer tolerance or guaranteed error bound.

## Primary-source comparison and identity limits

The official TowerPro galleries show the same broad single/double/asymmetric-
cross kit categories: [SG90 Analog arm set](https://towerpro.com.tw/wp-content/uploads/2014/08/Servo-arm-set-7.jpg),
[SG92R arm set](https://towerpro.com.tw/wp-content/uploads/2014/08/Servo-arm-set-8B.jpg)
and [MG90S arm set](https://towerpro.com.tw/wp-content/uploads/2014/08/Servo-arm-set-6.jpg).
Their product pages are [SG90 Analog](https://towerpro.com.tw/product/sg90-analog/),
[SG92R](https://towerpro.com.tw/product/sg92r-7/) and
[MG90S](https://towerpro.com.tw/product/mg90s-3/). These support the recognizable
SG90-style family, not identification of the supplied photograph's part number
or its exact dimensions. The page text does not provide a complete horn drawing.

The independent [AU TA0132 page 5](https://www.auselectronicsdirect.com.au/assets/brochures/TA0132.pdf#page=5)
gives another dimensioned set: its double is 32.0 long and its single 19.5, both
with 1.5 plates. Those remain separate source variants. Similar topology is not
a reason to replace the present 31.2/21.3, 2.36/2.08 or photo-inferred hole arrays
with AU values.

No commercial CAD data, FreeCAD-derived table, Thingiverse model or downloaded
vendor CAD model is used. The output is original analytic geometry constrained
by the supplied image. It should be named as an unidentified photo reconstruction,
not a universal factory SG90 horn. No source here establishes a full spline fit,
retaining screw standard, installed seating plane or calibrated servo-neutral
angle. Those limits do not prevent making the requested recognizable 3D parts;
they define how the reconstruction is described.
