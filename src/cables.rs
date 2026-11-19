//! Cable ports are hardware facts; their routing allowances are explicit user input.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{Error, Runner, catalog::Catalog, insert::execute_plan, invalid};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExitDirection {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CableSpaceParams {
    pub asset_id: String,
    pub port_id: String,
    pub exit_x_mm: f64,
    pub exit_y_mm: f64,
    pub exit_z_mm: f64,
    pub direction: ExitDirection,
    pub bundle_width_mm: f64,
    pub bundle_height_mm: f64,
    pub straight_length_mm: f64,
    #[serde(default)]
    pub flex_space_mm: f64,
    #[serde(default)]
    pub connector_width_mm: f64,
    #[serde(default)]
    pub connector_height_mm: f64,
    #[serde(default)]
    pub connector_length_mm: f64,
}

pub fn cable_recipe(catalog: &Catalog, params: &CableSpaceParams) -> Result<Value, Error> {
    catalog.validate()?;
    let asset = catalog.asset(&params.asset_id)?;
    if !asset.cable_ports.iter().any(|port| port.id == params.port_id) {
        return Err(invalid("port_id", "unknown cable port; select a documented port"));
    }
    for (field, value) in [
        ("exit_x_mm", params.exit_x_mm),
        ("exit_y_mm", params.exit_y_mm),
        ("exit_z_mm", params.exit_z_mm),
    ] {
        if !value.is_finite() || value.abs() > 100000.0 {
            return Err(invalid(field, "must be finite and within 100000 mm of the origin"));
        }
    }
    for (field, value) in [
        ("bundle_width_mm", params.bundle_width_mm),
        ("bundle_height_mm", params.bundle_height_mm),
        ("straight_length_mm", params.straight_length_mm),
    ] {
        if !value.is_finite() || !(0.01..=1000.0).contains(&value) {
            return Err(invalid(field, "must be finite and between 0.01 and 1000 mm"));
        }
    }
    for (field, value) in [
        ("flex_space_mm", params.flex_space_mm),
        ("connector_width_mm", params.connector_width_mm),
        ("connector_height_mm", params.connector_height_mm),
        ("connector_length_mm", params.connector_length_mm),
    ] {
        if !value.is_finite() || !(0.0..=100.0).contains(&value) {
            return Err(invalid(field, "must be finite and between 0 and 100 mm"));
        }
    }
    let connector = [
        params.connector_width_mm,
        params.connector_height_mm,
        params.connector_length_mm,
    ];
    if connector.iter().any(|v| *v > 0.0) && connector.contains(&0.0) {
        return Err(invalid(
            "connector",
            "supply all three connector envelope sizes, or leave all three at zero",
        ));
    }
    let [x, y, z] = [params.exit_x_mm, params.exit_y_mm, params.exit_z_mm];
    let [w, h, length] = [
        params.bundle_width_mm,
        params.bundle_height_mm,
        params.straight_length_mm,
    ];
    let local_box = |start: f64, along: f64, width: f64, height: f64| match params.direction {
        ExitDirection::PositiveX => ([x + start, y - width / 2.0, z - height / 2.0], [along, width, height]),
        ExitDirection::NegativeX => (
            [x - start - along, y - width / 2.0, z - height / 2.0],
            [along, width, height],
        ),
        ExitDirection::PositiveY => ([x - width / 2.0, y + start, z - height / 2.0], [width, along, height]),
        ExitDirection::NegativeY => (
            [x - width / 2.0, y - start - along, z - height / 2.0],
            [width, along, height],
        ),
        ExitDirection::PositiveZ => ([x - width / 2.0, y - height / 2.0, z + start], [width, height, along]),
        ExitDirection::NegativeZ => (
            [x - width / 2.0, y - height / 2.0, z - start - along],
            [width, height, along],
        ),
    };
    let mut steps = Vec::new();
    let mut add = |role: &str, start: f64, along: f64, width: f64, height: f64| {
        let (corner, [a, b, c]) = local_box(start, along, width, height);
        let label = format!("{} {} {} planning space", asset.model, params.port_id, role);
        steps.push(json!({"command": "solid.box", "params": {
            "corner": corner, "length": format!("{a} mm"), "width": format!("{b} mm"), "height": format!("{c} mm"),
            "name": label, "body_name": label, "operation": "new"
        }}));
    };
    add("cable", 0.0, length, w, h);
    if params.flex_space_mm > 0.0 {
        let flex = params.flex_space_mm;
        add("entry flex", -flex, 2.0 * flex, w + 2.0 * flex, h + 2.0 * flex);
    }
    if params.connector_length_mm > 0.0 {
        add(
            "connector",
            length,
            params.connector_length_mm,
            params.connector_width_mm,
            params.connector_height_mm,
        );
    }
    Ok(Value::Array(steps))
}

pub fn insert_cable_space(
    runner: &mut impl Runner,
    catalog: &Catalog,
    params: &CableSpaceParams,
) -> Result<Value, Error> {
    let plan = cable_recipe(catalog, params)?;
    let parameters = serde_json::to_value(params).map_err(|e| invalid("params", e))?;
    let mut reply = execute_plan(
        runner,
        catalog,
        &params.asset_id,
        "cable-space",
        parameters,
        plan,
        "User-specified planning envelopes. Exit location, cross-section, flex allowance and connector size are not factory dimensions. Flex space is not a certified minimum bend radius.",
    )?;
    let port = catalog
        .asset(&params.asset_id)?
        .cable_ports
        .iter()
        .find(|port| port.id == params.port_id)
        .ok_or_else(|| invalid("port_id", "unknown port"))?;
    if let Some(object) = reply.as_object_mut() {
        object.insert(
            "port".into(),
            serde_json::to_value(port).map_err(|e| invalid("port", e))?,
        );
    }
    Ok(reply)
}
