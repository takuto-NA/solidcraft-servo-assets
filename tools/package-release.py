#!/usr/bin/env python3
"""Create and verify Servo Assets archives using only the Python standard library."""

import argparse
import hashlib
import json
import os
import re
import shutil
import stat
import subprocess
import tempfile
import time
import tomllib
import zipfile
from pathlib import Path, PurePosixPath


REPO = Path(__file__).resolve().parents[1]
ROOT_REQUIRED = ("Cargo.toml", "Cargo.lock", "README.md", "ATTRIBUTION.md", "LICENSE-MIT", "LICENSE-APACHE", "rustfmt.toml", "build.sh", "build-legacy.sh")
ROOT_OPTIONAL = (".gitignore", "AGENTS.md", "CONTRIBUTING.md", "CHANGELOG.md", "SECURITY.md", "CODE_OF_CONDUCT.md", "rust-toolchain.toml", ".cargo/config.toml", ".github/PULL_REQUEST_TEMPLATE.md", ".github/pull_request_template.md")
SOURCE_TREES = {"src": {".rs"}, "catalog": {".json"}, "tests": {".json"}, "docs": {".md"}, "wit": {".wit"}, "wit-0.1": {".wit"}, "templates": {".json", ".md", ".toml"}, "xtask/src": {".rs"}, "tools/componentize/src": {".rs"}, ".github/workflows": {".yml", ".yaml"}, ".github/ISSUE_TEMPLATE": {".md", ".yml", ".yaml"}}
SOURCE_EXACT = ("tools/build-lineup.py", "tools/build-horn-lineup.py", "tools/build-horn-references.py", "tools/update-photo-horns.py", "tools/package-release.py", "tools/componentize/Cargo.toml", "tools/componentize/Cargo.lock", "xtask/Cargo.toml")
BLOCKED_PARTS = {"target", "dist", "evidence", "vendor", ".git", "__pycache__", "node_modules"}
COMPONENT_MAGIC = b"\0asm\x0d\0\x01\0"
MAX_ARCHIVE_BYTES = 64 * 1024 * 1024


def read_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def encoded(value):
    return (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def version(repo):
    value = tomllib.loads((repo / "Cargo.toml").read_text(encoding="utf-8"))["package"]["version"]
    if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?", value):
        raise ValueError("Cargo package version must be a literal semantic version")
    return value


def git(repo, *args):
    result = subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True)
    return result.stdout.strip() if result.returncode == 0 else None


def provenance(repo):
    top = git(repo, "rev-parse", "--show-toplevel")
    commit = git(repo, "rev-parse", "HEAD") if top and Path(top).resolve() == repo else None
    status = git(repo, "status", "--porcelain", "--untracked-files=all") if commit else None
    return {"commit": commit, "dirty": bool(status) if commit else True}


def checked_file(repo, path):
    if not path.is_file() or path.is_symlink() or any(parent.is_symlink() for parent in path.parents if parent != repo.parent):
        raise ValueError(f"Release input must be a regular file without symlink parents: {path.relative_to(repo)}")
    path.resolve().relative_to(repo)
    return path


def tree_files(repo, directory, suffixes):
    start = repo / directory
    if not start.exists():
        return []
    if start.is_symlink():
        raise ValueError(f"Release input directory is a symlink: {directory}")
    result = []
    for current, directories, files in os.walk(start, followlinks=False):
        for name in list(directories):
            path = Path(current) / name
            if path.is_symlink():
                raise ValueError(f"Release input directory is a symlink: {path.relative_to(repo)}")
            if name in BLOCKED_PARTS or name.startswith("."):
                directories.remove(name)
        for name in files:
            path = Path(current) / name
            if not name.startswith(".") and path.suffix.lower() in suffixes:
                result.append(checked_file(repo, path))
    return result


