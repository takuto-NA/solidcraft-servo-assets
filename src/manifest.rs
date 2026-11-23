//! Forms for a short insertion path and the existing detailed commands.

use std::collections::BTreeSet;

use serde_json::{Value, json};

use crate::catalog::{Asset, Catalog, selector_key};
use crate::horns::{HornCatalog, automatic_assembly_available};
use crate::workflow::{DatumKind, datums, horn_modes, horn_solid_mode, servo_title};
use crate::{Error, invalid};

pub const LEGACY_MANIFEST_BYTES: usize = 16 * 1024;
pub const MODERN_MANIFEST_BYTES: usize = 64 * 1024;
const POSITION_FIELDS: &[&str] = &["x_mm", "y_mm", "z_mm"];

fn number(title: &str, min: f64, max: f64, unit: &str, default: Option<f64>) -> Value {
    let mut property = json!({"type":"number","title":title,"minimum":min,"maximum":max,"unit":unit});
    if let Some(default) = default {
        property["default"] = json!(default);
    }
    property
}

fn positioned(mut properties: Value) -> Value {
    let default = if cfg!(feature = "legacy-host") { None } else { Some(0.0) };
    for (key, label) in [("x_mm", "Offset X"), ("y_mm", "Offset Y"), ("z_mm", "Offset Z")] {
        properties[key] = number(label, -100000.0, 100000.0, "mm", default);
    }
    properties
}

fn option(value: &str, title: &str) -> Value {
    json!({"const":value,"title":title})
}

fn catalog_option(id: &str, title: &str) -> Value {
    option(&selector_key(id), title)
}

fn choice(title: &str, options: Vec<Value>, default: Option<&str>) -> Value {
    let mut property = json!({"type":"string","title":title,"oneOf":options});
    if let Some(default) = default {
        property["default"] = json!(default);
    }
    property
}

fn catalog_field(title: &str, options: Vec<Value>) -> Value {
    if cfg!(feature = "legacy-host") {
        let names: Vec<_> = options.iter().filter_map(|option| option["title"].as_str()).collect();
        // Plain enum values fit all the rows in an older host's manifest. Keep
        // distinct keyed choices if two catalog records have the same title.
        if names.iter().copied().collect::<BTreeSet<_>>().len() == options.len() {
            return json!({"type":"string","title":title,"enum":names});
        }
    }
    choice(title, options, None)
}

fn id_field(title: &str, default: &str) -> Value {
    json!({"type":"string","title":title,"default":default})
}

fn entry(name: &str, title: &str, effect: &str, mut schema: Value, presentation: (&str, &str)) -> Value {
    let (label, icon) = presentation;
    // SolidCraft 0.47 uses the first ribbon slot for the Model button label.
    // Naming another tab opts into that tab's generic plugin toolbar.
    let slots = json!([{"kind":"menu","name":title},{"kind":"ribbon","name":label}]);
    schema["x-solidcraft-icon"] = json!(format!("icons/{icon}.svg"));
    json!({"name":name,"effect":effect,"slots":slots,"input_schema":schema})
}

fn fields(required: &[&str], properties: Value) -> Value {
    json!({"type":"object","additionalProperties":false,"required":required,"properties":properties})
}

fn picker_entry(name: &str, title: &str, presentation: (&str, &str), key: &str, property: Value) -> Value {
    let mut schema = fields(&[key], json!({key:property}));
    schema["title"] = json!(title);
    if cfg!(feature = "legacy-host") {
        return entry(name, title, "edit", schema, presentation);
    }
    let filter = if key == "horn_id" {
        "Filter by horn name"
    } else {
        "Filter by model or SKU"
    };
    schema["properties"]["model_filter"] = json!({"type":"string","title":filter,"maxLength":128});
    let mut entry = entry(name, title, "edit", schema, presentation);
    entry["dialog_hook"] = json!(true);
    entry
}

fn quick_entry(
    name: &str,
    title: &str,
    presentation: (&str, &str),
    key: &str,
    property: Value,
    instruction: &str,
) -> Value {
    let mut entry = picker_entry(name, title, presentation, key, property);
    let schema = &mut entry["input_schema"];
    schema["description"] = json!(instruction);
    let properties = &mut schema["properties"];
    *properties = positioned(std::mem::take(properties));
    if !cfg!(feature = "legacy-host") {
        properties["placement_options"] = json!({"type":"boolean","title":"Set a position"});
        for &field in POSITION_FIELDS {
            properties[field]["x-solidcraft-visible-when"] = placement_visibility();
        }
    }
    entry
}

