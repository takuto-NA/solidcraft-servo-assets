use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};
use solidcraft_servo_assets::catalog::Catalog;
use solidcraft_servo_assets::horns::HornCatalog;
use solidcraft_servo_assets::{Error, Runner};

use crate::TaskResult;

fn json_files(directory: &Path) -> TaskResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    for (index, entry) in fs::read_dir(directory)?.enumerate() {
        if index >= 4096 {
            return Err(format!("{} exceeds 4096 directory entries", directory.display()).into());
        }
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "json") {
            let metadata = fs::symlink_metadata(&path)?;
            if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 512 * 1024 {
                return Err(format!("{} must be a regular JSON file of at most 512 KiB", path.display()).into());
            }
            files.push(path);
        }
    }
    if files.is_empty() || files.len() > 4096 {
        return Err(format!("{} needs 1..4096 JSON files", directory.display()).into());
    }
    files.sort();
    Ok(files)
}

fn records(value: &Value, field: &str) -> TaskResult<BTreeMap<String, Value>> {
    let mut records = BTreeMap::new();
    for record in value
        .get(field)
        .and_then(Value::as_array)
        .ok_or("missing record list")?
    {
        let id = if field == "mounts" {
            format!(
                "{}/{}",
                record
                    .get("asset_id")
                    .and_then(Value::as_str)
                    .ok_or("mount asset ID missing")?,
                record
                    .get("horn_id")
                    .and_then(Value::as_str)
                    .ok_or("mount horn ID missing")?
            )
        } else {
            record
                .get("id")
                .and_then(Value::as_str)
                .ok_or("record ID missing")?
                .into()
        };
        if records.insert(id, record.clone()).is_some() {
            return Err(format!("duplicate {field} identity").into());
        }
    }
    Ok(records)
}

fn catalogs(repo: &Path) -> TaskResult<(Catalog, HornCatalog)> {
    let embedded = Catalog::embedded()?;
    let embedded_horns = HornCatalog::embedded()?;
    let mut servo_packs = Vec::new();
    let mut horn_packs = Vec::new();
    for path in json_files(&repo.join("catalog"))? {
        let text = fs::read_to_string(&path)?;
        let value: Value = serde_json::from_str(&text)?;
        match (value.get("assets").is_some(), value.get("horns").is_some()) {
            (true, false) => servo_packs.push(text),
            (false, true) => horn_packs.push(text),
            _ => return Err(format!("{} is not one unambiguous catalog pack", path.display()).into()),
        }
    }
    let catalog = Catalog::merge_packs(
        &embedded.release,
        &servo_packs.iter().map(String::as_str).collect::<Vec<_>>(),
    )?;
    let horns = HornCatalog::merge_packs(
        &embedded_horns.release,
        &horn_packs.iter().map(String::as_str).collect::<Vec<_>>(),
    )?;
    catalog.validate()?;
    horns.validate_for_catalog(&catalog)?;
    for (disk, compiled, fields) in [
        (
            serde_json::to_value(&catalog)?,
            serde_json::to_value(&embedded)?,
            vec!["assets", "sources"],
        ),
        (
            serde_json::to_value(&horns)?,
            serde_json::to_value(&embedded_horns)?,
            vec!["horns", "sources", "mounts"],
        ),
    ] {
        for field in fields {
            if records(&disk, field)? != records(&compiled, field)? {
                return Err(format!(
                    "disk {field} differ from embedded packs; update the loader and source records together"
                )
                .into());
            }
        }
    }
    println!(
        "Validated {} servo profiles and {} horn variants from all JSON packs.",
        catalog.assets.len(),
        horns.horns.len()
    );
    Ok((catalog, horns))
}

#[derive(Default)]
struct Recorder {
    calls: u64,
    bodies: BTreeSet<String>,
}

fn invalid(reason: impl ToString) -> Error {
    Error::Invalid {
        field: "fixture".into(),
        reason: reason.to_string(),
    }
}

