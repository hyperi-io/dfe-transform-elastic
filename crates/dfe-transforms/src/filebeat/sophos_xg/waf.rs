// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `waf` pipeline.
pub struct Waf;

impl Transform for Waf {
    fn name(&self) -> &str {
        "waf"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.get_str("sophos.xg.reason") == Some("-") };
            if _cond {
            event.set("event.action", json!("allowed"))?;
            }

            let _cond = { event.get_str("sophos.xg.reason") != Some("-") };
            if _cond {
            event.set("event.action", json!("denied"))?;
            }

            let _cond = { event.has_value("sophos.xg.reason") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("sophos.xg.reason") != Some("-") };
            if _cond {
            event.set("event.kind", json!("alert"))?;
            }

            let _cond = { event.get_str("sophos.xg.reason") == Some("Antivirus") };
            if _cond {
                event.append("event.category", json!("malware"))?;
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.get_str("sophos.xg.reason") != Some("Antivirus") && event.get_str("sophos.xg.reason") != Some("-") };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.get_str("sophos.xg.reason") == Some("-") };
            if _cond {
                event.append("event.type", json!("allowed"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("sophos.xg.reason") != Some("-") };
            if _cond {
                event.append("event.type", json!("denied"))?;
                event.append("event.type", json!("connection"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("sophos.xg.responsetime") {
                if let Some(val) = event.get("sophos.xg.responsetime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.responsetime".into(),
                            message,
                        })?;
                    event.set("sophos.xg.responsetime", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_sophos_xg_responsetime_04a3f9c7")?;
                        if event.remove("sophos.xg.responsetime").is_none() {
                            return Err(TransformError::FieldNotFound { path: "sophos.xg.responsetime".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                // Painless script
                // Source: if (ctx.sophos?.xg?.responsetime != null && ctx.sophos.xg.responsetime > 0) {\n  ctx.event.duration = ctx.sophos.xg.responsetime * 1000; \n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"if (ctx.sophos?.xg?.responsetime != null && ctx.sophos.xg.responsetime > 0) {\n  ctx.event.duration = ctx.sophos.xg.responsetime * 1000; \n}\n"#))?;

            let _cond = { event.has_value("sophos.xg.localip") };
            if _cond {
                if event.has_value("sophos.xg.localip") {
                    event.rename("sophos.xg.localip", "destination.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.bytessent") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.bytessent") {
                if let Some(val) = event.get("sophos.xg.bytessent") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.bytessent".into(),
                            message,
                        })?;
                    event.set("destination.bytes", converted)?;
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

            let _cond = { event.has_value("sophos.xg.bytesrcv") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.bytesrcv") {
                if let Some(val) = event.get("sophos.xg.bytesrcv") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.bytesrcv".into(),
                            message,
                        })?;
                    event.set("source.bytes", converted)?;
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

            let _cond = { event.has_value("sophos.xg.url") };
            if _cond {
                if event.has_value("sophos.xg.url") {
                    event.rename("sophos.xg.url", "url.full")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.domain") };
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

            let _cond = { event.has_value("sophos.xg.httpstatus") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.httpstatus") {
                if let Some(val) = event.get("sophos.xg.httpstatus") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.httpstatus".into(),
                            message,
                        })?;
                    event.set("destination.bytes", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.method") };
            if _cond {
                if event.has_value("sophos.xg.method") {
                    event.rename("sophos.xg.method", "http.request.method")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.ws_protocol") };
            if _cond {
                if event.has_value("sophos.xg.ws_protocol") {
                    event.rename("sophos.xg.ws_protocol", "http.version")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.useragent") };
            if _cond {
                if event.has_value("sophos.xg.useragent") {
                    event.rename("sophos.xg.useragent", "user_agent.original")?;
                }
            }

                if event.has_value("sophos.xg.SQLi") {
                    event.rename("sophos.xg.SQLi", "sophos.xg.sqli")?;
                }

                if event.has_value("sophos.xg.XSS") {
                    event.rename("sophos.xg.XSS", "sophos.xg.xss")?;
                }

                event.remove("sophos.xg.bytesrcv");
                event.remove("sophos.xg.bytessent");
                event.remove("sophos.xg.httpstatus");
                event.remove("sophos.xg.responsetime");

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
