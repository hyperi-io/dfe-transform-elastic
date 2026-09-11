// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `common` pipeline.
pub struct Common;

impl Transform for Common {
    fn name(&self) -> &str {
        "common"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("9.4.0"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("json.timestamp") && event.has_value("json.date") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

                if event.has_value("json.message") {
                    event.rename("json.message", "message")?;
                }

                if event.has_value("json.service") {
                    event.rename("json.service", "service.name")?;
                }

                if event.has_value("json.level") {
                    event.rename("json.level", "log.level")?;
                }

                if event.has_value("json.trace_id") {
                    event.rename("json.trace_id", "trace.id")?;
                }

                if event.has_value("json.span_id") {
                    event.rename("json.span_id", "span.id")?;
                }

            let _cond = { event.get_bool("json.isAuditEvent") == Some(true) && event.has_value("json.request.url") };
            if _cond {
                if event.has_value("json.request.url") {
                    event.rename("json.request.url", "_tmp.common.url")?;
                }
            }

            let _cond = { event.get_bool("json.isAuditEvent") != Some(true) && event.has_value("json.url") };
            if _cond {
                if event.has_value("json.url") {
                    event.rename("json.url", "_tmp.common.url")?;
                }
            }

            let _cond = { event.has_value("_tmp.common.url") };
            if _cond {
            if let Some(v) = event.get("_tmp.common.url").cloned() {
                event.set("url.original", v)?;
            }
            }

            let _cond = { event.has_value("_tmp.common.url") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "_tmp.common.url", "url", true, false)?;
                Ok(())
            })();
            }

                event.remove("url.extension");
                event.remove("url.domain");
                event.remove("url.scheme");

                event.remove("_tmp.common.url");

            let _cond = { event.get_bool("json.isAuditEvent") == Some(true) && event.has_value("json.actor.userAgent") };
            if _cond {
                if event.has_value("json.actor.userAgent") {
                    event.rename("json.actor.userAgent", "_tmp.common.user_agent")?;
                }
            }

            let _cond = { event.get_bool("json.isAuditEvent") != Some(true) && event.has_value("json.userAgent") };
            if _cond {
                if event.has_value("json.userAgent") {
                    event.rename("json.userAgent", "_tmp.common.user_agent")?;
                }
            }

                if event.has_value("_tmp.common.user_agent") {
                    event.rename("_tmp.common.user_agent", "user_agent.original")?;
                }

            let _cond = { event.has_value("user_agent.original") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
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
                Ok(())
            })();
            }

            let _cond = { event.get_bool("json.isAuditEvent") == Some(true) && event.has_value("json.request.method") };
            if _cond {
                if event.has_value("json.request.method") {
                    event.rename("json.request.method", "_tmp.common.http_method")?;
                }
            }

            let _cond = { event.get_bool("json.isAuditEvent") != Some(true) && event.has_value("json.method") };
            if _cond {
                if event.has_value("json.method") {
                    event.rename("json.method", "_tmp.common.http_method")?;
                }
            }

                if event.has_value("_tmp.common.http_method") {
                    event.rename("_tmp.common.http_method", "http.request.method")?;
                }

                event.remove("json.timestamp");
                event.remove("json.date");

                event.remove("_tmp");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Pipeline '{}' failed at processor '{}' {}failed with message '{}'", event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
