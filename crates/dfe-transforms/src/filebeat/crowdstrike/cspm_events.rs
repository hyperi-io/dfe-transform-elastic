// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `cspm_events` pipeline.
pub struct CspmEvents;

impl Transform for CspmEvents {
    fn name(&self) -> &str {
        "cspm_events"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("alert"))?;

        event.append("event.category", json!("configuration"))?;

        event.append("event.type", json!("info"))?;
        event.append("event.type", json!("change"))?;

        let cond = { event.get_str("crowdstrike.event.Disposition") == Some("Passed") };
        if cond {
            event.set("event.outcome", json!("success"))?;
        }

        let cond = { event.get_str("crowdstrike.event.Disposition") == Some("Failed") };
        if cond {
            event.set("event.outcome", json!("failure"))?;
        }

        if event.has("crowdstrike.event.EventAction") {
            event.rename("crowdstrike.event.EventAction", "event.action")?;
        }

        if event.has("crowdstrike.event.ReportUrl") {
            event.rename("crowdstrike.event.ReportUrl", "event.reference")?;
        }

        let cond = { event.has("crowdstrike.event.ResourceAttributes") };
        if cond {
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

        let cond = { !event.has("cloud.account.id") };
        if cond {
            if event.has("crowdstrike.event.AccountId") {
                event.rename("crowdstrike.event.AccountId", "cloud.account.id")?;
            }
        }

        let cond = { !event.has("cloud.region") };
        if cond {
            if event.has("crowdstrike.event.Region") {
                event.rename("crowdstrike.event.Region", "cloud.region")?;
            }
        }

        let cond = { !event.has("cloud.provider") };
        if cond {
            if event.has("crowdstrike.event.CloudProvider") {
                event.rename("crowdstrike.event.CloudProvider", "cloud.provider")?;
            }
        }

        let cond = { !event.has("cloud.provider") };
        if cond {
            if event.has("crowdstrike.event.CloudPlatform") {
                event.rename("crowdstrike.event.CloudPlatform", "cloud.provider")?;
            }
        }

        let cond = { !event.has("cloud.service.name") };
        if cond {
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

        let cond = {
            event.has("crowdstrike.event.Timestamp")
                && event
                    .get_as_string("crowdstrike.event.Timestamp")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
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

        let cond = {
            event.has("crowdstrike.event.Timestamp")
                && event
                    .get_as_string("crowdstrike.event.Timestamp")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
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

        let cond = {
            event.has("crowdstrike.event.EventCreatedTimestamp")
                && event
                    .get_as_string("crowdstrike.event.EventCreatedTimestamp")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.EventCreatedTimestamp") {
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

        let cond = {
            event.has("crowdstrike.event.EventCreatedTimestamp")
                && event
                    .get_as_string("crowdstrike.event.EventCreatedTimestamp")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.EventCreatedTimestamp") {
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

        // ResourceCreateTime is present and epoch zero, which means unset
        let cond = event
            .get_i64("crowdstrike.event.ResourceCreateTime")
            .is_some_and(|t| t == 0);
        if cond {
            event.remove("crowdstrike.event.ResourceCreateTime");
        }

        let cond = {
            event.has("crowdstrike.event.ResourceCreateTime") && event.get_as_string("crowdstrike.event.ResourceCreateTime != 0 && String.valueOf(ctx.crowdstrike.event.ResourceCreateTime").is_some_and(|s| s.len() >= 12)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.ResourceCreateTime") {
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

        let cond = {
            event.has("crowdstrike.event.ResourceCreateTime") && event.get_as_string("crowdstrike.event.ResourceCreateTime != 0 && String.valueOf(ctx.crowdstrike.event.ResourceCreateTime").is_some_and(|s| s.len() <= 11)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.ResourceCreateTime") {
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

        Ok(TransformResult::Continue)
    }
}
