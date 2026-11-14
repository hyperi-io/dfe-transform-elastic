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
            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = { event.has_value("message") };
            if _cond {
                parse_json_field(event, "message", "parsed_email")?;
            }

            if event.has_value("parsed_email") {
                event.rename("parsed_email", "proofpoint.email")?;
            }

            event.remove("message");

            let _cond = {
                event
                    .get("proofpoint.email.size")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "bdfc7b2b5fba2ca0503f32dfdd96bae1240c0e9b7836505ec38b193e83c17183",
                            r#"{"params":null,"known":[{"ConcatAssignment":{"target":"proofpoint.email.size","terms":[{"Field":"proofpoint.email.size.value"},{"Literal":" "},{"Field":"proofpoint.email.size.unit"}]}}]}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_size_object_to_string",
                    )?;
                    event.append(
                        "error.message",
                        json!("Failed to convert size object to string"),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.get("proofpoint.email.attachments").is_some_and(|v| v.is_array()) && event.get("proofpoint.email.attachments").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "1b966c2ab9a0fda4ee874724d42ef5d6dce40a24a0460a363e0db2dc9cba022f",
                            r#"{"params":null,"known":[]}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "process_attachments_to_ecs",
                    )?;
                    event.append(
                        "error.message",
                        json!("Failed to process attachments to ECS format"),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { !event.has_value("proofpoint.email.attachmentsList") };
            if _cond {
                event.set("proofpoint.email.attachments", json!("no"))?;
            }

            let _cond = { !event.has_value("proofpoint.email.attachmentsList") };
            if _cond {
                event.set("proofpoint.email.attachmentsList", Value::Array(vec![]))?;
            }

            let _cond = { !event.has_value("proofpoint.email.attachmentsExtensions") };
            if _cond {
                event.set(
                    "proofpoint.email.attachmentsExtensions",
                    Value::Array(vec![]),
                )?;
            }

            let _cond = { !event.has_value("proofpoint.email.msg_id") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.has_value("proofpoint.email.msg_id") };
            if _cond {
                if let Some(v) = event.get("proofpoint.email.msg_id").cloned() {
                    event.set("_id", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("proofpoint.email.date") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss", "ISO8601"], None, None)
                    {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "proofpoint.email.date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event
                    .get("proofpoint.email.classification")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "2cdbe0a1b0d047d28842c5c4d83d86f947393a311ee75837375f2f7c790370f6",
                            r#"{"params":null,"known":[{"PlainAssignments":{"statements":[{"If":{"test":[[{"ContainsKey":{"path":"proofpoint.email.classification","key":"text"}}]],"then":[{"Literal":{"Set":{"path":"proofpoint.email.classification_text","value":{"Field":"proofpoint.email.classification.text"}}}},{"Literal":{"Set":{"path":"proofpoint.email.classification_id","value":{"Field":"proofpoint.email.classification.id"}}}}],"alt":[]}}],"whole":true}}]}"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint.email.status")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "766f8a06d412ffc22855c2d8fe1251cf6ec4eee792f6ba123b3f5f9544bb4750",
                            r#"{"params":null,"known":[{"PlainAssignments":{"statements":[{"If":{"test":[[{"ContainsKey":{"path":"proofpoint.email.status","key":"text"}}]],"then":[{"Literal":{"Set":{"path":"proofpoint.email.status_text","value":{"Field":"proofpoint.email.status.text"}}}},{"Literal":{"Set":{"path":"proofpoint.email.status_id","value":{"Field":"proofpoint.email.status.id"}}}}],"alt":[]}}],"whole":true}}]}"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("proofpoint.email.direction") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "eacae9846fb4d607e299f53fa8c977dd156792a7c6d9caa42e1fb34bb4623e07",
                            r#"{"params":null,"known":[{"BandLadder":{"subject":"proofpoint.email.direction","arms":[[{"bounds":[{"op":"Eq","scaled":1000}],"all":true},{"Text":"inbound"}],[{"bounds":[{"op":"Eq","scaled":2000}],"all":true},{"Text":"outbound"}]],"default":{"Text":"unknown"},"absent":null,"target":"proofpoint.email.direction_text"}}]}"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint.email.crypt_type_in")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "3c4d9d3315a55b5ea6011e14d944288188db651b0d9157b2585e812cd2e00346",
                            r#"{"params":null,"known":[{"PlainAssignments":{"statements":[{"If":{"test":[[{"ContainsKey":{"path":"proofpoint.email.crypt_type_in","key":"text"}}]],"then":[{"Literal":{"Set":{"path":"proofpoint.email.incoming_encryption","value":{"Field":"proofpoint.email.crypt_type_in.text"}}}}],"alt":[]}}],"whole":true}}]}"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("proofpoint.email.crypt_type_out")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "833058baa2e1d32da976cfd4a7254368e8c03b5472afb33232b5d01c376c2297",
                            r#"{"params":null,"known":[{"PlainAssignments":{"statements":[{"If":{"test":[[{"ContainsKey":{"path":"proofpoint.email.crypt_type_out","key":"text"}}]],"then":[{"Literal":{"Set":{"path":"proofpoint.email.outgoing_encryption","value":{"Field":"proofpoint.email.crypt_type_out.text"}}}}],"alt":[]}}],"whole":true}}]}"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("proofpoint.email.owner") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "a5ffacd94db3f0b2666f0c7977feaf458e0519d6c1bead284a13d87e2c8e3b41",
                            r#"{"params":null,"known":[{"GuardedCopy":{"statements":[{"If":{"test":[[{"Compare":{"path":"proofpoint.email.direction","wanted":"Null","negated":true,"lowered":false}},{"Compare":{"path":"proofpoint.email.direction","wanted":{"Value":1},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"proofpoint.email.to","value":{"Field":"proofpoint.email.owner"}}}},{"If":{"test":[[{"ContainsKey":{"path":"proofpoint.email","key":"comm_partner"}},{"Compare":{"path":"proofpoint.email.comm_partner","wanted":"Null","negated":true,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"proofpoint.email.from","value":{"Field":"proofpoint.email.comm_partner"}}}}],"alt":[]}}],"alt":[{"If":{"test":[[{"Compare":{"path":"proofpoint.email.direction","wanted":"Null","negated":true,"lowered":false}},{"Compare":{"path":"proofpoint.email.direction","wanted":{"Value":2},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"proofpoint.email.from","value":{"Field":"proofpoint.email.owner"}}}},{"If":{"test":[[{"ContainsKey":{"path":"proofpoint.email","key":"comm_partner"}},{"Compare":{"path":"proofpoint.email.comm_partner","wanted":"Null","negated":true,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"proofpoint.email.to","value":{"Field":"proofpoint.email.comm_partner"}}}}],"alt":[]}}],"alt":[]}}]}}],"whole":false}}]}"#,
                            r#"def direction = ctx.proofpoint.email.containsKey('direction') ? ctx.proofpoint.email.direction : null;\nif (direction != null && direction == 1) {\n    // Inbound: owner is recipient, comm_partner is sender\n    ctx.proofpoint.email.to = ctx.proofpoint.email.owner;\n    if (ctx.proofpoint.email.containsKey('comm_partner') && ctx.proofpoint.email.comm_partner != null) {\n        ctx.proofpoint.email.from = ctx.proofpoint.email.comm_partner;\n    }\n} else if (direction != null && direction == 2) {\n    // Outbound: owner is sender, comm_partner is recipient\n    ctx.proofpoint.email.from = ctx.proofpoint.email.owner;\n    if (ctx.proofpoint.email.containsKey('comm_partner') && ctx.proofpoint.email.comm_partner != null) {\n        ctx.proofpoint.email.to = ctx.proofpoint.email.comm_partner;\n    }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("proofpoint.email.from") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("proofpoint.email.from")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("proofpoint.email.to") };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("proofpoint.email.to")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("proofpoint.email.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if let Some(v) = event
                .get("proofpoint.email.message_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.message_id", v)?;
            }

            if let Some(v) = event
                .get("proofpoint.email.direction_text")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.direction", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("proofpoint.email.source_ip") {
                    if let Some(val) = event.get("proofpoint.email.source_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "proofpoint.email.source_ip".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint.email.source_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_ip_proofpoint",
                )?;
                if event.remove("proofpoint.email.source_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "proofpoint.email.source_ip".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("proofpoint.email.destination_ip") {
                    if let Some(val) = event.get("proofpoint.email.destination_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "proofpoint.email.destination_ip".into(),
                                message,
                            }
                        })?;
                        event.set("proofpoint.email.destination_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_ip_proofpoint",
                )?;
                if event.remove("proofpoint.email.destination_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "proofpoint.email.destination_ip".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event
                .get("proofpoint.email.source_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event
                .get("proofpoint.email.destination_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("email")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { event.has_value("proofpoint.email.source_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint.email.source_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("proofpoint.email.destination_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint.email.destination_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("proofpoint.email.from") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint.email.from")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("proofpoint.email.to") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint.email.to")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("proofpoint.email.smtp_dialog") };
            if _cond {
                event.remove("proofpoint.email.smtp_dialog");
            }

            let _cond = { !event.has_value("proofpoint.email.smtp_status") };
            if _cond {
                event.remove("proofpoint.email.smtp_status");
            }

            let _cond = { !event.has_value("proofpoint.email.smtp_status_code") };
            if _cond {
                event.remove("proofpoint.email.smtp_status_code");
            }

            event.remove("message");

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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
