# Micro-servo horn sources after the correction

Checked on 2026-11-19, merged catalog release `2026-11-19.10`. The eight records are
four AU TA0132 drawing variants and four named Hitec Revision D horns. The
[user photograph](https://ae-pic-a1.aliexpress-media.com/kf/Se22ca210d021427aafa90dafb6c79844N.jpg?has_lang=1&ver=3)
has two additional unidentified records in a separate pack. It does not revise
or identify the AU or Hitec parts.

See the [correction audit](horn-micro-correction-audit.md) for a feature-by-feature
source reading and the unsupported dimensions removed from version 0.7.
The user subsequently requested photo-based shape inference: version 0.9 adds
explicit photo-estimate solids for the two supplied-image records, without
changing AU or Hitec identities or promoting estimates to verified source facts.

## AU TA0132

[TA0132 page 5](https://www.auselectronicsdirect.com.au/assets/brochures/TA0132.pdf#page=5)
supplies the following millimetre dimensions and hole-center radii.

| Drawing | Published plan extents | Located linkage references |
| --- | --- | --- |
| #1 double | 32 total, 16 each side; included taper 6.3 degrees | +/-4,6,8,10,12,14 along the long centerline |
| #2 cross | 20.3 total, separately labelled 10.2 halves | 4.3,6.3,8.3 on each arm centerline |
| #3 asymmetric cross | 34.5 = 16 + 18.5; short span 16.3; included tapers 6.3/9.8 degrees | Shorter arm 4..14 by 2; longer 4.5..16.5 by 2; short arms 4.3, 6.3 |
| #4 single | 19.5 total, reach 16, back reach 3.5; included taper 6.3 degrees | 4,6,8,10,12,14 along the arm centerline |

The side views give plate 1.5, boss diameter 6.9 and boss rise 2.5; their sum is 4.0.
Peripheral holes are diameter 1.0 typical. No central hole or spline is dimensioned.
Taper slope does not locate the flanks or fix an end radius. The prior 3.2-wide
capsules, R1.6 tips, diameter 2 center and diameter 5.2 rear pocket are removed.
All four are now `dimensions_only`, retaining hole patterns without a fabricated
outline. The 20.3/10.2 and 19.5/16/3.5/6.9 rounding conflicts remain unresolved.

The TowerPro kit photographs show horn topology only. They do not establish AU
horn dimensions or compatibility with SG90, SG92R or MG90S. All twelve proposed
cross-brand bindings have been removed.

## Hitec Revision D

[HITEC ALL SERVO HORN, Revision D, 2023-04-28](https://shop.multiplex-rc.de/userdata/files/HITEC%20ALL%20SERVO%20HORN_Rev%20D_230428.pdf)
provides named F15T and H24T compatibility. Page 2 identifies FS-IL15/FS-X15 with
HS-55 and M-I24/M-O24 with HS-311. Their installed plate-face heights are not
established by those loose-part sheets; all four mount origins remain null.

| Horn; page | Supported references | Representation |
| --- | --- | --- |
| FS-IL15;3 | R1.8 ends; diameter1 holes at +/-7.5,10.5,13.5; plate1.5; rear-face span3.5 | Dimensions only: flank registration missing |
| FS-X15;5 | R1.6 ends; diameter1 holes at (+/-6.5,0),(0,+/-7); plate1; rear-face span3.2 | Dimensions only: diagonal transitions/root blends not located |
| M-I24;12 | R2 ends; diameter1.2 holes at +/-7.5,10,12.5; plate1.6; rear-face span4.1 | Dimensions only: taper and R3 root blends not fully located |
| M-O24;14 | Diameter17 flange; rear hub8; front boss7; plate1.6; rear-face span4.1; front rise0.9 | Partial outer solid with four known cardinal holes |

M-O24 cardinal centers are (-6,0), (6,0), (0,-6.5), (0,6.5). The four previously
invented 20-degree oblique cuts are removed. The rear spline pocket is omitted,
not replaced by a nominal diameter 6 round bore. The diameter 2.8 through passage
and diameter 6 front recess/depth 1.3 are source-defined. The displayed partial
solid consequently leaves material in the unknown toothed pocket, and omits
four unlocated holes. It does not reproduce a complete mating or clearance model.

Nominal F15T/H24T classes do not specify smooth bore geometry. No neutral phase,
retaining screw or seat is inferred. Manufacturer general tolerances remain
attached to direct source dimensions; a derived span does not gain an invented
position tolerance. The older Hitec Japan sheet is a separate source revision.

## User photograph

The unidentified double-arm image prints length 31.2, plate-edge span 2.36,
axial span 4.72 and hub-region span 6.74. The single arm prints 21.3, 2.08, 4.82 and
6.76 respectively. These are `reported_*` photo annotations with unknown error
and unnamed support planes, not verified manufacturing dimensions.

Version 0.9 makes two `photo_estimate` solids from those annotations and calibrated
image features. The double has twelve linkage cuts, six per arm, and the single
has seven. Both plan and side views support those counts; paired larger molded
dimples are distinguished from through openings. Their tapered line/arc profiles,
1.90/1.96 tip radii and hole arrays are calibrated estimates rather than arbitrary
constant-width strips. The single's smooth tangent contour is explicitly
regularized from the measured flank fit.

Each numerical shape interpretation lives in `Horn.estimates`, with unit,
`photo_trace` or `photo_inference` evidence, pixel anchors where available and
unknown `uncertainty_mm: null`. The original `reported_*` labels remain separate.
Formal `holes` stays empty: estimated linkage cuts are not verified mounting
datums. The 2.43 pitch annotation conflicts with the pictured counts/reach and is
retained as an observation, with the reconstruction decision explained.

Smooth cup openings use the ambiguous 4.65/4.54 labels as visual estimates;
central passages interpret the 2.36 label. Cup depths 2.36/2.74 follow the adopted
side-view section. These are not spline major/minor diameters, engagement depths
or installed seats. Teeth and cosmetic dimples are omitted. No servo binding or
automatic assembly is added.

Select `horn-reference` with `mode: photo-solid` for these detached 3D models;
`source-solid` rejects them and `dimensions` still shows their source annotations.
See [photo reconstruction](sg90-photo-reconstruction.md) for pixel calibration,
hole arrays and contour equations, and [source review](sg90-photo-review.md) for
other supplier/manufacturer variants. `tools/update-photo-horns.py` reproduces
the recipes. The package contains no vendor photograph.
