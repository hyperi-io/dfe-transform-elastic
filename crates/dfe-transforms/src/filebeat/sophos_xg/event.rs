// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `event` pipeline.
pub struct Event;

impl Transform for Event {
    fn name(&self) -> &str {
        "event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Authentication") && event.get_str("sophos.xg.status") == Some("Successful") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Authentication") && event.get_str("sophos.xg.status") == Some("Failed") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Admin") && event.get_str("sophos.xg.status") == Some("Successful") && event.get_str("event.code") == Some("17507") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Admin") && event.get_str("sophos.xg.status") == Some("Failed") && event.get_str("event.code") == Some("17507") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { ["17701", "17704", "17707", "17710", "17713"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { ["17703", "17706", "17709", "17712", "17715"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = { ["SSLVPN", "IPSec", "Thin Client", "Radius SSO"].contains(&event.get_str("sophos.xg.auth_client").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { ["SSLVPN", "IPSec", "Thin Client", "Radius SSO"].contains(&event.get_str("sophos.xg.auth_client").unwrap_or("")) };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Authentication") };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = { event.get_str("event.code") == Some("17819") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("event.code") == Some("17819") };
            if _cond {
                event.append("event.category", json!("host"))?;
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.has_value("sophos.xg.dst_ip") };
            if _cond {
                if event.has_value("sophos.xg.dst_ip") {
                    event.rename("sophos.xg.dst_ip", "destination.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.localinterfaceip") };
            if _cond {
                if event.has_value("sophos.xg.localinterfaceip") {
                    event.rename("sophos.xg.localinterfaceip", "destination.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.src_ip") };
            if _cond {
                if event.has_value("sophos.xg.src_ip") {
                    event.rename("sophos.xg.src_ip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.remoteinterfaceip") };
            if _cond {
                if event.has_value("sophos.xg.remoteinterfaceip") {
                    event.rename("sophos.xg.remoteinterfaceip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.user_name") };
            if _cond {
                if event.has_value("sophos.xg.user_name") {
                    event.rename("sophos.xg.user_name", "source.user.name")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.name") };
            if _cond {
            event.set("source.user.name", json!(event.get("sophos.xg.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("sophos.xg.log_subtype") == Some("Authentication") };
            if _cond {
            let v = json!(event.get("source.user.name").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("user.name", v)?;
            }
            }

            let _cond = { event.has_value("sophos.xg.usergroupname") };
            if _cond {
                if event.has_value("sophos.xg.usergroupname") {
                    event.rename("sophos.xg.usergroupname", "source.user.group.name")?;
                }
            }

                if event.has_value("sophos.xg.message") {
                    event.rename("sophos.xg.message", "message")?;
                }

                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.src_port");
                event.remove("sophos.xg.name");

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
