// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `ipd_detection_summary` pipeline.
pub struct IpdDetectionSummary;

impl Transform for IpdDetectionSummary {
    fn name(&self) -> &str {
        "ipd_detection_summary"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("malware"))?;

            event.append("event.type", json!("info"))?;

            event.set("event.action", json!("ipd-detection"))?;

            let _cond = { event.get_bool("crowdstrike.event.AttemptOutcome") == Some(true) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_bool("crowdstrike.event.AttemptOutcome") == Some(false) };
            if _cond {
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

            let _cond = {
                event.has_value("crowdstrike.event.SourceEndpointIpAddress")
                    && event
                        .get_str("crowdstrike.event.SourceEndpointIpAddress")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                event.append(
                    "host.ip",
                    json!(
                        event
                            .get("crowdstrike.event.SourceEndpointIpAddress")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.SourceEndpointIpAddress") };
            if _cond {
                if event
                    .remove("crowdstrike.event.SourceEndpointIpAddress")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "crowdstrike.event.SourceEndpointIpAddress".into(),
                    });
                }
            }

            let _cond = { event.has_value("crowdstrike.event.Technique") };
            if _cond {
                event.append(
                    "threat.technique.name",
                    json!(
                        event
                            .get("crowdstrike.event.Technique")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.TechniqueId") };
            if _cond {
                event.append(
                    "threat.technique.id",
                    json!(
                        event
                            .get("crowdstrike.event.TechniqueId")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.Tactic") };
            if _cond {
                event.append(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("crowdstrike.event.Tactic")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.TacticId") };
            if _cond {
                event.append(
                    "threat.tactic.id",
                    json!(
                        event
                            .get("crowdstrike.event.TacticId")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("message") };
            if _cond {
                if let Some(v) = event.get("message").cloned() {
                    event.set("rule.description", v)?;
                }
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

            let _cond = { event.has_value("crowdstrike.event.ContextTimeStamp") };
            if _cond {
                event.remove("event.created");
            }

            let _cond = { event.has_value("crowdstrike.event.ContextTimeStamp") };
            if _cond {
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

            let _cond = {
                event.has_value("crowdstrike.event.ContextTimeStamp")
                    && event
                        .get_as_string("crowdstrike.event.ContextTimeStamp")
                        .is_some_and(|s| s.len() > 18)
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.ContextTimeStamp") {
                    let re = cached_regex!("\\d{6}$");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("crowdstrike.event.ContextTimeStamp", replaced)?;
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
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("event.created", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ContextTimeStamp")
                    && event
                        .get_as_string("crowdstrike.event.ContextTimeStamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ContextTimeStamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("event.created", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("crowdstrike.event.AccountCreationTimeStamp") };
            if _cond {
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

            let _cond = {
                event.has_value("crowdstrike.event.AccountCreationTimeStamp")
                    && event
                        .get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                        .is_some_and(|s| s.len() > 18)
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.AccountCreationTimeStamp") {
                    let re = cached_regex!("\\d{6}$");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("crowdstrike.event.AccountCreationTimeStamp", replaced)?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.AccountCreationTimeStamp")
                    && event
                        .get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("crowdstrike.event.AccountCreationTimeStamp", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.AccountCreationTimeStamp")
                    && event
                        .get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("crowdstrike.event.AccountCreationTimeStamp", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("crowdstrike.event.StartTime") };
            if _cond {
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

            let _cond = {
                event.has_value("crowdstrike.event.StartTime")
                    && event
                        .get_as_string("crowdstrike.event.StartTime")
                        .is_some_and(|s| s.len() > 18)
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.StartTime") {
                    let re = cached_regex!("\\d{6}$");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("crowdstrike.event.StartTime", replaced)?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.StartTime")
                    && event
                        .get_as_string("crowdstrike.event.StartTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("event.start", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.StartTime")
                    && event
                        .get_as_string("crowdstrike.event.StartTime")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("event.start", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("crowdstrike.event.EndTime") };
            if _cond {
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

            let _cond = {
                event.has_value("crowdstrike.event.EndTime")
                    && event
                        .get_as_string("crowdstrike.event.EndTime")
                        .is_some_and(|s| s.len() > 18)
            };
            if _cond {
                if let Some(s) = event.get_string("crowdstrike.event.EndTime") {
                    let re = cached_regex!("\\d{6}$");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("crowdstrike.event.EndTime", replaced)?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.EndTime")
                    && event
                        .get_as_string("crowdstrike.event.EndTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("event.end", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.EndTime")
                    && event
                        .get_as_string("crowdstrike.event.EndTime")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("event.end", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("crowdstrike.event.TargetEndpointHostName") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("crowdstrike.event.TargetEndpointHostName")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.TargetDomain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("crowdstrike.event.TargetDomain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.TargetAccountName") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("crowdstrike.event.TargetAccountName")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.AdditionalAccountDomain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("crowdstrike.event.AdditionalAccountDomain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.AdditionalAccountName") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("crowdstrike.event.AdditionalAccountName")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.AdditionalEndpointHostName") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("crowdstrike.event.AdditionalEndpointHostName")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.AdditionalEndpointIpAddress") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("crowdstrike.event.AdditionalEndpointIpAddress")
                            .map_or_else(String::new, painless_to_string)
                    ),
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
