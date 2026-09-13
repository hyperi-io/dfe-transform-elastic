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
                    // Painless script
                    // Source: ctx.proofpoint.email.size = ctx.proofpoint.email.size.value + ' ' + ctx.proofpoint.email.size.unit;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.proofpoint.email.size = ctx.proofpoint.email.size.value + ' ' + ctx.proofpoint.email.size.unit;\n"#
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
                    // Painless script
                    // Source: def attachmentNames = [];\ndef attachmentExtensions = [];\ndef ecsAttachments = [];\nfor (att in ctx.proofpoint.email.attachments) {\n    if (att instanceof Map && att.containsKey('name')) {\n        def filename = att.name;\n        attachmentNames.add(filename);\n        \n        // Extract file extension\n        def lastDot = filename.lastIndexOf('.');\n        def ext = null;\n        if (lastDot > 0 && lastDot < filename.length() - 1) {\n            ext = filename.substring(lastDot + 1).toLowerCase();\n            if (!attachmentExtensions.contains(ext)) {\n                attachmentExtensions.add(ext);\n            }\n        }\n        \n        // Create ECS attachment object\n        def ecsAtt = ['file': ['name': filename]];\n        if (ext != null) {\n            ecsAtt.file.extension = ext;\n        }\n        if (att.containsKey('size')) {\n            ecsAtt.file.size = att.size;\n        }\n        ecsAttachments.add(ecsAtt);\n    }\n}\nctx.proofpoint.email.attachmentsList = attachmentNames;\nctx.proofpoint.email.attachmentsExtensions = attachmentExtensions;\nctx.proofpoint.email.attachments = attachmentNames.size() > 0 ? 'yes' : 'no';\nif (!ctx.containsKey('email')) {\n    ctx.email = [:];\n}\nctx.email.attachments = ecsAttachments;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def attachmentNames = [];\ndef attachmentExtensions = [];\ndef ecsAttachments = [];\nfor (att in ctx.proofpoint.email.attachments) {\n    if (att instanceof Map && att.containsKey('name')) {\n        def filename = att.name;\n        attachmentNames.add(filename);\n        \n        // Extract file extension\n        def lastDot = filename.lastIndexOf('.');\n        def ext = null;\n        if (lastDot > 0 && lastDot < filename.length() - 1) {\n            ext = filename.substring(lastDot + 1).toLowerCase();\n            if (!attachmentExtensions.contains(ext)) {\n                attachmentExtensions.add(ext);\n            }\n        }\n        \n        // Create ECS attachment object\n        def ecsAtt = ['file': ['name': filename]];\n        if (ext != null) {\n            ecsAtt.file.extension = ext;\n        }\n        if (att.containsKey('size')) {\n            ecsAtt.file.size = att.size;\n        }\n        ecsAttachments.add(ecsAtt);\n    }\n}\nctx.proofpoint.email.attachmentsList = attachmentNames;\nctx.proofpoint.email.attachmentsExtensions = attachmentExtensions;\nctx.proofpoint.email.attachments = attachmentNames.size() > 0 ? 'yes' : 'no';\nif (!ctx.containsKey('email')) {\n    ctx.email = [:];\n}\nctx.email.attachments = ecsAttachments;\n"#
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
                    // Painless script
                    // Source: if (ctx.proofpoint.email.classification.containsKey('text')) {\n    ctx.proofpoint.email.classification_text = ctx.proofpoint.email.classification.text;\n    ctx.proofpoint.email.classification_id = ctx.proofpoint.email.classification.id;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.proofpoint.email.classification.containsKey('text')) {\n    ctx.proofpoint.email.classification_text = ctx.proofpoint.email.classification.text;\n    ctx.proofpoint.email.classification_id = ctx.proofpoint.email.classification.id;\n}\n"#
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
                    // Painless script
                    // Source: if (ctx.proofpoint.email.status.containsKey('text')) {\n    ctx.proofpoint.email.status_text = ctx.proofpoint.email.status.text;\n    ctx.proofpoint.email.status_id = ctx.proofpoint.email.status.id;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.proofpoint.email.status.containsKey('text')) {\n    ctx.proofpoint.email.status_text = ctx.proofpoint.email.status.text;\n    ctx.proofpoint.email.status_id = ctx.proofpoint.email.status.id;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("proofpoint.email.direction") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def dir = ctx.proofpoint.email.direction;\n// Set direction text based on integer value\nif (dir == 1) {\n    ctx.proofpoint.email.direction_text = 'inbound';\n} else if (dir == 2) {\n    ctx.proofpoint.email.direction_text = 'outbound';\n} else {\n    ctx.proofpoint.email.direction_text = 'unknown';\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def dir = ctx.proofpoint.email.direction;\n// Set direction text based on integer value\nif (dir == 1) {\n    ctx.proofpoint.email.direction_text = 'inbound';\n} else if (dir == 2) {\n    ctx.proofpoint.email.direction_text = 'outbound';\n} else {\n    ctx.proofpoint.email.direction_text = 'unknown';\n}\n"#
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
                    // Painless script
                    // Source: if (ctx.proofpoint.email.crypt_type_in.containsKey('text')) {\n    ctx.proofpoint.email.incoming_encryption = ctx.proofpoint.email.crypt_type_in.text;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.proofpoint.email.crypt_type_in.containsKey('text')) {\n    ctx.proofpoint.email.incoming_encryption = ctx.proofpoint.email.crypt_type_in.text;\n}\n"#
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
                    // Painless script
                    // Source: if (ctx.proofpoint.email.crypt_type_out.containsKey('text')) {\n    ctx.proofpoint.email.outgoing_encryption = ctx.proofpoint.email.crypt_type_out.text;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.proofpoint.email.crypt_type_out.containsKey('text')) {\n    ctx.proofpoint.email.outgoing_encryption = ctx.proofpoint.email.crypt_type_out.text;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("proofpoint.email.owner") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def direction = ctx.proofpoint.email.containsKey('direction') ? ctx.proofpoint.email.direction : null;\nif (direction != null && direction == 1) {\n    // Inbound: owner is recipient, comm_partner is sender\n    ctx.proofpoint.email.to = ctx.proofpoint.email.owner;\n    if (ctx.proofpoint.email.containsKey('comm_partner') && ctx.proofpoint.email.comm_partner != null) {\n        ctx.proofpoint.email.from = ctx.proofpoint.email.comm_partner;\n    }\n} else if (direction != null && direction == 2) {\n    // Outbound: owner is sender, comm_partner is recipient\n    ctx.proofpoint.email.from = ctx.proofpoint.email.owner;\n    if (ctx.proofpoint.email.containsKey('comm_partner') && ctx.proofpoint.email.comm_partner != null) {\n        ctx.proofpoint.email.to = ctx.proofpoint.email.comm_partner;\n    }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
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
