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
            event.set("ecs.version", json!("9.1.0"))?;

            let _cond = { event.has_value("error.message") && !event.has_value("event.original") };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "event.original".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            parse_json_field(
                event,
                "event.original",
                "beyondinsight_password_safe.session",
            )?;

            // Painless script
            // Source: ctx.beyondinsight_password_safe.session.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.beyondinsight_password_safe.session.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n"#
                ),
            )?;

            // Painless script
            // Source: for (field in params.numeric_ids) {\n  def value = ctx.beyondinsight_password_safe.session[field];\n  if (value instanceof Number) {\n    ctx.beyondinsight_password_safe.session[field] =\n      Integer.toString(value.intValue());\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"for (field in params.numeric_ids) {\n  def value = ctx.beyondinsight_password_safe.session[field];\n  if (value instanceof Number) {\n    ctx.beyondinsight_password_safe.session[field] =\n      Integer.toString(value.intValue());\n  }\n}\n"#
                ),
                cached_params!(
                    "{\"numeric_ids\":[\"ApplicationID\",\"ArchiveStatus\",\"ManagedAccountID\",\"ManagedSystemID\",\"Protocol\",\"RequestID\",\"SessionID\",\"SessionType\",\"Status\",\"UserID\"]}"
                ),
            )?;

            // Painless script
            // Source: Map renamedFields = [:];\nfor (entry in ctx.beyondinsight_password_safe.session.entrySet()) {\n  def originalKey = entry.getKey();\n  def snakeKey = params.field_mappings[originalKey];\n  if (snakeKey != null) {\n    renamedFields[snakeKey] = entry.getValue();\n  } else {\n    renamedFields[originalKey] = entry.getValue();\n  }\n}\nctx.beyondinsight_password_safe.session = renamedFields;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"Map renamedFields = [:];\nfor (entry in ctx.beyondinsight_password_safe.session.entrySet()) {\n  def originalKey = entry.getKey();\n  def snakeKey = params.field_mappings[originalKey];\n  if (snakeKey != null) {\n    renamedFields[snakeKey] = entry.getValue();\n  } else {\n    renamedFields[originalKey] = entry.getValue();\n  }\n}\nctx.beyondinsight_password_safe.session = renamedFields;\n"#
                ),
                cached_params!(
                    "{\"field_mappings\":{\"ApplicationID\":\"application_id\",\"ArchiveStatus\":\"archive_status\",\"AssetName\":\"asset_name\",\"Duration\":\"duration\",\"EndTime\":\"end_time\",\"ManagedAccountID\":\"managed_account_id\",\"ManagedAccountName\":\"managed_account_name\",\"ManagedSystemID\":\"managed_system_id\",\"NodeID\":\"node_id\",\"Protocol\":\"protocol\",\"RecordKey\":\"record_key\",\"RequestID\":\"request_id\",\"SessionID\":\"session_id\",\"SessionType\":\"session_type\",\"StartTime\":\"start_time\",\"Status\":\"status\",\"Token\":\"token\",\"UserID\":\"user_id\"}}"
                ),
            )?;

            let _cond = { event.has_value("beyondinsight_password_safe.session.start_time") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("beyondinsight_password_safe.session.start_time")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondinsight_password_safe.session.start_time", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondinsight_password_safe.session.start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.end_time") };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("beyondinsight_password_safe.session.end_time")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondinsight_password_safe.session.end_time", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondinsight_password_safe.session.end_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.start_time") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.session.start_time")
                    .cloned()
                {
                    event.set("event.start", v)?;
                }
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.end_time") };
            if _cond {
                if let Some(v) = event
                    .get("beyondinsight_password_safe.session.end_time")
                    .cloned()
                {
                    event.set("event.end", v)?;
                }
            }

            let _cond = { event.has_value("event.end") };
            if _cond {
                if let Some(v) = event.get("event.end").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { !event.has_value("event.end") && event.has_value("event.start") };
            if _cond {
                if let Some(v) = event.get("event.start").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("session"))?;

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("beyondinsight_password_safe.session.duration")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.duration", v)?;
            }

            let _cond = { event.has_value("event.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = ctx.event.duration * 1000000000L;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.event.duration = ctx.event.duration * 1000000000L;\n"#),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.status") };
            if _cond {
                // Painless script
                // Source: def description = params.descriptions.get(ctx.beyondinsight_password_safe.session.status);\nif (description != null) {\n  ctx.beyondinsight_password_safe.session.status = description;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def description = params.descriptions.get(ctx.beyondinsight_password_safe.session.status);\nif (description != null) {\n  ctx.beyondinsight_password_safe.session.status = description;\n}\n"#
                    ),
                    cached_params!(
                        "{\"descriptions\":{\"0\":\"not_started\",\"1\":\"in_progress\",\"2\":\"completed\",\"5\":\"locked\",\"7\":\"terminated\",\"8\":\"logged_off\",\"9\":\"disconnected\"}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.archive_status") };
            if _cond {
                // Painless script
                // Source: def description = params.descriptions.get(ctx.beyondinsight_password_safe.session.archive_status);\nif (description != null) {\n  ctx.beyondinsight_password_safe.session.archive_status = description;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def description = params.descriptions.get(ctx.beyondinsight_password_safe.session.archive_status);\nif (description != null) {\n  ctx.beyondinsight_password_safe.session.archive_status = description;\n}\n"#
                    ),
                    cached_params!(
                        "{\"descriptions\":{\"0\":\"not_archived\",\"1\":\"archived\",\"2\":\"restoring\",\"3\":\"archiving\",\"4\":\"session_not_found\",\"5\":\"repository_offline\",\"6\":\"unknown\"}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.protocol") };
            if _cond {
                // Painless script
                // Source: def description = params.descriptions.get(ctx.beyondinsight_password_safe.session.protocol);\nif (description != null) {\n  ctx.beyondinsight_password_safe.session.protocol = description;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def description = params.descriptions.get(ctx.beyondinsight_password_safe.session.protocol);\nif (description != null) {\n  ctx.beyondinsight_password_safe.session.protocol = description;\n}\n"#
                    ),
                    cached_params!("{\"descriptions\":{\"0\":\"rdp\",\"1\":\"ssh\"}}"),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.session_type") };
            if _cond {
                // Painless script
                // Source: def description = params.descriptions.get(ctx.beyondinsight_password_safe.session.session_type);\nif (description != null) {\n  ctx.beyondinsight_password_safe.session.session_type = description;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def description = params.descriptions.get(ctx.beyondinsight_password_safe.session.session_type);\nif (description != null) {\n  ctx.beyondinsight_password_safe.session.session_type = description;\n}\n"#
                    ),
                    cached_params!(
                        "{\"descriptions\":{\"1\":\"regular\",\"2\":\"isa\",\"3\":\"admin\"}}"
                    ),
                )?;
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.session.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.protocol", v)?;
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.session.session_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.session.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.session.managed_account_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.asset_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("beyondinsight_password_safe.session.asset_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("beyondinsight_password_safe.session.managed_account_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondinsight_password_safe.session.managed_account_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondinsight_password_safe.session.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
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
