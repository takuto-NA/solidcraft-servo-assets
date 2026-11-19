//! The short insertion forms resolve a complete catalog choice before calling the host.

use serde::Deserialize;
use serde_json::Value;

use crate::catalog::{Asset, Catalog, LookupQuery, MountGroup, model_datum_status, selector_key};
use crate::horns::{Horn, HornCatalog, HornParams, insert_horn};
use crate::models::{ModelParams, insert_model};
use crate::outputs::{OutputInterface, OutputParams, insert_output_reference};
use crate::{Error, InsertParams, Runner, insert, invalid};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServoPlacement {
    pub asset_id: String,
    #[serde(default)]
    pub show_callouts: bool,
    #[serde(default)]
    pub x_mm: f64,
    #[serde(default)]
    pub y_mm: f64,
    #[serde(default)]
    pub z_mm: f64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    pub reference_id: String,
    #[serde(default)]
    pub x_mm: f64,
    #[serde(default)]
    pub y_mm: f64,
    #[serde(default)]
    pub z_mm: f64,
}

pub(crate) fn horn_modes(horn: &Horn) -> impl Iterator<Item = &'static str> {
    let solid = if horn.parts.is_empty() {
        None
    } else if horn.status == "photo_estimate" {
        Some("photo-solid")
    } else {
        Some("source-solid")
    };
    [solid, Some("dimensions")].into_iter().flatten()
}

pub(crate) fn horn_reference_id(horn: &Horn, mode: &str) -> String {
    format!("{}/{mode}", horn.id)
}

pub(crate) fn servo_title(asset: &Asset) -> String {
    let basis = match model_datum_status(asset) {
        "mount_and_axis_datums" => "source datums",
        "mount_datums" => "source mounts",
        "axis_datum" => "source axis",
        _ => "preview positions",
    };
    if cfg!(feature = "legacy-host") && basis == "preview positions" {
        format!("Preview · {}", asset.model)
    } else {
        format!("{} {} · {basis}", asset.manufacturer, asset.model)
    }
}

pub(crate) fn horn_title(horn: &Horn, mode: &str) -> String {
    let kind = match mode {
        "source-solid" => "Partial solid",
        "photo-solid" => "Photo estimate",
        _ => "Dimensions",
    };
    let model = horn.model.strip_suffix(" / photo estimate").unwrap_or(&horn.model);
    format!("{kind} · {model}")
}

fn selected(value: &str, canonical: &str) -> bool {
    value == canonical || value == selector_key(canonical)
}

pub fn place_servo(runner: &mut impl Runner, catalog: &Catalog, placement: ServoPlacement) -> Result<Value, Error> {
    let asset = servo_choice(catalog, &placement.asset_id)?;
    insert_model(
        runner,
        catalog,
        &ModelParams {
            asset_id: asset.id.clone(),
            x_mm: placement.x_mm,
            y_mm: placement.y_mm,
            z_mm: placement.z_mm,
            horn_id: None,
            horn_angle_deg: 0.0,
            horn_axial_offset_mm: 0.0,
            include_cable: true,
            include_connector: false,
            outlet_height_mm: None,
            wire_diameter_mm: None,
            wire_pitch_mm: None,
            wire_length_mm: None,
            labels: if placement.show_callouts {
                crate::models::Labels::Notes
            } else {
                crate::models::Labels::None
            },
        },
    )
}

fn servo_choice<'a>(catalog: &'a Catalog, choice: &str) -> Result<&'a Asset, Error> {
    let choice = choice.trim();
    let missing = || {
        invalid(
            "servo model",
            "Choose a servo model or search Find assets for its source ID.",
        )
    };
    if choice.len() > 256 || !choice.chars().any(char::is_alphanumeric) {
        return Err(missing());
    }
    if let Ok(asset) = catalog.asset(choice) {
        return Ok(asset);
    }
    let query = LookupQuery::new(choice);
    let mut matches = catalog.assets.iter().filter(|asset| {
        if asset.reference_model.is_none() {
            return false;
        }
        if cfg!(feature = "legacy-host") && choice == servo_title(asset) {
            return true;
        }
        let mut names = vec![asset.id.as_str(), asset.model.as_str()];
        names.extend(asset.seller_items.iter().map(|part| part.sku.as_str()));
        query.rank(&names, &[]).is_some_and(|(rank, _)| rank <= 2)
    });
    let asset = matches.next().ok_or_else(missing)?;
    if matches.next().is_some() {
        return Err(invalid(
            "servo model",
            "Several servo variants match. Choose the complete source profile ID.",
        ));
    }
    Ok(asset)
}

