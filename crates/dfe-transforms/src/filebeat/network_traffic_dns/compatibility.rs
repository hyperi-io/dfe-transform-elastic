// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `compatibility` pipeline.
pub struct Compatibility;

impl Transform for Compatibility {
    fn name(&self) -> &str {
        "compatibility"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("dns.additionals_count") {
                    event.rename("dns.additionals_count", "network_traffic.dns.additionals_count")?;
                }

                if event.has_value("dns.answers_count") {
                    event.rename("dns.answers_count", "network_traffic.dns.answers_count")?;
                }

                if event.has_value("dns.authorities_count") {
                    event.rename("dns.authorities_count", "network_traffic.dns.authorities_count")?;
                }

                if event.has_value("dns.authorities") {
                    event.rename("dns.authorities", "network_traffic.dns.authorities")?;
                }

                if event.has_value("dns.flags") {
                    event.rename("dns.flags", "network_traffic.dns.flags")?;
                }

                if event.has_value("dns.question.etld_plus_one") {
                    event.rename("dns.question.etld_plus_one", "network_traffic.dns.question.etld_plus_one")?;
                }

                if event.has_value("method") {
                    event.rename("method", "network_traffic.dns.method")?;
                }

                if event.has_value("query") {
                    event.rename("query", "network_traffic.dns.query")?;
                }

                if event.has_value("resource") {
                    event.rename("resource", "network_traffic.dns.resource")?;
                }

                if event.has_value("status") {
                    event.rename("status", "network_traffic.status")?;
                }

                if event.has_value("process.ppid") {
                    event.rename("process.ppid", "process.parent.pid")?;
                }

                event.remove("type");

                event.remove("event.dataset");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
