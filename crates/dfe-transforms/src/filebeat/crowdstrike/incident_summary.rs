// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `incident_summary` pipeline.
pub struct IncidentSummary;

impl Transform for IncidentSummary {
    fn name(&self) -> &str {
        "incident_summary"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("alert"))?;

        event.append("event.category", json!("malware"))?;

        event.append("event.type", json!("info"))?;

        event.append("event.action", json!("incident"))?;

        if event.has("crowdstrike.event.UserId") {
            event.rename("crowdstrike.event.UserId", "user.name")?;
        }

        let cond = {
            event.has("crowdstrike.event.IncidentStartTime")
                && event
                    .get_as_string("crowdstrike.event.IncidentStartTime")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentStartTime") {
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
            event.has("crowdstrike.event.IncidentStartTime")
                && event
                    .get_as_string("crowdstrike.event.IncidentStartTime")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentStartTime") {
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

        let cond = {
            event.has("crowdstrike.event.IncidentEndTime")
                && event
                    .get_as_string("crowdstrike.event.IncidentEndTime")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentEndTime") {
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
            event.has("crowdstrike.event.IncidentEndTime")
                && event
                    .get_as_string("crowdstrike.event.IncidentEndTime")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentEndTime") {
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

        if event.has("crowdstrike.event.FalconHostLink") {
            event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
        }

        if event.has("crowdstrike.event.HostID") {
            event.rename("crowdstrike.event.HostID", "host.id")?;
        }

        if event.has("crowdstrike.event.IncidentID") {
            event.rename("crowdstrike.event.IncidentID", "event.id")?;
        }

        let cond = { event.has("crowdstrike.event.FineScore") };
        if cond {
            event.set(
                "message",
                json!(format!(
                    "Incident score {}",
                    event.get_str("crowdstrike.event.FineScore").unwrap_or("")
                )),
            )?;
        }

        Ok(TransformResult::Continue)
    }
}
