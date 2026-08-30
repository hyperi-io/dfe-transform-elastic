// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `cfilter` pipeline.
pub struct Cfilter;

impl Transform for Cfilter {
    fn name(&self) -> &str {
        "cfilter"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("sophos.xg.log_subtype") };
            if _cond {
            event.set("event.action", json!(event.get("sophos.xg.log_subtype").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("sophos.xg.log_subtype") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
            if _cond {
            event.set("event.kind", json!("alert"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
            if _cond {
                event.append("event.category", json!("malware"))?;
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") != Some("Denied") };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { ["Allowed", "Warned"].contains(&event.get_str("sophos.xg.log_subtype").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("allowed"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
            if _cond {
                event.append("event.type", json!("info"))?;
                event.append("event.type", json!("denied"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.has_value("sophos.xg.dst_ip") };
            if _cond {
                if event.has_value("sophos.xg.dst_ip") {
                    event.rename("sophos.xg.dst_ip", "destination.ip")?;
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

            let _cond = { event.has_value("sophos.xg.user_name") };
            if _cond {
                if event.has_value("sophos.xg.user_name") {
                    event.rename("sophos.xg.user_name", "source.user.name")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.user_gp") };
            if _cond {
                if event.has_value("sophos.xg.user_gp") {
                    event.rename("sophos.xg.user_gp", "source.user.group.name")?;
                }
            }

                if event.has_value("sophos.xg.url") {
                    event.rename("sophos.xg.url", "url.original")?;
                }

            let _cond = { event.has_value("url.original") };
            if _cond {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            if let Some(v) = event.get("url.original").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.full", v)?;
            }

            let _cond = { !event.has_value("url.domain") };
            if _cond {
                if event.has_value("sophos.xg.domain") {
                    event.rename("sophos.xg.domain", "url.domain")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.referer") };
            if _cond {
                if event.has_value("sophos.xg.referer") {
                    event.rename("sophos.xg.referer", "http.request.referrer")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.status_code") && event.get_str("sophos.xg.status_code") != Some("") };
            if _cond {
            if event.has_value("sophos.xg.status_code") {
                if let Some(val) = event.get("sophos.xg.status_code") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.status_code".into(),
                            message,
                        })?;
                    event.set("http.response.status_code", converted)?;
                }
            }
            }

            let _cond = { event.has_value("sophos.xg.http_status") && event.get_str("sophos.xg.http_status") != Some("") && event.get_str("sophos.xg.http_status") != Some("0") };
            if _cond {
            if event.has_value("sophos.xg.http_status") {
                if let Some(val) = event.get("sophos.xg.http_status") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.http_status".into(),
                            message,
                        })?;
                    event.set("http.response.status_code", converted)?;
                }
            }
            }

                if event.has_value("sophos.xg.user_agent") {
                    event.rename("sophos.xg.user_agent", "user_agent.original")?;
                }

            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
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

                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }

            if let Some(v) = event.get("url.scheme").filter(|v| !painless_is_empty_value(v)).cloned() {
                if !event.has("network.protocol") {
                    event.set("network.protocol", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                Ok(())
            })();

                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");
                event.remove("sophos.xg.domain");
                event.remove("sophos.xg.http_status");
                event.remove("sophos.xg.http_user_agent");

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
