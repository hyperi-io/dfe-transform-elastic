// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `user_activity_audit` pipeline.
pub struct UserActivityAudit;

impl Transform for UserActivityAudit {
    fn name(&self) -> &str {
        "user_activity_audit"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("iam"))?;

            event.append("event.type", json!("change"))?;

            event.set("event.action", json!("user_activity_audit_event"))?;

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

            if event.has("crowdstrike.event.OperationName") {
                event.rename("crowdstrike.event.OperationName", "message")?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.UserIp")
                    && event.get_str("crowdstrike.event.UserIp") != Some("")
            };
            if _cond {
                if event.has("crowdstrike.event.UserIp") {
                    event.rename("crowdstrike.event.UserIp", "source.ip")?;
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