fn placement_visibility() -> Value {
    // The unchecked form hides these fields. A script can supply offsets without the form toggle.
    json!({"not":{"field":"placement_options","eq":false}})
}

fn datum_field(assets: &[&Asset], kind: DatumKind, title: &str) -> Value {
    let options = assets
        .iter()
        .flat_map(|asset| {
            datums(asset)
                .filter(move |datum| kind.includes(datum))
                .map(move |datum| catalog_option(&datum.reference_id(asset), &datum.title(asset, kind)))
        })
        .collect();
    catalog_field(title, options)
}

fn horns_by_name(horns: &HornCatalog) -> Value {
    let mut sorted: Vec<_> = horns.horns.iter().collect();
    sorted.sort_by_key(|horn| (&horn.manufacturer, &horn.model, &horn.id));
    let choices = sorted
        .iter()
        .map(|horn| catalog_option(&horn.id, &horn.model))
        .collect();
    let mut picker = quick_entry(
        "insert-horn",
        "Insert a separate horn",
        ("Horn", "place-horn"),
        "horn_id",
        choice("Horn model", choices, None),
        "Choose a separate horn and what you want to insert.",
    );
    let legacy = cfg!(feature = "legacy-host");
    let mut modes = vec![option("dimensions", "Dimensions and hole references")];
    for (mode, label) in [
        ("source-solid", "3D outline from drawing"),
        ("photo-solid", "3D photo estimate"),
    ] {
        let available: Vec<_> = sorted
            .iter()
            .filter(|horn| horn_modes(horn).any(|value| value == mode))
            .map(|horn| selector_key(&horn.id))
            .collect();
        if !available.is_empty() {
            let mut variant = option(mode, label);
            if !legacy {
                variant["x-solidcraft-when"] = json!({"field":"horn_id","in":available});
            }
            modes.push(variant);
        }
    }
    let mut mode = choice("Insert as", modes, Some("dimensions"));
    if !legacy && !sorted.is_empty() {
        let available: Vec<_> = sorted.iter().map(|horn| selector_key(&horn.id)).collect();
        mode["x-solidcraft-visible-when"] = json!({"field":"horn_id","in":available});
    }
    picker["input_schema"]["properties"]["mode"] = mode;
    picker
}

fn servo_extras(properties: &mut Value, assets: &[&Asset], horns: &HornCatalog) {
    let leads: Vec<_> = assets
        .iter()
        .filter(|asset| {
            asset
                .reference_model
                .as_ref()
                .is_some_and(|model| !model.cables.is_empty())
        })
        .map(|asset| selector_key(&asset.id))
        .collect();
    let mut fitted = Vec::new();
    let mut fitted_models = BTreeSet::new();
    for horn in &horns.horns {
        let models: Vec<_> = horns
            .mounts
            .iter()
            .filter(|mount| mount.horn_id == horn.id && automatic_assembly_available(horn, mount))
            .filter(|mount| {
                assets
                    .iter()
                    .any(|asset| asset.id == mount.asset_id && asset.reference_model.is_some())
            })
            .map(|mount| selector_key(&mount.asset_id))
            .collect();
        if !models.is_empty() {
            fitted_models.extend(models.iter().cloned());
            let mut item = catalog_option(&horn.id, &horn.model);
            item["x-solidcraft-when"] = json!({"field":"asset_id","in":models});
            fitted.push(item);
        }
    }
    if !leads.is_empty() {
        properties["options"] = json!({
            "type":"boolean","title":"Customize fixed cable","default":false,
            "x-solidcraft-visible-when":{"field":"asset_id","in":leads}
        });
        properties["options_cable"] = json!({
            "type":"boolean","title":"Show fixed cable marker","default":true,
            "x-solidcraft-visible-when":{"field":"options","eq":true}
        });
        let cable = json!({"field":"options_cable","eq":true});
        properties["options_connector"] = json!({
            "type":"boolean","title":"Show connector marker","default":false,"x-solidcraft-visible-when":cable
        });
        properties["options_length_mm"] = number("Cable marker length", 2.0, 250.0, "mm", None);
        properties["options_length_mm"]["description"] = json!("Leave blank to use the model's default marker length.");
        properties["options_length_mm"]["x-solidcraft-visible-when"] = cable;
    }
    if !fitted.is_empty() {
        fitted.insert(0, option("none", "No extra horn"));
        properties["options_horn"] = choice("Fitted horn", fitted, Some("none"));
        properties["options_horn"]["x-solidcraft-visible-when"] = json!({"field":"asset_id","in":fitted_models});
    }
}

