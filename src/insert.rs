//! One host transaction creates an envelope and saves its source snapshot in the design.

#[cfg(not(feature = "legacy-host"))]
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

use crate::catalog::{Catalog, valid_id};
use crate::{Error, invalid};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InsertParams {
    pub asset_id: String,
    #[serde(default = "default_representation")]
    pub representation: String,
    #[serde(default)]
    pub mount_group_id: Option<String>,
    #[serde(default)]
    pub clearance_mm: f64,
    #[serde(default)]
    pub x_mm: f64,
    #[serde(default)]
    pub y_mm: f64,
    #[serde(default)]
    pub z_mm: f64,
}

fn default_representation() -> String {
    "case-envelope".into()
}

pub trait Runner {
    fn run(&mut self, command: &str, params: Value) -> Result<Value, Error>;
}

pub fn recipe(catalog: &Catalog, params: &InsertParams) -> Result<Value, Error> {
    catalog.validate()?;
    validate_placement(params)?;
    if params.representation == "output-reference" {
        if params.mount_group_id.is_some() {
            return Err(invalid("mount_group_id", "select a group only for a mounting pattern"));
        }
        return crate::outputs::output_recipe(
            catalog,
            &crate::outputs::OutputParams {
                asset_id: params.asset_id.clone(),
                output_id: "main".into(),
                clearance_mm: params.clearance_mm,
                x_mm: params.x_mm,
                y_mm: params.y_mm,
                z_mm: params.z_mm,
            },
        );
    }
    reference_recipe(catalog, params)
}

pub(crate) fn validate_placement(params: &InsertParams) -> Result<(), Error> {
    if !valid_id(&params.asset_id) || !valid_id(&params.representation) {
        return Err(invalid("asset_id", "use a catalog ID and representation"));
    }
    if params.mount_group_id.as_ref().is_some_and(|id| !valid_id(id)) {
        return Err(invalid("mount_group_id", "use a catalog mounting group ID"));
    }
    if !params.clearance_mm.is_finite() || !(0.0..=10.0).contains(&params.clearance_mm) {
        return Err(invalid(
            "clearance_mm",
            "must be finite and between 0 and 10 mm per side",
        ));
    }
    for (field, value) in [("x_mm", params.x_mm), ("y_mm", params.y_mm), ("z_mm", params.z_mm)] {
        if !value.is_finite() || value.abs() > 100000.0 {
            return Err(invalid(field, "must be finite and within 100000 mm of the origin"));
        }
    }
    Ok(())
}

fn reference_recipe(catalog: &Catalog, params: &InsertParams) -> Result<Value, Error> {
    let asset = catalog.asset(&params.asset_id)?;
    let representation = asset.representations.get(&params.representation).ok_or_else(|| {
        Error::Unsupported(format!(
            "{} has no `{}` representation; mounting holes and horns need further source dimensions",
            asset.model, params.representation
        ))
    })?;
    let margin = params.clearance_mm;
    let label = format!("{} {} ref r{}", asset.model, params.representation, asset.data_revision);
    if representation.kind == "mounting_pattern" {
        let group_id = params
            .mount_group_id
            .as_ref()
            .filter(|id| id.as_str() != "profile-default")
            .or(representation.mount_group.as_ref());
        let group = asset
            .mount_groups
            .iter()
            .find(|group| Some(&group.id) == group_id)
            .ok_or_else(|| invalid("mount_group", "unknown mounting group"))?;
        let plane_height = match &group.plane_height_dimension {
            Some(key) => {
                asset
                    .dimensions
                    .get(key)
                    .ok_or_else(|| invalid("mount_group", "missing plane height dimension"))?
                    .nominal_mm
            }
            None => 0.0,
        };
        let plane = match &group.plane {
            Some(plane) => {
                json!({"origin": [params.x_mm + plane.origin_mm[0], params.y_mm + plane.origin_mm[1], params.z_mm + plane.origin_mm[2]], "x_dir": plane.x_dir, "y_dir": plane.y_dir})
            }
            None => {
                json!({"origin": [params.x_mm, params.y_mm, params.z_mm + plane_height], "x_dir": [1,0,0], "y_dir": [0,1,0]})
            }
        };
        let mut steps = vec![json!({"command": "sketch.create", "params": {
            "plane": plane,
            "frame": "world", "name": format!("{label} / {}", group.id)
        }})];
        for hole in &group.holes {
            steps.push(json!({"command": "sketch.circle.center", "params": {
                "center": hole.center_mm, "diameter": format!("{} mm", hole.diameter.nominal_mm + 2.0 * margin),
                "id": hole.id, "connect": false
            }}));
        }
        steps.push(json!({"command": "sketch.constraint.fix", "params": {
            "entities": group.holes.iter().map(|hole| &hole.id).collect::<Vec<_>>(), "fixed": true
        }}));
        steps.push(json!({"command": "sketch.finish", "params": {}}));
        return Ok(Value::Array(steps));
    }
    if params.mount_group_id.is_some() {
        return Err(invalid("mount_group_id", "select a group only for a mounting pattern"));
    }
    let [length, width, height] = representation
        .size_dimensions
        .as_ref()
        .ok_or_else(|| invalid("representation", "missing envelope dimensions"))?;
    let size = |key: &str| -> Result<String, Error> {
        let dimension = asset.dimensions.get(key).ok_or_else(|| invalid("dimension", key))?;
        Ok(format!("{} mm", dimension.nominal_mm + 2.0 * params.clearance_mm))
    };
    Ok(json!([{"command": "solid.box", "params": {
        "length": size(length)?, "width": size(width)?, "height": size(height)?,
        "corner": [params.x_mm - margin, params.y_mm - margin, params.z_mm - margin],
        "operation": "new", "name": label, "body_name": label
    }}]))
}

