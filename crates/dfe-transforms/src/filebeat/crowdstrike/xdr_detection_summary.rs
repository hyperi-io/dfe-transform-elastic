// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `xdr_detection_summary` pipeline.
pub struct XdrDetectionSummary;

impl Transform for XdrDetectionSummary {
    fn name(&self) -> &str {
        "xdr_detection_summary"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("malware"))?;

            event.append("event.type", json!("info"))?;

            event.set("event.action", json!("xdr-detection"))?;

            let _cond = { event.has_value("crowdstrike.event.Author") };
            if _cond {
                event.append(
                    "rule.author",
                    json!(
                        event
                            .get("crowdstrike.event.Author")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("crowdstrike.event.Name") {
                event.rename("crowdstrike.event.Name", "rule.name")?;
            }

            if event.has("crowdstrike.event.DetectId") {
                event.rename("crowdstrike.event.DetectId", "rule.id")?;
            }

            if event.has_value("crowdstrike.event.PatternId") {
                if let Some(val) = event.get("crowdstrike.event.PatternId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "crowdstrike.event.PatternId".into(),
                            message,
                        }
                    })?;
                    event.set("rule.uuid", converted)?;
                }
            }

            if event.has("crowdstrike.event.Description") {
                event.rename("crowdstrike.event.Description", "message")?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.DataDomains")
                    && event
                        .get("crowdstrike.event.DataDomains")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.DataDomains") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("crowdstrike.event.DataDomains", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.EmailAddresses")
                    && event
                        .get("crowdstrike.event.EmailAddresses")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.EmailAddresses") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("crowdstrike.event.EmailAddresses", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.IPV4Addresses")
                    && event
                        .get("crowdstrike.event.IPV4Addresses")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.IPV4Addresses") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("related.ip", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.IPV4Addresses")
                    && !(event
                        .get("crowdstrike.event.IPV4Addresses")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        }))
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("crowdstrike.event.IPV4Addresses")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.IPV6Addresses")
                    && event
                        .get("crowdstrike.event.IPV6Addresses")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.IPV6Addresses") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("related.ip", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.IPV6Addresses")
                    && !(event
                        .get("crowdstrike.event.IPV6Addresses")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        }))
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("crowdstrike.event.IPV6Addresses")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.HostNames")
                    && event
                        .get("crowdstrike.event.HostNames")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.HostNames") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("related.hosts", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.HostNames")
                    && !(event
                        .get("crowdstrike.event.HostNames")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        }))
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("crowdstrike.event.HostNames")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.DomainNames")
                    && event
                        .get("crowdstrike.event.DomainNames")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.DomainNames") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("related.hosts", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.DomainNames")
                    && !(event
                        .get("crowdstrike.event.DomainNames")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        }))
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("crowdstrike.event.DomainNames")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.SHA256Hashes")
                    && event
                        .get("crowdstrike.event.SHA256Hashes")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.SHA256Hashes") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("related.hash", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.SHA256Hashes")
                    && !(event
                        .get("crowdstrike.event.SHA256Hashes")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        }))
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("crowdstrike.event.SHA256Hashes")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.MD5Hashes")
                    && event
                        .get("crowdstrike.event.MD5Hashes")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.MD5Hashes") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("related.hash", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.MD5Hashes")
                    && !(event
                        .get("crowdstrike.event.MD5Hashes")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        }))
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("crowdstrike.event.MD5Hashes")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.Users")
                    && event
                        .get("crowdstrike.event.Users")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.Users") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("related.user", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.Users")
                    && !(event
                        .get("crowdstrike.event.Users")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        }))
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.event.Users")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("message") };
            if _cond {
                if let Some(v) = event.get("message").cloned() {
                    event.set("rule.description", v)?;
                }
            }

            let _cond = { event.has_value("crowdstrike.event.StartTimeEpoch") };
            if _cond {
                if event.has_value("crowdstrike.event.StartTimeEpoch") {
                    if let Some(val) = event.get("crowdstrike.event.StartTimeEpoch") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.StartTimeEpoch".into(),
                                message,
                            }
                        })?;
                        event.set("crowdstrike.event.StartTimeEpoch", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.StartTimeEpoch")
                    && event
                        .get_as_string("crowdstrike.event.StartTimeEpoch")
                        .is_some_and(|s| s.len() > 18)
            };
            if _cond {
                gsub_field(
                    event,
                    "crowdstrike.event.StartTimeEpoch",
                    "crowdstrike.event.StartTimeEpoch",
                    cached_regex!("\\d{6}$"),
                    "",
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.StartTimeEpoch")
                    && event
                        .get_as_string("crowdstrike.event.StartTimeEpoch")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimeEpoch") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("event.start", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.StartTimeEpoch")
                    && event
                        .get_as_string("crowdstrike.event.StartTimeEpoch")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimeEpoch") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("event.start", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("event.start") };
            if _cond {
                if let Some(v) = event.get("event.start").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { event.has_value("crowdstrike.event.EndTimeEpoch") };
            if _cond {
                if event.has_value("crowdstrike.event.EndTimeEpoch") {
                    if let Some(val) = event.get("crowdstrike.event.EndTimeEpoch") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.EndTimeEpoch".into(),
                                message,
                            }
                        })?;
                        event.set("crowdstrike.event.EndTimeEpoch", converted)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.EndTimeEpoch")
                    && event
                        .get_as_string("crowdstrike.event.EndTimeEpoch")
                        .is_some_and(|s| s.len() > 18)
            };
            if _cond {
                gsub_field(
                    event,
                    "crowdstrike.event.EndTimeEpoch",
                    "crowdstrike.event.EndTimeEpoch",
                    cached_regex!("\\d{6}$"),
                    "",
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.EndTimeEpoch")
                    && event
                        .get_as_string("crowdstrike.event.EndTimeEpoch")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimeEpoch") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("event.end", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.EndTimeEpoch")
                    && event
                        .get_as_string("crowdstrike.event.EndTimeEpoch")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimeEpoch") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("event.end", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("crowdstrike.event.Techniques") };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.Techniques") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("threat.technique.name", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("crowdstrike.event.TechniqueIds") };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.TechniqueIds") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("threat.technique.id", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("crowdstrike.event.Tactics") };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.Tactics") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("threat.tactic.name", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("crowdstrike.event.TacticIds") };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.TacticIds") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("threat.tactic.id", Value::Array(parts))?;
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
        Ok(TransformResult::Continue)
    }
}
