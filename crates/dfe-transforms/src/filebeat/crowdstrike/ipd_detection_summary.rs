// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `ipd_detection_summary` pipeline.
pub struct IpdDetectionSummary;

impl Transform for IpdDetectionSummary {
    fn name(&self) -> &str {
        "ipd_detection_summary"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("alert"))?;

        event.append("event.category", json!("malware"))?;

        event.append("event.type", json!("info"))?;

        event.set("event.action", json!("ipd-detection"))?;

        let cond = { event.get_bool("crowdstrike.event.AttemptOutcome") == Some(true) };
        if cond {
            event.set("event.outcome", json!("success"))?;
        }

        let cond = { event.get_bool("crowdstrike.event.AttemptOutcome") == Some(false) };
        if cond {
            event.set("event.outcome", json!("failure"))?;
        }

        if event.has("crowdstrike.event.DetectDescription") {
            event.rename("crowdstrike.event.DetectDescription", "message")?;
        }

        if event.has("crowdstrike.event.LocationCountryCode") {
            event.rename(
                "crowdstrike.event.LocationCountryCode",
                "host.geo.country_iso_code",
            )?;
        }

        if event.has("crowdstrike.event.PatternId") {
            if let Some(val) = event.get("crowdstrike.event.PatternId") {
                let converted = match val {
                    Value::String(_) => val.clone(),
                    Value::Number(n) => json!(n.to_string()),
                    Value::Bool(b) => json!(b.to_string()),
                    Value::Null => json!("null"),
                    _ => json!(val.to_string()),
                };
                event.set("rule.uuid", converted)?;
            }
        }

        if event.has("crowdstrike.event.Severity") {
            event.rename("crowdstrike.event.Severity", "event.severity")?;
        }

        if event.has("crowdstrike.event.SourceAccountDomain") {
            event.rename("crowdstrike.event.SourceAccountDomain", "user.domain")?;
        }

        if event.has("crowdstrike.event.SourceAccountName") {
            event.rename("crowdstrike.event.SourceAccountName", "user.name")?;
        }

        if event.has("crowdstrike.event.SourceAccountObjectSid") {
            event.rename("crowdstrike.event.SourceAccountObjectSid", "user.id")?;
        }

        if event.has("crowdstrike.event.SourceEndpointHostName") {
            event.rename("crowdstrike.event.SourceEndpointHostName", "host.name")?;
        }

        if event.has("crowdstrike.event.SourceEndpointIpAddress") {
            event.rename("crowdstrike.event.SourceEndpointIpAddress", "host.ip")?;
        }

