// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_authentication` pipeline.
pub struct PipelineAuthentication;

impl Transform for PipelineAuthentication {
    fn name(&self) -> &str {
        "pipeline_authentication"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("authentication")]))?;

                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^GUI: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from session %{GREEDYDATA:cisco_secure_email_gateway.log.session} because of inactivity timeout$
                    // Grok pattern: ^CLI: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from %{GREEDYDATA:cisco_secure_email_gateway.log.session} because of inactivity timeout$
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.action}:%{IP:host.ip} user:%{USERNAME:user.name} session:%{WORD:cisco_secure_email_gateway.log.session}$
                    // Grok pattern: ^User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} of %{WORD:network.protocol} session %{IP:host.ip}$
                    // Grok pattern: ^An authentication attempt by the user %{USERNAME:user.name} from %{IP:host.ip} %{WORD:cisco_secure_email_gateway.log.outcome} using an %{WORD:network.protocol} connection\\.$
                    // Grok pattern: ^The user %{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.outcome} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from %{IP:host.ip} with privilege %{DATA:cisco_secure_email_gateway.log.privilege} using an %{WORD:network.protocol} connection\\.$
                    // Grok pattern: ^User %{USERNAME:user.name} was %{WORD:cisco_secure_email_gateway.log.action} %{WORD:cisco_secure_email_gateway.log.outcome}\\.$
                    // Grok pattern: ^User %{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.outcome} %{WORD:cisco_secure_email_gateway.log.action}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^GUI: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from session %{GREEDYDATA:cisco_secure_email_gateway.log.session} because of inactivity timeout$"),
                            cached_grok!("^CLI: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from %{GREEDYDATA:cisco_secure_email_gateway.log.session} because of inactivity timeout$"),
                            cached_grok!("^%{WORD:cisco_secure_email_gateway.log.action}:%{IP:host.ip} user:%{USERNAME:user.name} session:%{WORD:cisco_secure_email_gateway.log.session}$"),
                            cached_grok!("^User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} of %{WORD:network.protocol} session %{IP:host.ip}$"),
                            cached_grok!("^An authentication attempt by the user %{USERNAME:user.name} from %{IP:host.ip} %{WORD:cisco_secure_email_gateway.log.outcome} using an %{WORD:network.protocol} connection\\.$"),
                            cached_grok!("^The user %{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.outcome} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from %{IP:host.ip} with privilege %{DATA:cisco_secure_email_gateway.log.privilege} using an %{WORD:network.protocol} connection\\.$"),
                            cached_grok!("^User %{USERNAME:user.name} was %{WORD:cisco_secure_email_gateway.log.action} %{WORD:cisco_secure_email_gateway.log.outcome}\\.$"),
                            cached_grok!("^User %{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.outcome} %{WORD:cisco_secure_email_gateway.log.action}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
                Ok(())
            })();

            let _cond = { event.get_str("cisco_secure_email_gateway.log.outcome") == Some("failed") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.outcome") == Some("successfully") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.action") == Some("logged on") || event.get_str("cisco_secure_email_gateway.log.action") == Some("authenticated") };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("start")]))?;
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.action") == Some("logged out") || event.get_str("cisco_secure_email_gateway.log.action") == Some("logout") };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("end")]))?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("host.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("host.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

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