fn servo_picker(assets: &[&Asset]) -> Value {
    let choices = assets
        .iter()
        .filter(|asset| asset.reference_model.is_some())
        .map(|asset| catalog_option(&asset.id, &servo_title(asset)))
        .collect();
    catalog_field("Servo model", choices)
}

fn model_entries(assets: &[&Asset], horns: &HornCatalog) -> Vec<Value> {
    let mut available: Vec<_> = horns
        .horns
        .iter()
        .filter_map(|horn| horn_solid_mode(horn).map(|mode| (horn, mode)))
        .collect();
    available.sort_by_key(|(horn, _)| (&horn.manufacturer, &horn.model, &horn.id));
    let horn_choices = available
        .into_iter()
        .map(|(horn, mode)| {
            let title = if mode == "photo-solid" {
                if horn.model.ends_with(" / photo estimate") {
                    horn.model.clone()
                } else {
                    format!("{} · photo estimate", horn.model)
                }
            } else {
                format!("{} · partial 3D", horn.model)
            };
            catalog_option(&horn.id, &title)
        })
        .collect();
    vec![
        picker_entry(
            "insert-servo",
            "Insert servo model",
            ("Servo model", "insert-servo"),
            "asset_id",
            servo_picker(assets),
        ),
        picker_entry(
            "insert-horn-model",
            "Insert horn model",
            ("Horn model", "place-horn"),
            "horn_id",
            choice("Horn model", horn_choices, None),
        ),
    ]
}

