//! Catalog identity is separate from a manufacturer's physical revision and a seller's SKU.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{Error, invalid};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub id: String,
    pub publisher: String,
    pub title: String,
    pub url: String,
    pub accessed_on: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub source: String,
    pub locator: String,
    pub method: String,
    pub note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dimension {
    pub nominal_mm: f64,
    pub tolerance_mm: Option<f64>,
    pub evidence: Evidence,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SellerItem {
    pub seller: String,
    pub sku: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub source: String,
    pub note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Representation {
    pub title: String,
    pub kind: String,
    pub size_dimensions: Option<[String; 3]>,
    pub mount_group: Option<String>,
    pub frame: String,
    pub omitted: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountHole {
    pub id: String,
    pub center_mm: [f64; 2],
    pub diameter: Dimension,
    pub evidence: Evidence,
    #[serde(default)]
    pub thread: Option<ThreadSpec>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ThreadSpec {
    pub designation: String,
    pub pitch_mm: Option<Dimension>,
    pub specified_depth_mm: Option<Dimension>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountPlane {
    pub origin_mm: [f64; 3],
    pub x_dir: [f64; 3],
    pub y_dir: [f64; 3],
    pub evidence: Evidence,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountGroup {
    pub id: String,
    pub frame: String,
    #[serde(default)]
    pub plane_height_dimension: Option<String>,
    #[serde(default)]
    pub plane: Option<MountPlane>,
    pub holes: Vec<MountHole>,
    pub note: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub id: String,
    pub manufacturer: String,
    pub model: String,
    pub product_revision: Option<String>,
    pub data_revision: u32,
    pub family: String,
    pub seller_items: Vec<SellerItem>,
    pub dimensions: BTreeMap<String, Dimension>,
    pub representations: BTreeMap<String, Representation>,
    pub mount_groups: Vec<MountGroup>,
    pub cable_ports: Vec<CablePort>,
    #[serde(default)]
    pub output_interfaces: Vec<crate::outputs::OutputInterface>,
    #[serde(default)]
    pub reference_model: Option<crate::models::ReferenceModel>,
    pub missing_mount_dimensions: Vec<String>,
    pub observations: Vec<Observation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Count {
    pub value: u8,
    pub evidence: Evidence,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CablePort {
    pub id: String,
    pub attachment: String,
    pub bundle_count: Option<Count>,
    pub conductors_per_bundle: Option<Count>,
    pub connector_count: Option<Count>,
    #[serde(default)]
    pub contact_count: Option<Count>,
    pub lead_length: Option<Dimension>,
    pub exit_observation: Option<Observation>,
    pub unknown: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub release: String,
    pub sources: Vec<Source>,
    pub assets: Vec<Asset>,
}

/// Search metadata without turning a human-readable name into a selected variant.
pub(crate) struct LookupQuery {
    compact: String,
    terms: Vec<String>,
}

fn lookup_text(value: &str) -> String {
    value
        .chars()
        .flat_map(char::to_lowercase)
        .filter(|character| character.is_alphanumeric())
        .collect()
}

impl LookupQuery {
    pub(crate) fn new(query: &str) -> Self {
        Self {
            compact: lookup_text(query),
            terms: query
                .split(|character: char| !character.is_alphanumeric())
                .filter(|term| !term.is_empty())
                .map(lookup_text)
                .collect(),
        }
    }

    fn contains_identity(&self, identity: &str) -> bool {
        !identity.is_empty()
            && self.terms.iter().enumerate().any(|(start, _)| {
                let mut remaining = identity;
                for term in self.terms.iter().skip(start) {
                    let Some(tail) = remaining.strip_prefix(term) else {
                        break;
                    };
                    if tail.is_empty() {
                        return true;
                    }
                    remaining = tail;
                }
                false
            })
    }

    /// Identities are the canonical ID, model, then seller SKUs, in that order.
    pub(crate) fn rank(&self, identities: &[&str], fields: &[&str]) -> Option<(u8, usize)> {
        if self.compact.is_empty() {
            return Some((0, 0));
        }
        let identities: Vec<_> = identities.iter().map(|value| lookup_text(value)).collect();
        let fields: Vec<_> = fields.iter().map(|value| lookup_text(value)).collect();
        if !self
            .terms
            .iter()
            .all(|term| identities.iter().chain(&fields).any(|field| field.contains(term)))
        {
            return None;
        }
        if let Some(index) = identities.iter().position(|value| value == &self.compact) {
            return Some((
                match index {
                    0 => 0,
                    1 => 1,
                    _ => 2,
                },
                0,
            ));
        }
        if identities.iter().any(|identity| self.contains_identity(identity)) {
            return Some((3, 0));
        }
        for (rank, prefix) in [(4, true), (5, false)] {
            if let Some(extra) = identities
                .iter()
                .filter(|identity| {
                    if prefix {
                        identity.starts_with(&self.compact)
                    } else {
                        identity.contains(&self.compact)
                    }
                })
                .map(|identity| identity.len().saturating_sub(self.compact.len()))
                .min()
            {
                return Some((rank, extra));
            }
        }
        Some((6, 0))
    }
}

impl Catalog {
    pub fn embedded() -> Result<Self, Error> {
        let catalog = Self::merge_packs("2026-11-19.10", crate::catalog_packs::SERVO_PACKS)?;
        crate::horns::HornCatalog::embedded()?.validate_for_catalog(&catalog)?;
        Ok(catalog)
    }

    pub fn merge_packs(release: &str, packs: &[&str]) -> Result<Self, Error> {
        if packs.is_empty()
            || packs.len() > 16
            || release.is_empty()
            || release.len() > 64
            || packs
                .iter()
                .try_fold(0_usize, |total, text| total.checked_add(text.len()))
                .is_none_or(|size| size > 2 * 1024 * 1024)
        {
            return Err(invalid(
                "catalog",
                "use 1 to 16 packs, a release ID and at most 2 MiB total",
            ));
        }
        let mut combined = Self {
            schema_version: 1,
            release: release.into(),
            sources: Vec::new(),
            assets: Vec::new(),
        };
        for text in packs {
            let pack = Self::parse(text)?;
            combined.sources.extend(pack.sources);
            combined.assets.extend(pack.assets);
        }
        combined.validate()?;
        Ok(combined)
    }

    pub fn parse(text: &str) -> Result<Self, Error> {
        if text.len() > 512 * 1024 {
            return Err(invalid("catalog", "catalog packs are limited to 512 KiB"));
        }
        let catalog: Self = serde_json::from_str(text).map_err(|e| invalid("catalog", e))?;
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.schema_version != 1 || self.assets.len() > 256 || self.sources.len() > 256 {
            return Err(invalid("catalog", "unsupported schema or catalog pack size"));
        }
        let mut sources = BTreeSet::new();
        for source in &self.sources {
            if !sources.insert(source.id.as_str()) || !source.url.starts_with("https://") {
                return Err(invalid(
                    "catalog.sources",
                    "source IDs must be unique and URLs must use HTTPS",
                ));
            }
        }
        let mut ids = BTreeSet::new();
        let mut selectors = BTreeSet::new();
        for asset in &self.assets {
            if !ids.insert(asset.id.as_str())
                || !valid_id(&asset.id)
                || asset.id.starts_with("ref-")
                || !selectors.insert(selector_key(&asset.id))
                || asset.data_revision == 0
            {
                return Err(invalid(
                    "catalog.assets",
                    "asset IDs and selector keys must be unique; ref- is reserved; data revisions start at 1",
                ));
            }
            crate::outputs::validate(asset, &sources)?;
            crate::models::validate(asset)?;
            for dimension in asset
                .dimensions
                .values()
                .chain(asset.mount_groups.iter().flat_map(|group| {
                    group.holes.iter().flat_map(|hole| {
                        std::iter::once(&hole.diameter).chain(
                            hole.thread
                                .iter()
                                .flat_map(|thread| thread.pitch_mm.iter().chain(thread.specified_depth_mm.iter())),
                        )
                    })
                }))
            {
                if !dimension.nominal_mm.is_finite() || !(0.0..=10000.0).contains(&dimension.nominal_mm) {
                    return Err(invalid(
                        "catalog.dimensions",
                        "dimensions must be finite and between 0 and 10000 mm",
                    ));
                }
                if dimension
                    .tolerance_mm
                    .is_some_and(|v| !v.is_finite() || !(0.0..=100.0).contains(&v))
                    || !sources.contains(dimension.evidence.source.as_str())
                    || !matches!(
                        dimension.evidence.method.as_str(),
                        "drawing" | "specification" | "measurement"
                    )
                {
                    return Err(invalid(
                        "catalog.evidence",
                        "a dimension needs a known source, method and valid tolerance",
                    ));
                }
            }
            if asset.mount_groups.len() > 64 {
                return Err(invalid("catalog.mount_groups", "use at most 64 mounting groups"));
            }
            let mut groups = BTreeSet::new();
            let mut ports = BTreeSet::new();
            for port in &asset.cable_ports {
                if !valid_id(&port.id) || !ports.insert(port.id.as_str()) {
                    return Err(invalid("catalog.cable_ports", "port IDs must be valid and unique"));
                }
                for count in [
                    &port.bundle_count,
                    &port.conductors_per_bundle,
                    &port.connector_count,
                    &port.contact_count,
                ]
                .into_iter()
                .flatten()
                {
                    if count.value == 0
                        || count.value > 64
                        || !sources.contains(count.evidence.source.as_str())
                        || !matches!(
                            count.evidence.method.as_str(),
                            "photo_observation" | "drawing" | "specification" | "measurement"
                        )
                    {
                        return Err(invalid(
                            "catalog.cable_ports",
                            "cable counts need evidence and must be 1 to 64",
                        ));
                    }
                }
                if port.lead_length.as_ref().is_some_and(|length| {
                    !length.nominal_mm.is_finite()
                        || length.nominal_mm <= 0.0
                        || length.nominal_mm > 10000.0
                        || !sources.contains(length.evidence.source.as_str())
                        || length
                            .tolerance_mm
                            .is_some_and(|t| !t.is_finite() || !(0.0..=100.0).contains(&t))
                }) || port
                    .exit_observation
                    .as_ref()
                    .is_some_and(|o| !sources.contains(o.source.as_str()))
                {
                    return Err(invalid(
                        "catalog.cable_ports",
                        "lead and exit observations need finite dimensions and known sources",
                    ));
                }
            }
            for group in &asset.mount_groups {
                if !valid_id(&group.id)
                    || group.id == "profile-default"
                    || !groups.insert(group.id.as_str())
                    || !matches!(group.frame.as_str(), "near_mount_center" | "envelope_minimum")
                    || group.holes.is_empty()
                    || group.holes.len() > 128
                    || match group.frame.as_str() {
                        "near_mount_center" => group.plane_height_dimension.is_some() || group.plane.is_some(),
                        _ => match (&group.plane, &group.plane_height_dimension) {
                            (Some(_), None) => false,
                            (None, Some(key)) => !asset.dimensions.contains_key(key),
                            _ => true,
                        },
                    }
                {
                    return Err(invalid(
                        "catalog.mount_groups",
                        "use unique groups with 1 to 128 holes; case-frame patterns need either a sourced plane or height dimension",
                    ));
                }
                if let Some(plane) = &group.plane {
                    let unit = |direction: &[f64; 3]| {
                        direction.iter().all(|v| v.is_finite() && v.abs() <= 1.0)
                            && (direction.iter().map(|v| v * v).sum::<f64>() - 1.0).abs() <= 1e-9
                    };
                    let dot = plane.x_dir.iter().zip(&plane.y_dir).map(|(x, y)| x * y).sum::<f64>();
                    if !plane.origin_mm.iter().all(|v| v.is_finite() && v.abs() <= 10000.0)
                        || !unit(&plane.x_dir)
                        || !unit(&plane.y_dir)
                        || dot.abs() > 1e-9
                        || !sources.contains(plane.evidence.source.as_str())
                        || !matches!(plane.evidence.method.as_str(), "drawing" | "derived" | "measurement")
                    {
                        return Err(invalid(
                            "catalog.mount_groups",
                            "mount planes need finite coordinates, orthonormal directions and known evidence",
                        ));
                    }
                }
                let mut holes = BTreeSet::new();
                for hole in &group.holes {
                    if !valid_id(&hole.id)
                        || !holes.insert(hole.id.as_str())
                        || !hole.center_mm.iter().all(|v| v.is_finite() && v.abs() <= 10000.0)
                        || hole.diameter.nominal_mm <= 0.0
                        || !sources.contains(hole.evidence.source.as_str())
                    {
                        return Err(invalid(
                            "catalog.mount_groups",
                            "holes need unique IDs, finite coordinates, a positive diameter and known evidence",
                        ));
                    }
                    if let Some(thread) = &hole.thread
                        && (thread.designation.is_empty()
                            || thread.designation.len() > 64
                            || thread.designation.chars().any(char::is_control)
                            || thread
                                .pitch_mm
                                .iter()
                                .chain(thread.specified_depth_mm.iter())
                                .any(|dimension| dimension.nominal_mm <= 0.0))
                    {
                        return Err(invalid(
                            "catalog.mount_groups",
                            "thread labels must be short text and documented pitch/depth must be positive",
                        ));
                    }
                }
            }
            for observation in &asset.observations {
                if !sources.contains(observation.source.as_str()) {
                    return Err(invalid("catalog.observations", "unknown source"));
                }
            }
            for representation in asset.representations.values() {
                if representation.kind == "mounting_pattern" {
                    if representation.size_dimensions.is_some()
                        || !representation.mount_group.as_ref().is_some_and(|id| {
                            asset
                                .mount_groups
                                .iter()
                                .any(|group| &group.id == id && group.frame == representation.frame)
                        })
                    {
                        return Err(invalid(
                            "catalog.representation",
                            "a mounting pattern needs a known mounting group and its frame",
                        ));
                    }
                    continue;
                }
                if representation.kind != "box_envelope"
                    || representation.frame != "envelope_minimum"
                    || representation.mount_group.is_some()
                {
                    return Err(Error::Unsupported(
                        "unknown representation kind or coordinate frame".into(),
                    ));
                }
                let size = representation
                    .size_dimensions
                    .as_ref()
                    .ok_or_else(|| invalid("catalog.representation", "an envelope needs its dimensions"))?;
                for key in size {
                    let dimension = asset
                        .dimensions
                        .get(key)
                        .ok_or_else(|| invalid("catalog.representation", "unknown dimension"))?;
                    if dimension.nominal_mm <= 0.0 {
                        return Err(invalid(
                            "catalog.representation",
                            "an envelope dimension must be positive",
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn asset(&self, id: &str) -> Result<&Asset, Error> {
        self.assets
            .iter()
            .find(|asset| asset.id == id || selector_key(&asset.id) == id)
            .ok_or_else(|| invalid("asset_id", format!("unknown asset `{id}`")))
    }

    pub fn search(&self, query: &str) -> Result<Value, Error> {
        if query.len() > 128 {
            return Err(invalid("query", "use at most 128 bytes"));
        }
        let lookup = LookupQuery::new(query);
        let horns = crate::horns::HornCatalog::embedded()?;
        let mut matches: Vec<_> = self
            .assets
            .iter()
            .enumerate()
            .filter_map(|(index, asset)| {
                let mut identities = vec![asset.id.as_str(), asset.model.as_str()];
                identities.extend(asset.seller_items.iter().map(|item| item.sku.as_str()));
                let mut fields = vec![asset.manufacturer.as_str(), asset.family.as_str()];
                fields.extend(asset.seller_items.iter().map(|item| item.seller.as_str()));
                lookup.rank(&identities, &fields).map(|rank| (rank, index, asset))
            })
            .collect();
        matches.sort_by_key(|(rank, index, _)| (*rank, *index));
        let assets: Vec<_> = matches.into_iter().map(|(_, _, asset)| json!({
            "id": asset.id, "model": asset.model, "manufacturer": asset.manufacturer,
            "product_revision": asset.product_revision, "data_revision": asset.data_revision,
            "representations": asset.representations.keys().collect::<Vec<_>>(),
            "mount_pattern_available": !asset.mount_groups.is_empty(), "missing_mount_dimensions": asset.missing_mount_dimensions,
            "cable_ports": asset.cable_ports,
            "output_axis_available": asset.output_interfaces.iter().any(|output| output.axis.is_some()),
            "output_interfaces": asset.output_interfaces,
            "reference_model_available": asset.reference_model.is_some(),
            "reference_model_status": model_datum_status(asset),
            "horn_ids": horns.mounts.iter().filter(|mount| mount.asset_id == asset.id).map(|mount| &mount.horn_id).collect::<Vec<_>>()
        })).collect();
        Ok(
            json!({"catalog_release": self.release, "assets": assets, "horn_catalog_release":horns.release, "horns":horns.search(query)}),
        )
    }

    pub fn describe(&self, id: &str) -> Result<Value, Error> {
        let asset = self.asset(id)?;
        let horns = crate::horns::HornCatalog::embedded()?;
        let outputs = asset
            .output_interfaces
            .iter()
            .map(|output| crate::outputs::summary(asset, output))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(
            json!({"catalog_release": self.release, "asset": asset, "sources": self.sources_for_asset(asset),
            "mount_pattern_available": !asset.mount_groups.is_empty(),
            "reference_model_status": model_datum_status(asset),
            "output_axis_available": asset.output_interfaces.iter().any(|output| output.axis.is_some()),
            "output_references": outputs,
            "horns": horns.for_asset(&asset.id), "horn_catalog_release":horns.release, "note": "Nominal reference geometry; read each reference frame, source and omission. Horn dimensions, fit and placement have separate uncertainty. An unknown output axis is not placed at the box center."}),
        )
    }

    pub fn sources_for_asset<'a>(&'a self, asset: &Asset) -> Vec<&'a Source> {
        let mut ids = BTreeSet::new();
        for dimension in asset.dimensions.values() {
            ids.insert(dimension.evidence.source.as_str());
        }
        for group in &asset.mount_groups {
            if let Some(plane) = &group.plane {
                ids.insert(plane.evidence.source.as_str());
            }
            for hole in &group.holes {
                ids.insert(hole.evidence.source.as_str());
                ids.insert(hole.diameter.evidence.source.as_str());
                if let Some(thread) = &hole.thread {
                    for dimension in thread.pitch_mm.iter().chain(thread.specified_depth_mm.iter()) {
                        ids.insert(dimension.evidence.source.as_str());
                    }
                }
            }
        }
        for port in &asset.cable_ports {
            for count in [
                &port.bundle_count,
                &port.conductors_per_bundle,
                &port.connector_count,
                &port.contact_count,
            ]
            .into_iter()
            .flatten()
            {
                ids.insert(count.evidence.source.as_str());
            }
            if let Some(length) = &port.lead_length {
                ids.insert(length.evidence.source.as_str());
            }
            if let Some(exit) = &port.exit_observation {
                ids.insert(exit.source.as_str());
            }
        }
        for output in &asset.output_interfaces {
            if let Some(axis) = &output.axis {
                ids.insert(axis.evidence.source.as_str());
            }
        }
        for observation in &asset.observations {
            ids.insert(observation.source.as_str());
        }
        self.sources
            .iter()
            .filter(|source| ids.contains(source.id.as_str()))
            .collect()
    }
}

/// Compact UI values keep the host manifest small; saved instances use canonical IDs.
pub fn selector_key(asset_id: &str) -> String {
    let hash = asset_id.bytes().fold(0x811c9dc5_u32, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(0x01000193)
    });
    format!("ref-{hash:08x}")
}

pub fn model_datum_status(asset: &Asset) -> &'static str {
    let Some(model) = &asset.reference_model else {
        return "unavailable";
    };
    let mounts = asset.mount_groups.iter().any(|group| group.frame == model.frame);
    let axis = asset
        .output_interfaces
        .iter()
        .any(|output| output.axis.as_ref().is_some_and(|axis| axis.frame == model.frame));
    match (mounts, axis) {
        (true, true) => "mount_and_axis_datums",
        (true, false) => "mount_datums",
        (false, true) => "axis_datum",
        (false, false) => "preview",
    }
}

pub(crate) fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, b'.' | b'-' | b'_'))
}
