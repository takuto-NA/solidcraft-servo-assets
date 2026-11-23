//! Build the small lookup index used by the host's read-only dialog hook.

use std::{env, error::Error, fs, path::PathBuf};

use serde::Deserialize;
use serde_json::Value;

#[path = "src/catalog_packs.rs"]
mod catalog_packs;

#[derive(Deserialize)]
struct Pack {
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    manufacturer: String,
    model: String,
    id: String,
    seller_items: Vec<SellerItem>,
    reference_model: Option<Value>,
    mount_groups: Vec<MountGroup>,
    #[serde(default)]
    output_interfaces: Vec<Output>,
}

#[derive(Deserialize)]
struct SellerItem {
    sku: String,
}

#[derive(Deserialize)]
struct MountGroup {
    id: String,
}

#[derive(Deserialize)]
struct Output {
    id: String,
    axis: Option<Value>,
}

#[derive(Deserialize)]
struct HornPack {
    horns: Vec<Horn>,
}

#[derive(Deserialize)]
struct Horn {
    manufacturer: String,
    model: String,
    id: String,
    parts: Vec<serde::de::IgnoredAny>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=catalog");
    println!("cargo:rerun-if-changed=src/catalog_packs.rs");
    let output = PathBuf::from(env::var_os("OUT_DIR").ok_or("Cargo did not set OUT_DIR")?);
    let mut assets = Vec::new();
    for pack in catalog_packs::SERVO_PACKS {
        assets.extend(serde_json::from_str::<Pack>(pack)?.assets);
    }
    assets.sort_by(|a, b| (&a.manufacturer, &a.model, &a.id).cmp(&(&b.manufacturer, &b.model, &b.id)));
    let mut servos = Vec::new();
    let mut mounts = Vec::new();
    let mut axes = Vec::new();
    for asset in &assets {
        let skus: Vec<_> = asset.seller_items.iter().map(|item| item.sku.as_str()).collect();
        let row = |id| (id, asset.manufacturer.as_str(), asset.model.as_str(), skus.clone());
        if asset.reference_model.is_some() {
            servos.push(row(asset.id.clone()));
        }
        for group in &asset.mount_groups {
            mounts.push(row(format!("{}/mount/{}", asset.id, group.id)));
        }
        for interface in asset
            .output_interfaces
            .iter()
            .filter(|interface| interface.axis.is_some())
        {
            axes.push(row(format!("{}/output/{}", asset.id, interface.id)));
        }
    }
    let mut horns = Vec::new();
    for pack in catalog_packs::HORN_PACKS {
        horns.extend(serde_json::from_str::<HornPack>(pack)?.horns);
    }
    horns.sort_by(|a, b| (&a.manufacturer, &a.model, &a.id).cmp(&(&b.manufacturer, &b.model, &b.id)));
    let mut references = Vec::new();
    let mut horn_models = Vec::new();
    for horn in &horns {
        let row = (
            horn.id.clone(),
            horn.manufacturer.as_str(),
            horn.model.as_str(),
            Vec::new(),
        );
        if !horn.parts.is_empty() {
            horn_models.push(row.clone());
        }
        references.push(row);
    }
    for (name, index) in [
        ("servo", servos),
        ("horn", references),
        ("horn-model", horn_models),
        ("mount", mounts),
        ("axis", axes),
    ] {
        fs::write(output.join(format!("{name}-search.json")), serde_json::to_vec(&index)?)?;
    }
    Ok(())
}