impl Runner for Recorder {
    fn run(&mut self, command: &str, params: Value) -> Result<Value, Error> {
        if self.calls >= 100_000 {
            return Err(invalid("fixture recipe exceeds 100000 commands"));
        }
        self.calls += 1;
        let feature = self.calls;
        match command {
            "component.list" => Ok(json!({"active":0,"active_path":[],"components":[]})),
            "component.create" => Ok(json!({"component":1,"occurrence":2,"name":params["name"]})),
            "component.activate" | "occurrence.move" => Ok(json!({})),
            "document.inspect" => Ok(json!({"timeline":[]})),
            "solid.box" | "solid.cylinder" | "solid.extrude" | "solid.move" => {
                let operation = params.get("operation").and_then(Value::as_str).unwrap_or("new");
                if command == "solid.move" || matches!(operation, "cut" | "join") {
                    let key = if command == "solid.move" { "bodies" } else { "targets" };
                    let targets = params
                        .get(key)
                        .and_then(Value::as_array)
                        .ok_or_else(|| invalid("missing owned targets"))?;
                    if targets.is_empty()
                        || targets
                            .iter()
                            .any(|value| value.as_str().is_none_or(|name| !self.bodies.contains(name)))
                    {
                        return Err(invalid("recipe edits a body it did not create"));
                    }
                    Ok(json!({"feature":feature,"created_bodies":[],"changed_bodies":targets,"removed_bodies":[]}))
                } else if operation == "new" {
                    let name = format!("Body{feature}");
                    self.bodies.insert(name.clone());
                    Ok(json!({"feature":feature,"created_bodies":[name],"changed_bodies":[],"removed_bodies":[]}))
                } else {
                    Err(invalid("unsupported primitive operation"))
                }
            }
            "body.rename" => {
                let body = params
                    .get("body")
                    .and_then(Value::as_str)
                    .ok_or_else(|| invalid("body name missing"))?;
                let name = params
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| invalid("new body name missing"))?;
                if !self.bodies.remove(body) || !self.bodies.insert(name.into()) {
                    return Err(invalid("invalid or duplicate owned body rename"));
                }
                Ok(json!({"body":body,"name":name}))
            }
            "sketch.create" => Ok(json!({"sketch":feature})),
            "construct.axis.two_points" => Ok(json!({"feature":feature})),
            "note.add" => Ok(json!({"note":feature})),
            "appearance.assign" => {
                if params.get("bodies").and_then(Value::as_array).is_none_or(|bodies| {
                    bodies.is_empty()
                        || bodies
                            .iter()
                            .any(|value| value.as_str().is_none_or(|name| !self.bodies.contains(name)))
                }) {
                    return Err(invalid("appearance targets are not owned"));
                }
                Ok(json!({}))
            }
            "sketch.line"
            | "sketch.arc.center_point"
            | "sketch.circle.center"
            | "sketch.constraint.fix"
            | "sketch.text"
            | "sketch.finish"
            | "storage.patch" => Ok(json!({})),
            _ => Err(invalid(format!("unrecognized recipe command {command}"))),
        }
    }

    fn save_snapshot(&mut self, _name: &str, _data: &[u8]) -> Result<(), Error> {
        Ok(())
    }
}

