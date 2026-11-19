#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$repo_root"
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
cargo_bin="${CARGO:-cargo}"
target_root="${CARGO_TARGET_DIR:-$repo_root/target}"
if [[ "$target_root" != /* ]]; then
    target_root="$repo_root/$target_root"
fi
legacy_target="$target_root/legacy"
nice "$cargo_bin" build --offline --locked --release --lib --target wasm32-unknown-unknown --features legacy-host --target-dir "$legacy_target"
nice "$cargo_bin" build --offline --locked --bin servo-assets --features legacy-host --target-dir "$legacy_target"
package_tmp="$(mktemp -d "$legacy_target/package-legacy.XXXXXX")"
trap 'rm -rf -- "$package_tmp"' EXIT
nice "$cargo_bin" run --offline --locked --manifest-path tools/componentize/Cargo.toml --target-dir "$target_root/componentize" -- \
    "$legacy_target/wasm32-unknown-unknown/release/solidcraft_servo_assets.wasm" "$package_tmp/servo-assets.wasm"
"$legacy_target/debug/servo-assets" manifest > "$package_tmp/plugin.json"
python3 tools/package-release.py stage --edition legacy --manifest "$package_tmp/plugin.json" --module "$package_tmp/servo-assets.wasm"
