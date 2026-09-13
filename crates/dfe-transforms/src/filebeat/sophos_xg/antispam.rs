// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `antispam` pipeline.
pub struct Antispam;

impl Transform for Antispam {
    fn name(&self) -> &str {
        "antispam"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let v = json!(event.get("sophos.xg.log_subtype").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("event.action", v)?;
            }

            let v = json!("success");
            if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
            }

            let _cond = { ["13001", "13002", "13004", "13005", "13006", "13009", "13012", "13014", "14001", "14002", "15001", "15002"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
            event.set("event.kind", json!("alert"))?;
            }

            let _cond = { ["13001", "13002", "13004", "13005", "13006", "13009", "13014", "14001", "14002", "15001", "15002"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.get_str("event.code") == Some("13012") };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

                event.append("event.category", json!("network"))?;

            let _cond = { ["13003", "13007", "13008", "13010", "13013", "14003", "15003", "18035"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("allowed"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { ["13001", "13002", "13004", "13005", "13006", "13009", "13012", "13014", "14001", "14002", "15001", "15002"].contains(&event.get_str("event.code").unwrap_or("")) };
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

                if event.has_value("sophos.xg.src_ip") {
                    event.rename("sophos.xg.src_ip", "source.ip")?;
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

                if event.has_value("sophos.xg.src_domainname") {
                    event.rename("sophos.xg.src_domainname", "source.domain")?;
                }

                if event.has_value("sophos.xg.from_email_address") {
                    event.rename("sophos.xg.from_email_address", "source.user.email")?;
                }

                if event.has_value("sophos.xg.to_email_address") {
                    event.rename("sophos.xg.to_email_address", "destination.user.email")?;
                }

            let _cond = { event.has_value("source.user.email") };
            if _cond {
                event.append("email.from.address", json!(event.get("source.user.email").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("destination.user.email") };
            if _cond {
                event.append("email.to.address", json!(event.get("destination.user.email").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("sophos.xg.email_subject") };
            if _cond {
            if let Some(v) = event.get("sophos.xg.email_subject").cloned() {
                event.set("email.subject", v)?;
            }
            }

            let _cond = { event.has_value("sophos.xg.subject") && !event.has_value("email.subject") };
            if _cond {
            if let Some(v) = event.get("sophos.xg.subject").cloned() {
                event.set("email.subject", v)?;
            }
            }

                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }

            if event.has_value("sophos.xg.log_component") {
                map_strings(event, "sophos.xg.log_component", "network.protocol", str::to_lowercase)?;
            }

                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");
                event.remove("sophos.xg.from_email_address");
                event.remove("sophos.xg.to_email_address");

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
