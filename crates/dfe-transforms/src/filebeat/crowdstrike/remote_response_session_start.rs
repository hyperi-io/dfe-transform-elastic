// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `remote_response_session_start` pipeline.
pub struct RemoteResponseSessionStart;

impl Transform for RemoteResponseSessionStart {
    fn name(&self) -> &str {
        "remote_response_session_start"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;
            event.append("event.category", json!("session"))?;

            event.append("event.action", json!("remote_response_session_start_event"))?;

            event.append("event.type", json!("start"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("crowdstrike.event.UserName") {
                    if let Some(input) = event.get_string("crowdstrike.event.UserName") {
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
                event.has_value("crowdstrike.event.UserName")
                    && event
                        .get_str("crowdstrike.event.UserName")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("crowdstrike.event.UserName").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.StartTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.StartTimestamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.StartTimestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.StartTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.StartTimestamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimestamp") {
                    match parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.StartTimestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.set("message", json!("Remote response session started."))?;

            if event.has("crowdstrike.event.HostnameField") {
                event.rename("crowdstrike.event.HostnameField", "host.name")?;
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
