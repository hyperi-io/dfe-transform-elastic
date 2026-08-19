// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `remote_response_session_end` pipeline.
pub struct RemoteResponseSessionEnd;

impl Transform for RemoteResponseSessionEnd {
    fn name(&self) -> &str {
        "remote_response_session_end"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;
            event.append("event.category", json!("session"))?;

            event.append("event.action", json!("remote_response_session_end_event"))?;

            event.append("event.type", json!("end"))?;

            if event.has("crowdstrike.event.UserName") {
                event.rename("crowdstrike.event.UserName", "user.name")?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.EndTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.EndTimestamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimestamp") {
                    // Try UNIX_MS timestamp (skip epoch 0)
                    if let Ok(ms) = date_str.parse::<i64>() {
                        if ms > 0 {
                            if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
                                event.set(
                                    "event.end",
                                    dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                                )?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.EndTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.EndTimestamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimestamp") {
                    // Try UNIX timestamp (skip epoch 0)
                    if let Ok(ts) = date_str.parse::<f64>() {
                        if ts > 0.0 {
                            let secs = ts as i64;
                            let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                            if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
                                event.set(
                                    "event.end",
                                    dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                                )?;
                            }
                        }
                    }
                }
            }

            event.set("message", json!("Remote response session ended."))?;

            if event.has("crowdstrike.event.HostnameField") {
                event.rename("crowdstrike.event.HostnameField", "host.name")?;
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
