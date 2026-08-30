// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_url` pipeline.
pub struct PipelineUrl;

impl Transform for PipelineUrl {
    fn name(&self) -> &str {
        "pipeline_url"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.url.address") {
                    event.rename("json.url.address", "sentinel_one_cloud_funnel.event.url.address")?;
                }

            if let Some(v) = event.get("sentinel_one_cloud_funnel.event.url.address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.original", v)?;
            }

            if event.has_value("url.original") {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            if let Some(v) = event.get("url.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.address", v)?;
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(domain_str) = event.get_string("destination.address") {
                    let domain = domain_str.to_string();
                    event.set("destination.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        if let Some(registered) = rd.registered_domain {
                            event.set("destination.registered_domain", json!(registered))?;
                        }
                        event.set("destination.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("destination.subdomain", json!(sub))?;
                        }
                    }
                }
            }

                if event.has_value("json.event.url.action") {
                    event.rename("json.event.url.action", "sentinel_one_cloud_funnel.event.url.action")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
