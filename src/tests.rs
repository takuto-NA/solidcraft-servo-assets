#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json};

use crate::{Error, InsertParams, Runner, catalog::Catalog, insert, recipe};

fn params(id: &str, representation: &str) -> InsertParams {
    InsertParams {
        asset_id: id.into(),
        representation: representation.into(),
        mount_group_id: None,
        clearance_mm: 0.0,
        x_mm: 0.0,
        y_mm: 0.0,
        z_mm: 0.0,
    }
}

#[derive(Default)]
struct Host {
    calls: Vec<(String, Value)>,
    model: bool,
    fallback_names: bool,
    sequential_sketch_ids: bool,
    feature_names: Vec<String>,
}

impl Runner for Host {
    fn run(&mut self, command: &str, args: Value) -> Result<Value, Error> {
        self.calls.push((command.into(), args.clone()));
        if command == "sketch.create"
            && let Some(name) = args["name"].as_str()
        {
            self.feature_names.push(name.into());
        }
        if self.model && command == "solid.move" {
            return Ok(
                json!({"feature":self.calls.len(),"created_bodies":[],"changed_bodies":args["bodies"],"removed_bodies":[]}),
            );
        }
        if command == "body.rename" {
            return Ok(json!({"body":args["body"],"name":args["name"]}));
        }
        if self.sequential_sketch_ids && command == "sketch.create" {
            return Ok(json!({"sketch":self.calls.len()}));
        }
        if self.model && matches!(command, "solid.box" | "solid.cylinder" | "solid.extrude") {
            let feature = self.calls.len() as u64;
            return if args["operation"] == "new" {
                // The host may rename a new body when an existing design uses its requested name.
                Ok(
                    json!({"feature":feature,"created_bodies":[if self.fallback_names { format!("Body{feature}") } else { format!("Allocated copy {feature}") }]}),
                )
            } else {
                Ok(json!({"feature":feature,"created_bodies":[],"changed_bodies":args["targets"],"removed_bodies":[]}))
            };
        }
        Ok(match command {
            "document.inspect" => {
                json!({"timeline":self.feature_names.iter().map(|name| json!({"name":name})).collect::<Vec<_>>()})
            }
            "solid.box" => {
                let index = self.calls.iter().filter(|(command, _)| command == "solid.box").count();
                let body = if index == 1 {
                    "SG90 ref (2)".into()
                } else {
                    format!("Cable body {index}")
                };
                json!({"feature": 41 + index, "created_bodies": [body], "bodies": ["Existing",body]})
            }
            "sketch.create" => json!({"sketch": 43}),
            "construct.axis.two_points" => json!({"feature":44, "created_bodies":[]}),
            "note.add" => json!({"note":self.calls.len()}),
            "solid.cylinder" => json!({"feature":45, "created_bodies":["Shaft envelope"]}),
            _ => json!({}),
        })
    }
}

fn source_snapshot(host: &Host, reply: &Value) -> Value {
    if cfg!(feature = "legacy-host") {
        assert_eq!(reply["metadata_saved_in_design"], false);
        assert!(!host.calls.iter().any(|(command, _)| command.starts_with("storage.")));
        return reply["source_snapshot"].clone();
    }
    assert_eq!(reply["metadata_saved_in_design"], true);
    let (_, blob) = host
        .calls
        .iter()
        .rev()
        .find(|(id, _)| id == "storage.put_blob")
        .expect("snapshot");
    let bytes = STANDARD
        .decode(blob["data"].as_str().expect("encoded"))
        .expect("base64");
    serde_json::from_slice(&bytes).expect("snapshot JSON")
}

#[test]
fn documentary_conflicts_do_not_become_production_revisions() {
    let catalog = Catalog::embedded().expect("embedded catalog");
    let asset = catalog.asset("towerpro.sg90-analog.configure").expect("SG90");
    assert!(asset.product_revision.is_none());
    assert_eq!(asset.dimensions["housing_height"].nominal_mm, 27.0);
    assert_eq!(asset.dimensions["overall_height"].nominal_mm, 30.3);
    assert!(asset.dimensions["housing_height"].tolerance_mm.is_none());
    assert!(!asset.observations.is_empty());
    assert!(asset.mount_groups.is_empty());
}

#[test]
fn catalog_lookup_accepts_natural_names_and_skus_without_selecting_a_variant() {
    let catalog = Catalog::embedded().expect("catalog");
    let ids = |query: &str, field: &str| {
        catalog.search(query).expect("search")[field]
            .as_array()
            .expect("result list")
            .iter()
            .map(|row| row["id"].as_str().expect("canonical ID").to_owned())
            .collect::<Vec<_>>()
    };
    let sg90 = ids("SG90", "assets");
    assert_eq!(sg90.len(), 2);
    assert!(sg90.iter().any(|id| id == "towerpro.sg90-analog.configure"));
    assert!(sg90.iter().any(|id| id == "aus-electronics-direct.sg90.ta0132"));
    assert_eq!(ids("sG-90", "assets"), sg90);
    assert_eq!(ids("SG 90", "assets"), sg90);
    for query in ["108761", "Akizuki 108 761", "tower pro sg-90 analog"] {
        assert_eq!(ids(query, "assets"), ["towerpro.sg90-analog.configure"]);
    }
    let c001 = vec!["feetech.sts3215-c001.a0", "feetech.sts3215-c001.lightguide"];
    for query in ["STS3215 C001", "fEeTeCh STS-3215 C001", "Akizuki 116312"] {
        assert_eq!(ids(query, "assets"), c001);
    }
    for query in ["XL330 M288", "m288 robotis xl330"] {
        assert_eq!(ids(query, "assets"), ["robotis.xl330-m288-t.brochure"]);
    }
    assert!(catalog.asset("SG-90").is_err());
    assert!(catalog.asset("STS3215 C001").is_err());
    assert!(catalog.asset("116312").is_err());

    // Exact identities must win even when a partial model occurs earlier in the pack.
    let mut reversed = catalog.clone();
    reversed.assets.reverse();
    for query in ["FS90", "feetech.fs90.akizuki-drawing", "FEETECH FS-90"] {
        assert_eq!(
            reversed.search(query).expect("ranked search")["assets"][0]["id"],
            "feetech.fs90.akizuki-drawing"
        );
    }
    assert_eq!(ids("ROBOTIS HN12 N101", "horns"), ["robotis.hn12-n101.normal-2019"]);
    let hn12 = ids("hn-12", "horns");
    assert_eq!(hn12.len(), 2);
    assert!(hn12.iter().any(|id| id == "robotis.hn12-n101.normal-2019"));
    assert!(hn12.iter().any(|id| id == "robotis.hn12-i101.idler-2019"));
    let double_arms = ids("hitec double arm", "horns");
    assert_eq!(double_arms.len(), 2);
    assert!(double_arms.iter().any(|id| id == "hitec.fs-il15.rev-d-20230428"));
    assert!(double_arms.iter().any(|id| id == "hitec.m-i24.rev-d-20230428"));

    let empty = catalog.search("").expect("empty search");
    assert_eq!(
        empty
            .as_object()
            .expect("unchanged reply shape")
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["assets", "catalog_release", "horn_catalog_release", "horns"]
    );
    assert_eq!(
        ids("", "assets"),
        catalog.assets.iter().map(|asset| asset.id.clone()).collect::<Vec<_>>()
    );
    assert_eq!(ids("", "assets"), ids(" -- . ", "assets"));
    assert!(ids("FS90 missing-unrelated-token", "assets").is_empty());
    assert!(catalog.search(&"x".repeat(129)).is_err());
    assert!(catalog.search(&"界".repeat(43)).is_err());
}

fn cable_params() -> crate::cables::CableSpaceParams {
    serde_json::from_value(json!({
        "asset_id": "towerpro.sg90-analog.configure", "port_id": "servo-lead",
        "exit_x_mm": 10, "exit_y_mm": 20, "exit_z_mm": 30, "direction": "negative_x",
        "bundle_width_mm": 3, "bundle_height_mm": 1, "straight_length_mm": 20,
        "flex_space_mm": 2, "connector_width_mm": 8, "connector_height_mm": 4, "connector_length_mm": 10
    }))
    .expect("cable parameters")
}

#[test]
fn cable_ports_distinguish_one_bundle_from_three_conductors() {
    let catalog = Catalog::embedded().expect("catalog");
    for port in catalog
        .assets
        .iter()
        .filter(|asset| {
            matches!(
                asset.id.as_str(),
                "towerpro.sg90-analog.configure"
                    | "towerpro.sg92r.configure"
                    | "towerpro.mg90s.configure"
                    | "feetech.fs90.akizuki-drawing"
            )
        })
        .flat_map(|asset| &asset.cable_ports)
    {
        assert_eq!(port.bundle_count.as_ref().expect("bundle count").value, 1);
        assert_eq!(port.conductors_per_bundle.as_ref().expect("wire count").value, 3);
        assert_eq!(port.lead_length.as_ref().expect("lead length").nominal_mm, 250.0);
        assert!(port.unknown.iter().any(|field| field.contains("Exit position")));
    }
}

