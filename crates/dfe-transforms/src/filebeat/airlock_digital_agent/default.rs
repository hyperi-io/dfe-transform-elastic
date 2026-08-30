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

            parse_json_field(event, "event.original", "json")?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            event.append("event.category", json!("host"))?;

            event.set("observer.vendor", json!("Airlock Digital"))?;

            if event.has_value("json.agentid") {
                event.rename("json.agentid", "airlock_digital.agent.agentid")?;
            }

            if let Some(v) = event
                .get("airlock_digital.agent.agentid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if event.has_value("json.clientversion") {
                event.rename("json.clientversion", "airlock_digital.agent.clientversion")?;
            }

            if event.has_value("json.domain") {
                event.rename("json.domain", "airlock_digital.agent.domain")?;
            }

            if let Some(v) = event
                .get("airlock_digital.agent.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }

            let _cond = { event.has_value("airlock_digital.agent.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("airlock_digital.agent.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.freespace") {
                    if let Some(val) = event.get("json.freespace") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.freespace".into(),
                                message,
                            }
                        })?;
                        event.set("airlock_digital.agent.freespace", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_freespace_to_long_224d6972",
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

            if event.has_value("json.groupid") {
                event.rename("json.groupid", "airlock_digital.agent.groupid")?;
            }

            if let Some(v) = event
                .get("airlock_digital.agent.groupid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has_value("json.hostname") {
                event.rename("json.hostname", "airlock_digital.agent.hostname")?;
            }

            if let Some(v) = event
                .get("airlock_digital.agent.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("airlock_digital.agent.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("airlock_digital.agent.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.ip") != Some("") };
            if _cond {
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
                            event.set("airlock_digital.agent.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ip_to_ip_38823b29",
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

            let _cond = { event.has_value("airlock_digital.agent.ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("airlock_digital.agent.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("airlock_digital.agent.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("airlock_digital.agent.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.lastcheckin") && event.get_str("json.lastcheckin") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastcheckin") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("airlock_digital.agent.lastcheckin", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.lastcheckin".into(),
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
                        "date_lastcheckin_265aeac0",
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

            let _cond = { event.get_str("json.localip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.localip") {
                        if let Some(val) = event.get("json.localip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.localip".into(),
                                    message,
                                }
                            })?;
                            event.set("airlock_digital.agent.localip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_localip_to_ip_4bcabbce",
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

            let _cond = { event.has_value("airlock_digital.agent.localip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("airlock_digital.agent.localip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.os") {
                event.rename("json.os", "airlock_digital.agent.os")?;
            }

            if let Some(v) = event
                .get("airlock_digital.agent.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            let _cond = { event.has_value("host.os.full") };
            if _cond {
                if let Some(input) = event.get_string("host.os.full") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("host.os.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "host.os.full".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.has_value("host.os.full") };
            if _cond {
                // Painless script
                // Source: String os_family = ctx.host.os.full.toLowerCase();\nfor (String os: params.os_type) {\n  if (os_family.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String os_family = ctx.host.os.full.toLowerCase();\nfor (String os: params.os_type) {\n  if (os_family.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"os_type\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                    ),
                )?;
            }

            if event.has_value("json.policy_details.agentstopcode") {
                event.rename(
                    "json.policy_details.agentstopcode",
                    "airlock_digital.agent.poilcy_details.agentstopcode",
                )?;
            }

            if event.has_value("json.policy_details.applications") {
                event.rename(
                    "json.policy_details.applications",
                    "airlock_digital.agent.poilcy_details.applications",
                )?;
            }

            if event.has_value("json.policy_details.auditmode") {
                if let Some(val) = event.get("json.policy_details.auditmode") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.auditmode".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.auditmode", converted)?;
                }
            }

            if event.has_value("json.policy_details.autoupdate") {
                if let Some(val) = event.get("json.policy_details.autoupdate") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.autoupdate".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.autoupdate", converted)?;
                }
            }

            if event.has_value("json.policy_details.baselines") {
                event.rename(
                    "json.policy_details.baselines",
                    "airlock_digital.agent.poilcy_details.baselines",
                )?;
            }

            if event.has_value("json.policy_details.batch") {
                if let Some(val) = event.get("json.policy_details.batch") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.batch".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.batch", converted)?;
                }
            }

            if event.has_value("json.policy_details.blocklists") {
                event.rename(
                    "json.policy_details.blocklists",
                    "airlock_digital.agent.poilcy_details.blocklists",
                )?;
            }

            if event.has_value("json.policy_details.check_ea") {
                if let Some(val) = event.get("json.policy_details.check_ea") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.check_ea".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.check_ea", converted)?;
                }
            }

            if event.has_value("json.policy_details.command") {
                if let Some(val) = event.get("json.policy_details.command") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.command".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.command", converted)?;
                }
            }

            let _cond = {
                event
                    .get("json.policy_details.commlist")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.policy_details.commlist", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.ip") {
                            if let Some(val) = event.get("_ingest._value.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_policy_details_commlist_ip_to_ip_69f3a84c",
                        )?;
                        event.remove("_ingest._value.ip");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.policy_details.commlist")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.policy_details.commlist", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.policy_details.commlist") {
                event.rename(
                    "json.policy_details.commlist",
                    "airlock_digital.agent.poilcy_details.commlist",
                )?;
            }

            if event.has_value("json.policy_details.commlistid") {
                event.rename(
                    "json.policy_details.commlistid",
                    "airlock_digital.agent.poilcy_details.commlistid",
                )?;
            }

            if event.has_value("json.policy_details.compiledhtml") {
                if let Some(val) = event.get("json.policy_details.compiledhtml") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.compiledhtml".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.compiledhtml",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.dylib") {
                if let Some(val) = event.get("json.policy_details.dylib") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.dylib".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.dylib", converted)?;
                }
            }

            if event.has_value("json.policy_details.enable_notifications") {
                if let Some(val) = event.get("json.policy_details.enable_notifications") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.enable_notifications".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.enable_notifications",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.extensions_enabled") {
                if let Some(val) = event.get("json.policy_details.extensions_enabled") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.extensions_enabled".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.extensions_enabled",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.generalisation") {
                if let Some(val) = event.get("json.policy_details.generalisation") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.generalisation".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.generalisation",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.gprocesses") {
                event.rename(
                    "json.policy_details.gprocesses",
                    "airlock_digital.agent.poilcy_details.gprocesses",
                )?;
            }

            event.remove("json.policy_details.groupid");

            if event.has_value("json.policy_details.hashdb_ver") {
                if let Some(val) = event.get("json.policy_details.hashdb_ver") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.hashdb_ver".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.hashdb_ver", converted)?;
                }
            }

            if event.has_value("json.policy_details.htmlapplication") {
                if let Some(val) = event.get("json.policy_details.htmlapplication") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.htmlapplication".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.htmlapplication",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.javaapplication") {
                if let Some(val) = event.get("json.policy_details.javaapplication") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.javaapplication".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.javaapplication",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.javascript") {
                if let Some(val) = event.get("json.policy_details.javascript") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.javascript".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.javascript", converted)?;
                }
            }

            if event.has_value("json.policy_details.modreload") {
                if let Some(val) = event.get("json.policy_details.modreload") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.modreload".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.modreload", converted)?;
                }
            }

            if event.has_value("json.policy_details.name") {
                event.rename(
                    "json.policy_details.name",
                    "airlock_digital.agent.poilcy_details.name",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.agent.poilcy_details.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.policy_details.notification_message") {
                event.rename(
                    "json.policy_details.notification_message",
                    "airlock_digital.agent.poilcy_details.notification_message",
                )?;
            }

            if let Some(v) = event
                .get("airlock_digital.agent.poilcy_details.notification_message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("json.policy_details.parent") {
                event.rename(
                    "json.policy_details.parent",
                    "airlock_digital.agent.poilcy_details.parent",
                )?;
            }

            let _cond = {
                event
                    .get("json.policy_details.paths")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.policy_details.paths", |event| {
                    event.append_unique(
                        "file.path",
                        json!(
                            event
                                .get("_ingest._value.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.policy_details.paths") {
                event.rename(
                    "json.policy_details.paths",
                    "airlock_digital.agent.poilcy_details.paths",
                )?;
            }

            if event.has_value("json.policy_details.policyver") {
                event.rename(
                    "json.policy_details.policyver",
                    "airlock_digital.agent.poilcy_details.policyver",
                )?;
            }

            if event.has_value("json.policy_details.poll_time") {
                if let Some(val) = event.get("json.policy_details.poll_time") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.poll_time".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.poll_time", converted)?;
                }
            }

            if event.has_value("json.policy_details.powershell") {
                if let Some(val) = event.get("json.policy_details.powershell") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.powershell".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.powershell", converted)?;
                }
            }

            let _cond = {
                event
                    .get("json.policy_details.pprocesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.policy_details.pprocesses", |event| {
                    event.append_unique(
                        "process.parent.name",
                        json!(
                            event
                                .get("_ingest._value.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.policy_details.pprocesses") {
                event.rename(
                    "json.policy_details.pprocesses",
                    "airlock_digital.agent.poilcy_details.pprocesses",
                )?;
            }

            if event.has_value("json.policy_details.proxyauth") {
                if let Some(val) = event.get("json.policy_details.proxyauth") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.proxyauth".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.proxyauth", converted)?;
                }
            }

            if event.has_value("json.policy_details.proxyenabled") {
                if let Some(val) = event.get("json.policy_details.proxyenabled") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.proxyenabled".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.proxyenabled",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.proxypass") {
                event.rename(
                    "json.policy_details.proxypass",
                    "airlock_digital.agent.poilcy_details.proxypass",
                )?;
            }

            let _cond = {
                event.has_value("json.policy_details.proxyport")
                    && event.get_str("json.policy_details.proxyport") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.policy_details.proxyport") {
                        if let Some(val) = event.get("json.policy_details.proxyport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.policy_details.proxyport".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("airlock_digital.agent.poilcy_details.proxyport", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_policy_details_proxyport_to_long_bf44a572",
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

            if event.has_value("json.policy_details.proxyserver") {
                event.rename(
                    "json.policy_details.proxyserver",
                    "airlock_digital.agent.poilcy_details.proxyserver",
                )?;
            }

            if event.has_value("json.policy_details.proxyuser") {
                event.rename(
                    "json.policy_details.proxyuser",
                    "airlock_digital.agent.poilcy_details.proxyuser",
                )?;
            }

            if event.has_value("json.policy_details.pslockdown") {
                if let Some(val) = event.get("json.policy_details.pslockdown") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.pslockdown".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.pslockdown", converted)?;
                }
            }

            if event.has_value("json.policy_details.publishers") {
                event.rename(
                    "json.policy_details.publishers",
                    "airlock_digital.agent.poilcy_details.publishers",
                )?;
            }

            if event.has_value("json.policy_details.python") {
                if let Some(val) = event.get("json.policy_details.python") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.python".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.python", converted)?;
                }
            }

            if event.has_value("json.policy_details.reflection") {
                if let Some(val) = event.get("json.policy_details.reflection") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.reflection".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.reflection", converted)?;
                }
            }

            if event.has_value("json.policy_details.script_custom") {
                if let Some(val) = event.get("json.policy_details.script_custom") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.script_custom".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.script_custom",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.script_enabled") {
                if let Some(val) = event.get("json.policy_details.script_enabled") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.script_enabled".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.script_enabled",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.selfservice") {
                if let Some(val) = event.get("json.policy_details.selfservice") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.selfservice".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.selfservice",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.selfupgrade") {
                if let Some(val) = event.get("json.policy_details.selfupgrade") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.selfupgrade".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.selfupgrade",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.shellscript") {
                if let Some(val) = event.get("json.policy_details.shellscript") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.shellscript".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.shellscript",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.targetvers") {
                event.rename(
                    "json.policy_details.targetvers",
                    "airlock_digital.agent.poilcy_details.targetvers",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.policy_details.trusted_config") {
                    if let Some(val) = event.get("json.policy_details.trusted_config") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.policy_details.trusted_config".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "airlock_digital.agent.poilcy_details.trusted_config",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_policy_details_trusted_config_to_boolean_07ce1563",
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

            if event.has_value("json.policy_details.trusted_upload") {
                if let Some(val) = event.get("json.policy_details.trusted_upload") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.trusted_upload".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.trusted_upload",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.vbscript") {
                if let Some(val) = event.get("json.policy_details.vbscript") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.vbscript".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.poilcy_details.vbscript", converted)?;
                }
            }

            if event.has_value("json.policy_details.windowsinstaller") {
                if let Some(val) = event.get("json.policy_details.windowsinstaller") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.windowsinstaller".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.windowsinstaller",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policy_details.windowsscriptcomponent") {
                if let Some(val) = event.get("json.policy_details.windowsscriptcomponent") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.policy_details.windowsscriptcomponent".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "airlock_digital.agent.poilcy_details.windowsscriptcomponent",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.policyversion") {
                event.rename("json.policyversion", "airlock_digital.agent.policyversion")?;
            }

            if let Some(v) = event
                .get("airlock_digital.agent.policyversion")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.version", v)?;
            }

            if event.has_value("json.status") {
                if let Some(val) = event.get("json.status") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.status".into(),
                            message,
                        }
                    })?;
                    event.set("airlock_digital.agent.status", converted)?;
                }
            }

            // Painless script
            // Source: if (ctx.json?.status == null || ctx.json.status == '') {\n  return;\n}\nctx.airlock_digital.agent.status_value = params[(ctx.json.status).toString()];
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.json?.status == null || ctx.json.status == '') {\n  return;\n}\nctx.airlock_digital.agent.status_value = params[(ctx.json.status).toString()];"#
                ),
                cached_params!("{\"0\":\"Offline\",\"1\":\"Online\",\"3\":\"Safemode\"}"),
            )?;

            if event.has_value("json.username") {
                event.rename("json.username", "airlock_digital.agent.username")?;
            }

            if let Some(v) = event
                .get("airlock_digital.agent.username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("airlock_digital.agent.username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("airlock_digital.agent.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("airlock_digital.agent.poilcy_details.paths")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "airlock_digital.agent.poilcy_details.paths",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.name");
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("airlock_digital.agent.poilcy_details.pprocesses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "airlock_digital.agent.poilcy_details.pprocesses",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.name");
                        }
                        Ok(())
                    },
                )?;
            }

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
                event.remove("airlock_digital.agent.agentid");
                event.remove("airlock_digital.agent.domain");
                event.remove("airlock_digital.agent.groupid");
                event.remove("airlock_digital.agent.hostname");
                event.remove("airlock_digital.agent.ip");
                event.remove("airlock_digital.agent.os");
                event.remove("airlock_digital.agent.poilcy_details.name");
                event.remove("airlock_digital.agent.poilcy_details.notification_message");
                event.remove("airlock_digital.agent.policyversion");
                event.remove("airlock_digital.agent.username");
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

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
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
