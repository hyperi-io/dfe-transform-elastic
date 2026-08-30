// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `bot_feature` pipeline.
pub struct BotFeature;

impl Transform for BotFeature {
    fn name(&self) -> &str {
        "bot_feature"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("citrix.extended.message") {
                    // Grok pattern: ^Bot%{SPACE}New%{SPACE}Signature%{SPACE}Available.%{SPACE}Newly%{SPACE}added%{SPACE}Rules%{SPACE}:%{SPACE}%{INT:citrix_adc.log.newly_added_rules}%{SPACE}Deleted%{SPACE}Rules%{SPACE}:%{SPACE}%{INT:citrix_adc.log.deleted_rules}$
                    // Grok pattern: %{GREEDYDATA:citrix_adc.log.message}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^Bot%{SPACE}New%{SPACE}Signature%{SPACE}Available.%{SPACE}Newly%{SPACE}added%{SPACE}Rules%{SPACE}:%{SPACE}%{INT:citrix_adc.log.newly_added_rules}%{SPACE}Deleted%{SPACE}Rules%{SPACE}:%{SPACE}%{INT:citrix_adc.log.deleted_rules}$"),
                            cached_grok!("%{GREEDYDATA:citrix_adc.log.message}"),
                        ],
                        &input,
                        event,
                    )?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.newly_added_rules") {
                if let Some(val) = event.get("citrix_adc.log.newly_added_rules") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.newly_added_rules".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.newly_added_rules", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_newly_added_rules_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("citrix_adc.log.deleted_rules") {
                if let Some(val) = event.get("citrix_adc.log.deleted_rules") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "citrix_adc.log.deleted_rules".into(),
                            message,
                        })?;
                    event.set("citrix_adc.log.deleted_rules", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_deleted_rules_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
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
