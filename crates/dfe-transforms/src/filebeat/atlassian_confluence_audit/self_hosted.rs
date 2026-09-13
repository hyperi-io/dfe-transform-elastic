// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `self_hosted` pipeline.
pub struct SelfHosted;

impl Transform for SelfHosted {
    fn name(&self) -> &str {
        "self_hosted"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("json.timestamp") && event.get("json.timestamp").is_some_and(|v| v.is_string()) };
            if _cond {
            if let Some(v) = event.get("json.timestamp").cloned() {
                event.set("_tmp.timestamp", v)?;
            }
            }

            let _cond = { event.has_value("json.timestamp") && event.get("json.timestamp").is_some_and(|v| v.is_object()) && event.has_value("json.timestamp.epochSecond") && event.has_value("json.timestamp.nano") };
            if _cond {
            event.set("_tmp.timestamp", json!(format!("{}.{}", event.get("json.timestamp.epochSecond").map_or_else(String::new, template_to_string), event.get("json.timestamp.nano").map_or_else(String::new, template_to_string))))?;
            }

                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }

                if event.has_value("json.source") {
                    event.rename("json.source", "source.address")?;
                }

                if event.has_value("json.author.id") {
                    event.rename("json.author.id", "user.id")?;
                }

                if event.has_value("json.author.name") {
                    event.rename("json.author.name", "user.full_name")?;
                }

            let _cond = { event.get_str("json.author.uri") != Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.author.uri") {
                if let Some(input) = event.get_string("json.author.uri") {
                    // Grok pattern: \\?username=%{USER:user.name}$
                    if !cached_grok!("\\?username=%{USER:user.name}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }
                Ok(())
            })();
            }

                if event.has_value("json.auditType") {
                    event.rename("json.auditType", "confluence.audit.type")?;
                }

                if event.has_value("json.type") {
                    event.rename("json.type", "confluence.audit.type")?;
                }

                if event.has_value("json.method") {
                    event.rename("json.method", "confluence.audit.method")?;
                }

                if event.has_value("json.system") {
                    event.rename("json.system", "service.address")?;
                }

            let _cond = { event.has_value("service.address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "service.address", "_tmp.service", true, false)?;
                Ok(())
            })();
            }

                if event.has_value("json.extraAttributes") {
                    event.rename("json.extraAttributes", "confluence.audit.extra_attributes")?;
                }

                if event.has_value("json.changedValues") {
                    event.rename("json.changedValues", "confluence.audit.changed_values")?;
                }

                if event.has_value("json.affectedObjects") {
                    event.rename("json.affectedObjects", "confluence.audit.affected_objects")?;
                }

            if let Some(v) = event.get("confluence.audit.type.actionI18nKey").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.action", v)?;
            }

            let _cond = { event.has_value("_tmp.service.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("_tmp.service.domain").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