fn tooling_entries(assets: &[&Asset], horns: &HornCatalog) -> Vec<Value> {
    let legacy = cfg!(feature = "legacy-host");
    let fs90 = "feetech.fs90.akizuki-drawing";
    let sg90 = "towerpro.sg90-analog.configure";
    let first_photo = "unidentified.photo-se22ca210.double-arm";
    let mut primary = vec![
        quick_entry(
            "insert-servo",
            "Insert servo",
            ("Servo", "insert-servo"),
            "asset_id",
            servo_picker(assets),
            "Choose a servo model to preview the housing, shaft and cable entry.",
        ),
        horns_by_name(horns),
        quick_entry(
            "insert-mounts",
            "Sketch mounting holes",
            ("Hole sketch", "place-datum"),
            "reference_id",
            datum_field(assets, DatumKind::Mounting, "Model and mounting face"),
            "Creates a sketch at the documented hole and slot centers.",
        ),
        quick_entry(
            "insert-axis",
            "Insert shaft axis",
            ("Shaft axis", "shaft-axis"),
            "reference_id",
            datum_field(assets, DatumKind::ShaftAxis, "Model and shaft"),
            "Creates a construction axis and marks the documented center in the sketch.",
        ),
    ];
    if let Some(servo) = primary.first_mut() {
        let schema = &mut servo["input_schema"];
        schema["properties"]["show_callouts"] =
            json!({"type":"boolean","title":"Add fixed-position notes","default":false});
        if !legacy {
            schema["properties"]["show_callouts"]["x-solidcraft-visible-when"] = placement_visibility();
            servo_extras(&mut schema["properties"], assets, horns);
        }
    }
    let identifiers = |key: &str| id_field("Servo profile ID", key);
    let margin = || number("Extra clearance per side", 0.0, 10.0, "mm", Some(0.0));
    let angle = |title: &str| number(title, -360.0, 360.0, "°", Some(0.0));
    let cord = |title: &str, min, max, default| number(title, min, max, "mm", default);
    let mut assembly = vec![option("none", "Base profile / no extra horn")];
    assembly.extend(
        horns
            .horns
            .iter()
            .filter(|horn| {
                horns
                    .mounts
                    .iter()
                    .any(|mount| mount.horn_id == horn.id && automatic_assembly_available(horn, mount))
            })
            .map(|horn| option(&selector_key(&horn.id), &horn.model)),
    );
    let model_options = positioned(json!({
        "asset_id":identifiers(fs90),
        "include_cable":{"type":"boolean","title":"Show fixed cable marker","default":true},
        "include_connector":{"type":"boolean","title":"Show illustrative connector","default":false},
        "outlet_height_mm":cord("Approximate outlet height",0.1,20.0,Some(2.0)),
        "wire_diameter_mm":cord("Approximate wire diameter",0.1,5.0,Some(1.0)),
        "wire_pitch_mm":cord("Approximate wire spacing",0.1,10.0,Some(1.1)),
        "wire_length_mm":cord("Cable marker length",2.0,250.0,Some(3.0)),
        "labels":choice("Labels",vec![option("notes","Viewport callouts"),option("front","Front view labels"),option("none","No labels")],Some("notes")),
        "horn_id":choice("Documented horn assembly",assembly,Some("none")),
        "horn_angle_deg":angle("Horn visual angle"),
        "horn_axial_offset_mm":cord("Horn axial offset",-100.0,100.0,Some(0.0))
    }));
    let mut cable_options = json!({
        "asset_id":identifiers(sg90), "port_id":id_field("Cable port ID","servo-lead"),
        "exit_x_mm":cord("Outlet X",-100000.0,100000.0,None),
        "exit_y_mm":cord("Outlet Y",-100000.0,100000.0,None),
        "exit_z_mm":cord("Outlet Z",-100000.0,100000.0,None),
        "direction":choice("Exit direction",vec![option("positive_x","+X"),option("negative_x","−X"),option("positive_y","+Y"),option("negative_y","−Y"),option("positive_z","+Z"),option("negative_z","−Z")],Some("negative_x")),
        "bundle_width_mm":cord("Harness width",0.01,1000.0,None),
        "bundle_height_mm":cord("Harness thickness",0.01,1000.0,None),
        "straight_length_mm":cord("Routing length",0.01,1000.0,None),
        "flex_space_mm":cord("Extra flex space",0.0,100.0,Some(0.0)),
        "connector_width_mm":cord("Connector width",0.0,100.0,Some(0.0)),
        "connector_height_mm":cord("Connector thickness",0.0,100.0,Some(0.0)),
        "connector_length_mm":cord("Connector length",0.0,100.0,Some(0.0))
    });
    let mut envelope_model = identifiers(sg90);
    if !legacy {
        let envelope_models = assets
            .iter()
            .filter(|asset| asset.representations.contains_key("case-envelope"))
            .map(|asset| option(&asset.id, &servo_title(asset)))
            .collect();
        envelope_model = choice("Servo model", envelope_models, None);
        let cable_models = assets
            .iter()
            .filter(|asset| !asset.cable_ports.is_empty())
            .map(|asset| option(&asset.id, &servo_title(asset)))
            .collect();
        cable_options["asset_id"] = choice("Servo model", cable_models, None);
        let ports: BTreeSet<_> = assets
            .iter()
            .flat_map(|asset| asset.cable_ports.iter().map(|port| port.id.as_str()))
            .collect();
        cable_options["port_id"] = choice(
            "Cable exit",
            ports
                .into_iter()
                .map(|id| {
                    let label = match id {
                        "servo-lead" => "Fixed cable",
                        "bus-a" => "Bus socket A",
                        "bus-b" => "Bus socket B",
                        "bus-negative-y" => "Bus socket on side −Y",
                        "bus-positive-y" => "Bus socket on side +Y",
                        "dual-joint" => "Dual-joint socket",
                        "external-io" => "External I/O socket",
                        "ttl-a" => "TTL socket A",
                        "ttl-b" => "TTL socket B",
                        id => id,
                    };
                    option(id, label)
                })
                .collect(),
            None,
        );
    }
    let mut envelope = fields(
        &["asset_id"],
        positioned(json!({
            "asset_id":envelope_model,
            "representation":choice("Envelope",vec![option("case-envelope","Case envelope"),option("overall-envelope","Overall envelope")],Some("case-envelope")),
            "clearance_mm":margin()
        })),
    );
    envelope["title"] = json!("Insert clearance envelope");
    primary.push(entry(
        "insert",
        "Insert clearance envelope",
        "edit",
        envelope,
        ("Size box", "envelope"),
    ));
    let mut cable_space = fields(
        &[
            "asset_id",
            "port_id",
            "exit_x_mm",
            "exit_y_mm",
            "exit_z_mm",
            "direction",
            "bundle_width_mm",
            "bundle_height_mm",
            "straight_length_mm",
        ],
        cable_options,
    );
    cable_space["title"] = json!("Create cable clearance");
    cable_space["description"] = json!("Enter the planned cable exit, bundle size and routing length.");
    primary.push(entry(
        "cable-space",
        "Create cable clearance",
        "edit",
        cable_space,
        ("Cable space", "cable-space"),
    ));

    let detailed_entries = [
        (
            "servo-reference",
            "Insert servo with manual dimensions",
            "edit",
            ("Manual servo inputs", "options"),
            fields(&["asset_id"], model_options),
        ),
        (
            "horn-reference",
            "Insert horn by source ID",
            "edit",
            ("Horn source ID", "asset-id"),
            fields(
                &["horn_id"],
                positioned(json!({
                    "horn_id":id_field("Horn variant ID",first_photo),
                    "mode":choice("Representation",vec![option("dimensions","Dimension references"),option("source-solid","Partial source solid"),option("photo-solid","Photo estimate")],Some("dimensions")),
                    "angle_deg":angle("Visual angle")
                })),
            ),
        ),
        (
            "catalog",
            "Look up catalog data (JSON)",
            "read",
            ("Catalog JSON", "search"),
            fields(
                &[],
                json!({"query":{"type":"string","title":"Model, manufacturer or SKU","default":"","maxLength":128}}),
            ),
        ),
        (
            "describe",
            "Read servo dimensions and sources (JSON)",
            "read",
            ("Source data", "sources"),
            fields(&["asset_id"], json!({"asset_id":identifiers(sg90)})),
        ),
        (
            "mount-pattern",
            "Insert mounting sketch by ID",
            "edit",
            ("Mounts by ID", "place-datum"),
            fields(
                &["asset_id"],
                positioned(
                    json!({"asset_id":identifiers(fs90),"mount_group_id":id_field("Mounting group ID","profile-default"),"clearance_mm":margin()}),
                ),
            ),
        ),
        (
            "output-reference",
            "Insert output reference by ID",
            "edit",
            ("Output reference ID", "shaft-axis"),
            fields(
                &["asset_id"],
                positioned(
                    json!({"asset_id":identifiers(fs90),"output_id":id_field("Output ID","main"),"clearance_mm":margin()}),
                ),
            ),
        ),
        (
            "place-servo",
            "Insert servo by ID",
            "edit",
            ("Servo by ID", "asset-id"),
            fields(
                &["asset_id"],
                positioned(json!({
                    "asset_id":{"type":"string","title":"Servo model or source ID"},
                    "show_callouts":{"type":"boolean","title":"Add fixed-position notes","default":false}
                })),
            ),
        ),
        (
            "place-horn",
            "Insert horn by variant and representation ID",
            "edit",
            ("Horn variant ID", "asset-id"),
            fields(
                &["reference_id"],
                positioned(json!({
                    "reference_id":{"type":"string","title":"Horn ID and representation"}
                })),
            ),
        ),
        (
            "place-datum",
            "Insert a mounting or output reference by ID",
            "edit",
            ("Datum source ID", "asset-id"),
            fields(
                &["reference_id"],
                positioned(json!({
                    "reference_id":{"type":"string","title":"Mounting face or shaft source ID"}
                })),
            ),
        ),
    ];
    primary.extend(
        detailed_entries
            .into_iter()
            .map(|(name, title, effect, presentation, schema)| entry(name, title, effect, schema, presentation)),
    );
    primary
}