#[test]
fn cable_space_keeps_source_facts_separate_from_user_geometry() {
    let catalog = Catalog::embedded().expect("catalog");
    let p = cable_params();
    let plan = crate::cables::cable_recipe(&catalog, &p).expect("plan");
    assert_eq!(plan[0]["params"]["corner"], json!([-10.0, 18.5, 29.5]));
    assert_eq!(plan[2]["params"]["corner"], json!([-20.0, 16.0, 28.0]));
    let mut host = Host::default();
    let reply = crate::cables::insert_cable_space(&mut host, &catalog, &p).expect("insert");
    assert_eq!(reply["bodies"].as_array().expect("bodies").len(), 3);
    assert_eq!(reply["features"], json!([42, 43, 44]));
    let snapshot = source_snapshot(&host, &reply);
    assert_eq!(snapshot["geometry_basis"], "user_specified_planning_envelopes");
    assert_eq!(snapshot["parameters"]["bundle_width_mm"], 3.0);
    assert!(snapshot["asset"]["cable_ports"][0].get("bundle_width_mm").is_none());
}

#[test]
fn cable_space_refuses_partial_connectors_and_nonfinite_exits_without_edits() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut p = cable_params();
    p.connector_height_mm = 0.0;
    let mut host = Host::default();
    assert!(crate::cables::insert_cable_space(&mut host, &catalog, &p).is_err());
    assert!(host.calls.is_empty());
    p = cable_params();
    p.exit_x_mm = f64::INFINITY;
    assert!(crate::cables::insert_cable_space(&mut host, &catalog, &p).is_err());
    assert!(host.calls.is_empty());
}

#[test]
fn hostile_input_is_refused_before_any_host_command() {
    let catalog = Catalog::embedded().expect("embedded catalog");
    for value in [f64::NAN, f64::INFINITY, -0.1, 10.1] {
        let mut p = params("towerpro.sg90-analog.configure", "case-envelope");
        p.clearance_mm = value;
        let mut host = Host::default();
        assert!(insert(&mut host, &catalog, &p).is_err());
        assert!(host.calls.is_empty());
    }
    for value in [f64::NAN, f64::NEG_INFINITY, 100001.0] {
        let mut p = params("towerpro.sg90-analog.configure", "case-envelope");
        p.x_mm = value;
        assert!(recipe(&catalog, &p).is_err());
    }
    for text in ["null", "[]", r#"{"asset_id":"unknown","extra":1}"#, r#"{"asset_id":2}"#] {
        assert!(serde_json::from_str::<InsertParams>(text).is_err());
    }
}

#[test]
fn unknown_mounting_geometry_is_not_inherited_from_another_model() {
    let catalog = Catalog::embedded().expect("embedded catalog");
    for id in [
        "towerpro.sg90-analog.configure",
        "towerpro.sg92r.configure",
        "towerpro.mg90s.configure",
    ] {
        assert!(matches!(
            recipe(&catalog, &params(id, "mounting-pattern")),
            Err(Error::Unsupported(_))
        ));
    }
}

#[test]
fn fs90_pattern_uses_explicit_centers_and_keeps_clearance_separate() {
    let catalog = Catalog::embedded().expect("embedded catalog");
    let mut p = params("feetech.fs90.akizuki-drawing", "mounting-pattern");
    p.clearance_mm = 0.2;
    p.x_mm = 10.0;
    p.y_mm = 20.0;
    p.z_mm = 30.0;
    let plan = recipe(&catalog, &p).expect("recipe");
    assert_eq!(plan[0]["params"]["plane"]["origin"], json!([10.0, 20.0, 30.0]));
    assert_eq!(plan[1]["params"]["center"], json!([0.0, 0.0]));
    assert_eq!(plan[2]["params"]["center"], json!([27.7, 0.0]));
    let literal = plan[1]["params"]["diameter"]
        .as_str()
        .expect("dimension")
        .trim_end_matches(" mm");
    assert!((literal.parse::<f64>().expect("number") - 2.6).abs() < 1e-12);
    assert_eq!(
        catalog.asset(&p.asset_id).expect("asset").mount_groups[0].holes[0]
            .diameter
            .nominal_mm,
        2.2
    );
}

#[test]
fn inserted_snapshots_keep_actual_allocated_names_and_original_facts() {
    let catalog = Catalog::embedded().expect("embedded catalog");
    let mut host = Host::default();
    let p = params("towerpro.sg90-analog.configure", "case-envelope");
    let reply = insert(&mut host, &catalog, &p).expect("insert");
    assert_eq!(host.calls[0].1["operation"], "new");
    assert!(host.calls[0].1.get("targets").is_none());
    assert_eq!(reply["bodies"], json!(["SG90 ref (2)"]));
    let snapshot = source_snapshot(&host, &reply);
    assert_eq!(snapshot["asset"]["dimensions"]["overall_height"]["nominal_mm"], 30.3);
    assert_eq!(snapshot["recipe"][0]["params"]["height"], "27 mm");
    assert_eq!(snapshot["bodies"], json!(["SG90 ref (2)"]));
    if !cfg!(feature = "legacy-host") {
        assert_eq!(
            host.calls[2].1["patch"]["instances"]["feature-42"]["snapshot"],
            "instances/feature-42.json"
        );
    }
}

#[test]
fn a_pattern_snapshot_references_its_sketch_and_creates_no_solid() {
    let catalog = Catalog::embedded().expect("embedded catalog");
    let mut host = Host::default();
    let reply = insert(
        &mut host,
        &catalog,
        &params("feetech.fs90.akizuki-drawing", "mounting-pattern"),
    )
    .expect("pattern");
    assert_eq!(reply["sketch"], 43);
    assert_eq!(reply["bodies"], json!([]));
    assert!(!host.calls.iter().any(|(command, _)| command == "solid.box"));
}

#[test]
fn catalog_validation_catches_broken_provenance_and_missing_dimensions() {
    let mut catalog = Catalog::embedded().expect("embedded catalog");
    catalog.assets[0]
        .dimensions
        .get_mut("case_width")
        .expect("width")
        .evidence
        .source = "missing".into();
    assert!(catalog.validate().is_err());
    let mut catalog = Catalog::embedded().expect("embedded catalog");
    catalog.assets[0].dimensions.remove("case_width");
    assert!(catalog.validate().is_err());
    let mut catalog = Catalog::embedded().expect("embedded catalog");
    catalog.assets.push(catalog.assets[0].clone());
    assert!(catalog.validate().is_err());
}

fn output_params(id: &str) -> crate::outputs::OutputParams {
    crate::outputs::OutputParams {
        asset_id: id.into(),
        output_id: "main".into(),
        clearance_mm: 0.0,
        x_mm: 0.0,
        y_mm: 0.0,
        z_mm: 0.0,
    }
}

#[test]
fn fs90_output_axis_and_shaft_share_the_mounting_frame() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut p = output_params("feetech.fs90.akizuki-drawing");
    p.x_mm = 10.0;
    p.y_mm = 20.0;
    p.z_mm = 30.0;
    p.clearance_mm = 0.2;
    let plan = crate::outputs::output_recipe(&catalog, &p).expect("output plan");
    let steps = plan.as_array().expect("steps");
    let axis = steps
        .iter()
        .find(|s| s["command"] == "construct.axis.two_points")
        .expect("axis");
    assert_eq!(axis["params"]["a"]["point"], json!([18.7, 20.0, 30.0]));
    assert_eq!(axis["params"]["b"]["point"], json!([18.7, 20.0, 31.0]));
    let shaft = steps
        .iter()
        .find(|s| s["command"] == "solid.cylinder")
        .expect("shaft envelope");
    assert_eq!(shaft["params"]["base"], json!([18.7, 20.0, 41.0]));
    assert_eq!(shaft["params"]["height"], "3 mm");
    assert_eq!(shaft["params"]["diameter"], "4.86 mm");
    let ordinary_circles = steps
        .iter()
        .filter(|s| s["command"] == "sketch.circle.center" && s["params"].get("construction") != Some(&json!(true)))
        .count();
    assert_eq!(
        ordinary_circles, 2,
        "the shaft marker must not become another hole profile"
    );
    let asset = catalog.asset(&p.asset_id).expect("asset");
    let summary = crate::outputs::summary(asset, &asset.output_interfaces[0]).expect("summary");
    assert_eq!(summary["shaft_top_in_frame"]["nominal_mm"], 14.0);
    assert_eq!(summary["shaft_top_in_frame"]["tolerance_mm"], 0.2);
    assert_eq!(summary["housing_top_in_frame"]["nominal_mm"], 11.0);
}

