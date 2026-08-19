// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `cspm_events` pipeline.
pub struct CspmEvents;

impl Transform for CspmEvents {
    fn name(&self) -> &str {
        "cspm_events"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("configuration"))?;

            event.append("event.type", json!("info"))?;
            event.append("event.type", json!("change"))?;

            let _cond = { event.get_str("crowdstrike.event.Disposition") == Some("Passed") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("crowdstrike.event.Disposition") == Some("Failed") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has("crowdstrike.event.EventAction") {
                event.rename("crowdstrike.event.EventAction", "event.action")?;
            }

            if event.has("crowdstrike.event.ReportUrl") {
                event.rename("crowdstrike.event.ReportUrl", "event.reference")?;
            }

            let _cond = { event.has_value("crowdstrike.event.ResourceAttributes") };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.ResourceAttributes") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "crowdstrike.event.ResourceAttributes".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("crowdstrike.event.ResourceAttributes", parsed)?;
                }
            }

            if event.has("crowdstrike.event.EventSource") {
                event.rename("crowdstrike.event.EventSource", "event.provider")?;
            }

            if event.has("crowdstrike.event.Severity") {
                event.rename("crowdstrike.event.Severity", "event.severity")?;
            }

            let _cond = { !event.has_value("cloud.account.id") };
            if _cond {
                if event.has("crowdstrike.event.AccountId") {
                    event.rename("crowdstrike.event.AccountId", "cloud.account.id")?;
                }
            }

            let _cond = { !event.has_value("cloud.region") };
            if _cond {
                if event.has("crowdstrike.event.Region") {
                    event.rename("crowdstrike.event.Region", "cloud.region")?;
                }
            }

            let _cond = { !event.has_value("cloud.provider") };
            if _cond {
                if event.has("crowdstrike.event.CloudProvider") {
                    event.rename("crowdstrike.event.CloudProvider", "cloud.provider")?;
                }
            }

            let _cond = { !event.has_value("cloud.provider") };
            if _cond {
                if event.has("crowdstrike.event.CloudPlatform") {
                    event.rename("crowdstrike.event.CloudPlatform", "cloud.provider")?;
                }
            }

            let _cond = { !event.has_value("cloud.service.name") };
            if _cond {
                if event.has("crowdstrike.event.CloudService") {
                    event.rename("crowdstrike.event.CloudService", "cloud.service.name")?;
                }
            }

            if event.has("crowdstrike.event.PolicyStatement") {
                event.rename("crowdstrike.event.PolicyStatement", "message")?;
            }

            if event.has("crowdstrike.event.UserName") {
                event.rename("crowdstrike.event.UserName", "user.name")?;
            }

            if event.has("crowdstrike.event.UserId") {
                event.rename("crowdstrike.event.UserId", "user.id")?;
            }

            if event.has("crowdstrike.event.UserSourceIp") {
                event.rename("crowdstrike.event.UserSourceIp", "source.ip")?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.Timestamp")
                    && event
                        .get_as_string("crowdstrike.event.Timestamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.Timestamp") {
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
                event.has_value("crowdstrike.event.Timestamp")
                    && event
                        .get_as_string("crowdstrike.event.Timestamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.Timestamp") {
                    // Try UNIX timestamp (skip epoch 0)
                    if let Ok(ts) = date_str.parse::<f64>() {
                        if ts > 0.0 {
                            let secs = ts as i64;
                            let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                            if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
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
                event.has_value("crowdstrike.event.EventCreatedTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.EventCreatedTimestamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.event.EventCreatedTimestamp")
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
                event.has_value("crowdstrike.event.EventCreatedTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.EventCreatedTimestamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.event.EventCreatedTimestamp")
                {
                    // Try UNIX timestamp (skip epoch 0)
                    if let Ok(ts) = date_str.parse::<f64>() {
                        if ts > 0.0 {
                            let secs = ts as i64;
                            let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                            if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
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
                event.has_value("crowdstrike.event.ResourceCreateTime")
                    && event.get_i64("crowdstrike.event.ResourceCreateTime") == Some(0)
            };
            if _cond {
                event.remove("crowdstrike.event.ResourceCreateTime");
            }

            let _cond = {
                event.has_value("crowdstrike.event.ResourceCreateTime")
                    && event.get_i64("crowdstrike.event.ResourceCreateTime") != Some(0)
                    && event
                        .get_as_string("crowdstrike.event.ResourceCreateTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ResourceCreateTime")
                {
                    // Try UNIX_MS timestamp (skip epoch 0)
                    if let Ok(ms) = date_str.parse::<i64>() {
                        if ms > 0 {
                            if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
                                event.set(
                                    "crowdstrike.event.ResourceCreateTime",
                                    dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                                )?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ResourceCreateTime")
                    && event.get_i64("crowdstrike.event.ResourceCreateTime") != Some(0)
                    && event
                        .get_as_string("crowdstrike.event.ResourceCreateTime")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ResourceCreateTime")
                {
                    // Try UNIX timestamp (skip epoch 0)
                    if let Ok(ts) = date_str.parse::<f64>() {
                        if ts > 0.0 {
                            let secs = ts as i64;
                            let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                            if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
                                event.set(
                                    "crowdstrike.event.ResourceCreateTime",
                                    dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                                )?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("crowdstrike.event.Tactic") };
            if _cond {
                event.append(
                    "threat.tactic.name",
                    event
                        .get("crowdstrike.event.Tactic")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.Technique") };
            if _cond {
                event.append(
                    "threat.technique.name",
                    event
                        .get("crowdstrike.event.Technique")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
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
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