fn execute(
    recorder: &mut Recorder,
    catalog: &Catalog,
    horns: &HornCatalog,
    entry: &str,
    params: &Value,
) -> Result<Value, Error> {
    use solidcraft_servo_assets::workflow::{
        DatumKind, picker_params, place_datum_kind, place_horn_model, place_servo_model,
    };

    macro_rules! parse {
        ($kind:ty) => {
            serde_json::from_value::<$kind>(params.clone()).map_err(invalid)?
        };
    }
    match entry {
        "insert" => {
            let params = parse!(solidcraft_servo_assets::InsertParams);
            if matches!(params.representation.as_str(), "mounting-pattern" | "output-reference") {
                return Err(invalid("use the dedicated datum entry"));
            }
            solidcraft_servo_assets::insert(recorder, catalog, &params)
        }
        "insert-servo" => place_servo_model(
            recorder,
            catalog,
            parse!(solidcraft_servo_assets::workflow::ServoPlacement),
        ),
        "place-servo" => solidcraft_servo_assets::workflow::place_servo(
            recorder,
            catalog,
            parse!(solidcraft_servo_assets::workflow::ServoPlacement),
        ),
        "insert-horn-model" => place_horn_model(recorder, horns, &picker_params(params.clone())?),
        "insert-horn" => solidcraft_servo_assets::horns::insert_horn(recorder, horns, &picker_params(params.clone())?),
        "insert-mounts" | "insert-axis" => place_datum_kind(
            recorder,
            catalog,
            &picker_params(params.clone())?,
            if entry == "insert-mounts" {
                DatumKind::Mounting
            } else {
                DatumKind::ShaftAxis
            },
        ),
        "servo-reference" => solidcraft_servo_assets::models::insert_model(
            recorder,
            catalog,
            &parse!(solidcraft_servo_assets::models::ModelParams),
        ),
        "horn-reference" => solidcraft_servo_assets::horns::insert_horn(
            recorder,
            horns,
            &parse!(solidcraft_servo_assets::horns::HornParams),
        ),
        "place-horn" => solidcraft_servo_assets::workflow::place_horn(
            recorder,
            horns,
            &parse!(solidcraft_servo_assets::workflow::Placement),
        ),
        "place-datum" => solidcraft_servo_assets::workflow::place_datum(
            recorder,
            catalog,
            &parse!(solidcraft_servo_assets::workflow::Placement),
        ),
        "output-reference" => solidcraft_servo_assets::outputs::insert_output_reference(
            recorder,
            catalog,
            &parse!(solidcraft_servo_assets::outputs::OutputParams),
        ),
        "cable-space" => solidcraft_servo_assets::cables::insert_cable_space(
            recorder,
            catalog,
            &parse!(solidcraft_servo_assets::cables::CableSpaceParams),
        ),
        "mount-pattern" => {
            if params.as_object().is_none_or(|fields| {
                fields.keys().any(|key| {
                    !matches!(
                        key.as_str(),
                        "asset_id" | "mount_group_id" | "clearance_mm" | "x_mm" | "y_mm" | "z_mm"
                    )
                })
            }) {
                return Err(invalid("unknown mounting-pattern parameter"));
            }
            let mut params = params.clone();
            params
                .as_object_mut()
                .ok_or_else(|| invalid("parameters must be an object"))?
                .insert("representation".into(), json!("mounting-pattern"));
            solidcraft_servo_assets::insert(recorder, catalog, &serde_json::from_value(params).map_err(invalid)?)
        }
        "catalog" => {
            if params
                .as_object()
                .is_none_or(|fields| fields.keys().any(|key| key != "query"))
                || params.get("query").is_some_and(|value| !value.is_string())
            {
                return Err(invalid("invalid catalog query"));
            }
            catalog.search(params.get("query").and_then(Value::as_str).unwrap_or(""))
        }
        "describe" => {
            if params
                .as_object()
                .is_none_or(|fields| fields.keys().any(|key| key != "asset_id"))
            {
                return Err(invalid("unknown describe parameter"));
            }
            catalog.describe(
                params
                    .get("asset_id")
                    .and_then(Value::as_str)
                    .ok_or_else(|| invalid("asset ID missing"))?,
            )
        }
        _ => Err(invalid("unknown fixture entry")),
    }
}

fn subset(actual: &Value, expected: &Value) -> bool {
    match expected {
        Value::Object(fields) => fields
            .iter()
            .all(|(key, value)| actual.get(key).is_some_and(|actual| subset(actual, value))),
        Value::Array(values) => actual
            .as_array()
            .is_some_and(|actual| actual.len() >= values.len() && actual.iter().zip(values).all(|(a, e)| subset(a, e))),
        Value::Number(number) => actual
            .as_f64()
            .zip(number.as_f64())
            .is_some_and(|(a, b)| (a - b).abs() <= 1e-9 * b.abs().max(1.0)),
        _ => actual == expected,
    }
}