def source_files(repo):
    result = [checked_file(repo, repo / name) for name in ROOT_REQUIRED]
    result += [checked_file(repo, repo / name) for name in ROOT_OPTIONAL + SOURCE_EXACT if (repo / name).exists()]
    for directory, suffixes in SOURCE_TREES.items():
        result.extend(tree_files(repo, directory, suffixes))
    for name in ("tools/package-release.py", "tools/componentize/Cargo.toml", "tools/componentize/Cargo.lock", "src/lib.rs", "wit/plugin.wit", "wit-0.1/plugin.wit"):
        if repo / name not in result:
            raise ValueError(f"Missing source archive input: {name}")
    manifest = tomllib.loads((repo / "Cargo.toml").read_text(encoding="utf-8"))
    if "xtask" in manifest.get("workspace", {}).get("members", []) and repo / "xtask/Cargo.toml" not in result:
        raise ValueError("The declared xtask workspace member must be included in the source archive")
    return sorted(set(result), key=lambda path: path.relative_to(repo).as_posix())


def source_name_allowed(name):
    if name in ROOT_REQUIRED + ROOT_OPTIONAL + SOURCE_EXACT:
        return True
    for directory, suffixes in SOURCE_TREES.items():
        prefix = directory + "/"
        if name.startswith(prefix):
            relative = PurePosixPath(name[len(prefix):])
            if relative.suffix.lower() in suffixes and not any(part.startswith(".") for part in relative.parts):
                return True
    return False


def payloads(repo, paths):
    return {path.relative_to(repo).as_posix(): checked_file(repo, path).read_bytes() for path in paths}


def require_checkout(repo, paths, expected_commit=None):
    state = provenance(repo)
    if not state["commit"] or state["dirty"]:
        raise ValueError("A release requires a committed, clean repository checkout")
    if expected_commit and state["commit"] != expected_commit:
        raise ValueError("Release checkout does not match the expected commit")
    tracked = git(repo, "ls-files", "-z")
    tracked = set(tracked.split("\0")) if tracked is not None else set()
    untracked = [path.relative_to(repo).as_posix() for path in paths if path.relative_to(repo).as_posix() not in tracked]
    if untracked:
        raise ValueError(f"Release allowlist includes untracked or ignored files: {untracked[:5]}")
    return state


def epoch(repo):
    value = os.environ.get("SOURCE_DATE_EPOCH") or git(repo, "log", "-1", "--format=%ct") or "315532800"
    return max(315532800, min(int(value), 4354819199))


def atomic_write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as handle:
        temporary = Path(handle.name)
        handle.write(data)
    try:
        os.replace(temporary, path)
    finally:
        temporary.unlink(missing_ok=True)


def safe_name(name):
    path = PurePosixPath(name)
    return bool(name) and not path.is_absolute() and "\\" not in name and "\0" not in name and all(part not in {"", ".", ".."} and ":" not in part for part in name.split("/"))


