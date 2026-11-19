//! Integrated reference models keep visible parts and uncertainty in one datum frame.

use crate::catalog::{Asset, Catalog, valid_id};
use crate::insert::{PlanContext, execute_plan_context, validate_placement};
use crate::{Error, InsertParams, Runner, invalid};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Primitive {
    Box {
        corner_mm: [f64; 3],
        size_mm: [f64; 3],
    },
    Cylinder {
        base_mm: [f64; 3],
        axis: [f64; 3],
        diameter_mm: f64,
        height_mm: f64,
    },
    Profile {
        z_mm: f64,
        height_mm: f64,
        segments: Vec<ProfileSegment>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProfileSegment {
    Line {
        start_mm: [f64; 2],
        end_mm: [f64; 2],
    },
    Arc {
        center_mm: [f64; 2],
        start_mm: [f64; 2],
        end_mm: [f64; 2],
        ccw: bool,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Modifier {
    pub operation: String,
    pub geometry: Primitive,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Part {
    pub id: String,
    pub label: String,
    pub basis: String,
    pub note: String,
    pub color: String,
    pub geometry: Primitive,
    pub modifiers: Vec<Modifier>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CableModel {
    pub port_id: String,
    pub face_base_mm: [f64; 3],
    pub conductor_colors: Vec<String>,
    pub default_outlet_height_mm: f64,
    pub default_wire_diameter_mm: f64,
    pub default_wire_pitch_mm: f64,
    pub default_wire_length_mm: f64,
    pub connector_size_mm: [f64; 3],
    pub connector_color: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelNote {
    pub anchor_mm: [f64; 3],
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrontLabel {
    pub at_mm: [f64; 2],
    pub anchor_mm: [f64; 2],
    pub text: String,
    pub cable_port: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceModel {
    pub frame: String,
    pub assumptions: Vec<String>,
    pub parts: Vec<Part>,
    pub cables: Vec<CableModel>,
    pub notes: Vec<ModelNote>,
    pub front_labels: Vec<FrontLabel>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Labels {
    #[default]
    Notes,
    Front,
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelParams {
    pub asset_id: String,
    #[serde(default)]
    pub horn_id: Option<String>,
    #[serde(default)]
    pub horn_angle_deg: f64,
    #[serde(default)]
    pub horn_axial_offset_mm: f64,
    #[serde(default)]
    pub x_mm: f64,
    #[serde(default)]
    pub y_mm: f64,
    #[serde(default)]
    pub z_mm: f64,
    #[serde(default = "yes")]
    pub include_cable: bool,
    #[serde(default)]
    pub include_connector: bool,
    #[serde(default)]
    pub outlet_height_mm: Option<f64>,
    #[serde(default)]
    pub wire_diameter_mm: Option<f64>,
    #[serde(default)]
    pub wire_pitch_mm: Option<f64>,
    #[serde(default)]
    pub wire_length_mm: Option<f64>,
    #[serde(default)]
    pub labels: Labels,
}

fn yes() -> bool {
    true
}
fn vector(v: &[f64; 3]) -> bool {
    v.iter().all(|v| v.is_finite() && v.abs() <= 10000.0)
}
pub(crate) fn color(v: &str) -> bool {
    v.len() == 7 && v.starts_with('#') && v.bytes().skip(1).all(|c| c.is_ascii_hexdigit())
}

const PROFILE_TOL_MM: f64 = 1e-7;

fn distance2(a: [f64; 2], b: [f64; 2]) -> f64 {
    (a[0] - b[0]).hypot(a[1] - b[1])
}

impl ProfileSegment {
    fn start(&self) -> [f64; 2] {
        match self {
            Self::Line { start_mm, .. } | Self::Arc { start_mm, .. } => *start_mm,
        }
    }

    fn end(&self) -> [f64; 2] {
        match self {
            Self::Line { end_mm, .. } | Self::Arc { end_mm, .. } => *end_mm,
        }
    }

    fn sweep(&self) -> f64 {
        match self {
            Self::Line { .. } => 0.0,
            Self::Arc {
                center_mm,
                start_mm,
                end_mm,
                ccw,
            } => {
                let start = (start_mm[1] - center_mm[1]).atan2(start_mm[0] - center_mm[0]);
                let end = (end_mm[1] - center_mm[1]).atan2(end_mm[0] - center_mm[0]);
                if *ccw {
                    (end - start).rem_euclid(std::f64::consts::TAU)
                } else {
                    -(start - end).rem_euclid(std::f64::consts::TAU)
                }
            }
        }
    }

    fn valid(&self) -> bool {
        let point = |point: &[f64; 2]| point.iter().all(|value| value.is_finite() && value.abs() <= 10000.0);
        if !point(&self.start()) || !point(&self.end()) {
            return false;
        }
        match self {
            Self::Line { start_mm, end_mm } => (0.001..=1000.0).contains(&distance2(*start_mm, *end_mm)),
            Self::Arc {
                center_mm,
                start_mm,
                end_mm,
                ..
            } => {
                let radius = distance2(*center_mm, *start_mm);
                point(center_mm)
                    && (0.0005..=500.0).contains(&radius)
                    && (radius - distance2(*center_mm, *end_mm)).abs() <= PROFILE_TOL_MM
                    && distance2(*start_mm, *end_mm) > PROFILE_TOL_MM
                    && radius * self.sweep().abs() >= 0.001
            }
        }
    }

    fn signed_area(&self) -> f64 {
        let [ax, ay] = self.start();
        let [bx, by] = self.end();
        match self {
            Self::Line { .. } => (ax * by - ay * bx) * 0.5,
            Self::Arc { center_mm, .. } => {
                let [cx, cy] = center_mm;
                let radius = distance2(*center_mm, self.start());
                (cx * (by - ay) - cy * (bx - ax) + radius * radius * self.sweep()) * 0.5
            }
        }
    }

    fn bounds(&self) -> ([f64; 2], [f64; 2]) {
        match self {
            Self::Line { start_mm, end_mm } => (
                [start_mm[0].min(end_mm[0]), start_mm[1].min(end_mm[1])],
                [start_mm[0].max(end_mm[0]), start_mm[1].max(end_mm[1])],
            ),
            Self::Arc {
                center_mm, start_mm, ..
            } => {
                let radius = distance2(*center_mm, *start_mm);
                (
                    [center_mm[0] - radius, center_mm[1] - radius],
                    [center_mm[0] + radius, center_mm[1] + radius],
                )
            }
        }
    }
}

fn profile_valid(z_mm: f64, height_mm: f64, segments: &[ProfileSegment]) -> bool {
    if !z_mm.is_finite()
        || z_mm.abs() > 10000.0
        || !height_mm.is_finite()
        || !(0.001..=1000.0).contains(&height_mm)
        || !(2..=64).contains(&segments.len())
        || !segments.iter().all(ProfileSegment::valid)
        || !segments
            .iter()
            .zip(segments.iter().cycle().skip(1))
            .all(|(left, right)| distance2(left.end(), right.start()) <= PROFILE_TOL_MM)
        || segments.iter().map(ProfileSegment::signed_area).sum::<f64>().abs() <= 1e-8
    {
        return false;
    }
    let (min, max) = segments.iter().map(ProfileSegment::bounds).fold(
        ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]),
        |(min, max), (a, b)| {
            (
                [min[0].min(a[0]), min[1].min(a[1])],
                [max[0].max(b[0]), max[1].max(b[1])],
            )
        },
    );
    max[0] - min[0] <= 1000.0 && max[1] - min[1] <= 1000.0
}

/// Conservative label placement only: arcs use their full circle's analytic bound.
pub(crate) fn profile_lower_y(z_mm: f64, height_mm: f64, segments: &[ProfileSegment], matrix: &[f64; 12]) -> f64 {
    let world_y = |point: [f64; 2]| matrix[4] * point[0] + matrix[5] * point[1] + matrix[6] * z_mm + matrix[7];
    segments
        .iter()
        .map(|segment| match segment {
            ProfileSegment::Line { start_mm, end_mm } => world_y(*start_mm).min(world_y(*end_mm)),
            ProfileSegment::Arc {
                center_mm, start_mm, ..
            } => world_y(*center_mm) - distance2(*center_mm, *start_mm) * matrix[4].hypot(matrix[5]),
        })
        .fold(f64::INFINITY, f64::min)
        + (matrix[6] * height_mm).min(0.0)
}

impl Primitive {
    fn lower_y(&self) -> f64 {
        match self {
            Self::Box { corner_mm, .. } => corner_mm[1],
            Self::Cylinder {
                base_mm,
                axis,
                diameter_mm,
                height_mm,
            } => {
                let norm = axis[0].hypot(axis[1]).hypot(axis[2]);
                let y_direction = axis[1] / norm;
                let radius_y = diameter_mm * 0.5 * (1.0 - y_direction * y_direction).max(0.0).sqrt();
                base_mm[1].min(base_mm[1] + height_mm * y_direction) - radius_y
            }
            Self::Profile {
                z_mm,
                height_mm,
                segments,
            } => profile_lower_y(
                *z_mm,
                *height_mm,
                segments,
                &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            ),
        }
    }
    pub(crate) fn valid(&self) -> bool {
        match self {
            Self::Box { corner_mm, size_mm } => {
                vector(corner_mm) && size_mm.iter().all(|v| v.is_finite() && (0.001..=1000.0).contains(v))
            }
            Self::Cylinder {
                base_mm,
                axis,
                diameter_mm,
                height_mm,
            } => {
                vector(base_mm)
                    && vector(axis)
                    && axis.iter().any(|v| *v != 0.0)
                    && diameter_mm.is_finite()
                    && (0.001..=1000.0).contains(diameter_mm)
                    && height_mm.is_finite()
                    && (0.001..=1000.0).contains(height_mm)
            }
            Self::Profile {
                z_mm,
                height_mm,
                segments,
            } => profile_valid(*z_mm, *height_mm, segments),
        }
    }
    fn command(&self) -> &'static str {
        match self {
            Self::Box { .. } => "solid.box",
            Self::Cylinder { .. } => "solid.cylinder",
            Self::Profile { .. } => "solid.extrude",
        }
    }
    fn params(&self, offset: [f64; 3]) -> Value {
        let [dx, dy, dz] = offset;
        match self {
            Self::Box {
                corner_mm: [x, y, z],
                size_mm: [a, b, c],
            } => {
                json!({"corner":[x+dx,y+dy,z+dz],"length":format!("{a} mm"),"width":format!("{b} mm"),"height":format!("{c} mm")})
            }
            Self::Cylinder {
                base_mm: [x, y, z],
                axis,
                diameter_mm,
                height_mm,
            } => {
                json!({"base":[x+dx,y+dy,z+dz],"axis":axis,"diameter":format!("{diameter_mm} mm"),"height":format!("{height_mm} mm")})
            }
            Self::Profile { height_mm, .. } => json!({"distance":format!("{height_mm} mm"),"direction":"positive"}),
        }
    }
}

pub(crate) fn validate(asset: &Asset) -> Result<(), Error> {
    let Some(model) = &asset.reference_model else {
        return Ok(());
    };
    if !valid_id(&model.frame)
        || !asset.mount_groups.iter().any(|group| group.frame == model.frame)
            && !asset.representations.values().any(|view| view.frame == model.frame)
            && !asset
                .output_interfaces
                .iter()
                .any(|output| output.axis.as_ref().is_some_and(|axis| axis.frame == model.frame))
        || model.parts.is_empty()
        || model.parts.len() > 64
        || model.notes.len() > 16
        || model.front_labels.len() > 16
        || model.cables.len() > 16
        || model.assumptions.is_empty()
    {
        return Err(invalid(
            "catalog.reference_model",
            "a model needs a supported frame, parts and explicit assumptions",
        ));
    }
    let mut ids = BTreeSet::new();
    for part in &model.parts {
        if !valid_id(&part.id)
            || !ids.insert(part.id.clone())
            || !part.geometry.valid()
            || !color(&part.color)
            || part.basis.is_empty()
            || part.note.is_empty()
            || part.modifiers.len() > 64
        {
            return Err(invalid(
                "catalog.reference_model",
                "invalid part, appearance or evidence classification",
            ));
        }
        for modifier in &part.modifiers {
            if !matches!(modifier.operation.as_str(), "cut" | "join") || !modifier.geometry.valid() {
                return Err(invalid("catalog.reference_model", "invalid part modifier"));
            }
        }
    }
    for note in &model.notes {
        if !vector(&note.anchor_mm) || note.text.is_empty() || note.text.len() > 512 {
            return Err(invalid("catalog.reference_model", "invalid note"));
        }
    }
    for label in &model.front_labels {
        if !label
            .at_mm
            .iter()
            .chain(label.anchor_mm.iter())
            .all(|v| v.is_finite() && v.abs() <= 10000.0)
            || label.text.is_empty()
            || label.text.len() > 512
            || label
                .cable_port
                .as_ref()
                .is_some_and(|port| !model.cables.iter().any(|cable| &cable.port_id == port))
        {
            return Err(invalid("catalog.reference_model", "invalid front-view label"));
        }
    }
    let mut cable_ids = BTreeSet::new();
    for cable in &model.cables {
        if !cable_ids.insert(&cable.port_id) {
            return Err(invalid(
                "catalog.reference_model",
                "cable model port IDs must be unique",
            ));
        }
        let port = asset
            .cable_ports
            .iter()
            .find(|p| p.id == cable.port_id)
            .ok_or_else(|| invalid("catalog.reference_model", "unknown cable port"))?;
        let count = port.conductors_per_bundle.as_ref().map(|c| usize::from(c.value));
        if !vector(&cable.face_base_mm)
            || cable.conductor_colors.is_empty()
            || cable.conductor_colors.len() > 16
            || Some(cable.conductor_colors.len()) != count
            || port.bundle_count.as_ref().is_none_or(|count| count.value != 1)
            || !cable.conductor_colors.iter().all(|c| color(c))
            || !color(&cable.connector_color)
            || !cable
                .connector_size_mm
                .iter()
                .all(|v| v.is_finite() && (0.1..=100.0).contains(v))
        {
            return Err(invalid(
                "catalog.reference_model",
                "cable colors/count/geometry must match its sourced conductor count",
            ));
        }
        for role in (1..=cable.conductor_colors.len())
            .map(|index| format!("{}-wire-{index}", cable.port_id))
            .chain(std::iter::once(format!("{}-connector", cable.port_id)))
        {
            if !ids.insert(role) {
                return Err(invalid(
                    "catalog.reference_model",
                    "generated cable roles must not collide with model parts",
                ));
            }
        }
        cable_values(
            cable,
            &ModelParams {
                asset_id: asset.id.clone(),
                horn_id: None,
                horn_angle_deg: 0.0,
                horn_axial_offset_mm: 0.0,
                x_mm: 0.0,
                y_mm: 0.0,
                z_mm: 0.0,
                include_cable: true,
                include_connector: false,
                outlet_height_mm: None,
                wire_diameter_mm: None,
                wire_pitch_mm: None,
                wire_length_mm: None,
                labels: Labels::None,
            },
        )?;
    }
    Ok(())
}

fn cable_values(cable: &CableModel, p: &ModelParams) -> Result<[f64; 4], Error> {
    let values = [
        p.outlet_height_mm.unwrap_or(cable.default_outlet_height_mm),
        p.wire_diameter_mm.unwrap_or(cable.default_wire_diameter_mm),
        p.wire_pitch_mm.unwrap_or(cable.default_wire_pitch_mm),
        p.wire_length_mm.unwrap_or(cable.default_wire_length_mm),
    ];
    let [height, diameter, pitch, length] = values;
    for (name, value, low, high) in [
        ("outlet_height_mm", height, 0.1, 20.0),
        ("wire_diameter_mm", diameter, 0.1, 5.0),
        ("wire_pitch_mm", pitch, 0.1, 10.0),
        ("wire_length_mm", length, 2.0, 250.0),
    ] {
        if !value.is_finite() || !(low..=high).contains(&value) {
            return Err(invalid(name, format!("must be finite and between {low} and {high} mm")));
        }
    }
    if pitch < diameter {
        return Err(invalid(
            "wire_pitch_mm",
            "parallel wires need pitch at least their diameter",
        ));
    }
    Ok(values)
}

fn add_geometry(
    steps: &mut Vec<Value>,
    geometry: &Primitive,
    role: &str,
    label: &str,
    operation: &str,
    offset: [f64; 3],
) -> Result<(), Error> {
    if !geometry.valid() || !matches!(operation, "new" | "cut" | "join") {
        return Err(invalid(
            "model.geometry",
            "use a bounded primitive or closed nondegenerate line/arc profile",
        ));
    }
    let profile = matches!(geometry, Primitive::Profile { .. });
    if let Primitive::Profile { z_mm, segments, .. } = geometry {
        let [dx, dy, dz] = offset;
        steps.push(
            json!({"command":"sketch.create","params":{"name":format!("{label} profile"),"frame":"world",
            "plane":{"origin":[dx,dy,z_mm+dz],"x_dir":[1,0,0],"y_dir":[0,1,0]}}}),
        );
        for segment in segments {
            match segment {
                ProfileSegment::Line { start_mm, end_mm } => steps.push(json!({"command":"sketch.line",
                    "params":{"points":[start_mm,end_mm],"closed":false,"infer":false,"connect":true}})),
                ProfileSegment::Arc {
                    center_mm,
                    start_mm,
                    end_mm,
                    ccw,
                } => steps.push(json!({"command":"sketch.arc.center_point",
                    "params":{"center":center_mm,"start":start_mm,"end":end_mm,"clockwise":!ccw,"connect":true}})),
            }
        }
        steps.push(json!({"command":"sketch.finish","params":{}}));
    }
    let mut args = geometry.params(offset);
    let object = args
        .as_object_mut()
        .ok_or_else(|| invalid("recipe", "expected primitive parameters"))?;
    object.insert("operation".into(), json!(operation));
    if operation == "new" {
        object.insert("name".into(), json!(label));
        object.insert("body_name".into(), json!(label));
    }
    let mut step = json!({"command":geometry.command(),"params":args});
    let object = step
        .as_object_mut()
        .ok_or_else(|| invalid("recipe", "expected primitive step"))?;
    if operation == "new" {
        object.insert("bind_role".into(), json!(role));
    } else {
        object.insert("target_roles".into(), json!([role]));
    }
    if profile {
        object.insert("use_last_sketch".into(), json!(true));
    }
    steps.push(step);
    Ok(())
}

pub(crate) fn add_part(steps: &mut Vec<Value>, part: &Part, model_name: &str, offset: [f64; 3]) -> Result<(), Error> {
    let label = format!("{model_name} {}", part.label);
    add_geometry(steps, &part.geometry, &part.id, &label, "new", offset)?;
    for modifier in &part.modifiers {
        add_geometry(steps, &modifier.geometry, &part.id, &label, &modifier.operation, offset)?;
    }
    steps.push(
        json!({"command":"appearance.assign","params":{"color":part.color,"opacity":1.0},"target_roles":[part.id]}),
    );
    Ok(())
}

pub fn model_recipe(catalog: &Catalog, p: &ModelParams) -> Result<Value, Error> {
    let selected = crate::horns::selection(catalog.asset(&p.asset_id)?, p)?;
    model_plan(catalog, p, selected.as_ref())
}

fn model_plan(
    catalog: &Catalog,
    p: &ModelParams,
    selected: Option<&crate::horns::SelectedHorn>,
) -> Result<Value, Error> {
    catalog.validate()?;
    validate_placement(&InsertParams {
        asset_id: p.asset_id.clone(),
        representation: "servo-reference".into(),
        mount_group_id: None,
        clearance_mm: 0.0,
        x_mm: p.x_mm,
        y_mm: p.y_mm,
        z_mm: p.z_mm,
    })?;
    let asset = catalog.asset(&p.asset_id)?;
    let model = asset
        .reference_model
        .as_ref()
        .ok_or_else(|| Error::Unsupported("this source profile has no integrated reference model".into()))?;
    let offset = [p.x_mm, p.y_mm, p.z_mm];
    let mut steps = Vec::new();
    let name = format!("{} reference", asset.model);
    for part in &model.parts {
        if selected.is_some_and(|selected| selected.mount.replace_model_parts.contains(&part.id)) {
            continue;
        }
        add_part(&mut steps, part, &name, offset)?;
    }
    for cable in &model.cables {
        cable_values(cable, p)?;
    }
    let mut outlets = Vec::new();
    if p.include_cable {
        for cable in &model.cables {
            let [height, diameter, pitch, length] = cable_values(cable, p)?;
            let [x, y, z] = cable.face_base_mm;
            let z = z + height;
            outlets.push(([x, y, z], cable.port_id.clone(), cable.conductor_colors.len()));
            let count = cable.conductor_colors.len();
            for (index, color) in cable.conductor_colors.iter().enumerate() {
                let y = y + (index as f64 - (count as f64 - 1.0) / 2.0) * pitch;
                let part = Part {
                    id: format!("{}-wire-{}", cable.port_id, index + 1),
                    label: format!("APPROX {} wire {}", cable.port_id, index + 1),
                    basis: "planning".into(),
                    note: "Editable displayed lead segment; not measured outlet geometry or full lead length.".into(),
                    color: color.clone(),
                    geometry: Primitive::Cylinder {
                        base_mm: [x, y, z],
                        axis: [-1.0, 0.0, 0.0],
                        diameter_mm: diameter,
                        height_mm: length,
                    },
                    modifiers: vec![],
                };
                add_part(&mut steps, &part, &name, offset)?;
            }
            if p.include_connector {
                let [l, w, h] = cable.connector_size_mm;
                add_part(
                    &mut steps,
                    &Part {
                        id: format!("{}-connector", cable.port_id),
                        label: format!("APPROX {} connector", cable.port_id),
                        basis: "planning".into(),
                        note: "Optional illustrative connector envelope and position.".into(),
                        color: cable.connector_color.clone(),
                        geometry: Primitive::Box {
                            corner_mm: [x - length - l, y - w / 2.0, z - h / 2.0],
                            size_mm: [l, w, h],
                        },
                        modifiers: vec![],
                    },
                    &name,
                    offset,
                )?;
            }
        }
    }
    for output in &asset.output_interfaces {
        if let Some(axis) = &output.axis
            && axis.frame == model.frame
        {
            let [x, y, z] = axis.origin_mm;
            steps.push(json!({"command":"construct.axis.two_points","params":{"a":{"point":[x+p.x_mm,y+p.y_mm,z+p.z_mm]},"b":{"point":[x+p.x_mm+axis.direction[0],y+p.y_mm+axis.direction[1],z+p.z_mm+axis.direction[2]]},"name":format!("{} OUTPUT AXIS {}",asset.model,output.id)}}));
        }
    }
    if let Some(selected) = selected {
        selected.append_recipe(&mut steps, p)?;
    }
    match p.labels {
        Labels::Notes => {
            for note in &model.notes {
                if selected.is_some() && note.text.to_ascii_lowercase().contains("horn") {
                    continue;
                }
                let [x, y, z] = note.anchor_mm;
                steps.push(json!({"command":"note.add","params":{"anchor":[x+p.x_mm,y+p.y_mm,z+p.z_mm],"text":note.text,"hidden":false}}));
            }
            for ([x, y, z], port, count) in &outlets {
                steps.push(json!({"command":"note.add","params":{"anchor":[x+p.x_mm,y+p.y_mm,z+p.z_mm],"text":format!("CABLE OUTLET {port}: {count} conductors\nAPPROX position, wire sizes and routing; editable."),"hidden":false}}));
            }
            if let Some(selected) = selected {
                let matrix = selected.matrix(p);
                steps.push(json!({"command":"note.add","params":{"anchor":[matrix[3],matrix[7],matrix[11]],"text":selected.warning(),"hidden":false}}));
            }
        }
        Labels::Front => {
            front_labels(
                &mut steps,
                p,
                model,
                &outlets,
                selected.map(|horn| horn.lower_y(p) - p.y_mm),
                selected.is_some(),
            );
            if let Some(selected) = selected {
                let matrix = selected.matrix(p);
                let x = matrix[3] - p.x_mm;
                let z = matrix[11] - p.z_mm;
                let lower_y = selected.lower_y(p).min(
                    model
                        .parts
                        .iter()
                        .map(|part| part.geometry.lower_y() + p.y_mm)
                        .fold(f64::INFINITY, f64::min),
                ) - 3.0;
                steps.push(json!({"command":"sketch.create","params":{"name":"Horn reference labels","frame":"world","plane":{"origin":[p.x_mm,lower_y,p.z_mm],"x_dir":[1,0,0],"y_dir":[0,0,1]}}}));
                steps.push(json!({"command":"sketch.line","params":{"points":[[x,z],[x+8.0,z+6.0]],"construction":true,"connect":false}}));
                steps.push(json!({"command":"sketch.text","params":{"at":[x+9.0,z+6.0],"text":format!("HORN {} / {}\nFIT {} / POSE {}",selected.horn.model,selected.horn.status.to_uppercase(),selected.mount.compatibility.to_uppercase(),selected.mount.placement_basis.to_uppercase()),"height":1.5,"angle":0}}));
                steps.push(json!({"command":"sketch.finish","params":{}}));
            }
        }
        Labels::None => {}
    }
    Ok(Value::Array(steps))
}

fn front_labels(
    steps: &mut Vec<Value>,
    p: &ModelParams,
    model: &ReferenceModel,
    outlets: &[([f64; 3], String, usize)],
    extra_lower_y: Option<f64>,
    hide_base_horn_labels: bool,
) {
    let lower_y = model
        .parts
        .iter()
        .flat_map(|part| {
            std::iter::once(&part.geometry).chain(
                part.modifiers
                    .iter()
                    .filter(|modifier| modifier.operation == "join")
                    .map(|modifier| &modifier.geometry),
            )
        })
        .map(Primitive::lower_y)
        .fold(extra_lower_y.unwrap_or(f64::INFINITY), f64::min);
    steps.push(json!({"command":"sketch.create","params":{"name":"Reference labels - front view","frame":"world","plane":{"origin":[p.x_mm,p.y_mm+lower_y-2.0,p.z_mm],"x_dir":[1,0,0],"y_dir":[0,0,1]}}}));
    for label in &model.front_labels {
        if hide_base_horn_labels && label.text.to_ascii_lowercase().contains("horn") {
            continue;
        }
        let anchor = if let Some(port) = &label.cable_port {
            let Some(([x, _, z], _, _)) = outlets.iter().find(|(_, id, _)| id == port) else {
                continue;
            };
            [*x, *z]
        } else {
            label.anchor_mm
        };
        let at = label.at_mm;
        let text = &label.text;
        let [x, y] = at;
        let points = [anchor, [x - 3.0, y]];
        steps.push(json!({"command":"sketch.line","params":{"points":points,"construction":true,"connect":false}}));
        steps.push(json!({"command":"sketch.text","params":{"at":at,"text":text,"height":1.5,"angle":0}}));
    }
    steps.push(json!({"command":"sketch.finish","params":{}}));
}

pub fn insert_model(runner: &mut impl Runner, catalog: &Catalog, p: &ModelParams) -> Result<Value, Error> {
    let selected = crate::horns::selection(catalog.asset(&p.asset_id)?, p)?;
    let plan = model_plan(catalog, p, selected.as_ref())?;
    let mut parameters = serde_json::to_value(p).map_err(|e| invalid("params", e))?;
    if let Some(selected) = &selected
        && let Some(object) = parameters.as_object_mut()
    {
        object.insert("horn_id".into(), json!(selected.horn.id));
    }
    let base_warning = "Integrated reference mixes source dimensions with explicit visual planning assumptions. Read this profile's assumptions for unverified mounting and shaft positions. Cable outlets are short approximate markers by default; optional lead/connector geometry is planning space. This is not a verified drilling template or manufacturing model.";
    let warning = selected.as_ref().map_or_else(
        || base_warning.to_owned(),
        |selected| format!("{} {base_warning}", selected.warning()),
    );
    let mut reply = execute_plan_context(
        runner,
        catalog,
        &p.asset_id,
        "servo-reference",
        PlanContext {
            parameters,
            accessories: selected.iter().map(|selected| selected.snapshot(p)).collect(),
        },
        plan,
        &warning,
    )?;
    if let Some(object) = reply.as_object_mut() {
        object.insert(
            "assumptions".into(),
            json!(
                catalog
                    .asset(&p.asset_id)?
                    .reference_model
                    .as_ref()
                    .map(|m| &m.assumptions)
            ),
        );
    }
    Ok(reply)
}
