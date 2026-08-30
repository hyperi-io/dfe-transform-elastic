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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("alert"))?;

            event.set("event.category", Value::Array(vec![json!("malware")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "json")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.guid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastUpdateTime") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.@class") {
                event.rename("json.@class", "cybereason.poll_malop.class")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.closed") {
                    if let Some(val) = event.get("json.closed") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.closed".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.poll_malop.closed", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_closed_to_boolean",
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

            if event.has_value("json.closerName") {
                event.rename("json.closerName", "cybereason.poll_malop.closer_name")?;
            }

            if event.has_value("json.containers") {
                event.rename("json.containers", "cybereason.poll_malop.containers")?;
            }

            let _cond = {
                event.has_value("json.creationTime")
                    && event.get_str("json.creationTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.creationTime") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("cybereason.poll_malop.creation_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.creationTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_creationTime")?;
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
            }

            if let Some(v) = event
                .get("cybereason.poll_malop.creation_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            let _cond = {
                event.has_value("json.malopCloseTime")
                    && event.get_str("json.malopCloseTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.malopCloseTime") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("cybereason.poll_malop.data.close_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.malopCloseTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_malopCloseTime")?;
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
            }

            if event.has_value("json.malopDetectionType") {
                event.rename(
                    "json.malopDetectionType",
                    "cybereason.poll_malop.data.detection_type",
                )?;
            }

            if event.has_value("json.malopPriority") {
                event.rename("json.malopPriority", "cybereason.poll_malop.data.priority")?;
            }

            if event.has_value("json.malopSeverity") {
                event.rename("json.malopSeverity", "cybereason.poll_malop.data.severity")?;
            }

            if event.has_value("json.malopStatus") {
                event.rename("json.malopStatus", "cybereason.poll_malop.data.status")?;
            }

            if event.has_value("json.malopType") {
                event.rename("json.malopType", "cybereason.poll_malop.data.type")?;
            }

            if event.has_value("json.decisionStatuses") {
                event.rename(
                    "json.decisionStatuses",
                    "cybereason.poll_malop.decision_statuses",
                )?;
            }

            if event.has_value("json.detectionEngines") {
                event.rename(
                    "json.detectionEngines",
                    "cybereason.poll_malop.detection.engines",
                )?;
            }

            if event.has_value("json.detectionTypes") {
                event.rename(
                    "json.detectionTypes",
                    "cybereason.poll_malop.detection.types",
                )?;
            }

            if event.has_value("json.displayName") {
                event.rename("json.displayName", "cybereason.poll_malop.display_name")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.edr") {
                    if let Some(val) = event.get("json.edr") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.edr".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.poll_malop.edr", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_edr_to_boolean")?;
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
                if event.has_value("json.empty") {
                    if let Some(val) = event.get("json.empty") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.empty".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.poll_malop.empty", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_empty_to_boolean",
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
                if event.has_value("json.escalated") {
                    if let Some(val) = event.get("json.escalated") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.escalated".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.poll_malop.escalated", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_escalated_to_boolean",
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

            if event.has_value("json.files") {
                event.rename("json.files", "cybereason.poll_malop.files")?;
            }

            if event.has_value("json.group") {
                event.rename("json.group", "cybereason.poll_malop.group")?;
            }

            event.append_unique(
                "group.id",
                json!(
                    event
                        .get("cybereason.poll_malop.group")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.guid") {
                event.rename("json.guid", "cybereason.poll_malop.guid")?;
            }

            if let Some(v) = event
                .get("cybereason.poll_malop.guid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.iconBase64") {
                    if let Some(val) = event.get("json.iconBase64") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.iconBase64".into(),
                                message,
                            }
                        })?;
                        event.set("cybereason.poll_malop.icon_base64", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_iconBase64_to_string",
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

            if event.has_value("json.labels") {
                event.rename("json.labels", "cybereason.poll_malop.labels")?;
            }

            let _cond = {
                event.has_value("json.lastUpdateTime")
                    && event.get_str("json.lastUpdateTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastUpdateTime") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("cybereason.poll_malop.last_update_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.lastUpdateTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastUpdateTime")?;
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
            }

            if let Some(v) = event
                .get("cybereason.poll_malop.last_update_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        if event.has_value("_ingest._value.@class") {
                            event.rename("_ingest._value.@class", "_ingest._value.class")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.connected") {
                                if let Some(val) = event.get("_ingest._value.connected") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.connected".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.connected", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_machines_connected_to_boolean",
                            )?;
                            event.remove("_ingest._value.connected");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        if event.has_value("_ingest._value.displayName") {
                            event.rename(
                                "_ingest._value.displayName",
                                "_ingest._value.display_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.display_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        event.append_unique(
                            "host.name",
                            json!(
                                event
                                    .get("_ingest._value.display_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.empty") {
                                if let Some(val) = event.get("_ingest._value.empty") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.empty".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.empty", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_machines_empty_to_boolean",
                            )?;
                            event.remove("_ingest._value.empty");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.guid")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        event.append_unique(
                            "host.id",
                            json!(
                                event
                                    .get("_ingest._value.guid")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.isolated") {
                                if let Some(val) = event.get("_ingest._value.isolated") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.isolated".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.isolated", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_machines_isolated_to_boolean",
                            )?;
                            event.remove("_ingest._value.isolated");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.machines").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.lastConnected")
                                    {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => event
                                                .set("_ingest._value.last_connected", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.lastConnected".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.set(
                                        "_ingest.on_failure_processor_tag",
                                        "date_machines_lastConnected",
                                    )?;
                                    event.remove("_ingest._value.lastConnected");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "json.machines",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        if event.has_value("_ingest._value.osType") {
                            event.rename("_ingest._value.osType", "_ingest._value.os_type")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.os_type")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.get("json.machines").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def os_types = new ArrayList(); for (def obj : ctx.json.machines) {\n  if (obj.containsKey(\"os_type\") && [\"linux\", \"macos\", \"unix\", \"windows\", \"ios\", \"android\"].contains((obj.get(\"os_type\")).toLowerCase())) {\n    os_types.add(obj.get(\"os_type\").toLowerCase());\n  }\n} if (os_types.size() > 0){\n  if (ctx.host.os == null) {\n    ctx.host.os = new HashMap();\n  }\n  ctx.host.os.type= os_types;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def os_types = new ArrayList(); for (def obj : ctx.json.machines) {\n  if (obj.containsKey(\"os_type\") && [\"linux\", \"macos\", \"unix\", \"windows\", \"ios\", \"android\"].contains((obj.get(\"os_type\")).toLowerCase())) {\n    os_types.add(obj.get(\"os_type\").toLowerCase());\n  }\n} if (os_types.size() > 0){\n  if (ctx.host.os == null) {\n    ctx.host.os = new HashMap();\n  }\n  ctx.host.os.type= os_types;\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("json.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.machines", |event| {
                        event.remove("_ingest._value.lastConnected");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.machines") {
                event.rename("json.machines", "cybereason.poll_malop.machines")?;
            }

            if event.has_value("json.primaryRootCauseName") {
                event.rename(
                    "json.primaryRootCauseName",
                    "cybereason.poll_malop.primary_root_cause_name",
                )?;
            }

            if event.has_value("json.priority") {
                event.rename("json.priority", "cybereason.poll_malop.priority")?;
            }

            if event.has_value("json.processes") {
                event.rename("json.processes", "cybereason.poll_malop.processes")?;
            }

            if event.has_value("json.rootCauseElementHashes") {
                event.rename(
                    "json.rootCauseElementHashes",
                    "cybereason.poll_malop.root_cause_element.hashes",
                )?;
            }

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("cybereason.poll_malop.root_cause_element.hashes")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.rootCauseElementNamesCount") {
                    if let Some(val) = event.get("json.rootCauseElementNamesCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.rootCauseElementNamesCount".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cybereason.poll_malop.root_cause_element.names_count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_rootCauseElementNamesCount_to_long",
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

            if event.has_value("json.rootCauseElementType") {
                event.rename(
                    "json.rootCauseElementType",
                    "cybereason.poll_malop.root_cause_element.type",
                )?;
            }

            if event.has_value("json.severity") {
                event.rename("json.severity", "cybereason.poll_malop.severity")?;
            }

            if event.has_value("json.status") {
                event.rename("json.status", "cybereason.poll_malop.status")?;
            }

            let _cond = { event.has_value("json.users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.users", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.admin") {
                                if let Some(val) = event.get("_ingest._value.admin") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.admin".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.admin", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_users_admin_to_boolean",
                            )?;
                            event.remove("_ingest._value.admin");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.users", |event| {
                        if event.has_value("_ingest._value.displayName") {
                            event.rename(
                                "_ingest._value.displayName",
                                "_ingest._value.display_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.users", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.display_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.users", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.guid")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.users", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.domainUser") {
                                if let Some(val) = event.get("_ingest._value.domainUser") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.domainUser".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.domain_user", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_users_domainUser_to_boolean",
                            )?;
                            event.remove("_ingest._value.domainUser");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.users", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.localSystem") {
                                if let Some(val) = event.get("_ingest._value.localSystem") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.localSystem".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.local_system", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_users_localSystem_to_boolean",
                            )?;
                            event.remove("_ingest._value.localSystem");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.users") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.users", |event| {
                        event.remove("_ingest._value.localSystem");
                        event.remove("_ingest._value.domainUser");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.users") {
                event.rename("json.users", "cybereason.poll_malop.users")?;
            }

            let _cond = { event.has_value("cybereason.poll_malop.machines") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "cybereason.poll_malop.machines", |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.display_name");
                            event.remove("_ingest._value.guid");
                            event.remove("_ingest._value.os_type");
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("cybereason.poll_malop.creation_time");
                event.remove("cybereason.poll_malop.group");
                event.remove("cybereason.poll_malop.guid");
                event.remove("cybereason.poll_malop.last_update_time");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
