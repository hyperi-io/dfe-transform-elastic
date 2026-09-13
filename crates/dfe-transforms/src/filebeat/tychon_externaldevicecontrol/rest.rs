// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `rest` pipeline.
pub struct Rest;

impl Transform for Rest {
    fn name(&self) -> &str {
        "rest"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            if event.has_value("tychon.log.event_data.device_id") {
                event.rename("tychon.log.event_data.device_id", "tychon.log.user_data.device_id")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("tychon.winlog.time_created") {
                match parse_date_out(&date_str, &["epoch_second"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "tychon.winlog.time_created".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })();

        let _cond = { !event.has_value("tychon.log.event_data.device_description") };
        if _cond {
        event.set("tychon.log.event_data.device_description", json!("Unknown"))?;
        }

        if event.has_value("tychon.policy.whitelist.previous_value") {
            if let Some(s) = event.get_string("tychon.policy.whitelist.previous_value") {
                let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                while parts.last().and_then(Value::as_str) == Some("") {
                    parts.pop();
                }
                event.set("tychon.policy.whitelist.previous_value", Value::Array(parts))?;
            }
        }

        if event.has_value("tychon.policy.whitelist.current_value") {
            if let Some(s) = event.get_string("tychon.policy.whitelist.current_value") {
                let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                while parts.last().and_then(Value::as_str) == Some("") {
                    parts.pop();
                }
                event.set("tychon.policy.whitelist.current_value", Value::Array(parts))?;
            }
        }

        if event.has_value("tychon.log.event_id") {
            if let Some(val) = event.get("tychon.log.event_id") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.log.event_id".into(),
                        message,
                    })?;
                event.set("tychon.log.event_id", converted)?;
            }
        }

        if event.has_value("tychon.log.record_id") {
            if let Some(val) = event.get("tychon.log.record_id") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.log.record_id".into(),
                        message,
                    })?;
                event.set("tychon.log.record_id", converted)?;
            }
        }

        event.set("event.category", Value::Array(vec![json!("configuration")]))?;

        Ok(TransformResult::Continue)
    }
}
