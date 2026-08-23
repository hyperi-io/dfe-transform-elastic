// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `recon_notification_summary` pipeline.
pub struct ReconNotificationSummary;

impl Transform for ReconNotificationSummary {
    fn name(&self) -> &str {
        "recon_notification_summary"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("threat"))?;

            event.append("event.type", json!("indicator"))?;

            let _cond = { !event.has_value("crowdstrike.event.ItemType") };
            if _cond {
                event.set("event.action", json!("recon-notification"))?;
            }

            let _cond = { event.has_value("crowdstrike.event.ItemType") };
            if _cond {
                event.set(
                    "event.action",
                    json!(format!(
                        "recon-notification-{}",
                        event
                            .get("crowdstrike.event.ItemType")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if event.has("crowdstrike.event.ItemId") {
                event.rename("crowdstrike.event.ItemId", "event.id")?;
            }

            if event.has("crowdstrike.event.RuleId") {
                event.rename("crowdstrike.event.RuleId", "rule.id")?;
            }

            if event.has("crowdstrike.event.RuleName") {
                event.rename("crowdstrike.event.RuleName", "rule.name")?;
            }

            if let Some(v) = event
                .get("crowdstrike.event.RuleTopic")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.ruleset", v)?;
            }

            if event.has("crowdstrike.event.RuleTopic") {
                event.rename("crowdstrike.event.RuleTopic", "rule.description")?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.MatchedTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.MatchedTimestamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.MatchedTimestamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("event.created", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.MatchedTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.MatchedTimestamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.MatchedTimestamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("event.created", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ItemPostedTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.ItemPostedTimestamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ItemPostedTimestamp")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("event.created", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.ItemPostedTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.ItemPostedTimestamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.ItemPostedTimestamp")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("event.created", parsed)?;
                    }
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

        Ok(TransformResult::Continue)
    }
}
