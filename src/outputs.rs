//! Output axes have explicit source frames; unknown positions never become box centers.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::catalog::{Asset, Catalog, Dimension, Evidence, valid_id};
use crate::insert::{execute_plan, validate_placement};
use crate::{Error, InsertParams, Runner, invalid, recipe};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutputAxis {
    pub frame: String,
    pub origin_mm: [f64; 3],
    pub coordinate_tolerances_mm: [Option<f64>; 3],
    pub direction: [f64; 3],
    pub evidence: Evidence,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutputInterface {
    pub id: String,
    pub axis: Option<OutputAxis>,
    pub housing_top_dimension: Option<String>,
    pub shaft_top_dimension: Option<String>,
    pub frame_height_dimension: Option<String>,
    pub shaft_diameter_dimension: Option<String>,
    pub unknown: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutputParams {
    pub asset_id: String,
    #[serde(default = "main_output")]
    pub output_id: String,
    #[serde(default)]
    pub clearance_mm: f64,
    #[serde(default)]
    pub x_mm: f64,
    #[serde(default)]
    pub y_mm: f64,
    #[serde(default)]
    pub z_mm: f64,
}

fn main_output() -> String {
    "main".into()
}

pub(crate) fn validate(asset: &Asset, sources: &BTreeSet<&str>) -> Result<(), Error> {
    if asset.output_interfaces.len() > 32 {
        return Err(invalid(
            "catalog.output_interfaces",
            "at most 32 output interfaces per asset",
        ));
    }
    let mut ids = BTreeSet::new();
    for output in &asset.output_interfaces {
        if !valid_id(&output.id) || !ids.insert(&output.id) {
            return Err(invalid(
                "catalog.output_interfaces",
                "output IDs must be valid and unique",
            ));
        }
        for key in [
            &output.housing_top_dimension,
            &output.shaft_top_dimension,
            &output.frame_height_dimension,
            &output.shaft_diameter_dimension,
        ]
        .into_iter()
        .flatten()
        {
            if !asset.dimensions.contains_key(key) {
                return Err(invalid(
                    "catalog.output_interfaces",
                    format!("unknown dimension `{key}`"),
                ));
            }
        }
        let Some(axis) = &output.axis else {
            continue;
        };
        if !matches!(axis.frame.as_str(), "envelope_minimum" | "near_mount_center")
            || !matches!(axis.direction, [0.0, 0.0, 1.0] | [0.0, 0.0, -1.0])
            || !axis
                .origin_mm
                .iter()
                .all(|value| value.is_finite() && value.abs() <= 10000.0)
            || axis
                .coordinate_tolerances_mm
                .iter()
                .flatten()
                .any(|v| !v.is_finite() || !(0.0..=100.0).contains(v))
            || !sources.contains(axis.evidence.source.as_str())
            || !matches!(axis.evidence.method.as_str(), "drawing" | "derived" | "measurement")
        {
            return Err(invalid(
                "catalog.output_interfaces",
                "axis needs a supported frame, finite coordinates, +Z or -Z direction and known evidence",
            ));
        }
        let bottom = optional_dimension(asset, &output.housing_top_dimension);
        let top = optional_dimension(asset, &output.shaft_top_dimension);
        if axis.direction == [0.0, 0.0, -1.0] && (bottom.is_some() || top.is_some()) {
            return Err(invalid(
                "catalog.output_interfaces",
                "opposed axes currently support pose references without vertical extent adapters",
            ));
        }
        if bottom
            .zip(top)
            .is_some_and(|(bottom, top)| bottom.nominal_mm >= top.nominal_mm)
        {
            return Err(invalid(
                "catalog.output_interfaces",
                "shaft top must be above housing top",
            ));
        }
        if let Some(key) = &output.shaft_diameter_dimension
            && asset.dimensions.get(key).is_some_and(|d| d.nominal_mm <= 0.0)
        {
            return Err(invalid("catalog.output_interfaces", "shaft diameter must be positive"));
        }
        if axis.frame == "near_mount_center"
            && (asset
                .representations
                .get("mounting-pattern")
                .is_none_or(|representation| {
                    representation.kind != "mounting_pattern"
                        || representation.frame != axis.frame
                        || !asset.mount_groups.iter().any(|group| {
                            Some(&group.id) == representation.mount_group.as_ref() && group.frame == axis.frame
                        })
                })
                || (output.housing_top_dimension.is_some() || output.shaft_top_dimension.is_some())
                    && output.frame_height_dimension.is_none())
        {
            return Err(invalid(
                "catalog.output_interfaces",
                "mount-relative outputs need a mounting group; base-referenced heights also need a height datum",
            ));
        }
        if axis.frame == "envelope_minimum" && output.frame_height_dimension.is_some() {
            return Err(invalid(
                "catalog.output_interfaces",
                "the case-minimum frame uses the case base as its zero-height datum",
            ));
        }
    }
    Ok(())
}

fn optional_dimension<'a>(asset: &'a Asset, key: &Option<String>) -> Option<&'a Dimension> {
    key.as_ref().and_then(|key| asset.dimensions.get(key))
}

