// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `sandstorm` pipeline.
pub struct Sandstorm;

impl Transform for Sandstorm {
    fn name(&self) -> &str {
        "sandstorm"
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

            let _cond = { ["Allowed"].contains(&event.get_str("sophos.xg.log_subtype").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { ["pending"].contains(&event.get_str("sophos.xg.reason").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("start"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("sophos.xg.reason") == Some("eligible") };
            if _cond {
                event.append("event.type", json!("end"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Denied") };
            if _cond {
                event.append("event.type", json!("denied"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_component") == Some("Web") };
            if _cond {
                if event.has_value("sophos.xg.source") {
                    event.rename("sophos.xg.source", "url.domain")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.src_ip") };
            if _cond {
                if event.has_value("sophos.xg.src_ip") {
                    event.rename("sophos.xg.src_ip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.user_name") };
            if _cond {
                if event.has_value("sophos.xg.user_name") {
                    event.rename("sophos.xg.user_name", "source.user.name")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("url.domain") {
                if let Some(val) = event.get("url.domain") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "url.domain".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_url_domain_to_destination_ip_01f5ca51")?;
                    if let Some(v) = event.get("url.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                        event.set("destination.domain", v)?;
                    }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("sophos.xg.filename") };
            if _cond {
                if event.has_value("sophos.xg.filename") {
                    event.rename("sophos.xg.filename", "file.name")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.filesize") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.filesize") {
                if let Some(val) = event.get("sophos.xg.filesize") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.filesize".into(),
                            message,
                        })?;
                    event.set("file.size", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.filetype") };
            if _cond {
                if event.has_value("sophos.xg.filetype") {
                    event.rename("sophos.xg.filetype", "file.mime_type")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.sha1sum") && event.get_as_string("sophos.xg.sha1sum").is_some_and(|s| s.len() == 40) };
            if _cond {
                if event.has_value("sophos.xg.sha1sum") {
                    event.rename("sophos.xg.sha1sum", "file.hash.sha1")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.sha1sum") && event.get_as_string("sophos.xg.sha1sum").is_some_and(|s| s.len() == 64) };
            if _cond {
                if event.has_value("sophos.xg.sha1sum") {
                    event.rename("sophos.xg.sha1sum", "file.hash.sha256")?;
                }
            }

                event.remove("sophos.xg.filesize");
                event.remove("sophos.xg.sha1sum");

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
