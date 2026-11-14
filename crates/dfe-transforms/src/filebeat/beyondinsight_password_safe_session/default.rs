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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "799451f1c7dfe949f31e2baa6bbf83e9946b01a594bf38d26342c09d9587675d",
                    r#"{"params":"SentinelRemoval","known":["SentinelRemovalLiteral"]}"#,
                    r#"ctx.beyondinsight_password_safe.session.entrySet().removeIf(entry ->\n  entry.getValue() == null ||\n  (entry.getValue() instanceof String && entry.getValue().isEmpty())\n);\n"#
                ),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "c54e4d7ce65c64053f2b91f0884ea9aca77dc346cdfb994ace33bd692ef414ec",
                    r#"{"params":{"StringifyNamedFields":{"container":"beyondinsight_password_safe.session","table":"numeric_ids"}},"known":[]}"#
                ),
                cached_params!(
                    "{\"numeric_ids\":[\"ApplicationID\",\"ArchiveStatus\",\"ManagedAccountID\",\"ManagedSystemID\",\"Protocol\",\"RequestID\",\"SessionID\",\"SessionType\",\"Status\",\"UserID\"]}"
                ),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "fb3f09dcd181afdb5638c1440ead20c75333b4c766f0cada45d7e7ab05eb9d45",
                    r#"{"params":{"SelectRenameKeys":{"subject":"beyondinsight_password_safe.session","table":"field_mappings","keep_unmapped":true,"wrap_in_list":false}},"known":[]}"#
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
                // The pipeline's script, emitted as the runtime's equivalent
                scale_field(
                    event,
                    &ScaleField::new("event.duration", "event.duration", Factor::Long(1000000000)),
                );
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.status") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "139c6476f40a0b64dc0a953eecaa748de821a4ac973483fabef9a4c65dafcbf4",
                        r#"{"params":{"MemberLookup":{"key":"beyondinsight_password_safe.session.status","table":"descriptions","fold":"None","target":"beyondinsight_password_safe.session.status","wrap":false,"miss":"Keep","literals":{"statements":[],"whole":true}}},"known":[]}"#
                    ),
                    cached_params!(
                        "{\"descriptions\":{\"0\":\"not_started\",\"1\":\"in_progress\",\"2\":\"completed\",\"5\":\"locked\",\"7\":\"terminated\",\"8\":\"logged_off\",\"9\":\"disconnected\"}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.archive_status") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "86534b64ab3a570f06540406835591253d0e87b79a2615ff5fdb636f10ce8652",
                        r#"{"params":{"MemberLookup":{"key":"beyondinsight_password_safe.session.archive_status","table":"descriptions","fold":"None","target":"beyondinsight_password_safe.session.archive_status","wrap":false,"miss":"Keep","literals":{"statements":[],"whole":true}}},"known":[]}"#
                    ),
                    cached_params!(
                        "{\"descriptions\":{\"0\":\"not_archived\",\"1\":\"archived\",\"2\":\"restoring\",\"3\":\"archiving\",\"4\":\"session_not_found\",\"5\":\"repository_offline\",\"6\":\"unknown\"}}"
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.protocol") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "0fccd0c2bd550659788710df87a4329058dbb1a09081db82d1bc3afc7205a1ca",
                        r#"{"params":{"MemberLookup":{"key":"beyondinsight_password_safe.session.protocol","table":"descriptions","fold":"None","target":"beyondinsight_password_safe.session.protocol","wrap":false,"miss":"Keep","literals":{"statements":[],"whole":true}}},"known":[]}"#
                    ),
                    cached_params!("{\"descriptions\":{\"0\":\"rdp\",\"1\":\"ssh\"}}"),
                )?;
            }

            let _cond = { event.has_value("beyondinsight_password_safe.session.session_type") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "d80da5d0d71cd0ab903413642d494f3976773000e2ba6f60dd7440c15cc363b4",
                        r#"{"params":{"MemberLookup":{"key":"beyondinsight_password_safe.session.session_type","table":"descriptions","fold":"None","target":"beyondinsight_password_safe.session.session_type","wrap":false,"miss":"Keep","literals":{"statements":[],"whole":true}}},"known":[]}"#
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