fn output<'a>(
    catalog: &'a Catalog,
    params: &OutputParams,
) -> Result<(&'a Asset, &'a OutputInterface, &'a OutputAxis), Error> {
    let asset = catalog.asset(&params.asset_id)?;
    let interface = asset
        .output_interfaces
        .iter()
        .find(|o| o.id == params.output_id)
        .ok_or_else(|| invalid("output_id", "unknown output interface"))?;
    let axis = interface.axis.as_ref().ok_or_else(|| {
        Error::Unsupported(format!(
            "{}: output-axis position is not established by this source profile",
            asset.model
        ))
    })?;
    Ok((asset, interface, axis))
}

pub fn summary(asset: &Asset, interface: &OutputInterface) -> Result<Value, Error> {
    let Some(axis) = &interface.axis else {
        return Ok(json!({"id": interface.id, "status": "unlocated", "axis": null, "unknown": interface.unknown}));
    };
    let housing = optional_dimension(asset, &interface.housing_top_dimension);
    let shaft_top = optional_dimension(asset, &interface.shaft_top_dimension);
    let datum = interface
        .frame_height_dimension
        .as_ref()
        .and_then(|key| asset.dimensions.get(key));
    let datum_value = datum.map_or(0.0, |d| d.nominal_mm);
    let datum_tolerance = datum.map_or(Some(0.0), |d| d.tolerance_mm);
    let derived = |value: f64, tolerance: Option<f64>| json!({"nominal_mm": value, "tolerance_mm": tolerance, "method": "difference_of_source_dimensions"});
    Ok(json!({
        "id": interface.id, "status": "located", "axis": axis,
        "housing_top_in_frame": housing.map(|height| derived(height.nominal_mm - datum_value, height.tolerance_mm.zip(datum_tolerance).map(|(a,b)|a+b))),
        "shaft_top_in_frame": shaft_top.map(|height| derived(height.nominal_mm - datum_value, height.tolerance_mm.zip(datum_tolerance).map(|(a,b)|a+b))),
        "shaft_protrusion": shaft_top.zip(housing).map(|(top,bottom)| derived(top.nominal_mm - bottom.nominal_mm, top.tolerance_mm.zip(bottom.tolerance_mm).map(|(a,b)|a+b))),
        "shaft_outer_diameter": interface.shaft_diameter_dimension.as_ref().and_then(|key| asset.dimensions.get(key)),
        "tolerance_note": "Derived bounds add the declared input tolerances; any unknown input leaves the bound unknown. A datum definition is exact, not an invented manufacturing tolerance.",
        "unknown": interface.unknown
    }))
}

