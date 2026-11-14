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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "e9b3df15153d2d58c1766e0b05a93f90116611dd6b4e56d6206947309a672585",
                        r#"{"params":null,"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "11397b52efe4568ea14166aa100674912864976fc9a082265cb7f08f2cd9fc62",
                        r#"{"params":null,"known":[{"PlainAssignments":{"statements":[{"If":{"test":[[{"Compare":{"path":"rule","wanted":"Null","negated":false,"lowered":false}}]],"then":[],"alt":[]}},{"Literal":{"Set":{"path":"rule.name","value":{"Field":"jamf_protect.alerts.input.match.facts.0.name"}}}}],"whole":true}}]}"#
                    ),
                )?;
            }

            let _cond = {
                event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| v.is_array()) && event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } != 0) && event.has_value("jamf_protect.alerts.input.match.facts.0.human")
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "4af99fccfa1392cc2f3801731550ce9b147a937d06df5d8d8beb1d2f3e8d4b30",
                        r#"{"params":null,"known":[{"PlainAssignments":{"statements":[{"If":{"test":[[{"Compare":{"path":"rule","wanted":"Null","negated":false,"lowered":false}}]],"then":[],"alt":[]}},{"Literal":{"Set":{"path":"rule.description","value":{"Field":"jamf_protect.alerts.input.match.facts.0.human"}}}}],"whole":true}}]}"#
                    ),
                )?;
            }

            let _cond = {
                event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| v.is_array()) && event.get("jamf_protect.alerts.input.match.facts").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.has("jamf_protect.alerts.input.match.facts.0.name")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "dd70e5d7a8d88b5174f93a7a51583a0ebe877a78053b887afad40c4f9ce2fd81",
                            r#"{"params":null,"known":[{"PlainAssignments":{"statements":[{"Literal":{"Set":{"path":"event.action","value":{"Field":"jamf_protect.alerts.input.match.facts.0.name"}}}}],"whole":true}}]}"#
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
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "437053e7da03bc3733c8389576551684cdb1df7f95c79e6e3ca7629de48acf1d",
                            r#"{"params":null,"known":[{"PlainAssignments":{"statements":[{"Literal":{"Set":{"path":"event.reason","value":{"Field":"jamf_protect.alerts.input.match.facts.0.human"}}}}],"whole":true}}]}"#
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
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "3a99359fd3e3a2ab38197846b106dc735208ae79d234a8267b041c102ec499f9",
                            r#"{"params":null,"known":[]}"#
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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "76f3efc1318de5a450510a3f754acbf560c53d91b5fb84b32440d98e9bd18607",
                    r#"{"params":null,"known":[{"GatherMembers":{"list":"jamf_protect.alerts.input.related.users","stop_on_null_element":false,"columns":[{"member":"name","targets":["related.user"],"allowed":[],"lowercase":false,"flatten":false,"dedupe":false,"selector":null,"literals":[],"split":null}],"flag":null}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "111774e6ebd0a7020d712088ce0e0c15b4b57c8ce6d27fcd64f587ffeddea893",
                        r#"{"params":null,"known":[{"ElementMapping":{"locals":[{"name":"file","list":"jamf_protect.alerts.input.related.files","index":{"At":0}}],"body":[{"If":{"test":{"Longer":{"list":"jamf_protect.alerts.input.related.files","least":0}},"body":[{"EnsureMap":"file"},{"Write":{"path":"file.path","read":{"Member":{"local":0,"members":["path"]}},"render":"Direct"}},{"Write":{"path":"file.size","read":{"Member":{"local":0,"members":["size"]}},"render":"Direct"}},{"Write":{"path":"file.inode","read":{"Member":{"local":0,"members":["inode"]}},"render":"StringValueOf"}},{"Write":{"path":"file.gid","read":{"Member":{"local":0,"members":["gid"]}},"render":"StringValueOf"}},{"Write":{"path":"file.mode","read":{"Member":{"local":0,"members":["mode"]}},"render":"StringValueOf"}},{"Write":{"path":"file.uid","read":{"Member":{"local":0,"members":["uid"]}},"render":"StringValueOf"}},{"EnsureMap":"file.hash"},{"Write":{"path":"file.hash.sha1","read":{"Member":{"local":0,"members":["sha1hex"]}},"render":"Direct"}},{"Write":{"path":"file.hash.sha256","read":{"Member":{"local":0,"members":["sha256hex"]}},"render":"Direct"}},{"EnsureMap":"file.code_signature"},{"Write":{"path":"file.code_signature.signing_id","read":{"Member":{"local":0,"members":["signingInfo","appid"]}},"render":"Direct"}},{"Write":{"path":"file.code_signature.status","read":{"Member":{"local":0,"members":["signingInfo","statusMessage"]}},"render":"Direct"}},{"Write":{"path":"file.code_signature.team_id","read":{"Member":{"local":0,"members":["signingInfo","teamid"]}},"render":"Direct"}}]}}]}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "421244aa20a3a3b104ee99f40b7d71ffaab1d285892a7bb3402873541e00385e",
                        r#"{"params":null,"known":[{"ElementMapping":{"locals":[{"name":"process","list":"jamf_protect.alerts.input.related.processes","index":{"At":0}},{"name":"binary","list":"jamf_protect.alerts.input.related.binaries","index":{"At":0}},{"name":"parentProcess","list":"jamf_protect.alerts.input.related.processes","index":{"At":1}},{"name":"processGroupLeader","list":"jamf_protect.alerts.input.related.processes","index":"Last"}],"body":[{"If":{"test":{"All":[{"NotNull":{"Field":"jamf_protect.alerts.input.related.processes"}},{"Longer":{"list":"jamf_protect.alerts.input.related.processes","least":0}}]},"body":[{"EnsureMap":"process"},{"Write":{"path":"process.name","read":{"Member":{"local":0,"members":["name"]}},"render":"Direct"}},{"Write":{"path":"process.executable","read":{"Member":{"local":0,"members":["path"]}},"render":"Direct"}},{"Write":{"path":"process.pid","read":{"Member":{"local":0,"members":["pid"]}},"render":"Direct"}},{"If":{"test":{"ContainsKey":{"local":0,"member":"startTimestamp"}},"body":[{"Write":{"path":"process.start","read":{"Member":{"local":0,"members":["startTimestamp"]}},"render":"EpochSecond"}}]}},{"If":{"test":{"NotNull":{"Member":{"local":0,"members":["exitCode"]}}},"body":[{"Write":{"path":"process.exit_code","read":{"Member":{"local":0,"members":["exitCode"]}},"render":"Direct"}}]}},{"Write":{"path":"process.args","read":{"Member":{"local":0,"members":["args"]}},"render":"OrEmptyList"}},{"Write":{"path":"process.entity_id","read":{"Member":{"local":0,"members":["uuid"]}},"render":"Direct"}},{"ResetMap":"process.parent"},{"Write":{"path":"process.parent.pid","read":{"Member":{"local":0,"members":["responsiblePID"]}},"render":"Direct"}},{"ResetMap":"process.user"},{"Write":{"path":"process.user.id","read":{"Member":{"local":0,"members":["uid"]}},"render":"ToString"}},{"ResetMap":"process.group_leader"},{"If":{"test":{"NotNull":{"Member":{"local":0,"members":["pgid"]}}},"body":[{"Write":{"path":"process.group_leader.pid","read":{"Member":{"local":0,"members":["pgid"]}},"render":"Direct"}}]}},{"ResetMap":"process.group_leader.group"},{"If":{"test":{"NotNull":{"Member":{"local":0,"members":["gid"]}}},"body":[{"Write":{"path":"process.group_leader.group.id","read":{"Member":{"local":0,"members":["gid"]}},"render":"ToString"}}]}},{"ResetMap":"process.real_user"},{"If":{"test":{"NotNull":{"Member":{"local":0,"members":["ruid"]}}},"body":[{"Write":{"path":"process.real_user.id","read":{"Member":{"local":0,"members":["ruid"]}},"render":"ToString"}}]}},{"ResetMap":"process.real_group"},{"If":{"test":{"NotNull":{"Member":{"local":0,"members":["rgid"]}}},"body":[{"Write":{"path":"process.real_group.id","read":{"Member":{"local":0,"members":["rgid"]}},"render":"ToString"}}]}},{"EnsureMap":"process.hash"},{"If":{"test":{"NotNull":{"Member":{"local":1,"members":["sha1hex"]}}},"body":[{"Write":{"path":"process.hash.sha1","read":{"Member":{"local":1,"members":["sha1hex"]}},"render":"Direct"}}]}},{"If":{"test":{"NotNull":{"Member":{"local":1,"members":["sha256hex"]}}},"body":[{"Write":{"path":"process.hash.sha256","read":{"Member":{"local":1,"members":["sha256hex"]}},"render":"Direct"}}]}},{"EnsureMap":"process.code_signature"},{"If":{"test":{"NotNull":{"Member":{"local":0,"members":["signingInfo","appid"]}}},"body":[{"Write":{"path":"process.code_signature.signing_id","read":{"Member":{"local":0,"members":["signingInfo","appid"]}},"render":"Direct"}}]}},{"If":{"test":{"NotNull":{"Member":{"local":0,"members":["signingInfo","statusMessage"]}}},"body":[{"Write":{"path":"process.code_signature.status","read":{"Member":{"local":0,"members":["signingInfo","statusMessage"]}},"render":"Direct"}}]}},{"If":{"test":{"NotNull":{"Member":{"local":0,"members":["signingInfo","teamid"]}}},"body":[{"Write":{"path":"process.code_signature.team_id","read":{"Member":{"local":0,"members":["signingInfo","teamid"]}},"render":"Direct"}}]}},{"If":{"test":{"Longer":{"list":"jamf_protect.alerts.input.related.processes","least":1}},"body":[{"ResetMap":"process.parent"},{"Write":{"path":"process.parent.name","read":{"Member":{"local":2,"members":["name"]}},"render":"Direct"}},{"Write":{"path":"process.parent.pid","read":{"Member":{"local":2,"members":["pid"]}},"render":"Direct"}},{"Write":{"path":"process.parent.executable","read":{"Member":{"local":2,"members":["path"]}},"render":"Direct"}},{"Write":{"path":"process.parent.entity_id","read":{"Member":{"local":2,"members":["uuid"]}},"render":"Direct"}},{"If":{"test":{"ContainsKey":{"local":2,"member":"startTimestamp"}},"body":[{"Write":{"path":"process.parent.start","read":{"Member":{"local":2,"members":["startTimestamp"]}},"render":"EpochSecond"}}]}},{"ResetMap":"process.parent.user"},{"If":{"test":{"NotNull":{"Member":{"local":2,"members":["uid"]}}},"body":[{"Write":{"path":"process.parent.user.id","read":{"Member":{"local":2,"members":["uid"]}},"render":"ToString"}}]}},{"ResetMap":"process.parent.real_user"},{"If":{"test":{"NotNull":{"Member":{"local":2,"members":["ruid"]}}},"body":[{"Write":{"path":"process.parent.real_user.id","read":{"Member":{"local":2,"members":["ruid"]}},"render":"ToString"}}]}},{"ResetMap":"process.parent.real_group"},{"If":{"test":{"NotNull":{"Member":{"local":2,"members":["rgid"]}}},"body":[{"Write":{"path":"process.parent.real_group.id","read":{"Member":{"local":2,"members":["rgid"]}},"render":"ToString"}}]}},{"EnsureMap":"process.parent.code_signature"},{"If":{"test":{"NotNull":{"Member":{"local":2,"members":["signingInfo","appid"]}}},"body":[{"Write":{"path":"process.parent.code_signature.signing_id","read":{"Member":{"local":2,"members":["signingInfo","appid"]}},"render":"Direct"}}]}},{"If":{"test":{"NotNull":{"Member":{"local":2,"members":["signingInfo","statusMessage"]}}},"body":[{"Write":{"path":"process.parent.code_signature.status","read":{"Member":{"local":2,"members":["signingInfo","statusMessage"]}},"render":"Direct"}}]}},{"If":{"test":{"NotNull":{"Member":{"local":2,"members":["signingInfo","teamid"]}}},"body":[{"Write":{"path":"process.parent.code_signature.team_id","read":{"Member":{"local":2,"members":["signingInfo","teamid"]}},"render":"Direct"}}]}}]}},{"ResetMap":"process.group_leader"},{"Write":{"path":"process.group_leader.name","read":{"Member":{"local":3,"members":["name"]}},"render":"Direct"}},{"Write":{"path":"process.group_leader.pid","read":{"Member":{"local":3,"members":["pid"]}},"render":"Direct"}},{"Write":{"path":"process.group_leader.executable","read":{"Member":{"local":3,"members":["path"]}},"render":"Direct"}},{"If":{"test":{"ContainsKey":{"local":3,"member":"startTimestamp"}},"body":[{"Write":{"path":"process.group_leader.start","read":{"Member":{"local":3,"members":["startTimestamp"]}},"render":"EpochSecond"}}]}},{"ResetMap":"process.group_leader.user"},{"If":{"test":{"NotNull":{"Member":{"local":3,"members":["uid"]}}},"body":[{"Write":{"path":"process.group_leader.user.id","read":{"Member":{"local":3,"members":["uid"]}},"render":"ToString"}}]}},{"ResetMap":"process.group_leader.real_user"},{"If":{"test":{"NotNull":{"Member":{"local":3,"members":["ruid"]}}},"body":[{"Write":{"path":"process.group_leader.real_user.id","read":{"Member":{"local":3,"members":["ruid"]}},"render":"ToString"}}]}},{"ResetMap":"process.group_leader.real_group"},{"If":{"test":{"NotNull":{"Member":{"local":3,"members":["rgid"]}}},"body":[{"Write":{"path":"process.group_leader.real_group.id","read":{"Member":{"local":3,"members":["rgid"]}},"render":"ToString"}}]}}]}}]}}]}"#
                    ),
                )?;
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "b9056046b38a42feae5f99c22d1eed8abbe876e5bbce08d4c5d638eeb6bc5c73",
                    r#"{"params":null,"known":[{"ElementMapping":{"locals":[{"name":"group","list":"jamf_protect.alerts.input.related.groups","index":{"At":0}}],"body":[{"If":{"test":{"All":[{"NotNull":{"Field":"jamf_protect.alerts.input.related.groups"}},{"Longer":{"list":"jamf_protect.alerts.input.related.groups","least":0}}]},"body":[{"EnsureMap":"group"},{"Write":{"path":"group.name","read":{"Member":{"local":0,"members":["name"]}},"render":"Direct"}},{"Write":{"path":"group.id","read":{"Member":{"local":0,"members":["gid"]}},"render":"ToString"}}]}}]}}]}"#
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

            // The pipeline's script, emitted as the runtime's equivalent
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
