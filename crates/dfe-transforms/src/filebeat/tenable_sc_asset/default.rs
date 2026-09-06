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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("state"))?;

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            let _cond = {
                event.has_value("json.uniqueness") && event.get_str("json.uniqueness") != Some("")
            };
            if _cond {
                // Painless script
                // Source: def keys = ctx.json.uniqueness.toString().splitOnToken(','); ctx.tenable_sc = new HashMap(); ctx.tenable_sc.asset = new HashMap(); String uniqueKey = ''; if (keys.length > 0) {\n  for (int i = 0; i < keys.length; i++) {\n    if(keys[i] == 'repositoryID') {\n      uniqueKey = uniqueKey + ctx.json.repository.id + (i == keys.length - 1 ? '' : '_');\n    } else {\n      uniqueKey = uniqueKey + ctx.json[keys[i]] + (i == keys.length - 1 ? '' : '_');\n    }\n  }\n} ctx.tenable_sc.asset.custom_hash = uniqueKey;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def keys = ctx.json.uniqueness.toString().splitOnToken(','); ctx.tenable_sc = new HashMap(); ctx.tenable_sc.asset = new HashMap(); String uniqueKey = ''; if (keys.length > 0) {\n  for (int i = 0; i < keys.length; i++) {\n    if(keys[i] == 'repositoryID') {\n      uniqueKey = uniqueKey + ctx.json.repository.id + (i == keys.length - 1 ? '' : '_');\n    } else {\n      uniqueKey = uniqueKey + ctx.json[keys[i]] + (i == keys.length - 1 ? '' : '_');\n    }\n  }\n} ctx.tenable_sc.asset.custom_hash = uniqueKey;"#
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("tenable_sc.asset.custom_hash") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "tenable_sc.asset.custom_hash",
                        json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                            TransformError::ParseError {
                                path: "tenable_sc.asset.custom_hash".into(),
                                message,
                            }
                        })?),
                    )?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ip") {
                    if let Some(val) = event.get("json.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ip".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.asset.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("tenable_sc.asset.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("tenable_sc.asset.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("tenable_sc.asset.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("tenable_sc.asset.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.uuid") {
                event.rename("json.uuid", "tenable_sc.asset.uuid")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.score") {
                    if let Some(val) = event.get("json.score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.score".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.asset.score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.total") {
                    if let Some(val) = event.get("json.total") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.total".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.asset.total", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.severityInfo") {
                    if let Some(val) = event.get("json.severityInfo") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severityInfo".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.asset.severity.info", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.severityLow") {
                    if let Some(val) = event.get("json.severityLow") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severityLow".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.asset.severity.low", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.severityMedium") {
                    if let Some(val) = event.get("json.severityMedium") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severityMedium".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.asset.severity.medium", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.severityHigh") {
                    if let Some(val) = event.get("json.severityHigh") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severityHigh".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.asset.severity.high", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
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
                if event.has_value("json.severityCritical") {
                    if let Some(val) = event.get("json.severityCritical") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severityCritical".into(),
                                message,
                            }
                        })?;
                        event.set("tenable_sc.asset.severity.critical", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.macAddress") {
                gsub_field(
                    event,
                    "json.macAddress",
                    "json.macAddress",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            if event.has_value("json.macAddress") {
                map_strings(
                    event,
                    "json.macAddress",
                    "json.macAddress",
                    str::to_uppercase,
                )?;
            }

            let _cond = { event.has_value("json.macAddress") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "host.mac",
                        json!(
                            event
                                .get("json.macAddress")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.macAddress") {
                event.rename("json.macAddress", "tenable_sc.asset.mac")?;
            }

            if event.has_value("json.policyName") {
                event.rename("json.policyName", "tenable_sc.asset.policy.name")?;
            }

            if event.has_value("json.pluginSet") {
                event.rename("json.pluginSet", "tenable_sc.asset.plugin_set")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.netbiosName").cloned() {
                    event.set("tenable_sc.asset.netbios.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.dnsName").cloned() {
                    event.set("host.hostname", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.dnsName").cloned() {
                    event.set("tenable_sc.asset.dns.name", v)?;
                }
                Ok(())
            })();

            let _cond =
                { event.has_value("json.dnsName") && event.get_str("json.dnsName") != Some("") };
            if _cond {
                // Painless script
                // Source: def domain = '';\ndef nameArray = ctx.json.dnsName.toString().splitOnToken('.');\nif (nameArray?.length != null && nameArray.length > 0) {\n  for (int i = 1; i < nameArray.length; i++) {\n    domain += nameArray[i] + (i < nameArray.length - 1 ? '.' : '');\n  }\n  ctx.host.name = nameArray[0];\n  ctx.host.domain = domain;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def domain = '';\ndef nameArray = ctx.json.dnsName.toString().splitOnToken('.');\nif (nameArray?.length != null && nameArray.length > 0) {\n  for (int i = 1; i < nameArray.length; i++) {\n    domain += nameArray[i] + (i < nameArray.length - 1 ? '.' : '');\n  }\n  ctx.host.name = nameArray[0];\n  ctx.host.domain = domain;\n}\n"#
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("json.netbiosName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("json.osCPE") {
                event.rename("json.osCPE", "tenable_sc.asset.os_cpe")?;
            }

            if event.has_value("json.biosGUID") {
                event.rename("json.biosGUID", "tenable_sc.asset.bios.guid")?;
            }

            if event.has_value("json.tpmID") {
                event.rename("json.tpmID", "tenable_sc.asset.tpm.id")?;
            }

            if event.has_value("json.mcafeeGUID") {
                event.rename("json.mcafeeGUID", "tenable_sc.asset.mcafee.guid")?;
            }

            if event.has_value("json.lastAuthRun") {
                event.rename("json.lastAuthRun", "tenable_sc.asset.last_auth_run")?;
            }

            if event.has_value("json.lastUnauthRun") {
                event.rename("json.lastUnauthRun", "tenable_sc.asset.last_unauth_run")?;
            }

            if event.has_value("json.hostUniqueness") {
                event.rename("json.hostUniqueness", "tenable_sc.asset.host_uniqueness")?;
            }

            if event.has_value("json.uniqueness") {
                event.rename("json.uniqueness", "tenable_sc.asset.uniqueness")?;
            }

            if event.has_value("json.repository.id") {
                event.rename("json.repository.id", "tenable_sc.asset.repository.id")?;
            }

            if event.has_value("json.repository.name") {
                event.rename("json.repository.name", "tenable_sc.asset.repository.name")?;
            }

            if event.has_value("json.repository.description") {
                event.rename(
                    "json.repository.description",
                    "tenable_sc.asset.repository.description",
                )?;
            }

            if event.has_value("json.repository.sciID") {
                event.rename(
                    "json.repository.sciID",
                    "tenable_sc.asset.repository.sci.id",
                )?;
            }

            if event.has_value("json.repository.dataFormat") {
                event.rename(
                    "json.repository.dataFormat",
                    "tenable_sc.asset.repository.data_format",
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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

            event.remove("json");

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
