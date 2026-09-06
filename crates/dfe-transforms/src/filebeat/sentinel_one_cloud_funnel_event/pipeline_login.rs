// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_login` pipeline.
pub struct PipelineLogin;

impl Transform for PipelineLogin {
    fn name(&self) -> &str {
        "pipeline_login"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.category", Value::Array(vec![json!("authentication")]))?;

            let _cond = { event.get_str("sentinel_one_cloud_funnel.event.type") == Some("Login") };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("start")]))?;
            }

            let _cond = { event.get_str("sentinel_one_cloud_funnel.event.type") == Some("Logout") };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("end")]))?;
            }

            if !event.has("event.type") {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

                if event.has_value("json.event.login.userName") {
                    event.rename("json.event.login.userName", "sentinel_one_cloud_funnel.event.login.user_name")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.login.user_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.src.endpoint.ip.address") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.src.endpoint.ip.address") {
                if let Some(val) = event.get("json.src.endpoint.ip.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.src.endpoint.ip.address".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.src.endpoint_ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_src_endpoint_ip_address")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.src.endpoint_ip_address") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("sentinel_one_cloud_funnel.event.src.endpoint_ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.src.endpoint_ip_address") };
            if _cond {
                event.append_unique("host.ip", json!(event.get("sentinel_one_cloud_funnel.event.src.endpoint_ip_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.src.endpoint_ip_address") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("sentinel_one_cloud_funnel.event.src.endpoint_ip_address").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.event.login.accountDomain") {
                    event.rename("json.event.login.accountDomain", "sentinel_one_cloud_funnel.event.login.account.domain")?;
                }

                if event.has_value("json.event.login.accountName") {
                    event.rename("json.event.login.accountName", "sentinel_one_cloud_funnel.event.login.account.name")?;
                }

                if event.has_value("json.event.login.accountSid") {
                    event.rename("json.event.login.accountSid", "sentinel_one_cloud_funnel.event.login.account.sid")?;
                }

                if event.has_value("json.event.login.baseType") {
                    event.rename("json.event.login.baseType", "sentinel_one_cloud_funnel.event.login.base_type")?;
                }

                if event.has_value("json.event.login.failureReason") {
                    event.rename("json.event.login.failureReason", "sentinel_one_cloud_funnel.event.login.failure_reason")?;
                }

            let _cond = { event.get_str("json.event.login.isAdministratorEquivalent") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.event.login.isAdministratorEquivalent") {
                if let Some(val) = event.get("json.event.login.isAdministratorEquivalent") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event.login.isAdministratorEquivalent".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.login.is_administrator_equivalent", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_event_login_isAdministratorEquivalent")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.event.login.loginIsSuccessful") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.event.login.loginIsSuccessful") {
                if let Some(val) = event.get("json.event.login.loginIsSuccessful") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event.login.loginIsSuccessful".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.login.is_successful", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_event_login_loginIsSuccessful")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.event.login.sessionId") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.event.login.sessionId") {
                if let Some(val) = event.get("json.event.login.sessionId") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event.login.sessionId".into(),
                            message,
                        })?;
                    event.set("sentinel_one_cloud_funnel.event.login.session_id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_event_login_sessionId")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.event.login.type") {
                    event.rename("json.event.login.type", "sentinel_one_cloud_funnel.event.login.type")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
