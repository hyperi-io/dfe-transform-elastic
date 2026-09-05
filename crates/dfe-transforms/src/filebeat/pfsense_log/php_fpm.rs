// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `php_fpm` pipeline.
pub struct PhpFpm;

impl Transform for PhpFpm {
    fn name(&self) -> &str {
        "php_fpm"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(input) = event.get_string("message") {
                // Grok pattern: ^%{DATA}: (?:((?:(%{DATA:_tmp.action}) for user '%{USER:user.name}' from: %{IP:source.address} \\(%{DATA}\\))|(?:User (%{DATA:_tmp.action}) for user '%{USER:user.name}' from: %{IP:source.address})|(?:webConfigurator %{DATA:_tmp.action} for user '%{DATA:user.name}' from: %{IP:source.address})))
                // Grok pattern: ^%{GREEDYDATA}
                if !extract_first_match(
                    &[
                        cached_grok!(
                            "^%{DATA}: (?:((?:(%{DATA:_tmp.action}) for user '%{USER:user.name}' from: %{IP:source.address} \\(%{DATA}\\))|(?:User (%{DATA:_tmp.action}) for user '%{USER:user.name}' from: %{IP:source.address})|(?:webConfigurator %{DATA:_tmp.action} for user '%{DATA:user.name}' from: %{IP:source.address})))"
                        ),
                        cached_grok!("^%{GREEDYDATA}"),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            event.append_unique("event.category", json!("authentication"))?;

            let _cond = {
                event
                    .get_str("_tmp.action")
                    .is_some_and(|s| s.to_lowercase().contains("success"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event
                    .get_str("_tmp.action")
                    .is_some_and(|s| s.to_lowercase().contains("authentication error"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has_value("observer.ip") {
                event.rename("observer.ip", "host.ip")?;
            }

            if event.has_value("observer.name") {
                event.rename("observer.name", "host.name")?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