pub fn insert(runner: &mut impl Runner, catalog: &Catalog, params: &InsertParams) -> Result<Value, Error> {
    let plan = recipe(catalog, params)?;
    let warning = if catalog
        .asset(&params.asset_id)?
        .representations
        .get(&params.representation)
        .is_some_and(|representation| representation.kind == "mounting_pattern")
    {
        "Nominal screw reference circles in this source profile's mounting frame and plane. Read each hole's type/thread evidence; circles do not reproduce full slot contours or specify drill size or safe engagement. No horn geometry is included."
    } else {
        "Reference envelope only. No mounting holes, output shaft location or cable geometry is included. Use cable-space for explicit cable routing allowances."
    };
    execute_plan(
        runner,
        catalog,
        &params.asset_id,
        &params.representation,
        serde_json::to_value(params).map_err(|e| invalid("params", e))?,
        plan,
        warning,
    )
}

pub(crate) fn execute_plan(
    runner: &mut impl Runner,
    catalog: &Catalog,
    asset_id: &str,
    representation: &str,
    parameters: Value,
    plan: Value,
    warning: &str,
) -> Result<Value, Error> {
    execute_plan_context(
        runner,
        catalog,
        asset_id,
        representation,
        PlanContext {
            parameters,
            accessories: vec![],
        },
        plan,
        warning,
    )
}

pub(crate) struct PlanContext {
    pub parameters: Value,
    pub accessories: Vec<Value>,
}

pub(crate) struct PlanAsset<'a> {
    pub id: &'a str,
    pub data_revision: u32,
    pub release: &'a str,
    pub kind: &'a str,
    pub record: Value,
    pub sources: Value,
}

fn persisted_body_name(
    runner: &mut impl Runner,
    body: &str,
    feature: u64,
    requested: &str,
    executed: &mut Vec<Value>,
) -> Result<String, Error> {
    // The host's unlabelled BodyN aliases can change during save/rebuild.
    // An explicit rename is persisted, and the creating feature ID separates copies.
    if !body
        .strip_prefix("Body")
        .is_some_and(|suffix| !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return Ok(body.into());
    }
    let mut prefix = String::new();
    for character in requested.chars() {
        if prefix.len() + character.len_utf8() > 88 {
            break;
        }
        prefix.push(character);
    }
    let name = format!("{} [servo F{feature}]", prefix.trim());
    let args = json!({"body":body,"name":name});
    let reply = runner.run("body.rename", args.clone())?;
    if reply.get("body").and_then(Value::as_str) != Some(body)
        || reply.get("name").and_then(Value::as_str) != Some(name.as_str())
    {
        return Err(invalid(
            "host.reply",
            "body.rename must preserve the owned body's explicit name",
        ));
    }
    executed.push(json!({"command":"body.rename","params":args}));
    Ok(name)
}

pub(crate) fn execute_plan_context(
    runner: &mut impl Runner,
    catalog: &Catalog,
    asset_id: &str,
    representation: &str,
    context: PlanContext,
    plan: Value,
    warning: &str,
) -> Result<Value, Error> {
    let asset = catalog.asset(asset_id)?;
    execute_asset_plan(
        runner,
        PlanAsset {
            id: &asset.id,
            data_revision: asset.data_revision,
            release: &catalog.release,
            kind: "servo",
            record: json!(asset),
            sources: json!(catalog.sources_for_asset(asset)),
        },
        representation,
        context,
        plan,
        warning,
    )
}

