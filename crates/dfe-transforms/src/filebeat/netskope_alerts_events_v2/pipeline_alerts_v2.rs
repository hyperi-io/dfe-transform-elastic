// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_alerts_v2` pipeline.
pub struct PipelineAlertsV2;

impl Transform for PipelineAlertsV2 {
    fn name(&self) -> &str {
        "pipeline_alerts_v2"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.17.0"))?;

                if event.has_value("netskope.alerts_events_v2") {
                    event.rename("netskope.alerts_events_v2", "netskope.alert_v2")?;
                }

                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("netskope.alert_v2._id") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }

                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == '-' || v == 'N/A' || v == 'NotChecked' || v == 'NotAvailable' || v == 'NoSSL'\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == '-' || v == 'N/A' || v == 'NotChecked' || v == 'NotAvailable' || v == 'NoSSL'\n  });\n}\nhandleMap(ctx);
                drop_empty(event, &DropPolicy { prune_lists: true, sentinels: vec!["-".into(), "N/A".into(), "NotChecked".into(), "NotAvailable".into(), "NoSSL".into()], ..DropPolicy::none() }, None);

            let _cond = { event.get("netskope.alert_v2.custom_attr").is_some_and(|v| v.is_string()) || event.get("netskope.alert_v2.custom_attr").is_some_and(|v| v.is_object()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "netskope.alert_v2.custom_attr", "netskope.alert_v2.custom_attr")?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("netskope.alert_v2._id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.id", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.acked") {
                if let Some(val) = event.get("netskope.alert_v2.acked") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.acked".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.acked", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_acked_to_boolean")?;
                        if event.remove("netskope.alert_v2.acked").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.acked".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dlp_is_unique_count") {
                if let Some(val) = event.get("netskope.alert_v2.dlp_is_unique_count") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dlp_is_unique_count".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dlp_is_unique_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_is_unique_count_to_boolean")?;
                        if event.remove("netskope.alert_v2.dlp_is_unique_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.dlp_is_unique_count".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.action").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.has_value("event.action") && event.get_str("event.action") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("event.action") {
                if let Some(s) = event.get_string("event.action") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("event.action", Value::Array(parts))?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "split")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("event.action") && event.get_str("event.action") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                if let Some(joined) = joined {
                    event.set("event.action", json!(joined))?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "join")?;
                event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("netskope.alert_v2.alert_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.app").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.application", v)?;
            }

            if event.has_value("network.application") {
                map_strings(event, "network.application", "network.application", str::to_lowercase)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.total_packets") {
                if let Some(val) = event.get("netskope.alert_v2.total_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.total_packets".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.total_packets", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_packets_to_long")?;
                        if event.remove("netskope.alert_v2.total_packets").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.total_packets".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.total_packets").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.packets", v)?;
            }

            if event.has_value("netskope.alert_v2.app_session_id") {
                if let Some(val) = event.get("netskope.alert_v2.app_session_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.app_session_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.app_session_id", converted)?;
                }
            }

            let _cond = { event.has_value("netskope.alert_v2.breach_date") && event.get_str("netskope.alert_v2.breach_date") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.alert_v2.breach_date") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX", "epoch_second"], None, None) {
                        Some(parsed) => event.set("netskope.alert_v2.breach_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.alert_v2.breach_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_breach_date")?;
                        if event.remove("netskope.alert_v2.breach_date").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.breach_date".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
            if event.has_value("netskope.alert_v2.breach_score") {
                if let Some(val) = event.get("netskope.alert_v2.breach_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.breach_score".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.breach_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_breach_score_to_long")?;
                        if event.remove("netskope.alert_v2.breach_score").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.breach_score".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("netskope.alert_v2.browser_session_id") {
                if let Some(val) = event.get("netskope.alert_v2.browser_session_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.browser_session_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.browser_session_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.cci") {
                if let Some(val) = event.get("netskope.alert_v2.cci") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.cci".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.cci", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cci_to_long")?;
                        if event.remove("netskope.alert_v2.cci").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.cci".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.client_bytes") {
                if let Some(val) = event.get("netskope.alert_v2.client_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.client_bytes".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.client_bytes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_bytes_to_long")?;
                        if event.remove("netskope.alert_v2.client_bytes").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.client_bytes".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.client_bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.bytes", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.client_packets") {
                if let Some(val) = event.get("netskope.alert_v2.client_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.client_packets".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.client_packets", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_packets_to_long")?;
                        if event.remove("netskope.alert_v2.client_packets").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.client_packets".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.client_packets").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.packets", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.conn_duration") {
                if let Some(val) = event.get("netskope.alert_v2.conn_duration") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.conn_duration".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.conn_duration", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_conn_duration_to_long")?;
                        if event.remove("netskope.alert_v2.conn_duration").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.conn_duration".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("netskope.alert_v2.conn_endtime") && event.get_str("netskope.alert_v2.conn_endtime") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.alert_v2.conn_endtime") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX", "epoch_second"], None, None) {
                        Some(parsed) => event.set("netskope.alert_v2.conn_endtime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.alert_v2.conn_endtime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_conn_endtime")?;
                        if event.remove("netskope.alert_v2.conn_endtime").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.conn_endtime".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("netskope.alert_v2.conn_starttime") && event.get_str("netskope.alert_v2.conn_starttime") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.alert_v2.conn_starttime") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX", "epoch_second"], None, None) {
                        Some(parsed) => event.set("netskope.alert_v2.conn_starttime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.alert_v2.conn_starttime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_conn_starttime")?;
                        if event.remove("netskope.alert_v2.conn_starttime").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.conn_starttime".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("netskope.alert_v2.connection_id") {
                if let Some(val) = event.get("netskope.alert_v2.connection_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.connection_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.connection_id", converted)?;
                }
            }

            if event.has_value("netskope.alert_v2.dlp_incident_id") {
                if let Some(val) = event.get("netskope.alert_v2.dlp_incident_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dlp_incident_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dlp_incident_id", converted)?;
                }
            }

            if event.has_value("netskope.alert_v2.dlp_parent_id") {
                if let Some(val) = event.get("netskope.alert_v2.dlp_parent_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dlp_parent_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dlp_parent_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dlp_rule_count") {
                if let Some(val) = event.get("netskope.alert_v2.dlp_rule_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dlp_rule_count".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dlp_rule_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_rule_count_to_long")?;
                        if event.remove("netskope.alert_v2.dlp_rule_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.dlp_rule_count".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dlp_unique_count") {
                if let Some(val) = event.get("netskope.alert_v2.dlp_unique_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dlp_unique_count".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dlp_unique_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_unique_count_to_long")?;
                        if event.remove("netskope.alert_v2.dlp_unique_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.dlp_unique_count".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.domain", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("netskope.alert_v2.domain").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.domain_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.domain_ip") {
                if let Some(val) = event.get("netskope.alert_v2.domain_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.domain_ip".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.domain_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_domain_ip_to_ip")?;
                        if event.remove("netskope.alert_v2.domain_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.domain_ip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("netskope.alert_v2.domain_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("netskope.alert_v2.domain_ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.alert_v2.dst_country").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.country_iso_code", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dst_geoip_src") {
                if let Some(val) = event.get("netskope.alert_v2.dst_geoip_src") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dst_geoip_src".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dst_geoip_src", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_geoip_src_to_long")?;
                        if event.remove("netskope.alert_v2.dst_geoip_src").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.dst_geoip_src".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dst_latitude") {
                if let Some(val) = event.get("netskope.alert_v2.dst_latitude") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dst_latitude".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dst_latitude", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_latitude_to_double")?;
                        event.rename("netskope.alert_v2.dst_latitude", "netskope.alert_v2.dst_latitude_keyword")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.dst_latitude").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.location.lat", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dst_longitude") {
                if let Some(val) = event.get("netskope.alert_v2.dst_longitude") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dst_longitude".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dst_longitude", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_longitude_to_double")?;
                        event.rename("netskope.alert_v2.dst_longitude", "netskope.alert_v2.dst_longitude_keyword")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.dst_longitude").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.location.lon", v)?;
            }

            let _cond = { !(event.get("destination.geo.location.lat").is_some_and(|v| v.is_number())) || !(event.get("destination.geo.location.lon").is_some_and(|v| v.is_number())) || event.get_f64("destination.geo.location.lat").is_some_and(|n| n < -90.0) || event.get_f64("destination.geo.location.lat").is_some_and(|n| n > 90.0) || event.get_f64("destination.geo.location.lon").is_some_and(|n| n < -180.0) || event.get_f64("destination.geo.location.lon").is_some_and(|n| n > 180.0) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("destination.geo.location").is_none() {
                    return Err(TransformError::FieldNotFound { path: "destination.geo.location".into() });
                }
                Ok(())
            })();
            }

            if let Some(v) = event.get("netskope.alert_v2.dst_location").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.city_name", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.dst_region").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.region_name", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.dst_timezone").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.timezone", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.dst_zipcode").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.postal_code", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.dsthost").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.domain", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.dsthost") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("netskope.alert_v2.dsthost").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.dstip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dstip") {
                if let Some(val) = event.get("netskope.alert_v2.dstip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dstip".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dstip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dstip_to_ip")?;
                        if event.remove("netskope.alert_v2.dstip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.dstip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("netskope.alert_v2.dstip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.dstip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("netskope.alert_v2.dstip").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dstport") {
                if let Some(val) = event.get("netskope.alert_v2.dstport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dstport".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dstport", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dstport_to_long")?;
                        if event.remove("netskope.alert_v2.dstport").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.dstport".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.dstport").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.email_title").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.subject", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.file_cls_encrypted") {
                if let Some(val) = event.get("netskope.alert_v2.file_cls_encrypted") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.file_cls_encrypted".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.file_cls_encrypted", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_cls_encrypted_to_boolean")?;
                        if event.remove("netskope.alert_v2.file_cls_encrypted").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.file_cls_encrypted".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.executable_signed") {
                if let Some(val) = event.get("netskope.alert_v2.executable_signed") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.executable_signed".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.executable_signed", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_executable_signed_to_boolean")?;
                        if event.remove("netskope.alert_v2.executable_signed").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.executable_signed".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("netskope.alert_v2.file_exposure") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("netskope.alert_v2.file_exposure").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.alert_v2.file_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.file_size") {
                if let Some(val) = event.get("netskope.alert_v2.file_size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.file_size".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.file_size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_size_to_long")?;
                        if event.remove("netskope.alert_v2.file_size").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.file_size".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.file_size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.file_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.type", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.hostname").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("netskope.alert_v2.hostname").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.iaas_remediated") {
                if let Some(val) = event.get("netskope.alert_v2.iaas_remediated") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.iaas_remediated".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.iaas_remediated", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_iaas_remediated_to_boolean")?;
                        if event.remove("netskope.alert_v2.iaas_remediated").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.iaas_remediated".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.iaas_remediated_on") {
                if let Some(val) = event.get("netskope.alert_v2.iaas_remediated_on") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.iaas_remediated_on".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.iaas_remediated_on", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_iaas_remediated_on_to_long")?;
                        if event.remove("netskope.alert_v2.iaas_remediated_on").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.iaas_remediated_on".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.local_md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.hash.md5", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.local_md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("netskope.alert_v2.local_md5").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.alert_v2.local_sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.hash.sha1", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.local_sha1") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("netskope.alert_v2.local_sha1").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.alert_v2.local_sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.hash.sha256", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.local_sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("netskope.alert_v2.local_sha256").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.alert_v2.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("netskope.alert_v2.md5").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.alert_v2.mime_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.mime_type", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.numbytes") {
                if let Some(val) = event.get("netskope.alert_v2.numbytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.numbytes".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.numbytes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_numbytes_to_long")?;
                        if event.remove("netskope.alert_v2.numbytes").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.numbytes".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dlp_fingerprint_score") {
                if let Some(val) = event.get("netskope.alert_v2.dlp_fingerprint_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dlp_fingerprint_score".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dlp_fingerprint_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_fingerprint_score_to_long")?;
                        if event.remove("netskope.alert_v2.dlp_fingerprint_score").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.dlp_fingerprint_score".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.dlp_rule_score") {
                if let Some(val) = event.get("netskope.alert_v2.dlp_rule_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.dlp_rule_score".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.dlp_rule_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_rule_score_to_long")?;
                        if event.remove("netskope.alert_v2.dlp_rule_score").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.dlp_rule_score".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.org").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("organization.name", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.os").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.full", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.os_family").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.family", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.os_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.version", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.referer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.referrer", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.req_cnt") {
                if let Some(val) = event.get("netskope.alert_v2.req_cnt") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.req_cnt".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.req_cnt", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_req_cnt_to_long")?;
                        if event.remove("netskope.alert_v2.req_cnt").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.req_cnt".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("netskope.alert_v2.request_id") {
                if let Some(val) = event.get("netskope.alert_v2.request_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.request_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.request_id", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.resp_cnt") {
                if let Some(val) = event.get("netskope.alert_v2.resp_cnt") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.resp_cnt".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.resp_cnt", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_resp_cnt_to_long")?;
                        if event.remove("netskope.alert_v2.resp_cnt").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.resp_cnt".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.server_bytes") {
                if let Some(val) = event.get("netskope.alert_v2.server_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.server_bytes".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.server_bytes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_server_bytes_to_long")?;
                        if event.remove("netskope.alert_v2.server_bytes").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.server_bytes".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.server_bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.bytes", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.server_packets") {
                if let Some(val) = event.get("netskope.alert_v2.server_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.server_packets".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.server_packets", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_server_packets_to_long")?;
                        if event.remove("netskope.alert_v2.server_packets").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.server_packets".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.server_packets").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.packets", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.session_duration") {
                if let Some(val) = event.get("netskope.alert_v2.session_duration") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.session_duration".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.session_duration", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_duration_to_long")?;
                        if event.remove("netskope.alert_v2.session_duration").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.session_duration".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.srcport") {
                if let Some(val) = event.get("netskope.alert_v2.srcport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.srcport".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.srcport", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_alert_v2_to_long")?;
                        if event.remove("netskope.event_v2.alert_v2").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.event_v2.alert_v2".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("netskope.alert_v2.severity") && event.get("netskope.alert_v2.severity").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.netskope.alert_v2.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.netskope.alert_v2.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("netskope.alert_v2.end_time") && event.get_str("netskope.alert_v2.end_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.alert_v2.end_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss", "ISO8601"], None, None) {
                        Some(parsed) => event.set("netskope.alert_v2.end_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.alert_v2.end_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_end_time")?;
                        if event.remove("netskope.alert_v2.end_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.end_time".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("netskope.alert_v2.end_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.end", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.start_time") && event.get_str("netskope.alert_v2.start_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.alert_v2.start_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss", "ISO8601"], None, None) {
                        Some(parsed) => event.set("netskope.alert_v2.start_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.alert_v2.start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_start_time")?;
                        if event.remove("netskope.alert_v2.start_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.start_time".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("netskope.alert_v2.start_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.start", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.src_country").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.country_iso_code", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.src_geoip_src") {
                if let Some(val) = event.get("netskope.alert_v2.src_geoip_src") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.src_geoip_src".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.src_geoip_src", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_geoip_src_to_long")?;
                        if event.remove("netskope.alert_v2.src_geoip_src").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.src_geoip_src".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.src_latitude") {
                if let Some(val) = event.get("netskope.alert_v2.src_latitude") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.src_latitude".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.src_latitude", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_latitude_to_double")?;
                        event.rename("netskope.alert_v2.src_latitude", "netskope.alert_v2.src_latitude_keyword")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.src_latitude").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.location.lat", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.src_longitude") {
                if let Some(val) = event.get("netskope.alert_v2.src_longitude") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.src_longitude".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.src_longitude", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_longitude_to_double")?;
                        event.rename("netskope.alert_v2.src_longitude", "netskope.alert_v2.src_longitude_keyword")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.src_longitude").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.location.lon", v)?;
            }

            let _cond = { !event.has_value("source.geo.location.lat") || !event.has_value("source.geo.location.lon") || event.get_f64("source.geo.location.lat").is_some_and(|n| n < -90.0) || event.get_f64("source.geo.location.lat").is_some_and(|n| n > 90.0) || event.get_f64("source.geo.location.lon").is_some_and(|n| n < -180.0) || event.get_f64("source.geo.location.lon").is_some_and(|n| n > 180.0) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("source.geo.location").is_none() {
                    return Err(TransformError::FieldNotFound { path: "source.geo.location".into() });
                }
                Ok(())
            })();
            }

            if let Some(v) = event.get("netskope.alert_v2.src_location").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.city_name", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.src_region").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.region_name", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.src_timezone").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.timezone", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.src_zipcode").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.postal_code", v)?;
            }

            let _cond = { event.get_str("json.srcip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.srcip") {
                if let Some(val) = event.get("netskope.alert_v2.srcip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.srcip".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.srcip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_srcip_to_ip")?;
                        if event.remove("netskope.alert_v2.srcip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.srcip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("netskope.alert_v2.srcip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.srcip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("netskope.alert_v2.srcip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("netskope.alert_v2.timestamp") && event.get_str("netskope.alert_v2.timestamp") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.alert_v2.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX", "epoch_second"], None, None) {
                        Some(parsed) => event.set("netskope.alert_v2.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.alert_v2.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
                        if event.remove("netskope.alert_v2.timestamp").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.timestamp".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("netskope.alert_v2.modified_date") && event.get_str("netskope.alert_v2.modified_date") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.alert_v2.modified_date") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX", "epoch_second"], None, None) {
                        Some(parsed) => event.set("netskope.alert_v2.modified_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.alert_v2.modified_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_modified_date")?;
                        if event.remove("netskope.alert_v2.modified_date").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.modified_date".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("netskope.alert_v2.timestamp").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("@timestamp", v)?;
            }

            if event.has_value("netskope.alert_v2.transaction_id") {
                if let Some(val) = event.get("netskope.alert_v2.transaction_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.transaction_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.transaction_id", converted)?;
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.original", v)?;
            }

            if let Some(v) = event.get("netskope.alert_v2.user").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.alert_v2.user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("netskope.alert_v2.act_user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.alert_v2.act_user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("netskope.alert_v2.to_user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.alert_v2.to_user").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.user_confidence_index") {
                if let Some(val) = event.get("netskope.alert_v2.user_confidence_index") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.user_confidence_index".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.user_confidence_index", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_user_confidence_index_to_long")?;
                        if event.remove("netskope.alert_v2.user_confidence_index").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.user_confidence_index".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.user_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.user_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.alert_v2.user_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("netskope.alert_v2.useragent") && event.get_str("netskope.alert_v2.useragent") != Some("") };
            if _cond {
                if let Some(ua_str) = event.get_string("netskope.alert_v2.useragent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                        if let Some(version) = ua.version { event.set("user_agent.version", json!(version))?; }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                            }
                        }
                        if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                    }
                }
            }

            if let Some(v) = event.get("netskope.alert_v2.usergroup").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.group.name", v)?;
            }

            let _cond = { event.has_value("netskope.alert_v2.usergroup") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.alert_v2.usergroup").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.userip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.alert_v2.userip") {
                if let Some(val) = event.get("netskope.alert_v2.userip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.userip".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.userip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_userip_to_ip")?;
                        if event.remove("netskope.alert_v2.userip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.alert_v2.userip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("netskope.alert_v2.userip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("netskope.alert_v2.userip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("netskope.alert_v2.userkey") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.alert_v2.userkey").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.alert_v2.web_url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.url", v)?;
            }

            if event.has_value("netskope.alert_v2.incident_id") {
                if let Some(val) = event.get("netskope.alert_v2.incident_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.incident_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.incident_id", converted)?;
                }
            }

            if event.has_value("netskope.alert_v2.risk_level_id") {
                if let Some(val) = event.get("netskope.alert_v2.risk_level_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.risk_level_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.risk_level_id", converted)?;
                }
            }

            if event.has_value("netskope.alert_v2.severity_id") {
                if let Some(val) = event.get("netskope.alert_v2.severity_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.severity_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.severity_id", converted)?;
                }
            }

            if event.has_value("netskope.alert_v2.tunnel_id") {
                if let Some(val) = event.get("netskope.alert_v2.tunnel_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.tunnel_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.tunnel_id", converted)?;
                }
            }

            if event.has_value("netskope.alert_v2.network_session_id") {
                if let Some(val) = event.get("netskope.alert_v2.network_session_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.alert_v2.network_session_id".into(),
                            message,
                        })?;
                    event.set("netskope.alert_v2.network_session_id", converted)?;
                }
            }

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("netskope.alert_v2._id");
                event.remove("netskope.alert_v2.action");
                event.remove("netskope.alert_v2.alert_name");
                event.remove("netskope.alert_v2.app");
                event.remove("netskope.alert_v2.client_bytes");
                event.remove("netskope.alert_v2.client_packets");
                event.remove("netskope.alert_v2.domain");
                event.remove("netskope.alert_v2.dst_country");
                event.remove("netskope.alert_v2.dst_location");
                event.remove("netskope.alert_v2.dst_region");
                event.remove("netskope.alert_v2.dst_timezone");
                event.remove("netskope.alert_v2.dst_zipcode");
                event.remove("netskope.alert_v2.dsthost");
                event.remove("netskope.alert_v2.dstip");
                event.remove("netskope.alert_v2.dstport");
                event.remove("netskope.alert_v2.email_title");
                event.remove("netskope.alert_v2.end_time");
                event.remove("netskope.alert_v2.file_path");
                event.remove("netskope.alert_v2.file_size");
                event.remove("netskope.alert_v2.file_type");
                event.remove("netskope.alert_v2.hostname");
                event.remove("netskope.alert_v2.local_md5");
                event.remove("netskope.alert_v2.local_sha1");
                event.remove("netskope.alert_v2.local_sha256");
                event.remove("netskope.alert_v2.md5");
                event.remove("netskope.alert_v2.mime_type");
                event.remove("netskope.alert_v2.org");
                event.remove("netskope.alert_v2.os");
                event.remove("netskope.alert_v2.os_family");
                event.remove("netskope.alert_v2.os_version");
                event.remove("netskope.alert_v2.referer");
                event.remove("netskope.alert_v2.server_bytes");
                event.remove("netskope.alert_v2.server_packets");
                event.remove("netskope.alert_v2.severity");
                event.remove("netskope.alert_v2.start_time");
                event.remove("netskope.alert_v2.src_country");
                event.remove("netskope.alert_v2.src_location");
                event.remove("netskope.alert_v2.src_region");
                event.remove("netskope.alert_v2.src_timezone");
                event.remove("netskope.alert_v2.src_zipcode");
                event.remove("netskope.alert_v2.srcip");
                event.remove("netskope.alert_v2.timestamp");
                event.remove("netskope.alert_v2.total_packets");
                event.remove("netskope.alert_v2.url");
                event.remove("netskope.alert_v2.user");
                event.remove("netskope.alert_v2.user_id");
                event.remove("netskope.alert_v2.useragent");
                event.remove("netskope.alert_v2.usergroup");
                event.remove("netskope.alert_v2.web_url");
            }

                event.remove("json");

                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
                drop_empty(event, &DropPolicy { nulls: true, empty_strings: true, empty_collections: true, prune_lists: true, ..DropPolicy::none() }, None);

            event.set("event.kind", json!("alert"))?;

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
                    event.append("error.message", json!(format!("Processor '{}'\n{}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}'\n", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
