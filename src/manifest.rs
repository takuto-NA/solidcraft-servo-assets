//! Forms for a short insertion path and the existing detailed commands.

use std::collections::BTreeSet;

use serde_json::{Value, json};

use crate::catalog::{Asset, Catalog, model_datum_status, selector_key};
use crate::horns::{HornCatalog, automatic_assembly_available};
use crate::workflow::{Datum, datums, horn_modes, horn_reference_id, horn_title, servo_title};
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
    for (key, label) in [("x_mm", "Offset X"), ("y_mm", "Offset Y"), ("z_mm", "Offset Z")] {
        properties[key] = number(label, -100000.0, 100000.0, "mm", Some(0.0));
    }
    properties
}

fn option(value: &str, title: &str) -> Value {
    json!({"const":value,"title":title})
}

fn catalog_option(id: &str, title: &str, detail: String) -> Value {
    let mut choice = option(&selector_key(id), title);
    if !cfg!(feature = "legacy-host") {
        choice["description"] = json!(detail);
    }
    choice
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

fn entry(name: &str, title: &str, effect: &str, schema: Value, ribbon: Option<&str>) -> Value {
    let mut slots = vec![json!({"kind":"menu","name":title})];
    if let Some(tab) = ribbon {
        slots.push(json!({"kind":"ribbon","name":tab}));
    }
    json!({"name":name,"effect":effect,"slots":slots,"input_schema":schema})
}

fn fields(required: &[&str], properties: Value) -> Value {
    json!({"type":"object","additionalProperties":false,"required":required,"properties":properties})
}

fn quick_entry(name: &str, title: &str, tab: &str, key: &str, property: Value, instruction: &str) -> Value {
    let mut schema = fields(&[key], positioned(json!({key:property})));
    schema["title"] = json!(title);
    schema["description"] = json!(instruction);
    schema["x-solidcraft-form"] = json!({"search":[key],"advanced":POSITION_FIELDS,"submit":"insert"});
    entry(name, title, "edit", schema, Some(tab))
}

fn origin(frame: &str) -> &str {
    match frame {
        "near_mount_center" => "near mounting centre on the ear underside",
        "envelope_minimum" => "minimum corner of the source envelope",
        _ => frame,
    }
}

fn servo_detail(asset: &Asset) -> String {
    let Some(model) = &asset.reference_model else {
        return String::new();
    };
    let mount_and_axis = match model_datum_status(asset) {
        "mount_and_axis_datums" => "Mounting and axis datums from the source. Case details are simplified.",
        "mount_datums" => "Mounting datums from the source. Output position is approximate.",
        "axis_datum" => "Output axis from the source. Mounting positions are approximate.",
        _ => "Mounting and output positions in this model are approximate.",
    };
    let case = asset
        .representations
        .get("case-envelope")
        .and_then(|envelope| envelope.size_dimensions.as_ref())
        .and_then(|keys| {
            Some([
                asset.dimensions.get(&keys[0])?.nominal_mm,
                asset.dimensions.get(&keys[1])?.nominal_mm,
                asset.dimensions.get(&keys[2])?.nominal_mm,
            ])
        })
        .map_or_else(String::new, |[x, y, z]| format!("Case envelope {x} × {y} × {z} mm. "));
    let cable = if let Some(lead) = model.cables.first() {
        format!(
            " Cable entry shows {} conductors over {} mm. Outlet location and size are approximate.",
            lead.conductor_colors.len(),
            lead.default_wire_length_mm
        )
    } else {
        let sockets: Vec<_> = asset
            .cable_ports
            .iter()
            .filter(|port| model.parts.iter().any(|part| part.id == format!("{}-socket", port.id)))
            .collect();
        if let [first, rest @ ..] = sockets.as_slice() {
            let contacts = first.contact_count.as_ref().filter(|count| {
                rest.iter().all(|port| {
                    port.contact_count
                        .as_ref()
                        .is_some_and(|other| other.value == count.value)
                })
            });
            let count = contacts.map_or_else(String::new, |count| format!(" with {} contacts each", count.value));
            format!(
                " {} detachable sockets{count}. Socket geometry is approximate.",
                sockets.len()
            )
        } else {
            String::new()
        }
    };
    let seller = asset
        .seller_items
        .first()
        .map_or_else(String::new, |item| format!("{} {}. ", item.seller, item.sku));
    format!(
        "{seller}{case}{mount_and_axis}{cable} Origin: {}.",
        origin(&model.frame)
    )
}

fn horn_choices(horns: &HornCatalog) -> Vec<Value> {
    let mut sorted: Vec<_> = horns.horns.iter().collect();
    sorted.sort_by_key(|horn| (&horn.manufacturer, &horn.model, &horn.id));
    sorted
        .into_iter()
        .flat_map(|horn| {
            horn_modes(horn).map(move |mode| {
                let detail = match mode {
                    "source-solid" => {
                        "Source geometry with omitted details. Loose reference, independent of installed fit."
                    }
                    "photo-solid" => {
                        "Shape inferred from a photo. Part identity, spline fit and installed seat are unknown."
                    }
                    _ => "Dimension and hole references from the source. This choice creates no solid.",
                };
                catalog_option(&horn_reference_id(horn, mode), &horn_title(horn, mode), detail.into())
            })
        })
        .collect()
}

fn datum_field(assets: &[&Asset]) -> Value {
    let options: Vec<_> = assets
        .iter()
        .flat_map(|asset| {
            datums(asset).map(move |datum| {
                let detail = match &datum {
                    Datum::Mount(group) => format!(
                            "{} source mounting centres. Circles mark holes or slots. See the selected profile for threads and tolerances. Origin: {}.",
                            group.holes.len(), origin(&group.frame)
                    ),
                    Datum::Output(output) => {
                        let frame = output.axis.as_ref().map_or("", |axis| axis.frame.as_str());
                        let geometry = if frame == "near_mount_center" { "Includes mounting centres in this frame." } else { "Includes a case envelope." };
                        format!("Source axis position. {geometry} A shaft envelope needs source diameter and extents. Origin: {}.", origin(frame))
                    }
                };
                catalog_option(&datum.reference_id(asset), &datum.title(asset), detail)
            })
        })
        .collect();
    catalog_field("Model and reference", options)
}

/// The optional form hints affect presentation only. Older hosts still get ordinary forms.
pub fn manifest(catalog: &Catalog) -> Result<Value, Error> {
    let horns = HornCatalog::embedded()?;
    let mut assets: Vec<_> = catalog.assets.iter().collect();
    assets.sort_by_key(|asset| (&asset.manufacturer, &asset.model, &asset.id));
    let servos: Vec<_> = assets
        .iter()
        .filter(|asset| asset.reference_model.is_some())
        .map(|asset| catalog_option(&asset.id, &servo_title(asset), servo_detail(asset)))
        .collect();
    let fs90 = "feetech.fs90.akizuki-drawing";
    let sg90 = "towerpro.sg90-analog.configure";
    let first_photo = "unidentified.photo-se22ca210.double-arm";
    let mut primary = vec![
        quick_entry(
            "place-servo",
            "Insert servo",
            "Insert",
            "asset_id",
            catalog_field("Servo model", servos),
            "Choose a model and check its mounting, shaft and cable entry in the viewport.",
        ),
        quick_entry(
            "place-horn",
            "Insert horn",
            "Insert",
            "reference_id",
            catalog_field("Horn and representation", horn_choices(&horns)),
            "Choose a loose horn solid or its dimension references.",
        ),
        quick_entry(
            "place-datum",
            "Insert mounts or axis",
            "Inspect",
            "reference_id",
            datum_field(&assets),
            "Choose a mounting face or output axis with a source-defined position.",
        ),
    ];
    if let Some(servo) = primary.first_mut() {
        let schema = &mut servo["input_schema"];
        schema["properties"]["show_callouts"] =
            json!({"type":"boolean","title":"Show interface callouts","default":false});
        schema["x-solidcraft-form"]["advanced"] = json!(["show_callouts", "x_mm", "y_mm", "z_mm"]);
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
    let cable_options = json!({
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
    let optional_fields = [
        (
            "servo-reference",
            "Servo setup",
            "edit",
            fields(&["asset_id"], model_options),
        ),
        (
            "horn-reference",
            "Horn setup",
            "edit",
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
            "Find assets",
            "read",
            fields(
                &[],
                json!({"query":{"type":"string","title":"Model, manufacturer or SKU","default":"","maxLength":128}}),
            ),
        ),
        (
            "describe",
            "Read servo sources",
            "read",
            fields(&["asset_id"], json!({"asset_id":identifiers(sg90)})),
        ),
        (
            "insert",
            "Insert envelope",
            "edit",
            fields(
                &["asset_id"],
                positioned(json!({
                    "asset_id":identifiers(sg90),
                    "representation":choice("Envelope",vec![option("case-envelope","Case envelope"),option("overall-envelope","Overall envelope")],Some("case-envelope")),
                    "clearance_mm":margin()
                })),
            ),
        ),
        (
            "mount-pattern",
            "Mounting setup",
            "edit",
            fields(
                &["asset_id"],
                positioned(
                    json!({"asset_id":identifiers(fs90),"mount_group_id":id_field("Mounting group ID","profile-default"),"clearance_mm":margin()}),
                ),
            ),
        ),
        (
            "output-reference",
            "Output setup",
            "edit",
            fields(
                &["asset_id"],
                positioned(
                    json!({"asset_id":identifiers(fs90),"output_id":id_field("Output ID","main"),"clearance_mm":margin()}),
                ),
            ),
        ),
        (
            "cable-space",
            "Plan cable space",
            "edit",
            fields(
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
            ),
        ),
    ];
    primary.extend(
        optional_fields
            .into_iter()
            .map(|(name, title, effect, schema)| entry(name, title, effect, schema, None)),
    );
    let legacy = cfg!(feature = "legacy-host");
    let mut commands = vec![
        "appearance.assign",
        "body.rename",
        "construct.axis.two_points",
        "document.inspect",
        "note.add",
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
        commands.extend(["storage.patch", "storage.put_blob"]);
    }
    let mut runtime = json!({"module":"servo-assets.wasm","commands":commands,"entries":primary,"limits":{"fuel":500000000,"memory_mib":32,"deadline_ms":30000}});
    if !legacy {
        runtime["data_format"] = json!(1);
    }
    let capabilities = if legacy {
        vec!["commands@0.1"]
    } else {
        vec!["commands@0.2", "storage@0.1"]
    };
    let mut manifest = json!({
        "id":"servo-assets","name":"Servo Assets","version":env!("CARGO_PKG_VERSION"),
        "description":"Servo and horn references for mounting and cable layout.",
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
        (
            "place-datum",
            "reference_id",
            "Use a source profile ID followed by /mount/GROUP or /output/OUTPUT.",
        ),
        (
            "place-horn",
            "reference_id",
            "Use a horn ID followed by /dimensions, /source-solid or /photo-solid.",
        ),
        (
            "place-servo",
            "asset_id",
            "Use Find assets to get the servo profile ID.",
        ),
    ] {
        if encoded_size(manifest)? <= budget {
            return Ok(());
        }
        let Some(schema) = manifest["runtime"]["entries"]
            .as_array_mut()
            .and_then(|entries| entries.iter_mut().find(|entry| entry["name"] == entry_name))
            .map(|entry| &mut entry["input_schema"])
        else {
            continue;
        };
        let Some(field) = schema["properties"].get_mut(field).and_then(Value::as_object_mut) else {
            continue;
        };
        field.remove("oneOf");
        field.remove("enum");
        field.insert("description".into(), json!(hint));
        schema["x-solidcraft-form"]["search"] = json!([]);
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
    // The old host has a 16 KiB manifest limit. Keep named choices in the three
    // insertion forms and keep the types and limits of the detailed commands.
    for entry in entries
        .iter_mut()
        .filter(|entry| !entry["name"].as_str().is_some_and(|name| name.starts_with("place-")))
    {
        if let Some(properties) = entry["input_schema"]["properties"].as_object_mut() {
            for field in properties.values_mut().filter_map(Value::as_object_mut) {
                field.remove("title");
                field.remove("unit");
            }
        }
    }
}