#[test]
fn sg90_alternate_source_has_its_own_case_frame_and_no_guessed_shaft_diameter() {
    let catalog = Catalog::embedded().expect("catalog");
    let asset = catalog
        .asset("aus-electronics-direct.sg90.ta0132")
        .expect("source-specific SG90");
    let axis = asset.output_interfaces[0].axis.as_ref().expect("located axis");
    assert_eq!(axis.origin_mm, [5.8, 6.05, 0.0]);
    assert_eq!(axis.evidence.method, "derived");
    assert!(asset.cable_ports.is_empty());
    assert!(asset.mount_groups.is_empty());
    let plan = crate::outputs::output_recipe(&catalog, &output_params(&asset.id)).expect("plan");
    assert!(
        !plan
            .as_array()
            .expect("steps")
            .iter()
            .any(|s| s["command"] == "solid.cylinder")
    );
    assert_eq!(plan[0]["params"]["width"], "12.1 mm");
    let sketch = plan
        .as_array()
        .expect("steps")
        .iter()
        .find(|s| s["command"] == "sketch.create")
        .expect("shaft-top marker");
    assert_eq!(sketch["params"]["plane"]["origin"], json!([0.0, 0.0, 30.7]));
}

#[test]
fn unknown_axes_fail_before_any_host_edits_and_are_visible_in_inspection() {
    let catalog = Catalog::embedded().expect("catalog");
    for id in [
        "towerpro.sg90-analog.configure",
        "towerpro.sg92r.configure",
        "towerpro.mg90s.configure",
    ] {
        let mut host = Host::default();
        assert!(matches!(
            crate::outputs::insert_output_reference(&mut host, &catalog, &output_params(id)),
            Err(Error::Unsupported(_))
        ));
        assert!(host.calls.is_empty());
        assert_eq!(
            catalog.describe(id).expect("description")["output_references"][0]["status"],
            "unlocated"
        );
    }
}

#[test]
fn output_snapshots_keep_the_axis_datum_and_source_interpretation() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut host = Host::default();
    let reply =
        crate::outputs::insert_output_reference(&mut host, &catalog, &output_params("feetech.fs90.akizuki-drawing"))
            .expect("output reference");
    assert_eq!(reply["features"], json!([43, 44, 45]));
    let snapshot = source_snapshot(&host, &reply);
    assert_eq!(
        snapshot["asset"]["output_interfaces"][0]["axis"]["frame"],
        "near_mount_center"
    );
    assert_eq!(reply["output_reference"]["axis"]["origin_mm"], json!([8.7, 0.0, 0.0]));
}

#[test]
fn invalid_output_frames_and_unknown_sources_are_refused() {
    let mut catalog = Catalog::embedded().expect("catalog");
    let asset = catalog.assets.iter_mut().find(|a| a.model == "FS90").expect("FS90");
    asset.output_interfaces[0].axis.as_mut().expect("axis").origin_mm[0] = f64::INFINITY;
    assert!(catalog.validate().is_err());
    let mut catalog = Catalog::embedded().expect("catalog");
    let asset = catalog.assets.iter_mut().find(|a| a.model == "FS90").expect("FS90");
    asset.output_interfaces[0].axis.as_mut().expect("axis").evidence.source = "unknown".into();
    assert!(catalog.validate().is_err());
}

fn model_params() -> crate::models::ModelParams {
    serde_json::from_value(json!({"asset_id":"feetech.fs90.akizuki-drawing"})).expect("model parameters")
}

#[test]
fn complete_model_modifiers_and_colors_follow_allocated_body_names() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut host = Host {
        model: true,
        ..Host::default()
    };
    let reply = crate::models::insert_model(&mut host, &catalog, &model_params()).expect("integrated model");
    let bodies = reply["bodies"].as_array().expect("created bodies");
    assert_eq!(bodies.len(), 8);
    assert!(
        !reply["body_roles"]
            .as_object()
            .expect("roles")
            .contains_key("servo-lead-connector")
    );
    assert_eq!(reply["notes"].as_array().expect("notes").len(), 6);
    let roles = reply["body_roles"].as_object().expect("roles");
    for (_, args) in host.calls.iter().filter(|(command, _)| command == "appearance.assign") {
        let targets = args["bodies"].as_array().expect("appearance targets");
        assert_eq!(targets.len(), 1);
        assert!(bodies.contains(&targets[0]));
    }
    for (_, args) in host.calls.iter().filter(|(command, args)| {
        matches!(command.as_str(), "solid.box" | "solid.cylinder") && args["operation"] != "new"
    }) {
        let targets = args["targets"].as_array().expect("modifier targets");
        assert_eq!(targets.len(), 1);
        assert!([&roles["near-ear"], &roles["far-ear"], &roles["turret"]].contains(&&targets[0]));
    }
    let snapshot = source_snapshot(&host, &reply);
    assert_eq!(snapshot["geometry_basis"], "mixed_dimensioned_assumed_and_approximate");
    assert!(
        !snapshot["asset"]["reference_model"]["assumptions"]
            .as_array()
            .expect("assumptions")
            .is_empty()
    );
    assert_eq!(snapshot["body_roles"], reply["body_roles"]);
}

#[test]
fn front_labels_follow_adjusted_outlets_and_disappear_with_disabled_cables() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut p = model_params();
    p.labels = crate::models::Labels::Front;
    p.outlet_height_mm = Some(4.0);
    p.x_mm = 100.0;
    p.z_mm = 50.0;
    let plan = crate::models::model_recipe(&catalog, &p).expect("plan");
    let steps = plan.as_array().expect("steps");
    let leaders: Vec<_> = steps.iter().filter(|s| s["command"] == "sketch.line").collect();
    assert_eq!(leaders[3]["params"]["points"][0], json!([2.6, -11.7]));
    let label_sketch = steps
        .iter()
        .find(|s| s["command"] == "sketch.create")
        .expect("label sketch");
    assert_eq!(label_sketch["params"]["plane"]["origin"], json!([100.0, -8.05, 50.0]));
    p.include_cable = false;
    let plan = crate::models::model_recipe(&catalog, &p).expect("no-cable plan");
    let steps = plan.as_array().expect("steps");
    assert_eq!(steps.iter().filter(|s| s["command"] == "sketch.text").count(), 4);
    assert!(!steps.iter().any(|s| {
        s["bind_role"]
            .as_str()
            .is_some_and(|role| role.starts_with("servo-lead-"))
    }));
}

#[test]
fn invalid_model_geometry_is_refused_before_host_edits() {
    let catalog = Catalog::embedded().expect("catalog");
    for value in [f64::NAN, f64::INFINITY, 0.0, 251.0] {
        let mut p = model_params();
        p.wire_length_mm = Some(value);
        let mut host = Host::default();
        assert!(crate::models::insert_model(&mut host, &catalog, &p).is_err());
        assert!(host.calls.is_empty());
    }
    let mut p = model_params();
    p.wire_pitch_mm = Some(0.5);
    let mut host = Host::default();
    assert!(crate::models::insert_model(&mut host, &catalog, &p).is_err());
    assert!(host.calls.is_empty());
    p = model_params();
    p.asset_id = "aus-electronics-direct.sg90.ta0132".into();
    assert!(matches!(
        crate::models::insert_model(&mut host, &catalog, &p),
        Err(Error::Unsupported(_))
    ));
    assert!(host.calls.is_empty());
}

#[test]
fn model_catalog_rejects_role_collisions_and_unbound_label_ports() {
    for mutation in 0..3 {
        let mut catalog = Catalog::embedded().expect("catalog");
        let asset = catalog
            .assets
            .iter_mut()
            .find(|asset| asset.model == "FS90")
            .expect("FS90");
        let model = asset.reference_model.as_mut().expect("model");
        match mutation {
            0 => model.parts[0].id = "servo-lead-wire-1".into(),
            1 => model.front_labels[0].cable_port = Some("missing-port".into()),
            _ => model.frame = "undeclared-frame".into(),
        }
        assert!(catalog.validate().is_err());
    }
}

#[test]
fn servo_previews_do_not_promote_planning_positions_into_sourced_datums() {
    let catalog = Catalog::embedded().expect("catalog");
    for id in [
        "towerpro.sg90-analog.configure",
        "towerpro.sg92r.configure",
        "towerpro.mg90s.configure",
    ] {
        let asset = catalog.asset(id).expect("asset");
        assert!(asset.mount_groups.is_empty());
        assert!(asset.output_interfaces.iter().all(|output| output.axis.is_none()));
        let model = asset.reference_model.as_ref().expect("preview model");
        assert!(
            model
                .assumptions
                .iter()
                .any(|assumption| assumption.contains("PREVIEW"))
        );
        let mut p = model_params();
        p.asset_id = id.into();
        let mut host = Host {
            model: true,
            ..Host::default()
        };
        let reply = crate::models::insert_model(&mut host, &catalog, &p).expect("preview insert");
        assert_eq!(reply["bodies"].as_array().expect("bodies").len(), 8);
        let snapshot = source_snapshot(&host, &reply);
        assert_eq!(snapshot["asset"]["output_interfaces"][0]["axis"], Value::Null);
    }
}