fn fixtures(repo: &Path, catalog: &Catalog, horns: &HornCatalog) -> TaskResult {
    let files = json_files(&repo.join("tests"))?;
    let mut covered_servos = BTreeSet::new();
    let mut covered_horns = BTreeSet::new();
    let mut covered_assemblies = BTreeSet::new();
    let mut geometry_assertions = 0;
    for path in &files {
        let case: Value = serde_json::from_slice(&fs::read(path)?)?;
        let object = case.as_object().ok_or("fixture must be an object")?;
        if object
            .keys()
            .any(|key| !matches!(key.as_str(), "entry" | "params" | "expect"))
        {
            return Err(format!("{} has unsupported fixture fields", path.display()).into());
        }
        let entry = case
            .get("entry")
            .and_then(Value::as_str)
            .ok_or("fixture entry missing")?;
        let params = case
            .get("params")
            .filter(|value| value.is_object())
            .ok_or("fixture params missing")?;
        let expect = case
            .get("expect")
            .filter(|value| value.is_object())
            .ok_or("fixture expectation missing")?;
        let body_count = expect
            .pointer("/model/body_count")
            .and_then(Value::as_u64)
            .filter(|count| *count <= 4096)
            .ok_or("fixture needs bounded expected body_count")?;
        let mut recorder = Recorder::default();
        let reply = execute(&mut recorder, catalog, horns, entry, params);
        if let Some(error) = expect.get("error").and_then(Value::as_str) {
            let failure = reply
                .err()
                .ok_or_else(|| format!("{} expected a refusal but the recipe succeeded", path.display()))?;
            if error != "any" && !(error == "invalid_input" && matches!(failure, Error::Invalid { .. })) {
                return Err(format!("{} returned an unexpected error class: {failure}", path.display()).into());
            }
            if recorder.calls != 0 || body_count != 0 {
                return Err(format!("{} must refuse before host mutation", path.display()).into());
            }
        } else {
            let reply = reply.map_err(|error| format!("{} unexpectedly refused: {error}", path.display()))?;
            if recorder.bodies.len() as u64 != body_count {
                return Err(format!(
                    "{} expected {body_count} bodies, recipe creates {}",
                    path.display(),
                    recorder.bodies.len()
                )
                .into());
            }
            if expect
                .get("reply")
                .is_some_and(|expected| !subset(&json!({"result":&reply}), expected))
            {
                return Err(format!(
                    "{} reply no longer matches its expected source metadata",
                    path.display()
                )
                .into());
            }
            let id = reply.get("asset_id").and_then(Value::as_str).unwrap_or("");
            if entry == "servo-reference" {
                covered_servos.insert(id.to_string());
                if let Some(horn) = params.get("horn_id").and_then(Value::as_str).filter(|id| *id != "none") {
                    covered_assemblies.insert((id.to_string(), horns.horn(horn)?.id.clone()));
                }
            } else if entry == "horn-reference" {
                covered_horns.insert((
                    id.to_string(),
                    params
                        .get("mode")
                        .and_then(Value::as_str)
                        .unwrap_or("dimensions")
                        .to_string(),
                ));
            }
        }
        geometry_assertions += usize::from(expect.pointer("/model/total").is_some());
    }
    for asset in &catalog.assets {
        if asset.reference_model.is_some() && !covered_servos.contains(&asset.id) {
            return Err(format!("missing successful servo-reference fixture for {}", asset.id).into());
        }
    }
    for horn in &horns.horns {
        let mut modes = vec!["dimensions"];
        if !horn.parts.is_empty() {
            modes.push(if horn.status == "photo_estimate" {
                "photo-solid"
            } else {
                "source-solid"
            });
        }
        for mode in modes {
            if !covered_horns.contains(&(horn.id.clone(), mode.to_string())) {
                return Err(format!("missing successful {mode} fixture for {}", horn.id).into());
            }
        }
    }
    for mount in &horns.mounts {
        if solidcraft_servo_assets::horns::automatic_assembly_available(horns.horn(&mount.horn_id)?, mount)
            && !covered_assemblies.contains(&(mount.asset_id.clone(), mount.horn_id.clone()))
        {
            return Err(format!(
                "missing documented assembly fixture for {} / {}",
                mount.asset_id, mount.horn_id
            )
            .into());
        }
    }
    println!(
        "Checked {} fixtures: recipe bodies, owned targets, source replies, refusals and catalog coverage.",
        files.len()
    );
    println!(
        "NOT RUN in data check: exact kernel geometry ({geometry_assertions} fixture totals); SOLIDCRAFT_CLI host tests perform these assertions."
    );
    Ok(())
}

fn walk_source(repo: &Path, directory: &Path, paths: &mut Vec<PathBuf>, depth: usize) -> TaskResult {
    if depth > 16 || paths.len() > 4096 {
        return Err("source tree exceeds bounded scan size".into());
    }
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        if [".git", "target", "dist", "evidence", "__pycache__"]
            .iter()
            .any(|skip| name == *skip)
        {
            continue;
        }
        if entry.file_type()?.is_dir() {
            walk_source(repo, &path, paths, depth + 1)?;
        } else {
            if paths.len() >= 4096 {
                return Err("source tree exceeds 4096 files".into());
            }
            paths.push(path.strip_prefix(repo)?.to_path_buf());
        }
    }
    Ok(())
}

