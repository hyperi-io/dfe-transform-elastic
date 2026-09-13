// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `identity_protection_incident` pipeline.
pub struct IdentityProtectionIncident;

impl Transform for IdentityProtectionIncident {
    fn name(&self) -> &str {
        "identity_protection_incident"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("iam"))?;

            event.append("event.type", json!("info"))?;

            if event.has_value("crowdstrike.event.IncidentType") {
                event.rename("crowdstrike.event.IncidentType", "event.action")?;
            }

            if event.has_value("crowdstrike.event.IncidentDescription") {
                event.rename("crowdstrike.event.IncidentDescription", "message")?;
            }

            if event.has_value("crowdstrike.event.IdentityProtectionIncidentId") {
                event.rename("crowdstrike.event.IdentityProtectionIncidentId", "event.id")?;
            }

            if event.has_value("crowdstrike.event.FalconHostLink") {
                event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
            }

            if event.has_value("crowdstrike.event.UserName") {
                event.rename("crowdstrike.event.UserName", "user.name")?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event.get("user.name").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\")),
                        serde_json::Value::String(s) => s.contains("\\"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(input) = event.get_string("user.name") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("\\") else {
                            break 'dissect false;
                        };
                        captured.push(("user.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("\\") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("user.name", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "user.name".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            if event.has_value("crowdstrike.event.EndpointName") {
                event.rename("crowdstrike.event.EndpointName", "host.hostname")?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.EndpointIp")
                    && event.get_str("crowdstrike.event.EndpointIp") != Some("")
            };
            if _cond {
                event.append(
                    "host.ip",
                    json!(
                        event
                            .get("crowdstrike.event.EndpointIp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.EndpointIp") };
            if _cond {
                if event.remove("crowdstrike.event.EndpointIp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "crowdstrike.event.EndpointIp".into(),
                    });
                }
            }

            let _cond = { event.has_value("crowdstrike.event.StartTime") };
            if _cond {
                if event.has_value("crowdstrike.event.StartTime") {
                    if let Some(val) = event.get("crowdstrike.event.StartTime") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.StartTime".into(),
                                message,
                            }
                        })?;
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
                gsub_field(
                    event,
                    "crowdstrike.event.StartTime",
                    "crowdstrike.event.StartTime",
                    cached_regex!("\\d{6}$"),
                    "",
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.StartTime")
                    && event
                        .get_as_string("crowdstrike.event.StartTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTime") {
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.StartTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
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
                    match parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.StartTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("crowdstrike.event.EndTime") };
            if _cond {
                if event.has_value("crowdstrike.event.EndTime") {
                    if let Some(val) = event.get("crowdstrike.event.EndTime") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.EndTime".into(),
                                message,
                            }
                        })?;
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
                gsub_field(
                    event,
                    "crowdstrike.event.EndTime",
                    "crowdstrike.event.EndTime",
                    cached_regex!("\\d{6}$"),
                    "",
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.EndTime")
                    && event
                        .get_as_string("crowdstrike.event.EndTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTime") {
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.EndTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
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
                    match parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.EndTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("event.start") };
            if _cond {
                if let Some(v) = event.get("event.start").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
