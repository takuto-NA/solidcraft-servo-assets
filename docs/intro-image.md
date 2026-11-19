# Introduction image

The README overview presents the servo reference library, its useful interfaces
and the different horn evidence levels. Every geometry image is an actual
SolidCraft capture of the released v0.9.1 plugin. The original SVG supplies the
layout and captions; the CAD captures are embedded without changing their shapes.

- [PNG overview](https://github.com/takuto-NA/solidcraft-servo-assets/releases/download/v0.9.1/servo-assets-overview-v1.png)
- [Editable SVG with embedded captures and camera metadata](https://github.com/takuto-NA/solidcraft-servo-assets/releases/download/v0.9.1/servo-assets-overview-v1.svg)

The files are additional documentation assets. The existing release ZIPs,
checksums and version tag retain their original contents. The image and its
original layout use MIT OR Apache-2.0; see [ATTRIBUTION](../ATTRIBUTION.md).

## What the image claims

There are 28 integrated references, 29 servo source profiles and 18 horn variants.
These counts include case, document and electrical variants; they are not a count
of 28 unrelated mechanical designs. Eight representative references are shown:

| Caption | Canonical asset ID | Important limit |
| --- | --- | --- |
| SG90 Analog | `towerpro.sg90-analog.configure` | Mount and shaft positions are previews |
| SG92R | `towerpro.sg92r.configure` | Mount and shaft positions are previews |
| MG90S | `towerpro.mg90s.configure` | Mount and shaft positions are previews |
| FS90 | `feetech.fs90.akizuki-drawing` | Source mounting/output datums; simplified case registration and contours |
| STS3215-C001 | `feetech.sts3215-c001.a0` | Old-case profile; output and socket placement remain previews |
| XL330-M288-T | `robotis.xl330-m288-t.brochure` | Source case datums; simplified contours and connector geometry |
| XM430-W350-T | `robotis.xm430-w350-t.drawing` | Source case datums; provisional bare coupling and sockets |
| XM540-W270-T | `robotis.xm540-w270-t.reference-20190318` | Source case datums; provisional coupling, idler and sockets |

The main eight views share a calibrated orthographic scale. The interface and
horn views are enlarged independently; their sizes cannot be compared with the
main grid or each other.

The FS90 datum detail uses `output-reference`, which already includes the
mounting-pattern references. It uses the `near_mount_center` source frame:
27.7 ±0.1 mm between mounting centers and an 8.7 ±0.1 mm main-axis offset from
the near mounting center. Reference circles do not reproduce complete slot
mouths or establish a drilling fit. The shaft cylinder is an envelope, not a spline.
See [the source audit](sources.md).

The fixed-lead detail is the FS90 default three-conductor, 3 mm display marker.
The source establishes three conductors; the displayed outlet position, wire
size and marker length are planning choices. It is not the published lead length.
The STS3215 detail shows two separate three-contact receptacles on the rear/idler
side, without attached cables. Their source counts do not verify the displayed
connector dimensions or pose.

The horn examples deliberately show two different evidence levels:

- `robotis.hn12-n101.normal-2019`, `source-solid`: a partial source solid, with
  omitted mating details. See [the HN12 audit](horn-robotis-430-corrections.md).
- `unidentified.photo-se22ca210.double-arm`, `photo-solid`: an identified-photo
  reconstruction with twelve linkage holes. Manufacturer, physical part identity,
  spline fit and installed seat remain unknown. See [the reconstruction](sg90-photo-reconstruction.md).

The total 18 horn records comprise nine dimension-only variants, seven partial
source solids and two photo estimates. The image does not advertise 18 complete
manufacturing models. Part colors are display choices; the captions and source
records carry the evidence meaning.

## Reproduce the captures

Use a compatible SolidCraft CLI and the released v0.9.1 package. Its source commit
is `a9b5044289ab16031cba912a729622fea39f9580`. Keep the package directory named
`servo-assets`, matching its manifest ID. Work in a fresh design for each tile.

Insert `servo-reference` with `labels: "none"` and its default short cable marker.
Place each case base at world Z0 and center its case width on Y0. The helper
`tools/build-lineup.py` computes the same placement from the packaged housing
bounds and verifies closed bodies and exact recipe bounds.

Every servo tile uses a 600 × 375 pixel capture, `yaw: 0`, `pitch: 0.83`,
`distance: 95`, and `fov: 0` (orthographic). Target X is that case's center;
target Y is its row center, and target Z is 20 mm above the shared base. The
same SVG display size, 412 ×257.5, is used for every tile. No per-model auto-fit
or independent resizing is applied. The SVG metadata records the exact targets
and canonical IDs. Hide the grid and origin axes for these captures.

The detail views use fresh designs with these entries and cameras. Their original
captures are 900 ×700 pixels, with grid and origin axes hidden:

| View | Entry and parameters | Camera: target; yaw; pitch; distance |
| --- | --- | --- |
| FS90 datums | `output-reference`, FS90 ID above | `[13.85,0,4]`; `-0.35`; `0.95`; `42` |
| Fixed cable entry | `servo-reference`, FS90 ID, `labels: "none"` | `[2,0,-13.7]`; `0.8`; `0.32`; `20` |
| Rear sockets | `servo-reference`, STS3215 old-case ID, `labels: "none"` | `[27,12.35,3]`; `0`; `-1.12`; `36` |
| Partial source horn | `horn-reference`, HN12 ID above, `mode: "source-solid"` | `[0,0,1]`; `0`; `1.0`; `40` |
| Photo-estimated horn | Released `sg90-photo-horns.sc`, presentation labels hidden | `[0,0,1]`; `0`; `1.0`; `48` |

All detail cameras also use `fov: 0`. The photo demo turns the mating side upward
for inspection; this is a detached display orientation, not an installed pose.
Hide the source-dimension annotation sketch for the partial horn close-up; its
source metadata and geometry remain intact.

Edit the SVG captions or replace captures with their source and camera metadata
retained. Export the static SVG to PNG; the published PNG was rendered with
resvg 0.50.0 and the system Noto Sans text font. Neither renderer nor fonts are
new plugin runtime dependencies. Keep intermediate captures outside tracked
source directories, and attach reviewed media as release assets with fresh names.
