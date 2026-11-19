//! Bounded host forms; catalog growth falls back to source-ID fields.

use crate::catalog::{Catalog, selector_key};
use crate::{Error, invalid};
use serde_json::{Value, json};

pub fn manifest(catalog: &Catalog) -> Result<Value, crate::Error> {
    let horns = crate::horns::HornCatalog::embedded()?;
    let pattern_options: Vec<_> = catalog
        .assets
        .iter()
        .filter(|asset| !asset.mount_groups.is_empty())
        .map(|asset| {
            json!({
                "const": selector_key(&asset.id), "title": asset.model
            })
        })
        .collect();
    let asset_field =
        json!({"type": "string", "title": "Source profile ID", "default": "towerpro.sg90-analog.configure"});
    let insert_properties = json!({
        "asset_id": asset_field,
        "representation": {"type": "string", "title": "Reference envelope", "default": "case-envelope", "oneOf": [
            {"const": "case-envelope", "title": "Housing envelope (ears and shaft excluded)"},
            {"const": "overall-envelope", "title": "Outer envelope (cable and horn excluded)"}
        ]},
        "clearance_mm": {"type": "number", "title": "Extra clearance per side", "description": "An explicit design allowance, separate from the source's tolerances.", "default": 0, "minimum": 0, "maximum": 10, "unit": "mm"},
        "x_mm": {"type": "number", "title": "Origin X", "default": 0, "minimum": -100000, "maximum": 100000, "unit": "mm"},
        "y_mm": {"type": "number", "title": "Origin Y", "default": 0, "minimum": -100000, "maximum": 100000, "unit": "mm"},
        "z_mm": {"type": "number", "title": "Origin Z", "default": 0, "minimum": -100000, "maximum": 100000, "unit": "mm"}
    });
    let mut pattern_properties = insert_properties.clone();
    if let Some(properties) = pattern_properties.as_object_mut() {
        properties.remove("representation");
        properties.insert("asset_id".into(), json!({"type": "string", "title": "Source profile", "oneOf": pattern_options, "default": selector_key("feetech.fs90.akizuki-drawing")}));
    }
    let output_options: Vec<_> = catalog
        .assets
        .iter()
        .filter(|asset| asset.output_interfaces.iter().any(|o| o.axis.is_some()))
        .map(|asset| json!({"const": selector_key(&asset.id), "title": asset.model}))
        .collect();
    let output_ids: std::collections::BTreeSet<_> = catalog
        .assets
        .iter()
        .flat_map(|asset| {
            asset
                .output_interfaces
                .iter()
                .filter(|output| output.axis.is_some())
                .map(|output| output.id.as_str())
        })
        .collect();
    let output_id_options: Vec<_> = output_ids.into_iter().collect();
    let mut output_properties = pattern_properties.clone();
    if let Some(properties) = output_properties.as_object_mut() {
        properties.insert("asset_id".into(), json!({"type":"string", "title":"Source profile", "oneOf":output_options, "default":selector_key("feetech.fs90.akizuki-drawing")}));
        properties.insert(
            "output_id".into(),
            json!({"type":"string", "title":"Output", "enum":output_id_options, "default":"main"}),
        );
        properties.insert("clearance_mm".into(), json!({"type":"number", "title":"Case / screw clearance per side", "description":"The output axis and shaft diameter stay at their source values.", "default":0, "minimum":0, "maximum":10, "unit":"mm"}));
    }
    let group_ids: std::collections::BTreeSet<_> = catalog
        .assets
        .iter()
        .flat_map(|asset| asset.mount_groups.iter().map(|group| group.id.as_str()))
        .collect();
    let mut group_options = vec![json!({"const":"profile-default", "title":"Profile default"})];
    group_options.extend(group_ids.into_iter().map(|id| json!({"const":id, "title":id})));
    if let Some(properties) = pattern_properties.as_object_mut() {
        properties.insert("mount_group_id".into(), json!({"type":"string", "title":"Mounting face / group", "description":"Select a group in this profile. Circles use nominal major diameter; thread and depth callouts are stored with each hole.", "oneOf":group_options, "default":"profile-default"}));
    }
    let length = |title: &str, minimum: f64, maximum: f64| {
        json!({
            "type": "number", "title": title, "minimum": minimum, "maximum": maximum, "unit": "mm"
        })
    };
    let port_ids: std::collections::BTreeSet<_> = catalog
        .assets
        .iter()
        .flat_map(|asset| asset.cable_ports.iter().map(|port| port.id.as_str()))
        .collect();
    let port_options: Vec<_> = port_ids.into_iter().collect();
    let cable_properties = json!({
        "asset_id": {"type":"string", "title":"Source profile", "default":selector_key("towerpro.sg90-analog.configure"), "oneOf": catalog.assets.iter().filter(|a| !a.cable_ports.is_empty()).map(|a| json!({"const":selector_key(&a.id),"title":a.model})).collect::<Vec<_>>()},
        "port_id": {"type": "string", "title": "Cable port", "default": "servo-lead", "enum": port_options},
        "exit_x_mm": length("Planned outlet X", -100000.0, 100000.0),
        "exit_y_mm": length("Planned outlet Y", -100000.0, 100000.0),
        "exit_z_mm": length("Planned outlet Z", -100000.0, 100000.0),
        "direction": {"type": "string", "title": "Cable leaves toward", "default": "negative_x", "oneOf": [
            {"const": "positive_x", "title": "+X"}, {"const": "negative_x", "title": "-X"},
            {"const": "positive_y", "title": "+Y"}, {"const": "negative_y", "title": "-Y"},
            {"const": "positive_z", "title": "+Z"}, {"const": "negative_z", "title": "-Z"}
        ]},
        "bundle_width_mm": length("Planned harness width", 0.01, 1000.0),
        "bundle_height_mm": length("Planned harness thickness", 0.01, 1000.0),
        "straight_length_mm": length("Straight routing space", 0.01, 1000.0),
        "flex_space_mm": {"type": "number", "title": "Extra flex space at outlet", "description": "Extra room around the outlet; not a factory minimum bend radius.", "minimum": 0, "maximum": 100, "default": 0, "unit": "mm"},
        "connector_width_mm": {"type": "number", "title": "Connector space width", "description": "Enter all three connector sizes, or zero for all to omit it.", "minimum": 0, "maximum": 100, "default": 0, "unit": "mm"},
        "connector_height_mm": {"type": "number", "title": "Connector space thickness", "minimum": 0, "maximum": 100, "default": 0, "unit": "mm"},
        "connector_length_mm": {"type": "number", "title": "Connector space length", "minimum": 0, "maximum": 100, "default": 0, "unit": "mm"}
    });
    let model_options: Vec<_> = catalog
        .assets
        .iter()
        .filter(|asset| asset.reference_model.is_some())
        .map(|asset| {
            let basis = match crate::catalog::model_datum_status(asset) {
                "mount_and_axis_datums" => "datums",
                "mount_datums" => "mount datums",
                "axis_datum" => "axis datum",
                _ => "PREVIEW",
            };
            json!({"const":selector_key(&asset.id),"title":format!("{} {} ({basis})",asset.manufacturer,asset.model)})
        })
        .collect();
    let model_properties = json!({
        "asset_id":{"type":"string","title":"Servo reference","default":selector_key("feetech.fs90.akizuki-drawing"),"oneOf":model_options},
        "x_mm":length("Origin X",-100000.0,100000.0),"y_mm":length("Origin Y",-100000.0,100000.0),"z_mm":length("Origin Z",-100000.0,100000.0),
        "include_cable":{"type":"boolean","title":"Show fixed-lead marker","default":true},
        "include_connector":{"type":"boolean","title":"Show loose cable connector","description":"Optional planning envelope. The default model shows only a short cable outlet marker.","default":false},
        "outlet_height_mm":{"type":"number","title":"Approx outlet height above base","default":2,"minimum":0.1,"maximum":20,"unit":"mm"},
        "wire_diameter_mm":{"type":"number","title":"Approx insulated wire diameter","default":1,"minimum":0.1,"maximum":5,"unit":"mm"},
        "wire_pitch_mm":{"type":"number","title":"Approx wire center spacing","default":1.1,"minimum":0.1,"maximum":10,"unit":"mm"},
        "wire_length_mm":{"type":"number","title":"Cable outlet marker length (planning)","default":3,"minimum":2,"maximum":250,"unit":"mm"},
        "labels":{"type":"string","title":"Labels","default":"notes","oneOf":[{"const":"notes","title":"Viewport callouts"},{"const":"front","title":"Front-view labels (for snapshots)"},{"const":"none","title":"No labels"}]}
    });
    let mut model_properties = model_properties;
    let mut horn_options = vec![json!({"const":"none","title":"Base profile / no extra horn"})];
    horn_options.extend(
        horns
            .horns
            .iter()
            .filter(|horn| {
                horns
                    .mounts
                    .iter()
                    .any(|mount| mount.horn_id == horn.id && crate::horns::automatic_assembly_available(horn, mount))
            })
            .map(|horn| json!({"const":selector_key(&horn.id),"title":horn.model})),
    );
    if let Some(properties) = model_properties.as_object_mut() {
        properties.insert("horn_id".into(),json!({"type":"string","title":"Documented horn assembly","description":"Requires a documented fit and installed position. Other accessories use Horn reference.","oneOf":horn_options,"default":"none"}));
        properties.insert(
            "horn_angle_deg".into(),
            json!({"type":"number","title":"Horn visual angle","minimum":-360,"maximum":360,"default":0}),
        );
        properties.insert("horn_axial_offset_mm".into(),json!({"type":"number","title":"Horn axial adjustment","description":"Positive moves outward.","minimum":-100,"maximum":100,"default":0,"unit":"mm"}));
    }
    let mut manifest = json!({
        "id": "servo-assets", "name": "Servo Assets", "version": env!("CARGO_PKG_VERSION"),
        "description": "Servo reference assets with visible drive, fastening and cable interfaces. Source dimensions and planning geometry stay distinct.",
        "capabilities": ["commands@0.2", "storage@0.1"],
        "runtime": {"module": "servo-assets.wasm", "data_format": 1,
            "commands": ["appearance.assign", "body.rename", "construct.axis.two_points", "note.add", "sketch.arc.center_point", "sketch.circle.center", "sketch.constraint.fix", "sketch.create", "sketch.finish", "sketch.line", "sketch.text", "solid.box", "solid.cylinder", "solid.extrude", "solid.move", "storage.patch", "storage.put_blob"],
            "entries": [
                {"name":"servo-reference","effect":"edit","input_schema":{"type":"object","additionalProperties":false,"required":["asset_id"],"description":"Insert source-labelled references with explicit planning geometry. Fixed-lead controls affect pigtail markers; detachable sockets stay visible.","properties":model_properties},"slots":[{"kind":"ribbon","name":"Insert"}]},
                {"name":"horn-reference","effect":"edit","input_schema":{"type":"object","additionalProperties":false,"required":["horn_id"],"description":"Separate source dimensions or a supported partial solid. No installed servo position is inferred.","properties":{
                    "horn_id":{"type":"string","title":"Horn source variant","oneOf":horns.horns.iter().map(|horn|json!({"const":selector_key(&horn.id),"title":horn.model})).collect::<Vec<_>>(),"default":selector_key("unidentified.photo-se22ca210.double-arm")},
                    "mode":{"type":"string","title":"Reference type","default":"dimensions","oneOf":[{"const":"dimensions","title":"Known dimensions and hole references"},{"const":"source-solid","title":"Supported partial outer solid"},{"const":"photo-solid","title":"Shape estimated from photo"}]},
                    "x_mm":length("Origin X",-100000.0,100000.0),"y_mm":length("Origin Y",-100000.0,100000.0),"z_mm":length("Origin Z",-100000.0,100000.0),
                    "angle_deg":{"type":"number","title":"Visual angle","minimum":-360,"maximum":360,"default":0}
                }},"slots":[{"kind":"ribbon","name":"Insert"}]},
                {"name": "catalog", "effect": "read", "input_schema": {"type": "object", "additionalProperties": false, "properties": {
                    "query": {"type": "string", "title": "Search manufacturer or model", "default": "", "maxLength": 128}
                }}},
                {"name": "describe", "effect": "read", "input_schema": {"type": "object", "additionalProperties": false, "required": ["asset_id"], "properties": {"asset_id": {"type":"string","title":"Source profile ID","default":"towerpro.sg90-analog.configure"}}}},
                {"name": "insert", "effect": "edit", "input_schema": {"type": "object", "additionalProperties": false, "required": ["asset_id"], "description": "Envelope boxes only. Use servo-reference for a visible integrated servo with mounts, shaft and cable.", "properties": insert_properties}, "slots": [{"kind": "ribbon", "name": "Insert"}]},
                {"name": "mount-pattern", "effect": "edit", "input_schema": {"type": "object", "additionalProperties": false, "required": ["asset_id"], "description": "Insert screw-reference circles on a source-defined mounting face. Read hole type and thread notes.", "properties": pattern_properties}, "slots": [{"kind": "ribbon", "name": "Insert"}]},
                {"name":"output-reference", "effect":"edit", "input_schema":{"type":"object", "additionalProperties":false, "required":["asset_id"], "description":"Insert a source-frame output axis. A shaft solid requires a sourced diameter and vertical extents; unknown values stay unknown.", "properties":output_properties}, "slots":[{"kind":"ribbon","name":"Insert"}]},
                {"name": "cable-space", "effect": "edit", "input_schema": {"type": "object", "additionalProperties": false,
                    "required": ["asset_id", "port_id", "exit_x_mm", "exit_y_mm", "exit_z_mm", "direction", "bundle_width_mm", "bundle_height_mm", "straight_length_mm"],
                    "description": "Reserve measured or planned cable, outlet flex and connector space. These user-sized boxes are routing allowances.",
                    "properties": cable_properties}, "slots": [{"kind": "ribbon", "name": "Insert"}]}
            ],
            "limits": {"fuel": 500000000, "memory_mib": 32, "deadline_ms": 30000}
        }
    });
    if cfg!(feature = "legacy-host") {
        if let Some(object) = manifest.as_object_mut() {
            object.insert("capabilities".into(), json!(["commands@0.1"]));
            object.insert("description".into(), json!("Legacy host edition: servo references and cable planning spaces. Source snapshots are returned to callers; the older host cannot embed them in designs."));
        }
        if let Some(runtime) = manifest.get_mut("runtime").and_then(Value::as_object_mut) {
            runtime.remove("data_format");
            if let Some(commands) = runtime.get_mut("commands").and_then(Value::as_array_mut) {
                commands.retain(|command| !command.as_str().is_some_and(|id| id.starts_with("storage.")));
            }
        }
    }
    fit_host_budget(&mut manifest, catalog, &horns)?;
    Ok(manifest)
}

