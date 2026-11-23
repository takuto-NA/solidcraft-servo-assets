# Servo Assets for SolidCraft

An offline Rust/WebAssembly plugin that inserts editable servo and horn references
using ordinary SolidCraft commands. Body shapes, mounting references, output axes
and cable entrances have separate evidence and explicit unknowns.

[![FEETECH FS90 CAD reference with both mounting slots, the output axis and the approximate three-conductor cable entry. SG90, STS3215 and XM430 at one scale, with a detached SG90-style photo-estimated horn.](https://github.com/takuto-NA/solidcraft-servo-assets/releases/download/v0.10.0/servo-assets-readme-hero-v2.png)](https://github.com/takuto-NA/solidcraft-servo-assets/releases/download/v0.10.0/servo-assets-readme-hero-v2.png)

[Image sources and the full comparison](docs/intro-image.md)

**28 integrated servo references, 29 source profiles and 18 horn variants** are
included. The catalog covers SG90, SG92R, MG90S, other common TowerPro/FEETECH/Hitec
servos, STS3215 variants, XL330/XC330, XM430 and XM540. Horns include dimension
references, supported partial solids and two explicitly photo-estimated SG90-style
arms. Vendor, document and physical revisions remain separate identities.

[Download and install](#install-and-use)
· [Usage and model limits](docs/usage.md)
· [Add a servo or horn](docs/adding-assets.md)

## Install and use

For **SolidCraft 0.47**, use the standard **`servo-assets-0.13.0.zip`**.
`./build.sh` creates it in `dist/`. Open **Preferences → Plugins → Install
plugin…** and choose the ZIP. The package embeds source snapshots in the saved
design and uses model filters in the native insertion forms.

For an older host, `./build-legacy.sh` builds the compatibility edition with a
16 KiB manifest. For the published v0.10.0 release on 0.47, use the
[v0.10.0 compatibility ZIP](https://github.com/takuto-NA/solidcraft-servo-assets/releases/download/v0.10.0/servo-assets-legacy-0.10.0.zip).
[GitHub Releases](https://github.com/takuto-NA/solidcraft-servo-assets/releases)
also provides source ZIPs and SHA-256 checksums.

On the **Model** tab, click **Servo model** in the **Plugins** group. Filter by
model number, brand or SKU, select the exact variant and click **Run**. Select
the named Browser component to move the housing, shaft and cable together.
Use **Horn model** to insert a separate 3D horn by name.
[Insertion guide and UX research](docs/insertion-workflow.md)

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
in `dist/`. Add `--tools` for an automation package with the detailed commands
under `target/tooling`. See [releasing](docs/releasing.md) for reproducible
packaging, source archives and tag/release verification.

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
