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
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.ExecutionMetadata.ExecutionStart".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
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
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.ExecutionMetadata.SearchWindowStart"
                                    .into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
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
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.ExecutionMetadata.SearchWindowEnd".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("crowdstrike.event.ExecutionMetadata.ExecutionDuration") {
                if let Some(val) =
                    event.get("crowdstrike.event.ExecutionMetadata.ExecutionDuration")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "crowdstrike.event.ExecutionMetadata.ExecutionDuration".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "crowdstrike.event.ExecutionMetadata.ExecutionDuration",
                        converted,
                    )?;
                }
            }

            if event.has_value("crowdstrike.event.ExecutionMetadata.ResultCount") {
                if let Some(val) = event.get("crowdstrike.event.ExecutionMetadata.ResultCount") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "crowdstrike.event.ExecutionMetadata.ResultCount".into(),
                            message,
                        }
                    })?;
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
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("user.domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "user.id".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
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

            if event.has_value("crowdstrike.event.Status") {
                if let Some(val) = event.get("crowdstrike.event.Status") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "crowdstrike.event.Status".into(),
                            message,
                        }
                    })?;
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
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
