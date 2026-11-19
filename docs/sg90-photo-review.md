# Micro horn photo and source review

Reviewed on 2026-11-19. This is a source and inference review only; no catalog,
runtime, build or existing design was changed. Lengths below are millimetres.

The [source photograph](https://ae-pic-a1.aliexpress-media.com/kf/Se22ca210d021427aafa90dafb6c79844N.jpg?has_lang=1&ver=3) is a dimensioned product photograph, 1600 × 2706 pixels. It shows one black double arm and one black single
arm. Its original seller and exact servo compatibility are not established by
the image. Its footer says the measurements are manual and asks allowance for
errors. Printed decimals therefore describe reported measurements, not a
manufacturer's tolerance or a calibrated drawing.

## What makes the supplied horns recognizable

The highest-value features, in order, are the continuous tapered outline with a
rounded tip, the raised hollow boss, the stepped central opening, the actual
linkage-hole count, and the molded surface depressions. A long rectangular strip
with independently chosen end cylinders loses the visible taper and shoulder.
A solid boss with a single uniform hole loses the visible cup and floor.

The side views show **six linkage holes on each double arm: twelve total**. The
single arm has **seven** centerline linkage holes. The nearest double-arm hole
can be partly hidden by the raised boss in the oblique plan view; this is not a
reason to remove it. The separate central retaining opening is not one of those
linkage holes.

Paired circles alongside the hole row are molded dimples or surface marks. They
do not show the background through them. The single arm visibly has three
paired groups; the double arm has paired groups near its ends and its central
region. Their depth and precise manufacturing purpose cannot be read from this
image. If modeled, they should be shallow estimated details, never additional
through holes.

The cavity opens at the raised boss end. This is the mating side that faces the
actuator when installed. The broad arm face on the opposite side is the useful
local outer-face datum. With positive local Z defined away from the actuator,
the raised mating cup and its opening belong toward negative Z. This convention
must not be copied from the outward-boss ROBOTIS HN13 normal horn, whose geometry
and orientation are different.

## Reported dimensions and defensible inference

| Feature in the supplied image | Double arm | Single arm | Interpretation |
| --- | --- | --- | --- |
| Overall length | 31.2 | 21.3 | Printed measurement, specimen-specific |
| Boss outside diameter | 6.74 | 6.76 | Printed measurement; the 0.02 difference is not a demonstrated revision |
| Cup opening | 4.65 | 4.54 | Reported visible opening; not a full spline standard or fit specification |
| Retaining opening | 2.36 | 2.36 | Smaller central opening visible in the cavity floor |
| Linkage opening | 1.45 | 1.45 | Hole-size callout, not the hole pitch |
| End-region hole pitch | 2.43 | 2.43 | Labelled between two end holes; do not silently apply it to every interval |
| Overall axial height | 4.72 | 4.82 | Interpret the matching side-view arrows, not the highest image pixel |
| Plate thickness | 2.36 in the double side-view annotation | 2.08 | Double annotation also spans a boss/plate step; its assignment should remain explicit |
| Derived above-plate boss projection | 2.36 if the plate is 2.36 | 4.82 − 2.08 = 2.74 | Conditional/derived reconstruction, not a separately measured feature |

The single image also labels 14.3 from the projected boss-region boundary to the
arm end. It is not an unambiguous shaft-center-to-tip dimension. Likewise the
double's 4.4 annotation must be interpreted against its particular extension
lines rather than used as an unquestioned radial datum.

The photo supports a hollow cup leading to an annular floor and then a smaller
retaining hole. It does not support cutting the 4.54/4.65 opening through the
whole horn. A minimal estimated reconstruction can stop the cup at the arm-top
datum, with the smaller 2.36 opening continuing through the plate. That floor
placement is a declared estimate: the real pocket may extend into the plate.
It is preferable to inventing an unrelated pocket depth or silently deleting
the visible floor.

Teeth are visible, but a reliable count, flank profile and fit cannot be obtained
from these dark, partly occluded openings. A smooth estimated cup can preserve
the recognizable cavity while keeping tooth compatibility unknown. Do not add a
standard tooth count merely because another product is called SG90.

## Pixel calibration checks

Use the original pixels; enlarged crops add no new measurement information. A
rough transverse calibration uses the same horn's boss outside diameter and its
horizontal silhouette span. The following repeatable edge samples were taken
from the original image using a dark-pixel threshold with a small vertical
majority window to reduce thin annotation-line contamination. They are checks
on an estimated silhouette, not dimensions to promote to measured facts.

| Sample | Original-image sample | Derived transverse width |
| --- | --- | --- |
| Double boss | At y660, x313…477: about 165 px | 6.74/165 ≈ 0.0408 mm/px |
| Double near-root arm | At y575, about 115 px wide | About 4.7 |
| Double mid/far arm | At y430, about 96 px wide | About 3.9 |
| Double near rounded tip | At y380, about 88 px wide | About 3.6 |
| Single boss | At y1465, x329…506: about 178 px | 6.76/178 ≈ 0.0380 mm/px |
| Single near-root arm | At y1625, about 159 px wide | About 6.0 |
| Single middle arm | At y1800, about 126 px wide | About 4.8 |
| Single near-tip arm | At y1920, about 93 px wide | About 3.5 |

The taper is substantially wider near the cup than a narrow constant-width
strip. Use multiple flank samples and a rounded contour fit. A semicircular tip
of roughly 1.8–2.1 radius is a reasonable starting inference from near-tip widths,
but it is not a printed radius; fit its join to the sampled flanks rather than
choosing a cylinder independently.

Perspective and cavity height shift the apparent center. Fit the boss's outside
rim for an in-plane axis estimate, not the displaced white hole at the cavity
floor. Do not use one global pixel scale for longitudinal and transverse
directions, or for separate views. Raised rim and recessed floor are not coplanar.
Some rows are contaminated by dimension leaders: for example, the double arm
near y1000 picks up a right-hand annotation if the entire row is thresholded.
Use a constrained region, exclude leaders, and retain a visible uncertainty band.

## Additional primary/vendor research

These sources are separate specimens. Similar shape, color, a shared marketing
name or a close dimension is insufficient to merge their numbers or fit claims.
No commercial CAD models, third-party part-library models or CAD-site geometry
were used.

| Provider and exact source | What it supports | Boundary |
| --- | --- | --- |
| Aus Electronics Direct, [TA0132 PDF](https://www.auselectronicsdirect.com.au/assets/brochures/TA0132.pdf), page 5 | Four dimensioned SG90 horn drawings; double overall 32, boss 6.9, plate 1.5 and boss projection 2.5; explicit linkage references | These disagree with the supplied black specimen's 31.2, 6.74 and thicker profile. Keep the AU record separate. Missing flank details may be estimated from that same drawing, not replaced by the supplied photo. |
| TowerPro, [SG90 Analog](https://towerpro.com.tw/product/sg90-analog/), [SG92R](https://towerpro.com.tw/product/sg92r-7/), [MG90S](https://towerpro.com.tw/product/mg90s-3/) | Manufacturer-associated product and arm-gallery topology for three named products | Gallery images are not dimensioned loose-horn drawings. Match the exact gallery/version before transferring visible counts; no universal SG90 spline or interchangeable horn is established. |
| Adafruit, [horn set 4251](https://www.adafruit.com/product/4251), [top photo](https://cdn-shop.adafruit.com/970x728/4251-02.jpg), [oblique detail](https://cdn-shop.adafruit.com/970x728/4251-00.jpg) | Detailed black-horn cup, tapered flank and linkage/counterbore photos; inspected single has six linkage holes, double six per arm | The single differs from the user's seven-hole specimen. Adafruit explicitly limits its fit claim to the linked SG92R micro servos. No exact loose-horn dimensions are published here. |
| REES52, [SG-90 360-degree kit](https://rees52.com/products/rees52-sg-90-9g-360-degree-servo-motor-metal-gear), [dimension photo](https://rees52.com/cdn/shop/files/SG-90_SERVO_9GM_360_DEGREE_MOTOR-1.jpg?v=1737526314&width=1946) | Seller's own white-horn dimensional photo: single 20 overall/7 boss width; double 32/7; asymmetric cross 34 × 17. Single visibly has six linkage holes; double six per arm | Reported seller dimensions, distinct from both AU and the supplied black photo; no axial/cavity dimensions or globally applicable fit claim. |
| SparkFun, [ROB-09065 product](https://www.sparkfun.com/servo-generic-sub-micro-size.html), [one-page A0090 drawing](https://cdn.sparkfun.com/datasheets/Robotics/Small%20Servo%20-%20ROB-09065.pdf) | Actual horn plan/side drawings: single length 17.78/overall thickness 4.318; double 28.804/4.293; four-point width 19.355/4.293; disc radius 9.792/overall thickness 3.937 | Printed 16 July 2013, a separate A0090 specimen. Overall thickness is not plate thickness. Hole pitch, bore, full fit and compatibility with SG90 are not established. |
| Pololu/FEETECH, [FS90 product](https://www.pololu.com/product/2818), [accessory photo](https://a.pololu-files.com/picture/0J5404.1200.jpg?b1c148758938faa70f82f595008ae689), [FS90-C specification](https://www.pololu.com/file/0J1435/FS90-specs.pdf) | Clear alternate white horn set including a disc and crosses; manufacturer specification identifies this servo's 20T/4.8 spline | The page says the illustrated included hardware may vary. Servo spline data does not supply loose-horn dimensions or authorize importing an SG90 horn. |
| Waveshare, [WS-SG90 product](https://www.waveshare.com/sg90-servo.htm), [own kit photograph](https://www.waveshare.com/media/catalog/product/s/g/sg90-servo-1_1.jpg) | Own SG90-branded servo with three white horn shapes; illustrates tapered forms and cup orientation | Product dimensions refer to the actuator. No numerical loose-horn profile was found in the inspected gallery. White-on-white details are too weak for precise hole/edge measurement. |
| Waveshare, [WS-MG90S product](https://www.waveshare.com/mg90s-servo.htm), [own black-horn kit photograph](https://www.waveshare.com/media/catalog/product/m/g/mg90s-servo-1.jpg) | Separate metal-output servo and black horn kit; clear raised cup openings, tapered arms and rounded ends. The asymmetric cross has one visible linkage opening in each short stub | This is a useful independent MG90S topology reference. It is not the user's exact measured specimen and does not publish loose-horn dimensions. Do not reuse its short-stub pattern for the Adafruit cross, which visibly differs. |
| DFRobot, [SER0006 product](https://www.dfrobot.com/product-255.html/) | Own product gallery and variant identity; current product is sold as a 1.3 kg 9g servo | Its actuator dimensions are not horn dimensions; older 1.6 kg references should not be silently merged with the current specimen. No additional horn geometry was established. |

The strongest new dimensional source is the SparkFun sheet, but it adds another
horn family rather than resolving the user's exact black specimen. Its small
plan views also illustrate a different hole arrangement. The REES52 image gives
useful independent overall sizes but does not determine thickness or a mating
cup. Adafruit's sharp closeups are valuable for shape and cavity construction,
with a demonstrably different single-arm hole count.

## Practical reconstruction boundary

For the immediate image-based model, keep a distinct `user supplied black horn`
source profile. Use its printed dimensions as reported, record silhouette and
hole positions as photo-derived estimates, and retain unknown manufacturer fit.
Label the model as a source-based estimated reference. Keep factory dimensions,
reported manual measurements, pixel-calibrated estimates and modeling omissions
separate in the stored provenance.

The first implementation should reproduce taper, round tip, cup side, stepped
central opening and 7/12 linkage-hole counts before adding dimples or cosmetic
teeth. New vendor or mold variants can then bring their own photographs,
dimensions and explicit fit evidence without changing this specimen's identity.