const MAX_MANIFEST_BYTES: usize = 16 * 1024;

fn encoded_size(manifest: &Value) -> Result<usize, Error> {
    // The native manifest command appends one newline.
    serde_json::to_vec(manifest)
        .map(|bytes| bytes.len() + 1)
        .map_err(|error| invalid("manifest", error))
}

fn fit_host_budget(manifest: &mut Value, catalog: &Catalog, horns: &crate::horns::HornCatalog) -> Result<(), Error> {
    // Keep the integrated-servo picker as long as possible. All entries still
    // validate canonical IDs in Rust after a long picker becomes a text field.
    for (entry, field, help) in [
        (
            "cable-space",
            "asset_id",
            "Source profile ID. Search Catalog to find a model.",
        ),
        (
            "mount-pattern",
            "asset_id",
            "Source profile ID. Search Catalog to find a model.",
        ),
        (
            "output-reference",
            "asset_id",
            "Source profile ID. Search Catalog to find a model.",
        ),
        (
            "mount-pattern",
            "mount_group_id",
            "Mounting group ID from Describe, or profile-default.",
        ),
        ("output-reference", "output_id", "Output ID from Describe."),
        ("cable-space", "port_id", "Cable port ID from Describe."),
        ("horn-reference", "horn_id", "Horn source ID from Catalog."),
        ("servo-reference", "asset_id", "Source profile ID from Catalog."),
        (
            "servo-reference",
            "horn_id",
            "Documented horn ID from Describe, or none.",
        ),
    ] {
        if encoded_size(manifest)? <= MAX_MANIFEST_BYTES {
            return Ok(());
        }
        let Some(properties) = manifest
            .get_mut("runtime")
            .and_then(|runtime| runtime.get_mut("entries"))
            .and_then(Value::as_array_mut)
            .and_then(|entries| entries.iter_mut().find(|item| item["name"] == entry))
            .and_then(|item| item.get_mut("input_schema"))
            .and_then(|schema| schema.get_mut("properties"))
            .and_then(|properties| properties.get_mut(field))
            .and_then(Value::as_object_mut)
        else {
            continue;
        };
        if !properties.contains_key("oneOf") && !properties.contains_key("enum") {
            continue;
        }
        let default = properties.get("default").and_then(Value::as_str).unwrap_or("");
        let canonical = match field {
            "asset_id" => catalog.asset(default)?.id.clone(),
            "horn_id" if default != "none" => horns.horn(default)?.id.clone(),
            _ => default.to_owned(),
        };
        properties.remove("oneOf");
        properties.remove("enum");
        properties.insert("default".into(), json!(canonical));
        properties.insert("description".into(), json!(help));
    }
    if encoded_size(manifest)? > MAX_MANIFEST_BYTES {
        return Err(invalid(
            "manifest",
            "host forms exceed the 16 KiB budget after compacting catalog selectors",
        ));
    }
    Ok(())
}