#[test]
fn default_outlet_markers_are_short_and_connectors_require_explicit_opt_in() {
    let catalog = Catalog::embedded().expect("catalog");
    for asset in catalog.assets.iter().filter(|asset| {
        asset
            .reference_model
            .as_ref()
            .is_some_and(|model| !model.cables.is_empty())
    }) {
        let mut p = model_params();
        p.asset_id = asset.id.clone();
        let plan = crate::models::model_recipe(&catalog, &p).expect("recipe");
        let steps = plan.as_array().expect("steps");
        let wires: Vec<_> = steps
            .iter()
            .filter(|step| step["bind_role"].as_str().is_some_and(|role| role.contains("-wire-")))
            .collect();
        assert_eq!(wires.len(), 3);
        assert!(
            wires
                .iter()
                .all(|wire| wire["params"]["height"] == "3 mm" && wire["params"]["axis"] == json!([-1.0, 0.0, 0.0]))
        );
        assert!(!steps.iter().any(|step| {
            step["bind_role"]
                .as_str()
                .is_some_and(|role| role.ends_with("-connector"))
        }));
        p.include_connector = true;
        p.wire_length_mm = Some(20.0);
        let plan = crate::models::model_recipe(&catalog, &p).expect("optional connector recipe");
        assert_eq!(
            plan.as_array()
                .expect("steps")
                .iter()
                .filter(|step| step["bind_role"]
                    .as_str()
                    .is_some_and(|role| role.ends_with("-connector")))
                .count(),
            1
        );
        p.include_cable = false;
        let plan = crate::models::model_recipe(&catalog, &p).expect("no-cable recipe");
        assert!(!plan.as_array().expect("steps").iter().any(|step| {
            step["bind_role"]
                .as_str()
                .is_some_and(|role| role.starts_with("servo-lead-"))
        }));
    }
}

#[test]
fn manifest_keeps_host_forms_bounded_when_contributors_expand_the_catalog() {
    let mut catalog = Catalog::embedded().expect("catalog");
    let current = crate::manifest::manifest(&catalog).expect("current manifest");
    let original = entry_schema(&current, "place-servo");
    let original_choice = &original["properties"]["asset_id"];
    assert_eq!(
        listed_values(original_choice).len(),
        catalog
            .assets
            .iter()
            .filter(|asset| asset.reference_model.is_some())
            .count()
    );
    assert!(original_choice.get("default").is_none());
    let budget = if cfg!(feature = "legacy-host") {
        crate::manifest::LEGACY_MANIFEST_BYTES
    } else {
        crate::manifest::MODERN_MANIFEST_BYTES
    };
    assert!(serde_json::to_vec(&current).expect("encoded manifest").len() < budget);

    // Synthetic metadata tests the host ABI limit, without inventing new hardware facts.
    let template = catalog.asset("feetech.fs90.akizuki-drawing").expect("FS90").clone();
    for index in 0..400 {
        let mut asset = template.clone();
        asset.id = format!("test.catalog-growth-{index}");
        asset.model = format!("Synthetic selector {index}: {}", "long metadata ".repeat(12));
        catalog.assets.push(asset);
    }
    let expanded = crate::manifest::manifest(&catalog).expect("bounded expanded manifest");
    assert!(serde_json::to_vec(&expanded).expect("encoded expanded manifest").len() < budget);
    let expanded_choice = &entry_schema(&expanded, "place-servo")["properties"]["asset_id"];
    assert_eq!(expanded_choice["type"], "string");
    assert!(expanded_choice.get("oneOf").is_none() && expanded_choice.get("enum").is_none());
    assert_eq!(entry_schema(&expanded, "place-servo")["required"], original["required"]);
    assert_eq!(expanded["capabilities"], current["capabilities"]);
    assert_eq!(expanded["runtime"]["commands"], current["runtime"]["commands"]);
    assert_eq!(expanded["runtime"]["limits"], current["runtime"]["limits"]);
}

fn entry_schema<'a>(manifest: &'a Value, name: &str) -> &'a Value {
    &manifest["runtime"]["entries"]
        .as_array()
        .expect("entries")
        .iter()
        .find(|entry| entry["name"] == name)
        .expect("entry")["input_schema"]
}

fn listed_values(field: &Value) -> Vec<&str> {
    if let Some(options) = field["oneOf"].as_array() {
        return options
            .iter()
            .map(|option| option["const"].as_str().expect("choice value"))
            .collect();
    }
    field["enum"]
        .as_array()
        .expect("enumerated picker")
        .iter()
        .map(|option| option.as_str().expect("choice value"))
        .collect()
}

#[test]
fn every_quick_picker_option_resolves_to_an_available_reference() {
    use crate::workflow::{place_datum, place_horn, place_servo};

    let catalog = Catalog::embedded().expect("catalog");
    let horns = crate::horns::HornCatalog::embedded().expect("horns");
    let manifest = crate::manifest::manifest(&catalog).expect("manifest");
    for (name, key) in [
        ("place-servo", "asset_id"),
        ("place-horn", "reference_id"),
        ("place-datum", "reference_id"),
    ] {
        let schema = entry_schema(&manifest, name);
        assert_eq!(schema["required"], json!([key]));
        let options = listed_values(&schema["properties"][key]);
        assert!(options.len() > 1);
        for option in options {
            let mut host = Host {
                model: true,
                sequential_sketch_ids: true,
                ..Host::default()
            };
            let params = json!({key:option});
            let result = match name {
                "place-servo" => place_servo(
                    &mut host,
                    &catalog,
                    serde_json::from_value(params).expect("servo parameters"),
                ),
                "place-horn" => place_horn(
                    &mut host,
                    &horns,
                    &serde_json::from_value(params).expect("horn parameters"),
                ),
                _ => place_datum(
                    &mut host,
                    &catalog,
                    &serde_json::from_value(params).expect("datum parameters"),
                ),
            };
            assert!(result.is_ok(), "{name} / {option}: {result:?}");
        }
    }
}

#[test]
fn repeated_datum_insertion_keeps_the_source_caption_and_owns_a_new_sketch() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut host = Host {
        sequential_sketch_ids: true,
        ..Host::default()
    };
    let first = crate::workflow::Placement {
        reference_id: "feetech.fs90.akizuki-drawing/mount/case-ears".into(),
        x_mm: 0.0,
        y_mm: 0.0,
        z_mm: 0.0,
    };
    let a = crate::workflow::place_datum(&mut host, &catalog, &first).expect("first mount");
    let second = crate::workflow::Placement { x_mm: 40.0, ..first };
    let b = crate::workflow::place_datum(&mut host, &catalog, &second).expect("another mount");
    assert_eq!(host.feature_names.len(), 2);
    assert!(host.feature_names[0].contains("FS90 mounting-pattern"));
    assert_eq!(host.feature_names[1], format!("{} (2)", host.feature_names[0]));
    assert_ne!(a["feature"], b["feature"]);
    let snapshot = source_snapshot(&host, &b);
    assert!(
        snapshot["recipe"]
            .as_array()
            .expect("recipe")
            .iter()
            .all(|step| step["command"] != "document.inspect")
    );
}

#[test]
fn catalog_packs_merge_without_changing_existing_profiles_or_sharing_source_ids() {
    let original = Catalog::parse(include_str!("../catalog/towerpro-micro.json")).expect("original pack");
    let merged = Catalog::embedded().expect("merged catalog");
    assert_eq!(merged.assets.len(), 29);
    assert_eq!(
        merged
            .assets
            .iter()
            .filter(|asset| asset.reference_model.is_some())
            .count(),
        28
    );
    for asset in &original.assets {
        assert_eq!(
            serde_json::to_value(merged.asset(&asset.id).expect("preserved asset")).expect("JSON"),
            serde_json::to_value(asset).expect("JSON")
        );
    }
    assert!(Catalog::merge_packs("test", &[]).is_err());
    assert!(
        Catalog::merge_packs(
            "test",
            &[
                include_str!("../catalog/towerpro-micro.json"),
                include_str!("../catalog/towerpro-micro.json")
            ]
        )
        .is_err()
    );
    let description = merged.describe("hitec.hs55.hcs-v2-2").expect("description");
    let source_count = description["sources"].as_array().expect("sources").len();
    assert!(source_count > 0 && source_count < merged.sources.len());
    assert!(
        !description["sources"]
            .as_array()
            .expect("sources")
            .iter()
            .any(|source| source["id"] == "towerpro-sg90")
    );
}

