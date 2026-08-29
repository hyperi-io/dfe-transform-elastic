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
            event.set("ecs.version", json!("9.4.0"))?;

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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
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

            parse_json_field(event, "event.original", "thor")?;

            // Begin nested pipeline: "categorize"
            let _cond = { event.has_value("thor.module") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\ndef m = params.get(ctx.thor.module);\nif (m != null) {\n  m.forEach((k, v) -> {\n    if (v instanceof List) {\n      ctx.event[k] = new ArrayList(v);\n    } else {\n      ctx.event[k] = v;\n    }\n  });\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\ndef m = params.get(ctx.thor.module);\nif (m != null) {\n  m.forEach((k, v) -> {\n    if (v instanceof List) {\n      ctx.event[k] = new ArrayList(v);\n    } else {\n      ctx.event[k] = v;\n    }\n  });\n}"#
                        ),
                        cached_params!(
                            "{\"Amcache\":{\"category\":[\"registry\"],\"kind\":\"event\",\"type\":[\"access\"]},\"AtJobs\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Autoruns\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"DNSCache\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"EnvCheck\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"EtwWatcher\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Eventlog\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Events\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Filescan\":{\"category\":[\"file\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Firewall\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Hosts\":{\"category\":[\"host\"],\"kind\":\"event\",\"type\":[\"info\"]},\"HotfixCheck\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Init\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"LSASessions\":{\"category\":[\"session\",\"iam\"],\"kind\":\"event\",\"type\":[\"info\"]},\"LogScan\":{\"category\":[\"file\"],\"kind\":\"event\",\"type\":[\"info\"]},\"LoggedIn\":{\"category\":[\"iam\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Mutex\":{\"category\":[\"process\"],\"kind\":\"event\",\"type\":[\"info\"]},\"NetworkSessions\":{\"category\":[\"network\",\"session\"],\"kind\":\"event\",\"type\":[\"info\"]},\"NetworkShares\":{\"category\":[\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Pipes\":{\"category\":[\"process\"],\"kind\":\"event\",\"type\":[\"info\"]},\"ProcessCheck\":{\"category\":[\"process\"],\"kind\":\"event\",\"type\":[\"info\"]},\"ProcessConnections\":{\"category\":[\"process\",\"network\"],\"kind\":\"event\",\"type\":[\"info\"]},\"RegistryChecks\":{\"category\":[\"registry\"],\"kind\":\"event\",\"type\":[\"access\"]},\"RegistryHive\":{\"category\":[\"registry\"],\"kind\":\"event\",\"type\":[\"access\"]},\"Report\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Rootkit\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"SHIMCache\":{\"category\":[\"registry\"],\"kind\":\"event\",\"type\":[\"access\"]},\"ScheduledTasks\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"ServiceCheck\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Sigma\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Startup\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Timestomp\":{\"category\":[\"file\"],\"kind\":\"event\",\"type\":[\"info\"]},\"UserDir\":{\"category\":[\"iam\"],\"kind\":\"event\",\"type\":[\"info\",\"user\"]},\"Users\":{\"category\":[\"iam\"],\"kind\":\"event\",\"type\":[\"info\",\"user\"]},\"VulnerabilityCheck\":{\"category\":[\"vulnerability\"],\"kind\":\"event\",\"type\":[\"info\"]},\"WER\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"WMIPersistence\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"WMIStartup\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]},\"Yara\":{\"category\":[\"configuration\"],\"kind\":\"event\",\"type\":[\"info\"]}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_set_event_categorization",
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
            }
            let _cond = { event.get_str("thor.level") == Some("Alert") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }
            let _cond = { event.get_str("thor.message") == Some("Malware file found") };
            if _cond {
                event.append_unique("event.category", json!("malware"))?;
            }
            // End nested pipeline: "categorize"

            let _cond = {
                event.get("event.category").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("process")),
                    serde_json::Value::String(s) => s.contains("process"),
                    _ => false,
                })
            };
            if _cond {
                event.set("_temp.isProcess", json!(true))?;
            }

            let _cond = {
                event.get("event.category").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("network")),
                    serde_json::Value::String(s) => s.contains("network"),
                    _ => false,
                })
            };
            if _cond {
                event.set("_temp.isNetwork", json!(true))?;
            }

            let _cond = {
                event.get("event.category").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("registry")),
                    serde_json::Value::String(s) => s.contains("registry"),
                    _ => false,
                })
            };
            if _cond {
                event.set("_temp.isRegistry", json!(true))?;
            }

            if event.has_value("thor.scanid") {
                event.rename("thor.scanid", "thor.scan_id")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("log_line") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "_id",
                        json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                            TransformError::ParseError {
                                path: "_id".into(),
                                message,
                            }
                        })?),
                    )?;
                }
            }

            event.remove("log_line");

            let _cond = {
                event.has_value("thor.file")
                    && event.get("thor.file").is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("thor.file") {
                    event.rename("thor.file", "file.path")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("thor.time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "thor.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "parse_thor_time")?;
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

            let _cond = { event.has_value("@timestamp") };
            if _cond {
                event.remove("thor.time");
            }

            let _cond =
                { event.has_value("thor.end_time") && event.get_str("thor.end_time") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.end_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("event.end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.end_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_thor_end_time")?;
                    event.remove("thor.end_time");
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

            let _cond = { event.has_value("event.end") };
            if _cond {
                event.remove("thor.end_time");
            }

            if event.has_value("thor.level") {
                event.rename("thor.level", "log.level")?;
            }

            if event.has_value("thor.message") {
                event.rename("thor.message", "message")?;
            }

            if event.has_value("thor.module") {
                event.rename("thor.module", "event.module")?;
            }

            let _cond =
                { event.has_value("thor.image_path") && !event.has_value("thor.image.path") };
            if _cond {
                if let Some(v) = event.get("thor.image_path").cloned() {
                    event.set("thor.image.path", v)?;
                }
            }

            let _cond = { event.has_value("thor.image.path") };
            if _cond {
                event.remove("thor.image_path");
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("thor.exec_flag") {
                    if let Some(val) = event.get("thor.exec_flag") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.exec_flag".into(),
                                message,
                            }
                        })?;
                        event.set("thor.exec_flag", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_exec_flag_to_boolean",
                )?;
                event.remove("thor.exec_flag");
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
                if event.has_value("thor.enabled") {
                    if let Some(val) = event.get("thor.enabled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.enabled".into(),
                                message,
                            }
                        })?;
                        event.set("thor.enabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_enabled_to_boolean",
                )?;
                event.remove("thor.enabled");
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
                if event.has_value("thor.is_admin") {
                    if let Some(val) = event.get("thor.is_admin") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.is_admin".into(),
                                message,
                            }
                        })?;
                        event.set("thor.is_admin", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_is_admin_to_boolean",
                )?;
                event.remove("thor.is_admin");
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
                if event.has_value("thor.no_expire") {
                    if let Some(val) = event.get("thor.no_expire") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.no_expire".into(),
                                message,
                            }
                        })?;
                        event.set("thor.no_expire", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_no_expire_to_boolean",
                )?;
                event.remove("thor.no_expire");
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
                if event.has_value("thor.active") {
                    if let Some(val) = event.get("thor.active") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.active".into(),
                                message,
                            }
                        })?;
                        event.set("thor.active", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_active_to_boolean",
                )?;
                event.remove("thor.active");
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
                if event.has_value("thor.locked") {
                    if let Some(val) = event.get("thor.locked") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.locked".into(),
                                message,
                            }
                        })?;
                        event.set("thor.locked", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_locked_to_boolean",
                )?;
                event.remove("thor.locked");
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
                if event.has_value("thor.valid") {
                    if let Some(val) = event.get("thor.valid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.valid".into(),
                                message,
                            }
                        })?;
                        event.set("thor.valid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_valid_to_boolean",
                )?;
                event.remove("thor.valid");
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
                if event.has_value("thor.badpwcount") {
                    if let Some(val) = event.get("thor.badpwcount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.badpwcount".into(),
                                message,
                            }
                        })?;
                        event.set("thor.badpwcount", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_badpwcount_to_long",
                )?;
                event.remove("thor.badpwcount");
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
                if event.has_value("thor.entries") {
                    if let Some(val) = event.get("thor.entries") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.entries".into(),
                                message,
                            }
                        })?;
                        event.set("thor.entries", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_entries_to_long",
                )?;
                event.remove("thor.entries");
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
                if event.has_value("thor.event_id") {
                    if let Some(val) = event.get("thor.event_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.event_id".into(),
                                message,
                            }
                        })?;
                        event.set("thor.event_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_event_id_to_long",
                )?;
                event.remove("thor.event_id");
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
                if event.has_value("thor.num_logons") {
                    if let Some(val) = event.get("thor.num_logons") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.num_logons".into(),
                                message,
                            }
                        })?;
                        event.set("thor.num_logons", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_num_logons_to_long",
                )?;
                event.remove("thor.num_logons");
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
                if event.has_value("thor.scanned") {
                    if let Some(val) = event.get("thor.scanned") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.scanned".into(),
                                message,
                            }
                        })?;
                        event.set("thor.scanned", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_scanned_to_long",
                )?;
                event.remove("thor.scanned");
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
                if event.has_value("thor.pass_age") {
                    if let Some(val) = event.get("thor.pass_age") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.pass_age".into(),
                                message,
                            }
                        })?;
                        event.set("thor.pass_age", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_pass_age_to_double",
                )?;
                event.remove("thor.pass_age");
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

            let _cond = { event.get_bool("_temp.isProcess") == Some(true) };
            if _cond {
                if event.has_value("thor.pid") {
                    event.rename("thor.pid", "process.pid")?;
                }
            }

            let _cond = { event.get_bool("_temp.isProcess") == Some(true) };
            if _cond {
                if event.has_value("thor.ppid") {
                    event.rename("thor.ppid", "process.parent.pid")?;
                }
            }

            let _cond = { event.get_bool("_temp.isProcess") == Some(true) };
            if _cond {
                if event.has_value("thor.process_name") {
                    event.rename("thor.process_name", "process.name")?;
                }
            }

            let _cond = { event.get_bool("_temp.isProcess") == Some(true) };
            if _cond {
                if event.has_value("thor.command") {
                    event.rename("thor.command", "process.command_line")?;
                }
            }

            let _cond = { event.get_bool("_temp.isProcess") == Some(true) };
            if _cond {
                if event.has_value("thor.parent") {
                    event.rename("thor.parent", "process.parent.executable")?;
                }
            }

            let _cond = { event.get_bool("_temp.isProcess") == Some(true) };
            if _cond {
                if event.has_value("thor.image.path") {
                    event.rename("thor.image.path", "process.executable")?;
                }
            }

            let _cond = { event.get_bool("_temp.isNetwork") == Some(true) };
            if _cond {
                if event.has_value("thor.protocol") {
                    event.rename("thor.protocol", "network.transport")?;
                }
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("thor.ip") == Some("*") };
            if _cond {
                event.remove("thor.ip");
            }

            let _cond = { event.get_str("thor.rip") == Some("*") };
            if _cond {
                event.remove("thor.rip");
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("thor.ip") {
                    if let Some(val) = event.get("thor.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.ip".into(),
                                message,
                            }
                        })?;
                        event.set("thor.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_ip_24095cde",
                )?;
                event.remove("thor.ip");
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

            let _cond = { event.get_bool("_temp.isNetwork") == Some(true) };
            if _cond {
                if event.has_value("thor.ip") {
                    event.rename("thor.ip", "source.ip")?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("thor.port") {
                    if let Some(val) = event.get("thor.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.port".into(),
                                message,
                            }
                        })?;
                        event.set("thor.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_port_8044a21e",
                )?;
                event.remove("thor.port");
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

            let _cond = { event.get_bool("_temp.isNetwork") == Some(true) };
            if _cond {
                if event.has_value("thor.port") {
                    event.rename("thor.port", "source.port")?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("thor.rip") {
                    if let Some(val) = event.get("thor.rip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.rip".into(),
                                message,
                            }
                        })?;
                        event.set("thor.rip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_rip_061cd984",
                )?;
                event.remove("thor.rip");
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

            let _cond = { event.get_bool("_temp.isNetwork") == Some(true) };
            if _cond {
                if event.has_value("thor.rip") {
                    event.rename("thor.rip", "destination.ip")?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("thor.rport") {
                    if let Some(val) = event.get("thor.rport") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "thor.rport".into(),
                                message,
                            }
                        })?;
                        event.set("thor.rport", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_thor_rport_08e203a6",
                )?;
                event.remove("thor.rport");
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

            let _cond = { event.get_bool("_temp.isNetwork") == Some(true) };
            if _cond {
                if event.has_value("thor.rport") {
                    event.rename("thor.rport", "destination.port")?;
                }
            }

            let _cond = { event.get_str("event.module") == Some("ProcessConnections") };
            if _cond {
                event.append_unique("event.type", json!("connection"))?;
            }

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("source.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("destination.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.get_bool("_temp.isRegistry") == Some(true) };
            if _cond {
                if event.has_value("thor.key") {
                    event.rename("thor.key", "registry.key")?;
                }
            }

            let _cond = { event.get_bool("_temp.isRegistry") == Some(true) };
            if _cond {
                if event.has_value("thor.entry") {
                    event.rename("thor.entry", "registry.value")?;
                }
            }

            let _cond = { event.get_bool("_temp.isRegistry") == Some(true) };
            if _cond {
                if event.has_value("thor.hive") {
                    event.rename("thor.hive", "registry.path")?;
                }
            }

            let _cond = {
                event.get("thor.reasons").is_some_and(|v| v.is_array()) && event.get("thor.reasons").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                if let Some(v) = event
                    .get("thor.reasons.0.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
            }

            let _cond = {
                event.get("thor.reasons").is_some_and(|v| v.is_array()) && event.get("thor.reasons").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.has_value("thor.reasons.0.signature.rulename")
            };
            if _cond {
                if let Some(v) = event
                    .get("thor.reasons.0.signature.rulename")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.id", v)?;
                }
            }

            let _cond = {
                event.get("thor.reasons").is_some_and(|v| v.is_array()) && event.get("thor.reasons").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.has_value("thor.reasons.0.signature.author")
            };
            if _cond {
                let v = Value::Array(vec![json!(
                    event
                        .get("thor.reasons.0.signature.author")
                        .map_or_else(String::new, template_to_string)
                )]);
                if !painless_is_empty_value(&v) {
                    event.set("rule.author", v)?;
                }
            }

            let _cond = {
                event.get("thor.reasons").is_some_and(|v| v.is_array()) && event.get("thor.reasons").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                if let Some(v) = event
                    .get("thor.reasons.0.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.description", v)?;
                }
            }

            let _cond = {
                event.get("thor.reasons").is_some_and(|v| v.is_array()) && event.get("thor.reasons").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.has_value("thor.reasons.0.signature.ref")
            };
            if _cond {
                if let Some(v) = event
                    .get("thor.reasons.0.signature.ref")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.ruleset", v)?;
                }
            }

            if event.has_value("thor.log_version") {
                event.rename("thor.log_version", "event.version")?;
            }

            if event.has_value("thor.user") {
                event.rename("thor.user", "user.name")?;
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

            if event.has_value("thor.userid") {
                event.rename("thor.userid", "user.id")?;
            }

            let _cond = { event.get_str("event.module") == Some("Users") };
            if _cond {
                if event.has_value("thor.full_name") {
                    event.rename("thor.full_name", "user.full_name")?;
                }
            }

            if event.has_value("thor.domain") {
                event.rename("thor.domain", "user.domain")?;
            }

            if event.has_value("thor.hostname") {
                event.rename("thor.hostname", "host.name")?;
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

            if event.has_value("thor.file.group") {
                event.rename("thor.file.group", "file.group")?;
            }

            if event.has_value("thor.file.md5") {
                event.rename("thor.file.md5", "file.hash.md5")?;
            }

            if event.has_value("thor.file.owner") {
                event.rename("thor.file.owner", "file.owner")?;
            }

            let _cond = { !event.has_value("file.path") };
            if _cond {
                if event.has_value("thor.file.path") {
                    event.rename("thor.file.path", "file.path")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("file.path") {
                    if let Some(input) = event.get_string("file.path") {
                        // Grok pattern: ^(?:[A-Za-z]:\\\\|\\\\)(?:.*\\\\)?(?P<file_name>(?:[^\\\\]+))$
                        // Grok pattern: ^/(?:.*/)?(?P<file_name>(?:[^/]+))$
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^(?:[A-Za-z]:\\\\|\\\\)(?:.*\\\\)?(?P<file_name>(?:[^\\\\]+))$",
                                    [("file_name", "file.name")]
                                ),
                                cached_grok_mapped!(
                                    "^/(?:.*/)?(?P<file_name>(?:[^/]+))$",
                                    [("file_name", "file.name")]
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "extract_file_name_from_path",
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

            if event.has_value("thor.file.sha1") {
                event.rename("thor.file.sha1", "file.hash.sha1")?;
            }

            if event.has_value("thor.file.sha256") {
                event.rename("thor.file.sha256", "file.hash.sha256")?;
            }

            if event.has_value("thor.file.size") {
                event.rename("thor.file.size", "file.size")?;
            }

            if event.has_value("thor.lastrun") {
                event.rename("thor.lastrun", "thor.last_run")?;
            }

            if event.has_value("thor.nextrun") {
                event.rename("thor.nextrun", "thor.next_run")?;
            }

            if event.has_value("thor.firstbytes") {
                event.rename("thor.firstbytes", "thor.first_bytes")?;
            }

            if event.has_value("thor.file.firstbytes") {
                event.rename("thor.file.firstbytes", "thor.file.first_bytes")?;
            }

            let _cond = { event.get("thor.files").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("thor.files") {
                    foreach_array(event, "thor.files", |event| {
                        if event.has_value("_ingest._value.firstbytes") {
                            event.rename(
                                "_ingest._value.firstbytes",
                                "_ingest._value.first_bytes",
                            )?;
                        }
                        Ok(())
                    })?;
                }
            }

            if event.has_value("thor.eventconsumername") {
                event.rename("thor.eventconsumername", "thor.event_consumer_name")?;
            }

            if event.has_value("thor.eventconsumer") {
                event.rename("thor.eventconsumer", "thor.event_consumer")?;
            }

            if event.has_value("thor.eventfiltername") {
                event.rename("thor.eventfiltername", "thor.event_filter_name")?;
            }

            if event.has_value("thor.eventfilter") {
                event.rename("thor.eventfilter", "thor.event_filter")?;
            }

            if event.has_value("thor.filtertype") {
                event.rename("thor.filtertype", "thor.filter_type")?;
            }

            if event.has_value("thor.file.desc") {
                event.rename("thor.file.desc", "thor.file.description")?;
            }

            let _cond = { event.get("thor.files").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("thor.files") {
                    foreach_array(event, "thor.files", |event| {
                        if event.has_value("_ingest._value.desc") {
                            event.rename("_ingest._value.desc", "_ingest._value.description")?;
                        }
                        Ok(())
                    })?;
                }
            }

            if event.has_value("thor.image.desc") {
                event.rename("thor.image.desc", "thor.image.description")?;
            }

            if event.has_value("thor.image.firstbytes") {
                event.rename("thor.image.firstbytes", "thor.image.first_bytes")?;
            }

            if event.has_value("thor.app.firstbytes") {
                event.rename("thor.app.firstbytes", "thor.app.first_bytes")?;
            }

            if event.has_value("thor.app.desc") {
                event.rename("thor.app.desc", "thor.app.description")?;
            }

            if event.has_value("thor.archive.firstbytes") {
                event.rename("thor.archive.firstbytes", "thor.archive.first_bytes")?;
            }

            let _cond = { event.has_value("thor.file.accessed") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.file.accessed") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("file.accessed", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.file.accessed".into(),
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
                        "parse_thor_file_accessed",
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
            }

            let _cond = { event.has_value("file.accessed") };
            if _cond {
                event.remove("thor.file.accessed");
            }

            let _cond = { event.has_value("thor.file.changed") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.file.changed") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("file.ctime", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.file.changed".into(),
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
                        "parse_thor_file_changed",
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
            }

            let _cond = { event.has_value("file.ctime") };
            if _cond {
                event.remove("thor.file.changed");
            }

            let _cond = { event.has_value("thor.file.modified") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.file.modified") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("file.mtime", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.file.modified".into(),
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
                        "parse_thor_file_modified",
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
            }

            let _cond = { event.has_value("file.mtime") };
            if _cond {
                if event.remove("thor.file.modified").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "thor.file.modified".into(),
                    });
                }
            }

            let _cond = { event.has_value("thor.image.accessed") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.image.accessed") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.image.accessed", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.image.accessed".into(),
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
                        "parse_thor_image_accessed",
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
            }

            let _cond = { event.has_value("thor.image.created") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.image.created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.image.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.image.created".into(),
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
                        "parse_thor_image_created",
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
            }

            let _cond = { event.has_value("thor.image.modified") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.image.modified") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.image.modified", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.image.modified".into(),
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
                        "parse_thor_image_modified",
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
            }

            let _cond = { event.get("thor.files").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("thor.files") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("thor.files").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if let Some(date_str) =
                                            event.get_as_string("_ingest._value.created")
                                        {
                                            match parse_date_out(
                                                &date_str,
                                                &["ISO8601"],
                                                None,
                                                None,
                                            ) {
                                                Some(parsed) => {
                                                    event.set("_ingest._value.created", parsed)?
                                                }
                                                None => {
                                                    return Err(TransformError::ParseError {
                                                        path: "_ingest._value.created".into(),
                                                        message: format!(
                                                            "unable to parse date [{date_str}]"
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                        Ok(())
                                    })();
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "thor.files",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "foreach")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "foreach_parse_thor_files_created",
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
            }

            let _cond = { event.get("thor.files").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("thor.files") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("thor.files").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if let Some(date_str) =
                                            event.get_as_string("_ingest._value.accessed")
                                        {
                                            match parse_date_out(
                                                &date_str,
                                                &["ISO8601"],
                                                None,
                                                None,
                                            ) {
                                                Some(parsed) => {
                                                    event.set("_ingest._value.accessed", parsed)?
                                                }
                                                None => {
                                                    return Err(TransformError::ParseError {
                                                        path: "_ingest._value.accessed".into(),
                                                        message: format!(
                                                            "unable to parse date [{date_str}]"
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                        Ok(())
                                    })();
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "thor.files",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "foreach")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "foreach_parse_thor_files_accessed",
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
            }

            let _cond = { event.get("thor.files").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("thor.files") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("thor.files").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if let Some(date_str) =
                                            event.get_as_string("_ingest._value.modified")
                                        {
                                            match parse_date_out(
                                                &date_str,
                                                &["ISO8601"],
                                                None,
                                                None,
                                            ) {
                                                Some(parsed) => {
                                                    event.set("_ingest._value.modified", parsed)?
                                                }
                                                None => {
                                                    return Err(TransformError::ParseError {
                                                        path: "_ingest._value.modified".into(),
                                                        message: format!(
                                                            "unable to parse date [{date_str}]"
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                        Ok(())
                                    })();
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "thor.files",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "foreach")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "foreach_parse_thor_files_modified",
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
            }

            let _cond = { event.has_value("thor.file.created") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.file.created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("file.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.file.created".into(),
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
                        "parse_thor_file_created",
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
            }

            let _cond = { event.has_value("file.created") };
            if _cond {
                if event.remove("thor.file.created").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "thor.file.created".into(),
                    });
                }
            }

            let _cond = { event.has_value("thor.created") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_created")?;
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

            let _cond = { event.has_value("thor.last_run") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.last_run") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.last_run", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.last_run".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_last_run")?;
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

            let _cond = { event.has_value("thor.modified") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.modified") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.modified", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.modified".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_modified")?;
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

            let _cond = { event.has_value("thor.next_run") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.next_run") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.next_run", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.next_run".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_next_run")?;
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
                event.has_value("thor.start")
                    && event.get("thor.start").is_some_and(|v| v.is_string())
                    && event
                        .get_str("thor.start")
                        .map(|s| s.find(",").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.thor.start = ctx.thor.start.splitOnToken(',')[0].trim();
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.thor.start = ctx.thor.start.splitOnToken(',')[0].trim();"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_normalize_thor_start_multivalue",
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
            }

            let _cond =
                { event.has_value("thor.start") && event.get_str("thor.start") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.start") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_start")?;
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

            let _cond = { event.has_value("thor.image.changed") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.image.changed") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.image.changed", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.image.changed".into(),
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
                        "parse_thor_image_changed",
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
            }

            let _cond = {
                event.has_value("thor.app.created") && event.get_str("thor.app.created") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.app.created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.app.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.app.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_app_created")?;
                    event.remove("thor.app.created");
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
                event.has_value("thor.archive.accessed")
                    && event.get_str("thor.archive.accessed") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.archive.accessed") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.archive.accessed", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.archive.accessed".into(),
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
                        "parse_thor_archive_accessed",
                    )?;
                    event.remove("thor.archive.accessed");
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
                event.has_value("thor.archive.created")
                    && event.get_str("thor.archive.created") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.archive.created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.archive.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.archive.created".into(),
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
                        "parse_thor_archive_created",
                    )?;
                    event.remove("thor.archive.created");
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
                event.has_value("thor.archive.modified")
                    && event.get_str("thor.archive.modified") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.archive.modified") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.archive.modified", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.archive.modified".into(),
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
                        "parse_thor_archive_modified",
                    )?;
                    event.remove("thor.archive.modified");
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

            let _cond = { event.has_value("thor.date") && event.get_str("thor.date") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_date")?;
                    event.remove("thor.date");
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
                event.has_value("thor.event_time") && event.get_str("thor.event_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.event_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.event_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.event_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_event_time")?;
                    event.remove("thor.event_time");
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
                event.has_value("thor.last_logon") && event.get_str("thor.last_logon") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.last_logon") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.last_logon", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.last_logon".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_last_logon")?;
                    event.remove("thor.last_logon");
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
                event.has_value("thor.log_accessed")
                    && event.get_str("thor.log_accessed") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.log_accessed") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.log_accessed", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.log_accessed".into(),
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
                        "parse_thor_log_accessed",
                    )?;
                    event.remove("thor.log_accessed");
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
                event.has_value("thor.log_created") && event.get_str("thor.log_created") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.log_created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.log_created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.log_created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_log_created")?;
                    event.remove("thor.log_created");
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
                event.has_value("thor.log_modified")
                    && event.get_str("thor.log_modified") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.log_modified") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.log_modified", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.log_modified".into(),
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
                        "parse_thor_log_modified",
                    )?;
                    event.remove("thor.log_modified");
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
                event.has_value("thor.start_time") && event.get_str("thor.start_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.start_time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_start_time")?;
                    event.remove("thor.start_time");
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
                event.has_value("thor.timestamp") && event.get_str("thor.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thor.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "parse_thor_timestamp")?;
                    event.remove("thor.timestamp");
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
                event.has_value("thor.installed_on")
                    && event.get_str("thor.installed_on") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("thor.installed_on") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "M/d/yyyy", "MM/dd/yyyy"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("thor.installed_on", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "thor.installed_on".into(),
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
                        "parse_thor_installed_on",
                    )?;
                    event.remove("thor.installed_on");
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

            let _cond = { event.get("thor.reasons").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("thor.reasons") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("thor.reasons").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if let Some(date_str) =
                                            event.get_as_string("_ingest._value.signature.ruledate")
                                        {
                                            match parse_date_out(
                                                &date_str,
                                                &["ISO8601"],
                                                None,
                                                None,
                                            ) {
                                                Some(parsed) => event.set(
                                                    "_ingest._value.signature.ruledate",
                                                    parsed,
                                                )?,
                                                None => {
                                                    return Err(TransformError::ParseError {
                                                        path: "_ingest._value.signature.ruledate"
                                                            .into(),
                                                        message: format!(
                                                            "unable to parse date [{date_str}]"
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                        Ok(())
                                    })();
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "thor.reasons",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "foreach")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "foreach_parse_thor_reasons_signature_ruledate",
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
            }

            let _cond = { event.has_value("thor.duration") };
            if _cond {
                // Painless script
                // Source: String s = ctx.thor.duration.toString().trim();\nif (s.length() == 0) return;\n\nlong hours = 0L;\nlong mins  = 0L;\nlong secs  = 0L;\n\nString[] parts = s.splitOnToken(\" \");\nfor (int i = 0; i < parts.length - 1; i++) {\n  String v = parts[i];\n  String u = parts[i + 1].toLowerCase(Locale.ROOT);\n\n  long n;\n  try {\n    n = Long.parseLong(v);\n  } catch (Exception e) {\n    continue;  // skip non-numeric tokens safely\n  }\n\n  if (u.startsWith(\"hour\")) {\n    hours = n;\n  } else if (u.startsWith(\"min\")) {\n    mins = n;\n  } else if (u.startsWith(\"sec\")) {\n    secs = n;\n  }\n}\n\nctx.thor.duration = hours * 3600L + mins * 60L + secs;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String s = ctx.thor.duration.toString().trim();\nif (s.length() == 0) return;\n\nlong hours = 0L;\nlong mins  = 0L;\nlong secs  = 0L;\n\nString[] parts = s.splitOnToken(\" \");\nfor (int i = 0; i < parts.length - 1; i++) {\n  String v = parts[i];\n  String u = parts[i + 1].toLowerCase(Locale.ROOT);\n\n  long n;\n  try {\n    n = Long.parseLong(v);\n  } catch (Exception e) {\n    continue;  // skip non-numeric tokens safely\n  }\n\n  if (u.startsWith(\"hour\")) {\n    hours = n;\n  } else if (u.startsWith(\"min\")) {\n    mins = n;\n  } else if (u.startsWith(\"sec\")) {\n    secs = n;\n  }\n}\n\nctx.thor.duration = hours * 3600L + mins * 60L + secs;"#
                    ),
                )?;
            }

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("file.hash.md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("file.hash.sha1")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("file.hash.sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("file.owner")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("thor.run_as_user")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("thor.exe_owner")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("thor.image.owner")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("thor.owner")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("thor.unit_owner")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.get("thor.files").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("thor.files") {
                    foreach_array(event, "thor.files", |event| {
                        let _cond = {
                            event.has_value("_ingest._value.owner")
                                && event.get_str("_ingest._value.owner") != Some("")
                        };
                        if _cond {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.owner")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                        }
                        Ok(())
                    })?;
                }
            }

            event.remove("_temp");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            // SKIPPED: nested pipeline "global@custom" is not in this pipeline set

            // SKIPPED: nested pipeline "logs@custom" is not in this pipeline set

            // SKIPPED: nested pipeline "logs-nextron_thor_apt_scanner.integration@custom" is not in this pipeline set

            // SKIPPED: nested pipeline "logs-nextron_thor_apt_scanner.thor_forwarding@custom" is not in this pipeline set

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