pub fn output_recipe(catalog: &Catalog, params: &OutputParams) -> Result<Value, Error> {
    catalog.validate()?;
    let placement = InsertParams {
        asset_id: params.asset_id.clone(),
        representation: "output-reference".into(),
        mount_group_id: None,
        clearance_mm: params.clearance_mm,
        x_mm: params.x_mm,
        y_mm: params.y_mm,
        z_mm: params.z_mm,
    };
    validate_placement(&placement)?;
    if !valid_id(&params.output_id) {
        return Err(invalid("output_id", "invalid output ID"));
    }
    let (asset, interface, axis) = output(catalog, params)?;
    let base_representation = if axis.frame == "near_mount_center" {
        "mounting-pattern"
    } else {
        "case-envelope"
    };
    let base = recipe(
        catalog,
        &InsertParams {
            representation: base_representation.into(),
            ..placement
        },
    )?;
    let mut steps = base
        .as_array()
        .ok_or_else(|| invalid("recipe", "expected command array"))?
        .clone();
    let reference_height = interface
        .frame_height_dimension
        .as_ref()
        .and_then(|key| asset.dimensions.get(key))
        .map_or(0.0, |d| d.nominal_mm);
    let bottom = optional_dimension(asset, &interface.housing_top_dimension).map(|d| d.nominal_mm - reference_height);
    let top = optional_dimension(asset, &interface.shaft_top_dimension).map(|d| d.nominal_mm - reference_height);
    let [ax, ay, az] = axis.origin_mm;
    let marker_height = top.or(bottom).unwrap_or(az);
    let world_axis = [params.x_mm + ax, params.y_mm + ay, params.z_mm + az];
    let model = &asset.model;
    let mut marker_ids = Vec::new();
    if axis.frame == "near_mount_center" {
        let finish = steps.pop().ok_or_else(|| invalid("recipe", "missing sketch finish"))?;
        if finish.get("command").and_then(Value::as_str) != Some("sketch.finish") {
            return Err(invalid("recipe", "mounting recipe must finish its sketch"));
        }
    } else {
        steps.push(json!({"command": "sketch.create", "params": {
            "name": format!("{model} output center {}", if top.is_some() { "at shaft top" } else if bottom.is_some() { "at housing top" } else { "on datum plane" }),
            "plane": {"origin": [params.x_mm, params.y_mm, params.z_mm + marker_height], "x_dir": [1,0,0], "y_dir": [0,1,0]}, "frame": "world"
        }}));
    }
    // Cross size is a visual marker, not a source dimension or a shaft diameter.
    for (suffix, points) in [
        ("x", [[ax - 1.0, ay], [ax + 1.0, ay]]),
        ("y", [[ax, ay - 1.0], [ax, ay + 1.0]]),
    ] {
        let id = format!("output-{}-{suffix}", interface.id);
        steps.push(json!({"command": "sketch.line", "params": {"points": points, "construction": true, "ids": [id], "connect": false}}));
        marker_ids.push(id);
    }
    if let Some(diameter) = interface
        .shaft_diameter_dimension
        .as_ref()
        .and_then(|key| asset.dimensions.get(key))
    {
        let id = format!("output-{}-diameter", interface.id);
        steps.push(json!({"command": "sketch.circle.center", "params": {
            "center": [ax,ay], "diameter": format!("{} mm", diameter.nominal_mm), "construction": true, "id": id, "connect": false
        }}));
        marker_ids.push(id);
    }
    steps.push(json!({"command": "sketch.constraint.fix", "params": {"entities": marker_ids, "fixed": true}}));
    steps.push(json!({"command": "sketch.finish", "params": {}}));
    steps.push(json!({"command": "construct.axis.two_points", "params": {
        "a": {"point": world_axis}, "b": {"point": [world_axis[0] + axis.direction[0], world_axis[1] + axis.direction[1], world_axis[2] + axis.direction[2]]},
        "name": format!("{model} output axis {} ref r{}", interface.id, asset.data_revision)
    }}));
    if let Some(diameter) = interface
        .shaft_diameter_dimension
        .as_ref()
        .and_then(|key| asset.dimensions.get(key))
        && let Some((bottom, top)) = bottom.zip(top)
    {
        steps.push(json!({"command": "solid.cylinder", "params": {
            "base": [params.x_mm + ax, params.y_mm + ay, params.z_mm + bottom], "axis": axis.direction,
            "diameter": format!("{} mm", diameter.nominal_mm), "height": format!("{} mm", top - bottom), "operation": "new",
            "name": format!("{model} shaft envelope ref r{}", asset.data_revision), "body_name": format!("{model} shaft envelope ref r{}", asset.data_revision)
        }}));
    }
    Ok(Value::Array(steps))
}

pub fn insert_output_reference(
    runner: &mut impl Runner,
    catalog: &Catalog,
    params: &OutputParams,
) -> Result<Value, Error> {
    let plan = output_recipe(catalog, params)?;
    let (_, interface, axis) = output(catalog, params)?;
    let warning = "Source-frame output-axis reference. A shaft cylinder is an outside envelope, not mating spline teeth. Horn seating height and neutral angular phase remain unknown. Case-corner and mounting frames are distinct.";
    let mut reply = execute_plan(
        runner,
        catalog,
        &params.asset_id,
        "output-reference",
        serde_json::to_value(params).map_err(|e| invalid("params", e))?,
        plan,
        warning,
    )?;
    if let Some(object) = reply.as_object_mut() {
        object.insert(
            "output_interface".into(),
            serde_json::to_value(interface).map_err(|e| invalid("output_interface", e))?,
        );
        object.insert("reference_frame".into(), json!(axis.frame));
        object.insert(
            "output_reference".into(),
            summary(catalog.asset(&params.asset_id)?, interface)?,
        );
    }
    Ok(reply)
}