pub fn place_horn(runner: &mut impl Runner, catalog: &HornCatalog, placement: &Placement) -> Result<Value, Error> {
    let choice = placement.reference_id.trim();
    let mut matches = catalog
        .horns
        .iter()
        .flat_map(|horn| horn_modes(horn).map(move |mode| (horn, mode)))
        .filter(|(horn, mode)| {
            selected(choice, &horn_reference_id(horn, mode))
                || (cfg!(feature = "legacy-host") && choice == horn_title(horn, mode))
        });
    let (horn, mode) = matches.next().ok_or_else(|| {
        invalid(
            "horn",
            "Choose a horn representation listed for this variant. Use Find assets to look up its source ID.",
        )
    })?;
    if matches.next().is_some() {
        return Err(invalid(
            "horn",
            "Several horn variants share that name. Choose the complete source ID and representation.",
        ));
    }
    insert_horn(
        runner,
        catalog,
        &HornParams {
            horn_id: horn.id.clone(),
            mode: mode.into(),
            x_mm: placement.x_mm,
            y_mm: placement.y_mm,
            z_mm: placement.z_mm,
            angle_deg: 0.0,
        },
    )
}

pub(crate) enum Datum<'a> {
    Mount(&'a MountGroup),
    Output(&'a OutputInterface),
}

impl Datum<'_> {
    pub(crate) fn reference_id(&self, asset: &Asset) -> String {
        let (kind, id) = match self {
            Self::Mount(group) => ("mount", &group.id),
            Self::Output(output) => ("output", &output.id),
        };
        format!("{}/{kind}/{id}", asset.id)
    }

    pub(crate) fn title(&self, asset: &Asset) -> String {
        match self {
            Self::Mount(group) => {
                let face = match group.id.as_str() {
                    "case-front" => "output face",
                    "case-back" => "idler face",
                    "case-side-negative-y" => "side −Y",
                    "case-side-positive-y" => "side +Y",
                    "case-end-near" => "shaft end",
                    "case-end-far" => "far end",
                    "case-ears" | "drawing-ears" => "ears",
                    "case-mounts" => "case",
                    other => other,
                };
                format!("{} · mounts / {face}", asset.model)
            }
            Self::Output(output) => format!("{} · {} axis", asset.model, output.id),
        }
    }
}

pub(crate) fn datums(asset: &Asset) -> impl Iterator<Item = Datum<'_>> {
    asset.mount_groups.iter().map(Datum::Mount).chain(
        asset
            .output_interfaces
            .iter()
            .filter(|output| output.axis.is_some())
            .map(Datum::Output),
    )
}

pub fn place_datum(runner: &mut impl Runner, catalog: &Catalog, placement: &Placement) -> Result<Value, Error> {
    let choice = placement.reference_id.trim();
    let mut matches = catalog
        .assets
        .iter()
        .flat_map(|asset| datums(asset).map(move |datum| (asset, datum)))
        .filter(|(asset, datum)| {
            selected(choice, &datum.reference_id(asset))
                || (cfg!(feature = "legacy-host") && choice == datum.title(asset))
        });
    let (asset, datum) = matches.next().ok_or_else(|| {
            invalid(
                "reference",
                "Choose a source mounting face or output axis for this model. Preview positions are available through Insert servo.",
            )
        })?;
    if matches.next().is_some() {
        return Err(invalid(
            "reference",
            "Several source profiles share that name. Choose the complete source and interface ID.",
        ));
    }
    match datum {
        Datum::Mount(group) => insert(
            runner,
            catalog,
            &InsertParams {
                asset_id: asset.id.clone(),
                representation: "mounting-pattern".into(),
                mount_group_id: Some(group.id.clone()),
                clearance_mm: 0.0,
                x_mm: placement.x_mm,
                y_mm: placement.y_mm,
                z_mm: placement.z_mm,
            },
        ),
        Datum::Output(output) => insert_output_reference(
            runner,
            catalog,
            &OutputParams {
                asset_id: asset.id.clone(),
                output_id: output.id.clone(),
                clearance_mm: 0.0,
                x_mm: placement.x_mm,
                y_mm: placement.y_mm,
                z_mm: placement.z_mm,
            },
        ),
    }
}
