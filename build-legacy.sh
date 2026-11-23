#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$repo_root"
package_out="$repo_root/dist"
case "$#:${1:-}" in
    0:) ;;
    1:--tools) package_out="$repo_root/target/tooling" ;;
    *) printf 'Usage: %s [--tools]\n' "$0" >&2; exit 2 ;;
esac
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
cargo_bin="${CARGO:-cargo}"
target_root="${CARGO_TARGET_DIR:-$repo_root/target}"
if [[ "$target_root" != /* ]]; then
    target_root="$repo_root/$target_root"
fi
legacy_target="$target_root/legacy"
wasm_rustflags="$(python3 - "$repo_root" "${CARGO_HOME:-$HOME/.cargo}" <<'PY'
import os
import sys
encoded = os.environ.get("CARGO_ENCODED_RUSTFLAGS")
flags = (encoded.split("\x1f") if encoded else []) if encoded is not None else os.environ.get("RUSTFLAGS", "").split()
flags.extend(f"--remap-path-prefix={old}={new}" for old, new in ((sys.argv[1], "/source/solidcraft-servo-assets"), (os.path.abspath(sys.argv[2]), "/cargo")))
sys.stdout.write("\x1f".join(flags))
PY
)"
CARGO_ENCODED_RUSTFLAGS="$wasm_rustflags" nice "$cargo_bin" build --offline --locked --release --lib --target wasm32-unknown-unknown --features legacy-host --target-dir "$legacy_target"
nice "$cargo_bin" build --offline --locked --bin servo-assets --features legacy-host --target-dir "$legacy_target"
package_tmp="$(mktemp -d "$legacy_target/package-legacy.XXXXXX")"
trap 'rm -rf -- "$package_tmp"' EXIT
nice "$cargo_bin" run --offline --locked --manifest-path tools/componentize/Cargo.toml --target-dir "$target_root/componentize" -- \
    "$legacy_target/wasm32-unknown-unknown/release/solidcraft_servo_assets.wasm" "$package_tmp/servo-assets.wasm"
"$legacy_target/debug/servo-assets" manifest "$@" > "$package_tmp/plugin.json"
python3 tools/package-release.py --out "$package_out" stage --edition legacy --manifest "$package_tmp/plugin.json" --module "$package_tmp/servo-assets.wasm"
