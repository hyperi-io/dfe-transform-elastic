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

            event.set("ecs.version", json!("8.11.0"))?;

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

            parse_json_field(event, "event.original", "json")?;

            let _cond = {
                event.get("json.value").is_some_and(|v| match v {
                    serde_json::Value::String(s) => s.is_empty(),
                    serde_json::Value::Array(a) => a.is_empty(),
                    serde_json::Value::Object(o) => o.is_empty(),
                    serde_json::Value::Null => true,
                    _ => false,
                })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("json.comments");
            event.remove("host");
            event.remove("cloud");

            if event.has_value("json.evidence.processCommandLine") {
                event.rename("json.evidence.processCommandLine", "process.command_line")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.process = ctx.process ?: [:];\nctx.process.name = ctx.process.name ?: [];\n// Normalize process.name to a list\ndef nameList = [];\nif (ctx.process.name != null) {\n  if (ctx.process.name instanceof String) {\n    nameList.add(ctx.process.name);\n  } else if (ctx.process.name instanceof List) {\n    nameList.addAll(ctx.process.name);\n  }\n}\n\n// Deduplication using HashSet\ndef currentNames = new HashSet();\ncurrentNames.addAll(nameList);\n// Handle process.command_line (string or list)\nif (ctx.process.command_line != null) {\n  // Convert string to list for unified handling\n  def cmdList = [];\n  if (ctx.process.command_line instanceof String) {\n    cmdList.add(ctx.process.command_line);\n  } else if (ctx.process.command_line instanceof List) {\n    cmdList.addAll(ctx.process.command_line);\n  }\n  for (cmd in cmdList) {\n    if (cmd != null && cmd.length() > 0) {\n      // Extract the first token\n      def parts = cmd.trim().splitOnToken(\" \");\n      if (parts.length > 0) {\n        def executable = parts[0];\n        // If executable is a path, take only the last part\n        if (executable.contains(\"/\")) {\n          def slashParts = executable.splitOnToken(\"/\");\n          executable = slashParts[slashParts.length - 1];\n        }\n        executable = /\\\"/.matcher(executable).replaceAll(\"\");\n        currentNames.add(executable);\n      }\n    }\n  }\n}\n// Update process.name with unique list\nif (currentNames != null && currentNames.size() == 1) {\n  ctx.process.name = currentNames.iterator().next();\n} else {\n  ctx.process.name = new ArrayList(currentNames);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.process = ctx.process ?: [:];\nctx.process.name = ctx.process.name ?: [];\n// Normalize process.name to a list\ndef nameList = [];\nif (ctx.process.name != null) {\n  if (ctx.process.name instanceof String) {\n    nameList.add(ctx.process.name);\n  } else if (ctx.process.name instanceof List) {\n    nameList.addAll(ctx.process.name);\n  }\n}\n\n// Deduplication using HashSet\ndef currentNames = new HashSet();\ncurrentNames.addAll(nameList);\n// Handle process.command_line (string or list)\nif (ctx.process.command_line != null) {\n  // Convert string to list for unified handling\n  def cmdList = [];\n  if (ctx.process.command_line instanceof String) {\n    cmdList.add(ctx.process.command_line);\n  } else if (ctx.process.command_line instanceof List) {\n    cmdList.addAll(ctx.process.command_line);\n  }\n  for (cmd in cmdList) {\n    if (cmd != null && cmd.length() > 0) {\n      // Extract the first token\n      def parts = cmd.trim().splitOnToken(\" \");\n      if (parts.length > 0) {\n        def executable = parts[0];\n        // If executable is a path, take only the last part\n        if (executable.contains(\"/\")) {\n          def slashParts = executable.splitOnToken(\"/\");\n          executable = slashParts[slashParts.length - 1];\n        }\n        executable = /\\\"/.matcher(executable).replaceAll(\"\");\n        currentNames.add(executable);\n      }\n    }\n  }\n}\n// Update process.name with unique list\nif (currentNames != null && currentNames.size() == 1) {\n  ctx.process.name = currentNames.iterator().next();\n} else {\n  ctx.process.name = new ArrayList(currentNames);\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "set_process_name_from_command_line",
                )?;
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

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: if (!ctx.json.empty) {\n  ctx.json.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (!ctx.json.empty) {\n  ctx.json.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));\n}\n"#
                    ),
                    cached_params!("{\"values\":[null,\"\",\"-\",\"N/A\"]}"),
                )?;
            }

            let _cond = { event.has_value("json.evidence") };
            if _cond {
                // Painless script
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\nif (!ctx.json.evidence.empty) {\n  ctx.json.evidence.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));\n}\ndrop(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\nif (!ctx.json.evidence.empty) {\n  ctx.json.evidence.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));\n}\ndrop(ctx);\n"#
                    ),
                    cached_params!("{\"values\":[null,\"\",\"-\",\"N/A\"]}"),
                )?;
            }

            event.set("cloud.provider", json!("azure"))?;

            let _cond = { event.has_value("json.alertUpdateTime") };
            if _cond {
                event.set(
                    "@timestamp",
                    json!(
                        event
                            .get("json.alertUpdateTime")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.aadTenantId") {
                event.rename("json.aadTenantId", "cloud.account.id")?;
            }

            if event.has_value("json.machineId") {
                event.rename("json.machineId", "cloud.instance.id")?;
            }

            if event.has_value("json.title") {
                event.rename("json.title", "message")?;
            }

            event.set("event.kind", json!("alert"))?;

            event.set("event.timezone", json!("UTC"))?;

            let _cond = { event.has_value("json.category") };
            if _cond {
                event.set(
                    "event.action",
                    json!(
                        event
                            .get("json.category")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.set("event.provider", json!("defender_endpoint"))?;

            let _cond = { event.has_value("json.alertCreationTime") };
            if _cond {
                event.set(
                    "event.created",
                    json!(
                        event
                            .get("json.alertCreationTime")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.append("event.category", json!("host"))?;

            let _cond = { event.get_str("json.category") == Some("Malware") };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = { event.get_str("json.evidence.entityType") == Some("Process") };
            if _cond {
                event.append("event.category", json!("process"))?;
            }

            let _cond = { event.get_str("json.evidence.entityType") == Some("User") };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = { event.get_str("json.status") == Some("New") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.get_str("json.status") == Some("Resolved") };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "event.id")?;
            }

            if event.has_value("json.firstEventTime") {
                event.rename("json.firstEventTime", "event.start")?;
            }

            if event.has_value("json.lastEventTime") {
                event.rename("json.lastEventTime", "event.end")?;
            }

            let _cond = { event.get("json.severity").is_some_and(|v| v.is_string()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.json.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.json.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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

            let _cond = { event.has_value("event.start") && event.has_value("event.end") };
            if _cond {
                // Painless script
                // Source: Instant eventstart = ZonedDateTime.parse(ctx.event.start).toInstant(); Instant eventend = ZonedDateTime.parse(ctx.event.end).toInstant(); ctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Instant eventstart = ZonedDateTime.parse(ctx.event.start).toInstant(); Instant eventend = ZonedDateTime.parse(ctx.event.end).toInstant(); ctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("json.category") };
            if _cond {
                event.set("threat.framework", json!("MITRE ATT&CK"))?;
            }

            let _cond = { event.has_value("json.category") };
            if _cond {
                event.append(
                    "threat.technique.name",
                    json!(
                        event
                            .get("json.category")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.description")
                    && event
                        .get_as_string("json.description")
                        .is_some_and(|s| s.len() < 1020)
            };
            if _cond {
                if event.has_value("json.description") {
                    event.rename("json.description", "rule.description")?;
                }
            }

            if event.has_value("json.evidence.fileName") {
                event.rename("json.evidence.fileName", "file.name")?;
            }

            if event.has_value("json.evidence.sha256") {
                event.rename("json.evidence.sha256", "file.hash.sha256")?;
            }

            if event.has_value("json.evidence.sha1") {
                event.rename("json.evidence.sha1", "file.hash.sha1")?;
            }

            if event.has_value("json.evidence.filePath") {
                event.rename("json.evidence.filePath", "file.path")?;
            }

            if event.has_value("json.evidence.processId") {
                event.rename("json.evidence.processId", "process.pid")?;
            }

            if event.has_value("json.evidence.processCreationTime") {
                event.rename("json.evidence.processCreationTime", "process.start")?;
            }

            if event.has_value("json.evidence.parentProcessId") {
                event.rename("json.evidence.parentProcessId", "process.parent.pid")?;
            }

            if event.has_value("json.evidence.parentProcessCreationTime") {
                event.rename(
                    "json.evidence.parentProcessCreationTime",
                    "process.parent.start",
                )?;
            }

            let v = json!(
                event
                    .get("process.pid")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("process.entity_id", v)?;
            }

            let v = json!(
                event
                    .get("process.parent.pid")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("process.parent.entity_id", v)?;
            }

            event.set("observer.product", json!("Defender for Endpoint"))?;

            event.set("observer.vendor", json!("Microsoft"))?;

            if event.has_value("json.detectionSource") {
                event.rename("json.detectionSource", "observer.name")?;
            }

            let _cond = { event.has_value("json.evidence.url") };
            if _cond {
                if event.has_value("json.evidence.url") {
                    event.rename("json.evidence.url", "url.full")?;
                }
            }

            let _cond = { event.has_value("url.full") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "url.full", "url", true, false)?;
                    Ok(())
                })();
            }

            if event.has_value("json.computerDnsName") {
                event.rename("json.computerDnsName", "host.hostname")?;
            }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                map_strings(event, "host.hostname", "host.name", str::to_lowercase)?;
            }

            let _cond = { event.has_value("cloud.instance.id") };
            if _cond {
                if let Some(v) = event
                    .get("cloud.instance.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
            }

            if event.has_value("json.relatedUser.userName") {
                event.rename("json.relatedUser.userName", "user.name")?;
            }

            if event.has_value("json.relatedUser.domainName") {
                event.rename("json.relatedUser.domainName", "user.domain")?;
            }

            if event.has_value("json.evidence.userSid") {
                event.rename("json.evidence.userSid", "user.id")?;
            }

            let _cond = { event.has_value("json.evidence.ipAddress") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("json.evidence.ipAddress")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") && event.get_str("host.name") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json.alertCreationTime");
            event.remove("json.severity");
            event.remove("json.relatedUser");
            event.remove("json.category");

            if event.has_value("json") {
                event.rename("json", "microsoft.defender_endpoint")?;
            }

            if event.has_value("microsoft.defender_endpoint.incidentId") {
                if let Some(val) = event.get("microsoft.defender_endpoint.incidentId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "microsoft.defender_endpoint.incidentId".into(),
                            message,
                        }
                    })?;
                    event.set("microsoft.defender_endpoint.incidentId", converted)?;
                }
            }

            if event.has_value("microsoft.defender_endpoint.investigationId") {
                if let Some(val) = event.get("microsoft.defender_endpoint.investigationId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "microsoft.defender_endpoint.investigationId".into(),
                            message,
                        }
                    })?;
                    event.set("microsoft.defender_endpoint.investigationId", converted)?;
                }
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