pub fn manifest(catalog: &Catalog) -> Result<Value, Error> {
    package_manifest(catalog, model_entries, "Insert editable 3D servo and horn models.")
}

pub fn tooling_manifest(catalog: &Catalog) -> Result<Value, Error> {
    package_manifest(
        catalog,
        tooling_entries,
        "Servo and horn references for assembly layout.",
    )
}

fn package_manifest(
    catalog: &Catalog,
    entries: fn(&[&Asset], &HornCatalog) -> Vec<Value>,
    description: &str,
) -> Result<Value, Error> {
    let horns = HornCatalog::embedded()?;
    let mut assets: Vec<_> = catalog.assets.iter().collect();
    assets.sort_by_key(|asset| (&asset.manufacturer, &asset.model, &asset.id));
    let legacy = cfg!(feature = "legacy-host");
    let mut commands = vec![
        "appearance.assign",
        "body.rename",
        "component.activate",
        "component.create",
        "component.list",
        "construct.axis.two_points",
        "document.inspect",
        "note.add",
        "occurrence.move",
        "sketch.arc.center_point",
        "sketch.circle.center",
        "sketch.constraint.fix",
        "sketch.create",
        "sketch.finish",
        "sketch.line",
        "sketch.text",
        "solid.box",
        "solid.cylinder",
        "solid.extrude",
        "solid.move",
    ];
    if !legacy {
        commands.push("storage.patch");
    }
    let mut runtime = json!({"module":"servo-assets.wasm","commands":commands,"entries":entries(&assets, &horns),"limits":{"fuel":500000000,"memory_mib":32,"deadline_ms":30000}});
    if !legacy {
        runtime["data_format"] = json!(1);
    }
    let capabilities = if legacy {
        vec!["commands@0.1"]
    } else {
        vec!["commands@0.2", "dialog@0.1", "storage@0.1"]
    };
    let mut manifest = json!({
        "id":"servo-assets","name":"Servo Assets","version":env!("CARGO_PKG_VERSION"),
        "description":description,
        "capabilities":capabilities,"runtime":runtime
    });
    fit_host_budget(&mut manifest)?;
    Ok(manifest)
}

