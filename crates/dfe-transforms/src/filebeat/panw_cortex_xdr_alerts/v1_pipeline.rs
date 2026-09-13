// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `v1_pipeline` pipeline.
pub struct V1Pipeline;

impl Transform for V1Pipeline {
    fn name(&self) -> &str {
        "v1_pipeline"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("panw_cortex.xdr.alert_id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("panw_cortex.xdr.events.event_id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("panw_cortex.xdr.events.event_timestamp") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("panw_cortex.xdr.events.event_type") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }

            let _cond = { event.has_value("panw_cortex.xdr.events.event_timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("panw_cortex.xdr.events.event_timestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "panw_cortex.xdr.events.event_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("panw_cortex.xdr.events.agent_host_boot_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("panw_cortex.xdr.events.agent_host_boot_time") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("panw_cortex.xdr.events.agent_host_boot_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "panw_cortex.xdr.events.agent_host_boot_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("panw_cortex.xdr.detection_timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("panw_cortex.xdr.detection_timestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "panw_cortex.xdr.detection_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("panw_cortex.xdr.end_match_attempt_ts") };
            if _cond {
                if let Some(date_str) = event.get_as_string("panw_cortex.xdr.end_match_attempt_ts") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("panw_cortex.xdr.end_match_attempt_ts", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "panw_cortex.xdr.end_match_attempt_ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("panw_cortex.xdr.local_insert_ts") };
            if _cond {
                if let Some(date_str) = event.get_as_string("panw_cortex.xdr.local_insert_ts") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("panw_cortex.xdr.local_insert_ts", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "panw_cortex.xdr.local_insert_ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

                if event.has_value("panw_cortex.xdr.name") {
                    event.rename_over("panw_cortex.xdr.name", "message")?;
                }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("unknown") };
            if _cond {
            event.set("event.severity", json!(0))?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("informational") };
            if _cond {
            event.set("event.severity", json!(1))?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("low") };
            if _cond {
            event.set("event.severity", json!(2))?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("medium") };
            if _cond {
            event.set("event.severity", json!(3))?;
            }

            let _cond = { event.get_str("panw_cortex.xdr.severity") == Some("high") };
            if _cond {
            event.set("event.severity", json!(4))?;
            }

                if event.has_value("panw_cortex.xdr.external_id") {
                    event.rename_over("panw_cortex.xdr.external_id", "event.id")?;
                }

                if event.has_value("panw_cortex.xdr.action") {
                    event.rename_over("panw_cortex.xdr.action", "event.action")?;
                }

            let _cond = { event.get("panw_cortex.xdr.description").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("panw_cortex.xdr.description") {
                    event.rename_over("panw_cortex.xdr.description", "event.reason")?;
                }
            }

            let _cond = { !event.has_value("event.reason") && event.get("panw_cortex.xdr.description").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("panw_cortex.xdr.description") {
                    event.rename_over("panw_cortex.xdr.description", "panw_cortex.xdr.bioc_description")?;
                }
            }

            let _cond = { !event.has_value("event.reason") && event.has_value("panw_cortex.xdr.bioc_description") };
            if _cond {
            event.set("event.reason", json!("Bioc Event"))?;
            }

                if event.has_value("panw_cortex.xdr.agent_device_domain") {
                    event.rename_over("panw_cortex.xdr.agent_device_domain", "host.domain")?;
                }

                if event.has_value("panw_cortex.xdr.agent_fqdn") {
                    event.rename_over("panw_cortex.xdr.agent_fqdn", "host.hostname")?;
                }

            let _cond = { !event.has_value("host.hostname") };
            if _cond {
                if event.has_value("panw_cortex.xdr.host_name") {
                    event.rename_over("panw_cortex.xdr.host_name", "host.hostname")?;
                }
            }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                map_strings(event, "host.hostname", "host.name", str::to_lowercase)?;
            }

                if event.has_value("panw_cortex.xdr.agent_os_type") {
                    event.rename_over("panw_cortex.xdr.agent_os_type", "host.os.name")?;
                }

                if event.has_value("panw_cortex.xdr.agent_os_sub_type") {
                    event.rename_over("panw_cortex.xdr.agent_os_sub_type", "host.os.version")?;
                }

                if event.has_value("panw_cortex.xdr.mac_addresses") {
                    event.rename_over("panw_cortex.xdr.mac_addresses", "host.mac")?;
                }

                if event.has_value("panw_cortex.xdr.host_ip") {
                    event.rename_over("panw_cortex.xdr.host_ip", "host.ip")?;
                }

                if event.has_value("panw_cortex.xdr.endpoint_id") {
                    event.rename_over("panw_cortex.xdr.endpoint_id", "host.id")?;
                }

            let _cond = { !event.has_value("host.mac") };
            if _cond {
            if event.has_value("panw_cortex.xdr.mac") {
                if let Some(s) = event.get_string("panw_cortex.xdr.mac") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("host.mac", Value::Array(parts))?;
                }
            }
            }

            let _cond = { event.has_value("host.mac") };
            if _cond {
                event.remove("panw_cortex.xdr.mac");
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[:.]"), "-")?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

                if event.has_value("panw_cortex.xdr.events.dns_query_name") {
                    event.rename_over("panw_cortex.xdr.events.dns_query_name", "dns.question.name")?;
                }

            let _cond = { event.has_value("panw_cortex.xdr.mitre_technique_id_and_name") };
            if _cond {
                // Painless script
                // Source: void addTechnique(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  if (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n  }\n  if (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n  }\n  if (!ctx.threat.technique.id.contains(x)) {\n    ctx.threat.technique.id.add(x);\n  }\n  if (!ctx.threat.technique.name.contains(y)) {\n    ctx.threat.technique.name.add(y);\n  }\n}\nfor (mitre_technique in ctx.panw_cortex.xdr.mitre_technique_id_and_name) {\n  addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], mitre_technique.splitOnToken(' - ')[1]);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"void addTechnique(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  if (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n  }\n  if (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n  }\n  if (!ctx.threat.technique.id.contains(x)) {\n    ctx.threat.technique.id.add(x);\n  }\n  if (!ctx.threat.technique.name.contains(y)) {\n    ctx.threat.technique.name.add(y);\n  }\n}\nfor (mitre_technique in ctx.panw_cortex.xdr.mitre_technique_id_and_name) {\n  addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], mitre_technique.splitOnToken(' - ')[1]);\n}"#))?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.mitre_tactic_id_and_name") };
            if _cond {
                // Painless script
                // Source: void addTactic(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.tactic == null) {\n    ctx.threat.tactic = new HashMap();\n  }\n  if (ctx.threat.tactic.id == null) {\n    ctx.threat.tactic.id = new ArrayList();\n  }\n  if (ctx.threat.tactic.name == null) {\n    ctx.threat.tactic.name = new ArrayList();\n  }\n  if (!ctx.threat.tactic.id.contains(x)) {\n    ctx.threat.tactic.id.add(x);\n  }\n  if (!ctx.threat.tactic.name.contains(y)) {\n    ctx.threat.tactic.name.add(y);\n  }\n}\nfor (mitre_tactic in ctx.panw_cortex.xdr.mitre_tactic_id_and_name) {\n  addTactic(ctx, mitre_tactic.splitOnToken(' - ')[0], mitre_tactic.splitOnToken(' - ')[1]);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"void addTactic(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.tactic == null) {\n    ctx.threat.tactic = new HashMap();\n  }\n  if (ctx.threat.tactic.id == null) {\n    ctx.threat.tactic.id = new ArrayList();\n  }\n  if (ctx.threat.tactic.name == null) {\n    ctx.threat.tactic.name = new ArrayList();\n  }\n  if (!ctx.threat.tactic.id.contains(x)) {\n    ctx.threat.tactic.id.add(x);\n  }\n  if (!ctx.threat.tactic.name.contains(y)) {\n    ctx.threat.tactic.name.add(y);\n  }\n}\nfor (mitre_tactic in ctx.panw_cortex.xdr.mitre_tactic_id_and_name) {\n  addTactic(ctx, mitre_tactic.splitOnToken(' - ')[0], mitre_tactic.splitOnToken(' - ')[1]);\n}"#))?;
            }

            let _cond = { event.has_value("threat.technique") || event.has_value("threat.tactic") };
            if _cond {
            event.set("threat.framework", json!("MITRE ATT&CK"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("panw_cortex.xdr.events.action_remote_ip") {
                if let Some(val) = event.get("panw_cortex.xdr.events.action_remote_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "panw_cortex.xdr.events.action_remote_ip".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }
                Ok(())
            })();

            if event.has_value("panw_cortex.xdr.events.action_remote_port") {
                if let Some(val) = event.get("panw_cortex.xdr.events.action_remote_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "panw_cortex.xdr.events.action_remote_port".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }

            if event.has_value("panw_cortex.xdr.events.action_local_ip") {
                if let Some(val) = event.get("panw_cortex.xdr.events.action_local_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "panw_cortex.xdr.events.action_local_ip".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            if event.has_value("panw_cortex.xdr.events.action_local_port") {
                if let Some(val) = event.get("panw_cortex.xdr.events.action_local_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "panw_cortex.xdr.events.action_local_port".into(),
                            message,
                        })?;
                    event.set("source.port", converted)?;
                }
            }

                if event.has_value("panw_cortex.xdr.events.action_process_image_sha256") {
                    event.rename_over("panw_cortex.xdr.events.action_process_image_sha256", "process.hash.sha256")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_process_image_command_line") {
                    event.rename_over("panw_cortex.xdr.events.action_process_image_command_line", "process.command_line")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_process_image_name") {
                    event.rename_over("panw_cortex.xdr.events.action_process_image_name", "process.name")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_process_signature_vendor") {
                    event.rename_over("panw_cortex.xdr.events.action_process_signature_vendor", "process.code_signature.subject_name")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_process_signature_status") {
                    event.rename_over("panw_cortex.xdr.events.action_process_signature_status", "process.code_signature.status")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_process_instance_id") {
                    event.rename_over("panw_cortex.xdr.events.action_process_instance_id", "process.entity_id")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_file_path") {
                    event.rename_over("panw_cortex.xdr.events.action_file_path", "file.path")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_file_name") {
                    event.rename_over("panw_cortex.xdr.events.action_file_name", "file.name")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_file_md5") {
                    event.rename_over("panw_cortex.xdr.events.action_file_md5", "file.hash.md5")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_file_sha256") {
                    event.rename_over("panw_cortex.xdr.events.action_file_sha256", "file.hash.sha256")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_registry_key_name") {
                    event.rename_over("panw_cortex.xdr.events.action_registry_key_name", "registry.key")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_registry_value_name") {
                    event.rename_over("panw_cortex.xdr.events.action_registry_value_name", "registry.value")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_registry_full_key") {
                    event.rename_over("panw_cortex.xdr.events.action_registry_full_key", "registry.path")?;
                }

                if event.has_value("panw_cortex.xdr.events.action_registry_data") {
                    event.rename_over("panw_cortex.xdr.events.action_registry_data", "registry.data.strings")?;
                }

            let _cond = { event.get("registry.data.strings").is_some_and(|v| v.is_string()) };
            if _cond {
            event.set("registry.data.strings", Value::Array(vec![json!(event.get("registry.data.strings").map_or_else(String::new, template_to_string))]))?;
            }

                if event.has_value("panw_cortex.xdr.events.actor_process_os_pid") {
                    event.rename_over("panw_cortex.xdr.events.actor_process_os_pid", "process.pid")?;
                }

            let _cond = { !event.has_value("process.entity_id") };
            if _cond {
                if event.has_value("panw_cortex.xdr.events.actor_process_instance_id") {
                    event.rename_over("panw_cortex.xdr.events.actor_process_instance_id", "process.entity_id")?;
                }
            }

                if event.has_value("panw_cortex.xdr.events.actor_process_image_path") {
                    event.rename_over("panw_cortex.xdr.events.actor_process_image_path", "process.executable")?;
                }

            let _cond = { !event.has_value("process.command_line") };
            if _cond {
                if event.has_value("panw_cortex.xdr.events.actor_process_command_line") {
                    event.rename_over("panw_cortex.xdr.events.actor_process_command_line", "process.command_line")?;
                }
            }

            let _cond = { !event.has_value("process.name") };
            if _cond {
                if event.has_value("panw_cortex.xdr.events.actor_process_image_name") {
                    event.rename_over("panw_cortex.xdr.events.actor_process_image_name", "process.name")?;
                }
            }

            let _cond = { !event.has_value("process.code_signature.subject_name") };
            if _cond {
                if event.has_value("panw_cortex.xdr.events.actor_process_signature_vendor") {
                    event.rename_over("panw_cortex.xdr.events.actor_process_signature_vendor", "process.code_signature.subject_name")?;
                }
            }

            let _cond = { !event.has_value("process.hash.sha256") };
            if _cond {
                if event.has_value("panw_cortex.xdr.events.actor_process_image_sha256") {
                    event.rename_over("panw_cortex.xdr.events.actor_process_image_sha256", "process.hash.sha256")?;
                }
            }

                if event.has_value("panw_cortex.xdr.events.actor_process_image_md5") {
                    event.rename_over("panw_cortex.xdr.events.actor_process_image_md5", "process.hash.md5")?;
                }

                if event.has_value("panw_cortex.xdr.events.actor_thread_thread_id") {
                    event.rename_over("panw_cortex.xdr.events.actor_thread_thread_id", "process.thread.id")?;
                }

                if event.has_value("panw_cortex.xdr.events.causality_actor_process_image_name") {
                    event.rename_over("panw_cortex.xdr.events.causality_actor_process_image_name", "process.parent.name")?;
                }

                if event.has_value("panw_cortex.xdr.events.causality_actor_process_image_path") {
                    event.rename_over("panw_cortex.xdr.events.causality_actor_process_image_path", "process.parent.executable")?;
                }

                if event.has_value("panw_cortex.xdr.events.causality_actor_process_image_md5") {
                    event.rename_over("panw_cortex.xdr.events.causality_actor_process_image_md5", "process.parent.hash.md5")?;
                }

                if event.has_value("panw_cortex.xdr.events.causality_actor_process_image_sha256") {
                    event.rename_over("panw_cortex.xdr.events.causality_actor_process_image_sha256", "process.parent.hash.sha256")?;
                }

                if event.has_value("panw_cortex.xdr.events.causality_actor_causality_id") {
                    event.rename_over("panw_cortex.xdr.events.causality_actor_causality_id", "process.parent.entity_id")?;
                }

                if event.has_value("panw_cortex.xdr.events.causality_actor_process_signature_vendor") {
                    event.rename_over("panw_cortex.xdr.events.causality_actor_process_signature_vendor", "process.parent.code_signature.subject_name")?;
                }

                if event.has_value("panw_cortex.xdr.events.causality_actor_process_signature_status") {
                    event.rename_over("panw_cortex.xdr.events.causality_actor_process_signature_status", "process.parent.code_signature.status")?;
                }

                if event.has_value("panw_cortex.xdr.events.causality_actor_process_command_line") {
                    event.rename_over("panw_cortex.xdr.events.causality_actor_process_command_line", "process.parent.command_line")?;
                }

                if event.has_value("panw_cortex.xdr.events.causality_actor_process_execution_time") {
                    event.rename_over("panw_cortex.xdr.events.causality_actor_process_execution_time", "process.parent.uptime")?;
                }

            let _cond = { event.has_value("panw_cortex.xdr.events.user_name") };
            if _cond {
            if event.has_value("panw_cortex.xdr.events.user_name") {
                if let Some(input) = event.get_string("panw_cortex.xdr.events.user_name") {
                    // Grok pattern: ^%{DATA:user.domain}\\\\\\\\%{DATA:user.name}$
                    // Grok pattern: ^%{DATA:user.domain}\\\\%{DATA:user.name}$
                    // Grok pattern: ^%{DATA:user.name}@%{DATA:user.domain}$
                    // Grok pattern: ^%{DATA:user.name}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{DATA:user.domain}\\\\\\\\%{DATA:user.name}$"),
                            cached_grok!("^%{DATA:user.domain}\\\\%{DATA:user.name}$"),
                            cached_grok!("^%{DATA:user.name}@%{DATA:user.domain}$"),
                            cached_grok!("^%{DATA:user.name}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }
            }

            let _cond = { event.get("panw_cortex.xdr.events.user_name").is_some_and(|v| v.is_string()) && event.get("panw_cortex.xdr.events.user_name").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")), serde_json::Value::String(s) => s.contains("@"), _ => false }) && event.get("panw_cortex.xdr.events.user_name").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")), serde_json::Value::String(s) => s.contains("."), _ => false }) };
            if _cond {
            if let Some(v) = event.get("panw_cortex.xdr.events.user_name").cloned() {
                event.set("user.email", v)?;
            }
            }

            let _cond = { event.get("panw_cortex.xdr.events.user_name").is_some_and(|v| v.is_string()) && event.get("panw_cortex.xdr.events.user_name").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")), serde_json::Value::String(s) => s.contains("@"), _ => false }) && event.get("panw_cortex.xdr.events.user_name").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")), serde_json::Value::String(s) => s.contains("."), _ => false }) };
            if _cond {
            if let Some(v) = event.get("panw_cortex.xdr.events.user_name").cloned() {
                event.set("user.id", v)?;
            }
            }

                event.remove("panw_cortex.xdr.events.user_name");

                if event.has_value("panw_cortex.xdr.events.fw_rule") {
                    event.rename_over("panw_cortex.xdr.events.fw_rule", "rule.name")?;
                }

                if event.has_value("panw_cortex.xdr.events.fw_rule_id") {
                    event.rename_over("panw_cortex.xdr.events.fw_rule_id", "rule.id")?;
                }

                if event.has_value("panw_cortex.xdr.events.fw_interface_from") {
                    event.rename_over("panw_cortex.xdr.events.fw_interface_from", "observer.ingress.interface.name")?;
                }

                if event.has_value("panw_cortex.xdr.events.fw_interface_to") {
                    event.rename_over("panw_cortex.xdr.events.fw_interface_to", "observer.egress.interface.name")?;
                }

                if event.has_value("panw_cortex.xdr.events.fw_serial_number") {
                    event.rename_over("panw_cortex.xdr.events.fw_serial_number", "observer.serial_number")?;
                }

            let _cond = { event.has_value("panw_cortex.xdr.events.fw_email_subject") };
            if _cond {
            if let Some(v) = event.get("panw_cortex.xdr.events.fw_email_subject").cloned() {
                event.set("email.subject", v)?;
            }
            }

            let _cond = { event.has_value("panw_cortex.xdr.events.fw_email_sender") };
            if _cond {
                event.append("email.from.address", json!(event.get("panw_cortex.xdr.events.fw_email_sender").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.events.fw_email_recipient") };
            if _cond {
                event.append("email.to.address", json!(event.get("panw_cortex.xdr.events.fw_email_recipient").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

                if event.has_value("source.as.asn") {
                    event.rename_over("source.as.asn", "source.as.number")?;
                }

                if event.has_value("source.as.organization_name") {
                    event.rename_over("source.as.organization_name", "source.as.organization.name")?;
                }

                if event.has_value("destination.as.asn") {
                    event.rename_over("destination.as.asn", "destination.as.number")?;
                }

                if event.has_value("destination.as.organization_name") {
                    event.rename_over("destination.as.organization_name", "destination.as.organization.name")?;
                }

            let _cond = { event.has_value("process.parent.hash.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.parent.hash.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("process.parent.hash.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.parent.hash.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("process.hash.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.hash.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("process.hash.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("process.hash.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("file.hash.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique("related.hash", json!(event.get("file.hash.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("panw_cortex.xdr.tags") };
            if _cond {
                foreach_array(event, "panw_cortex.xdr.tags", |event| {
                    event.append_unique("tags", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

                event.remove("panw_cortex.xdr.host_name");
                event.remove("panw_cortex.xdr.detection_timestamp");
                event.remove("panw_cortex.xdr.events.event_timestamp");
                event.remove("panw_cortex.xdr.severity");
                event.remove("panw_cortex.xdr.events.action_remote_ip");
                event.remove("panw_cortex.xdr.events.action_remote_port");
                event.remove("panw_cortex.xdr.events.action_local_ip");
                event.remove("panw_cortex.xdr.events.action_local_port");
                event.remove("panw_cortex.xdr.events.action_country");
                event.remove("panw_cortex.xdr.bioc_indicator");
                event.remove("panw_cortex.xdr.tags");
                event.remove("panw_cortex.xdr.mitre_technique_id_and_name");
                event.remove("panw_cortex.xdr.mitre_tactic_id_and_name");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
