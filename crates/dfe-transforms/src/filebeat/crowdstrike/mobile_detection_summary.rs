// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `mobile_detection_summary` pipeline.
pub struct MobileDetectionSummary;

impl Transform for MobileDetectionSummary {
    fn name(&self) -> &str {
        "mobile_detection_summary"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("malware"))?;

            event.append("event.type", json!("info"))?;

            event.set("event.action", json!("mobile-detection"))?;

            let _cond = { event.has_value("crowdstrike.event.ContextTimeStamp") };
            if _cond {
                event.remove("event.created");
            }

            let _cond = {
                event.has_value("crowdstrike.event.ContextTimeStamp")
                    && event
                        .get_as_string("crowdstrike.event.ContextTimeStamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ContextTimeStamp") {
                    // Try UNIX timestamp (skip epoch 0)
                    if let Ok(ts) = date_str.parse::<f64>() {
                        if ts > 0.0 {
                            let secs = ts as i64;
                            let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                            if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
                                event.set(
                                    "event.created",
                                    dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                                )?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ContextTimeStamp")
                    && event
                        .get_as_string("crowdstrike.event.ContextTimeStamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ContextTimeStamp") {
                    // Try UNIX_MS timestamp (skip epoch 0)
                    if let Ok(ms) = date_str.parse::<i64>() {
                        if ms > 0 {
                            if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
                                event.set(
                                    "event.created",
                                    dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                                )?;
                            }
                        }
                    }
                }
            }

            if event.has("crowdstrike.event.MobileDetectionId") {
                event.rename("crowdstrike.event.MobileDetectionId", "event.id")?;
            }

            if event.has("event.id") {
                if let Some(val) = event.get("event.id") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("event.id", converted)?;
                }
            }

            if event.has("crowdstrike.event.DetectId") {
                event.rename("crowdstrike.event.DetectId", "rule.id")?;
            }

            if event.has("crowdstrike.event.DetectName") {
                event.rename("crowdstrike.event.DetectName", "rule.name")?;
            }

            if event.has("crowdstrike.event.DetectDescription") {
                event.rename("crowdstrike.event.DetectDescription", "rule.description")?;
            }

            event.set("threat.framework", json!("MITRE ATT&CK"))?;

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

            let _cond = { event.has_value("crowdstrike.event.TechniqueId") };
            if _cond {
                event.append(
                    "threat.technique.id",
                    event
                        .get("crowdstrike.event.TechniqueId")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
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

            let _cond = { event.has_value("crowdstrike.event.TacticId") };
            if _cond {
                event.append(
                    "threat.tactic.id",
                    event
                        .get("crowdstrike.event.TacticId")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            if event.has("crowdstrike.event.ComputerName") {
                event.rename("crowdstrike.event.ComputerName", "host.name")?;
            }

            if event.has("crowdstrike.event.UserName") {
                event.rename("crowdstrike.event.UserName", "user.name")?;
            }

            if event.has("crowdstrike.event.FalconHostLink") {
                event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
            }

            if event.has("crowdstrike.event.Severity") {
                event.rename("crowdstrike.event.Severity", "event.severity")?;
            }

            if event.has("crowdstrike.event.SensorId") {
                event.rename("crowdstrike.event.SensorId", "device.id")?;
            }

            if event.has("crowdstrike.event.ProcessId") {
                event.rename("crowdstrike.event.ProcessId", "process.pid")?;
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
