// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `indicator_windows_registry` pipeline.
pub struct IndicatorWindowsRegistry;

impl Transform for IndicatorWindowsRegistry {
    fn name(&self) -> &str {
        "indicator_windows_registry"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("_ingest._value") {
                    // Grok pattern: ^\\[?windows-registry-key:key%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_path}'\\]?$
                    // Grok pattern: ^\\[?windows-registry-key:key%{SPACE}LIKE%{SPACE}'%{DATA:_tmp.reg_path}'\\]?$
                    // Grok pattern: ^\\[?windows-registry-value-type:name%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_key}'\\]?$
                    // Grok pattern: ^\\[?windows-registry-value-type:data%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_value}'\\]?$
                    if !extract_first_match(
                        &[
                            cached_grok!("^\\[?windows-registry-key:key%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_path}'\\]?$"),
                            cached_grok!("^\\[?windows-registry-key:key%{SPACE}LIKE%{SPACE}'%{DATA:_tmp.reg_path}'\\]?$"),
                            cached_grok!("^\\[?windows-registry-value-type:name%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_key}'\\]?$"),
                            cached_grok!("^\\[?windows-registry-value-type:data%{SPACE}=%{SPACE}'%{DATA:_tmp.reg_value}'\\]?$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.reg_path") };
            if _cond {
                event.append_unique("threat.indicator.registry.path", json!(event.get("_tmp.reg_path").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.reg_key") };
            if _cond {
                event.append_unique("threat.indicator.registry.key", json!(event.get("_tmp.reg_key").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_tmp.reg_value") };
            if _cond {
                event.append_unique("threat.indicator.registry.value", json!(event.get("_tmp.reg_value").map_or_else(String::new, template_to_string)))?;
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
