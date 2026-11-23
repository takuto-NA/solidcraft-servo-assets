//! Search the small name indexes within the host's read-only form budget.

use serde_json::{Value, json};

use crate::catalog::{LookupQuery, selector_key};
use crate::{Error, invalid};

pub(crate) fn filter(entry: &str, text: &str) -> Result<Value, Error> {
    let (field, noun, index) = match entry {
        "insert-servo" => (
            "asset_id",
            "servo",
            include_str!(concat!(env!("OUT_DIR"), "/servo-search.json")),
        ),
        "insert-horn" => (
            "horn_id",
            "horn",
            include_str!(concat!(env!("OUT_DIR"), "/horn-search.json")),
        ),
        "insert-horn-model" => (
            "horn_id",
            "3D horn model",
            include_str!(concat!(env!("OUT_DIR"), "/horn-model-search.json")),
        ),
        "insert-mounts" => (
            "reference_id",
            "mounting face",
            include_str!(concat!(env!("OUT_DIR"), "/mount-search.json")),
        ),
        "insert-axis" => (
            "reference_id",
            "shaft axis",
            include_str!(concat!(env!("OUT_DIR"), "/axis-search.json")),
        ),
        _ => return Err(invalid("entry", "this form has no catalog filter")),
    };
    if text.chars().count() > 128 {
        return Err(invalid("model_filter", "Use up to 128 characters for the filter."));
    }
    if text.trim().is_empty() {
        return Ok(json!({}));
    }
    type SearchRow = (String, String, String, Vec<String>);
    let rows: Vec<SearchRow> = serde_json::from_str(index).map_err(|error| invalid("model_filter", error))?;
    let query = LookupQuery::new(text);
    let matches: Vec<_> = rows
        .iter()
        .filter_map(|(id, manufacturer, model, skus)| {
            let mut identities = vec![id.as_str(), model.as_str()];
            identities.extend(skus.iter().map(String::as_str));
            query.rank(&identities, &[manufacturer]).map(|(rank, _)| (id, rank))
        })
        .collect();
    // Prefer complete model numbers and SKUs when present in the query. For
    // example, a filter ending in W350-T must not also offer the R variant.
    const IDENTITY_MATCH: u8 = 3;
    let has_identity = matches.iter().any(|(_, rank)| *rank <= IDENTITY_MATCH);
    let choices: Vec<_> = matches
        .into_iter()
        .filter(|(_, rank)| !has_identity || *rank <= IDENTITY_MATCH)
        .map(|(id, _)| selector_key(id))
        .collect();
    let mut result = json!({field: {"choices": choices}});
    if choices.is_empty() {
        result[field]["hint"] = json!(format!("No {noun} matches. Clear the filter to see available choices."));
    }
    Ok(result)
}
