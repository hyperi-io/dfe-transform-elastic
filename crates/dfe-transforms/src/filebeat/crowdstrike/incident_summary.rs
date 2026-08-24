// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `incident_summary` pipeline.
pub struct IncidentSummary;

impl Transform for IncidentSummary {
    fn name(&self) -> &str {
        "incident_summary"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("malware"))?;

            event.append("event.type", json!("info"))?;

            event.append("event.action", json!("incident"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("crowdstrike.event.UserId") {
                    if let Some(input) = event.get_string("crowdstrike.event.UserId") {
                        // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        let _ = extract_first_match(
                            &[
                                cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("crowdstrike.event.UserId")
                    && event
                        .get_str("crowdstrike.event.UserId")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("crowdstrike.event.UserId").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.IncidentStartTime")
                    && event
                        .get_as_string("crowdstrike.event.IncidentStartTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentStartTime") {
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.IncidentStartTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.IncidentStartTime")
                    && event
                        .get_as_string("crowdstrike.event.IncidentStartTime")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentStartTime") {
                    match parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.IncidentStartTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.IncidentEndTime")
                    && event
                        .get_as_string("crowdstrike.event.IncidentEndTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentEndTime") {
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.IncidentEndTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.IncidentEndTime")
                    && event
                        .get_as_string("crowdstrike.event.IncidentEndTime")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentEndTime") {
                    match parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.IncidentEndTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("crowdstrike.event.FalconHostLink") {
                event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
            }

            if event.has_value("crowdstrike.event.HostID") {
                event.rename("crowdstrike.event.HostID", "host.id")?;
            }

            if event.has_value("crowdstrike.event.IncidentID") {
                event.rename("crowdstrike.event.IncidentID", "event.id")?;
            }

            let _cond = { event.has_value("crowdstrike.event.FineScore") };
            if _cond {
                event.set(
                    "message",
                    json!(format!(
                        "Incident score {}",
                        event
                            .get("crowdstrike.event.FineScore")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
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

        Ok(TransformResult::Continue)
    }
}
