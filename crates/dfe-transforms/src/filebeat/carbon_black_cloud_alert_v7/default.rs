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

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.backend_timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.backend_update_timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.detection_timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("alert"))?;

            if event.has_value("json.id") {
                event.rename("json.id", "event.id")?;
            }

            let _cond = {
                event.has_value("json.detection_timestamp")
                    && event.get_str("json.detection_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.detection_timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.detection_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.detection_timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.detection_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            let _cond = {
                event.has_value("json.first_event_timestamp")
                    && event.get_str("json.first_event_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.first_event_timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("event.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.first_event_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.first_event_timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.first_event_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            let _cond = {
                event.has_value("json.last_event_timestamp")
                    && event.get_str("json.last_event_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_event_timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("event.end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_event_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.last_event_timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.last_event_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.severity") {
                    if let Some(val) = event.get("json.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severity".into(),
                                message,
                            }
                        })?;
                        event.set("event.severity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.severity").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.severity".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.reason") {
                event.rename("json.reason", "event.reason")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.device_id") {
                    if let Some(val) = event.get("json.device_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.device_id".into(),
                                message,
                            }
                        })?;
                        event.set("host.id", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.device_os") == Some("WINDOWS") };
            if _cond {
                event.set("host.os.type", json!("windows"))?;
            }

            let _cond = { event.get_str("json.device_os") == Some("LINUX") };
            if _cond {
                event.set("host.os.type", json!("linux"))?;
            }

            let _cond = { event.get_str("json.device_os") == Some("MAC") };
            if _cond {
                event.set("host.os.type", json!("macos"))?;
            }

            if event.has_value("json.device_os_version") {
                event.rename("json.device_os_version", "host.os.version")?;
            }

            if event.has_value("json.device_name") {
                event.rename("json.device_name", "host.hostname")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("host.hostname") {
                    if let Some(input) = event.get_string("host.hostname") {
                        // Grok pattern: ^(%{DATA:user.domain})\\\\(%{GREEDYDATA:host.hostname})$
                        if !cached_grok!("^(%{DATA:user.domain})\\\\(%{GREEDYDATA:host.hostname})$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "host.name",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.device_username") {
                    if let Some(input) = event.get_string("json.device_username") {
                        // Grok pattern: ^%{DATA:user.domain}\\\\%{GREEDYDATA:user.name}$
                        // Grok pattern: ^%{EMAILADDRESS:user.email}$
                        // Grok pattern: ^%{GREEDYDATA:user.name}$
                        if !extract_first_match(
                            &[
                                cached_grok!("^%{DATA:user.domain}\\\\%{GREEDYDATA:user.name}$"),
                                cached_grok!("^%{EMAILADDRESS:user.email}$"),
                                cached_grok!("^%{GREEDYDATA:user.name}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("user.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.backend_timestamp")
                    && event.get_str("json.backend_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.backend_timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("carbon_black_cloud.alert.backend_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.backend_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.backend_timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.backend_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            let _cond = {
                event.has_value("json.backend_update_timestamp")
                    && event.get_str("json.backend_update_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.backend_update_timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("carbon_black_cloud.alert.backend_update_timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.backend_update_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.backend_update_timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.backend_update_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            let _cond = {
                event.has_value("json.user_update_timestamp")
                    && event.get_str("json.user_update_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.user_update_timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("carbon_black_cloud.alert.user_update_timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.user_update_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.user_update_timestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.user_update_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            let _cond = { event.get("json.ttps").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("json.ttps") {
                    foreach_array(event, "json.ttps", |event| {
                        event.append_unique(
                            "carbon_black_cloud.alert.ttps",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("json.threat_category")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("json.threat_category") {
                    foreach_array(event, "json.threat_category", |event| {
                        event.append_unique(
                            "carbon_black_cloud.alert.threat_category",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("json.ml_classification_anomalies")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("json.ml_classification_anomalies") {
                    foreach_array(event, "json.ml_classification_anomalies", |event| {
                        event.append_unique(
                            "carbon_black_cloud.alert.ml_classification_anomalies",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.alert_notes_present") {
                    if let Some(val) = event.get("json.alert_notes_present") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.alert_notes_present".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.alert_notes_present", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.alert_notes_present").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.alert_notes_present".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threat_notes_present") {
                    if let Some(val) = event.get("json.threat_notes_present") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threat_notes_present".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.threat_notes_present", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.threat_notes_present").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.threat_notes_present".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.additional_events_present") {
                    if let Some(val) = event.get("json.additional_events_present") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.additional_events_present".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "carbon_black_cloud.alert.additional_events_present",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.additional_events_present").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.additional_events_present".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.alert_url") {
                event.rename("json.alert_url", "carbon_black_cloud.alert.url")?;
            }

            if event.has_value("json.primary_event_id") {
                event.rename(
                    "json.primary_event_id",
                    "carbon_black_cloud.alert.primary_event_id",
                )?;
            }

            if event.has_value("json.org_key") {
                event.rename("json.org_key", "carbon_black_cloud.alert.organization_key")?;
            }

            if event.has_value("json.vendor_name") {
                event.rename("json.vendor_name", "carbon_black_cloud.alert.vendor_name")?;
            }

            if event.has_value("json.product_name") {
                event.rename("json.product_name", "carbon_black_cloud.alert.product_name")?;
            }

            if event.has_value("json.serial_number") {
                event.rename(
                    "json.serial_number",
                    "carbon_black_cloud.alert.serial_number",
                )?;
            }

            if event.has_value("json.threat_id") {
                event.rename("json.threat_id", "carbon_black_cloud.alert.threat_id")?;
            }

            if event.has_value("json.policy_applied") {
                event.rename(
                    "json.policy_applied",
                    "carbon_black_cloud.alert.policy_applied",
                )?;
            }

            if event.has_value("json.parent_name") {
                event.rename("json.parent_name", "process.parent.name")?;
            }

            if let Some(v) = event
                .get("process.parent.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            if let Some(v) = event
                .get("process.parent.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.parent.name", v)?;
            }

            if event.has_value("json.parent_cmdline") {
                event.rename("json.parent_cmdline", "process.parent.command_line")?;
            }

            if let Some(v) = event
                .get("process.parent.command_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.parent.cmdline", v)?;
            }

            if event.has_value("json.parent_guid") {
                event.rename("json.parent_guid", "process.parent.entity_id")?;
            }

            if let Some(v) = event
                .get("process.parent.entity_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.parent.guid", v)?;
            }

            if event.has_value("json.parent_pid") {
                event.rename("json.parent_pid", "process.parent.pid")?;
            }

            if let Some(v) = event
                .get("process.parent.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.parent.pid", v)?;
            }

            if event.has_value("json.parent_sha256") {
                event.rename("json.parent_sha256", "process.parent.hash.sha256")?;
            }

            if let Some(v) = event
                .get("process.parent.hash.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.parent.hash.sha256", v)?;
            }

            let _cond = { event.has_value("carbon_black_cloud.alert.parent.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.alert.parent.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.parent_md5") {
                event.rename("json.parent_md5", "process.parent.hash.md5")?;
            }

            if let Some(v) = event
                .get("process.parent.hash.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.parent.hash.md5", v)?;
            }

            let _cond = { event.has_value("carbon_black_cloud.alert.parent.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.alert.parent.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.parent_username") {
                event.rename(
                    "json.parent_username",
                    "carbon_black_cloud.alert.parent.username",
                )?;
            }

            if event.has_value("json.parent_reputation") {
                event.rename(
                    "json.parent_reputation",
                    "carbon_black_cloud.alert.parent.reputation",
                )?;
            }

            if event.has_value("json.parent_effective_reputation") {
                event.rename(
                    "json.parent_effective_reputation",
                    "carbon_black_cloud.alert.parent.effective_reputation",
                )?;
            }

            let _cond = { event.has_value("process.parent.name") };
            if _cond {
                // Painless script
                // Source: ctx.process.parent.name = ctx.process.parent.name.substring(ctx.process.parent.name.lastIndexOf('\\\\') + 1);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.process.parent.name = ctx.process.parent.name.substring(ctx.process.parent.name.lastIndexOf('\\\\') + 1);\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("json.process_issuer")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("json.process_issuer") {
                    foreach_array(event, "json.process_issuer", |event| {
                        event.append_unique(
                            "carbon_black_cloud.alert.process.issuer",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("json.process_publisher")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("json.process_publisher") {
                    foreach_array(event, "json.process_publisher", |event| {
                        event.append_unique(
                            "carbon_black_cloud.alert.process.publisher",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            if event.has_value("json.process_name") {
                event.rename("json.process_name", "process.name")?;
            }

            if let Some(v) = event
                .get("process.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if let Some(v) = event
                .get("process.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.process.name", v)?;
            }

            if event.has_value("json.process_username") {
                event.rename(
                    "json.process_username",
                    "carbon_black_cloud.alert.process.username",
                )?;
            }

            if event.has_value("json.process_cmdline") {
                event.rename("json.process_cmdline", "process.command_line")?;
            }

            if let Some(v) = event
                .get("process.command_line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.process.cmdline", v)?;
            }

            if event.has_value("json.process_guid") {
                event.rename("json.process_guid", "process.entity_id")?;
            }

            if let Some(v) = event
                .get("process.entity_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.process.guid", v)?;
            }

            if event.has_value("json.process_pid") {
                event.rename("json.process_pid", "process.pid")?;
            }

            if let Some(v) = event
                .get("process.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.process.pid", v)?;
            }

            if event.has_value("json.process_sha256") {
                event.rename("json.process_sha256", "process.hash.sha256")?;
            }

            if let Some(v) = event
                .get("process.hash.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.process.hash.sha256", v)?;
            }

            let _cond = { event.has_value("carbon_black_cloud.alert.process.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.alert.process.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.process_md5") {
                event.rename("json.process_md5", "process.hash.md5")?;
            }

            if let Some(v) = event
                .get("process.hash.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("carbon_black_cloud.alert.process.hash.md5", v)?;
            }

            let _cond = { event.has_value("carbon_black_cloud.alert.process.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.alert.process.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.process_reputation") {
                event.rename(
                    "json.process_reputation",
                    "carbon_black_cloud.alert.process.reputation",
                )?;
            }

            if event.has_value("json.process_effective_reputation") {
                event.rename(
                    "json.process_effective_reputation",
                    "carbon_black_cloud.alert.process.effective_reputation",
                )?;
            }

            let _cond = { event.has_value("process.name") };
            if _cond {
                // Painless script
                // Source: ctx.process.name = ctx.process.name.substring(ctx.process.name.lastIndexOf('\\\\') + 1);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.process.name = ctx.process.name.substring(ctx.process.name.lastIndexOf('\\\\') + 1);\n"#
                    ),
                )?;
            }

            if event.has_value("json.childproc_name") {
                event.rename(
                    "json.childproc_name",
                    "carbon_black_cloud.alert.childproc.name",
                )?;
            }

            if event.has_value("json.childproc_username") {
                event.rename(
                    "json.childproc_username",
                    "carbon_black_cloud.alert.childproc.username",
                )?;
            }

            if event.has_value("json.childproc_cmdline") {
                event.rename(
                    "json.childproc_cmdline",
                    "carbon_black_cloud.alert.childproc.cmdline",
                )?;
            }

            if event.has_value("json.childproc_guid") {
                event.rename(
                    "json.childproc_guid",
                    "carbon_black_cloud.alert.childproc.guid",
                )?;
            }

            if event.has_value("json.childproc_sha256") {
                event.rename(
                    "json.childproc_sha256",
                    "carbon_black_cloud.alert.childproc.hash.sha256",
                )?;
            }

            let _cond = { event.has_value("carbon_black_cloud.alert.childproc.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.alert.childproc.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.childproc_md5") {
                event.rename(
                    "json.childproc_md5",
                    "carbon_black_cloud.alert.childproc.hash.md5",
                )?;
            }

            let _cond = { event.has_value("carbon_black_cloud.alert.childproc.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.alert.childproc.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.childproc_effective_reputation") {
                event.rename(
                    "json.childproc_effective_reputation",
                    "carbon_black_cloud.alert.childproc.effective_reputation",
                )?;
            }

            if event.has_value("json.blocked_name") {
                event.rename(
                    "json.blocked_name",
                    "carbon_black_cloud.alert.blocked_process.name",
                )?;
            }

            if event.has_value("json.blocked_sha256") {
                event.rename(
                    "json.blocked_sha256",
                    "carbon_black_cloud.alert.blocked_process.hash.sha256",
                )?;
            }

            let _cond = { event.has_value("carbon_black_cloud.alert.blocked_process.hash.sha256") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.alert.blocked_process.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.blocked_md5") {
                event.rename(
                    "json.blocked_md5",
                    "carbon_black_cloud.alert.blocked_process.hash.md5",
                )?;
            }

            let _cond = { event.has_value("carbon_black_cloud.alert.blocked_process.hash.md5") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("carbon_black_cloud.alert.blocked_process.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.blocked_effective_reputation") {
                event.rename(
                    "json.blocked_effective_reputation",
                    "carbon_black_cloud.alert.blocked_process.effective_reputation",
                )?;
            }

            if event.has_value("json.ioc_field") {
                event.rename("json.ioc_field", "carbon_black_cloud.alert.ioc.field")?;
            }

            if event.has_value("json.ioc_hit") {
                event.rename("json.ioc_hit", "carbon_black_cloud.alert.ioc.hit")?;
            }

            if event.has_value("json.ioc_id") {
                event.rename("json.ioc_id", "carbon_black_cloud.alert.ioc.id")?;
            }

            let _cond = { event.get("json.report_tags").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("json.report_tags") {
                    foreach_array(event, "json.report_tags", |event| {
                        event.append_unique(
                            "carbon_black_cloud.alert.report.tags",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            if event.has_value("json.report_id") {
                event.rename("json.report_id", "carbon_black_cloud.alert.report.id")?;
            }

            if event.has_value("json.report_name") {
                event.rename("json.report_name", "carbon_black_cloud.alert.report.name")?;
            }

            if event.has_value("json.report_description") {
                event.rename(
                    "json.report_description",
                    "carbon_black_cloud.alert.report.description",
                )?;
            }

            if event.has_value("json.report_link") {
                event.rename("json.report_link", "carbon_black_cloud.alert.report.link")?;
            }

            if event.has_value("json.device_location") {
                event.rename(
                    "json.device_location",
                    "carbon_black_cloud.alert.device.location",
                )?;
            }

            if event.has_value("json.device_os") {
                event.rename("json.device_os", "carbon_black_cloud.alert.device.os")?;
            }

            if event.has_value("json.device_policy_id") {
                event.rename(
                    "json.device_policy_id",
                    "carbon_black_cloud.alert.device.policy_id",
                )?;
            }

            if event.has_value("json.device_policy") {
                event.rename(
                    "json.device_policy",
                    "carbon_black_cloud.alert.device.policy",
                )?;
            }

            if event.has_value("json.device_target_value") {
                event.rename(
                    "json.device_target_value",
                    "carbon_black_cloud.alert.device.target_value",
                )?;
            }

            if event.has_value("json.device_external_ip") {
                event.rename(
                    "json.device_external_ip",
                    "carbon_black_cloud.alert.device.external_ip",
                )?;
            }

            if event.has_value("json.device_internal_ip") {
                event.rename(
                    "json.device_internal_ip",
                    "carbon_black_cloud.alert.device.internal_ip",
                )?;
            }

            if event.has_value("json.device_uem_id") {
                event.rename(
                    "json.device_uem_id",
                    "carbon_black_cloud.alert.device.uem_id",
                )?;
            }

            if event.has_value("json.workflow") {
                event.rename("json.workflow", "carbon_black_cloud.alert.workflow")?;
            }

            let _cond = {
                event.has_value("carbon_black_cloud.alert.workflow.change_timestamp")
                    && event.get_str("carbon_black_cloud.alert.workflow.change_timestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("carbon_black_cloud.alert.workflow.change_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "carbon_black_cloud.alert.workflow.change_timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "carbon_black_cloud.alert.workflow.change_timestamp"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("carbon_black_cloud.alert.workflow.change_timestamp")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "carbon_black_cloud.alert.workflow.change_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.determination") {
                event.rename(
                    "json.determination",
                    "carbon_black_cloud.alert.determination",
                )?;
            }

            let _cond = {
                event.has_value("carbon_black_cloud.alert.determination.change_timestamp")
                    && event.get_str("carbon_black_cloud.alert.determination.change_timestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("carbon_black_cloud.alert.determination.change_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "carbon_black_cloud.alert.determination.change_timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "carbon_black_cloud.alert.determination.change_timestamp"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("carbon_black_cloud.alert.determination.change_timestamp")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "carbon_black_cloud.alert.determination.change_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.mdr_alert") {
                    if let Some(val) = event.get("json.mdr_alert") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.mdr_alert".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.mdr.alert", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.mdr_alert").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.mdr_alert".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.mdr_alert_notes_present") {
                    if let Some(val) = event.get("json.mdr_alert_notes_present") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.mdr_alert_notes_present".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "carbon_black_cloud.alert.mdr.alert_notes_present",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.mdr_alert_notes_present").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.mdr_alert_notes_present".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.mdr_threat_notes_present") {
                    if let Some(val) = event.get("json.mdr_threat_notes_present") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.mdr_threat_notes_present".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "carbon_black_cloud.alert.mdr.threat_notes_present",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.mdr_threat_notes_present").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.mdr_threat_notes_present".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.mdr_determination") {
                event.rename(
                    "json.mdr_determination",
                    "carbon_black_cloud.alert.mdr.determination",
                )?;
            }

            let _cond = {
                event.has_value("carbon_black_cloud.alert.mdr.determination.change_timestamp")
                    && event.get_str("carbon_black_cloud.alert.mdr.determination.change_timestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "carbon_black_cloud.alert.mdr.determination.change_timestamp",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "carbon_black_cloud.alert.mdr.determination.change_timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "carbon_black_cloud.alert.mdr.determination.change_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("carbon_black_cloud.alert.mdr.determination.change_timestamp")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "carbon_black_cloud.alert.mdr.determination.change_timestamp"
                                .into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.mdr_workflow") {
                event.rename("json.mdr_workflow", "carbon_black_cloud.alert.mdr.workflow")?;
            }

            let _cond = {
                event.has_value("carbon_black_cloud.alert.mdr.workflow.change_timestamp")
                    && event.get_str("carbon_black_cloud.alert.mdr.workflow.change_timestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("carbon_black_cloud.alert.mdr.workflow.change_timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "carbon_black_cloud.alert.mdr.workflow.change_timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "carbon_black_cloud.alert.mdr.workflow.change_timestamp"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("carbon_black_cloud.alert.mdr.workflow.change_timestamp")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "carbon_black_cloud.alert.mdr.workflow.change_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.mdr_classification") {
                event.rename(
                    "json.mdr_classification",
                    "carbon_black_cloud.alert.mdr.classification",
                )?;
            }

            let _cond = {
                event.has_value("carbon_black_cloud.alert.mdr.classification.change_timestamp")
                    && event.get_str("carbon_black_cloud.alert.mdr.classification.change_timestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "carbon_black_cloud.alert.mdr.classification.change_timestamp",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "carbon_black_cloud.alert.mdr.classification.change_timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "carbon_black_cloud.alert.mdr.classification.change_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("carbon_black_cloud.alert.mdr.classification.change_timestamp")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "carbon_black_cloud.alert.mdr.classification.change_timestamp"
                                .into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.netconn_remote_port") {
                    if let Some(val) = event.get("json.netconn_remote_port") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.netconn_remote_port".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.netconn.remote_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.netconn_remote_port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.netconn_remote_port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.netconn_local_port") {
                    if let Some(val) = event.get("json.netconn_local_port") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.netconn_local_port".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.netconn.local_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.netconn_local_port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.netconn_local_port".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.netconn_remote_ip") {
                    if let Some(val) = event.get("json.netconn_remote_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.netconn_remote_ip".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.netconn.remote_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.netconn_remote_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.netconn_remote_ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.netconn_local_ip") {
                    if let Some(val) = event.get("json.netconn_local_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.netconn_local_ip".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.netconn.local_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.netconn_local_ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.netconn_local_ip".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.netconn_remote_ipv4") {
                    if let Some(val) = event.get("json.netconn_remote_ipv4") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.netconn_remote_ipv4".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.netconn.remote_ipv4", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.netconn_remote_ipv4").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.netconn_remote_ipv4".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.netconn_local_ipv4") {
                    if let Some(val) = event.get("json.netconn_local_ipv4") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.netconn_local_ipv4".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.netconn.local_ipv4", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.netconn_local_ipv4").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.netconn_local_ipv4".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.netconn_remote_ipv6") {
                    if let Some(val) = event.get("json.netconn_remote_ipv6") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.netconn_remote_ipv6".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.netconn.remote_ipv6", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.netconn_remote_ipv6").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.netconn_remote_ipv6".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.netconn_local_ipv6") {
                    if let Some(val) = event.get("json.netconn_local_ipv6") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.netconn_local_ipv6".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.netconn.local_ipv6", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.netconn_local_ipv6").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.netconn_local_ipv6".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            if event.has_value("json.netconn_protocol") {
                event.rename(
                    "json.netconn_protocol",
                    "carbon_black_cloud.alert.netconn.protocol",
                )?;
            }

            if event.has_value("json.netconn_remote_domain") {
                event.rename(
                    "json.netconn_remote_domain",
                    "carbon_black_cloud.alert.netconn.remote_domain",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ip_reputation") {
                    if let Some(val) = event.get("json.ip_reputation") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ip_reputation".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.ip_reputation", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ip_reputation").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ip_reputation".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.remote_is_private") {
                    if let Some(val) = event.get("json.remote_is_private") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.remote_is_private".into(),
                                message,
                            }
                        })?;
                        event.set("carbon_black_cloud.alert.remote_is_private", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.remote_is_private").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.remote_is_private".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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

            event.set("carbon_black_cloud.alert.category", json!("THREAT"))?;

            event.remove("json.severity");
            event.remove("json.detection_timestamp");
            event.remove("json.backend_timestamp");
            event.remove("json.backend_update_timestamp");
            event.remove("json.user_update_timestamp");
            event.remove("json.first_event_timestamp");
            event.remove("json.last_event_timestamp");
            event.remove("json.device_id");
            event.remove("json.device_username");
            event.remove("json.alert_notes_present");
            event.remove("json.threat_notes_present");
            event.remove("json.additional_events_present");
            event.remove("json.netconn_remote_port");
            event.remove("json.netconn_local_port");
            event.remove("json.netconn_remote_ip");
            event.remove("json.netconn_local_ip");
            event.remove("json.netconn_remote_ipv4");
            event.remove("json.netconn_local_ipv4");
            event.remove("json.netconn_remote_ipv6");
            event.remove("json.netconn_local_ipv6");
            event.remove("json.mdr_classification?.determination");
            event.remove("json.ml_classification_anomalies");
            event.remove("json.ip_reputation");
            event.remove("json.remote_is_private");
            event.remove("json.mdr_alert");
            event.remove("json.mdr_alert_notes_present");
            event.remove("json.mdr_threat_notes_present");
            event.remove("json.process_issuer");
            event.remove("json.process_publisher");
            event.remove("json.report_tags");

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.carbon_black_cloud.alert[m.getKey()] = m.getValue();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.carbon_black_cloud.alert[m.getKey()] = m.getValue();\n}\n"#
                    ),
                )?;
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
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
            }
        }

        Ok(TransformResult::Continue)
    }
}
