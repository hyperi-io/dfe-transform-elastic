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
            event.remove("host");

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

            let _cond = { !event.has_value("json") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            let _cond = {
                event.has_value("json.data")
                    && event.get("json.data").is_some_and(|v| match v {
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.data.connector_guid") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.data.detection_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.data.event_type_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.data.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.data.timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.data.timestamp_nanoseconds") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.data") {
                event.rename("json.data", "cisco.secure_endpoint")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("cisco.secure_endpoint.timestamp") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cisco.secure_endpoint.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("cisco.secure_endpoint.error.error_code") {
                event.rename("cisco.secure_endpoint.error.error_code", "error.code")?;
            }

            if event.has_value("error.code") {
                if let Some(val) = event.get("error.code") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "error.code".into(),
                            message,
                        }
                    })?;
                    event.set("error.code", converted)?;
                }
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("alert"))?;

            if event.has_value("cisco.secure_endpoint.id") {
                if let Some(val) = event.get("cisco.secure_endpoint.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cisco.secure_endpoint.id".into(),
                            message,
                        }
                    })?;
                    event.set("event.id", converted)?;
                }
            }

            let _cond = { event.has_value("cisco.secure_endpoint.file.file_name") };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond =
                { event.get_str("cisco.secure_endpoint.file.disposition") == Some("Malicious") };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            if event.has_value("cisco.secure_endpoint.event_type") {
                event.rename("cisco.secure_endpoint.event_type", "event.action")?;
            }

            if event.has_value("cisco.secure_endpoint.event_type_id") {
                if let Some(val) = event.get("cisco.secure_endpoint.event_type_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cisco.secure_endpoint.event_type_id".into(),
                            message,
                        }
                    })?;
                    event.set("event.code", converted)?;
                }
            }

            let _cond = { event.get_str("cisco.secure_endpoint.severity") == Some("Low") };
            if _cond {
                event.set("event.severity", json!(1))?;
            }

            let _cond = { event.get_str("cisco.secure_endpoint.severity") == Some("Medium") };
            if _cond {
                event.set("event.severity", json!(2))?;
            }

            let _cond = { event.get_str("cisco.secure_endpoint.severity") == Some("High") };
            if _cond {
                event.set("event.severity", json!(3))?;
            }

            let _cond = { event.get_str("cisco.secure_endpoint.severity") == Some("Critical") };
            if _cond {
                event.set("event.severity", json!(4))?;
            }

            let _cond = { !event.has_value("cisco.secure_endpoint.severity") };
            if _cond {
                event.set("event.severity", json!(0))?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.start_timestamp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cisco.secure_endpoint.start_timestamp")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("event.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cisco.secure_endpoint.start_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("cisco.secure_endpoint.techniques") && event.get("cisco.secure_endpoint.techniques").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.get("cisco.secure_endpoint.techniques.0").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename("cisco.secure_endpoint.techniques", "threat.technique.id")?;
            }

            let _cond = {
                event.has_value("cisco.secure_endpoint.tactics") && event.get("cisco.secure_endpoint.tactics").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.get("cisco.secure_endpoint.tactics.0").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename("cisco.secure_endpoint.tactics", "threat.tactic.id")?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.tactics") };
            if _cond {
                // Painless script
                // Source: if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n}\nif (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n}\nif (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n}\nif (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n}\nif (ctx.threat.technique.reference == null) {\n    ctx.threat.technique.reference = new ArrayList();\n}\nfor (technique in ctx.cisco?.secure_endpoint?.techniques) {\n    if (technique.name != null) {\n        if (!ctx.threat.technique.name.contains(technique.name)) {\n            ctx.threat.technique.name.add(technique.name);  \n        }\n    }\n    if (technique.external_id != null) {\n        if (!ctx.threat.technique.id.contains(technique.external_id)) {\n            ctx.threat.technique.id.add(technique.external_id);  \n        }\n    }\n    if (technique.mitre_url != null) {\n        if (!ctx.threat.technique.reference.contains(technique.mitre_url)) {\n            ctx.threat.technique.reference.add(technique.mitre_url);  \n        }\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n}\nif (ctx.threat.technique == null) {\n    ctx.threat.technique = new HashMap();\n}\nif (ctx.threat.technique.id == null) {\n    ctx.threat.technique.id = new ArrayList();\n}\nif (ctx.threat.technique.name == null) {\n    ctx.threat.technique.name = new ArrayList();\n}\nif (ctx.threat.technique.reference == null) {\n    ctx.threat.technique.reference = new ArrayList();\n}\nfor (technique in ctx.cisco?.secure_endpoint?.techniques) {\n    if (technique.name != null) {\n        if (!ctx.threat.technique.name.contains(technique.name)) {\n            ctx.threat.technique.name.add(technique.name);  \n        }\n    }\n    if (technique.external_id != null) {\n        if (!ctx.threat.technique.id.contains(technique.external_id)) {\n            ctx.threat.technique.id.add(technique.external_id);  \n        }\n    }\n    if (technique.mitre_url != null) {\n        if (!ctx.threat.technique.reference.contains(technique.mitre_url)) {\n            ctx.threat.technique.reference.add(technique.mitre_url);  \n        }\n    }\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.tactics") };
            if _cond {
                // Painless script
                // Source: if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n}\nif (ctx.threat.tactic == null) {\n    ctx.threat.tactic = new HashMap();\n}\nif (ctx.threat.tactic.id == null) {\n    ctx.threat.tactic.id = new ArrayList();\n}\nif (ctx.threat.tactic.name == null) {\n    ctx.threat.tactic.name = new ArrayList();\n}\nif (ctx.threat.tactic.reference == null) {\n    ctx.threat.tactic.reference = new ArrayList();\n}\nfor (tactic in ctx.cisco?.secure_endpoint?.tactics) {\n    if (tactic.name != null) {\n        if (!ctx.threat.tactic.name.contains(tactic.name)) {\n            ctx.threat.tactic.name.add(tactic.name);  \n        }\n    }\n    if (tactic.external_id != null) {\n        if (!ctx.threat.tactic.id.contains(tactic.external_id)) {\n            ctx.threat.tactic.id.add(tactic.external_id);  \n        }\n    }\n    if (tactic.mitre_url != null) {\n        if (!ctx.threat.tactic.reference.contains(tactic.mitre_url)) {\n            ctx.threat.tactic.reference.add(tactic.mitre_url);  \n        }\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.threat == null) {\n    ctx.threat = new HashMap();\n}\nif (ctx.threat.tactic == null) {\n    ctx.threat.tactic = new HashMap();\n}\nif (ctx.threat.tactic.id == null) {\n    ctx.threat.tactic.id = new ArrayList();\n}\nif (ctx.threat.tactic.name == null) {\n    ctx.threat.tactic.name = new ArrayList();\n}\nif (ctx.threat.tactic.reference == null) {\n    ctx.threat.tactic.reference = new ArrayList();\n}\nfor (tactic in ctx.cisco?.secure_endpoint?.tactics) {\n    if (tactic.name != null) {\n        if (!ctx.threat.tactic.name.contains(tactic.name)) {\n            ctx.threat.tactic.name.add(tactic.name);  \n        }\n    }\n    if (tactic.external_id != null) {\n        if (!ctx.threat.tactic.id.contains(tactic.external_id)) {\n            ctx.threat.tactic.id.add(tactic.external_id);  \n        }\n    }\n    if (tactic.mitre_url != null) {\n        if (!ctx.threat.tactic.reference.contains(tactic.mitre_url)) {\n            ctx.threat.tactic.reference.add(tactic.mitre_url);  \n        }\n    }\n}\n"#
                    ),
                )?;
            }

            if event.has_value("cisco.secure_endpoint.group_guids") {
                event.rename("cisco.secure_endpoint.group_guids", "group.id")?;
            }

            if event.has_value("cisco.secure_endpoint.computer.hostname") {
                event.rename("cisco.secure_endpoint.computer.hostname", "host.name")?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if event.has_value("cisco.secure_endpoint.hostname") {
                    event.rename("cisco.secure_endpoint.hostname", "host.name")?;
                }
            }

            let _cond = {
                event.has_value("host.name")
                    && event.get("host.name").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: def domain = \"\";\ndef nameArray = ctx.host.name.toString().splitOnToken(\".\");\nif (ctx.host == null) {\n    ctx.host = new HashMap();\n}\nif (nameArray.length > 0) {\n    for (int i = 1; i < nameArray.length; i++) {\n    domain += nameArray[i] + (i < nameArray.length - 1 ? \".\" : \"\");\n    }\n    ctx.host.hostname = nameArray[0];\n    ctx.host.domain = domain; \n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def domain = \"\";\ndef nameArray = ctx.host.name.toString().splitOnToken(\".\");\nif (ctx.host == null) {\n    ctx.host = new HashMap();\n}\nif (nameArray.length > 0) {\n    for (int i = 1; i < nameArray.length; i++) {\n    domain += nameArray[i] + (i < nameArray.length - 1 ? \".\" : \"\");\n    }\n    ctx.host.hostname = nameArray[0];\n    ctx.host.domain = domain; \n}\n"#
                    ),
                )?;
            }

            let _cond = { !event.has_value("host.hostname") };
            if _cond {
                if let Some(v) = event
                    .get("host.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                map_strings(event, "host.name", "host.name", str::to_lowercase)?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.computer.network_addresses") };
            if _cond {
                // Painless script
                // Source: if (ctx.host == null) {\n    ctx.host = new HashMap();\n}\nif (ctx.host.ip == null) {\n    ctx.host.ip = new ArrayList();\n}\nif (ctx.host.mac == null) {\n    ctx.host.mac = new ArrayList();\n}\nfor (addr in ctx.cisco.secure_endpoint.computer.network_addresses) {\n    if (addr.ip != null && !addr.ip.isEmpty()) {\n        if (!ctx.host.ip.contains(addr.ip)) {\n            ctx.host.ip.add(addr.ip);\n        }\n    }\n    if (addr.mac != null && !addr.mac.isEmpty()) {\n        def mac_addr = addr.mac.replace(\":\",\"-\").toUpperCase();\n        if (!ctx.host.mac.contains(mac_addr)) {\n            ctx.host.mac.add(mac_addr);\n        }\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.host == null) {\n    ctx.host = new HashMap();\n}\nif (ctx.host.ip == null) {\n    ctx.host.ip = new ArrayList();\n}\nif (ctx.host.mac == null) {\n    ctx.host.mac = new ArrayList();\n}\nfor (addr in ctx.cisco.secure_endpoint.computer.network_addresses) {\n    if (addr.ip != null && !addr.ip.isEmpty()) {\n        if (!ctx.host.ip.contains(addr.ip)) {\n            ctx.host.ip.add(addr.ip);\n        }\n    }\n    if (addr.mac != null && !addr.mac.isEmpty()) {\n        def mac_addr = addr.mac.replace(\":\",\"-\").toUpperCase();\n        if (!ctx.host.mac.contains(mac_addr)) {\n            ctx.host.mac.add(mac_addr);\n        }\n    }\n}\n"#
                    ),
                )?;
            }

            if event.has_value("cisco.secure_endpoint.computer.connector_guid") {
                event.rename("cisco.secure_endpoint.computer.connector_guid", "host.id")?;
            }

            let _cond = {
                event.has_value("cisco.secure_endpoint.computer.user")
                    && event.get_str("cisco.secure_endpoint.computer.user") == Some("Not Available")
            };
            if _cond {
                if event
                    .remove("cisco.secure_endpoint.computer.user")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "cisco.secure_endpoint.computer.user".into(),
                    });
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco.secure_endpoint.computer.user") {
                    if let Some(input) = event.get_string("cisco.secure_endpoint.computer.user") {
                        // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        if !extract_first_match(
                            &[
                                cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("cisco.secure_endpoint.computer.user")
                    && event
                        .get_str("cisco.secure_endpoint.computer.user")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("cisco.secure_endpoint.computer.user").cloned() {
                    event.set("user.email", v)?;
                }
            }

            if event.has_value("cisco.secure_endpoint.network_info.nfm.protocol") {
                event.rename(
                    "cisco.secure_endpoint.network_info.nfm.protocol",
                    "network.transport",
                )?;
            }

            let _cond = {
                event.get_str("cisco.secure_endpoint.network_info.nfm.direction")
                    == Some("Outgoing connection from")
            };
            if _cond {
                event.set("network.direction", json!("egress"))?;
            }

            let _cond = {
                event.has_value("cisco.secure_endpoint.network_info.nfm.direction")
                    && event.get_str("cisco.secure_endpoint.network_info.nfm.direction")
                        != Some("Outgoing connection from")
            };
            if _cond {
                event.set("network.direction", json!("ingress"))?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.network_info.dirty_url") };
            if _cond {
                uri_parts(
                    event,
                    "cisco.secure_endpoint.network_info.dirty_url",
                    "url",
                    true,
                    true,
                )?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.dirty_url") {
                event.rename(
                    "cisco.secure_endpoint.network_info.dirty_url",
                    "url.original",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.local_ip") {
                event.rename("cisco.secure_endpoint.network_info.local_ip", "source.ip")?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.local_port") {
                event.rename(
                    "cisco.secure_endpoint.network_info.local_port",
                    "source.port",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.remote_ip") {
                event.rename(
                    "cisco.secure_endpoint.network_info.remote_ip",
                    "destination.ip",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.remote_port") {
                event.rename(
                    "cisco.secure_endpoint.network_info.remote_port",
                    "destination.port",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.file.file_name") {
                event.rename("cisco.secure_endpoint.file.file_name", "file.name")?;
            }

            if event.has_value("cisco.secure_endpoint.file.file_path") {
                event.rename("cisco.secure_endpoint.file.file_path", "file.path")?;
            }

            if event.has_value("cisco.secure_endpoint.file.identity.sha256") {
                event.rename(
                    "cisco.secure_endpoint.file.identity.sha256",
                    "file.hash.sha256",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.file.identity.sha1") {
                event.rename("cisco.secure_endpoint.file.identity.sha1", "file.hash.sha1")?;
            }

            if event.has_value("cisco.secure_endpoint.file.identity.md5") {
                event.rename("cisco.secure_endpoint.file.identity.md5", "file.hash.md5")?;
            }

            let _cond = {
                event.has_value("file.path")
                    && event.get("file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\\\")),
                        serde_json::Value::String(s) => s.contains("\\\\"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("host.os.family", json!("windows"))?;
            }

            let _cond = {
                event.has_value("file.path")
                    && event.get("file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\\\")),
                        serde_json::Value::String(s) => s.contains("\\\\"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("host.os.platform", json!("windows"))?;
            }

            if event.has_value("cisco.secure_endpoint.file.parent.process_id") {
                event.rename(
                    "cisco.secure_endpoint.file.parent.process_id",
                    "process.pid",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.parent.process_id") {
                event.rename(
                    "cisco.secure_endpoint.network_info.parent.process_id",
                    "process.pid",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.file.parent.file_name") {
                event.rename(
                    "cisco.secure_endpoint.file.parent.file_name",
                    "process.name",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.file.parent.identity.sha256") {
                event.rename(
                    "cisco.secure_endpoint.file.parent.identity.sha256",
                    "process.hash.sha256",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.file.parent.identity.sha1") {
                event.rename(
                    "cisco.secure_endpoint.file.parent.identity.sha1",
                    "process.hash.sha1",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.file.parent.identity.md5") {
                event.rename(
                    "cisco.secure_endpoint.file.parent.identity.md5",
                    "process.hash.md5",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.file.parent.identity.md5") {
                event.rename(
                    "cisco.secure_endpoint.file.parent.identity.md5",
                    "process.hash.md5",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.parent.file_name") {
                event.rename(
                    "cisco.secure_endpoint.network_info.parent.file_name",
                    "process.name",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.parent.identity.sha256") {
                event.rename(
                    "cisco.secure_endpoint.network_info.parent.identity.sha256",
                    "process.hash.sha256",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.parent.identity.sha1") {
                event.rename(
                    "cisco.secure_endpoint.network_info.parent.identity.sha1",
                    "process.hash.sha1",
                )?;
            }

            if event.has_value("cisco.secure_endpoint.network_info.parent.identity.md5") {
                event.rename(
                    "cisco.secure_endpoint.network_info.parent.identity.md5",
                    "process.hash.md5",
                )?;
            }

            // Painless script
            // Source: def commandLine = ctx.cisco?.secure_endpoint?.command_line?.arguments;\nif (commandLine != null) {\n  commandLine = commandLine.trim();\n  if (commandLine != \"\") {\n    ctx.process.command_line = commandLine;\n\n    def args = [];\n    for (def v : / /.split(commandLine)) {\n      if (v != \"\") {\n        args.add(v);\n      }\n    }\n    if (args.size() > 0) {\n      ctx.process.args = args;\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def commandLine = ctx.cisco?.secure_endpoint?.command_line?.arguments;\nif (commandLine != null) {\n  commandLine = commandLine.trim();\n  if (commandLine != \"\") {\n    ctx.process.command_line = commandLine;\n\n    def args = [];\n    for (def v : / /.split(commandLine)) {\n      if (v != \"\") {\n        args.add(v);\n      }\n    }\n    if (args.size() > 0) {\n      ctx.process.args = args;\n    }\n  }\n}\n"#
                ),
            )?;

            let _cond = { event.has_value("process.args") };
            if _cond {
                // Painless script
                // Source: ctx.process.args_count = ctx.process.args.length;\nif (ctx.process.args.length > 0) {\n  ctx.process.executable = ctx.process.args[0];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.process.args_count = ctx.process.args.length;\nif (ctx.process.args.length > 0) {\n  ctx.process.executable = ctx.process.args[0];\n}\n"#
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

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
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
                            .get("process.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.parent.hash.md5") };
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

            let _cond = { event.has_value("process.parent.hash.sha1") };
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

            let _cond =
                { event.has_value("cisco.secure_endpoint.network_info.parent.identity.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cisco.secure_endpoint.network_info.parent.identity.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cisco.secure_endpoint.network_info.parent.identity.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cisco.secure_endpoint.network_info.parent.identity.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cisco.secure_endpoint.network_info.parent.identity.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cisco.secure_endpoint.network_info.parent.identity.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.computer.external_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cisco.secure_endpoint.computer.external_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.computer.network_addresses") };
            if _cond {
                // Painless script
                // Source: if (ctx.related == null) {\n    ctx.related = new HashMap();\n}\nif (ctx.related?.ip == null) {\n    ctx.related.ip = new ArrayList();\n}\nfor (addr in ctx.cisco?.secure_endpoint?.computer?.network_addresses) {\n    if (addr.ip != null && !addr.ip.isEmpty()) {\n        if (!ctx.related.ip.contains(addr.ip)) {\n            ctx.related.ip.add(addr.ip);  \n        }\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.related == null) {\n    ctx.related = new HashMap();\n}\nif (ctx.related?.ip == null) {\n    ctx.related.ip = new ArrayList();\n}\nfor (addr in ctx.cisco?.secure_endpoint?.computer?.network_addresses) {\n    if (addr.ip != null && !addr.ip.isEmpty()) {\n        if (!ctx.related.ip.contains(addr.ip)) {\n            ctx.related.ip.add(addr.ip);  \n        }\n    }\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.computer.network_addresses") };
            if _cond {
                // Painless script
                // Source: if (ctx.cisco?.secure_endpoint?.related == null) {\n    ctx.cisco.secure_endpoint.related = new HashMap();\n}\nif (ctx.cisco?.secure_endpoint?.related?.mac == null) {\n    ctx.cisco.secure_endpoint.related.mac = new ArrayList();\n}\nfor (addr in ctx.cisco?.secure_endpoint?.computer?.network_addresses) {\n    if (addr.mac != null && !addr.mac.isEmpty()) {\n        if (!ctx.cisco.secure_endpoint.related.mac.contains(addr.mac)) {\n            def mac_addr = addr.mac.replace(\":\",\"-\").toUpperCase();\n            ctx.cisco.secure_endpoint.related.mac.add(mac_addr);\n        }\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.cisco?.secure_endpoint?.related == null) {\n    ctx.cisco.secure_endpoint.related = new HashMap();\n}\nif (ctx.cisco?.secure_endpoint?.related?.mac == null) {\n    ctx.cisco.secure_endpoint.related.mac = new ArrayList();\n}\nfor (addr in ctx.cisco?.secure_endpoint?.computer?.network_addresses) {\n    if (addr.mac != null && !addr.mac.isEmpty()) {\n        if (!ctx.cisco.secure_endpoint.related.mac.contains(addr.mac)) {\n            def mac_addr = addr.mac.replace(\":\",\"-\").toUpperCase();\n            ctx.cisco.secure_endpoint.related.mac.add(mac_addr);\n        }\n    }\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.vulnerabilities") };
            if _cond {
                foreach_array(event, "cisco.secure_endpoint.vulnerabilities", |event| {
                    event.append_unique(
                        "cisco.secure_endpoint.related.cve",
                        json!(
                            event
                                .get("_ingest._value.cve")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { !event.has_value("source.geo") };
            if _cond {
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
            }

            let _cond = { !event.has_value("destination.geo") };
            if _cond {
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

            let _cond =
                { event.has_value("cisco.secure_endpoint.threat_hunting.incident_start_time") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("cisco.secure_endpoint.threat_hunting.incident_start_time")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set(
                                "cisco.secure_endpoint.threat_hunting.incident_start_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "cisco.secure_endpoint.threat_hunting.incident_start_time"
                                            .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("cisco.secure_endpoint.threat_hunting.incident_end_time") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("cisco.secure_endpoint.threat_hunting.incident_end_time")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set(
                                "cisco.secure_endpoint.threat_hunting.incident_end_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cisco.secure_endpoint.threat_hunting.incident_end_time"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: boolean dropEmptyFields(Object object) {\n    if (object == null || object == '') {\n    return true;\n    } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n    } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n    }\n    return false;\n}\ndropEmptyFields(ctx);\n
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
            }

            event.remove("cisco.secure_endpoint.computer.hostname");
            event.remove("cisco.secure_endpoint.computer.links");
            event.remove("cisco.secure_endpoint.computer.user");
            event.remove("cisco.secure_endpoint.date");
            event.remove("cisco.secure_endpoint.id");
            event.remove("cisco.secure_endpoint.severity");
            event.remove("cisco.secure_endpoint.start_date");
            event.remove("cisco.secure_endpoint.start_timestamp");
            event.remove("cisco.secure_endpoint.threat_hunting.tactics");
            event.remove("cisco.secure_endpoint.threat_hunting.techniques");
            event.remove("cisco.secure_endpoint.timestamp");
            event.remove("cisco.secure_endpoint.timestamp_nanoseconds");
            event.remove("json");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.remove("json");
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor \"{}\" with tag \"{}\" failed with message \"{}\"",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
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
