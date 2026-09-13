// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_gui_logs` pipeline.
pub struct PipelineGuiLogs;

impl Transform for PipelineGuiLogs {
    fn name(&self) -> &str {
        "pipeline_gui_logs"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^req:%{DATA:client.ip:IP} user:%{DATA:user.name} id:%{DATA:event.id} %{NUMBER:http.response.status_code:long} %{WORD:http.request.method} %{DATA:url.path} HTTP/%{NUMBER:http.version} %{GREEDYDATA:user_agent.original}$
                    // Grok pattern: ^Action: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from session %{GREEDYDATA:cisco_secure_email_gateway.log.session} beacuse of inactivity timeout$
                    // Grok pattern: ^Session %{DATA:cisco_secure_email_gateway.log.session} user:%{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.result}$
                    // Grok pattern: ^Session %{DATA:cisco_secure_email_gateway.log.session} from %{IP:host.ip} not found Destination:%{GREEDYDATA:cisco_secure_email_gateway.log.destination}$
                    // Grok pattern: ^SourceIP:%{IP:host.ip} Destination:%{GREEDYDATA:cisco_secure_email_gateway.log.destination} Username:%{USERNAME:user.name} Privilege:%{DATA:cisco_secure_email_gateway.log.privilege} session:%{DATA:cisco_secure_email_gateway.log.session} Action: %{GREEDYDATA:cisco_secure_email_gateway.log.action}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} %{IP:source.ip}:%{NUMBER:source.port:long} - \\(%{GREEDYDATA:cisco_secure_email_gateway.log.description}\\)$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} %{IP:source.ip} port %{NUMBER:source.port:long} - %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^%{DATA:cisco_secure_email_gateway.log.object} has been %{DATA:cisco_secure_email_gateway.log.action} for user %{USERNAME:user.name}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^req:%{DATA:client.ip:IP} user:%{DATA:user.name} id:%{DATA:event.id} %{NUMBER:http.response.status_code:long} %{WORD:http.request.method} %{DATA:url.path} HTTP/%{NUMBER:http.version} %{GREEDYDATA:user_agent.original}$"),
                            cached_grok!("^Action: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from session %{GREEDYDATA:cisco_secure_email_gateway.log.session} beacuse of inactivity timeout$"),
                            cached_grok!("^Session %{DATA:cisco_secure_email_gateway.log.session} user:%{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.result}$"),
                            cached_grok!("^Session %{DATA:cisco_secure_email_gateway.log.session} from %{IP:host.ip} not found Destination:%{GREEDYDATA:cisco_secure_email_gateway.log.destination}$"),
                            cached_grok!("^SourceIP:%{IP:host.ip} Destination:%{GREEDYDATA:cisco_secure_email_gateway.log.destination} Username:%{USERNAME:user.name} Privilege:%{DATA:cisco_secure_email_gateway.log.privilege} session:%{DATA:cisco_secure_email_gateway.log.session} Action: %{GREEDYDATA:cisco_secure_email_gateway.log.action}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} %{IP:source.ip}:%{NUMBER:source.port:long} - \\(%{GREEDYDATA:cisco_secure_email_gateway.log.description}\\)$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} %{IP:source.ip} port %{NUMBER:source.port:long} - %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"),
                            cached_grok!("^%{DATA:cisco_secure_email_gateway.log.object} has been %{DATA:cisco_secure_email_gateway.log.action} for user %{USERNAME:user.name}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            let _cond = { event.get_str("user_agent.original") != Some("-") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();
            }

            let _cond = { event.has_value("http.request.method") };
            if _cond {
            event.set("event.category", Value::Array(vec![json!("web")]))?;
            }

            let _cond = { event.has_value("http.request.method") };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("access")]))?;
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.result") == Some("expired") || event.get_str("cisco_secure_email_gateway.log.action") == Some("logged out") };
            if _cond {
            event.set("event.category", Value::Array(vec![json!("session")]))?;
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.result") == Some("expired") || event.get_str("cisco_secure_email_gateway.log.action") == Some("logged out") };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("end")]))?;
            }

            let _cond = { event.get_str("user.name") == Some("-") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("user.name").is_none() {
                    return Err(TransformError::FieldNotFound { path: "user.name".into() });
                }
                Ok(())
            })();
            }

            let _cond = { event.get_str("user_agent.original") == Some("-") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("user_agent.original").is_none() {
                    return Err(TransformError::FieldNotFound { path: "user_agent.original".into() });
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("-") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
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
