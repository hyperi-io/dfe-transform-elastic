// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            event.set("event.kind", json!("alert"))?;

            event.append_unique("event.category", json!("malware"))?;

            event.append_unique("event.type", json!("info"))?;

            parse_json_field(event, "event.original", "panw_cortex.xdr")?;

            let _cond = { event.get_i64("panw_cortex.xdr.reply.result_count") == Some(0) };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            let _cond =
                { !event.has_value("_conf.api") || event.get_str("_conf.api") == Some("v1") };
            if _cond {
                // Begin nested pipeline: "v1_pipeline"
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
                    if let Some(date_str) =
                        event.get_as_string("panw_cortex.xdr.events.event_timestamp")
                    {
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
                    if let Some(date_str) =
                        event.get_as_string("panw_cortex.xdr.events.agent_host_boot_time")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("panw_cortex.xdr.events.agent_host_boot_time", parsed)?
                            }
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
                    if let Some(date_str) =
                        event.get_as_string("panw_cortex.xdr.detection_timestamp")
                    {
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
                    if let Some(date_str) =
                        event.get_as_string("panw_cortex.xdr.end_match_attempt_ts")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("panw_cortex.xdr.end_match_attempt_ts", parsed)?
                            }
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
                let _cond = {
                    event
                        .get("panw_cortex.xdr.description")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if event.has_value("panw_cortex.xdr.description") {
                        event.rename_over("panw_cortex.xdr.description", "event.reason")?;
                    }
                }
                let _cond = {
                    !event.has_value("event.reason")
                        && event
                            .get("panw_cortex.xdr.description")
                            .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if event.has_value("panw_cortex.xdr.description") {
                        event.rename_over(
                            "panw_cortex.xdr.description",
                            "panw_cortex.xdr.bioc_description",
                        )?;
                    }
                }
                let _cond = {
                    !event.has_value("event.reason")
                        && event.has_value("panw_cortex.xdr.bioc_description")
                };
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
                    event.rename_over(
                        "panw_cortex.xdr.events.dns_query_name",
                        "dns.question.name",
                    )?;
                }
                let _cond = { event.has_value("panw_cortex.xdr.mitre_technique_id_and_name") };
                if _cond {
                    // Painless script
                    // Source: void addTechnique(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  if (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n  }\n  if (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n  }\n  if (!ctx.threat.technique.id.contains(x)) {\n    ctx.threat.technique.id.add(x);\n  }\n  if (!ctx.threat.technique.name.contains(y)) {\n    ctx.threat.technique.name.add(y);\n  }\n}\nfor (mitre_technique in ctx.panw_cortex.xdr.mitre_technique_id_and_name) {\n  addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], mitre_technique.splitOnToken(' - ')[1]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"void addTechnique(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  if (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n  }\n  if (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n  }\n  if (!ctx.threat.technique.id.contains(x)) {\n    ctx.threat.technique.id.add(x);\n  }\n  if (!ctx.threat.technique.name.contains(y)) {\n    ctx.threat.technique.name.add(y);\n  }\n}\nfor (mitre_technique in ctx.panw_cortex.xdr.mitre_technique_id_and_name) {\n  addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], mitre_technique.splitOnToken(' - ')[1]);\n}"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("panw_cortex.xdr.mitre_tactic_id_and_name") };
                if _cond {
                    // Painless script
                    // Source: void addTactic(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.tactic == null) {\n    ctx.threat.tactic = new HashMap();\n  }\n  if (ctx.threat.tactic.id == null) {\n    ctx.threat.tactic.id = new ArrayList();\n  }\n  if (ctx.threat.tactic.name == null) {\n    ctx.threat.tactic.name = new ArrayList();\n  }\n  if (!ctx.threat.tactic.id.contains(x)) {\n    ctx.threat.tactic.id.add(x);\n  }\n  if (!ctx.threat.tactic.name.contains(y)) {\n    ctx.threat.tactic.name.add(y);\n  }\n}\nfor (mitre_tactic in ctx.panw_cortex.xdr.mitre_tactic_id_and_name) {\n  addTactic(ctx, mitre_tactic.splitOnToken(' - ')[0], mitre_tactic.splitOnToken(' - ')[1]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"void addTactic(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.tactic == null) {\n    ctx.threat.tactic = new HashMap();\n  }\n  if (ctx.threat.tactic.id == null) {\n    ctx.threat.tactic.id = new ArrayList();\n  }\n  if (ctx.threat.tactic.name == null) {\n    ctx.threat.tactic.name = new ArrayList();\n  }\n  if (!ctx.threat.tactic.id.contains(x)) {\n    ctx.threat.tactic.id.add(x);\n  }\n  if (!ctx.threat.tactic.name.contains(y)) {\n    ctx.threat.tactic.name.add(y);\n  }\n}\nfor (mitre_tactic in ctx.panw_cortex.xdr.mitre_tactic_id_and_name) {\n  addTactic(ctx, mitre_tactic.splitOnToken(' - ')[0], mitre_tactic.splitOnToken(' - ')[1]);\n}"#
                        ),
                    )?;
                }
                let _cond =
                    { event.has_value("threat.technique") || event.has_value("threat.tactic") };
                if _cond {
                    event.set("threat.framework", json!("MITRE ATT&CK"))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.events.action_remote_ip") {
                        if let Some(val) = event.get("panw_cortex.xdr.events.action_remote_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.events.action_remote_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("panw_cortex.xdr.events.action_remote_port") {
                    if let Some(val) = event.get("panw_cortex.xdr.events.action_remote_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw_cortex.xdr.events.action_remote_port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.port", converted)?;
                    }
                }
                if event.has_value("panw_cortex.xdr.events.action_local_ip") {
                    if let Some(val) = event.get("panw_cortex.xdr.events.action_local_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw_cortex.xdr.events.action_local_ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                if event.has_value("panw_cortex.xdr.events.action_local_port") {
                    if let Some(val) = event.get("panw_cortex.xdr.events.action_local_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "panw_cortex.xdr.events.action_local_port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                if event.has_value("panw_cortex.xdr.events.action_process_image_sha256") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_process_image_sha256",
                        "process.hash.sha256",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.action_process_image_command_line") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_process_image_command_line",
                        "process.command_line",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.action_process_image_name") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_process_image_name",
                        "process.name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.action_process_signature_vendor") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_process_signature_vendor",
                        "process.code_signature.subject_name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.action_process_signature_status") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_process_signature_status",
                        "process.code_signature.status",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.action_process_instance_id") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_process_instance_id",
                        "process.entity_id",
                    )?;
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
                    event.rename_over(
                        "panw_cortex.xdr.events.action_file_sha256",
                        "file.hash.sha256",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.action_registry_key_name") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_registry_key_name",
                        "registry.key",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.action_registry_value_name") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_registry_value_name",
                        "registry.value",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.action_registry_full_key") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_registry_full_key",
                        "registry.path",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.action_registry_data") {
                    event.rename_over(
                        "panw_cortex.xdr.events.action_registry_data",
                        "registry.data.strings",
                    )?;
                }
                let _cond = {
                    event
                        .get("registry.data.strings")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "registry.data.strings",
                        Value::Array(vec![json!(
                            event
                                .get("registry.data.strings")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.actor_process_os_pid") {
                    event.rename_over(
                        "panw_cortex.xdr.events.actor_process_os_pid",
                        "process.pid",
                    )?;
                }
                let _cond = { !event.has_value("process.entity_id") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.events.actor_process_instance_id") {
                        event.rename_over(
                            "panw_cortex.xdr.events.actor_process_instance_id",
                            "process.entity_id",
                        )?;
                    }
                }
                if event.has_value("panw_cortex.xdr.events.actor_process_image_path") {
                    event.rename_over(
                        "panw_cortex.xdr.events.actor_process_image_path",
                        "process.executable",
                    )?;
                }
                let _cond = { !event.has_value("process.command_line") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.events.actor_process_command_line") {
                        event.rename_over(
                            "panw_cortex.xdr.events.actor_process_command_line",
                            "process.command_line",
                        )?;
                    }
                }
                let _cond = { !event.has_value("process.name") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.events.actor_process_image_name") {
                        event.rename_over(
                            "panw_cortex.xdr.events.actor_process_image_name",
                            "process.name",
                        )?;
                    }
                }
                let _cond = { !event.has_value("process.code_signature.subject_name") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.events.actor_process_signature_vendor") {
                        event.rename_over(
                            "panw_cortex.xdr.events.actor_process_signature_vendor",
                            "process.code_signature.subject_name",
                        )?;
                    }
                }
                let _cond = { !event.has_value("process.hash.sha256") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.events.actor_process_image_sha256") {
                        event.rename_over(
                            "panw_cortex.xdr.events.actor_process_image_sha256",
                            "process.hash.sha256",
                        )?;
                    }
                }
                if event.has_value("panw_cortex.xdr.events.actor_process_image_md5") {
                    event.rename_over(
                        "panw_cortex.xdr.events.actor_process_image_md5",
                        "process.hash.md5",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.actor_thread_thread_id") {
                    event.rename_over(
                        "panw_cortex.xdr.events.actor_thread_thread_id",
                        "process.thread.id",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.causality_actor_process_image_name") {
                    event.rename_over(
                        "panw_cortex.xdr.events.causality_actor_process_image_name",
                        "process.parent.name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.causality_actor_process_image_path") {
                    event.rename_over(
                        "panw_cortex.xdr.events.causality_actor_process_image_path",
                        "process.parent.executable",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.causality_actor_process_image_md5") {
                    event.rename_over(
                        "panw_cortex.xdr.events.causality_actor_process_image_md5",
                        "process.parent.hash.md5",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.causality_actor_process_image_sha256") {
                    event.rename_over(
                        "panw_cortex.xdr.events.causality_actor_process_image_sha256",
                        "process.parent.hash.sha256",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.causality_actor_causality_id") {
                    event.rename_over(
                        "panw_cortex.xdr.events.causality_actor_causality_id",
                        "process.parent.entity_id",
                    )?;
                }
                if event
                    .has_value("panw_cortex.xdr.events.causality_actor_process_signature_vendor")
                {
                    event.rename_over(
                        "panw_cortex.xdr.events.causality_actor_process_signature_vendor",
                        "process.parent.code_signature.subject_name",
                    )?;
                }
                if event
                    .has_value("panw_cortex.xdr.events.causality_actor_process_signature_status")
                {
                    event.rename_over(
                        "panw_cortex.xdr.events.causality_actor_process_signature_status",
                        "process.parent.code_signature.status",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.causality_actor_process_command_line") {
                    event.rename_over(
                        "panw_cortex.xdr.events.causality_actor_process_command_line",
                        "process.parent.command_line",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.causality_actor_process_execution_time")
                {
                    event.rename_over(
                        "panw_cortex.xdr.events.causality_actor_process_execution_time",
                        "process.parent.uptime",
                    )?;
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
                let _cond = {
                    event
                        .get("panw_cortex.xdr.events.user_name")
                        .is_some_and(|v| v.is_string())
                        && event
                            .get("panw_cortex.xdr.events.user_name")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("@"))
                                }
                                serde_json::Value::String(s) => s.contains("@"),
                                _ => false,
                            })
                        && event
                            .get("panw_cortex.xdr.events.user_name")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("."))
                                }
                                serde_json::Value::String(s) => s.contains("."),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(v) = event.get("panw_cortex.xdr.events.user_name").cloned() {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.events.user_name")
                        .is_some_and(|v| v.is_string())
                        && event
                            .get("panw_cortex.xdr.events.user_name")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("@"))
                                }
                                serde_json::Value::String(s) => s.contains("@"),
                                _ => false,
                            })
                        && event
                            .get("panw_cortex.xdr.events.user_name")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("."))
                                }
                                serde_json::Value::String(s) => s.contains("."),
                                _ => false,
                            })
                };
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
                    event.rename_over(
                        "panw_cortex.xdr.events.fw_interface_from",
                        "observer.ingress.interface.name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.fw_interface_to") {
                    event.rename_over(
                        "panw_cortex.xdr.events.fw_interface_to",
                        "observer.egress.interface.name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.events.fw_serial_number") {
                    event.rename_over(
                        "panw_cortex.xdr.events.fw_serial_number",
                        "observer.serial_number",
                    )?;
                }
                let _cond = { event.has_value("panw_cortex.xdr.events.fw_email_subject") };
                if _cond {
                    if let Some(v) = event
                        .get("panw_cortex.xdr.events.fw_email_subject")
                        .cloned()
                    {
                        event.set("email.subject", v)?;
                    }
                }
                let _cond = { event.has_value("panw_cortex.xdr.events.fw_email_sender") };
                if _cond {
                    event.append(
                        "email.from.address",
                        json!(
                            event
                                .get("panw_cortex.xdr.events.fw_email_sender")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("panw_cortex.xdr.events.fw_email_recipient") };
                if _cond {
                    event.append(
                        "email.to.address",
                        json!(
                            event
                                .get("panw_cortex.xdr.events.fw_email_recipient")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                    event.rename_over(
                        "source.as.organization_name",
                        "source.as.organization.name",
                    )?;
                }
                if event.has_value("destination.as.asn") {
                    event.rename_over("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename_over(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                let _cond = { event.has_value("process.parent.hash.md5") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.parent.hash.sha256") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.hash.md5") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.hash.sha256") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("file.hash.sha256") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("file.hash.md5") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("panw_cortex.xdr.tags") };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.tags", |event| {
                        event.append_unique(
                            "tags",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
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
                // End nested pipeline: "v1_pipeline"
            }

            let _cond = { event.get_str("_conf.api") == Some("v2") };
            if _cond {
                // Begin nested pipeline: "v2_pipeline"
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("panw_cortex.xdr.alert_id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("panw_cortex.xdr.event_id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("panw_cortex.xdr.event_timestamp") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("panw_cortex.xdr.event_type") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.event_timestamp")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw_cortex.xdr.event_timestamp.0")
                        {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw_cortex.xdr.event_timestamp.0".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_panw_cortex.xdr.event_timestamp.0",
                        )?;
                        event.remove("panw_cortex.xdr.event_timestamp");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.agent_host_boot_time")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw_cortex.xdr.agent_host_boot_time.0")
                        {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => {
                                    event.set("panw_cortex.xdr.agent_host_boot_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw_cortex.xdr.agent_host_boot_time.0".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_panw_cortex.xdr.agent_host_boot_time.0",
                        )?;
                        event.remove("panw_cortex.xdr.agent_host_boot_time");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("panw_cortex.xdr.detection_timestamp") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw_cortex.xdr.detection_timestamp")
                        {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => {
                                    event.set("panw_cortex.xdr.detection_timestamp", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw_cortex.xdr.detection_timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_panw_cortex.xdr.detection_timestamp",
                        )?;
                        event.remove("panw_cortex.xdr.detection_timestamp");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("panw_cortex.xdr.detection_timestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.created", v)?;
                }
                let _cond = { event.has_value("panw_cortex.xdr.end_match_attempt_ts") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw_cortex.xdr.end_match_attempt_ts")
                        {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => {
                                    event.set("panw_cortex.xdr.end_match_attempt_ts", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw_cortex.xdr.end_match_attempt_ts".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_panw_cortex.xdr.end_match_attempt_ts",
                        )?;
                        event.remove("panw_cortex.xdr.end_match_attempt_ts");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("panw_cortex.xdr.local_insert_ts") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw_cortex.xdr.local_insert_ts")
                        {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => {
                                    event.set("panw_cortex.xdr.local_insert_ts", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw_cortex.xdr.local_insert_ts".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_panw_cortex.xdr.local_insert_ts",
                        )?;
                        event.remove("panw_cortex.xdr.local_insert_ts");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("panw_cortex.xdr.last_modified_ts") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("panw_cortex.xdr.last_modified_ts")
                        {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => {
                                    event.set("panw_cortex.xdr.last_modified_ts", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "panw_cortex.xdr.last_modified_ts".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_panw_cortex.xdr.last_modified_ts",
                        )?;
                        event.remove("panw_cortex.xdr.last_modified_ts");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("panw_cortex.xdr.name") {
                    event.rename("panw_cortex.xdr.name", "message")?;
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
                if let Some(v) = event
                    .get("panw_cortex.xdr.external_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                if event.has_value("panw_cortex.xdr.action") {
                    event.rename("panw_cortex.xdr.action", "event.action")?;
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.description")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if event.has_value("panw_cortex.xdr.description") {
                        event.rename("panw_cortex.xdr.description", "event.reason")?;
                    }
                }
                let _cond = {
                    !event.has_value("event.reason")
                        && event
                            .get("panw_cortex.xdr.description")
                            .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if event.has_value("panw_cortex.xdr.description") {
                        event.rename(
                            "panw_cortex.xdr.description",
                            "panw_cortex.xdr.bioc_description",
                        )?;
                    }
                }
                let _cond = {
                    !event.has_value("event.reason")
                        && event.has_value("panw_cortex.xdr.bioc_description")
                };
                if _cond {
                    event.set("event.reason", json!("Bioc Event"))?;
                }
                if event.has_value("panw_cortex.xdr.agent_device_domain") {
                    event.rename("panw_cortex.xdr.agent_device_domain", "host.domain")?;
                }
                if event.has_value("panw_cortex.xdr.agent_fqdn") {
                    event.rename("panw_cortex.xdr.agent_fqdn", "host.hostname")?;
                }
                let _cond = { !event.has_value("host.hostname") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.host_name") {
                        event.rename("panw_cortex.xdr.host_name", "host.hostname")?;
                    }
                }
                let _cond = { event.has_value("host.hostname") };
                if _cond {
                    map_strings(event, "host.hostname", "host.name", str::to_lowercase)?;
                }
                if event.has_value("panw_cortex.xdr.agent_os_type") {
                    event.rename("panw_cortex.xdr.agent_os_type", "host.os.name")?;
                }
                if event.has_value("panw_cortex.xdr.agent_os_sub_type") {
                    event.rename("panw_cortex.xdr.agent_os_sub_type", "host.os.version")?;
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.mac_addresses")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.mac_addresses", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            event.append_unique(
                                "host.mac",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "append")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.mac_addresses")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("panw_cortex.xdr.mac_addresses") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "host.mac",
                        json!(
                            event
                                .get("panw_cortex.xdr.mac_addresses")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("host.mac") {
                    gsub_field(event, "host.mac", "host.mac", cached_regex!("[:.]"), "-")?;
                }
                if event.has_value("host.mac") {
                    map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
                }
                if event.has_value("panw_cortex.xdr.host_ip") {
                    event.rename("panw_cortex.xdr.host_ip", "host.ip")?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.agent_ip_addresses_v6") {
                        if let Some(val) = event.get("panw_cortex.xdr.agent_ip_addresses_v6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.agent_ip_addresses_v6".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.agent_ip_addresses_v6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.agent_ip_addresses_v6",
                    )?;
                    event.remove("panw_cortex.xdr.agent_ip_addresses_v6");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = { event.has_value("panw_cortex.xdr.agent_ip_addresses_v6") };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.agent_ip_addresses_v6", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            event.append_unique(
                                "host.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "append")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                if event.has_value("panw_cortex.xdr.endpoint_id") {
                    event.rename("panw_cortex.xdr.endpoint_id", "host.id")?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.agent_data_collection_status") {
                        if let Some(val) = event.get("panw_cortex.xdr.agent_data_collection_status")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.agent_data_collection_status".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.agent_data_collection_status", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.agent_data_collection_status",
                    )?;
                    event.remove("panw_cortex.xdr.agent_data_collection_status");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.agent_is_vdi") {
                        if let Some(val) = event.get("panw_cortex.xdr.agent_is_vdi") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.agent_is_vdi".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.agent_is_vdi", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.agent_is_vdi",
                    )?;
                    event.remove("panw_cortex.xdr.agent_is_vdi");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("panw_cortex.xdr.dns_query_name") {
                    event.rename("panw_cortex.xdr.dns_query_name", "dns.question.name")?;
                }
                if event.has_value("panw_cortex.xdr.cloud_provider") {
                    event.rename("panw_cortex.xdr.cloud_provider", "cloud.provider")?;
                }
                if event.has_value("panw_cortex.xdr.container_id") {
                    event.rename("panw_cortex.xdr.container_id", "container.id")?;
                }
                let _cond = { event.has_value("panw_cortex.xdr.mitre_technique_id_and_name") };
                if _cond {
                    // Painless script
                    // Source: void addTechnique(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  if (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n  }\n  if (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n  }\n  if (!ctx.threat.technique.id.contains(x)) {\n    ctx.threat.technique.id.add(x);\n  }\n  if (!ctx.threat.technique.name.contains(y)) {\n    ctx.threat.technique.name.add(y);\n  }\n}\nfor (mitre_technique in ctx.panw_cortex.xdr.mitre_technique_id_and_name) {\n  addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], mitre_technique.splitOnToken(' - ')[1]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"void addTechnique(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n  }\n  if (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n  }\n  if (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n  }\n  if (!ctx.threat.technique.id.contains(x)) {\n    ctx.threat.technique.id.add(x);\n  }\n  if (!ctx.threat.technique.name.contains(y)) {\n    ctx.threat.technique.name.add(y);\n  }\n}\nfor (mitre_technique in ctx.panw_cortex.xdr.mitre_technique_id_and_name) {\n  addTechnique(ctx, mitre_technique.splitOnToken(' - ')[0], mitre_technique.splitOnToken(' - ')[1]);\n}"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("panw_cortex.xdr.mitre_tactic_id_and_name") };
                if _cond {
                    // Painless script
                    // Source: void addTactic(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.tactic == null) {\n    ctx.threat.tactic = new HashMap();\n  }\n  if (ctx.threat.tactic.id == null) {\n    ctx.threat.tactic.id = new ArrayList();\n  }\n  if (ctx.threat.tactic.name == null) {\n    ctx.threat.tactic.name = new ArrayList();\n  }\n  if (!ctx.threat.tactic.id.contains(x)) {\n    ctx.threat.tactic.id.add(x);\n  }\n  if (!ctx.threat.tactic.name.contains(y)) {\n    ctx.threat.tactic.name.add(y);\n  }\n}\nfor (mitre_tactic in ctx.panw_cortex.xdr.mitre_tactic_id_and_name) {\n  addTactic(ctx, mitre_tactic.splitOnToken(' - ')[0], mitre_tactic.splitOnToken(' - ')[1]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"void addTactic(def ctx, def x, def y) {\n  if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n  }\n  if (ctx.threat.tactic == null) {\n    ctx.threat.tactic = new HashMap();\n  }\n  if (ctx.threat.tactic.id == null) {\n    ctx.threat.tactic.id = new ArrayList();\n  }\n  if (ctx.threat.tactic.name == null) {\n    ctx.threat.tactic.name = new ArrayList();\n  }\n  if (!ctx.threat.tactic.id.contains(x)) {\n    ctx.threat.tactic.id.add(x);\n  }\n  if (!ctx.threat.tactic.name.contains(y)) {\n    ctx.threat.tactic.name.add(y);\n  }\n}\nfor (mitre_tactic in ctx.panw_cortex.xdr.mitre_tactic_id_and_name) {\n  addTactic(ctx, mitre_tactic.splitOnToken(' - ')[0], mitre_tactic.splitOnToken(' - ')[1]);\n}"#
                        ),
                    )?;
                }
                let _cond =
                    { event.has_value("threat.technique") || event.has_value("threat.tactic") };
                if _cond {
                    event.set("threat.framework", json!("MITRE ATT&CK"))?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.case_id") {
                        if let Some(val) = event.get("panw_cortex.xdr.case_id") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.case_id".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.case_id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.case_id",
                    )?;
                    event.remove("panw_cortex.xdr.case_id");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.action_remote_ip") {
                        if let Some(val) = event.get("panw_cortex.xdr.action_remote_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.action_remote_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.action_remote_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.action_remote_ip",
                    )?;
                    event.remove("panw_cortex.xdr.action_remote_ip");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.action_remote_ip")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.action_remote_ip", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            event.append_unique(
                                "destination.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "append")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.action_remote_ip_v6") {
                        if let Some(val) = event.get("panw_cortex.xdr.action_remote_ip_v6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.action_remote_ip_v6".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.action_remote_ip_v6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.action_remote_ip_v6",
                    )?;
                    event.remove("panw_cortex.xdr.action_remote_ip_v6");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.action_remote_ip_v6")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.action_remote_ip_v6", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            event.append_unique(
                                "destination.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "append")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.action_remote_port") {
                        if let Some(val) = event.get("panw_cortex.xdr.action_remote_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.action_remote_port".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.action_remote_port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.action_remote_port",
                    )?;
                    event.remove("panw_cortex.xdr.action_remote_port");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("panw_cortex.xdr.action_remote_port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "set")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.action_local_ip") {
                        if let Some(val) = event.get("panw_cortex.xdr.action_local_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.action_local_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.action_local_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.action_local_ip",
                    )?;
                    event.remove("panw_cortex.xdr.action_local_ip");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.action_local_ip")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.action_local_ip", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            event.append_unique(
                                "source.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "append")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.action_local_ip_v6") {
                        if let Some(val) = event.get("panw_cortex.xdr.action_local_ip_v6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.action_local_ip_v6".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.action_local_ip_v6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.action_local_ip_v6",
                    )?;
                    event.remove("panw_cortex.xdr.action_local_ip_v6");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.action_local_ip_v6")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.action_local_ip_v6", |event| {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            event.append_unique(
                                "source.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "append")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.action_local_port") {
                        if let Some(val) = event.get("panw_cortex.xdr.action_local_port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.action_local_port".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.action_local_port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.action_local_port",
                    )?;
                    event.remove("panw_cortex.xdr.action_local_port");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("panw_cortex.xdr.action_local_port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "set")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("panw_cortex.xdr.action_process_image_sha256") {
                    event.rename(
                        "panw_cortex.xdr.action_process_image_sha256",
                        "process.hash.sha256",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.action_process_image_command_line") {
                    event.rename(
                        "panw_cortex.xdr.action_process_image_command_line",
                        "process.command_line",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.action_process_image_name") {
                    event.rename("panw_cortex.xdr.action_process_image_name", "process.name")?;
                }
                if event.has_value("panw_cortex.xdr.action_process_signature_vendor") {
                    event.rename(
                        "panw_cortex.xdr.action_process_signature_vendor",
                        "process.code_signature.subject_name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.action_process_signature_status") {
                    event.rename(
                        "panw_cortex.xdr.action_process_signature_status",
                        "process.code_signature.status",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.action_process_instance_id") {
                    event.rename(
                        "panw_cortex.xdr.action_process_instance_id",
                        "process.entity_id",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.action_file_path") {
                    event.rename("panw_cortex.xdr.action_file_path", "file.path")?;
                }
                if event.has_value("panw_cortex.xdr.action_file_name") {
                    event.rename("panw_cortex.xdr.action_file_name", "file.name")?;
                }
                if event.has_value("panw_cortex.xdr.action_file_md5") {
                    event.rename("panw_cortex.xdr.action_file_md5", "file.hash.md5")?;
                }
                if event.has_value("panw_cortex.xdr.action_file_sha256") {
                    event.rename("panw_cortex.xdr.action_file_sha256", "file.hash.sha256")?;
                }
                if event.has_value("panw_cortex.xdr.action_registry_key_name") {
                    event.rename("panw_cortex.xdr.action_registry_key_name", "registry.key")?;
                }
                if event.has_value("panw_cortex.xdr.action_registry_value_name") {
                    event.rename(
                        "panw_cortex.xdr.action_registry_value_name",
                        "registry.value",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.action_registry_full_key") {
                    event.rename("panw_cortex.xdr.action_registry_full_key", "registry.path")?;
                }
                if event.has_value("panw_cortex.xdr.action_registry_data") {
                    event.rename(
                        "panw_cortex.xdr.action_registry_data",
                        "registry.data.strings",
                    )?;
                }
                let _cond = {
                    event
                        .get("registry.data.strings")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.set(
                        "registry.data.strings",
                        Value::Array(vec![json!(
                            event
                                .get("registry.data.strings")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                if event.has_value("panw_cortex.xdr.actor_process_os_pid") {
                    event.rename("panw_cortex.xdr.actor_process_os_pid", "process.pid")?;
                }
                let _cond = { !event.has_value("process.entity_id") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.actor_process_instance_id") {
                        event.rename(
                            "panw_cortex.xdr.actor_process_instance_id",
                            "process.entity_id",
                        )?;
                    }
                }
                if event.has_value("panw_cortex.xdr.actor_process_image_path") {
                    event.rename(
                        "panw_cortex.xdr.actor_process_image_path",
                        "process.executable",
                    )?;
                }
                let _cond = { !event.has_value("process.command_line") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.actor_process_command_line") {
                        event.rename(
                            "panw_cortex.xdr.actor_process_command_line",
                            "process.command_line",
                        )?;
                    }
                }
                let _cond = { !event.has_value("process.name") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.actor_process_image_name") {
                        event.rename("panw_cortex.xdr.actor_process_image_name", "process.name")?;
                    }
                }
                let _cond = { !event.has_value("process.code_signature.subject_name") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.actor_process_signature_vendor") {
                        event.rename(
                            "panw_cortex.xdr.actor_process_signature_vendor",
                            "process.code_signature.subject_name",
                        )?;
                    }
                }
                let _cond = { !event.has_value("process.hash.sha256") };
                if _cond {
                    if event.has_value("panw_cortex.xdr.actor_process_image_sha256") {
                        event.rename(
                            "panw_cortex.xdr.actor_process_image_sha256",
                            "process.hash.sha256",
                        )?;
                    }
                }
                if event.has_value("panw_cortex.xdr.actor_process_image_md5") {
                    event.rename(
                        "panw_cortex.xdr.actor_process_image_md5",
                        "process.hash.md5",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.actor_thread_thread_id") {
                    event.rename(
                        "panw_cortex.xdr.actor_thread_thread_id",
                        "process.thread.id",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.causality_actor_process_image_name") {
                    event.rename(
                        "panw_cortex.xdr.causality_actor_process_image_name",
                        "process.parent.name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.causality_actor_process_image_path") {
                    event.rename(
                        "panw_cortex.xdr.causality_actor_process_image_path",
                        "process.parent.executable",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.causality_actor_process_image_md5") {
                    event.rename(
                        "panw_cortex.xdr.causality_actor_process_image_md5",
                        "process.parent.hash.md5",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.causality_actor_process_image_sha256") {
                    event.rename(
                        "panw_cortex.xdr.causality_actor_process_image_sha256",
                        "process.parent.hash.sha256",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.causality_actor_causality_id") {
                    event.rename(
                        "panw_cortex.xdr.causality_actor_causality_id",
                        "process.parent.entity_id",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.causality_actor_process_signature_vendor") {
                    event.rename(
                        "panw_cortex.xdr.causality_actor_process_signature_vendor",
                        "process.parent.code_signature.subject_name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.causality_actor_process_signature_status") {
                    event.rename(
                        "panw_cortex.xdr.causality_actor_process_signature_status",
                        "process.parent.code_signature.status",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.causality_actor_process_command_line") {
                    event.rename(
                        "panw_cortex.xdr.causality_actor_process_command_line",
                        "process.parent.command_line",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.causality_actor_process_execution_time") {
                    event.rename(
                        "panw_cortex.xdr.causality_actor_process_execution_time",
                        "process.parent.uptime",
                    )?;
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.user_name")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if event.has_value("panw_cortex.xdr.user_name.0") {
                        if let Some(input) = event.get_string("panw_cortex.xdr.user_name.0") {
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
                let _cond = {
                    event.get("panw_cortex.xdr.user_name").is_some_and(|v| v.is_array()) && event.get("panw_cortex.xdr.user_name").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.get("panw_cortex.xdr.user_name.0").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")), serde_json::Value::String(s) => s.contains("@"), _ => false }) && event.get("panw_cortex.xdr.user_name.0").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")), serde_json::Value::String(s) => s.contains("."), _ => false })
                };
                if _cond {
                    if let Some(v) = event.get("panw_cortex.xdr.user_name").cloned() {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = {
                    event.get("panw_cortex.xdr.user_name").is_some_and(|v| v.is_array()) && event.get("panw_cortex.xdr.user_name").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.get("panw_cortex.xdr.user_name.0").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")), serde_json::Value::String(s) => s.contains("@"), _ => false }) && event.get("panw_cortex.xdr.user_name.0").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")), serde_json::Value::String(s) => s.contains("."), _ => false })
                };
                if _cond {
                    if let Some(v) = event.get("panw_cortex.xdr.user_name").cloned() {
                        event.set("user.id", v)?;
                    }
                }
                if let Some(v) = event
                    .get("user")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.user", v)?;
                }
                if event.has_value("panw_cortex.xdr.fw_rule") {
                    event.rename("panw_cortex.xdr.fw_rule", "rule.name")?;
                }
                if event.has_value("panw_cortex.xdr.fw_rule_id") {
                    event.rename("panw_cortex.xdr.fw_rule_id", "rule.id")?;
                }
                if event.has_value("panw_cortex.xdr.fw_interface_from") {
                    event.rename(
                        "panw_cortex.xdr.fw_interface_from",
                        "observer.ingress.interface.name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.fw_interface_to") {
                    event.rename(
                        "panw_cortex.xdr.fw_interface_to",
                        "observer.egress.interface.name",
                    )?;
                }
                if event.has_value("panw_cortex.xdr.fw_serial_number") {
                    event.rename("panw_cortex.xdr.fw_serial_number", "observer.serial_number")?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("panw_cortex.xdr.filter_rule_id") {
                        if let Some(val) = event.get("panw_cortex.xdr.filter_rule_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "panw_cortex.xdr.filter_rule_id".into(),
                                    message,
                                }
                            })?;
                            event.set("panw_cortex.xdr.filter_rule_id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_panw_cortex.xdr.filter_rule_id",
                    )?;
                    event.remove("panw_cortex.xdr.filter_rule_id");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.fw_email_subject")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event.get("panw_cortex.xdr.fw_email_subject").cloned() {
                        event.set("email.subject", v)?;
                    }
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.fw_email_subject")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.fw_email_subject", |event| {
                        event.append_unique(
                            "email.subject",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.fw_email_sender")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("panw_cortex.xdr.fw_email_sender")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.fw_email_sender")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.fw_email_sender", |event| {
                        event.append_unique(
                            "email.from.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.fw_email_recipient")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("panw_cortex.xdr.fw_email_recipient")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("panw_cortex.xdr.fw_email_recipient")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.fw_email_recipient", |event| {
                        event.append_unique(
                            "email.to.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
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
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                let _cond = {
                    event
                        .get("process.parent.hash.md5")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("process.parent.hash.md5")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "process.parent.hash.md5", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("process.parent.hash.sha256")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("process.parent.hash.sha256")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "process.parent.hash.sha256", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = { event.get("process.hash.md5").is_some_and(|v| v.is_string()) };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get("process.hash.md5").is_some_and(|v| v.is_array()) };
                if _cond {
                    foreach_array(event, "process.hash.md5", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("process.hash.sha256")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("process.hash.sha256")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "process.hash.sha256", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = { event.get("file.hash.sha256").is_some_and(|v| v.is_string()) };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get("file.hash.sha256").is_some_and(|v| v.is_array()) };
                if _cond {
                    foreach_array(event, "file.hash.sha256", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("file.hash.md5") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get("file.hash.md5").is_some_and(|v| v.is_array()) };
                if _cond {
                    foreach_array(event, "file.hash.md5", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("panw_cortex.xdr.tags") };
                if _cond {
                    foreach_array(event, "panw_cortex.xdr.tags", |event| {
                        event.append_unique(
                            "tags",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                event.remove("_conf");
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        }))
                };
                if _cond {
                    event.remove("panw_cortex.xdr.detection_timestamp");
                    event.remove("panw_cortex.xdr.event_timestamp");
                    event.remove("panw_cortex.xdr.severity");
                    event.remove("panw_cortex.xdr.action_remote_ip");
                    event.remove("panw_cortex.xdr.action_remote_port");
                    event.remove("panw_cortex.xdr.action_local_ip");
                    event.remove("panw_cortex.xdr.action_local_port");
                    event.remove("panw_cortex.xdr.bioc_indicator");
                    event.remove("panw_cortex.xdr.tags");
                    event.remove("panw_cortex.xdr.mitre_technique_id_and_name");
                    event.remove("panw_cortex.xdr.mitre_tactic_id_and_name");
                    event.remove("panw_cortex.xdr.fw_email_subject");
                    event.remove("panw_cortex.xdr.fw_email_sender");
                    event.remove("panw_cortex.xdr.fw_email_recipient");
                    event.remove("panw_cortex.xdr.external_id");
                    event.remove("panw_cortex.xdr.user_name");
                }
                // End nested pipeline: "v2_pipeline"
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
