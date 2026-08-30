// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `cvpn_feature` pipeline.
pub struct CvpnFeature;

impl Transform for CvpnFeature {
    fn name(&self) -> &str {
        "cvpn_feature"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^HTML_URL %{URI:citrix_adc.log.html_url}$
                    // Grok pattern: ^REWRITTEN_URL %{URI:citrix_adc.log.rewritten_url}$
                    // Grok pattern: ^MATCHED_URL %{URI:citrix_adc.log.matched_url}$
                    // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^HTML_URL %{URI:citrix_adc.log.html_url}$"),
                            cached_grok!("^REWRITTEN_URL %{URI:citrix_adc.log.rewritten_url}$"),
                            cached_grok!("^MATCHED_URL %{URI:citrix_adc.log.matched_url}$"),
                            cached_grok!("%{GREEDYDATA:citrix_adc.log.message}"),
                        ],
                        &input,
                        event,
                    )?;
                }
                Ok(())
            })();

            if let Some(v) = event.get("citrix_adc.log.html_url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.original", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.rewritten_url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.original", v)?;
            }

            if let Some(v) = event.get("citrix_adc.log.matched_url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.original", v)?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