#[test]
fn case_frame_mount_patterns_apply_the_source_plane_height() {
    let mut catalog = Catalog::parse(include_str!("../catalog/towerpro-micro.json")).expect("catalog");
    let asset = catalog
        .assets
        .iter_mut()
        .find(|asset| asset.model == "FS90")
        .expect("FS90");
    // Synthetic case-frame fixture isolates registration from the original mounting-frame record.
    asset.reference_model = None;
    asset.output_interfaces.clear();
    asset.mount_groups[0].frame = "envelope_minimum".into();
    asset.mount_groups[0].plane_height_dimension = Some("mount_underside_height".into());
    asset
        .representations
        .get_mut("mounting-pattern")
        .expect("pattern")
        .frame = "envelope_minimum".into();
    let mut p = params("feetech.fs90.akizuki-drawing", "mounting-pattern");
    p.x_mm = 10.0;
    p.y_mm = 20.0;
    p.z_mm = 30.0;
    let plan = recipe(&catalog, &p).expect("case-frame pattern");
    assert_eq!(plan[0]["params"]["plane"]["origin"], json!([10.0, 20.0, 45.7]));
    let asset = catalog
        .assets
        .iter_mut()
        .find(|asset| asset.model == "FS90")
        .expect("FS90");
    asset.mount_groups[0].plane_height_dimension = Some("missing".into());
    assert!(catalog.validate().is_err());
}

#[test]
fn oriented_mount_groups_keep_their_plane_thread_callouts_and_selection() {
    let mut catalog = Catalog::parse(include_str!("../catalog/towerpro-micro.json")).expect("catalog");
    let asset = catalog.assets.iter_mut().find(|a| a.model == "FS90").expect("FS90");
    asset.reference_model = None;
    asset.output_interfaces.clear();
    let evidence = asset.dimensions["case_length"].evidence.clone();
    let group = &mut asset.mount_groups[0];
    group.frame = "envelope_minimum".into();
    group.plane = Some(crate::catalog::MountPlane {
        origin_mm: [12.0, 0.0, 7.0],
        x_dir: [1.0, 0.0, 0.0],
        y_dir: [0.0, 0.0, 1.0],
        evidence: evidence.clone(),
    });
    group.holes[0].thread = Some(crate::catalog::ThreadSpec {
        designation: "M2 tapping screw".into(),
        pitch_mm: None,
        specified_depth_mm: None,
    });
    let mut far_group = group.clone();
    far_group.id = "case-far-side".into();
    far_group.plane.as_mut().expect("plane").origin_mm[1] = 12.2;
    asset.mount_groups.push(far_group);
    asset
        .representations
        .get_mut("mounting-pattern")
        .expect("pattern")
        .frame = "envelope_minimum".into();
    let mut p = params("feetech.fs90.akizuki-drawing", "mounting-pattern");
    p.x_mm = 100.0;
    p.y_mm = 200.0;
    p.z_mm = 300.0;
    let plan = recipe(&catalog, &p).expect("oriented pattern");
    assert_eq!(
        plan[0]["params"]["plane"],
        json!({"origin":[112.0,200.0,307.0],"x_dir":[1.0,0.0,0.0],"y_dir":[0.0,0.0,1.0]})
    );
    p.mount_group_id = Some("case-far-side".into());
    let mut host = Host::default();
    let reply = insert(&mut host, &catalog, &p).expect("selected face");
    let (_, created) = host
        .calls
        .iter()
        .find(|(name, _)| name == "sketch.create")
        .expect("mount sketch");
    assert_eq!(created["plane"]["origin"], json!([112.0, 212.2, 307.0]));
    let snapshot = source_snapshot(&host, &reply);
    assert_eq!(snapshot["parameters"]["mount_group_id"], "case-far-side");
    let thread = &snapshot["asset"]["mount_groups"][1]["holes"][0]["thread"];
    assert_eq!(thread["designation"], "M2 tapping screw");
    assert!(thread["pitch_mm"].is_null() && thread["specified_depth_mm"].is_null());
    p.mount_group_id = Some("missing-face".into());
    let mut host = Host::default();
    assert!(insert(&mut host, &catalog, &p).is_err());
    assert!(host.calls.is_empty());
    for directions in [[1.0, 0.0, 0.0], [0.0, 0.0, 2.0], [f64::NAN, 0.0, 1.0]] {
        let mut invalid_catalog = catalog.clone();
        invalid_catalog
            .assets
            .iter_mut()
            .find(|a| a.model == "FS90")
            .expect("FS90")
            .mount_groups[0]
            .plane
            .as_mut()
            .expect("plane")
            .y_dir = directions;
        assert!(invalid_catalog.validate().is_err());
    }
    p.mount_group_id = Some("profile-default".into());
    assert_eq!(recipe(&catalog, &p).expect("profile default"), plan);
    p.representation = "case-envelope".into();
    assert!(recipe(&catalog, &p).is_err());
    let asset = catalog.assets.iter_mut().find(|a| a.model == "FS90").expect("FS90");
    let named_pattern = asset.representations["mounting-pattern"].clone();
    asset.representations.insert("mounting-side".into(), named_pattern);
    p.representation = "mounting-side".into();
    let mut host = Host::default();
    let reply = insert(&mut host, &catalog, &p).expect("named pattern");
    assert!(
        reply["warning"]
            .as_str()
            .expect("warning")
            .starts_with("Nominal screw reference circles")
    );
}

#[test]
fn compact_ui_selectors_preserve_canonical_identity_in_saved_instances() {
    let catalog = Catalog::embedded().expect("catalog");
    let id = "feetech.fs90.akizuki-drawing";
    let selector = crate::catalog::selector_key(id);
    assert_eq!(selector.len(), 12);
    assert_eq!(catalog.asset(&selector).expect("selector").id, id);
    let mut host = Host::default();
    let reply = insert(&mut host, &catalog, &params(&selector, "mounting-pattern")).expect("insert");
    let snapshot = source_snapshot(&host, &reply);
    assert_eq!(reply["asset_id"], id);
    assert_eq!(snapshot["asset"]["id"], id);
    assert_eq!(snapshot["parameters"]["asset_id"], id);
    assert_eq!(catalog.describe(&selector).expect("describe")["asset"]["id"], id);
    let mut invalid_catalog = catalog.clone();
    invalid_catalog.assets[0].id = "ref-reserved".into();
    assert!(invalid_catalog.validate().is_err());
    let first = "selector.fixture-ecfd338a";
    let second = "selector.fixture-75c44a1b";
    assert_eq!(
        crate::catalog::selector_key(first),
        crate::catalog::selector_key(second)
    );
    let mut collision_catalog = catalog.clone();
    collision_catalog.assets[0].id = first.into();
    collision_catalog.assets[1].id = second.into();
    assert!(collision_catalog.validate().is_err());
}

#[test]
fn axis_only_crosses_use_the_documented_datum_point_height() {
    let mut catalog = Catalog::parse(include_str!("../catalog/towerpro-micro.json")).expect("catalog");
    let asset = catalog.assets.iter_mut().find(|a| a.model == "FS90").expect("FS90");
    let interface = &mut asset.output_interfaces[0];
    interface.axis.as_mut().expect("axis").frame = "envelope_minimum".into();
    interface.axis.as_mut().expect("axis").origin_mm[2] = 23.0;
    interface.housing_top_dimension = None;
    interface.shaft_top_dimension = None;
    interface.shaft_diameter_dimension = None;
    interface.frame_height_dimension = None;
    let mut p = output_params("feetech.fs90.akizuki-drawing");
    p.z_mm = 50.0;
    let plan = crate::outputs::output_recipe(&catalog, &p).expect("axis-only");
    let steps = plan.as_array().expect("steps");
    let sketch = steps
        .iter()
        .find(|step| step["command"] == "sketch.create")
        .expect("cross plane");
    assert_eq!(sketch["params"]["plane"]["origin"][2], 73.0);
    assert!(!steps.iter().any(|step| step["command"] == "solid.cylinder"));
}

#[test]
fn robotis_case_patterns_keep_screw_types_and_opposed_output_datums() {
    let catalog = Catalog::embedded().expect("catalog");
    for asset in catalog.assets.iter().filter(|asset| asset.manufacturer == "ROBOTIS") {
        let (xy, depth, pattern_count, hole_count) = if asset.model.starts_with("XM540") {
            ([13.75, 16.75], Some(4.0), 5, 16)
        } else if asset.model.starts_with("XM430") {
            ([11.25, 14.25], Some(3.0), 4, 14)
        } else {
            ([9.5, 10.0], None, 2, 8)
        };
        assert_eq!(asset.mount_groups.len(), pattern_count);
        assert_eq!(
            asset.mount_groups.iter().map(|group| group.holes.len()).sum::<usize>(),
            hole_count
        );
        for group in &asset.mount_groups {
            assert_eq!(group.frame, "envelope_minimum");
            assert!(group.plane.is_some());
            for hole in &group.holes {
                let thread = hole.thread.as_ref().expect("screw callout");
                assert_eq!(
                    thread.specified_depth_mm.as_ref().map(|dimension| dimension.nominal_mm),
                    depth
                );
                if depth.is_some() {
                    assert!(thread.designation.starts_with("M2.5"));
                    assert_eq!(thread.pitch_mm.as_ref().expect("pitch").nominal_mm, 0.45);
                } else {
                    assert!(thread.designation.contains("tapping"));
                    assert!(thread.pitch_mm.is_none());
                }
            }
        }
        for (output, direction) in asset.output_interfaces.iter().zip([1.0, -1.0]) {
            let axis = output.axis.as_ref().expect("sourced axis");
            assert_eq!([axis.origin_mm[0], axis.origin_mm[1]], xy);
            assert_eq!(axis.direction, [0.0, 0.0, direction]);
        }
        let model = asset.reference_model.as_ref().expect("model");
        assert!(model.cables.is_empty());
        assert!(!model.parts.iter().any(|part| part.id.contains("horn")));
        let contact_count = if asset.model.ends_with("-R") { 4 } else { 3 };
        for port in asset
            .cable_ports
            .iter()
            .filter(|port| !matches!(port.id.as_str(), "external-io" | "dual-joint"))
        {
            assert_eq!(port.contact_count.as_ref().expect("contacts").value, contact_count);
            assert!(port.bundle_count.is_none() && port.conductors_per_bundle.is_none());
        }
    }
}

