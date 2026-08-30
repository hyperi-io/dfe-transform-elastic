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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("json") };
            if _cond {
                event.rename("json", "jamf_protect.alerts")?;
            }

            let _cond =
                { event.has_value("event.original") && !event.has_value("jamf_protect.alerts") };
            if _cond {
                parse_json_field(event, "event.original", "jamf_protect.alerts")?;
            }

            let _cond = {
                !event.has_value("event.original")
                    && event.has_value("jamf_protect.alerts")
                    && event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\nctx.event.original = Json.dump(ctx.jamf_protect.alerts);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event = ctx.event ?: [:];\nctx.event.original = Json.dump(ctx.jamf_protect.alerts);"#
                    ),
                )?;
            }

            event.set("event.kind", json!("alert"))?;

            event.set("event.provider", json!("Jamf Protect"))?;

            let _cond = { event.has_value("jamf_protect.alerts.input.match.event.timestamp") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("jamf_protect.alerts.input.match.event.timestamp")
                {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "jamf_protect.alerts.input.match.event.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.append("event.category", json!("host"))?;

            let _cond = {
                event.get_str("jamf_protect.alerts.input.eventType")
                    == Some("GPThreatMatchExecEvent")
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond =
                { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPProcessEvent") };
            if _cond {
                event.append("event.category", json!("process"))?;
            }

            let _cond =
                { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPFSEvent") };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond =
                { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPDownloadEvent") };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = {
                event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| v.is_array()) && event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } != 0) && event.has_value("jamf_protect.alerts.input.match.facts.0.name")
            };
            if _cond {
                // Painless script
                // Source: if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n}\nctx.rule.name = ctx.jamf_protect.alerts.input.match.facts[0].name\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n}\nctx.rule.name = ctx.jamf_protect.alerts.input.match.facts[0].name\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| v.is_array()) && event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } != 0) && event.has_value("jamf_protect.alerts.input.match.facts.0.human")
            };
            if _cond {
                // Painless script
                // Source: if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n}\nctx.rule.description = ctx.jamf_protect.alerts.input.match.facts[0].human\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.rule == null) {\n  ctx.rule = new HashMap();\n}\nctx.rule.description = ctx.jamf_protect.alerts.input.match.facts[0].human\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| v.is_array()) && event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.has("jamf_protect.alerts.input.match.facts.0.name")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event.action = ctx.jamf_protect.alerts.input.match.facts[0].name;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event.action = ctx.jamf_protect.alerts.input.match.facts[0].name;\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| v.is_array()) && event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.has("jamf_protect.alerts.input.match.facts.0.human")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event.reason = ctx.jamf_protect.alerts.input.match.facts[0].human;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event.reason = ctx.jamf_protect.alerts.input.match.facts[0].human;\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| v.is_array()) && event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.get("jamf_protect.alerts.input.match.facts.0.tags").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: if (!(ctx.tags instanceof List) || ctx.tags.size() == 0) {\n  ctx.tags = ctx.jamf_protect.alerts.input.match.facts[0].tags;\n  return;\n}  HashSet tags = new HashSet(ctx.tags); for (def t: ctx.jamf_protect.alerts.input.match.facts[0].tags) {\n  tags.add(t);\n} ctx.tags = tags;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (!(ctx.tags instanceof List) || ctx.tags.size() == 0) {\n  ctx.tags = ctx.jamf_protect.alerts.input.match.facts[0].tags;\n  return;\n}  HashSet tags = new HashSet(ctx.tags); for (def t: ctx.jamf_protect.alerts.input.match.facts[0].tags) {\n  tags.add(t);\n} ctx.tags = tags;\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("jamf_protect.alerts.input.match.uuid") };
            if _cond {
                if event.has_value("jamf_protect.alerts.input.match.uuid") {
                    event.rename("jamf_protect.alerts.input.match.uuid", "event.id")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.alerts.input.match.severity") };
            if _cond {
                if event.has_value("jamf_protect.alerts.input.match.severity") {
                    event.rename("jamf_protect.alerts.input.match.severity", "event.severity")?;
                }
            }

            let _cond = {
                event.get_str("jamf_protect.alerts.input.match.actions.0.name") == Some("Prevented")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("jamf_protect.alerts.input.match.actions.0.name") == Some("Report")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { !event.has_value("jamf_protect.alerts.input.match.actions.0.name") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = { event.has_value("jamf_protect.alerts.input.host.hostname") };
            if _cond {
                if event.has_value("jamf_protect.alerts.input.host.hostname") {
                    event.rename("jamf_protect.alerts.input.host.hostname", "host.hostname")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.alerts.input.host.provisioningUDID") };
            if _cond {
                if event.has_value("jamf_protect.alerts.input.host.provisioningUDID") {
                    event.rename("jamf_protect.alerts.input.host.provisioningUDID", "host.id")?;
                }
            }

            let _cond = {
                event.has_value("jamf_protect.alerts.input.host.ips")
                    && event.get_str("jamf_protect.alerts.input.host.ips") != Some("")
            };
            if _cond {
                if event.has_value("jamf_protect.alerts.input.host.ips") {
                    event.rename("jamf_protect.alerts.input.host.ips", "host.ip")?;
                }
            }

            let _cond = { event.has_value("jamf_protect.alerts.input.host.os") };
            if _cond {
                if event.has_value("jamf_protect.alerts.input.host.os") {
                    event.rename("jamf_protect.alerts.input.host.os", "host.os.full")?;
                }
            }

            event.set("host.os.family", json!("macos"))?;

            // Painless script
            // Source: if (ctx.jamf_protect?.alerts?.input?.related?.users != null && ctx.jamf_protect.alerts.input.related.users.size() > 0) {\n    ArrayList userNames = new ArrayList();\n\n    for (def user : ctx.jamf_protect.alerts.input.related.users) {\n        if (user.containsKey('name') && user['name'] != null) {\n            userNames.add(user['name']);\n        }\n    }\n    if (userNames.size() > 0) {\n        ctx.related = ctx.related ?: new HashMap();\n        ctx.related.user = userNames;\n    }\n}  \n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.jamf_protect?.alerts?.input?.related?.users != null && ctx.jamf_protect.alerts.input.related.users.size() > 0) {\n    ArrayList userNames = new ArrayList();\n\n    for (def user : ctx.jamf_protect.alerts.input.related.users) {\n        if (user.containsKey('name') && user['name'] != null) {\n            userNames.add(user['name']);\n        }\n    }\n    if (userNames.size() > 0) {\n        ctx.related = ctx.related ?: new HashMap();\n        ctx.related.user = userNames;\n    }\n}  \n"#
                ),
            )?;

            let _cond = {
                event.get_str("jamf_protect.alerts.input.eventType") == Some("GPFSEvent")
                    && event.get_i64("jamf_protect.alerts.input.match.event.type") == Some(0)
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.get_str("jamf_protect.alerts.input.eventType") == Some("GPFSEvent")
                    && event.get_i64("jamf_protect.alerts.input.match.event.type") == Some(1)
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.get_str("jamf_protect.alerts.input.eventType") == Some("GPFSEvent")
                    && event.get_i64("jamf_protect.alerts.input.match.event.type") == Some(3)
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.get_str("jamf_protect.alerts.input.eventType") == Some("GPFSEvent")
                    && event.get_i64("jamf_protect.alerts.input.match.event.type") == Some(4)
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.get_str("jamf_protect.alerts.input.eventType") == Some("GPFSEvent")
                    && event.get_i64("jamf_protect.alerts.input.match.event.type") == Some(7)
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("jamf_protect.alerts.input.related.files") && event.get("jamf_protect.alerts.input.related.files").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: if (ctx.jamf_protect.alerts.input.related.files.size() > 0) {\n    def file = ctx.jamf_protect.alerts.input.related.files[0];\n\n    ctx.file = ctx.file ?: new HashMap();\n    \n    ctx.file.path = file.path;\n    ctx.file.size = file.size;\n    ctx.file.inode = String.valueOf(file.inode);\n    ctx.file.gid = String.valueOf(file.gid);\n    ctx.file.mode = String.valueOf(file.mode);\n    ctx.file.uid = String.valueOf(file.uid);\n    \n    ctx.file.hash = ctx.file.hash ?: new HashMap();\n    ctx.file.hash.sha1 = file.sha1hex;\n    ctx.file.hash.sha256 = file.sha256hex;\n    \n    ctx.file.code_signature = ctx.file.code_signature ?: new HashMap();\n    ctx.file.code_signature.signing_id = file.signingInfo?.appid; // Use safe navigation for nested objects\n    ctx.file.code_signature.status = file.signingInfo?.statusMessage;\n    ctx.file.code_signature.team_id = file.signingInfo?.teamid;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.jamf_protect.alerts.input.related.files.size() > 0) {\n    def file = ctx.jamf_protect.alerts.input.related.files[0];\n\n    ctx.file = ctx.file ?: new HashMap();\n    \n    ctx.file.path = file.path;\n    ctx.file.size = file.size;\n    ctx.file.inode = String.valueOf(file.inode);\n    ctx.file.gid = String.valueOf(file.gid);\n    ctx.file.mode = String.valueOf(file.mode);\n    ctx.file.uid = String.valueOf(file.uid);\n    \n    ctx.file.hash = ctx.file.hash ?: new HashMap();\n    ctx.file.hash.sha1 = file.sha1hex;\n    ctx.file.hash.sha256 = file.sha256hex;\n    \n    ctx.file.code_signature = ctx.file.code_signature ?: new HashMap();\n    ctx.file.code_signature.signing_id = file.signingInfo?.appid; // Use safe navigation for nested objects\n    ctx.file.code_signature.status = file.signingInfo?.statusMessage;\n    ctx.file.code_signature.team_id = file.signingInfo?.teamid;\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get_str("jamf_protect.alerts.input.eventType") == Some("GPProcessEvent")
                    && event.get_i64("jamf_protect.alerts.input.match.event.type") == Some(1)
            };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = {
                event.get_str("jamf_protect.alerts.input.eventType") == Some("GPProcessEvent")
                    && event.get_i64("jamf_protect.alerts.input.match.event.type") == Some(2)
            };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

            let _cond = {
                event.has_value("jamf_protect.alerts.input.related.processes") && event.get("jamf_protect.alerts.input.related.processes").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Painless script
                // Source: if (ctx.jamf_protect?.alerts?.input?.related?.processes != null && ctx.jamf_protect.alerts.input.related.processes.size() > 0) {\n    def process = ctx.jamf_protect.alerts.input.related.processes[0];\n    def binary = ctx.jamf_protect.alerts.input.related.binaries[0];\n    \n    ctx.process = ctx.process ?: new HashMap();                          \n    ctx.process.name = process.name;\n    ctx.process.executable = process.path;\n    ctx.process.pid = process.pid;\n    if (process.containsKey('startTimestamp')) {\n        ctx.process.start = Instant.ofEpochSecond(process.startTimestamp).toString();\n    }\n    if (process?.exitCode != null) {\n        ctx.process.exit_code = process.exitCode;\n    }\n    ctx.process.args = process.args ?: new ArrayList();\n    ctx.process.entity_id = process.uuid;\n\n    ctx.process.parent = new HashMap();    \n    ctx.process.parent.pid = process.responsiblePID;\n\n    ctx.process.user = new HashMap();\n    ctx.process.user.id = process.uid.toString();\n\n    ctx.process.group_leader = new HashMap();\n    if (process?.pgid != null) {\n        ctx.process.group_leader.pid = process.pgid;\n    }\n    ctx.process.group_leader.group = new HashMap();\n    if (process?.gid != null) {\n        ctx.process.group_leader.group.id = process.gid.toString();\n    }\n\n    ctx.process.real_user = new HashMap();\n    if (process?.ruid != null) {\n        ctx.process.real_user.id = process.ruid.toString();\n    }               \n\n    ctx.process.real_group = new HashMap();\n    if (process?.rgid != null) {\n        ctx.process.real_group.id = process.rgid.toString();\n    }      \n\n    ctx.process.hash = ctx.process.hash ?: new HashMap();\n    if (binary?.sha1hex != null) {\n        ctx.process.hash.sha1 = binary.sha1hex;\n    }\n    if (binary?.sha256hex != null) {\n        ctx.process.hash.sha256 = binary.sha256hex;\n    }\n    \n    ctx.process.code_signature = ctx.process.code_signature ?: new HashMap();\n    if (process.signingInfo?.appid != null) {\n        ctx.process.code_signature.signing_id = process.signingInfo.appid;\n    }\n    if (process.signingInfo?.statusMessage != null) {\n        ctx.process.code_signature.status = process.signingInfo.statusMessage;\n    }\n    if (process?.signingInfo?.teamid != null) {\n        ctx.process.code_signature.team_id = process.signingInfo.teamid;\n    }\n\n    // Mapping out the parent process\n    if (ctx.jamf_protect.alerts.input.related.processes.size() > 1) {\n    def parentProcess = ctx.jamf_protect.alerts.input.related.processes[1];\n\n    ctx.process.parent = new HashMap();\n    ctx.process.parent.name = parentProcess.name;\n    ctx.process.parent.pid = parentProcess.pid;\n    ctx.process.parent.executable = parentProcess.path;\n    ctx.process.parent.entity_id = parentProcess.uuid;\n\n    if (parentProcess.containsKey('startTimestamp')) {\n        ctx.process.parent.start = Instant.ofEpochSecond(parentProcess.startTimestamp).toString();\n    }\n\n    ctx.process.parent.user = new HashMap();\n    if (parentProcess?.uid != null) {\n        ctx.process.parent.user.id = parentProcess.uid.toString();\n    }      \n\n    ctx.process.parent.real_user = new HashMap();\n    if (parentProcess?.ruid != null) {\n        ctx.process.parent.real_user.id = parentProcess.ruid.toString();\n    }               \n\n    ctx.process.parent.real_group = new HashMap();\n    if (parentProcess?.rgid != null) {\n        ctx.process.parent.real_group.id = parentProcess.rgid.toString();\n    }\n\n    ctx.process.parent.code_signature = ctx.process.parent.code_signature ?: new HashMap();\n    if (parentProcess.signingInfo?.appid != null) {\n        ctx.process.parent.code_signature.signing_id = parentProcess.signingInfo.appid;\n    }\n    if (parentProcess.signingInfo?.statusMessage != null) {\n        ctx.process.parent.code_signature.status = parentProcess.signingInfo.statusMessage;\n    }\n    if (parentProcess?.signingInfo?.teamid != null) {\n        ctx.process.parent.code_signature.team_id = parentProcess.signingInfo.teamid;\n    }\n\n    }\n\n    // Mapping out the process group leader, which can be the same as parent\n    def processGroupLeader = ctx.jamf_protect.alerts.input.related.processes[ctx.jamf_protect.alerts.input.related.processes.size() - 1];\n    ctx.process.group_leader = new HashMap();\n    ctx.process.group_leader.name = processGroupLeader.name;\n    ctx.process.group_leader.pid = processGroupLeader.pid;\n    ctx.process.group_leader.executable = processGroupLeader.path;\n\n    if (processGroupLeader.containsKey('startTimestamp')) {\n        ctx.process.group_leader.start = Instant.ofEpochSecond(processGroupLeader.startTimestamp).toString();\n    }\n\n    ctx.process.group_leader.user = new HashMap();\n    if (processGroupLeader?.uid != null) {\n        ctx.process.group_leader.user.id = processGroupLeader.uid.toString();\n    }      \n\n    ctx.process.group_leader.real_user = new HashMap();\n    if (processGroupLeader?.ruid != null) {\n        ctx.process.group_leader.real_user.id = processGroupLeader.ruid.toString();\n    }               \n\n    ctx.process.group_leader.real_group = new HashMap();\n    if (processGroupLeader?.rgid != null) {\n        ctx.process.group_leader.real_group.id = processGroupLeader.rgid.toString();\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.jamf_protect?.alerts?.input?.related?.processes != null && ctx.jamf_protect.alerts.input.related.processes.size() > 0) {\n    def process = ctx.jamf_protect.alerts.input.related.processes[0];\n    def binary = ctx.jamf_protect.alerts.input.related.binaries[0];\n    \n    ctx.process = ctx.process ?: new HashMap();                          \n    ctx.process.name = process.name;\n    ctx.process.executable = process.path;\n    ctx.process.pid = process.pid;\n    if (process.containsKey('startTimestamp')) {\n        ctx.process.start = Instant.ofEpochSecond(process.startTimestamp).toString();\n    }\n    if (process?.exitCode != null) {\n        ctx.process.exit_code = process.exitCode;\n    }\n    ctx.process.args = process.args ?: new ArrayList();\n    ctx.process.entity_id = process.uuid;\n\n    ctx.process.parent = new HashMap();    \n    ctx.process.parent.pid = process.responsiblePID;\n\n    ctx.process.user = new HashMap();\n    ctx.process.user.id = process.uid.toString();\n\n    ctx.process.group_leader = new HashMap();\n    if (process?.pgid != null) {\n        ctx.process.group_leader.pid = process.pgid;\n    }\n    ctx.process.group_leader.group = new HashMap();\n    if (process?.gid != null) {\n        ctx.process.group_leader.group.id = process.gid.toString();\n    }\n\n    ctx.process.real_user = new HashMap();\n    if (process?.ruid != null) {\n        ctx.process.real_user.id = process.ruid.toString();\n    }               \n\n    ctx.process.real_group = new HashMap();\n    if (process?.rgid != null) {\n        ctx.process.real_group.id = process.rgid.toString();\n    }      \n\n    ctx.process.hash = ctx.process.hash ?: new HashMap();\n    if (binary?.sha1hex != null) {\n        ctx.process.hash.sha1 = binary.sha1hex;\n    }\n    if (binary?.sha256hex != null) {\n        ctx.process.hash.sha256 = binary.sha256hex;\n    }\n    \n    ctx.process.code_signature = ctx.process.code_signature ?: new HashMap();\n    if (process.signingInfo?.appid != null) {\n        ctx.process.code_signature.signing_id = process.signingInfo.appid;\n    }\n    if (process.signingInfo?.statusMessage != null) {\n        ctx.process.code_signature.status = process.signingInfo.statusMessage;\n    }\n    if (process?.signingInfo?.teamid != null) {\n        ctx.process.code_signature.team_id = process.signingInfo.teamid;\n    }\n\n    // Mapping out the parent process\n    if (ctx.jamf_protect.alerts.input.related.processes.size() > 1) {\n    def parentProcess = ctx.jamf_protect.alerts.input.related.processes[1];\n\n    ctx.process.parent = new HashMap();\n    ctx.process.parent.name = parentProcess.name;\n    ctx.process.parent.pid = parentProcess.pid;\n    ctx.process.parent.executable = parentProcess.path;\n    ctx.process.parent.entity_id = parentProcess.uuid;\n\n    if (parentProcess.containsKey('startTimestamp')) {\n        ctx.process.parent.start = Instant.ofEpochSecond(parentProcess.startTimestamp).toString();\n    }\n\n    ctx.process.parent.user = new HashMap();\n    if (parentProcess?.uid != null) {\n        ctx.process.parent.user.id = parentProcess.uid.toString();\n    }      \n\n    ctx.process.parent.real_user = new HashMap();\n    if (parentProcess?.ruid != null) {\n        ctx.process.parent.real_user.id = parentProcess.ruid.toString();\n    }               \n\n    ctx.process.parent.real_group = new HashMap();\n    if (parentProcess?.rgid != null) {\n        ctx.process.parent.real_group.id = parentProcess.rgid.toString();\n    }\n\n    ctx.process.parent.code_signature = ctx.process.parent.code_signature ?: new HashMap();\n    if (parentProcess.signingInfo?.appid != null) {\n        ctx.process.parent.code_signature.signing_id = parentProcess.signingInfo.appid;\n    }\n    if (parentProcess.signingInfo?.statusMessage != null) {\n        ctx.process.parent.code_signature.status = parentProcess.signingInfo.statusMessage;\n    }\n    if (parentProcess?.signingInfo?.teamid != null) {\n        ctx.process.parent.code_signature.team_id = parentProcess.signingInfo.teamid;\n    }\n\n    }\n\n    // Mapping out the process group leader, which can be the same as parent\n    def processGroupLeader = ctx.jamf_protect.alerts.input.related.processes[ctx.jamf_protect.alerts.input.related.processes.size() - 1];\n    ctx.process.group_leader = new HashMap();\n    ctx.process.group_leader.name = processGroupLeader.name;\n    ctx.process.group_leader.pid = processGroupLeader.pid;\n    ctx.process.group_leader.executable = processGroupLeader.path;\n\n    if (processGroupLeader.containsKey('startTimestamp')) {\n        ctx.process.group_leader.start = Instant.ofEpochSecond(processGroupLeader.startTimestamp).toString();\n    }\n\n    ctx.process.group_leader.user = new HashMap();\n    if (processGroupLeader?.uid != null) {\n        ctx.process.group_leader.user.id = processGroupLeader.uid.toString();\n    }      \n\n    ctx.process.group_leader.real_user = new HashMap();\n    if (processGroupLeader?.ruid != null) {\n        ctx.process.group_leader.real_user.id = processGroupLeader.ruid.toString();\n    }               \n\n    ctx.process.group_leader.real_group = new HashMap();\n    if (processGroupLeader?.rgid != null) {\n        ctx.process.group_leader.real_group.id = processGroupLeader.rgid.toString();\n    }\n}\n"#
                    ),
                )?;
            }

            // Painless script
            // Source: if (ctx.jamf_protect?.alerts?.input?.related?.groups != null && ctx.jamf_protect.alerts.input.related.groups.size() > 0) {\n    def group = ctx.jamf_protect.alerts.input.related.groups[0];\n    \n        ctx.group = ctx.group ?: new HashMap();\n        \n        ctx.group.name = group.name;\n        ctx.group.id = group.gid.toString();\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.jamf_protect?.alerts?.input?.related?.groups != null && ctx.jamf_protect.alerts.input.related.groups.size() > 0) {\n    def group = ctx.jamf_protect.alerts.input.related.groups[0];\n    \n        ctx.group = ctx.group ?: new HashMap();\n        \n        ctx.group.name = group.name;\n        ctx.group.id = group.gid.toString();\n}\n"#
                ),
            )?;

            let _cond = {
                event
                    .get("jamf_protect.alerts.input.match.facts.0.tags")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("MITREattack"))
                        }
                        serde_json::Value::String(s) => s.contains("MITREattack"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("threat.framework", json!("MITRE ATT&CK"))?;
            }

            let _cond = {
                event
                    .get("jamf_protect.alerts.input.match.facts.0.tags")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("MITREattack"))
                        }
                        serde_json::Value::String(s) => s.contains("MITREattack"),
                        _ => false,
                    })
            };
            if _cond {
                event.set(
                    "threat.software.platforms",
                    Value::Array(vec![json!("macOS")]),
                )?;
            }

            let _cond = { event.has_value("host.name") };
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

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "host.ip", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append_unique(
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
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.sha1")
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

            let _cond =
                { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") };
            if _cond {
                // Begin nested pipeline: "gpusbevent"
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event.has_value("jamf_protect.alerts.input.match.event.device.bsdName")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.bsdName",
                        "volume.nt_name",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event.has_value("jamf_protect.alerts.input.match.event.device.content")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.content",
                        "volume.file_system_type",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event.has_value("jamf_protect.alerts.input.match.event.device.busName")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.busName",
                        "volume.bus_type",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event
                            .has_value("jamf_protect.alerts.input.match.event.device.productName")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.productName",
                        "volume.product_name",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event.has_value("jamf_protect.alerts.input.match.event.device.productId")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.productId",
                        "volume.product_id",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event
                            .has_value("jamf_protect.alerts.input.match.event.device.isRemovable")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.isRemovable",
                        "volume.removable",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event
                            .has_value("jamf_protect.alerts.input.match.event.device.serialNumber")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.serialNumber",
                        "volume.serial_number",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event.has_value("jamf_protect.alerts.input.match.event.device.size")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.size",
                        "volume.size",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event.has_value("jamf_protect.alerts.input.match.event.device.vendorId")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.vendorId",
                        "volume.vendor_id",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event
                            .has_value("jamf_protect.alerts.input.match.event.device.vendorName")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.vendorName",
                        "volume.vendor_name",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent")
                        && event
                            .has_value("jamf_protect.alerts.input.match.event.device.isWritable")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.device.isWritable",
                        "volume.writable",
                    )?;
                }
                // End nested pipeline: "gpusbevent"
            }

            let _cond = {
                event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUnifiedLogEvent")
            };
            if _cond {
                // Begin nested pipeline: "gpunifiedlogevent"
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType")
                        == Some("GPUnifiedLogEvent")
                        && event.has_value("jamf_protect.alerts.input.match.event.process")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.process",
                        "process.name",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType")
                        == Some("GPUnifiedLogEvent")
                        && event
                            .has_value("jamf_protect.alerts.input.match.event.processIdentifier")
                };
                if _cond {
                    event.rename(
                        "jamf_protect.alerts.input.match.event.processIdentifier",
                        "process.pid",
                    )?;
                }
                let _cond = {
                    event.get_str("jamf_protect.alerts.input.eventType")
                        == Some("GPUnifiedLogEvent")
                        && event.has_value("jamf_protect.alerts.input.match.event.timestamp")
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("jamf_protect.alerts.input.match.event.timestamp")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("process.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "jamf_protect.alerts.input.match.event.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                // End nested pipeline: "gpunifiedlogevent"
            }

            event.remove("jamf_protect.alerts");
            event.remove("jamf_protect");
            event.remove("message");
            event.remove("json");

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\nif (object == null || object == '') {\n    return true;\n} else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n} else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n}\nreturn false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\nif (object == null || object == '') {\n    return true;\n} else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n} else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n}\nreturn false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
