// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `scheduled_report_notification_event` pipeline.
pub struct ScheduledReportNotificationEvent;

impl Transform for ScheduledReportNotificationEvent {
    fn name(&self) -> &str {
        "scheduled_report_notification_event"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("event"))?;

        let _cond = {
            event.has("crowdstrike.event.ExecutionMetadata.ExecutionStart")
                && event
                    .get_as_string("crowdstrike.event.ExecutionMetadata.ExecutionStart")
                    .is_some_and(|s| s.len() >= 12)
        };
        if _cond {
            if let Some(date_str) =
                event.get_as_string("crowdstrike.event.ExecutionMetadata.ExecutionStart")
            {
                // Try UNIX_MS timestamp (skip epoch 0)
                if let Ok(ms) = date_str.parse::<i64>() {
                    if ms > 0 {
                        if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
                            event.set(
                                "@timestamp",
                                dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                            )?;
                        }
                    }
                }
            }
        }

        let _cond = {
            event.has("crowdstrike.event.ExecutionMetadata.SearchWindowStart")
                && event
                    .get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowStart")
                    .is_some_and(|s| s.len() >= 12)
        };
        if _cond {
            if let Some(date_str) =
                event.get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowStart")
            {
                // Try UNIX_MS timestamp (skip epoch 0)
                if let Ok(ms) = date_str.parse::<i64>() {
                    if ms > 0 {
                        if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
                            event.set(
                                "@timestamp",
                                dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                            )?;
                        }
                    }
                }
            }
        }

        let _cond = {
            event.has("crowdstrike.event.ExecutionMetadata.SearchWindowEnd")
                && event
                    .get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowEnd")
                    .is_some_and(|s| s.len() >= 12)
        };
        if _cond {
            if let Some(date_str) =
                event.get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowEnd")
            {
                // Try UNIX_MS timestamp (skip epoch 0)
                if let Ok(ms) = date_str.parse::<i64>() {
                    if ms > 0 {
                        if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
                            event.set(
                                "@timestamp",
                                dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                            )?;
                        }
                    }
                }
            }
        }

        if event.has("crowdstrike.event.ExecutionMetadata.ExecutionDuration") {
            if let Some(val) = event.get("crowdstrike.event.ExecutionMetadata.ExecutionDuration") {
                let converted = match val {
                    Value::String(s) => {
                        let s = s.trim();
                        if let Some(hex) = s.strip_prefix("0x") {
                            json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.ExecutionMetadata.ExecutionDuration"
                                        .into(),
                                    message: format!("cannot convert '{}' to integer", s),
                                }
                            })?)
                        } else {
                            json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                path:
                                    "crowdstrike.event.ExecutionMetadata.ExecutionDuration".into(),
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
                            path: "crowdstrike.event.ExecutionMetadata.ExecutionDuration".into(),
                            message: "cannot convert to integer".into(),
                        }
                        .into());
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
                                    path: "crowdstrike.event.ExecutionMetadata.ResultCount".into(),
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
                        }
                        .into());
                    }
                };
                event.set("crowdstrike.event.ExecutionMetadata.ResultCount", converted)?;
            }
        }

        if event.has("crowdstrike.event.UserID") {
            event.rename("crowdstrike.event.UserID", "user.id")?;
        }

        // user.id is an email address
        let _cond = event.get_str("user.id").is_some_and(|i| i.contains('@'));
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
    }
}