        let cond = { event.has("crowdstrike.event.Technique") };
        if cond {
            event.append(
                "threat.technique.name",
                event
                    .get("crowdstrike.event.Technique")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.TechniqueId") };
        if cond {
            event.append(
                "threat.technique.id",
                event
                    .get("crowdstrike.event.TechniqueId")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.Tactic") };
        if cond {
            event.append(
                "threat.tactic.name",
                event
                    .get("crowdstrike.event.Tactic")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.TacticId") };
        if cond {
            event.append(
                "threat.tactic.id",
                event
                    .get("crowdstrike.event.TacticId")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("message") };
        if cond {
            event.set(
                "rule.description",
                event.get("message").cloned().unwrap_or(Value::Null),
            )?;
        }

        if event.has("crowdstrike.event.DetectName") {
            event.rename("crowdstrike.event.DetectName", "rule.name")?;
        }

        if event.has("crowdstrike.event.DetectId") {
            event.rename("crowdstrike.event.DetectId", "rule.id")?;
        }

        if event.has("crowdstrike.event.FalconHostLink") {
            event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
        }

        let cond = { event.has("crowdstrike.event.ContextTimeStamp") };
        if cond {
            event.remove("event.created");
        }

        let cond = { event.has("crowdstrike.event.ContextTimeStamp") };
        if cond {
            if event.has("crowdstrike.event.ContextTimeStamp") {
                if let Some(val) = event.get("crowdstrike.event.ContextTimeStamp") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("crowdstrike.event.ContextTimeStamp", converted)?;
                }
            }
        }

        let cond = {
            event.has("crowdstrike.event.ContextTimeStamp")
                && event
                    .get_as_string("crowdstrike.event.ContextTimeStamp")
                    .is_some_and(|s| s.len() > 18)
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.ContextTimeStamp") {
                let re = regex::Regex::new("\\d{6}$").unwrap();
                let replaced = re.replace_all(&s, "").into_owned();
                event.set("crowdstrike.event.ContextTimeStamp", replaced)?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.ContextTimeStamp")
                && event
                    .get_as_string("crowdstrike.event.ContextTimeStamp")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
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

        let cond = {
            event.has("crowdstrike.event.ContextTimeStamp")
                && event
                    .get_as_string("crowdstrike.event.ContextTimeStamp")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
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

        let cond = { event.has("crowdstrike.event.AccountCreationTimeStamp") };
        if cond {
            if event.has("crowdstrike.event.AccountCreationTimeStamp") {
                if let Some(val) = event.get("crowdstrike.event.AccountCreationTimeStamp") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("crowdstrike.event.AccountCreationTimeStamp", converted)?;
                }
            }
        }

        let cond = {
            event.has("crowdstrike.event.AccountCreationTimeStamp")
                && event
                    .get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                    .is_some_and(|s| s.len() > 18)
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.AccountCreationTimeStamp") {
                let re = regex::Regex::new("\\d{6}$").unwrap();
                let replaced = re.replace_all(&s, "").into_owned();
                event.set("crowdstrike.event.AccountCreationTimeStamp", replaced)?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.AccountCreationTimeStamp")
                && event
                    .get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
            if let Some(date_str) =
                event.get_as_string("crowdstrike.event.AccountCreationTimeStamp")
            {
                // Try UNIX_MS timestamp (skip epoch 0)
                if let Ok(ms) = date_str.parse::<i64>() {
                    if ms > 0 {
                        if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
                            event.set(
                                "crowdstrike.event.AccountCreationTimeStamp",
                                dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                            )?;
                        }
                    }
                }
            }
        }

        let cond = {
            event.has("crowdstrike.event.AccountCreationTimeStamp")
                && event
                    .get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
            if let Some(date_str) =
                event.get_as_string("crowdstrike.event.AccountCreationTimeStamp")
            {
                // Try UNIX timestamp (skip epoch 0)
                if let Ok(ts) = date_str.parse::<f64>() {
                    if ts > 0.0 {
                        let secs = ts as i64;
                        let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                        if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
                            event.set(
                                "crowdstrike.event.AccountCreationTimeStamp",
                                dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                            )?;
                        }
                    }
                }
            }
        }

        let cond = { event.has("crowdstrike.event.StartTime") };
        if cond {
            if event.has("crowdstrike.event.StartTime") {
                if let Some(val) = event.get("crowdstrike.event.StartTime") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("crowdstrike.event.StartTime", converted)?;
                }
            }
        }

        let cond = {
            event.has("crowdstrike.event.StartTime")
                && event
                    .get_as_string("crowdstrike.event.StartTime")
                    .is_some_and(|s| s.len() > 18)
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.StartTime") {
                let re = regex::Regex::new("\\d{6}$").unwrap();
                let replaced = re.replace_all(&s, "").into_owned();
                event.set("crowdstrike.event.StartTime", replaced)?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.StartTime")
                && event
                    .get_as_string("crowdstrike.event.StartTime")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTime") {
                // Try UNIX_MS timestamp (skip epoch 0)
                if let Ok(ms) = date_str.parse::<i64>() {
                    if ms > 0 {
                        if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
                            event.set(
                                "event.start",
                                dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                            )?;
                        }
                    }
                }
            }
        }

        let cond = {
            event.has("crowdstrike.event.StartTime")
                && event
                    .get_as_string("crowdstrike.event.StartTime")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTime") {
                // Try UNIX timestamp (skip epoch 0)
                if let Ok(ts) = date_str.parse::<f64>() {
                    if ts > 0.0 {
                        let secs = ts as i64;
                        let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                        if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
                            event.set(
                                "event.start",
                                dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                            )?;
                        }
                    }
                }
            }
        }

        let cond = { event.has("crowdstrike.event.EndTime") };
        if cond {
            if event.has("crowdstrike.event.EndTime") {
                if let Some(val) = event.get("crowdstrike.event.EndTime") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("crowdstrike.event.EndTime", converted)?;
                }
            }
        }

        let cond = {
            event.has("crowdstrike.event.EndTime")
                && event
                    .get_as_string("crowdstrike.event.EndTime")
                    .is_some_and(|s| s.len() > 18)
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.EndTime") {
                let re = regex::Regex::new("\\d{6}$").unwrap();
                let replaced = re.replace_all(&s, "").into_owned();
                event.set("crowdstrike.event.EndTime", replaced)?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.EndTime")
                && event
                    .get_as_string("crowdstrike.event.EndTime")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTime") {
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

        let cond = {
            event.has("crowdstrike.event.EndTime")
                && event
                    .get_as_string("crowdstrike.event.EndTime")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTime") {
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

        let cond = { event.has("crowdstrike.event.TargetEndpointHostName") };
        if cond {
            event.append(
                "related.hosts",
                event
                    .get("crowdstrike.event.TargetEndpointHostName")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.TargetDomain") };
        if cond {
            event.append(
                "related.hosts",
                event
                    .get("crowdstrike.event.TargetDomain")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.TargetAccountName") };
        if cond {
            event.append(
                "related.user",
                event
                    .get("crowdstrike.event.TargetAccountName")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.AdditionalAccountDomain") };
        if cond {
            event.append(
                "related.hosts",
                event
                    .get("crowdstrike.event.AdditionalAccountDomain")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.AdditionalAccountName") };
        if cond {
            event.append(
                "related.hosts",
                event
                    .get("crowdstrike.event.AdditionalAccountName")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.AdditionalEndpointHostName") };
        if cond {
            event.append(
                "related.hosts",
                event
                    .get("crowdstrike.event.AdditionalEndpointHostName")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.AdditionalEndpointIpAddress") };
        if cond {
            event.append(
                "related.ip",
                event
                    .get("crowdstrike.event.AdditionalEndpointIpAddress")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        Ok(TransformResult::Continue)
    }
}
