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

            event.set("ecs.version", json!("8.16.0"))?;

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

            let _cond = { !event.has_value("json.events") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond =
                { event.has_value("json.id.time") && event.get_str("json.id.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.id.time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "UNIX",
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.id.time".into(),
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

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_workspace.id.time", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("json.events") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("json.id") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
                Ok(())
            })();

            if event.has_value("json.events.name") {
                event.rename("json.events.name", "event.action")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("event.action").cloned() {
                    event.set("google_workspace.event.name", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.id.applicationName") {
                event.rename("json.id.applicationName", "event.provider")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("event.provider").cloned() {
                    event.set("google_workspace.id.application_name", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.id.uniqueQualifier") {
                    if let Some(val) = event.get("json.id.uniqueQualifier") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.id.uniqueQualifier".into(),
                                message,
                            }
                        })?;
                        event.set("event.id", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("event.id").cloned() {
                    event.set("google_workspace.id.unique_qualifier", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.actor.email") {
                event.rename("json.actor.email", "source.user.email")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("source.user.email").cloned() {
                    event.set("google_workspace.actor.email", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("source.user.email").cloned() {
                    event.set("user.email", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.actor.profileId") {
                    if let Some(val) = event.get("json.actor.profileId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.actor.profileId".into(),
                                message,
                            }
                        })?;
                        event.set("source.user.id", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("source.user.id").cloned() {
                    event.set("google_workspace.actor.profile.id", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ipAddress") {
                    if let Some(val) = event.get("json.ipAddress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ipAddress".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("source.ip").cloned() {
                    event.set("google_workspace.ip_address", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.kind") {
                event.rename("json.kind", "google_workspace.kind")?;
            }

            if event.has_value("json.etag") {
                event.rename("json.etag", "google_workspace.etag")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.id.customerId") {
                    if let Some(val) = event.get("json.id.customerId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.id.customerId".into(),
                                message,
                            }
                        })?;
                        event.set("organization.id", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("organization.id").cloned() {
                    event.set("google_workspace.id.customer.id", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.actor.callerType") {
                event.rename("json.actor.callerType", "google_workspace.actor.type")?;
            }

            if event.has_value("json.actor.key") {
                event.rename("json.actor.key", "google_workspace.actor.key")?;
            }

            if event.has_value("json.ownerDomain") {
                event.rename("json.ownerDomain", "google_workspace.organization.domain")?;
            }

            if event.has_value("json.events.type") {
                event.rename("json.events.type", "google_workspace.event.type")?;
            }

            let _cond = { event.has_value("source.user.id") };
            if _cond {
                if let Some(v) = event.get("source.user.id").cloned() {
                    event.set("user.id", v)?;
                }
            }

            let _cond = {
                event.has_value("source.user.email")
                    && event.get("source.user.email").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.source.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} ctx.user.name = splitmail[0]; ctx.source.user.name = splitmail[0]; ctx.user.domain = splitmail[1]; ctx.source.user.domain = splitmail[1];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String[] splitmail = ctx.source.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} ctx.user.name = splitmail[0]; ctx.source.user.name = splitmail[0]; ctx.user.domain = splitmail[1]; ctx.source.user.domain = splitmail[1];\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("user.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("google_workspace.organization.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("google_workspace.organization.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("source.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.events.parameters")
                    && event
                        .get("json.events.parameters")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: if (ctx.google_workspace.rules == null) {\n  ctx.google_workspace.rules = new HashMap();\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"] != null) {\n    for (int j = 0;j < ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"].length; ++j ){\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"value\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"value\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiValue\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"intValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"intValue\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiIntValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiIntValue\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"boolValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"boolValue\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiBoolValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiBoolValue\"];\n      }\n    }\n  }\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"] != null) {\n    ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"].remove('parameter');\n  }\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"].length; ++j ){\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        for (int k = 0;k < ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"].length; ++k ){\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"value\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"value\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiValue\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"intValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"intValue\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiIntValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiIntValue\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"boolValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"boolValue\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiBoolValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiBoolValue\"];\n          }\n        }\n      }\n    }\n  }\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"].length; ++j ){\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j].remove('parameter');\n      }\n    }\n  }\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"intValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"intValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"boolValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"boolValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiIntValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiIntValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"];\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.google_workspace.rules == null) {\n  ctx.google_workspace.rules = new HashMap();\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"] != null) {\n    for (int j = 0;j < ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"].length; ++j ){\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"value\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"value\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiValue\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"intValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"intValue\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiIntValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiIntValue\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"boolValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"boolValue\"];\n      }\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiBoolValue\"] != null) {\n        ctx.json.events.parameters[i].messageValue[ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"][\"parameter\"][j][\"multiBoolValue\"];\n      }\n    }\n  }\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"] != null) {\n    ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"].remove('parameter');\n  }\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"].length; ++j ){\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        for (int k = 0;k < ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"].length; ++k ){\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"value\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"value\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiValue\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"intValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"intValue\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiIntValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiIntValue\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"boolValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"boolValue\"];\n          }\n          if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiBoolValue\"] != null) {\n            ctx.json.events.parameters[i].multiMessageValue[j][ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"][k][\"multiBoolValue\"];\n          }\n        }\n      }\n    }\n  }\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"] != null) {\n    for (int j = 0;j < ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"].length; ++j ){\n      if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j][\"parameter\"] != null) {\n        ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"][j].remove('parameter');\n      }\n    }\n  }\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"intValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"intValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"boolValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"boolValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiIntValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiIntValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"messageValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"] != null) {\n    ctx.google_workspace.rules[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiMessageValue\"];\n  }\n}\n"#
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.rules.actor_ip_address") {
                    if let Some(val) = event.get("google_workspace.rules.actor_ip_address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.rules.actor_ip_address".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.rules.actor_ip_address", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("google_workspace.rules.actor_ip_address")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "google_workspace.rules.actor_ip_address".into(),
                    });
                }
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

            if event.has_value("google_workspace.rules.device_id") {
                event.rename(
                    "google_workspace.rules.device_id",
                    "google_workspace.rules.device.id",
                )?;
            }

            if event.has_value("google_workspace.rules.device_type") {
                event.rename(
                    "google_workspace.rules.device_type",
                    "google_workspace.rules.device.type",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.rules.has_alert") {
                    if let Some(val) = event.get("google_workspace.rules.has_alert") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.rules.has_alert".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.rules.has_alert", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("google_workspace.rules.has_alert").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "google_workspace.rules.has_alert".into(),
                    });
                }
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

            if event.has_value("google_workspace.rules.matched_detectors") {
                event.rename(
                    "google_workspace.rules.matched_detectors",
                    "google_workspace.rules.matched.detectors",
                )?;
            }

            if event.has_value("google_workspace.rules.matched_threshold") {
                event.rename(
                    "google_workspace.rules.matched_threshold",
                    "google_workspace.rules.matched.threshold",
                )?;
            }

            if event.has_value("google_workspace.rules.matched_trigger") {
                event.rename(
                    "google_workspace.rules.matched_trigger",
                    "google_workspace.rules.matched.trigger",
                )?;
            }

            if event.has_value("google_workspace.rules.matched_templates") {
                event.rename(
                    "google_workspace.rules.matched_templates",
                    "google_workspace.rules.matched.templates",
                )?;
            }

            if event.has_value("google_workspace.rules.resource_id") {
                event.rename(
                    "google_workspace.rules.resource_id",
                    "google_workspace.rules.resource.id",
                )?;
            }

            if event.has_value("google_workspace.rules.resource_owner_email") {
                event.rename(
                    "google_workspace.rules.resource_owner_email",
                    "google_workspace.rules.resource.owner_email",
                )?;
            }

            if event.has_value("google_workspace.rules.resource_recipients") {
                event.rename(
                    "google_workspace.rules.resource_recipients",
                    "google_workspace.rules.resource.recipients",
                )?;
            }

            let _cond = { event.has_value("google_workspace.rules.resource.recipients") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def related_domains = new ArrayList(); if (ctx.related?.hosts != null) {\n  if (ctx.related.hosts instanceof List) {\n    for (host in ctx.related.hosts) {\n      related_domains.add(host)\n    }\n  } else {\n    related_domains.add(ctx.related.hosts)\n  }\n} else if (!ctx.containsKey('related') ){\n  ctx.related = new HashMap();\n} if (ctx.google_workspace.rules.resource.recipients instanceof List) {\n  List domains = ctx.google_workspace.rules.resource.recipients;\n  for (domain in domains) {\n    if (domain.contains('@')) {\n      related_domains.add(domain.splitOnToken('@')[1])\n    }\n  }\n} else {\n  if (ctx.google_workspace.rules.resource.recipients.contains('@')) {\n    related_domains.add(ctx.google_workspace.rules.resource.recipients.splitOnToken('@')[1])\n  }\n} related_domains = related_domains.stream().distinct().collect(Collectors.toList()); ctx.related.hosts = related_domains;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def related_domains = new ArrayList(); if (ctx.related?.hosts != null) {\n  if (ctx.related.hosts instanceof List) {\n    for (host in ctx.related.hosts) {\n      related_domains.add(host)\n    }\n  } else {\n    related_domains.add(ctx.related.hosts)\n  }\n} else if (!ctx.containsKey('related') ){\n  ctx.related = new HashMap();\n} if (ctx.google_workspace.rules.resource.recipients instanceof List) {\n  List domains = ctx.google_workspace.rules.resource.recipients;\n  for (domain in domains) {\n    if (domain.contains('@')) {\n      related_domains.add(domain.splitOnToken('@')[1])\n    }\n  }\n} else {\n  if (ctx.google_workspace.rules.resource.recipients.contains('@')) {\n    related_domains.add(ctx.google_workspace.rules.resource.recipients.splitOnToken('@')[1])\n  }\n} related_domains = related_domains.stream().distinct().collect(Collectors.toList()); ctx.related.hosts = related_domains;\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.rules.resource_recipients_omitted_count") {
                    if let Some(val) =
                        event.get("google_workspace.rules.resource_recipients_omitted_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.rules.resource_recipients_omitted_count"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "google_workspace.rules.resource.recipients_omitted_count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("google_workspace.rules.resource_recipients_omitted_count")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "google_workspace.rules.resource_recipients_omitted_count".into(),
                    });
                }
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

            if event.has_value("google_workspace.rules.resource_title") {
                event.rename(
                    "google_workspace.rules.resource_title",
                    "google_workspace.rules.resource.title",
                )?;
            }

            if event.has_value("google_workspace.rules.resource_type") {
                event.rename(
                    "google_workspace.rules.resource_type",
                    "google_workspace.rules.resource.type",
                )?;
            }

            if event.has_value("google_workspace.rules.rule_name") {
                event.rename("google_workspace.rules.rule_name", "rule.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("rule.name").cloned() {
                    event.set("google_workspace.rules.name", v)?;
                }
                Ok(())
            })();

            if event.has_value("google_workspace.rules.rule_resource_name") {
                event.rename(
                    "google_workspace.rules.rule_resource_name",
                    "google_workspace.rules.resource.name",
                )?;
            }

            if event.has_value("google_workspace.rules.rule_type") {
                event.rename(
                    "google_workspace.rules.rule_type",
                    "google_workspace.rules.type",
                )?;
            }

            if event.has_value("google_workspace.rules.space_id") {
                event.rename(
                    "google_workspace.rules.space_id",
                    "google_workspace.rules.space.id",
                )?;
            }

            if event.has_value("google_workspace.rules.space_type") {
                event.rename(
                    "google_workspace.rules.space_type",
                    "google_workspace.rules.space.type",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.rules.has_content_match") {
                    if let Some(val) = event.get("google_workspace.rules.has_content_match") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.rules.has_content_match".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.rules.has_content_match", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("google_workspace.rules.has_content_match")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "google_workspace.rules.has_content_match".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.rules.rule_id") {
                    if let Some(val) = event.get("google_workspace.rules.rule_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.rules.rule_id".into(),
                                message,
                            }
                        })?;
                        event.set("rule.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("google_workspace.rules.rule_id").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "google_workspace.rules.rule_id".into(),
                    });
                }
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("rule.id").cloned() {
                    event.set("google_workspace.rules.id", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("google_workspace.rules.rule_update_time_usec")
                    && event.get_str("google_workspace.rules.rule_update_time_usec") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_workspace.rules.rule_update_time_usec")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "UNIX",
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("google_workspace.rules.update_time_usec", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_workspace.rules.rule_update_time_usec".into(),
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = { event.has_value("google_workspace.rules.actor_ip_address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("google_workspace.rules.actor_ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("event.action")
                    && ["rule_match", "rule_trigger", "action_complete"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && ["rule_match", "rule_trigger", "action_complete"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && ["label_applied", "label_field_value_changed"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("event.action") == Some("label_removed") };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = { event.get_bool("google_workspace.rules.has_alert") == Some(true) };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            event.remove("json");
            event.remove("google_workspace.rules.resource_recipients_omitted_count");
            event.remove("google_workspace.rules.rule_id");
            event.remove("google_workspace.rules.rule_update_time_usec");

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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("google_workspace.ip_address");
                    event.remove("google_workspace.event.name");
                    event.remove("google_workspace.id.unique_qualifier");
                    event.remove("google_workspace.id.application_name");
                    event.remove("google_workspace.id.customer.id");
                    event.remove("google_workspace.actor.profile.id");
                    event.remove("google_workspace.actor.email");
                    event.remove("google_workspace.rules.id");
                    event.remove("google_workspace.rules.name");
                    event.remove("google_workspace.id.time");
                    Ok(())
                })();
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
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
