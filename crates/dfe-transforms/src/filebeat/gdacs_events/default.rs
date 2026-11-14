// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field_to_root(event, "event.original", false)?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("event.original");
            }

            if event.has_value("properties.eventid") {
                event.rename("properties.eventid", "gdacs.event_id")?;
            }

            if event.has_value("properties.episodeid") {
                event.rename("properties.episodeid", "gdacs.episode_id")?;
            }

            if event.has_value("properties.eventtype") {
                event.rename("properties.eventtype", "gdacs.event_type")?;
            }

            if event.has_value("properties.eventname") {
                event.rename("properties.eventname", "gdacs.event_name")?;
            }

            if event.has_value("properties.name") {
                event.rename("properties.name", "gdacs.name")?;
            }

            if event.has_value("properties.description") {
                event.rename("properties.description", "gdacs.description")?;
            }

            if event.has_value("properties.htmldescription") {
                event.rename("properties.htmldescription", "gdacs.html_description")?;
            }

            if event.has_value("properties.glide") {
                event.rename("properties.glide", "gdacs.glide")?;
            }

            if event.has_value("properties.country") {
                event.rename("properties.country", "gdacs.country")?;
            }

            if event.has_value("properties.iso3") {
                event.rename("properties.iso3", "gdacs.iso3")?;
            }

            if event.has_value("properties.alertlevel") {
                event.rename("properties.alertlevel", "gdacs.alert_level")?;
            }

            if event.has_value("properties.alertscore") {
                event.rename("properties.alertscore", "gdacs.alert_score")?;
            }

            if event.has_value("properties.episodealertlevel") {
                event.rename("properties.episodealertlevel", "gdacs.episode_alert_level")?;
            }

            if event.has_value("properties.episodealertscore") {
                event.rename("properties.episodealertscore", "gdacs.episode_alert_score")?;
            }

            if event.has_value("properties.iscurrent") {
                event.rename("properties.iscurrent", "gdacs.is_current")?;
            }

            if event.has_value("properties.istemporary") {
                event.rename("properties.istemporary", "gdacs.is_temporary")?;
            }

            if event.has_value("properties.source") {
                event.rename("properties.source", "gdacs.source")?;
            }

            if event.has_value("properties.sourceid") {
                event.rename("properties.sourceid", "gdacs.source_id")?;
            }

            if event.has_value("properties.Class") {
                event.rename("properties.Class", "gdacs.class")?;
            }

            if event.has_value("properties.polygonlabel") {
                event.rename("properties.polygonlabel", "gdacs.polygon_label")?;
            }

            if event.has_value("geometry_doc") {
                event.rename("geometry_doc", "gdacs.geometry_doc")?;
            }

            if event.has_value("geometry_id") {
                event.rename("geometry_id", "gdacs.geometry_id")?;
            }

            if event.has_value("geometry_role") {
                event.rename("geometry_role", "gdacs.geometry_role")?;
            }

            if event.has_value("properties.icon") {
                event.rename("properties.icon", "gdacs.icon")?;
            }

            if event.has_value("properties.url.report") {
                event.rename("properties.url.report", "gdacs.url.report")?;
            }

            if event.has_value("properties.url.details") {
                event.rename("properties.url.details", "gdacs.url.details")?;
            }

            if event.has_value("properties.url.geometry") {
                event.rename("properties.url.geometry", "gdacs.url.geometry")?;
            }

            if event.has_value("properties.severitydata") {
                event.rename("properties.severitydata", "gdacs.severity_raw")?;
            }

            if event.has_value("properties.affectedcountries") {
                event.rename("properties.affectedcountries", "gdacs.affected_countries")?;
            }

            if event.has_value("gdacs.severity_raw.severity") {
                event.rename("gdacs.severity_raw.severity", "gdacs.severity.value")?;
            }

            if event.has_value("gdacs.severity_raw.severitytext") {
                event.rename("gdacs.severity_raw.severitytext", "gdacs.severity.text")?;
            }

            if event.has_value("gdacs.severity_raw.severityunit") {
                event.rename("gdacs.severity_raw.severityunit", "gdacs.severity.unit")?;
            }

            event.remove("gdacs.severity_raw");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("properties.fromdate") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd'T'HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "properties.fromdate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_fromdate")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("properties.todate") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd'T'HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "properties.todate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_todate")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("properties.datemodified") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd'T'HH:mm:ss", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("event.modified", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "properties.datemodified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_datemodified")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event
                .get("event.modified")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { !event.has_value("event.modified") };
            if _cond {
                if let Some(v) = event
                    .get("event.start")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            // The pipeline's script, emitted as the runtime's equivalent
            coerce_boolean(
                event,
                &CoerceBoolean::new(
                    vec![
                        ("gdacs.is_current".to_owned(), "gdacs.is_current".to_owned()),
                        (
                            "gdacs.is_temporary".to_owned(),
                            "gdacs.is_temporary".to_owned(),
                        ),
                    ],
                    vec!["true".to_owned(), "1".to_owned()],
                ),
            );

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "f4a44fe7332da495c7d0236f5d2fb62846e40fd8e7ef371e07d222789b22c51f",
                    r#"{"params":null,"known":[{"WktGeometry":{"geometry":"polygon_geometry","target":"gdacs.affected_area","type_target":"gdacs.geometry_type","point":{"coordinates":"geometry.coordinates","target":"gdacs.geo.location","lon_index":0,"lat_index":1,"lon_first":true},"branches":[{"guard":"polygon_geometry.type","literal":"Polygon","copies":[["gdacs.class","polygon_class"],["gdacs.polygon_label","polygon_label"]]},{"guard":"polygon_geometry.type","literal":"MultiPolygon","copies":[["gdacs.class","polygon_class"],["gdacs.polygon_label","polygon_label"]]},{"guard":"polygon_geometry.type","literal":"LineString","copies":[["gdacs.class","polygon_class"],["gdacs.polygon_label","polygon_label"]]},{"guard":"polygon_geometry.type","literal":"MultiLineString","copies":[["gdacs.class","polygon_class"],["gdacs.polygon_label","polygon_label"]]}]}}]}"#
                ),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "880445b9af56578681d54deda6d3cee5c6e78018b404d75f06107835d47bcfa6",
                    r#"{"params":null,"known":[{"CollectFromList":{"source":"gdacs.affected_countries","columns":[{"member":"countryname","target":"gdacs.affected_country_names","replace":null,"upper":false},{"member":"iso2","target":"gdacs.affected_country_iso2","replace":null,"upper":false},{"member":"iso3","target":"gdacs.affected_country_iso3","replace":null,"upper":false}],"firsts":[["gdacs.geo.country_iso_code","gdacs.affected_country_iso2"],["gdacs.geo.country_name","gdacs.affected_country_names"]]}}]}"#
                ),
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gdacs.severity.value") {
                    if let Some(val) = event.get("gdacs.severity.value") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.severity.value".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.severity.value", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_severity_value")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gdacs.alert_score") {
                    if let Some(val) = event.get("gdacs.alert_score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.alert_score".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.alert_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_alert_score")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gdacs.episode_alert_score") {
                    if let Some(val) = event.get("gdacs.episode_alert_score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.episode_alert_score".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.episode_alert_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_episode_alert_score",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gdacs.event_id") {
                    if let Some(val) = event.get("gdacs.event_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.event_id".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.event_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_event_id")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("gdacs.episode_id") {
                    if let Some(val) = event.get("gdacs.episode_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "gdacs.episode_id".into(),
                                message,
                            }
                        })?;
                        event.set("gdacs.episode_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_episode_id")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "c1d1c9cdb6d1ae48f020876eb90f3031658f195dec09fc033b516c4dd20b79a8",
                    r#"{"params":null,"known":[{"LocalMapLookup":{"source":"gdacs.event_type","lowered":false,"target":"gdacs.event_type_name","table":{"DR":"Drought","EQ":"Earthquake","FL":"Flood","TC":"Tropical Cyclone","VO":"Volcano","WF":"Wildfire"}}}]}"#
                ),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "0b93625afd382840bee5f9a841b31f0245651659ddf719a7eaf28e2ff6d46488",
                    r#"{"params":null,"known":[{"EqualityLadder":{"subject":"gdacs.alert_level","fold_case":true,"test":"Equals","arms":[{"literals":["red"],"writes":[["event.severity",3],["event.risk_score",90.0]],"removes":[]},{"literals":["orange"],"writes":[["event.severity",2],["event.risk_score",60.0]],"removes":[]},{"literals":["green"],"writes":[["event.severity",1],["event.risk_score",30.0]],"removes":[]}]}}]}"#
                ),
            )?;

            event.set("ecs.version", json!("9.4.0"))?;

            event.set("event.kind", json!("alert"))?;

            event.set("event.dataset", json!("gdacs.events"))?;

            event.set("event.module", json!("gdacs"))?;

            event.set("event.provider", json!("gdacs"))?;

            // The pipeline's script, emitted as the runtime's equivalent
            join_present_fields(
                event,
                &JoinPresentFields::new(
                    vec![
                        "gdacs.event_id".to_owned(),
                        "gdacs.episode_id".to_owned(),
                        "gdacs.geometry_id".to_owned(),
                    ],
                    "-",
                    "event.id",
                ),
            );

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if let Some(v) = event
                .get("gdacs.url.report")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            if let Some(v) = event
                .get("gdacs.url.details")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "message",
                    json!(
                        event
                            .get("gdacs.description")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if let Some(v) = event
                .get("gdacs.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("gdacs.geo.name", v)?;
            }

            event.remove("properties");
            event.remove("geometry");
            event.remove("bbox");
            event.remove("type");
            event.remove("polygon_geometry");
            event.remove("polygon_class");
            event.remove("polygon_label");
            event.remove("geometry_doc");
            event.remove("geometry_id");
            event.remove("geometry_role");
            event.remove("gdacs.geometry_type");
            event.remove("gdacs.geometry_doc");
            event.remove("gdacs.geometry_id");

            // The pipeline's script, emitted as the runtime's equivalent
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
