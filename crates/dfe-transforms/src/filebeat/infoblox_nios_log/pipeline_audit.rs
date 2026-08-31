// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_audit` pipeline.
pub struct PipelineAudit;

impl Transform for PipelineAudit {
    fn name(&self) -> &str {
        "pipeline_audit"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Created")), serde_json::Value::String(s) => s.contains("Created"), _ => false }) || event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Modified")), serde_json::Value::String(s) => s.contains("Modified"), _ => false }) || event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Deleted")), serde_json::Value::String(s) => s.contains("Deleted"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{DATA:infoblox_nios.log.audit.object.name} %{DATA:infoblox_nios.log.audit.object.value}:? %{GREEDYDATA:infoblox_nios.log.audit.message}$
                    // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{GREEDYDATA:infoblox_nios.log.audit.message}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.audit.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{DATA:infoblox_nios.log.audit.object.name} %{DATA:infoblox_nios.log.audit.object.value}:? %{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                            cached_grok!("^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.get("message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Called")), serde_json::Value::String(s) => s.contains("Called"), _ => false }) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - %{WORD:infoblox_nios.log.audit.object.name}:? %{GREEDYDATA:infoblox_nios.log.audit.message}$
                    // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - %{GREEDYDATA:infoblox_nios.log.audit.message}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.audit.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - %{WORD:infoblox_nios.log.audit.object.name}:? %{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                            cached_grok!("^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - %{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - - %{GREEDYDATA:details}$
                    // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{GREEDYDATA:infoblox_nios.log.audit.message}$
                    // Grok pattern: ^%{IPORHOST:server.address}: AD authentication for user %{DATA:user.name} (?P<_tmp_ad_auth_failed>(?:failed))$
                    // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} %{GREEDYDATA:infoblox_nios.log.audit.message}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.audit.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - - %{GREEDYDATA:details}$"),
                            cached_grok!("^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                            cached_grok_mapped!("^%{IPORHOST:server.address}: AD authentication for user %{DATA:user.name} (?P<_tmp_ad_auth_failed>(?:failed))$", [("_tmp_ad_auth_failed", "_tmp.ad_auth_failed")]),
                            cached_grok!("^%{GREEDYDATA:_tmp.timestamp} %{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["dd-MMM-yyyy HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSS'Z'"], None, None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tmp_timestamp")?;
                        event.remove("_tmp.timestamp");
                        event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("details") {
                if let Some(kv_str) = event.get_string("details") {
                    for pair in kv_str.split(" ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "details".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("audit.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("event.action") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("event.action") == Some("login_allowed") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("success"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("event.action") == Some("login_allowed") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("start"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("event.action") == Some("login_allowed") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("event.action") == Some("login_denied") || event.has_value("_tmp.ad_auth_failed") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("failure"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("event.action") == Some("login_denied") || event.has_value("_tmp.ad_auth_failed") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("event.action") == Some("logout") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("end"))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("event.action") == Some("logout") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("audit") };
            if _cond {
                // Painless script
                // Source: if (ctx.infoblox_nios == null) {\n  ctx['infoblox_nios'] = new HashMap();\n}\nif (ctx.infoblox_nios.log == null) {\n  ctx.infoblox_nios['log'] = new HashMap();\n}\nif (ctx.infoblox_nios.log.audit == null) {\n  ctx.infoblox_nios.log['audit'] = new HashMap();\n}\nfor (Map.Entry m : ctx.audit.entrySet()) {\n  def value = m.getValue();\n  if (value instanceof String) {\n    value = value.replace('\\\\040', ' ')\n  }\n  ctx.infoblox_nios.log.audit[m.getKey()] = value;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"if (ctx.infoblox_nios == null) {\n  ctx['infoblox_nios'] = new HashMap();\n}\nif (ctx.infoblox_nios.log == null) {\n  ctx.infoblox_nios['log'] = new HashMap();\n}\nif (ctx.infoblox_nios.log.audit == null) {\n  ctx.infoblox_nios.log['audit'] = new HashMap();\n}\nfor (Map.Entry m : ctx.audit.entrySet()) {\n  def value = m.getValue();\n  if (value instanceof String) {\n    value = value.replace('\\\\040', ' ')\n  }\n  ctx.infoblox_nios.log.audit[m.getKey()] = value;\n}\n"#))?;
            }

            let _cond = { event.has_value("infoblox_nios.log.audit.ip") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: String s = ctx.infoblox_nios.log.audit.ip; StringBuilder sb = new StringBuilder(); for (int i = 0; i < s.length();) {\n    if (s.charAt(i) == (char)'\\\\') {\n        sb.append(':');\n        int b = Integer.parseInt(s.substring(i+1,i+4), 8);\n        if (b != (char)':') {\n            sb.append((char)b);\n        }\n        i+=4;\n        continue;\n    }\n    sb.append(s.charAt(i));\n    i++;\n} ctx.infoblox_nios.log.audit.ip = sb.toString();\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"String s = ctx.infoblox_nios.log.audit.ip; StringBuilder sb = new StringBuilder(); for (int i = 0; i < s.length();) {\n    if (s.charAt(i) == (char)'\\\\') {\n        sb.append(':');\n        int b = Integer.parseInt(s.substring(i+1,i+4), 8);\n        if (b != (char)':') {\n            sb.append((char)b);\n        }\n        i+=4;\n        continue;\n    }\n    sb.append(s.charAt(i));\n    i++;\n} ctx.infoblox_nios.log.audit.ip = sb.toString();\n"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_nios.log.audit.ip") && event.get_str("infoblox_nios.log.audit.ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("infoblox_nios.log.audit.ip") {
                if let Some(val) = event.get("infoblox_nios.log.audit.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "infoblox_nios.log.audit.ip".into(),
                            message,
                        })?;
                    event.set("infoblox_nios.log.audit.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("infoblox_nios.log.audit.ip");
                        event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("infoblox_nios.log.audit.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("infoblox_nios.log.audit.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("server.adress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "server.adress".into(),
                            message,
                        })?;
                    event.set("server.ip", converted)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("server.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("server.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            if event.has_value("user.name") {
                gsub_field(event, "user.name", "user.name", cached_regex!("\\\\040"), " ")?;
            }

                event.remove("details");
                event.remove("audit");

            let _cond = { event.has_value("user.name") };
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
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline {} failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