#[test]
fn partial_output_poses_do_not_invent_shaft_heights_or_cross_register_frames() {
    let catalog = Catalog::embedded().expect("catalog");
    for id in [
        "hitec.hs55.hcs-v2-2",
        "hitec.hs311.hcs-v2-2",
        "towerpro.mg996r.configure",
    ] {
        let asset = catalog.asset(id).expect("asset");
        let interface = &asset.output_interfaces[0];
        assert!(interface.axis.is_some());
        assert!(interface.shaft_top_dimension.is_none());
        let summary = crate::outputs::summary(asset, interface).expect("summary");
        assert_eq!(summary["shaft_top_in_frame"], Value::Null);
        assert_eq!(summary["shaft_protrusion"], Value::Null);
        let plan = crate::outputs::output_recipe(&catalog, &output_params(id)).expect("partial pose recipe");
        assert!(
            plan.as_array()
                .expect("steps")
                .iter()
                .any(|step| step["command"] == "construct.axis.two_points")
        );
        assert!(
            !plan
                .as_array()
                .expect("steps")
                .iter()
                .any(|step| matches!(step["command"].as_str(), Some("solid.box" | "solid.cylinder")))
        );
    }
    let mg996r = catalog.asset("towerpro.mg996r.configure").expect("MG996R");
    assert_eq!(crate::catalog::model_datum_status(mg996r), "preview");
    assert_eq!(
        crate::outputs::summary(mg996r, &mg996r.output_interfaces[0]).expect("summary")["housing_top_in_frame"],
        Value::Null
    );
    assert_eq!(
        crate::catalog::model_datum_status(catalog.asset("hitec.hs311.hcs-v2-2").expect("HS311")),
        "mount_and_axis_datums"
    );
}

#[test]
fn standard_servo_labels_stay_in_front_of_the_whole_model() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut p = model_params();
    p.asset_id = "hitec.hs311.hcs-v2-2".into();
    p.labels = crate::models::Labels::Front;
    let plan = crate::models::model_recipe(&catalog, &p).expect("labelled HS311");
    let sketch = plan
        .as_array()
        .expect("steps")
        .iter()
        .find(|step| step["command"] == "sketch.create")
        .expect("label sketch");
    assert!(sketch["params"]["plane"]["origin"][1].as_f64().expect("Y") <= -12.0);
}

#[test]
fn output_axes_reject_a_named_pattern_from_another_frame() {
    let mut catalog = Catalog::parse(include_str!("../catalog/towerpro-micro.json")).expect("catalog");
    let asset = catalog
        .assets
        .iter_mut()
        .find(|asset| asset.model == "FS90")
        .expect("FS90");
    let mut case_group = asset.mount_groups[0].clone();
    case_group.id = "case-pattern".into();
    case_group.frame = "envelope_minimum".into();
    case_group.plane_height_dimension = Some("mount_underside_height".into());
    asset.mount_groups.push(case_group);
    let pattern = asset.representations.get_mut("mounting-pattern").expect("pattern");
    pattern.frame = "envelope_minimum".into();
    pattern.mount_group = Some("case-pattern".into());
    assert!(catalog.validate().is_err());
}

#[test]
fn detachable_serial_ports_count_contacts_without_creating_attached_wires() {
    let catalog = Catalog::embedded().expect("catalog");
    let assets: Vec<_> = catalog
        .assets
        .iter()
        .filter(|asset| asset.id.starts_with("feetech.sts3215-"))
        .collect();
    assert_eq!(assets.len(), 5);
    for asset in assets {
        assert!(asset.mount_groups.is_empty());
        assert!(asset.output_interfaces.iter().all(|output| output.axis.is_none()));
        assert_eq!(asset.cable_ports.len(), 2);
        for port in &asset.cable_ports {
            assert!(port.attachment.contains("detachable"));
            assert!(port.bundle_count.is_none());
            assert!(port.conductors_per_bundle.is_none());
            assert_eq!(port.contact_count.as_ref().expect("contacts").value, 3);
        }
        let model = asset.reference_model.as_ref().expect("model");
        assert!(model.cables.is_empty());
        let mut p = model_params();
        p.asset_id = asset.id.clone();
        p.labels = crate::models::Labels::None;
        let mut host = Host {
            model: true,
            ..Host::default()
        };
        let reply = crate::models::insert_model(&mut host, &catalog, &p).expect("STS3215");
        assert_eq!(reply["bodies"].as_array().expect("bodies").len(), model.parts.len());
        assert!(
            !reply["body_roles"]
                .as_object()
                .expect("roles")
                .keys()
                .any(|role| role.contains("-wire-"))
        );
        p.include_cable = false;
        assert_eq!(
            crate::models::model_recipe(&catalog, &p).expect("no cable"),
            crate::models::model_recipe(&catalog, &{
                let mut q = p.clone();
                q.include_cable = true;
                q
            })
            .expect("default ports")
        );
    }
}

#[test]
fn opposed_axes_keep_their_direction_without_inventing_reverse_shaft_extents() {
    let mut catalog = Catalog::parse(include_str!("../catalog/towerpro-micro.json")).expect("catalog");
    let asset = catalog
        .assets
        .iter_mut()
        .find(|asset| asset.model == "FS90")
        .expect("FS90");
    let mut rear = asset.output_interfaces[0].clone();
    rear.id = "idler".into();
    rear.axis.as_mut().expect("axis").direction = [0.0, 0.0, -1.0];
    rear.housing_top_dimension = None;
    rear.shaft_top_dimension = None;
    rear.frame_height_dimension = None;
    asset.output_interfaces.push(rear);
    catalog.validate().expect("partial reverse axis");
    let mut p = output_params("feetech.fs90.akizuki-drawing");
    p.output_id = "idler".into();
    p.z_mm = 30.0;
    let plan = crate::outputs::output_recipe(&catalog, &p).expect("idler reference");
    let axis = plan
        .as_array()
        .expect("steps")
        .iter()
        .find(|step| step["command"] == "construct.axis.two_points")
        .expect("datum axis");
    assert_eq!(axis["params"]["b"]["point"][2], 29.0);
    assert!(
        !plan
            .as_array()
            .expect("steps")
            .iter()
            .any(|step| step["command"] == "solid.cylinder")
    );
    let recipe = crate::models::model_recipe(&catalog, &model_params()).expect("two axes");
    let axes: Vec<_> = recipe
        .as_array()
        .expect("steps")
        .iter()
        .filter(|step| step["command"] == "construct.axis.two_points")
        .collect();
    assert_eq!(axes.len(), 2);
    assert_eq!(axes[1]["params"]["b"]["point"][2], -1.0);
    let asset = catalog
        .assets
        .iter_mut()
        .find(|asset| asset.model == "FS90")
        .expect("FS90");
    asset.output_interfaces[1].housing_top_dimension = Some("housing_height".into());
    assert!(catalog.validate().is_err());
}

#[test]
fn horn_dimensions_fit_and_pose_remain_separate_in_saved_instances() {
    let catalog = Catalog::embedded().expect("catalog");
    let horns = crate::horns::HornCatalog::embedded().expect("horn catalog");
    assert_eq!(horns.horns.len(), 18);
    let mut p = model_params();
    p.asset_id = crate::catalog::selector_key("robotis.xm430-w350-t.drawing");
    p.horn_id = Some(crate::catalog::selector_key("robotis.hn12-n101.normal-2019"));
    p.horn_angle_deg = 90.0;
    let mut host = Host {
        model: true,
        ..Host::default()
    };
    let reply = crate::models::insert_model(&mut host, &catalog, &p).expect("documented horn");
    let snapshot = source_snapshot(&host, &reply);
    assert_eq!(snapshot["parameters"]["asset_id"], "robotis.xm430-w350-t.drawing");
    assert_eq!(snapshot["parameters"]["horn_id"], "robotis.hn12-n101.normal-2019");
    let accessory = &snapshot["accessories"][0];
    assert_eq!(accessory["horn"]["status"], "partial_dimensions");
    assert_eq!(accessory["mount"]["compatibility"], "documented");
    assert_eq!(accessory["mount"]["placement_basis"], "documented");
    // Assembly drawing: case front34 + flange projection2, not preview shaft35.2 + plate2.
    assert_eq!(accessory["mount"]["origin_mm"], json!([11.25, 14.25, 36.0]));
    assert!(accessory["sources"].as_array().expect("sources").len() < horns.sources.len());
    let horn_bodies: Vec<_> = reply["body_roles"]
        .as_object()
        .expect("roles")
        .iter()
        .filter(|(role, _)| role.starts_with("horn-"))
        .map(|(_, body)| body.clone())
        .collect();
    let (_, movement) = host
        .calls
        .iter()
        .find(|(command, _)| command == "solid.move")
        .expect("horn pose");
    assert_eq!(movement["bodies"], json!(horn_bodies));
    assert_eq!(movement["copy"], false);
    assert!((movement["matrix"][4].as_f64().expect("sin") - 1.0).abs() < 1e-12);
}

