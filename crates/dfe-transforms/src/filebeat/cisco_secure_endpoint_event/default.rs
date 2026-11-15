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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "4633917cf24135a5046ad66061e9c9c9e751ba64e925472f3592ceba1405feda",
                        r#"{"params":null,"known":[{"CollectFromList":{"source":"cisco.secure_endpoint.techniques","columns":[{"member":"name","target":"threat.technique.name","replace":null,"upper":false},{"member":"external_id","target":"threat.technique.id","replace":null,"upper":false},{"member":"mitre_url","target":"threat.technique.reference","replace":null,"upper":false}],"firsts":[]}}]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.tactics") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "7cafae657194e52e35a424b3d7737eb5baedd0c0dd906ff188bec4f5c69db804",
                        r#"{"params":null,"known":[{"CollectFromList":{"source":"cisco.secure_endpoint.tactics","columns":[{"member":"name","target":"threat.tactic.name","replace":null,"upper":false},{"member":"external_id","target":"threat.tactic.id","replace":null,"upper":false},{"member":"mitre_url","target":"threat.tactic.reference","replace":null,"upper":false}],"firsts":[]}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "8dc48eddcf27ca72c56b0ef51c4e27913c6a4a93cd3c8642319ae404f1d1aaf7",
                        r#"{"params":null,"known":[{"SplitFirstLabel":{"source":"host.name","separator":".","head":"host.hostname","tail":"host.domain"}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "38358a297be27cdd7222266e171855d611d04635204abd1077a7f975c5e693cf",
                        r#"{"params":null,"known":[{"CollectFromList":{"source":"cisco.secure_endpoint.computer.network_addresses","columns":[{"member":"ip","target":"host.ip","replace":null,"upper":false},{"member":"mac","target":"host.mac","replace":[":","-"],"upper":true}],"firsts":[]}}]}"#
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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "e3157abd37781afa0bde9bc562a8dda653693e3c72b804c721bb7f6b018d4973",
                    r#"{"params":null,"known":[{"TrimThenSplit":{"source":"cisco.secure_endpoint.command_line.arguments","text":"process.command_line","list":"process.args","separator":" "}}]}"#
                ),
            )?;

            let _cond = { event.has_value("process.args") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "f5b39befedd4c90a4916a5cbbc74a54e658ea8f1a5b201a317e844581bdadfd6",
                        r#"{"params":null,"known":[{"FirstElement":{"takes":[{"array":"process.args","target":"process.executable","only_if_unset":false,"only_if_single":false,"consumes_array":false}]}}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "1a54a513ac6236de0fd572920f0835efb1f4eaa54ae323947b7b42e8946c80ad",
                        r#"{"params":null,"known":[{"CollectFromList":{"source":"cisco.secure_endpoint.computer.network_addresses","columns":[{"member":"ip","target":"related.ip","replace":null,"upper":false}],"firsts":[]}}]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.secure_endpoint.computer.network_addresses") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "a0f74c7c44252bb7cba389b121dea5be305e79c8f822fc2597993dd1d3f89e05",
                        r#"{"params":null,"known":[{"CollectFromList":{"source":"cisco.secure_endpoint.computer.network_addresses","columns":[{"member":"mac","target":"cisco.secure_endpoint.related.mac","replace":[":","-"],"upper":true}],"firsts":[]}}]}"#
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
