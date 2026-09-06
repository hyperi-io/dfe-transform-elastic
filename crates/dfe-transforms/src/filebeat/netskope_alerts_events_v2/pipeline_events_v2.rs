// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_events_v2` pipeline.
pub struct PipelineEventsV2;

impl Transform for PipelineEventsV2 {
    fn name(&self) -> &str {
        "pipeline_events_v2"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.17.0"))?;

                if event.has_value("netskope.alerts_events_v2") {
                    event.rename("netskope.alerts_events_v2", "netskope.events_v2")?;
                }

                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("netskope.events_v2._id") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }

                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == '-' || v == 'N/A' || v == 'NotChecked' || v == 'NotAvailable' || v == 'NoSSL'\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == '-' || v == 'N/A' || v == 'NotChecked' || v == 'NotAvailable' || v == 'NoSSL'\n  });\n}\nhandleMap(ctx);
                drop_empty(event, &DropPolicy { prune_lists: true, sentinels: vec!["-".into(), "N/A".into(), "NotChecked".into(), "NotAvailable".into(), "NoSSL".into()], ..DropPolicy::none() }, None);

            event.set("event.kind", json!("event"))?;

            if let Some(v) = event.get("netskope.events_v2._id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.id", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.acting_user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.events_v2.acting_user").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.events_v2.action").filter(|v| !painless_is_empty_value(v)).cloned() {
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

            let _cond = { event.get_str("netskope.events_v2.record_type") == Some("network") };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.get_str("netskope.events_v2.record_type") == Some("network") && event.get_str("event.action") == Some("allow") };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { event.get_str("netskope.events_v2.record_type") == Some("network") && event.get_str("event.action") == Some("block") };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.has_value("netskope.events_v2.activity") && event.get_str("netskope.events_v2.activity").is_some_and(|s| ["success", "pass"].contains(&s.to_lowercase().as_str())) };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.has_value("netskope.events_v2.activity") && event.get_str("netskope.events_v2.activity").is_some_and(|s| s.to_lowercase().contains("fail")) };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
            event.set("event.outcome", json!("unknown"))?;
            }

            if let Some(v) = event.get("netskope.events_v2.app").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.application", v)?;
            }

            if event.has_value("network.application") {
                map_strings(event, "network.application", "network.application", str::to_lowercase)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.app_session_id") {
                if let Some(val) = event.get("netskope.events_v2.app_session_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.app_session_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.app_session_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_app_session_id_to_long")?;
                        if event.remove("netskope.events_v2.app_session_id").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.app_session_id".into() });
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
            if event.has_value("netskope.events_v2.browser_session_id") {
                if let Some(val) = event.get("netskope.events_v2.browser_session_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.browser_session_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.browser_session_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_browser_session_id_to_long")?;
                        if event.remove("netskope.events_v2.browser_session_id").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.browser_session_id".into() });
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
            if event.has_value("netskope.events_v2.cci") {
                if let Some(val) = event.get("netskope.events_v2.cci") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.cci".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.cci", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cci_to_long")?;
                        if event.remove("netskope.events_v2.cci").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.cci".into() });
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
            if event.has_value("netskope.events_v2.client_bytes") {
                if let Some(val) = event.get("netskope.events_v2.client_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.client_bytes".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.client_bytes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_bytes_to_long")?;
                        if event.remove("netskope.events_v2.client_bytes").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.client_bytes".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.client_bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.bytes", v)?;
            }

            if let Some(v) = event.get("client.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.bytes", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.client_packets") {
                if let Some(val) = event.get("netskope.events_v2.client_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.client_packets".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.client_packets", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_packets_to_long")?;
                        if event.remove("netskope.events_v2.client_packets").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.client_packets".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.client_packets").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.packets", v)?;
            }

            if let Some(v) = event.get("client.packets").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.packets", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.conn_duration") {
                if let Some(val) = event.get("netskope.events_v2.conn_duration") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.conn_duration".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.conn_duration", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_conn_duration_to_long")?;
                        if event.remove("netskope.events_v2.conn_duration").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.conn_duration".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("netskope.events_v2.conn_endtime") && event.get_str("netskope.events_v2.conn_endtime") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.events_v2.conn_endtime") {
                    match parse_date_out(&date_str, &["epoch_second"], None, None) {
                        Some(parsed) => event.set("netskope.events_v2.conn_endtime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.events_v2.conn_endtime".into(),
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
                        if event.remove("netskope.events_v2.conn_endtime").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.conn_endtime".into() });
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

            let _cond = { event.has_value("netskope.events_v2.conn_starttime") && event.get_str("netskope.events_v2.conn_starttime") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.events_v2.conn_starttime") {
                    match parse_date_out(&date_str, &["epoch_second"], None, None) {
                        Some(parsed) => event.set("netskope.events_v2.conn_starttime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.events_v2.conn_starttime".into(),
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
                        if event.remove("netskope.events_v2.conn_starttime").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.conn_starttime".into() });
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
            if event.has_value("netskope.events_v2.connection_id") {
                if let Some(val) = event.get("netskope.events_v2.connection_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.connection_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.connection_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_connection_id_to_long")?;
                        if event.remove("netskope.events_v2.connection_id").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.connection_id".into() });
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
            if event.has_value("netskope.events_v2.dlp_incident_id") {
                if let Some(val) = event.get("netskope.events_v2.dlp_incident_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dlp_incident_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dlp_incident_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_incident_id_to_long")?;
                        if event.remove("netskope.events_v2.dlp_incident_id").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.dlp_incident_id".into() });
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
            if event.has_value("netskope.events_v2.dlp_parent_id") {
                if let Some(val) = event.get("netskope.events_v2.dlp_parent_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dlp_parent_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dlp_parent_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_parent_id_to_long")?;
                        if event.remove("netskope.events_v2.dlp_parent_id").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.dlp_parent_id".into() });
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
            if event.has_value("netskope.events_v2.dlp_rule_count") {
                if let Some(val) = event.get("netskope.events_v2.dlp_rule_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dlp_rule_count".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dlp_rule_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_rule_count_to_long")?;
                        if event.remove("netskope.events_v2.dlp_rule_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.dlp_rule_count".into() });
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
            if event.has_value("netskope.events_v2.dlp_is_unique_count") {
                if let Some(val) = event.get("netskope.events_v2.dlp_is_unique_count") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dlp_is_unique_count".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dlp_is_unique_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_is_unique_count_to_boolean")?;
                        if event.remove("netskope.events_v2.dlp_is_unique_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.dlp_is_unique_count".into() });
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
            if event.has_value("netskope.events_v2.dlp_unique_count") {
                if let Some(val) = event.get("netskope.events_v2.dlp_unique_count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dlp_unique_count".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dlp_unique_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dlp_unique_count_to_long")?;
                        if event.remove("netskope.events_v2.dlp_unique_count").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.dlp_unique_count".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("netskope.events_v2.domain_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.domain_ip") {
                if let Some(val) = event.get("netskope.events_v2.domain_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.domain_ip".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.domain_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_domain_ip_to_ip")?;
                        if event.remove("netskope.events_v2.domain_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.domain_ip".into() });
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

            let _cond = { event.has_value("netskope.events_v2.domain_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("netskope.events_v2.domain_ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.events_v2.dst_country").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.country_iso_code", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.dst_geoip_src") {
                if let Some(val) = event.get("netskope.events_v2.dst_geoip_src") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dst_geoip_src".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dst_geoip_src", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_geoip_src_to_long")?;
                        if event.remove("netskope.events_v2.dst_geoip_src").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.dst_geoip_src".into() });
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
            if event.has_value("netskope.events_v2.dst_latitude") {
                if let Some(val) = event.get("netskope.events_v2.dst_latitude") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dst_latitude".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dst_latitude", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_latitude_to_double")?;
                        event.rename("netskope.events_v2.dst_latitude", "netskope.events_v2.dst_latitude_keyword")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.dst_longitude") {
                if let Some(val) = event.get("netskope.events_v2.dst_longitude") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dst_longitude".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dst_longitude", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_longitude_to_double")?;
                        event.rename("netskope.events_v2.dst_longitude", "netskope.events_v2.dst_longitude_keyword")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.dst_latitude").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.location.lat", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.dst_longitude").filter(|v| !painless_is_empty_value(v)).cloned() {
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

            if let Some(v) = event.get("netskope.events_v2.dst_location").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.city_name", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.dst_region").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.region_name", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.dst_timezone").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.timezone", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.dst_zipcode").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.postal_code", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.dsthost").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.domain", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.dsthost") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("netskope.events_v2.dsthost").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("netskope.events_v2.dstip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.dstip") {
                if let Some(val) = event.get("netskope.events_v2.dstip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dstip".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dstip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dstip_to_ip")?;
                        if event.remove("netskope.events_v2.dstip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.dstip".into() });
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

            if let Some(v) = event.get("netskope.events_v2.dstip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.dstip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("netskope.events_v2.dstip").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.dstport") {
                if let Some(val) = event.get("netskope.events_v2.dstport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dstport".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dstport", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dstport_to_long")?;
                        if event.remove("netskope.events_v2.dstport").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.dstport".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.dstport").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.end_time") && event.get_str("netskope.events_v2.end_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.events_v2.end_time") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss", "ISO8601"], None, None) {
                        Some(parsed) => event.set("netskope.events_v2.end_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.events_v2.end_time".into(),
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
                        if event.remove("netskope.events_v2.end_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.end_time".into() });
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

            if let Some(v) = event.get("netskope.events_v2.end_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.end", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.file_path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.path", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.file_size") {
                if let Some(val) = event.get("netskope.events_v2.file_size") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.file_size".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.file_size", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_file_size_to_long")?;
                        if event.remove("netskope.events_v2.file_size").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.file_size".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.file_size").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.from_user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.events_v2.from_user").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.events_v2.hostname").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("netskope.events_v2.hostname").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.incident_id") {
                if let Some(val) = event.get("netskope.events_v2.incident_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.incident_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.incident_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_incident_id_to_long")?;
                        if event.remove("netskope.events_v2.incident_id").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.incident_id".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.ip_protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.transport", v)?;
            }

            if event.has_value("network.transport") {
                map_strings(event, "network.transport", "network.transport", str::to_lowercase)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.latest_incident_id") {
                if let Some(val) = event.get("netskope.events_v2.latest_incident_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.latest_incident_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.latest_incident_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_latest_incident_id_to_long")?;
                        if event.remove("netskope.events_v2.latest_incident_id").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.latest_incident_id".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("netskope.events_v2.md5").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.numbytes") {
                if let Some(val) = event.get("netskope.events_v2.numbytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.numbytes".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.numbytes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_numbytes_to_long")?;
                        if event.remove("netskope.events_v2.numbytes").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.numbytes".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.numbytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.bytes", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.os").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.full", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.os_family").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.family", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.os_user_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.events_v2.os_user_name").map_or_else(String::new, template_to_string)))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("netskope.events_v2.os_version") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("host.os.version", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("netskope.events_v2.owner") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.events_v2.owner").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.pid") {
                if let Some(val) = event.get("netskope.events_v2.pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.pid".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_pid_to_long")?;
                        if event.remove("netskope.events_v2.pid").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.pid".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.pid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pid", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.process_cert_subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.x509.subject.distinguished_name", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.process_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.name", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.referer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.referrer", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.req_cnt") {
                if let Some(val) = event.get("netskope.events_v2.req_cnt") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.req_cnt".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.req_cnt", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_req_cnt_to_long")?;
                        if event.remove("netskope.events_v2.req_cnt").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.req_cnt".into() });
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
            if event.has_value("netskope.events_v2.request_id") {
                if let Some(val) = event.get("netskope.events_v2.request_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.request_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.request_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_request_id_to_long")?;
                        if event.remove("netskope.events_v2.request_id").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.request_id".into() });
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
            if event.has_value("netskope.events_v2.resp_cnt") {
                if let Some(val) = event.get("netskope.events_v2.resp_cnt") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.resp_cnt".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.resp_cnt", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_resp_cnt_to_long")?;
                        if event.remove("netskope.events_v2.resp_cnt").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.resp_cnt".into() });
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
            if event.has_value("netskope.events_v2.response_time") {
                if let Some(val) = event.get("netskope.events_v2.response_time") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.response_time".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.response_time", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_response_time_to_long")?;
                        if event.remove("netskope.events_v2.response_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.response_time".into() });
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
            if event.has_value("netskope.events_v2.server_bytes") {
                if let Some(val) = event.get("netskope.events_v2.server_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.server_bytes".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.server_bytes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_server_bytes_to_long")?;
                        if event.remove("netskope.events_v2.server_bytes").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.server_bytes".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.server_bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.bytes", v)?;
            }

            if let Some(v) = event.get("server.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.bytes", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.server_packets") {
                if let Some(val) = event.get("netskope.events_v2.server_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.server_packets".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.server_packets", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_server_packets_to_long")?;
                        if event.remove("netskope.events_v2.server_packets").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.server_packets".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.server_packets").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("server.packets", v)?;
            }

            if let Some(v) = event.get("server.packets").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.packets", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.session_duration") {
                if let Some(val) = event.get("netskope.events_v2.session_duration") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.session_duration".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.session_duration", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_session_duration_to_long")?;
                        if event.remove("netskope.events_v2.session_duration").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.session_duration".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("netskope.events_v2.severity") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\nctx.event.severity = params.get(ctx.netskope.events_v2.severity.toLowerCase());
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"ctx.event = ctx.event ?: [:];\nctx.event.severity = params.get(ctx.netskope.events_v2.severity.toLowerCase());"#), cached_params!("{\"informational\":21,\"low\":21,\"medium\":47,\"high\":73}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("netskope.events_v2.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.hash.sha256", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.src_geoip_src") {
                if let Some(val) = event.get("netskope.events_v2.src_geoip_src") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.src_geoip_src".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.src_geoip_src", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_geoip_src_to_long")?;
                        if event.remove("netskope.events_v2.src_geoip_src").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.src_geoip_src".into() });
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
            if event.has_value("netskope.events_v2.src_latitude") {
                if let Some(val) = event.get("netskope.events_v2.src_latitude") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.src_latitude".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.src_latitude", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_latitude_to_double")?;
                        event.rename("netskope.events_v2.src_latitude", "netskope.events_v2.src_latitude_keyword")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.src_longitude") {
                if let Some(val) = event.get("netskope.events_v2.src_longitude") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.src_longitude".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.src_longitude", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_longitude_to_double")?;
                        event.rename("netskope.events_v2.src_longitude", "netskope.events_v2.src_longitude_keyword")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.src_latitude").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.location.lat", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.src_longitude").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.location.lon", v)?;
            }

            let _cond = { !(event.get("source.geo.location.lat").is_some_and(|v| v.is_number())) || !(event.get("source.geo.location.lon").is_some_and(|v| v.is_number())) || event.get_f64("source.geo.location.lat").is_some_and(|n| n < -90.0) || event.get_f64("source.geo.location.lat").is_some_and(|n| n > 90.0) || event.get_f64("source.geo.location.lon").is_some_and(|n| n < -180.0) || event.get_f64("source.geo.location.lon").is_some_and(|n| n > 180.0) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("source.geo.location").is_none() {
                    return Err(TransformError::FieldNotFound { path: "source.geo.location".into() });
                }
                Ok(())
            })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.srcport") {
                if let Some(val) = event.get("netskope.events_v2.srcport") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.srcport".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.srcport", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_srcport_to_long")?;
                        if event.remove("netskope.events_v2.srcport").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.srcport".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.src_region").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.region_name", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.src_zipcode").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.postal_code", v)?;
            }

            let _cond = { event.get_str("netskope.events_v2.srcip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.srcip") {
                if let Some(val) = event.get("netskope.events_v2.srcip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.srcip".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.srcip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_srcip_to_ip")?;
                        if event.remove("netskope.events_v2.srcip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.srcip".into() });
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

            if let Some(v) = event.get("netskope.events_v2.srcip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.srcport").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.src_country").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.country_iso_code", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.start_time") && event.get_str("netskope.events_v2.start_time") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.events_v2.start_time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("netskope.events_v2.start_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.events_v2.start_time".into(),
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
                        if event.remove("netskope.events_v2.start_time").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.start_time".into() });
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

            if let Some(v) = event.get("netskope.events_v2.start_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.start", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.timestamp") && event.get_str("netskope.events_v2.timestamp") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("netskope.events_v2.timestamp") {
                    match parse_date_out(&date_str, &["epoch_second"], None, None) {
                        Some(parsed) => event.set("netskope.events_v2.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netskope.events_v2.timestamp".into(),
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
                        if event.remove("netskope.events_v2.timestamp").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.timestamp".into() });
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

            if let Some(v) = event.get("netskope.events_v2.timestamp").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event.get("netskope.events_v2.to_user").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.email", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.to_user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.events_v2.to_user").map_or_else(String::new, template_to_string)))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.total_packets") {
                if let Some(val) = event.get("netskope.events_v2.total_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.total_packets".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.total_packets", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_total_packets_to_long")?;
                        if event.remove("netskope.events_v2.total_packets").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.total_packets".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.total_packets").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.packets", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.transaction_id") {
                if let Some(val) = event.get("netskope.events_v2.transaction_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.transaction_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.transaction_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_transaction_id_to_long")?;
                        if event.remove("netskope.events_v2.transaction_id").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.transaction_id".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("netskope.events_v2.url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.original", v)?;
            }

            let _cond = { event.has_value("netskope.events_v2.url") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("netskope.events_v2.url").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("netskope.events_v2.user").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("user.email") {
                if let Some(input) = event.get_string("user.email") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else { break 'dissect false };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("user.domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "user.email".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set("_ingest.on_failure_processor_tag", "create_user_name_and_user_domain")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("netskope.events_v2.user") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.events_v2.user").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("netskope.events_v2.user_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.events_v2.user_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("netskope.events_v2.useragent") && event.get_str("netskope.events_v2.useragent") != Some("") };
            if _cond {
                if let Some(ua_str) = event.get_string("netskope.events_v2.useragent") {
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

            let _cond = { event.get_str("netskope.events_v2.userip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("netskope.events_v2.userip") {
                if let Some(val) = event.get("netskope.events_v2.userip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.userip".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.userip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_userip_to_ip")?;
                        if event.remove("netskope.events_v2.userip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "netskope.events_v2.userip".into() });
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

            let _cond = { event.has_value("netskope.events_v2.userip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("netskope.events_v2.userip").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("netskope.events_v2.app_session_id") {
                if let Some(val) = event.get("netskope.events_v2.app_session_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.app_session_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.app_session_id", converted)?;
                }
            }

            if event.has_value("netskope.events_v2.dlp_incident_id") {
                if let Some(val) = event.get("netskope.events_v2.dlp_incident_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dlp_incident_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dlp_incident_id", converted)?;
                }
            }

            if event.has_value("netskope.events_v2.connection_id") {
                if let Some(val) = event.get("netskope.events_v2.connection_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.connection_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.connection_id", converted)?;
                }
            }

            if event.has_value("netskope.events_v2.latest_incident_id") {
                if let Some(val) = event.get("netskope.events_v2.latest_incident_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.latest_incident_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.latest_incident_id", converted)?;
                }
            }

            if event.has_value("netskope.events_v2.browser_session_id") {
                if let Some(val) = event.get("netskope.events_v2.browser_session_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.browser_session_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.browser_session_id", converted)?;
                }
            }

            if event.has_value("netskope.events_v2.request_id") {
                if let Some(val) = event.get("netskope.events_v2.request_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.request_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.request_id", converted)?;
                }
            }

            if event.has_value("netskope.events_v2.transaction_id") {
                if let Some(val) = event.get("netskope.events_v2.transaction_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.transaction_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.transaction_id", converted)?;
                }
            }

            if event.has_value("netskope.events_v2.incident_id") {
                if let Some(val) = event.get("netskope.events_v2.incident_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.incident_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.incident_id", converted)?;
                }
            }

            if event.has_value("netskope.events_v2.dlp_parent_id") {
                if let Some(val) = event.get("netskope.events_v2.dlp_parent_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "netskope.events_v2.dlp_parent_id".into(),
                            message,
                        })?;
                    event.set("netskope.events_v2.dlp_parent_id", converted)?;
                }
            }

            let _cond = { event.has_value("netskope.events_v2.assignee") };
            if _cond {
                event.append_unique("related.user", json!(event.get("netskope.events_v2.assignee").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("netskope.events_v2._id");
                event.remove("netskope.events_v2.action");
                event.remove("netskope.events_v2.app");
                event.remove("netskope.events_v2.client_bytes");
                event.remove("netskope.events_v2.client_packets");
                event.remove("netskope.events_v2.dst_country");
                event.remove("netskope.events_v2.dst_location");
                event.remove("netskope.events_v2.dst_region");
                event.remove("netskope.events_v2.dst_timezone");
                event.remove("netskope.events_v2.dst_zipcode");
                event.remove("netskope.events_v2.dsthost");
                event.remove("netskope.events_v2.dstip");
                event.remove("netskope.events_v2.dstport");
                event.remove("netskope.events_v2.end_time");
                event.remove("netskope.events_v2.file_path");
                event.remove("netskope.events_v2.file_size");
                event.remove("netskope.events_v2.hostname");
                event.remove("netskope.events_v2.ip_protocol");
                event.remove("netskope.events_v2.md5");
                event.remove("netskope.events_v2.numbytes");
                event.remove("netskope.events_v2.os");
                event.remove("netskope.events_v2.os_family");
                event.remove("netskope.events_v2.os_version");
                event.remove("netskope.events_v2.pid");
                event.remove("netskope.events_v2.process_cert_subject");
                event.remove("netskope.events_v2.process_name");
                event.remove("netskope.events_v2.referer");
                event.remove("netskope.events_v2.server_bytes");
                event.remove("netskope.events_v2.server_packets");
                event.remove("netskope.events_v2.sha256");
                event.remove("netskope.events_v2.start_time");
                event.remove("netskope.events_v2.timestamp");
                event.remove("netskope.events_v2.to_user");
                event.remove("netskope.events_v2.total_packets");
                event.remove("netskope.events_v2.url");
                event.remove("netskope.events_v2.user");
            }

                // Painless script, resolved to its runners at generation time
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
                drop_empty(event, &DropPolicy { nulls: true, empty_strings: true, empty_collections: true, prune_lists: true, ..DropPolicy::none() }, None);

            let _cond = { event.has_value("error.message") };
            if _cond {
            event.set("event.kind", json!("pipeline_error"))?;
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
                    event.append("error.message", json!(format!("Processor '{}'\n{}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}'\n", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
