// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_domain_name` pipeline.
pub struct IndicatorDomainName;

impl Transform for IndicatorDomainName {
    fn name(&self) -> &str {
        "indicator_domain_name"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_ingest._value") {
                    // Grok pattern: ^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?$
                    // Grok pattern: ^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?$
                    // Grok pattern: ^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?$
                    // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?$
                    // Grok pattern: ^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?$
                    // Grok pattern: ^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?$
                    // Grok pattern: ^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?$
                    if !extract_first_match(
                        &[
                            cached_grok!("^\\[?domain-name:value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?$"),
                            cached_grok!("^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?$"),
                            cached_grok!("^\\[?domain-name:resolves_to_refs\\[\\*\\].value%{SPACE}=%{SPACE}'%{DATA:_tmp.url}'\\]?$"),
                            cached_grok!("^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?$"),
                            cached_grok!("^\\[?ipv4-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?$"),
                            cached_grok!("^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}'\\]?$"),
                            cached_grok!("^\\[?ipv6-addr:value%{SPACE}=%{SPACE}'%{IP:_tmp.ip}/%{NUMBER}'\\]?$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.url") };
            if _cond {
                event.append_unique("threat.indicator.url.domain", json!(event.get("_tmp.url").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.ip") };
            if _cond {
                event.append_unique("threat.indicator.ip", json!(event.get("_tmp.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("_tmp.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.url") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("_tmp.url").map_or_else(String::new, template_to_string)))?;
            }

                event.remove("_tmp");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
