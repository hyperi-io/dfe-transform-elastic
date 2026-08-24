// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `cspm_events` pipeline.
pub struct CspmEvents;

impl Transform for CspmEvents {
    fn name(&self) -> &str {
        "cspm_events"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("configuration"))?;

            event.append("event.type", json!("info"))?;
            event.append("event.type", json!("change"))?;

            let _cond = { event.get_str("crowdstrike.event.Disposition") == Some("Passed") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("crowdstrike.event.Disposition") == Some("Failed") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has("crowdstrike.event.EventAction") {
                event.rename("crowdstrike.event.EventAction", "event.action")?;
            }

            if event.has("crowdstrike.event.ReportUrl") {
                event.rename("crowdstrike.event.ReportUrl", "event.reference")?;
            }

            let _cond = { event.has_value("crowdstrike.event.ResourceAttributes") };
            if _cond {
                parse_json_field(
                    event,
                    "crowdstrike.event.ResourceAttributes",
                    "crowdstrike.event.ResourceAttributes",
                )?;
            }

            if event.has("crowdstrike.event.EventSource") {
                event.rename("crowdstrike.event.EventSource", "event.provider")?;
            }

            let _cond = { !event.has_value("cloud.account.id") };
            if _cond {
                if event.has("crowdstrike.event.AccountId") {
                    event.rename("crowdstrike.event.AccountId", "cloud.account.id")?;
                }
            }

            let _cond = { !event.has_value("cloud.region") };
            if _cond {
                if event.has("crowdstrike.event.Region") {
                    event.rename("crowdstrike.event.Region", "cloud.region")?;
                }
            }

            let _cond = { !event.has_value("cloud.provider") };
            if _cond {
                if event.has("crowdstrike.event.CloudProvider") {
                    event.rename("crowdstrike.event.CloudProvider", "cloud.provider")?;
                }
            }

            let _cond = { !event.has_value("cloud.provider") };
            if _cond {
                if event.has("crowdstrike.event.CloudPlatform") {
                    event.rename("crowdstrike.event.CloudPlatform", "cloud.provider")?;
                }
            }

            let _cond = { !event.has_value("cloud.service.name") };
            if _cond {
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

            let _cond = {
                event.has_value("crowdstrike.event.Timestamp")
                    && event
                        .get_as_string("crowdstrike.event.Timestamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.Timestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.Timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.Timestamp")
                    && event
                        .get_as_string("crowdstrike.event.Timestamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.Timestamp") {
                    match parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.Timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.EventCreatedTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.EventCreatedTimestamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.event.EventCreatedTimestamp")
                {
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.EventCreatedTimestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.EventCreatedTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.EventCreatedTimestamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.event.EventCreatedTimestamp")
                {
                    match parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.EventCreatedTimestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ResourceCreateTime")
                    && event.get_i64("crowdstrike.event.ResourceCreateTime") == Some(0)
            };
            if _cond {
                event.remove("crowdstrike.event.ResourceCreateTime");
            }

            let _cond = {
                event.has_value("crowdstrike.event.ResourceCreateTime")
                    && event.get_i64("crowdstrike.event.ResourceCreateTime") != Some(0)
                    && event
                        .get_as_string("crowdstrike.event.ResourceCreateTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ResourceCreateTime")
                {
                    match parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => {
                            event.set("crowdstrike.event.ResourceCreateTime", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.ResourceCreateTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ResourceCreateTime")
                    && event.get_i64("crowdstrike.event.ResourceCreateTime") != Some(0)
                    && event
                        .get_as_string("crowdstrike.event.ResourceCreateTime")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ResourceCreateTime")
                {
                    match parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        Some(parsed) => {
                            event.set("crowdstrike.event.ResourceCreateTime", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "crowdstrike.event.ResourceCreateTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("crowdstrike.event.Tactic") };
            if _cond {
                event.append(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("crowdstrike.event.Tactic")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.Technique") };
            if _cond {
                event.append(
                    "threat.technique.name",
                    json!(
                        event
                            .get("crowdstrike.event.Technique")
                            .map_or_else(String::new, template_to_string)
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
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