fn secret(text: &str) -> bool {
    if ["", "RSA ", "EC ", "DSA ", "OPENSSH ", "ENCRYPTED "]
        .iter()
        .any(|kind| text.contains(&format!("-----BEGIN {kind}PRIVATE KEY-----")))
    {
        return true;
    }
    text.split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .any(|word| {
            (["ghp_", "gho_", "ghu_", "ghs_", "ghr_"]
                .iter()
                .any(|prefix| word.starts_with(prefix))
                && word.len() >= 36)
                || (word.starts_with("github_pat_") && word.len() >= 40)
                || (word.len() == 20
                    && (word.starts_with("AKIA") || word.starts_with("ASIA"))
                    && word
                        .bytes()
                        .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit()))
        })
}

fn source_files(repo: &Path) -> TaskResult {
    let git_root = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(repo)
        .output()?;
    let own_git = git_root.status.success()
        && Path::new(String::from_utf8_lossy(&git_root.stdout).trim()).canonicalize()? == repo.canonicalize()?;
    let mut paths = if own_git {
        let output = Command::new("git")
            .args(["ls-files", "--cached", "--others", "--exclude-standard", "-z"])
            .current_dir(repo)
            .output()?;
        if !output.status.success() || output.stdout.len() > 2 * 1024 * 1024 {
            return Err("cannot enumerate bounded repository source paths".into());
        }
        String::from_utf8(output.stdout)?
            .split('\0')
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .collect()
    } else {
        let mut paths = Vec::new();
        walk_source(repo, repo, &mut paths, 0)?;
        paths
    };
    paths.sort();
    paths.dedup();
    if paths.len() > 4096 {
        return Err("source scan is limited to 4096 files".into());
    }
    let attribution = fs::read_to_string(repo.join("ATTRIBUTION.md"))?;
    for relative in &paths {
        let path_text = relative.to_string_lossy().replace('\\', "/");
        if relative.components().any(|part| {
            [".git", ".ssh", ".aws", "target", "dist", "evidence", "__pycache__"]
                .iter()
                .any(|bad| part.as_os_str() == *bad)
        }) || relative.file_name().is_some_and(|name| {
            let name = name.to_string_lossy();
            (name.starts_with(".env") && ![".env.example", ".env.sample"].contains(&name.as_ref()))
                || [
                    ".npmrc",
                    ".pypirc",
                    ".netrc",
                    ".git-credentials",
                    "id_rsa",
                    "id_ed25519",
                    "id_ecdsa",
                    "id_dsa",
                    "credentials",
                ]
                .iter()
                .any(|bad| name.as_ref() == *bad)
        }) {
            return Err(format!("generated or credential file must not be tracked: {path_text}").into());
        }
        let extension = relative
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ["pdf", "step", "stp", "iges", "igs", "dwg", "dxf"].contains(&extension.as_str()) {
            return Err(format!("raw PDF/CAD research assets must stay outside the repository: {path_text}").into());
        }
        if ["png", "jpg", "jpeg", "webp", "gif", "svg"].contains(&extension.as_str())
            && !attribution.contains(path_text.as_str())
        {
            return Err(
                format!("image needs an explicit original/openly-licensed ATTRIBUTION.md entry: {path_text}").into(),
            );
        }
        let path = repo.join(relative);
        let metadata = fs::symlink_metadata(&path)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 2 * 1024 * 1024 {
            return Err(format!("source must be a regular file of at most 2 MiB: {path_text}").into());
        }
        if String::from_utf8(fs::read(path)?).is_ok_and(|text| secret(&text)) {
            return Err(
                format!("possible credential in {path_text}; remove it before publishing (value redacted)").into(),
            );
        }
    }
    println!(
        "Scanned {} {} files for credentials, generated files and unattributed/raw media.",
        paths.len(),
        if own_git {
            "tracked/unignored"
        } else {
            "source (Git not initialized here)"
        }
    );
    Ok(())
}

pub fn all(repo: &Path) -> TaskResult {
    let (catalog, horns) = catalogs(repo)?;
    fixtures(repo, &catalog, &horns)?;
    source_files(repo)?;
    Ok(())
}
