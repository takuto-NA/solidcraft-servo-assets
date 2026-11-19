# Building and verifying releases

The release workflow builds two offline plugin editions and an explicit source
archive. It creates a new **draft** GitHub Release after the complete gate passes.
Publishing the reviewed draft is a separate maintainer action.

## Local builds

Use Rust with the `wasm32-unknown-unknown` target, Rustfmt, Clippy, and Python 3.11
or newer. Fetch dependencies once while network access is available:

```sh
cargo fetch --locked
cargo fetch --locked --manifest-path tools/componentize/Cargo.toml
```

The existing entry points remain:

```sh
./build.sh
./build-legacy.sh
```

Both scripts build with `--offline --locked`. They preserve `CARGO` and
`CARGO_TARGET_DIR` overrides. The modern package is staged in `dist/servo-assets`;
the legacy package is staged in `dist/legacy/servo-assets`. Each staging directory
is replaced with a fresh package, so deleted fixtures and catalog files cannot
survive from an older build. Build intermediates stay under `target`, including
the component encoder's build directory.

`cargo xtask ci` is the complete check gate and builds both editions. Local
development builds may run before the first commit or with uncommitted changes;
their provenance records that state. Release verification requires a clean,
committed checkout.

## Release files

For version `0.9.1`, the generated files are:

| File | Contents |
| --- | --- |
| `servo-assets-0.9.1.zip` | Modern `commands@0.2` and `storage@0.1` plugin; package files at ZIP root |
| `servo-assets-legacy-0.9.1.zip` | `commands@0.1` plugin, with the existing `servo-assets/` ZIP wrapper |
| `servo-assets-source-0.9.1.zip` | Explicit source tree under `solidcraft-servo-assets-0.9.1/` |
| Each ZIP's `.zip.sha256` file | SHA256 of that ZIP |
| `SHA256SUMS.txt` | Checksums for all three ZIPs |
| `release-provenance.json` | Source commit and ZIP hashes for the batch |

Plugin packages contain the Wasm component, compact manifest, licenses,
attribution, README, contribution instructions when present, documentation,
catalog JSON and test fixtures. Each contains `BUILD-INFO.json` with SDK edition,
version, commit/dirty state, source-file hashes and payload hashes. Manifests must
remain at most 16 KiB. Legacy packages request no design storage commands;
their source snapshots are returned to callers rather than embedded in designs.

The source archive includes Cargo manifests and both lockfiles, all `src/*.rs`
and nested Rust source, both WIT editions, catalog packs, fixtures, Markdown docs,
build scripts, the explicit development tools, component-encoder source, xtask,
`.cargo/config.toml`, contribution templates and GitHub workflows. Its
`SOURCE-MANIFEST.json` inventories every included source file and digest.

The allowlist is in `tools/package-release.py`. It excludes `target`, `dist`,
`evidence`, `.git`, vendor media, compiled binaries, private environment files and
unlisted paths. Source additions outside the allowlist require a deliberate
allowlist update. Clean release verification also requires every included source
file to be tracked. Screenshots or original CAD examples can be attached to a
release separately after public-content review; they are not copied wholesale
from a local evidence directory.

## Preparing a tagged release

Update the package version and lockfile together, complete the gate, and commit
the reviewed source. The tag must be exactly the Cargo version prefixed with
`v`, for example `v0.9.1`, and must point at that commit. Never move an existing
published version tag or replace its assets.

The workflow runs on a pushed `v*` tag or `workflow_dispatch` with an existing
version tag. It checks the tag, version and commit, fetches locked dependencies,
runs `cargo xtask ci`, and then runs the equivalent of:

```sh
release_commit="$(git rev-parse HEAD)"
python3 tools/package-release.py source --require-clean --commit "$release_commit"
python3 tools/package-release.py checksums --require-clean --commit "$release_commit"
python3 tools/package-release.py verify --require-clean --commit "$release_commit" --require-checksums
```

The two editions and source archive must have identical source inventories and
commit provenance. A source change between builds causes verification to fail.
ZIP members and timestamps are generated deterministically from the selected
inputs and commit date. This does not claim byte-identical Wasm across different
Rust toolchains.

Build and verification jobs have read-only repository access. Only the draft
release job has `contents: write`. It verifies the downloaded artifacts and the
remote tag again, then uses `gh release create --draft --verify-tag`. An existing
draft or published release causes a refusal; no workflow step uses `--clobber`,
updates tags, or publishes automatically.

## Reviewing and publishing the draft

Check the workflow run's source SHA and the draft's tag. Download the three ZIPs,
their checksum files, `SHA256SUMS.txt` and `release-provenance.json`. From their
download directory run:

```sh
sha256sum --check SHA256SUMS.txt
```

Check that the provenance commit equals the tag's commit. To verify against a
clean checkout of that same tag, use:

```sh
python3 tools/package-release.py --out /path/to/downloads verify \
  --require-clean --commit "$(git rev-parse HEAD)" --require-checksums
```

Inspect each manifest's version, capabilities and byte size. Confirm the modern
and legacy editions install and run on their intended hosts, including the
packaged fixture checks. Review any separately attached CAD screenshots/examples
and retain their source/estimate labels. Then an authorized maintainer may publish
the verified draft, for example with `gh release edit v0.9.1 --draft=false`.
Recheck that the published tag and files still match the recorded commit and
checksums.

If a release already exists, investigate its state rather than rerunning an
upload with replacement enabled. Published corrections use a new version.
