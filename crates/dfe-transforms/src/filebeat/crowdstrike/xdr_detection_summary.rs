// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `xdr_detection_summary` pipeline.
pub struct XdrDetectionSummary;

impl Transform for XdrDetectionSummary {
    fn name(&self) -> &str {
        "xdr_detection_summary"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("event.kind", json!("alert"))?;

        event.append("event.category", json!("malware"))?;

        event.append("event.type", json!("info"))?;

        event.set("event.action", json!("xdr-detection"))?;

        let cond = { event.has("crowdstrike.event.Author") };
        if cond {
            event.append(
                "rule.author",
                event
                    .get("crowdstrike.event.Author")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        if event.has("crowdstrike.event.Severity") {
            event.rename("crowdstrike.event.Severity", "event.severity")?;
        }

        if event.has("crowdstrike.event.Name") {
            event.rename("crowdstrike.event.Name", "rule.name")?;
        }

        if event.has("crowdstrike.event.DetectId") {
            event.rename("crowdstrike.event.DetectId", "rule.id")?;
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

        if event.has("crowdstrike.event.Description") {
            event.rename("crowdstrike.event.Description", "message")?;
        }

        let cond = {
            event.has("crowdstrike.event.DataDomains")
                && event
                    .get("crowdstrike.event.DataDomains")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.DataDomains") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("crowdstrike.event.DataDomains", Value::Array(parts))?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.EmailAddresses")
                && event
                    .get("crowdstrike.event.EmailAddresses")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.EmailAddresses") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("crowdstrike.event.EmailAddresses", Value::Array(parts))?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.IPV4Addresses")
                && event
                    .get("crowdstrike.event.IPV4Addresses")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.IPV4Addresses") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("related.ip", Value::Array(parts))?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.IPV4Addresses")
                && !(event
                    .get("crowdstrike.event.IPV4Addresses")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    }))
        };
        if cond {
            event.append(
                "related.ip",
                event
                    .get("crowdstrike.event.IPV4Addresses")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = {
            event.has("crowdstrike.event.IPV6Addresses")
                && event
                    .get("crowdstrike.event.IPV6Addresses")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.IPV6Addresses") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("related.ip", Value::Array(parts))?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.IPV6Addresses")
                && !(event
                    .get("crowdstrike.event.IPV6Addresses")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    }))
        };
        if cond {
            event.append(
                "related.ip",
                event
                    .get("crowdstrike.event.IPV6Addresses")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = {
            event.has("crowdstrike.event.HostNames")
                && event
                    .get("crowdstrike.event.HostNames")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.HostNames") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("related.hosts", Value::Array(parts))?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.HostNames")
                && !(event
                    .get("crowdstrike.event.HostNames")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    }))
        };
        if cond {
            event.append(
                "related.hosts",
                event
                    .get("crowdstrike.event.HostNames")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = {
            event.has("crowdstrike.event.DomainNames")
                && event
                    .get("crowdstrike.event.DomainNames")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.DomainNames") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("related.hosts", Value::Array(parts))?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.DomainNames")
                && !(event
                    .get("crowdstrike.event.DomainNames")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    }))
        };
        if cond {
            event.append(
                "related.hosts",
                event
                    .get("crowdstrike.event.DomainNames")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = {
            event.has("crowdstrike.event.SHA256Hashes")
                && event
                    .get("crowdstrike.event.SHA256Hashes")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.SHA256Hashes") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("related.hash", Value::Array(parts))?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.SHA256Hashes")
                && !(event
                    .get("crowdstrike.event.SHA256Hashes")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    }))
        };
        if cond {
            event.append(
                "related.hash",
                event
                    .get("crowdstrike.event.SHA256Hashes")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = {
            event.has("crowdstrike.event.MD5Hashes")
                && event
                    .get("crowdstrike.event.MD5Hashes")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.MD5Hashes") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("related.hash", Value::Array(parts))?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.MD5Hashes")
                && !(event
                    .get("crowdstrike.event.MD5Hashes")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    }))
        };
        if cond {
            event.append(
                "related.hash",
                event
                    .get("crowdstrike.event.MD5Hashes")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        let cond = {
            event.has("crowdstrike.event.Users")
                && event
                    .get("crowdstrike.event.Users")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    })
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.Users") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("related.user", Value::Array(parts))?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.Users")
                && !(event
                    .get("crowdstrike.event.Users")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(",")),
                        serde_json::Value::String(s) => s.contains(","),
                        _ => false,
                    }))
        };
        if cond {
            event.append(
                "related.user",
                event
                    .get("crowdstrike.event.Users")
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

        let cond = { event.has("crowdstrike.event.StartTimeEpoch") };
        if cond {
            if event.has("crowdstrike.event.StartTimeEpoch") {
                if let Some(val) = event.get("crowdstrike.event.StartTimeEpoch") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("crowdstrike.event.StartTimeEpoch", converted)?;
                }
            }
        }

        let cond = {
            event.has("crowdstrike.event.StartTimeEpoch")
                && event
                    .get_as_string("crowdstrike.event.StartTimeEpoch")
                    .is_some_and(|s| s.len() > 18)
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.StartTimeEpoch") {
                let re = regex::Regex::new("\\d{6}$").unwrap();
                let replaced = re.replace_all(&s, "").into_owned();
                event.set("crowdstrike.event.StartTimeEpoch", replaced)?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.StartTimeEpoch")
                && event
                    .get_as_string("crowdstrike.event.StartTimeEpoch")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimeEpoch") {
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
            event.has("crowdstrike.event.StartTimeEpoch")
                && event
                    .get_as_string("crowdstrike.event.StartTimeEpoch")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimeEpoch") {
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

        let cond = { event.has("event.start") };
        if cond {
            event.set(
                "@timestamp",
                event.get("event.start").cloned().unwrap_or(Value::Null),
            )?;
        }

        let cond = { event.has("crowdstrike.event.EndTimeEpoch") };
        if cond {
            if event.has("crowdstrike.event.EndTimeEpoch") {
                if let Some(val) = event.get("crowdstrike.event.EndTimeEpoch") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("crowdstrike.event.EndTimeEpoch", converted)?;
                }
            }
        }

        let cond = {
            event.has("crowdstrike.event.EndTimeEpoch")
                && event
                    .get_as_string("crowdstrike.event.EndTimeEpoch")
                    .is_some_and(|s| s.len() > 18)
        };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.EndTimeEpoch") {
                let re = regex::Regex::new("\\d{6}$").unwrap();
                let replaced = re.replace_all(&s, "").into_owned();
                event.set("crowdstrike.event.EndTimeEpoch", replaced)?;
            }
        }

        let cond = {
            event.has("crowdstrike.event.EndTimeEpoch")
                && event
                    .get_as_string("crowdstrike.event.EndTimeEpoch")
                    .is_some_and(|s| s.len() >= 12)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimeEpoch") {
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
            event.has("crowdstrike.event.EndTimeEpoch")
                && event
                    .get_as_string("crowdstrike.event.EndTimeEpoch")
                    .is_some_and(|s| s.len() <= 11)
        };
        if cond {
            if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimeEpoch") {
                // Try UNIX timestamp (skip epoch 0)
                if let Ok(ts) = date_str.parse::<f64>() {
                    if ts > 0.0 {
                        let secs = ts as i64;
                        let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                        if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
                            event.set(
                                "process.end",
                                dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                            )?;
                        }
                    }
                }
            }
        }

        event.set("threat.framework", json!("MITRE ATT&CK"))?;

        let cond = { event.has("crowdstrike.event.Techniques") };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.Techniques") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("threat.technique.name", Value::Array(parts))?;
            }
        }

        let cond = { event.has("crowdstrike.event.TechniqueIds") };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.TechniqueIds") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("threat.technique.id", Value::Array(parts))?;
            }
        }

        let cond = { event.has("crowdstrike.event.Tactics") };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.Tactics") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("threat.tactic.name", Value::Array(parts))?;
            }
        }

        let cond = { event.has("crowdstrike.event.TacticIds") };
        if cond {
            if let Some(s) = event.get_string("crowdstrike.event.TacticIds") {
                let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                event.set("threat.tactic.id", Value::Array(parts))?;
            }
        }

        Ok(TransformResult::Continue)
    }
}
