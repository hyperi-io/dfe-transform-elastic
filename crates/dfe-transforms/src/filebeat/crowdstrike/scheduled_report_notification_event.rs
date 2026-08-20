// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `scheduled_report_notification_event` pipeline.
pub struct ScheduledReportNotificationEvent;

impl Transform for ScheduledReportNotificationEvent {
    fn name(&self) -> &str {
        "scheduled_report_notification_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("crowdstrike.event.ExecutionMetadata.ExecutionStart")
                    && event
                        .get_as_string("crowdstrike.event.ExecutionMetadata.ExecutionStart")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.event.ExecutionMetadata.ExecutionStart")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ExecutionMetadata.SearchWindowStart")
                    && event
                        .get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowStart")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowStart")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ExecutionMetadata.SearchWindowEnd")
                    && event
                        .get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowEnd")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowEnd")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            if event.has("crowdstrike.event.ExecutionMetadata.ExecutionDuration") {
                if let Some(val) =
                    event.get("crowdstrike.event.ExecutionMetadata.ExecutionDuration")
                {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path:
                                            "crowdstrike.event.ExecutionMetadata.ExecutionDuration"
                                                .into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| {
                                    TransformError::ParseError {
                                        path:
                                            "crowdstrike.event.ExecutionMetadata.ExecutionDuration"
                                                .into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.ExecutionMetadata.ExecutionDuration"
                                    .into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set(
                        "crowdstrike.event.ExecutionMetadata.ExecutionDuration",
                        converted,
                    )?;
                }
            }

            if event.has("crowdstrike.event.ExecutionMetadata.ResultCount") {
                if let Some(val) = event.get("crowdstrike.event.ExecutionMetadata.ResultCount") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "crowdstrike.event.ExecutionMetadata.ResultCount"
                                            .into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "crowdstrike.event.ExecutionMetadata.ResultCount".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.ExecutionMetadata.ResultCount".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("crowdstrike.event.ExecutionMetadata.ResultCount", converted)?;
                }
            }

            if event.has("crowdstrike.event.UserID") {
                event.rename("crowdstrike.event.UserID", "user.id")?;
            }

            let _cond = {
                event.has_value("user.id")
                    && event.get("user.id").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(input) = event.get_string("user.id") {
                    let mut remaining: &str = &input;
                    if let Some(pos) = remaining.find("@") {
                        event.set("user.name", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("@") {
                        remaining = rest;
                    }
                    event.set("user.domain", remaining)?;
                }
            }

            let _cond = {
                event.has_value("user.id")
                    && event
                        .get_str("user.id")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("user.id").cloned() {
                    event.set("user.email", v)?;
                }
            }

            if event.has("crowdstrike.event.Status") {
                if let Some(val) = event.get("crowdstrike.event.Status") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("crowdstrike.event.Status", converted)?;
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
