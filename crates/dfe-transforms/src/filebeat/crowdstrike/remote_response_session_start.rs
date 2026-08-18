// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `remote_response_session_start` pipeline.
pub struct RemoteResponseSessionStart;

impl Transform for RemoteResponseSessionStart {
    fn name(&self) -> &str {
        "remote_response_session_start"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("event"))?;

        event.append("event.category", json!("network"))?;
        event.append("event.category", json!("session"))?;

        event.append("event.action", json!("remote_response_session_start_event"))?;

        event.append("event.type", json!("start"))?;

        if event.has("crowdstrike.event.UserName") {
            event.rename("crowdstrike.event.UserName", "user.name")?;
        }

        let _cond = {
            event.has("crowdstrike.event.StartTimestamp")
                && event
                    .get_as_string("crowdstrike.event.StartTimestamp")
                    .is_some_and(|s| s.len() >= 12)
        };
        if _cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimestamp") {
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

        let _cond = {
            event.has("crowdstrike.event.StartTimestamp")
                && event
                    .get_as_string("crowdstrike.event.StartTimestamp")
                    .is_some_and(|s| s.len() <= 11)
        };
        if _cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimestamp") {
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

        event.set("message", json!("Remote response session started."))?;

        if event.has("crowdstrike.event.HostnameField") {
            event.rename("crowdstrike.event.HostnameField", "host.name")?;
        }

        Ok(TransformResult::Continue)
    }
}
