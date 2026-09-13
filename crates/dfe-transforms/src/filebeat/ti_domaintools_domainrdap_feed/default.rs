// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "domaintools")?;

            let _cond = { event.has_value("domaintools.raw_record.first_request_timestamp") };
            if _cond {
                if let Some(v) = event
                    .get("domaintools.raw_record.first_request_timestamp")
                    .cloned()
                {
                    event.set("domaintools.first_request_timestamp", v)?;
                }
            }

            foreach_array(event, "domaintools.raw_record.requests", |event| {
                event.append(
                    "domaintools.requests_url",
                    json!(
                        event
                            .get("_ingest._value.url")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;

            event.set("threat.indicator.type", json!("domain-name"))?;

            let _cond = { event.has_value("domaintools.domain") };
            if _cond {
                if let Some(v) = event.get("domaintools.domain").cloned() {
                    event.set("threat.indicator.name", v)?;
                }
            }

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            event.remove("domaintools.raw_record");
            event.remove("domaintools.parsed_record.parsed_fields.conformance");
            event.remove("domaintools.parsed_record.parsed_fields.dnssec");
            event.remove("domaintools.parsed_record.parsed_fields.domain");
            event.remove("domaintools.parsed_record.parsed_fields.domain_statuses");
            event.remove("domaintools.parsed_record.parsed_fields.links");
            event.remove("domaintools.parsed_record.parsed_fields.unclassified_emails");
            event.remove("domaintools.parsed_record.registrar_request_url");
            event.remove("domaintools.parsed_record.registry_request_url");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
