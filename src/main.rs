//! Local catalog inspection and reproducible command scripts, without installing a plugin.

use serde_json::{Value, json};
use solidcraft_servo_assets::cables::{CableSpaceParams, cable_recipe};
use solidcraft_servo_assets::manifest::{manifest, tooling_manifest};
use solidcraft_servo_assets::{InsertParams, catalog::Catalog, recipe};
use std::io::Read as _;

fn main() -> std::process::ExitCode {
    match run() {
        Ok(value) => match if std::env::args().nth(1).as_deref() == Some("manifest") {
            serde_json::to_string(&value)
        } else {
            serde_json::to_string_pretty(&value)
        } {
            Ok(text) => {
                println!("{text}");
                std::process::ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("{error}");
                std::process::ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run() -> Result<Value, Box<dyn std::error::Error>> {
    let catalog = Catalog::embedded()?;
    let args: Vec<_> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("catalog");
    let id = args.get(1).map(String::as_str).unwrap_or("");
    Ok(match command {
        "catalog" => catalog.search(id)?,
        "describe" => catalog.describe(id)?,
        "cable-recipe" => {
            let mut file = String::new();
            std::fs::File::open(id)?.take(16385).read_to_string(&mut file)?;
            if file.len() > 16384 {
                return Err("cable parameter file is limited to 16 KiB".into());
            }
            cable_recipe(&catalog, &serde_json::from_str::<CableSpaceParams>(&file)?)?
        }
        "recipe" => recipe(
            &catalog,
            &InsertParams {
                asset_id: id.into(),
                representation: args.get(2).cloned().unwrap_or_else(|| "case-envelope".into()),
                mount_group_id: None,
                clearance_mm: 0.0,
                x_mm: 0.0,
                y_mm: 0.0,
                z_mm: 0.0,
            },
        )?,
        "horn-describe" => {
            let horns = solidcraft_servo_assets::horns::HornCatalog::embedded()?;
            json!({"horn":horns.horn(id)?,"sources":horns.sources,"catalog_release":horns.release})
        }
        "horn-recipe" => {
            let mut file = String::new();
            std::fs::File::open(id)?.take(16385).read_to_string(&mut file)?;
            if file.len() > 16384 {
                return Err("horn parameter file is limited to 16 KiB".into());
            }
            solidcraft_servo_assets::horns::horn_recipe(
                &solidcraft_servo_assets::horns::HornCatalog::embedded()?,
                &serde_json::from_str::<solidcraft_servo_assets::horns::HornParams>(&file)?,
            )?
        }
        "manifest" if id == "--tools" && args.len() == 2 => tooling_manifest(&catalog)?,
        "manifest" if args.len() == 1 => manifest(&catalog)?,
        _ => {
            return Err(
                "usage: servo-assets [catalog QUERY | describe ID | recipe ID [REPRESENTATION] | cable-recipe PARAMS.json | horn-describe ID | horn-recipe PARAMS.json | manifest [--tools]]".into(),
            );
        }
    })
}
