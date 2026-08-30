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

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("kafka_connect"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.kafka_connect") {
                    event.rename("jolokia.kafka_connect", "kafka_connect")?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("kafka_connect.client.id") };
            if _cond {
                if let Some(input) = event.get_string("kafka_connect.mbean") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("kafka.connect:client-id=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(",") else {
                            break 'dissect false;
                        };
                        captured.push(("kafka_connect.client.id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(",") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "kafka_connect.mbean".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
