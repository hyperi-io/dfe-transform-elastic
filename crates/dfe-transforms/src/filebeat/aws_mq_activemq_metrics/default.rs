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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "", "*")?;
                Ok(())
            })();

            let _cond = {
                event.get("aws.amazonmq.metrics").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 1) && event.has_value("aws.amazonmq.metrics.ConsumerCount.max")
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.has_value("aws.dimensions.Broker")
                    && (!event.has_value("aws.dimensions.Topic")
                        && !event.has_value("aws.dimensions.Queue"))
            };
            if _cond {
                if event.has_value("aws.amazonmq.metrics") {
                    event.rename(
                        "aws.amazonmq.metrics",
                        "aws.amazonmq.metrics.activemq.broker",
                    )?;
                }
            }

            let _cond = {
                event.has_value("aws.dimensions.Broker")
                    && (event.has_value("aws.dimensions.Topic")
                        || event.has_value("aws.dimensions.Queue"))
            };
            if _cond {
                if event.has_value("aws.amazonmq.metrics") {
                    event.rename(
                        "aws.amazonmq.metrics",
                        "aws.amazonmq.metrics.activemq.destination",
                    )?;
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.set(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
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
