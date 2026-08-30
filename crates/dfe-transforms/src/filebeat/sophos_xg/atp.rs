// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `atp` pipeline.
pub struct Atp;

impl Transform for Atp {
    fn name(&self) -> &str {
        "atp"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            let _cond = { event.has_value("sophos.xg.log_subtype") };
            if _cond {
            event.set("event.action", json!(event.get("sophos.xg.log_subtype").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("sophos.xg.log_subtype") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { ["18009", "18010"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
                event.append("event.category", json!("network"))?;
            }

            let _cond = { ["18009", "18010"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("denied"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.has_value("sophos.xg.eventid") };
            if _cond {
                if event.has_value("sophos.xg.eventid") {
                    event.rename("sophos.xg.eventid", "event.id")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.destinationip") };
            if _cond {
                if event.has_value("sophos.xg.destinationip") {
                    event.rename("sophos.xg.destinationip", "destination.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.dst_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.dst_port") {
                if let Some(val) = event.get("sophos.xg.dst_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.dst_port".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.sourceip") };
            if _cond {
                if event.has_value("sophos.xg.sourceip") {
                    event.rename("sophos.xg.sourceip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.src_ip") };
            if _cond {
                if event.has_value("sophos.xg.src_ip") {
                    event.rename("sophos.xg.src_ip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.src_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.src_port") {
                if let Some(val) = event.get("sophos.xg.src_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.src_port".into(),
                            message,
                        })?;
                    event.set("source.port", converted)?;
                }
            }
                Ok(())
            })();
            }

                if event.has_value("sophos.xg.user_name") {
                    event.rename("sophos.xg.user_name", "source.user.name")?;
                }

            let _cond = { event.has_value("sophos.xg.url") };
            if _cond {
                if event.has_value("sophos.xg.url") {
                    event.rename("sophos.xg.url", "url.original")?;
                }
            }

            let _cond = { event.has_value("url.original") && event.get("url.original").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("://")), serde_json::Value::String(s) => s.contains("://"), _ => false }) };
            if _cond {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            let _cond = { event.has_value("url.original") && event.get("url.original").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("://")), serde_json::Value::String(s) => s.contains("://"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("url.original").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.full", v)?;
            }
            }

                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "event.info", "event.info", str::to_lowercase)?;
                Ok(())
            })();

                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
