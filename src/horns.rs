//! Independent horn identities, fit claims and assembly poses.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::catalog::{
    Asset, Catalog, Dimension, Evidence, LookupQuery, MountHole, Observation, Source, selector_key, valid_id,
};
use crate::models::{ModelParams, Part, Primitive};
use crate::{Error, invalid};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Horn {
    pub id: String,
    pub manufacturer: String,
    pub model: String,
    pub data_revision: u32,
    pub shape: String,
    pub status: String,
    pub dimensions: BTreeMap<String, Dimension>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub estimates: BTreeMap<String, GeometryEstimate>,
    pub holes: Vec<MountHole>,
    pub parts: Vec<Part>,
    pub assumptions: Vec<String>,
    pub observations: Vec<Observation>,
    pub unknown: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeometryEstimate {
    pub value: f64,
    pub unit: String,
    pub evidence: Evidence,
    #[serde(default)]
    pub source_pixel_points: Vec<[f64; 2]>,
    pub uncertainty_mm: Option<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HornMount {
    pub asset_id: String,
    pub horn_id: String,
    pub output_id: String,
    pub frame: String,
    pub compatibility: String,
    pub compatibility_evidence: Option<Evidence>,
    pub origin_mm: Option<[f64; 3]>,
    pub direction: [f64; 3],
    pub placement_basis: String,
    pub placement_evidence: Option<Evidence>,
    #[serde(default)]
    pub replace_model_parts: Vec<String>,
    pub note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HornCatalog {
    pub schema_version: u32,
    pub release: String,
    pub sources: Vec<Source>,
    pub horns: Vec<Horn>,
    pub mounts: Vec<HornMount>,
}

fn vector(values: &[f64; 3]) -> bool {
    values.iter().all(|value| value.is_finite() && value.abs() <= 10000.0)
}

fn text_ok(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 2048 && !value.contains('\0')
}

fn evidence_ok(evidence: &Evidence, sources: &BTreeSet<&str>) -> bool {
    sources.contains(evidence.source.as_str())
        && matches!(
            evidence.method.as_str(),
            "drawing"
                | "specification"
                | "measurement"
                | "photo_observation"
                | "photo_annotation"
                | "photo_trace"
                | "photo_inference"
                | "derived"
        )
        && text_ok(&evidence.locator)
}

fn dimension_ok(dimension: &Dimension, sources: &BTreeSet<&str>) -> bool {
    dimension.nominal_mm.is_finite()
        && (0.0..=1000.0).contains(&dimension.nominal_mm)
        && dimension
            .tolerance_mm
            .is_none_or(|value| value.is_finite() && (0.0..=100.0).contains(&value))
        && evidence_ok(&dimension.evidence, sources)
        && matches!(
            dimension.evidence.method.as_str(),
            "drawing" | "specification" | "measurement" | "photo_annotation"
        )
}

impl HornCatalog {
    pub fn embedded() -> Result<Self, Error> {
        Self::merge_packs(
            "2026-11-19.10",
            &[
                include_str!("../catalog/horns-micro.json"),
                include_str!("../catalog/horns-robotis-430.json"),
                include_str!("../catalog/horns-robotis-540-sts.json"),
                include_str!("../catalog/horns-user-reference.json"),
            ],
        )
    }

    pub fn parse(text: &str) -> Result<Self, Error> {
        if text.len() > 512 * 1024 {
            return Err(invalid("horn_catalog", "use at most 512 KiB per horn pack"));
        }
        let catalog: Self = serde_json::from_str(text).map_err(|error| invalid("horn_catalog", error))?;
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn merge_packs(release: &str, packs: &[&str]) -> Result<Self, Error> {
        if packs.is_empty()
            || packs.len() > 16
            || !text_ok(release)
            || packs
                .iter()
                .try_fold(0_usize, |total, pack| total.checked_add(pack.len()))
                .is_none_or(|size| size > 2 * 1024 * 1024)
        {
            return Err(invalid("horn_catalog", "use 1 to 16 packs and at most 2 MiB total"));
        }
        let mut catalog = Self {
            schema_version: 1,
            release: release.into(),
            sources: vec![],
            horns: vec![],
            mounts: vec![],
        };
        for text in packs {
            let pack = Self::parse(text)?;
            catalog.sources.extend(pack.sources);
            catalog.horns.extend(pack.horns);
            catalog.mounts.extend(pack.mounts);
        }
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.schema_version != 1 || self.horns.len() > 128 || self.sources.len() > 256 || self.mounts.len() > 1024 {
            return Err(invalid("horn_catalog", "unsupported schema or excessive horn records"));
        }
        let mut sources = BTreeSet::new();
        for source in &self.sources {
            if !valid_id(&source.id) || !sources.insert(source.id.as_str()) || !source.url.starts_with("https://") {
                return Err(invalid("horn_catalog.sources", "use unique source IDs and HTTPS URLs"));
            }
        }
        let mut ids = BTreeSet::new();
        let mut selectors = BTreeSet::new();
        for horn in &self.horns {
            if !valid_id(&horn.id)
                || horn.id.starts_with("ref-")
                || horn.id == "none"
                || !ids.insert(horn.id.as_str())
                || !selectors.insert(selector_key(&horn.id))
                || horn.data_revision == 0
                || !valid_id(&horn.shape)
                || !matches!(
                    horn.status.as_str(),
                    "partial_dimensions" | "dimensions_only" | "photo_estimate"
                )
                || !text_ok(&horn.manufacturer)
                || !text_ok(&horn.model)
                || (horn.status == "dimensions_only") != horn.parts.is_empty()
                || horn.parts.len() > 8
                || horn.holes.len() > 128
                || horn.dimensions.len() > 128
                || horn.dimensions.iter().any(|(key, dimension)| {
                    dimension.evidence.method == "photo_annotation"
                        && (!matches!(horn.status.as_str(), "dimensions_only" | "photo_estimate")
                            || !key.starts_with("reported_"))
                })
                || (horn.status == "photo_estimate") != !horn.estimates.is_empty()
                || horn.estimates.len() > 128
                || horn.estimates.iter().any(|(key, estimate)| {
                    !valid_id(key)
                        || !estimate.value.is_finite()
                        || estimate.value.abs() > 10000.0
                        || !matches!(estimate.unit.as_str(), "mm" | "deg" | "count" | "ratio")
                        || estimate.unit == "count"
                            && (estimate.value < 1.0 || estimate.value > 128.0 || estimate.value.fract() != 0.0)
                        || !evidence_ok(&estimate.evidence, &sources)
                        || !matches!(estimate.evidence.method.as_str(), "photo_trace" | "photo_inference")
                        || !text_ok(&estimate.evidence.note)
                        || estimate.source_pixel_points.len() > 64
                        || estimate
                            .source_pixel_points
                            .iter()
                            .flatten()
                            .any(|value| !value.is_finite() || !(0.0..=100000.0).contains(value))
                        || estimate
                            .uncertainty_mm
                            .is_some_and(|value| !value.is_finite() || !(0.0..=100.0).contains(&value))
                })
                || horn.assumptions.is_empty()
                || horn.assumptions.len() > 64
                || horn.unknown.len() > 64
                || horn.observations.len() > 64
                || !horn.assumptions.iter().chain(&horn.unknown).all(|value| text_ok(value))
                || horn
                    .observations
                    .iter()
                    .any(|observation| !sources.contains(observation.source.as_str()) || !text_ok(&observation.note))
            {
                return Err(invalid(
                    "horn_catalog.horns",
                    "horns need bounded geometry, unique identity and explicit uncertainty",
                ));
            }
            let mut parts = BTreeSet::new();
            for part in &horn.parts {
                if !valid_id(&part.id)
                    || part.id.len() > 80
                    || !parts.insert(part.id.as_str())
                    || !part.geometry.valid()
                    || part.modifiers.len() > 64
                    || !text_ok(&part.basis)
                    || horn.status == "photo_estimate" && part.basis != "photo_estimate"
                    || !text_ok(&part.label)
                    || !text_ok(&part.note)
                    || !crate::models::color(&part.color)
                    || part.modifiers.iter().any(|modifier| {
                        !matches!(modifier.operation.as_str(), "cut" | "join") || !modifier.geometry.valid()
                    })
                {
                    return Err(invalid(
                        "horn_catalog.parts",
                        "invalid horn primitive, modifier, label or appearance",
                    ));
                }
            }
            let mut holes = BTreeSet::new();
            for hole in &horn.holes {
                if !valid_id(&hole.id)
                    || !holes.insert(hole.id.as_str())
                    || !hole
                        .center_mm
                        .iter()
                        .all(|value| value.is_finite() && value.abs() <= 1000.0)
                    || hole.diameter.nominal_mm <= 0.0
                    || hole.diameter.evidence.method == "photo_annotation"
                    || !evidence_ok(&hole.evidence, &sources)
                    || hole.thread.as_ref().is_some_and(|thread| {
                        !text_ok(&thread.designation)
                            || thread
                                .pitch_mm
                                .iter()
                                .chain(thread.specified_depth_mm.iter())
                                .any(|dimension| dimension.nominal_mm <= 0.0)
                    })
                {
                    return Err(invalid("horn_catalog.holes", "invalid horn hole or thread reference"));
                }
            }
            let dimensions = horn.dimensions.values().chain(horn.holes.iter().flat_map(|hole| {
                std::iter::once(&hole.diameter).chain(
                    hole.thread
                        .iter()
                        .flat_map(|thread| thread.pitch_mm.iter().chain(thread.specified_depth_mm.iter())),
                )
            }));
            if dimensions
                .into_iter()
                .any(|dimension| !dimension_ok(dimension, &sources))
            {
                return Err(invalid(
                    "horn_catalog.dimensions",
                    "only finite source-backed dimensions belong in factual fields",
                ));
            }
        }
        let mut pairs = BTreeSet::new();
        for mount in &self.mounts {
            if !valid_id(&mount.asset_id)
                || !valid_id(&mount.output_id)
                || !valid_id(&mount.frame)
                || !ids.contains(mount.horn_id.as_str())
                || !pairs.insert((&mount.asset_id, &mount.horn_id))
                || mount.origin_mm.as_ref().is_some_and(|origin| !vector(origin))
                || !matches!(mount.direction, [0.0, 0.0, 1.0] | [0.0, 0.0, -1.0])
                || !matches!(mount.compatibility.as_str(), "documented" | "unverified")
                || !matches!(mount.placement_basis.as_str(), "documented" | "unknown")
                || mount.compatibility == "documented" && mount.compatibility_evidence.is_none()
                || mount.placement_basis == "documented"
                    && (mount.placement_evidence.is_none() || mount.origin_mm.is_none())
                || mount.placement_basis == "unknown"
                    && (mount.origin_mm.is_some()
                        || mount.placement_evidence.is_some()
                        || !mount.replace_model_parts.is_empty())
                || mount
                    .compatibility_evidence
                    .iter()
                    .chain(mount.placement_evidence.iter())
                    .any(|evidence| !evidence_ok(evidence, &sources))
                || mount.replace_model_parts.len() > 4
                || !mount
                    .replace_model_parts
                    .iter()
                    .all(|id| valid_id(id) && id != "housing")
                || !text_ok(&mount.note)
            {
                return Err(invalid(
                    "horn_catalog.mounts",
                    "fit and placement need separate explicit claims and source evidence",
                ));
            }
        }
        Ok(())
    }

    pub fn validate_for_catalog(&self, catalog: &Catalog) -> Result<(), Error> {
        for mount in &self.mounts {
            validate_mount(mount, catalog.asset(&mount.asset_id)?)?;
        }
        Ok(())
    }

    pub fn horn(&self, id: &str) -> Result<&Horn, Error> {
        self.horns
            .iter()
            .find(|horn| horn.id == id || selector_key(&horn.id) == id)
            .ok_or_else(|| invalid("horn_id", format!("unknown horn `{id}`")))
    }

    pub fn for_asset(&self, asset_id: &str) -> Vec<Value> {
        self.mounts.iter().filter(|mount| mount.asset_id == asset_id).filter_map(|mount| {
            self.horn(&mount.horn_id).ok().map(|horn| json!({"id":horn.id,"model":horn.model,"shape":horn.shape,"status":horn.status,
                "dimensions":horn.dimensions,"estimates":horn.estimates,"holes":horn.holes,"assumptions":horn.assumptions,"unknown":horn.unknown,"mount":mount,
                "automatic_assembly_available":automatic_assembly_available(horn,mount)}))
        }).collect()
    }

    pub fn search(&self, query: &str) -> Vec<Value> {
        let query = LookupQuery::new(query);
        let mut matches: Vec<_> = self
            .horns
            .iter()
            .filter_map(|horn| {
                query
                    .rank(
                        &[&horn.id, &horn.model],
                        &[&horn.manufacturer, &horn.shape, &horn.status],
                    )
                    .map(|rank| (rank, horn))
            })
            .collect();
        matches.sort_by_key(|(rank, _)| *rank);
        matches.into_iter().map(|(_, horn)| horn)
            .map(|horn| json!({"id":horn.id,"model":horn.model,"manufacturer":horn.manufacturer,"shape":horn.shape,"status":horn.status,
                "candidate_assets":self.mounts.iter().filter(|mount| mount.horn_id == horn.id).map(|mount| json!({"asset_id":mount.asset_id,"compatibility":mount.compatibility,"placement_basis":mount.placement_basis,"automatic_assembly_available":automatic_assembly_available(horn,mount)})).collect::<Vec<_>>()})).collect()
    }
}

/// A separate accessory reference has a user-chosen pose, never an inferred servo seat.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HornParams {
    pub horn_id: String,
    #[serde(default = "dimension_mode")]
    pub mode: String,
    #[serde(default)]
    pub x_mm: f64,
    #[serde(default)]
    pub y_mm: f64,
    #[serde(default)]
    pub z_mm: f64,
    #[serde(default)]
    pub angle_deg: f64,
}

fn dimension_mode() -> String {
    "dimensions".into()
}

fn reference_horn<'a>(catalog: &'a HornCatalog, params: &HornParams) -> Result<&'a Horn, Error> {
    if ![params.x_mm, params.y_mm, params.z_mm]
        .iter()
        .all(|value| value.is_finite() && value.abs() <= 100000.0)
        || !params.angle_deg.is_finite()
        || !(-360.0..=360.0).contains(&params.angle_deg)
        || !matches!(params.mode.as_str(), "dimensions" | "source-solid" | "photo-solid")
    {
        return Err(invalid(
            "horn_reference",
            "use a known mode, finite origins within +/-100000 mm and an angle within +/-360 degrees",
        ));
    }
    let horn = catalog.horn(&params.horn_id)?;
    if params.mode == "source-solid" && horn.parts.is_empty() {
        return Err(invalid(
            "mode",
            "this source does not fully define a solid contour; use dimensions to insert only supported references",
        ));
    }
    if params.mode == "source-solid" && horn.status == "photo_estimate" {
        return Err(invalid(
            "mode",
            "this shape includes photo inference; use photo-solid to preserve its estimated status",
        ));
    }
    if params.mode == "photo-solid" && horn.status != "photo_estimate" {
        return Err(invalid("mode", "photo-solid requires a photo-estimated horn variant"));
    }
    Ok(horn)
}

pub fn horn_recipe(catalog: &HornCatalog, params: &HornParams) -> Result<Value, Error> {
    catalog.validate()?;
    let horn = reference_horn(catalog, params)?;
    let (sin, cos) = params.angle_deg.to_radians().sin_cos();
    let matrix = [
        cos,
        -sin,
        0.0,
        params.x_mm,
        sin,
        cos,
        0.0,
        params.y_mm,
        0.0,
        0.0,
        1.0,
        params.z_mm,
    ];
    let mut steps = Vec::new();
    if matches!(params.mode.as_str(), "source-solid" | "photo-solid") {
        let mut roles = Vec::new();
        for part in &horn.parts {
            let mut part = part.clone();
            part.id = format!("horn-{}", part.id);
            roles.push(part.id.clone());
            let prefix = if horn.status == "photo_estimate" {
                "HORN PHOTO ESTIMATE"
            } else {
                "HORN PARTIAL"
            };
            crate::models::add_part(&mut steps, &part, &format!("{prefix} / {}", horn.model), [0.0; 3])?;
        }
        steps.push(json!({"command":"solid.move","params":{"matrix":matrix,"copy":false,"name":"Separate horn reference placement"},"target_roles":roles}));
    }
    let plane = json!({"origin":[params.x_mm,params.y_mm,params.z_mm],"x_dir":[cos,sin,0.0],"y_dir":[-sin,cos,0.0]});
    let outer_diameter = ["outer_diameter", "disc_outer_diameter", "flange_outer_diameter"]
        .iter()
        .find_map(|key| horn.dimensions.get(*key));
    if params.mode == "dimensions" && (!horn.holes.is_empty() || outer_diameter.is_some()) {
        steps.push(json!({"command":"sketch.create","params":{"name":format!("{} / hole references",horn.model),"frame":"world","plane":plane}}));
        // A published outside diameter is a circle reference, not an invented arm outline.
        if let Some(dimension) = outer_diameter {
            steps.push(json!({"command":"sketch.circle.center","params":{"id":"rim","center":[0,0],"diameter":format!("{} mm",dimension.nominal_mm),"connect":false}}));
            steps.push(json!({"command":"sketch.constraint.fix","params":{"entities":["rim"],"fixed":true}}));
        }
        for (index, hole) in horn.holes.iter().enumerate() {
            let id = format!("hole{}", index + 1);
            steps.push(json!({"command":"sketch.circle.center","params":{"id":id,"center":hole.center_mm,"diameter":format!("{} mm",hole.diameter.nominal_mm),"connect":false}}));
            steps.push(json!({"command":"sketch.constraint.fix","params":{"entities":[id],"fixed":true}}));
        }
        steps.push(json!({"command":"sketch.finish","params":{}}));
    }
    steps.push(json!({"command":"sketch.create","params":{"name":format!("{} / source dimensions",horn.model),"frame":"world","plane":plane}}));
    let heading = if params.mode == "dimensions" {
        "DIMENSION REFERENCE / NO SOLID"
    } else if params.mode == "photo-solid" {
        "PHOTO RECONSTRUCTION / ESTIMATED CONTOUR AND HOLES"
    } else {
        "PARTIAL OUTER SOLID / READ OMISSIONS"
    };
    steps.push(json!({"command":"sketch.text","params":{"at":[-20,23],"text":horn.model,"height":1.3,"angle":0}}));
    steps.push(json!({"command":"sketch.text","params":{"at":[-20,20],"text":heading,"height":0.9,"angle":0}}));
    steps.push(json!({"command":"sketch.text","params":{"at":[-20,17],"text":"Separate reference / installed seat not inferred","height":0.9,"angle":0}}));
    // The complete fact table remains in the source snapshot; keep viewport text bounded.
    let keys = [
        "overall_length",
        "outer_diameter",
        "disc_outer_diameter",
        "flange_outer_diameter",
        "plate_thickness",
        "total_thickness",
        "overall_height",
        "reported_length",
        "reported_plate_span",
        "reported_axial_span",
        "reported_hub_span",
    ];
    let mut row = 0;
    for key in keys {
        if let Some(dimension) = horn.dimensions.get(key) {
            steps.push(json!({"command":"sketch.text","params":{"at":[-20,-20.0-2.2*f64::from(row)],"text":format!("{} = {} mm",key.replace('_'," "),dimension.nominal_mm),"height":0.9,"angle":0}}));
            row += 1;
        }
    }
    if horn.holes.is_empty() && params.mode == "dimensions" {
        steps.push(json!({"command":"sketch.text","params":{"at":[-20,0],"text":"Hole centers / complete contour unknown","height":0.9,"angle":0}}));
    }
    steps.push(json!({"command":"sketch.finish","params":{}}));
    Ok(json!(steps))
}

pub fn insert_horn(
    runner: &mut impl crate::Runner,
    catalog: &HornCatalog,
    params: &HornParams,
) -> Result<Value, Error> {
    let plan = horn_recipe(catalog, params)?;
    let horn = reference_horn(catalog, params)?;
    let mut parameters = serde_json::to_value(params).map_err(|error| invalid("params", error))?;
    if let Some(object) = parameters.as_object_mut() {
        object.insert("horn_id".into(), json!(horn.id));
    }
    let mut ids = BTreeSet::new();
    for dimension in horn.dimensions.values() {
        ids.insert(dimension.evidence.source.as_str());
    }
    for estimate in horn.estimates.values() {
        ids.insert(estimate.evidence.source.as_str());
    }
    for hole in &horn.holes {
        ids.insert(hole.evidence.source.as_str());
        ids.insert(hole.diameter.evidence.source.as_str());
        if let Some(thread) = &hole.thread {
            for dimension in thread.pitch_mm.iter().chain(thread.specified_depth_mm.iter()) {
                ids.insert(dimension.evidence.source.as_str());
            }
        }
    }
    for observation in &horn.observations {
        ids.insert(observation.source.as_str());
    }
    let sources: Vec<_> = catalog
        .sources
        .iter()
        .filter(|source| ids.contains(source.id.as_str()))
        .collect();
    let warning = if horn.status == "photo_estimate" {
        "PHOTO ESTIMATE: printed annotation dimensions constrain the shape; arm contour, hole centers and cavity interpretation are inferred from the photograph and recorded separately. This detached reference does not establish a mating spline or installed servo height."
    } else {
        "Separate source reference; no servo compatibility or installed seat is inferred. Dimension mode creates only known reference circles and annotations. Partial solids omit the explicitly listed features and do not define a spline fit or a complete interference envelope."
    };
    crate::insert::execute_asset_plan(
        runner,
        crate::insert::PlanAsset {
            id: &horn.id,
            data_revision: horn.data_revision,
            release: &catalog.release,
            kind: "servo_horn",
            record: json!(horn),
            sources: json!(sources),
        },
        "horn-reference",
        crate::insert::PlanContext {
            parameters,
            accessories: vec![],
        },
        plan,
        warning,
    )
}

pub fn automatic_assembly_available(horn: &Horn, mount: &HornMount) -> bool {
    horn.status != "photo_estimate"
        && !horn.parts.is_empty()
        && mount.compatibility == "documented"
        && mount.placement_basis == "documented"
        && mount.origin_mm.is_some()
}

fn validate_mount(mount: &HornMount, asset: &Asset) -> Result<(), Error> {
    let model = asset
        .reference_model
        .as_ref()
        .ok_or_else(|| invalid("horn_id", "horn placement needs an integrated actuator model"))?;
    let output = asset
        .output_interfaces
        .iter()
        .find(|output| output.id == mount.output_id)
        .ok_or_else(|| invalid("horn_id", "unknown actuator output interface"))?;
    if mount.frame != model.frame
        || mount
            .replace_model_parts
            .iter()
            .any(|id| !model.parts.iter().any(|part| &part.id == id))
    {
        return Err(invalid(
            "horn_id",
            "horn placement must use this model's frame and known replaced parts",
        ));
    }
    if mount.placement_basis == "documented"
        && output.axis.as_ref().is_none_or(|axis| {
            axis.frame != mount.frame
                || axis.direction != mount.direction
                || mount.origin_mm.is_none_or(|origin| {
                    (axis.origin_mm[0] - origin[0]).abs() > 1e-8 || (axis.origin_mm[1] - origin[1]).abs() > 1e-8
                })
        })
    {
        return Err(invalid(
            "horn_id",
            "a documented horn pose needs the matching sourced output centerline",
        ));
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub(crate) struct SelectedHorn {
    pub horn: Horn,
    pub mount: HornMount,
    pub sources: Vec<Source>,
    pub release: String,
    pub origin_mm: [f64; 3],
}

pub(crate) fn selection(asset: &Asset, params: &ModelParams) -> Result<Option<SelectedHorn>, Error> {
    if !params.horn_angle_deg.is_finite()
        || !(-360.0..=360.0).contains(&params.horn_angle_deg)
        || !params.horn_axial_offset_mm.is_finite()
        || !(-100.0..=100.0).contains(&params.horn_axial_offset_mm)
    {
        return Err(invalid(
            "horn",
            "use a finite angle within +/-360 degrees and axial adjustment within +/-100 mm",
        ));
    }
    let Some(id) = params.horn_id.as_deref().filter(|id| *id != "none") else {
        if params.horn_angle_deg != 0.0 || params.horn_axial_offset_mm != 0.0 {
            return Err(invalid("horn_id", "select a horn before adjusting its pose"));
        }
        return Ok(None);
    };
    if !valid_id(id) {
        return Err(invalid("horn_id", "use a catalog horn ID"));
    }
    let catalog = HornCatalog::embedded()?;
    let horn = catalog.horn(id)?.clone();
    let mount = catalog
        .mounts
        .iter()
        .find(|mount| mount.asset_id == asset.id && mount.horn_id == horn.id)
        .ok_or_else(|| {
            invalid(
                "horn_id",
                "this profile has no placement for that horn; inspect its available horns",
            )
        })?
        .clone();
    validate_mount(&mount, asset)?;
    if !automatic_assembly_available(&horn, &mount) {
        return Err(invalid(
            "horn_id",
            "automatic assembly needs documented compatibility, installed position and a supported solid; use horn-reference for the separate source facts",
        ));
    }
    let origin_mm = mount
        .origin_mm
        .ok_or_else(|| invalid("horn_id", "unknown installed horn origin"))?;
    let model = asset
        .reference_model
        .as_ref()
        .ok_or_else(|| invalid("horn_id", "missing actuator model"))?;
    for part in &horn.parts {
        let role = format!("horn-{}", part.id);
        if model.parts.iter().any(|part| part.id == role)
            || model.cables.iter().any(|cable| {
                role == format!("{}-connector", cable.port_id)
                    || (1..=cable.conductor_colors.len()).any(|index| role == format!("{}-wire-{index}", cable.port_id))
            })
        {
            return Err(invalid("horn_id", "horn and actuator part roles must remain distinct"));
        }
    }
    let mut source_ids = BTreeSet::new();
    for dimension in horn.dimensions.values() {
        source_ids.insert(dimension.evidence.source.as_str());
    }
    for hole in &horn.holes {
        source_ids.insert(hole.evidence.source.as_str());
        source_ids.insert(hole.diameter.evidence.source.as_str());
        if let Some(thread) = &hole.thread {
            for dimension in thread.pitch_mm.iter().chain(thread.specified_depth_mm.iter()) {
                source_ids.insert(dimension.evidence.source.as_str());
            }
        }
    }
    for observation in &horn.observations {
        source_ids.insert(observation.source.as_str());
    }
    for evidence in mount
        .compatibility_evidence
        .iter()
        .chain(mount.placement_evidence.iter())
    {
        source_ids.insert(evidence.source.as_str());
    }
    let sources = catalog
        .sources
        .iter()
        .filter(|source| source_ids.contains(source.id.as_str()))
        .cloned()
        .collect();
    Ok(Some(SelectedHorn {
        horn,
        mount,
        sources,
        release: catalog.release,
        origin_mm,
    }))
}

impl SelectedHorn {
    pub fn matrix(&self, params: &ModelParams) -> [f64; 12] {
        let (sin, cos) = params.horn_angle_deg.to_radians().sin_cos();
        let sign = self.mount.direction[2];
        let [x, y, z] = self.origin_mm;
        [
            cos,
            -sign * sin,
            0.0,
            x + params.x_mm,
            sin,
            sign * cos,
            0.0,
            y + params.y_mm,
            0.0,
            0.0,
            sign,
            z + params.z_mm + sign * params.horn_axial_offset_mm,
        ]
    }

    pub fn warning(&self) -> String {
        format!(
            "HORN {} / {}. FIT {}. PLACEMENT {}. Angular phase is user-selected, not a verified neutral or spline index. {}",
            self.horn.model,
            self.horn.status.to_uppercase(),
            self.mount.compatibility.to_uppercase(),
            self.mount.placement_basis.to_uppercase(),
            self.mount.note
        )
    }

    pub fn snapshot(&self, params: &ModelParams) -> Value {
        json!({"kind":"servo_horn","catalog_release":self.release,"horn":self.horn,"mount":self.mount,"sources":self.sources,
            "world_from_horn":self.matrix(params),"angle_deg":params.horn_angle_deg,"axial_adjustment_mm":params.horn_axial_offset_mm,
            "user_adjusted":params.horn_axial_offset_mm != 0.0 || params.horn_angle_deg != 0.0,"warning":self.warning()})
    }

    pub fn lower_y(&self, params: &ModelParams) -> f64 {
        let matrix = self.matrix(params);
        self.horn
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
            .map(|geometry| primitive_lower_y(geometry, &matrix))
            .fold(f64::INFINITY, f64::min)
    }

    pub fn append_recipe(&self, steps: &mut Vec<Value>, params: &ModelParams) -> Result<(), Error> {
        let name = format!(
            "HORN {} / FIT {} / POSE {} / {}",
            "PARTIAL",
            self.mount.compatibility.to_uppercase(),
            self.mount.placement_basis.to_uppercase(),
            self.horn.model
        );
        let mut roles = Vec::new();
        for part in &self.horn.parts {
            let mut part = part.clone();
            part.id = format!("horn-{}", part.id);
            roles.push(part.id.clone());
            crate::models::add_part(steps, &part, &name, [0.0, 0.0, 0.0])?;
        }
        steps.push(json!({"command":"solid.move","params":{"matrix":self.matrix(params),"copy":false,"name":format!("{} horn placement",self.horn.model)},"target_roles":roles}));
        Ok(())
    }
}

fn primitive_lower_y(geometry: &Primitive, matrix: &[f64; 12]) -> f64 {
    let world_y = |point: &[f64; 3]| matrix[4] * point[0] + matrix[5] * point[1] + matrix[6] * point[2] + matrix[7];
    match geometry {
        Primitive::Box { corner_mm, size_mm } => {
            world_y(corner_mm)
                + [matrix[4] * size_mm[0], matrix[5] * size_mm[1], matrix[6] * size_mm[2]]
                    .iter()
                    .map(|value| value.min(0.0))
                    .sum::<f64>()
        }
        Primitive::Cylinder {
            base_mm,
            axis,
            diameter_mm,
            height_mm,
        } => {
            let norm = axis[0].hypot(axis[1]).hypot(axis[2]);
            let direction_y = (matrix[4] * axis[0] + matrix[5] * axis[1] + matrix[6] * axis[2]) / norm;
            world_y(base_mm) + (direction_y * height_mm).min(0.0)
                - diameter_mm * 0.5 * (1.0 - direction_y * direction_y).max(0.0).sqrt()
        }
        Primitive::Profile {
            z_mm,
            height_mm,
            segments,
        } => crate::models::profile_lower_y(*z_mm, *height_mm, segments, matrix),
    }
}