pub(crate) fn execute_asset_plan(
    runner: &mut impl Runner,
    asset: PlanAsset<'_>,
    representation: &str,
    context: PlanContext,
    plan: Value,
    warning: &str,
) -> Result<Value, Error> {
    let PlanContext {
        mut parameters,
        accessories,
    } = context;
    let asset_id = asset.id;
    if asset.kind == "servo"
        && let Some(object) = parameters.as_object_mut()
    {
        object.insert("asset_id".into(), json!(asset_id));
    }
    let steps = plan.as_array().ok_or_else(|| invalid("recipe", "expected commands"))?;
    let mut feature = None;
    let mut features = Vec::new();
    let mut bodies = Vec::new();
    let mut sketch = None;
    let mut roles: BTreeMap<String, String> = BTreeMap::new();
    let mut notes = Vec::new();
    let mut executed = Vec::new();
    for step in steps {
        let command = step
            .get("command")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid("recipe", "missing command"))?;
        let mut args = step
            .get("params")
            .ok_or_else(|| invalid("recipe", "missing parameters"))?
            .clone();
        if let Some(binding) = step.get("use_last_sketch") {
            let binding = binding
                .as_bool()
                .ok_or_else(|| invalid("recipe", "use_last_sketch must be a boolean"))?;
            if binding {
                if command != "solid.extrude" {
                    return Err(invalid(
                        "recipe",
                        "only profile extrusions can bind the last owned sketch",
                    ));
                }
                let id = sketch.ok_or_else(|| invalid("recipe", "profile extrusion has no owned sketch ID"))?;
                args.as_object_mut()
                    .ok_or_else(|| invalid("recipe", "parameters must be an object"))?
                    .insert("sketch".into(), json!(id));
            }
        }
        let bound_targets = step
            .get("target_roles")
            .and_then(Value::as_array)
            .is_some_and(|targets| !targets.is_empty());
        let operation = args.get("operation").and_then(Value::as_str).unwrap_or("new");
        if (matches!(command, "solid.box" | "solid.cylinder" | "solid.extrude") && matches!(operation, "cut" | "join")
            || matches!(command, "appearance.assign" | "solid.move"))
            && !bound_targets
        {
            return Err(invalid(
                "recipe",
                "modifiers and appearances must name owned target roles",
            ));
        }
        if let Some(targets) = step.get("target_roles").and_then(Value::as_array) {
            let targets = targets
                .iter()
                .map(|role| {
                    let role = role
                        .as_str()
                        .ok_or_else(|| invalid("recipe", "target role must be text"))?;
                    roles
                        .get(role)
                        .cloned()
                        .ok_or_else(|| invalid("recipe", format!("unbound target role `{role}`")))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let object = args
                .as_object_mut()
                .ok_or_else(|| invalid("recipe", "parameters must be an object"))?;
            object.insert(
                if matches!(command, "appearance.assign" | "solid.move") {
                    "bodies"
                } else {
                    "targets"
                }
                .into(),
                json!(targets),
            );
        }
        let reply = runner.run(command, args.clone())?;
        executed.push(json!({"command":command,"params":args}));
        if matches!(command, "solid.box" | "solid.cylinder" | "solid.extrude" | "solid.move") {
            feature = reply.get("feature").and_then(Value::as_u64);
            let mut created = reply
                .get("created_bodies")
                .and_then(Value::as_array)
                .ok_or_else(|| invalid("host.reply", "primitive did not return created_bodies"))?
                .clone();
            let operation = args.get("operation").and_then(Value::as_str).unwrap_or("new");
            if matches!(operation, "cut" | "join") || command == "solid.move" {
                if step
                    .get("target_roles")
                    .and_then(Value::as_array)
                    .is_none_or(Vec::is_empty)
                    || !created.is_empty()
                    || reply
                        .get("changed_bodies")
                        .and_then(Value::as_array)
                        .is_none_or(Vec::is_empty)
                    || reply
                        .get("changed_bodies")
                        .and_then(Value::as_array)
                        .is_some_and(|changed| {
                            changed.iter().any(|body| {
                                args.get(if command == "solid.move" { "bodies" } else { "targets" })
                                    .and_then(Value::as_array)
                                    .is_none_or(|targets| !targets.contains(body))
                            })
                        })
                    || reply
                        .get("removed_bodies")
                        .and_then(Value::as_array)
                        .is_some_and(|b| !b.is_empty())
                {
                    return Err(invalid("host.reply", "a model edit must update only its bound bodies"));
                }
            } else {
                if created.len() != 1 || !created.iter().all(Value::is_string) {
                    return Err(invalid("host.reply", "expected one newly created envelope body"));
                }
                let body = created
                    .first()
                    .and_then(Value::as_str)
                    .ok_or_else(|| invalid("host.reply", "new body has no name"))?;
                let stable = persisted_body_name(
                    runner,
                    body,
                    feature.ok_or_else(|| invalid("host.reply", "new body has no feature ID"))?,
                    args.get("body_name").and_then(Value::as_str).unwrap_or(representation),
                    &mut executed,
                )?;
                if let Some(body) = created.first_mut() {
                    *body = json!(stable);
                }
                if let Some(role) = step.get("bind_role").and_then(Value::as_str) {
                    let body = created
                        .first()
                        .and_then(Value::as_str)
                        .ok_or_else(|| invalid("host.reply", "new body has no name"))?;
                    if roles.insert(role.into(), body.into()).is_some() {
                        return Err(invalid("recipe", "duplicate part role"));
                    }
                }
                bodies.extend(created);
            }
            if let Some(id) = feature {
                features.push(id);
            }
        } else if command == "sketch.create" {
            sketch = reply.get("sketch").and_then(Value::as_u64);
            feature = sketch;
            if let Some(id) = feature {
                features.push(id);
            }
        } else if command.starts_with("construct.") {
            feature = reply.get("feature").and_then(Value::as_u64);
            if let Some(id) = feature {
                features.push(id);
            }
        } else if command == "note.add"
            && let Some(note) = reply.get("note").and_then(Value::as_u64)
        {
            notes.push(note);
        }
    }
    let feature = feature.ok_or_else(|| invalid("host.reply", "the host did not return a feature ID"))?;
    #[cfg(not(feature = "legacy-host"))]
    let snapshot_path = format!("instances/feature-{feature}.json");
    let snapshot = json!({
        "snapshot_format": 1, "plugin_version": env!("CARGO_PKG_VERSION"),
        "catalog_release": asset.release, "asset": asset.record, "asset_kind": asset.kind,
        "sources": asset.sources, "parameters": parameters, "recipe": executed,
        "feature": feature, "features": features, "sketch": sketch, "bodies": bodies,
        "representation": representation, "warning": warning, "body_roles":roles, "notes":notes, "accessories":accessories,
        "geometry_basis": match representation { "cable-space" => "user_specified_planning_envelopes", "servo-reference" => "mixed_dimensioned_assumed_and_approximate", "horn-reference" if parameters["mode"] == "dimensions" => "source_dimension_references_only", "horn-reference" if parameters["mode"] == "photo-solid" => "photo_reconstructed_estimated_geometry", "horn-reference" => "source_partial_outer_profile", _ => "catalog_nominal_reference" }
    });
    #[cfg(not(feature = "legacy-host"))]
    runner.run(
        "storage.put_blob",
        json!({"name": snapshot_path, "data": STANDARD.encode(snapshot.to_string()), "cache": false}),
    )?;
    #[cfg(not(feature = "legacy-host"))]
    let instance_id = format!("feature-{feature}");
    #[cfg(not(feature = "legacy-host"))]
    runner.run(
        "storage.patch",
        json!({"patch": {"instances": {instance_id: {
            "asset_id": asset_id, "data_revision": asset.data_revision,
            "snapshot": snapshot_path, "bodies": bodies, "sketch": sketch
        }}}}),
    )?;
    #[cfg(feature = "legacy-host")]
    let saved_snapshot: Option<String> = None;
    #[cfg(not(feature = "legacy-host"))]
    let saved_snapshot = Some(snapshot_path);
    #[cfg(feature = "legacy-host")]
    let reply_snapshot = Some(snapshot);
    #[cfg(not(feature = "legacy-host"))]
    let reply_snapshot: Option<Value> = None;
    Ok(json!({"asset_id": asset_id, "data_revision": asset.data_revision,
        "feature": feature, "features": features, "bodies": bodies, "representation": representation, "snapshot": saved_snapshot,
        "metadata_saved_in_design": !cfg!(feature = "legacy-host"), "source_snapshot": reply_snapshot,
        "sketch": sketch, "warning": warning, "body_roles": roles, "notes":notes, "accessories":accessories}))
}
