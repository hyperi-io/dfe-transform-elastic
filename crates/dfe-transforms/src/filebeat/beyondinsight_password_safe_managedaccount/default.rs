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
                "beyondinsight_password_safe.managedaccount",
            )?;

            // Painless script
            // Source: ctx.beyondinsight_password_safe.managedaccount.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.beyondinsight_password_safe.managedaccount.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n"#
                ),
            )?;

            // Painless script
            // Source: for (field in params.numeric_ids) {\n  def value = ctx.beyondinsight_password_safe.managedaccount[field];\n  if (value instanceof Number) {\n    ctx.beyondinsight_password_safe.managedaccount[field] =\n      Integer.toString(value.intValue());\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"for (field in params.numeric_ids) {\n  def value = ctx.beyondinsight_password_safe.managedaccount[field];\n  if (value instanceof Number) {\n    ctx.beyondinsight_password_safe.managedaccount[field] =\n      Integer.toString(value.intValue());\n  }\n}\n"#
                ),
                cached_params!(
                    "{\"numeric_ids\":[\"AccountId\",\"ApplicationID\",\"ChangeState\",\"PlatformID\",\"SystemId\"]}"
                ),
            )?;

            // Painless script
            // Source: Map renamedFields = [:];\nfor (entry in ctx.beyondinsight_password_safe.managedaccount.entrySet()) {\n  def originalKey = entry.getKey();\n  def snakeKey = params.field_mappings[originalKey];\n  if (snakeKey != null) {\n    renamedFields[snakeKey] = entry.getValue();\n  } else {\n    renamedFields[originalKey] = entry.getValue();\n  }\n}\nctx.beyondinsight_password_safe.managedaccount = renamedFields;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"Map renamedFields = [:];\nfor (entry in ctx.beyondinsight_password_safe.managedaccount.entrySet()) {\n  def originalKey = entry.getKey();\n  def snakeKey = params.field_mappings[originalKey];\n  if (snakeKey != null) {\n    renamedFields[snakeKey] = entry.getValue();\n  } else {\n    renamedFields[originalKey] = entry.getValue();\n  }\n}\nctx.beyondinsight_password_safe.managedaccount = renamedFields;\n"#
                ),
                cached_params!(
                    "{\"field_mappings\":{\"AccountDescription\":\"account_description\",\"AccountId\":\"account_id\",\"AccountName\":\"account_name\",\"ApplicationDisplayName\":\"application_display_name\",\"ApplicationID\":\"application_id\",\"ChangeState\":\"change_state\",\"DefaultReleaseDuration\":\"default_release_duration\",\"DomainName\":\"domain_name\",\"InstanceName\":\"instance_name\",\"IsChanging\":\"is_changing\",\"IsISAAccess\":\"is_isa_access\",\"LastChangeDate\":\"last_change_date\",\"MaximumReleaseDuration\":\"maximum_release_duration\",\"NextChangeDate\":\"next_change_date\",\"PlatformID\":\"platform_id\",\"PreferredNodeID\":\"preferred_node_id\",\"SystemId\":\"system_id\",\"SystemName\":\"system_name\",\"UserPrincipalName\":\"user_principal_name\"}}"
                ),
            )?;

            let _cond =
                { event.has_value("beyondinsight_password_safe.managedaccount.last_change_date") };
            if _cond {
                if let Some(date_str) = event
                    .get_as_string("beyondinsight_password_safe.managedaccount.last_change_date")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondinsight_password_safe.managedaccount.last_change_date",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondinsight_password_safe.managedaccount.last_change_date"
                                    .into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.managedaccount.last_change_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond =
                { event.has_value("beyondinsight_password_safe.managedaccount.next_change_date") };
            if _cond {
                if let Some(date_str) = event
                    .get_as_string("beyondinsight_password_safe.managedaccount.next_change_date")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondinsight_password_safe.managedaccount.next_change_date",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondinsight_password_safe.managedaccount.next_change_date"
                                    .into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("iam"))?;

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("beyondinsight_password_safe.managedaccount.system_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.managedaccount.system_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.managedaccount.domain_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.managedaccount.account_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.managedaccount.account_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("beyondinsight_password_safe.managedaccount.user_principal_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond =
                { event.has_value("beyondinsight_password_safe.managedaccount.change_state") };
            if _cond {
                // Painless script
                // Source: def description = params.descriptions.get(ctx.beyondinsight_password_safe.managedaccount.change_state);\nif (description != null) {\n  ctx.beyondinsight_password_safe.managedaccount.change_state = description;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def description = params.descriptions.get(ctx.beyondinsight_password_safe.managedaccount.change_state);\nif (description != null) {\n  ctx.beyondinsight_password_safe.managedaccount.change_state = description;\n}\n"#
                    ),
                    cached_params!(
                        "{\"descriptions\":{\"0\":\"idle\",\"1\":\"changing\",\"2\":\"queued\"}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.managedaccount.system_id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedaccount.system_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("beyondinsight_password_safe.managedaccount.system_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedaccount.system_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("beyondinsight_password_safe.managedaccount.account_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedaccount.account_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("beyondinsight_password_safe.managedaccount.account_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedaccount.account_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("beyondinsight_password_safe.managedaccount.user_principal_name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondinsight_password_safe.managedaccount.user_principal_name")
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
