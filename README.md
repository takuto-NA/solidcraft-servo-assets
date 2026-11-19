# Servo Assets for SolidCraft

An offline Rust/WebAssembly plugin that inserts editable servo and horn references
using ordinary SolidCraft commands. Body shapes, mounting references, output axes
and cable entrances have separate evidence and explicit unknowns.

**28 integrated servo models, 29 source profiles and 18 horn variants** are
included. The catalog covers SG90, SG92R, MG90S, other common TowerPro/FEETECH/Hitec
servos, STS3215 variants, XL330/XC330, XM430 and XM540. Horns include dimension
references, supported partial solids and two explicitly photo-estimated SG90-style
arms. Vendor, document and physical revisions remain separate identities.

[Download a release](https://github.com/takuto-NA/solidcraft-servo-assets/releases)
· [Usage and model limits](docs/usage.md)
· [Add a servo or horn](docs/adding-assets.md)

![Two photo-estimated SG90-style horns inserted and rendered in SolidCraft](https://github.com/takuto-NA/solidcraft-servo-assets/releases/download/v0.9.1/sg90-photo-iso.png)

The image is an actual SolidCraft capture of the two photo reconstructions. Their
contours and linkage holes are inferred from an identified photograph; spline fit
and installed seating are unknown. [Evidence and calibration](docs/sg90-photo-reconstruction.md).

## Install and use

Choose the ZIP matching your SolidCraft host's command SDK:

| Package | Host capability | Source snapshots |
| --- | --- | --- |
| `servo-assets-VERSION.zip` | `commands@0.2` and `storage@0.1` | Embedded in the saved design |
| `servo-assets-legacy-VERSION.zip` | `commands@0.1` | Returned to the caller; not embedded by the older host |

Install the matching ZIP through SolidCraft's plugin manager. The release also
provides a source ZIP and SHA-256 checksums. Packages are sandboxed and need no
network or filesystem permissions at runtime.

Use **Servo reference** to insert an integrated model. The default fixed-cable
representation shows a short entrance marker with three conductors and no end
connector. Serial servos show their detachable sockets without invented attached
cables. **Mount pattern**, **Output reference** and **Cable space** expose separate
datums or caller-sized routing allowances. **Horn reference** provides explicit
`dimensions`, `source-solid` and `photo-solid` modes.

Amber parts and PREVIEW/APPROX labels identify planning geometry. These are
reference assets, with model-specific missing details; a partial model is not a
verified drilling template, manufacturing fit or conservative clearance bound.
Read the [model limits](docs/usage.md) and the selected record's evidence.

## Build and validate

Use current stable Rust, Python 3.11 or newer and the Wasm target. Fetch dependencies before
the offline package builds:

```sh
rustup target add wasm32-unknown-unknown
cargo fetch --locked
cargo fetch --locked --manifest-path tools/componentize/Cargo.toml
cargo xtask ci
```

The check formats/lints/tests the Rust workspaces, validates catalog data and host
fixtures, and builds both editions. With a compatible CLI, include actual host
checks:

```sh
SOLIDCRAFT_CLI=/absolute/path/to/solidcraft-cli cargo xtask ci
```

`./build.sh` and `./build-legacy.sh` build either edition separately. Artifacts go
in `dist/`. See [releasing](docs/releasing.md) for reproducible packaging, source
archives and tag/release verification. Catalog growth beyond the host form budget
falls back to canonical-ID fields; runtime validation remains in place.

## Contribute

New hardware, vendor variants, horn shapes, source corrections and specimen
measurements are welcome. Many additions use JSON without new runtime code.
Start with [the asset guide](docs/adding-assets.md), then open a focused pull
request. [CONTRIBUTING](CONTRIBUTING.md) explains the checks and CAD evidence.
Issues have forms for new assets and geometry problems; no CLA or claim step is
required.

Keep facts, printed image annotations and grounded estimates distinct. A cable
entrance, conductor count, socket contact count, mounting frame and horn seat are
individual claims. Unknown values stay unknown. New families, including future
QDD models, can add independent source profiles and geometry adapters without
rewriting existing saved instances.

Useful references: [servo sources](docs/sources.md),
[ROBOTIS sources](docs/robotis-sources.md), [horn schema](docs/horn-data-schema.md)
and [extension design](docs/design.md).

## License and provenance

Original code, catalog transcription and documentation are **MIT OR Apache-2.0**.
See [ATTRIBUTION](ATTRIBUTION.md), [dependency notices](docs/third-party-notices.md)
and the license files. Vendor PDFs, photographs,
artwork and CAD files remain external links. Hardware names identify referenced
products and do not imply vendor endorsement or verified interchangeability.
