# Horn corrections in version 0.8.0

Version 0.8.0 withdraws unsupported horn geometry and installed positions from
0.7. A warning or amber appearance did not make those invented values suitable
for mechanical layout. The original servo previews retain their documented
limitations; this correction does not certify all servo bodies or cable outlets.

## Concrete corrections

| Earlier construction | Source review and current treatment |
| --- | --- |
| AU horns used 3.2-wide strips and R1.6 ends | No source establishes those values. Remove the solids; retain source dimensions and located hole circles. Taper angle alone does not fix the outline. |
| AU retaining hole2 and rear opening5.2 | Both unsupported; remove. The user's image does not identify these AU parts. |
| AU horns proposed for SG90/SG92R/MG90S | No named fit evidence; remove all12 bindings. |
| Hitec M-O24 included four invented20-degree holes | Remove all four cuts; retain only source-located cardinal centers. |
| Nominal spline classes became smooth bores | Remove the surrogate cavities. Published class/section information remains data, without inventing tooth or bore geometry. |
| Unknown horn positions used preview shaft tip plus plate thickness | Remove the coordinates. Unknown placement has null origin and cannot create an assembly or replace a servo reference part. |
| XM430 normal horn outer face at Z37.2 | Assembly drawing gives case front Z34 plus2: outer face Z36; outward boss tip Z38.2. Correct the model. |
| ROBOTIS idlers used guessed sleeves/openings | Remove their solids. Keep published hole patterns and section dimensions; missing axial length remains unknown. |
| HN13 normal holes used old blind-depth assumptions | Individual May2019 drawing calls the normal pattern THRU. Use its actual2.6 flange and4.9 axial extent, while retaining the dated source discrepancy explicitly. |
| STS horn fit transferred to the lightguide case | The case notice does not prove fit; remove both proposals. A/0 compatibility remains documented, with all installed positions null. |

The unidentified photograph prints double-arm length31.2, plate-edge span2.36
and axial span4.72; the single-arm labels are21.3,2.08 and4.82. These are separate
photo annotations, not revisions to the AU32/19.5 and plate1.5 drawings. Error
bounds, manufacturer, complete contour, hole origins, spline limits and seats
are not established. No complete 3D reconstruction of those photographed horns
is claimed. The [micro correction audit](horn-micro-correction-audit.md) reads
every printed label, including the local2.43 spacing and ambiguous openings.

## Usable references and gates

The four packs contain18 source records:11 dimension-only and7 partial outer
solids. `horn-reference` defaults to separate known reference circles and source
annotations. Its solid mode refuses dimension-only records before host commands.
Hole-reference sketches and annotation sketches are separate.

Automatic assembly requires named compatibility, a source-located installed
outer face and a supported partial solid. Four normal-horn combinations qualify:
HN12-N101 on XM430-W350-T/R and HN13-N101 on XM540-W270-T/R. No unknown seat is
filled to make the menu choice work. Partial outer solids omit listed cavities,
teeth and small features; they are not complete contact or clearance models.

## Verification and limits

Both plugin editions pass43 Rust unit tests and clippy with warnings denied.
The refusal regression fails against the saved0.7 source and passes after the
fix. Snapshot parameters are checked as valid replay inputs.

Real CAD evidence includes18 detached dimension references with zero bodies,
7 closed partial solids, actual cylindrical hole centers/diameters, and25 saved
source snapshots. A separate assembly comparison checks both supported normal
horns, their actual transformed bounds, owned names and closed bodies.

The initial87-case suites ran with a15-second command limit. Modern passed85
and timed out in2 XM430 cases; legacy passed86 and timed out in1. The declared
limit was raised to30 seconds. Wasm components and all catalogs are byte-for-byte
unchanged by that adjustment. Both XM430 base models and both normal-horn
assemblies were rerun under the final limit in each edition, with10 passes,
0 failures and1 unrelated entry skipped for missing default inputs. Earlier
passing geometry cases remain evidence for the identical component/catalogs.

Screenshots are native SolidCraft renders. Closed bodies and matching source
primitives do not establish physical dimensions by themselves: the separate
source audits provide the interpretation, and missing features remain missing.
No vendor artwork, external CAD model or photographed product is redistributed.
