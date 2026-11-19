# ROBOTIS horn corrections

Re-audit date: 2026-11-19. This records corrections to the first prototype. The
source audit contains the full provenance and representation limits. No vendor
media is included in the package.

## Removed unsupported constructions

| Record | Previous construction | Correction |
| --- | --- | --- |
| HNX330-N101 | Smooth spline cavity, guessed depth 2.9 | Cut removed; true axial transition remains unknown |
| HNX330-N101 | Offset diameter 1.5 relief cut through the whole horn | Cut removed; diameter/offset remain metadata, axial extent unknown |
| X330 plastic | Smooth diameter 5 cavity with guessed depth 2.75 | Cut removed; directly dimensioned 2.6 is now metadata, not a fabricated circular serration |
| HN12-N101 | Guessed central passage diameter 2.8 | Cut removed; diameter remains unknown |
| HN12-N101 | Smooth female-serration surrogate | Cut removed; source diameters/step remain metadata |
| HN12-I101 | Combined idler/bearing sleeve with invented span 8 | Entire solid removed; dimensions-only record |
| X330 bindings | Reused preview face Z26 and hid output-flange | Origin null; pose unknown; no display replacement |

The remaining three solids contain source-dimensioned outer cylinders and
confirmed openings or nominal thread-major references. Omitted cavities stay
unmodeled. They are partial source outer volumes, not manufacturing or mating
solids. No new guessed hole, depth, sleeve or hidden contour replaces an old
guess.

## Numeric source cross-check

All values were read from callouts on the actual rendered official pages, not
scaled from pixels. Local Z0 is the outer wide-flange face.

| Record | Published geometry used | Local nominal bounds |
| --- | --- | --- |
| [HNX330-N101, 2163](https://en.robotis.com/service/download.php?no=2163) | Diameter 16; plate 2.7; hub diameter 7.4; rear span 5.6; diameter 2.6 axial opening; counterbore diameter 5.6/depth 1.65; 4 x M2 x 0.4 through, PCD 12 | X/Y -8..8; Z -5.6..0 |
| [X330 plastic, 2129](https://en.robotis.com/service/download.php?no=2129) | Diameter 16; plate 2.9; reference hub (7.45); rear span 5.6; diameter 2.6 HOLE THRU; counterbore diameter 5.6/depth 1.6; 4 x diameter 1.6 through pilots, PCD 12 | X/Y -8..8; Z -5.6..0 |
| [HN12-N101, 1735](https://en.robotis.com/service/download.php?no=1735) | Diameter 19.5; plate 2; front boss diameter 8; span 4.2, hence boss `4.2 - 2 = 2.2` beyond flange; 8 x M2 x 0.4 through, PCD 16 | X/Y -9.75..9.75; Z -2..2.2 |
| [HN12-I101, 1757](https://en.robotis.com/service/download.php?no=1757) | Diameter 19.5; flange 2; section diameters 9.5/12.2/13; 8 x M2 x 0.4, PCD 16, TAP DP.2; sleeve length absent | No complete solid bounds asserted |

M2 cuts/sketch circles represent nominal thread-major diameters, not physical
smooth factory bores. The plastic source's asymmetric tolerances 16 +0/-0.2
and 5.6 +0/-0.15 remain in evidence notes. Its counterbore +/-0.2 is retained in
the symmetric tolerance field. No additional tolerances are invented.

## Corrected assembled registration

The [XM430/XH430 (HINGE) drawing](https://www.robotis.com/service/download.php?no=157),
dated 2019/03/19, supplies the case datums and installed-horn offsets. Its main
witness lines show two intervals: case face to outer flange **2**, then outer
flange to diameter 8 boss tip **2.2**. This agrees with the standalone normal
horn's `4.2 = 2 + 2.2` section.

| Location in existing XM430 frame | Correct nominal value | Former value |
| --- | --- | --- |
| Shared output/idler XY | `(11.25, 28.5/2) = (11.25, 14.25)` | Source centerline unchanged |
| Main flange face | Z`34 + 2 = 36`, direction +Z | Unsupported preview tip `35.2 + 2 = 37.2` |
| Main diameter 8 boss tip | Z`36 + 2.2 = 38.2`, excluding screw | Incorrectly 39.4 from the former pose |
| Rear idler flange face | Z`0 - 2 = -2`, direction -Z | Same number, now supported by explicit installed callout |

The idler's source-backed face position does not supply its missing sleeve
length. It remains dimensions-only. Nominal flange registration does not define
omitted teeth, washer/bolt shapes, fit tolerances or electrical zero.

All eight X330 mount origins are now null. The
[2020 X330 assembly reference](https://en.robotis.com/service/download.php?no=1986)
publishes legacy 23/3/26 and optional-idler total 29, but does not identify its
installed normal part as either exact 2022 individual horn revision. A metal
plate 2.7 or plastic plate 2.9 cannot be registered by simply reusing the older
3 mm projection. No automatic attachment or base-part replacement claim remains
for those unknown poses.

## Data and verification scope

The four canonical horn IDs, existing source identities and twelve compatibility
bindings are retained. All four data revisions are 2. A new unique source entry
records the already inspected X430 assembly PDF. There are eight unknown poses
and four documented nominal X430 poses. The pack release is unchanged for the
parent task's release process.

Only the owned horn catalog and audit documents were edited. No shared Rust,
tests, generated release or build output was changed by this re-audit. Geometry
execution is left to the parent task after its schema/runtime update.