fn encoded_size(manifest: &Value) -> Result<usize, Error> {
    serde_json::to_vec(manifest)
        .map(|bytes| bytes.len() + 1)
        .map_err(|error| invalid("manifest", error))
}

fn fit_host_budget(manifest: &mut Value) -> Result<(), Error> {
    let budget = if cfg!(feature = "legacy-host") {
        LEGACY_MANIFEST_BYTES
    } else {
        MODERN_MANIFEST_BYTES
    };
    if cfg!(feature = "legacy-host") && encoded_size(manifest)? > budget {
        compact_setup_forms(manifest);
    }
    for (entry_name, field, hint) in [
        ("cable-space", "asset_id", "Enter the source profile ID for the servo."),
        ("insert", "asset_id", "Enter the source profile ID for the servo."),
        (
            "insert-mounts",
            "reference_id",
            "Use a source profile ID followed by /mount/GROUP.",
        ),
        (
            "insert-axis",
            "reference_id",
            "Use a source profile ID followed by /output/OUTPUT.",
        ),
        ("insert-horn", "horn_id", "Enter the horn's source ID."),
        (
            "insert-horn-model",
            "horn_id",
            "Enter the source ID for a horn with a 3D model.",
        ),
        ("insert-servo", "asset_id", "Enter the source profile ID for the servo."),
    ] {
        if encoded_size(manifest)? <= budget {
            return Ok(());
        }
        let Some(entry) = manifest["runtime"]["entries"]
            .as_array_mut()
            .and_then(|entries| entries.iter_mut().find(|entry| entry["name"] == entry_name))
        else {
            continue;
        };
        let Some(properties) = entry["input_schema"]["properties"].as_object_mut() else {
            continue;
        };
        if let Some(field) = properties.get_mut(field).and_then(Value::as_object_mut) {
            field.remove("oneOf");
            field.remove("enum");
            field.insert("description".into(), json!(hint));
        }
        properties.remove("model_filter");
        if entry_name == "insert-horn"
            && let Some(mode) = properties.get_mut("mode").and_then(Value::as_object_mut)
        {
            mode.remove("x-solidcraft-visible-when");
            if let Some(choices) = mode.get_mut("oneOf").and_then(Value::as_array_mut) {
                for choice in choices.iter_mut().filter_map(Value::as_object_mut) {
                    choice.remove("x-solidcraft-when");
                }
            }
        }
        if entry_name == "insert-servo" {
            properties.retain(|key, _| key != "options" && !key.starts_with("options_"));
        }
        if let Some(object) = entry.as_object_mut() {
            object.remove("dialog_hook");
        }
    }
    if encoded_size(manifest)? > budget {
        return Err(invalid("manifest", "forms exceed the host manifest limit"));
    }
    Ok(())
}

fn compact_setup_forms(manifest: &mut Value) {
    let Some(entries) = manifest["runtime"]["entries"].as_array_mut() else {
        return;
    };
    // The old host has a 16 KiB manifest limit. Keep named choices in the main
    // insertion forms and keep the types and limits of the detailed commands.
    for entry in entries.iter_mut().filter(|entry| {
        !matches!(
            entry["name"].as_str(),
            Some("insert-servo" | "insert-horn-model" | "insert-horn" | "insert-mounts" | "insert-axis")
        )
    }) {
        if let Some(properties) = entry["input_schema"]["properties"].as_object_mut() {
            for field in properties.values_mut().filter_map(Value::as_object_mut) {
                field.remove("title");
                field.remove("unit");
            }
        }
    }
}
