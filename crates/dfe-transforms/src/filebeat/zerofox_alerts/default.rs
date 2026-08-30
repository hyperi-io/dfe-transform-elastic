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

            parse_json_field(event, "event.original", "zerofox")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "zerofox.metadata", "zerofox.metadata")?;
                Ok(())
            })();

            let _cond = {
                event.has_value("zerofox.content_created_at")
                    && event.get_str("zerofox.content_created_at") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("zerofox.content_created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "zerofox.content_created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("event.created") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("zerofox.content_created_at");
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("zerofox.timestamp")
                    && event.get_str("zerofox.timestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("zerofox.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "zerofox.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("zerofox.id") && event.get_str("zerofox.id") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("zerofox.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "zerofox.id".into(),
                                message,
                            }
                        })?;
                        event.set("event.id", converted)?;
                    }
                    Ok(())
                })();
            }

            event.set("event.kind", json!("alert"))?;

            if event.has_value("zerofox.severity") {
                if let Some(val) = event.get("zerofox.severity") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "zerofox.severity".into(),
                            message,
                        }
                    })?;
                    event.set("event.severity", converted)?;
                }
            }

            let _cond = {
                event.has_value("zerofox.offending_content_url")
                    && event.get_str("zerofox.offending_content_url") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("zerofox.offending_content_url") {
                        event.rename("zerofox.offending_content_url", "event.url")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("zerofox.rule_id") && event.get_str("zerofox.rule_id") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("zerofox.rule_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "zerofox.rule_id".into(),
                                message,
                            }
                        })?;
                        event.set("rule.id", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("zerofox.rule_name")
                    && event.get_str("zerofox.rule_name") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("zerofox.rule_name") {
                        event.rename("zerofox.rule_name", "rule.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("zerofox.rule_group_id")
                    && event.get_str("zerofox.rule_group_id") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("zerofox.rule_group_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "zerofox.rule_group_id".into(),
                                message,
                            }
                        })?;
                        event.set("rule.ruleset", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("zerofox.alert_type")
                    && event.get_str("zerofox.alert_type") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("zerofox.alert_type") {
                        event.rename("zerofox.alert_type", "rule.category")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("zerofox.assignee") && event.get_str("zerofox.assignee") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("zerofox.assignee") {
                        event.rename("zerofox.assignee", "user.name")?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("zerofox.assignee") && event.get_str("zerofox.assignee") != Some("")
            };
            if _cond {
                event.append_unique("user.roles", json!("assignee"))?;
            }

            let _cond = {
                event.has_value("zerofox.network") && event.get_str("zerofox.network") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("zerofox.network") {
                        event.rename("zerofox.network", "network.name")?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("zerofox.entity.id") {
                    if let Some(val) = event.get("zerofox.entity.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "zerofox.entity.id".into(),
                                message,
                            }
                        })?;
                        event.set("zerofox.entity.id", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("zerofox.entity.entity_group.id") {
                    if let Some(val) = event.get("zerofox.entity.entity_group.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "zerofox.entity.entity_group.id".into(),
                                message,
                            }
                        })?;
                        event.set("zerofox.entity.entity_group.id", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("zerofox.entity_term.id") {
                    if let Some(val) = event.get("zerofox.entity_term.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "zerofox.entity_term.id".into(),
                                message,
                            }
                        })?;
                        event.set("zerofox.entity_term.id", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("zerofox.perpetrator.id") {
                    if let Some(val) = event.get("zerofox.perpetrator.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "zerofox.perpetrator.id".into(),
                                message,
                            }
                        })?;
                        event.set("zerofox.perpetrator.id", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("zerofox.last_modified")
                    && event.get_str("zerofox.last_modified") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("zerofox.last_modified") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("zerofox.last_modified", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "zerofox.last_modified".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("zerofox.perpetrator.timestamp")
                    && event.get_str("zerofox.perpetrator.timestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("zerofox.perpetrator.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("zerofox.perpetrator.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "zerofox.perpetrator.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("zerofox.entity.labels") && event.get("zerofox.entity.labels").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                foreach_array(event, "zerofox.entity.labels", |event| {
                    if let Some(val) = event.get("_ingest._value.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "_ingest._value.id".into(),
                                message,
                            }
                        })?;
                        event.set("_ingest._value.id", converted)?;
                    }
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_temp");
                event.remove("zerofox.id");
                event.remove("zerofox.severity");
                event.remove("zerofox.entered_by");
                event.remove("zerofox.asset");
                event.remove("zerofox.rule_id");
                event.remove("zerofox.rule_group_id");
                event.remove("zerofox.asset_term");
                event.remove("zerofox.business_network");
                event.remove("zerofox.entity_email_receiver_id");
                event.remove("zerofox.timestamp");
                event.remove("zerofox.logs");
                Ok(())
            })();

            let _cond = { event.has_value("zerofox.perpetrator") };
            if _cond {
                // Painless script
                // Source: ctx.zerofox.perpetrator?.entrySet().removeIf(entry -> entry.getValue() == null || entry.getValue().equals(\"\") || (entry.getValue() instanceof List && entry.getValue().length == 0) || (entry.getValue() instanceof Map && entry.getValue().size() == 0));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.zerofox.perpetrator?.entrySet().removeIf(entry -> entry.getValue() == null || entry.getValue().equals(\"\") || (entry.getValue() instanceof List && entry.getValue().length == 0) || (entry.getValue() instanceof Map && entry.getValue().size() == 0));"#
                    ),
                )?;
            }

            let _cond = { event.has_value("zerofox.metadata") };
            if _cond {
                // Painless script
                // Source: ctx?.zerofox?.metadata?.entrySet().removeIf(entry -> entry.getValue() == null || entry.getValue().equals(\"\") || (entry.getValue() instanceof List && entry.getValue().length == 0) || (entry.getValue() instanceof Map && entry.getValue().size() == 0));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx?.zerofox?.metadata?.entrySet().removeIf(entry -> entry.getValue() == null || entry.getValue().equals(\"\") || (entry.getValue() instanceof List && entry.getValue().length == 0) || (entry.getValue() instanceof Map && entry.getValue().size() == 0));"#
                    ),
                )?;
            }

            let _cond = { event.has_value("zerofox") };
            if _cond {
                // Painless script
                // Source: ctx?.zerofox?.entrySet().removeIf(entry -> entry.getValue() == null || entry.getValue().equals(\"\") || (entry.getValue() instanceof List && entry.getValue().length == 0) || (entry.getValue() instanceof Map && entry.getValue().size() == 0));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx?.zerofox?.entrySet().removeIf(entry -> entry.getValue() == null || entry.getValue().equals(\"\") || (entry.getValue() instanceof List && entry.getValue().length == 0) || (entry.getValue() instanceof Map && entry.getValue().size() == 0));"#
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
