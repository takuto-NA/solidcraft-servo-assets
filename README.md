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

[Download a release](https://github.com/takuto-NA/solidcraft-servo-assets/releases)
· [Usage and model limits](docs/usage.md)
· [Add a servo or horn](docs/adding-assets.md)

## Install and use

Choose the ZIP matching your SolidCraft host's command SDK:

| Package | Host capability | Source snapshots |
| --- | --- | --- |
| `servo-assets-VERSION.zip` | `commands@0.2`, `storage@0.1` and support for 64 KiB manifests | Embedded in the saved design |
| `servo-assets-legacy-VERSION.zip` | `commands@0.1` | Returned to the caller; not embedded by the older host |

Install the matching ZIP through SolidCraft's plugin manager. The release also
provides a source ZIP and SHA-256 checksums. Packages are sandboxed and need no
network or filesystem permissions at runtime.

Open **Insert → Servo Assets → Insert servo** and choose the full model and
variant. The default fixed cable shows three short conductors at the entry.
Serial servos show their detachable sockets. **Insert horn** lets you choose a
loose horn and its available representation together. In **Inspect**, use
**Insert mounts or axis** for a source-defined mounting face or axis.

Use **Preview** to check the placement before committing. Hosts with the catalog
form extension also show a search box, a summary of the selection and an
**Advanced** section for optional offsets. Existing hosts show the same catalog
choices and the offset fields in a regular form. The Plugins menu has **Find
assets**, **Read servo sources**, and the detailed setup commands.
[Insertion guide and UX research](docs/insertion-workflow.md).

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
archives and tag/release verification. The forms are generated from the same
catalog as the geometry. Fields switch to text input if their catalog exceeds
the host manifest budget.

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