def write_zip(path, files, timestamp):
    if not files or len(files) > 4096 or sum(len(data) for data in files.values()) > MAX_ARCHIVE_BYTES:
        raise ValueError("Release payload is empty or exceeds archive bounds")
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=path.parent, suffix=".zip", delete=False) as handle:
        temporary = Path(handle.name)
    try:
        with zipfile.ZipFile(temporary, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
            for name, data in sorted(files.items()):
                if not safe_name(name):
                    raise ValueError(f"Unsafe archive path: {name}")
                info = zipfile.ZipInfo(name, time.gmtime(timestamp)[:6])
                info.create_system = 3
                info.compress_type = zipfile.ZIP_DEFLATED
                mode = 0o755 if name.endswith(".sh") or "/tools/" in "/" + name and name.endswith(".py") else 0o644
                info.external_attr = (stat.S_IFREG | mode) << 16
                archive.writestr(info, data)
        verify_archive(temporary)
        os.replace(temporary, path)
    finally:
        temporary.unlink(missing_ok=True)
    atomic_write(path.with_suffix(".zip.sha256"), f"{digest(path.read_bytes())}  {path.name}\n".encode())


def validate_manifest(data, module, edition, expected_version):
    limit_kib = 16 if edition == "legacy" else 64
    if len(data) > limit_kib * 1024:
        raise ValueError(f"Manifest exceeds the {edition} host's {limit_kib} KiB limit")
    manifest = json.loads(data)
    if manifest.get("id") != "servo-assets" or manifest.get("version") != expected_version:
        raise ValueError("Manifest identity/version does not match Cargo.toml")
    runtime = manifest.get("runtime", {})
    if runtime.get("module") != "servo-assets.wasm" or not module.startswith(COMPONENT_MAGIC):
        raise ValueError("Expected the component-encoded servo-assets.wasm module")
    expected_caps = ["commands@0.2", "storage@0.1"] if edition == "modern" else ["commands@0.1"]
    if sorted(manifest.get("capabilities", [])) != sorted(expected_caps):
        raise ValueError(f"Wrong SDK capabilities for the {edition} edition")
    storage = {name for name in runtime.get("commands", []) if name.startswith("storage.")}
    if edition == "modern" and not {"storage.patch", "storage.put_blob"}.issubset(storage):
        raise ValueError("Modern edition must retain design snapshot storage commands")
    if edition == "legacy" and (storage or "data_format" in runtime):
        raise ValueError("Legacy edition cannot request design storage or data_format")
    if not runtime.get("entries"):
        raise ValueError("Plugin manifest has no entry points")


def verify_archive(path):
    with zipfile.ZipFile(path) as archive:
        members = archive.infolist()
        if len(members) > 4096 or sum(info.file_size for info in members) > MAX_ARCHIVE_BYTES:
            raise ValueError("Archive exceeds verification bounds")
        names = [info.filename for info in members]
        if len({name.casefold() for name in names}) != len(names):
            raise ValueError("Archive has duplicate or case-colliding paths")
        for info in members:
            if not safe_name(info.filename) or info.is_dir() or stat.S_ISLNK(info.external_attr >> 16):
                raise ValueError(f"Unsafe archive member: {info.filename}")
            if any(part in BLOCKED_PARTS for part in PurePosixPath(info.filename).parts):
                raise ValueError(f"Generated/private directory in archive: {info.filename}")
        files = {info.filename: archive.read(info) for info in members}
    build_names = [name for name in files if name in {"BUILD-INFO.json", "servo-assets/BUILD-INFO.json"}]
    source_names = [name for name in files if name.endswith("/SOURCE-MANIFEST.json")]
    if len(build_names) == 1 and not source_names:
        metadata_name = build_names[0]
        metadata = json.loads(files[metadata_name])
        edition = metadata.get("edition")
        prefix = "" if edition == "modern" else "servo-assets/" if edition == "legacy" else None
        if prefix is None or metadata_name != prefix + "BUILD-INFO.json" or metadata.get("kind") != "plugin":
            raise ValueError("Invalid SDK archive metadata/layout")
        if any(not name.startswith(prefix) for name in files):
            raise ValueError("Unexpected file outside the plugin root")
        relative = {name[len(prefix):]: data for name, data in files.items()}
        expected = metadata.get("files", {})
        if set(relative) != set(expected) | {"BUILD-INFO.json"}:
            raise ValueError("Plugin file inventory does not match its metadata")
        if any(digest(relative[name]) != value for name, value in expected.items()):
            raise ValueError("Plugin member digest mismatch")
        for name in relative:
            allowed = name in {"plugin.json", "servo-assets.wasm", "BUILD-INFO.json", "README.md", "ATTRIBUTION.md", "LICENSE-MIT", "LICENSE-APACHE", "CONTRIBUTING.md"} or any(name.startswith(folder + "/") and name.endswith(suffix) for folder, suffix in [("docs", ".md"), ("catalog", ".json"), ("tests", ".json")])
            if not allowed:
                raise ValueError(f"Unexpected plugin payload: {name}")
        validate_manifest(relative["plugin.json"], relative["servo-assets.wasm"], edition, metadata["version"])
    elif len(source_names) == 1 and not build_names:
        metadata_name = source_names[0]
        metadata = json.loads(files[metadata_name])
        prefix = f"solidcraft-servo-assets-{metadata['version']}/"
        if metadata.get("kind") != "source" or metadata_name != prefix + "SOURCE-MANIFEST.json":
            raise ValueError("Invalid source archive metadata/layout")
        expected = metadata.get("source_files", {})
        if set(files) != {prefix + name for name in expected} | {metadata_name}:
            raise ValueError("Source inventory does not match its metadata")
        if any(digest(files[prefix + name]) != value for name, value in expected.items()):
            raise ValueError("Source member digest mismatch")
        cargo = tomllib.loads(files[prefix + "Cargo.toml"].decode())
        if cargo["package"]["version"] != metadata["version"]:
            raise ValueError("Source Cargo version mismatch")
    else:
        raise ValueError("Expected one plugin or source manifest in the archive")
    if metadata.get("format") != 1 or not isinstance(metadata.get("source_files"), dict):
        raise ValueError("Unsupported archive provenance format")
    if any(not source_name_allowed(name) for name in metadata["source_files"]):
        raise ValueError("Source inventory contains a file outside the explicit release allowlist")
    return metadata


def archives(out, release_version):
    return [out / f"servo-assets-{release_version}.zip", out / f"servo-assets-legacy-{release_version}.zip", out / f"servo-assets-source-{release_version}.zip"]


def verify_all(repo, out, commit=None, clean=False, consolidated=True, require_checksums=False):
    release_version = version(repo)
    paths = archives(out, release_version)
    metadata = []
    for path in paths:
        info = verify_archive(path)
        if info["version"] != release_version:
            raise ValueError("Archive version differs from the release checkout")
        checksum = path.with_suffix(".zip.sha256").read_text(encoding="utf-8")
        if checksum != f"{digest(path.read_bytes())}  {path.name}\n":
            raise ValueError(f"Checksum mismatch: {path.name}")
        if commit and info["git"]["commit"] != commit or clean and (not info["git"]["commit"] or info["git"]["dirty"]):
            raise ValueError(f"Archive was not built from the required clean commit: {path.name}")
        metadata.append(info)
    if [info.get("edition", info["kind"]) for info in metadata] != ["modern", "legacy", "source"]:
        raise ValueError("The release must contain both SDK editions and the source archive")
    if any(info["source_files"] != metadata[0]["source_files"] or info["git"] != metadata[0]["git"] for info in metadata):
        raise ValueError("SDK editions and source archive came from different source snapshots; rebuild the batch")
    if clean:
        require_checkout(repo, source_files(repo), commit)
        current = {name: digest(data) for name, data in payloads(repo, source_files(repo)).items()}
        if current != metadata[0]["source_files"]:
            raise ValueError("Release checkout contents differ from the archived source snapshot")
    sums = out / "SHA256SUMS.txt"
    if consolidated and (sums.exists() or require_checksums):
        expected = "".join(f"{digest(path.read_bytes())}  {path.name}\n" for path in sorted(paths))
        if sums.read_text(encoding="utf-8") != expected:
            raise ValueError("SHA256SUMS.txt does not match the complete release batch")
    return paths, metadata


def stage(repo, out, edition, manifest_path, module_path):
    release_version = version(repo)
    source = payloads(repo, source_files(repo))
    package = payloads(repo, [repo / name for name in ("README.md", "ATTRIBUTION.md", "LICENSE-MIT", "LICENSE-APACHE")])
    if (repo / "CONTRIBUTING.md").exists():
        package.update(payloads(repo, [repo / "CONTRIBUTING.md"]))
    for directory, suffix in [("docs", ".md"), ("catalog", ".json"), ("tests", ".json")]:
        package.update(payloads(repo, tree_files(repo, directory, {suffix})))
    package["plugin.json"], package["servo-assets.wasm"] = manifest_path.read_bytes(), module_path.read_bytes()
    validate_manifest(package["plugin.json"], package["servo-assets.wasm"], edition, release_version)
    package["BUILD-INFO.json"] = encoded({"format": 1, "kind": "plugin", "edition": edition, "version": release_version, "git": provenance(repo), "source_files": {name: digest(data) for name, data in source.items()}, "files": {name: digest(data) for name, data in package.items()}})
    target = out / ("servo-assets" if edition == "modern" else "legacy/servo-assets")
    target.parent.mkdir(parents=True, exist_ok=True)
    if target.exists() and (target.is_symlink() or not target.is_dir() or not (target / "plugin.json").is_file() or read_json(target / "plugin.json").get("id") != "servo-assets"):
        raise ValueError(f"Refusing to replace an unmanaged staging path: {target}")
    with tempfile.TemporaryDirectory(prefix=".servo-assets-stage-", dir=target.parent) as temporary:
        fresh = Path(temporary) / "package"
        fresh.mkdir()
        for name, data in package.items():
            destination = fresh / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(data)
        prefix = "" if edition == "modern" else "servo-assets/"
        archive = out / f"servo-assets-{'legacy-' if edition == 'legacy' else ''}{release_version}.zip"
        write_zip(archive, {prefix + name: data for name, data in package.items()}, epoch(repo))
        backup = Path(temporary) / "previous"
        if target.exists():
            target.rename(backup)
        try:
            fresh.rename(target)
        except OSError:
            if backup.exists():
                backup.rename(target)
            raise
    return {"archive": str(archive), "staging": str(target), "edition": edition, "manifest_bytes": len(package["plugin.json"]), "files": len(package)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=REPO)
    parser.add_argument("--out", type=Path)
    commands = parser.add_subparsers(dest="command", required=True)
    build = commands.add_parser("stage", help="Replace one generated package directory with a clean, verified payload")
    build.add_argument("--edition", choices=("modern", "legacy"), required=True)
    build.add_argument("--manifest", type=Path, required=True)
    build.add_argument("--module", type=Path, required=True)
    for name in ("source", "verify", "checksums", "validate-tag"):
        command = commands.add_parser(name)
        command.add_argument("--require-clean", action="store_true")
        command.add_argument("--commit")
        if name == "validate-tag":
            command.add_argument("tag")
        if name == "verify":
            command.add_argument("--require-checksums", action="store_true")
    args = parser.parse_args()
    repo = args.repo.resolve()
    out = args.out.resolve() if args.out else repo / "dist"
    release_version = version(repo)
    if args.command == "stage":
        result = stage(repo, out, args.edition, args.manifest, args.module)
    elif args.command == "validate-tag":
        if args.tag != "v" + release_version:
            raise ValueError(f"Tag must match Cargo version exactly: v{release_version}")
        state = require_checkout(repo, source_files(repo), args.commit) if args.require_clean else provenance(repo)
        tag_commit = git(repo, "rev-parse", f"refs/tags/{args.tag}^{{commit}}")
        if not state["commit"] or tag_commit != state["commit"]:
            raise ValueError("The existing version tag must point to the checked-out commit")
        result = {"version": release_version, "tag": args.tag, "commit": state["commit"]}
    elif args.command == "source":
        paths = source_files(repo)
        state = require_checkout(repo, paths, args.commit) if args.require_clean else provenance(repo)
        if args.commit and state["commit"] != args.commit:
            raise ValueError("Source checkout does not match the expected commit")
        files = payloads(repo, paths)
        metadata = {"format": 1, "kind": "source", "version": release_version, "git": state, "source_files": {name: digest(data) for name, data in files.items()}}
        files["SOURCE-MANIFEST.json"] = encoded(metadata)
        prefix = f"solidcraft-servo-assets-{release_version}/"
        archive = out / f"servo-assets-source-{release_version}.zip"
        write_zip(archive, {prefix + name: data for name, data in files.items()}, epoch(repo))
        result = {"archive": str(archive), "files": len(files), "git": state}
    else:
        paths, metadata = verify_all(repo, out, args.commit, args.require_clean, consolidated=args.command == "verify", require_checksums=getattr(args, "require_checksums", False))
        if args.command == "checksums":
            atomic_write(out / "SHA256SUMS.txt", "".join(f"{digest(path.read_bytes())}  {path.name}\n" for path in sorted(paths)).encode())
            atomic_write(out / "release-provenance.json", encoded({"version": release_version, "git": metadata[0]["git"], "archives": {path.name: digest(path.read_bytes()) for path in paths}}))
        result = {"version": release_version, "archives": [str(path) for path in paths], "verified": True, "git": metadata[0]["git"]}
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, zipfile.BadZipFile) as error:
        raise SystemExit(str(error)) from error