#[test]
fn unknown_horn_positions_and_unverified_fits_never_edit_the_design() {
    let catalog = Catalog::embedded().expect("catalog");
    for (asset, horn) in [
        ("towerpro.sg90-analog.configure", "aus.ta0132.horn-1"),
        ("hitec.hs55.hcs-v2-2", "hitec.fs-il15.rev-d-20230428"),
        ("robotis.xl330-m288-t.brochure", "robotis.hnx330-n101.metal-2022"),
        ("robotis.xm430-w350-t.drawing", "robotis.hn12-i101.idler-2019"),
    ] {
        let mut p = model_params();
        p.asset_id = asset.into();
        p.horn_id = Some(horn.into());
        let mut host = Host {
            model: true,
            ..Host::default()
        };
        assert!(
            crate::models::insert_model(&mut host, &catalog, &p).is_err(),
            "{asset} / {horn}"
        );
        assert!(host.calls.is_empty());
    }
}

#[test]
fn reverse_horn_pose_is_rigid_and_axial_adjustment_moves_outward() {
    let catalog = Catalog::embedded().expect("catalog");
    let horns = crate::horns::HornCatalog::embedded().expect("horns");
    let mount = horns
        .mounts
        .iter()
        .find(|m| m.asset_id == "robotis.xm430-w350-t.drawing" && m.horn_id == "robotis.hn12-i101.idler-2019")
        .expect("documented idler datum")
        .clone();
    // Exercise the transform independently; missing sleeve dimensions still prohibit insertion.
    let selected = crate::horns::SelectedHorn {
        horn: horns.horn(&mount.horn_id).expect("horn").clone(),
        origin_mm: mount.origin_mm.expect("documented datum"),
        mount,
        sources: vec![],
        release: horns.release.clone(),
    };
    let mut p = model_params();
    p.x_mm = 100.0;
    p.y_mm = 200.0;
    p.z_mm = 300.0;
    p.horn_axial_offset_mm = 2.0;
    assert_eq!(
        selected.matrix(&p),
        [1.0, 0.0, 0.0, 111.25, 0.0, -1.0, 0.0, 214.25, 0.0, 0.0, -1.0, 296.0]
    );
    assert!(catalog.asset("robotis.xm430-w350-t.drawing").is_ok());
}

#[test]
fn x330_horn_position_stays_unknown_instead_of_replacing_the_factory_face() {
    let catalog = Catalog::embedded().expect("catalog");
    let horns = crate::horns::HornCatalog::embedded().expect("horns");
    for mount in horns
        .mounts
        .iter()
        .filter(|mount| mount.asset_id == "robotis.xl330-m288-t.brochure")
    {
        assert!(mount.origin_mm.is_none());
        assert!(mount.replace_model_parts.is_empty());
    }
    let mut p = model_params();
    p.asset_id = "robotis.xl330-m288-t.brochure".into();
    let base = crate::models::model_recipe(&catalog, &p).expect("base");
    assert!(
        base.as_array()
            .expect("steps")
            .iter()
            .any(|step| step["bind_role"] == "output-flange")
    );
}

#[test]
fn unsupported_horns_and_invalid_adjustments_are_refused_before_edits() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut p = model_params();
    p.asset_id = "towerpro.sg90-analog.configure".into();
    p.horn_id = Some("aus.ta0132.horn-4".into());
    for (angle, offset) in [(f64::NAN, 0.0), (361.0, 0.0), (0.0, f64::INFINITY), (0.0, 101.0)] {
        p.horn_angle_deg = angle;
        p.horn_axial_offset_mm = offset;
        let mut host = Host {
            model: true,
            ..Host::default()
        };
        assert!(crate::models::insert_model(&mut host, &catalog, &p).is_err());
        assert!(host.calls.is_empty());
    }
    p.horn_angle_deg = 0.0;
    p.horn_axial_offset_mm = 0.0;
    p.horn_id = Some("robotis.hn13-n101.reference-20190318".into());
    let mut host = Host {
        model: true,
        ..Host::default()
    };
    assert!(crate::models::insert_model(&mut host, &catalog, &p).is_err());
    assert!(host.calls.is_empty());
    p.horn_id = None;
    p.horn_angle_deg = 10.0;
    assert!(crate::models::insert_model(&mut host, &catalog, &p).is_err());
    assert!(host.calls.is_empty());
}

#[test]
fn horn_catalog_rejects_assumed_values_in_factual_fields_and_unbacked_claims() {
    let catalog = Catalog::embedded().expect("catalog");
    let original = crate::horns::HornCatalog::embedded().expect("horns");
    for mutation in 0..5 {
        let mut horns = original.clone();
        match mutation {
            0 => {
                horns.horns[0]
                    .dimensions
                    .values_mut()
                    .next()
                    .expect("dimension")
                    .evidence
                    .method = "assumed".into()
            }
            1 => {
                horns.mounts[0].compatibility = "documented".into();
                horns.mounts[0].compatibility_evidence = None;
            }
            2 => horns.mounts[0].origin_mm = Some([0.0, f64::NAN, 0.0]),
            3 => {
                horns
                    .horns
                    .iter_mut()
                    .find(|h| !h.parts.is_empty())
                    .expect("solid")
                    .parts[0]
                    .modifiers[0]
                    .operation = "all_bodies".into()
            }
            _ => horns.mounts[0].replace_model_parts = vec!["housing".into()],
        }
        assert!(horns.validate().is_err());
    }
    let mut horns = original.clone();
    horns.mounts[0].frame = "wrong-frame".into();
    assert!(horns.validate_for_catalog(&catalog).is_err());
    assert!(
        catalog.describe("towerpro.sg90-analog.configure").expect("description")["horns"]
            .as_array()
            .expect("available horns")
            .is_empty()
    );
}

#[test]
fn horn_assemblies_replace_base_callouts_that_describe_an_absent_horn() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut p = model_params();
    p.asset_id = "robotis.xm540-w270-t.reference-20190318".into();
    let bare = crate::models::model_recipe(&catalog, &p).expect("base");
    assert!(bare.as_array().expect("steps").iter().any(|step| {
        step["params"]["text"]
            .as_str()
            .is_some_and(|text| text.to_ascii_lowercase().contains("no fitted horn"))
    }));
    p.horn_id = Some("robotis.hn13-n101.reference-20190318".into());
    for labels in [crate::models::Labels::Notes, crate::models::Labels::Front] {
        p.labels = labels;
        let plan = crate::models::model_recipe(&catalog, &p).expect("assembled labels");
        let texts: Vec<_> = plan
            .as_array()
            .expect("steps")
            .iter()
            .filter_map(|step| step["params"]["text"].as_str())
            .map(str::to_ascii_lowercase)
            .collect();
        assert!(
            !texts
                .iter()
                .any(|text| text.contains("no fitted horn") || text.contains("horn not installed"))
        );
        assert!(
            texts
                .iter()
                .any(|text| text.contains("hn13-n101") && text.contains("documented"))
        );
    }
}

#[test]
fn generated_fallback_body_names_are_persisted_before_binding_and_saving() {
    let catalog = Catalog::embedded().expect("catalog");
    let mut p = model_params();
    p.asset_id = "towerpro.sg90-analog.configure".into();
    p.horn_id = None;
    let mut host = Host {
        model: true,
        fallback_names: true,
        ..Host::default()
    };
    let reply = crate::models::insert_model(&mut host, &catalog, &p).expect("stable body names");
    let snapshot = source_snapshot(&host, &reply);
    assert!(
        reply["bodies"]
            .as_array()
            .expect("bodies")
            .iter()
            .all(|body| body.as_str().expect("name").contains("[servo F"))
    );
    assert_eq!(snapshot["body_roles"], reply["body_roles"]);
    assert_eq!(
        host.calls
            .iter()
            .filter(|(command, _)| command == "body.rename")
            .count(),
        8
    );
    for (command, args) in &host.calls {
        if matches!(command.as_str(), "solid.box" | "solid.cylinder") && args["operation"] != "new" {
            assert!(
                args["targets"]
                    .as_array()
                    .expect("owned targets")
                    .iter()
                    .all(|name| name.as_str().expect("name").contains("[servo F"))
            );
        }
        if command == "solid.move" {
            assert!(
                args["bodies"]
                    .as_array()
                    .expect("owned targets")
                    .iter()
                    .all(|name| name.as_str().expect("name").contains("[servo F"))
            );
        }
    }
}

