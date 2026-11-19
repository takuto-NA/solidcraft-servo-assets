# Contributing to Servo Assets

New servo profiles, horn variants, measurements, source corrections and clear bug
reports are welcome. Many additions need only JSON and a short source note; you
do not need to write Rust to improve the catalog. Incomplete evidence is useful
when the unknowns are recorded honestly.

Start with [adding an asset](docs/adding-assets.md). For a geometry problem, open
an issue with the selected IDs, parameters, expected result and an actual CAD
screenshot. A source link or a measured specimen can also start a useful issue.

## A simple contribution flow

1. Check existing issues and pull requests for related work.
2. Fork the repository and create a branch for one asset, correction or feature.
3. Make the change and run the relevant checks below.
4. Open a pull request describing the result, evidence and remaining unknowns.

You can open a pull request directly. An issue, draft claim or separate approval
is not required before starting. Small pull requests are easier to review. If
you are unsure how to encode a source, open an issue with what you have; someone
can help with the JSON.

```sh
git clone https://github.com/YOUR-USER/solidcraft-servo-assets.git
cd solidcraft-servo-assets
git switch -c add-servo-profile
rustup target add wasm32-unknown-unknown
cargo fetch --locked
cargo fetch --locked --manifest-path tools/componentize/Cargo.toml
cargo xtask ci
```

Use current stable Rust. The first build needs the dependencies and Wasm target;
later builds can use the local cache. See [README](README.md) for package and
host compatibility. Do not commit `target/`, generated packages, downloaded
vendor material or machine-specific configuration.

## Evidence that makes an asset useful

Keep manufacturer, exact model, seller SKU, document revision and physical
revision distinct. A new document date does not prove a new factory revision.
Different vendors' products with the same model name can need separate records.
Do not replace an unknown with another model's dimension.

Every numeric fact needs its source, location, method and interpretation. Prefer
manufacturer documents, manufacturer-authored sheets hosted by a supplier, or
your own identified specimen measurements. Preserve conflicting values and
explain which source a representation uses. A missing tolerance stays unknown.

Grounded photo reconstruction is welcome. Keep printed annotations in
`reported_*` fields and inferred features in `estimates`, with image anchors,
calibration and an explanation of each modeling choice. A rounded guess without
evidence is not a measurement. A photo estimate can be a useful 3D asset without
claiming a verified spline, manufacturing fit or installed seat. See the
[horn schema](docs/horn-data-schema.md) and
[photo reconstruction](docs/sg90-photo-reconstruction.md).

For mounting and interference work, include more than the outside box: shaft
axes, mounting frames, hole/thread meaning and cable entrances matter. Separate
socket contact count from attached cable count. Keep horn compatibility separate
from installed position. The [asset guide](docs/adding-assets.md) gives examples
and the required distinctions.

## Validation and visible results

Run the repository check before submitting an implementation or catalog change:

```sh
cargo xtask ci
```

When a compatible SolidCraft CLI is available, enable the host checks:

```sh
SOLIDCRAFT_CLI=/absolute/path/to/solidcraft-cli cargo xtask ci
```

Report the commands you ran and their results. Say which checks were unavailable
instead of reporting them as passed. Native tests do not replace insertion and
save/reopen checks in the actual host. New host fixtures belong in `tests/`;
Rust regressions belong in the existing test module for that area.

For a visible model change, insert the asset through SolidCraft and capture your
own CAD screenshot. Show the views needed to inspect the change: output face,
rear face and cable entrance are often more useful than a single front view.
Check dimensions, hole positions, body roles, labels, uncertainty and saved
metadata. Use before/after views for a correction when practical. A rendered
mockup or vendor photograph is not evidence that the plugin created the model.
Documentation-only changes do not need an artificial screenshot.

Attach review screenshots or a short recording to the pull request. Remove
private designs, account details, tokens and unrelated machine paths. Include a
small reproducible script or the parameters used for the capture.

## Code and independence

The product runtime is Rust and WebAssembly. Python scripts are development
helpers for data generation and evidence, not application logic. Programmatic
commands must not open dialogs. Validate hostile input and return errors; do not
introduce `unsafe`, `unwrap`, `expect` or panics into non-test Rust.

Use original code and geometry constructed from public facts, measurements and
documented inference. Do not obtain code or geometry from commercial CAD
products. Do not read, copy or depend on GPL, LGPL or AGPL implementations,
including FreeCAD/OpenCASCADE-derived models or tables. Keep vendor images,
drawings, PDFs and CAD downloads outside the repository; record links and facts
instead. See [ATTRIBUTION](ATTRIBUTION.md) for the project's source policy.

Contributions use the repository's **MIT OR Apache-2.0** terms. Do not copy vendor
text, artwork or templates into a contribution. Credit any permitted third-party
material explicitly. If dependencies change, update the lockfiles and the notices
in `docs/third-party-notices.md`. There is no separate contributor agreement or
CLA step.

Write code, data notes, documentation and pull requests in plain English. Keep
review discussion concrete and considerate, especially when a source is
ambiguous. Corrections and additional measurements are part of improving the
catalog, not a reason to hide uncertainty.
