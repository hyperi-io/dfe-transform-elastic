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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "b5f142dbefe7db23107c99621e3e070285bce46555eaa53813207d573021ea58",
                    r#"{"params":"SentinelRemoval","known":["SentinelRemovalLiteral"]}"#,
                    r#"ctx.beyondinsight_password_safe.managedaccount.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n"#
                ),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "a79a8271188cd758a789b1b2c579b10a48f5f7d851a600a8790db179e16724d5",
                    r#"{"params":{"StringifyNamedFields":{"container":"beyondinsight_password_safe.managedaccount","table":"numeric_ids"}},"known":[]}"#
                ),
                cached_params!(
                    "{\"numeric_ids\":[\"AccountId\",\"ApplicationID\",\"ChangeState\",\"PlatformID\",\"SystemId\"]}"
                ),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "a9368b763cce320a3a55d2bb2b00182efd52f51297f360afd7c91db83b6eedf7",
                    r#"{"params":{"SelectRenameKeys":{"subject":"beyondinsight_password_safe.managedaccount","table":"field_mappings","keep_unmapped":true,"wrap_in_list":false}},"known":[]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "c49d0ff727011128b7e9fc32d09cb7748a93e1f7989d4275abf02bf69407ede7",
                        r#"{"params":{"MemberLookup":{"key":"beyondinsight_password_safe.managedaccount.change_state","table":"descriptions","fold":"None","target":"beyondinsight_password_safe.managedaccount.change_state","wrap":false,"miss":"Keep","literals":{"statements":[],"whole":true}}},"known":[]}"#
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