#[test]
fn photo_annotations_do_not_become_au_dimensions_or_fitted_geometry() {
    let catalog = crate::horns::HornCatalog::embedded().expect("horns");
    let photo = catalog.horn("unidentified.photo-se22ca210.double-arm").expect("photo");
    let au = catalog.horn("aus.ta0132.horn-1").expect("AU");
    assert_eq!(photo.dimensions["reported_length"].nominal_mm, 31.2);
    assert_eq!(photo.dimensions["reported_plate_span"].nominal_mm, 2.36);
    assert_eq!(au.dimensions["overall_length"].nominal_mm, 32.0);
    assert_eq!(au.dimensions["plate_thickness"].nominal_mm, 1.5);
    assert_eq!(photo.status, "photo_estimate");
    assert!(!photo.parts.is_empty() && !photo.estimates.is_empty());
    assert!(photo.holes.is_empty() && au.parts.is_empty());
    assert!(
        !catalog
            .mounts
            .iter()
            .any(|mount| mount.horn_id == photo.id || mount.horn_id == au.id)
    );
    assert!(
        photo
            .dimensions
            .values()
            .all(|dimension| dimension.evidence.method == "photo_annotation" && dimension.tolerance_mm.is_none())
    );
    let mut promoted = catalog.clone();
    promoted
        .horns
        .iter_mut()
        .find(|horn| horn.id == photo.id)
        .expect("photo")
        .status = "partial_dimensions".into();
    assert!(promoted.validate().is_err());
}

#[test]
fn separate_dimension_references_preserve_source_identity_without_creating_solids() {
    let catalog = crate::horns::HornCatalog::embedded().expect("horns");
    for id in ["unidentified.photo-se22ca210.double-arm", "aus.ta0132.horn-1"] {
        let mut params: crate::horns::HornParams =
            serde_json::from_value(json!({"horn_id":crate::catalog::selector_key(id),"x_mm":72,"angle_deg":90}))
                .expect("params");
        let mut host = Host {
            model: true,
            ..Host::default()
        };
        let reply = crate::horns::insert_horn(&mut host, &catalog, &params).expect("source reference");
        assert!(reply["bodies"].as_array().expect("bodies").is_empty());
        assert!(!host.calls.iter().any(|(command, _)| command.starts_with("solid.")));
        let snapshot = source_snapshot(&host, &reply);
        assert_eq!(snapshot["asset_kind"], "servo_horn");
        assert_eq!(snapshot["asset"]["id"], id);
        assert_eq!(snapshot["parameters"]["horn_id"], id);
        let _: crate::horns::HornParams =
            serde_json::from_value(snapshot["parameters"].clone()).expect("saved parameters can replay the same entry");
        assert!(!snapshot["sources"].as_array().expect("sources").is_empty());
        params.mode = "source-solid".into();
        let mut refused = Host {
            model: true,
            ..Host::default()
        };
        assert!(crate::horns::insert_horn(&mut refused, &catalog, &params).is_err());
        assert!(refused.calls.is_empty());
    }
}

#[test]
fn hitec_disc_omits_unspecified_oblique_holes_and_nominal_spline_surrogate() {
    let catalog = crate::horns::HornCatalog::embedded().expect("horns");
    let horn = catalog.horn("hitec.m-o24.rev-d-20230428").expect("disc");
    let holes: Vec<_> = horn
        .parts
        .iter()
        .flat_map(|part| part.modifiers.iter())
        .filter_map(|modifier| {
            if modifier.operation != "cut" {
                return None;
            }
            if let crate::models::Primitive::Cylinder {
                base_mm, diameter_mm, ..
            } = &modifier.geometry
            {
                if base_mm[0] != 0.0 || base_mm[1] != 0.0 {
                    return Some(([base_mm[0], base_mm[1]], *diameter_mm));
                }
                assert!(
                    !(*diameter_mm == 6.0 && base_mm[2] < -4.0),
                    "nominal H24T class is not a round rear bore"
                );
            }
            None
        })
        .collect();
    assert_eq!(
        holes,
        vec![
            ([-6.0, 0.0], 1.2),
            ([6.0, 0.0], 1.2),
            ([0.0, -6.5], 1.2),
            ([0.0, 6.5], 1.2)
        ]
    );
}

#[test]
fn photo_horns_create_tapered_solids_with_replayable_estimate_evidence() {
    let catalog = crate::horns::HornCatalog::embedded().expect("horns");
    for (id, count, length, plate) in [
        ("unidentified.photo-se22ca210.double-arm", 12, 31.2, 2.36),
        ("unidentified.photo-se22ca210.single-arm", 7, 21.3, 2.08),
    ] {
        let horn = catalog.horn(id).expect("photo variant");
        assert_eq!(horn.dimensions["reported_length"].nominal_mm, length);
        assert_eq!(horn.estimates["link_hole_count"].value, f64::from(count));
        assert_eq!(horn.estimates["link_hole_diameter"].value, 1.45);
        let crate::models::Primitive::Profile {
            height_mm, segments, ..
        } = &horn.parts[0].geometry
        else {
            panic!("a tapered line/arc profile is required")
        };
        assert_eq!(*height_mm, plate);
        assert!(
            segments
                .iter()
                .any(|segment| matches!(segment, crate::models::ProfileSegment::Arc { .. }))
        );
        assert!(segments.iter().any(|segment| matches!(segment,crate::models::ProfileSegment::Line {start_mm,end_mm} if (end_mm[0]-start_mm[0]).abs()>1.0 && (end_mm[1]-start_mm[1]).abs()>0.1)));
        let params: crate::horns::HornParams =
            serde_json::from_value(json!({"horn_id":id,"mode":"photo-solid","x_mm":50})).expect("params");
        let mut host = Host {
            model: true,
            fallback_names: true,
            sequential_sketch_ids: true,
            ..Host::default()
        };
        let reply = crate::horns::insert_horn(&mut host, &catalog, &params).expect("photo reconstruction");
        assert_eq!(reply["bodies"].as_array().expect("bodies").len(), 1);
        let snapshot = source_snapshot(&host, &reply);
        assert_eq!(snapshot["geometry_basis"], "photo_reconstructed_estimated_geometry");
        assert_eq!(snapshot["asset"]["status"], "photo_estimate");
        assert_eq!(
            snapshot["asset"]["estimates"]["link_hole_count"]["value"].as_f64(),
            Some(f64::from(count))
        );
        assert!(reply["warning"].as_str().expect("warning").contains("PHOTO ESTIMATE"));
        let _: crate::horns::HornParams = serde_json::from_value(snapshot["parameters"].clone()).expect("replay");
        let first_sketch = host
            .calls
            .iter()
            .position(|(command, _)| command == "sketch.create")
            .expect("profile sketch")
            + 1;
        let (_, extrude) = host
            .calls
            .iter()
            .find(|(command, _)| command == "solid.extrude")
            .expect("extrusion");
        assert_eq!(extrude["sketch"], first_sketch);
        let owned = &reply["bodies"][0];
        for (command, args) in &host.calls {
            if matches!(command.as_str(), "solid.cylinder" | "solid.extrude") && args["operation"] != "new" {
                assert_eq!(args["targets"], json!([owned]));
            }
        }
    }
}

#[test]
fn malformed_photo_profiles_and_unbacked_estimates_are_rejected_before_edits() {
    let original = crate::horns::HornCatalog::embedded().expect("horns");
    for mode in 0..5 {
        let mut catalog = original.clone();
        let horn = catalog
            .horns
            .iter_mut()
            .find(|horn| horn.status == "photo_estimate")
            .expect("photo");
        match mode {
            0 => horn.estimates.values_mut().next().expect("estimate").value = f64::NAN,
            1 => horn.estimates.values_mut().next().expect("estimate").evidence.source = "unknown-source".into(),
            2 => horn.parts[0].basis = "dimensioned".into(),
            3 => {
                let crate::models::Primitive::Profile { segments, .. } = &mut horn.parts[0].geometry else {
                    panic!("profile")
                };
                if let crate::models::ProfileSegment::Line { end_mm, .. } = &mut segments[0] {
                    end_mm[0] += 1.0;
                }
            }
            _ => horn.estimates.clear(),
        }
        let params: crate::horns::HornParams =
            serde_json::from_value(json!({"horn_id":horn.id,"mode":"photo-solid"})).expect("params");
        let mut host = Host {
            model: true,
            ..Host::default()
        };
        assert!(crate::horns::insert_horn(&mut host, &catalog, &params).is_err());
        assert!(host.calls.is_empty());
    }
}
