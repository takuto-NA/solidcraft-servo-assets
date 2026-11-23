//! WIT adapter. Host refusals retain their original typed errors.

#[cfg(not(feature = "legacy-host"))]
wit_bindgen::generate!({ path: "wit", world: "command-dialog-plugin" });
#[cfg(feature = "legacy-host")]
wit_bindgen::generate!({ path: "wit-0.1", world: "command-plugin" });

use craft::plugin::commands;
use craft::plugin::types::{CommandFailure, FieldFailure, PluginError};
use exports::craft::plugin::entry::Guest;
use serde::Deserialize;
use serde_json::Value;

use crate::cables::{CableSpaceParams, insert_cable_space};
use crate::catalog::Catalog;
use crate::horns::{HornCatalog, HornParams, insert_horn};
use crate::models::{ModelParams, insert_model};
use crate::outputs::{OutputParams, insert_output_reference};
use crate::workflow::{
    DatumKind, Placement, ServoPlacement, picker_params, place_datum, place_datum_kind, place_horn, place_horn_model,
    place_servo, place_servo_model,
};
use crate::{Error, InsertParams, Runner, insert, invalid};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    #[serde(default)]
    query: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Describe {
    asset_id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Pattern {
    asset_id: String,
    #[serde(default)]
    mount_group_id: Option<String>,
    #[serde(default)]
    clearance_mm: f64,
    #[serde(default)]
    x_mm: f64,
    #[serde(default)]
    y_mm: f64,
    #[serde(default)]
    z_mm: f64,
}

struct Host {
    error: Option<PluginError>,
}

impl Runner for Host {
    fn run(&mut self, command: &str, args: Value) -> Result<Value, Error> {
        match commands::run(command, &args.to_string()) {
            Ok(reply) => serde_json::from_str(&reply).map_err(|error| invalid("host.reply", error)),
            Err(error) => {
                let reason = format!("{error:?}");
                self.error = Some(error);
                Err(Error::Host {
                    command: command.into(),
                    reason,
                })
            }
        }
    }

    #[cfg(not(feature = "legacy-host"))]
    fn save_snapshot(&mut self, name: &str, data: &[u8]) -> Result<(), Error> {
        craft::plugin::storage::put_blob(name, data, false, false)
            .map(|_| ())
            .map_err(|error| {
                self.error = Some(error);
                invalid("snapshot", "SolidCraft could not save the source snapshot")
            })
    }
}

fn error(error: Error) -> PluginError {
    match error {
        Error::Invalid { field, reason } => PluginError::InvalidInput(FieldFailure { field, reason }),
        Error::Unsupported(reason) => unsupported(reason),
        Error::Host { command, reason } => PluginError::CommandFailed(CommandFailure { command, reason }),
    }
}

#[cfg(not(feature = "legacy-host"))]
fn unsupported(reason: String) -> PluginError {
    PluginError::Unsupported(reason)
}

#[cfg(feature = "legacy-host")]
fn unsupported(reason: String) -> PluginError {
    PluginError::InvalidInput(FieldFailure {
        field: "representation".into(),
        reason,
    })
}

fn parse<T: serde::de::DeserializeOwned>(text: &str) -> Result<T, Error> {
    serde_json::from_str(text).map_err(|error| invalid("params", error))
}

struct Servos;

#[cfg(not(feature = "legacy-host"))]
impl exports::craft::plugin::dialog_provider::Guest for Servos {
    fn update_fields(entry: String, values: String) -> Result<String, PluginError> {
        let values: Value = parse(&values).map_err(error)?;
        let query = values["model_filter"].as_str().unwrap_or_default();
        crate::picker_filter::filter(&entry, query)
            .map(|reply| reply.to_string())
            .map_err(error)
    }
}

impl Guest for Servos {
    fn execute(entry: String, params: String) -> Result<String, PluginError> {
        if params.len() > 16384 {
            return Err(error(invalid("params", "parameters are limited to 16 KiB")));
        }
        let catalog = Catalog::embedded().map_err(error)?;
        let mut host = Host { error: None };
        let result = (|| match entry.as_str() {
            "catalog" => catalog.search(&parse::<Query>(&params)?.query),
            "describe" => catalog.describe(&parse::<Describe>(&params)?.asset_id),
            "insert-servo" => place_servo_model(&mut host, &catalog, parse::<ServoPlacement>(&params)?),
            "place-servo" => place_servo(&mut host, &catalog, parse::<ServoPlacement>(&params)?),
            "insert-horn-model" => {
                place_horn_model(&mut host, &HornCatalog::embedded()?, &picker_params(parse(&params)?)?)
            }
            "insert-horn" => insert_horn(&mut host, &HornCatalog::embedded()?, &picker_params(parse(&params)?)?),
            "insert-mounts" | "insert-axis" => place_datum_kind(
                &mut host,
                &catalog,
                &picker_params(parse(&params)?)?,
                if entry == "insert-mounts" {
                    DatumKind::Mounting
                } else {
                    DatumKind::ShaftAxis
                },
            ),
            "servo-reference" => insert_model(&mut host, &catalog, &parse::<ModelParams>(&params)?),
            "place-horn" => place_horn(&mut host, &HornCatalog::embedded()?, &parse::<Placement>(&params)?),
            "place-datum" => place_datum(&mut host, &catalog, &parse::<Placement>(&params)?),
            "horn-reference" => insert_horn(&mut host, &HornCatalog::embedded()?, &parse::<HornParams>(&params)?),
            "cable-space" => insert_cable_space(&mut host, &catalog, &parse::<CableSpaceParams>(&params)?),
            "output-reference" => insert_output_reference(&mut host, &catalog, &parse::<OutputParams>(&params)?),
            "insert" => {
                let params = parse::<InsertParams>(&params)?;
                if matches!(params.representation.as_str(), "mounting-pattern" | "output-reference") {
                    return Err(invalid(
                        "representation",
                        "use the mount-pattern or output-reference entry for datum references",
                    ));
                }
                insert(&mut host, &catalog, &params)
            }
            "mount-pattern" => {
                let p = parse::<Pattern>(&params)?;
                insert(
                    &mut host,
                    &catalog,
                    &InsertParams {
                        asset_id: p.asset_id,
                        representation: "mounting-pattern".into(),
                        mount_group_id: p.mount_group_id,
                        clearance_mm: p.clearance_mm,
                        x_mm: p.x_mm,
                        y_mm: p.y_mm,
                        z_mm: p.z_mm,
                    },
                )
            }
            _ => Err(invalid("entry", "unknown entry")),
        })();
        match result {
            Ok(reply) => Ok(reply.to_string()),
            Err(failure) => Err(host.error.take().unwrap_or_else(|| error(failure))),
        }
    }
}

// Only the dependency-generated C ABI exports need unsafe code. No application
// logic is inside this block; all handwritten Rust keeps the crate's deny lint.
#[allow(unsafe_code)]
const _: () = {
    export!(Servos);
};
