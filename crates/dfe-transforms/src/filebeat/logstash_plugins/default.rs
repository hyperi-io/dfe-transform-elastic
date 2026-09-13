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
            event.remove("logstash.pipeline.plugin.input.flow.throughput.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.plugin.input.flow.throughput.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.plugin.input.flow.throughput.current");
            }

            let _cond = {
                event.get_str("logstash.pipeline.plugin.input.flow.throughput.last_1_minute")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.plugin.input.flow.throughput.last_1_minute");
            }

            event.remove("logstash.pipeline.plugin.input.flow.throughput.last_5_minutes");

            event.remove("logstash.pipeline.plugin.input.flow.throughput.last_15_minutes");

            event.remove("logstash.pipeline.plugin.input.flow.throughput.last_1_hour");

            event.remove("logstash.pipeline.plugin.input.flow.throughput.last_24_hours");

            event.remove("logstash.pipeline.plugin.filter.flow.worker_millis_per_event.lifetime");

            let _cond = {
                event
                    .get_str("logstash.pipeline.plugin.filter.flow.worker_millis_per_event.current")
                    == Some("Infinity")
            };
            if _cond {
                event
                    .remove("logstash.pipeline.plugin.filter.flow.worker_millis_per_event.current");
            }

            let _cond = {
                event.get_str(
                    "logstash.pipeline.plugin.filter.flow.worker_millis_per_event.last_1_minute",
                ) == Some("Infinity")
            };
            if _cond {
                event.remove(
                    "logstash.pipeline.plugin.filter.flow.worker_millis_per_event.last_1_minute",
                );
            }

            event.remove(
                "logstash.pipeline.plugin.filter.flow.worker_millis_per_event.last_5_minutes",
            );

            event.remove(
                "logstash.pipeline.plugin.filter.flow.worker_millis_per_event.last_15_minutes",
            );

            event
                .remove("logstash.pipeline.plugin.filter.flow.worker_millis_per_event.last_1_hour");

            event.remove(
                "logstash.pipeline.plugin.filter.flow.worker_millis_per_event.last_24_hours",
            );

            event.remove("logstash.pipeline.plugin.filter.flow.worker_utilization.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.plugin.filter.flow.worker_utilization.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.plugin.filter.flow.worker_utilization.current");
            }

            let _cond = {
                event.get_str(
                    "logstash.pipeline.plugin.filter.flow.worker_utilization.last_1_minute",
                ) == Some("Infinity")
            };
            if _cond {
                event.remove(
                    "logstash.pipeline.plugin.filter.flow.worker_utilization.last_1_minute",
                );
            }

            event.remove("logstash.pipeline.plugin.filter.flow.worker_utilization.last_5_minutes");

            event.remove("logstash.pipeline.plugin.filter.flow.worker_utilization.last_15_minutes");

            event.remove("logstash.pipeline.plugin.filter.flow.worker_utilization.last_1_hour");

            event.remove("logstash.pipeline.plugin.filter.flow.worker_utilization.last_24_hours");

            event.remove("logstash.pipeline.plugin.output.flow.worker_millis_per_event.lifetime");

            let _cond = {
                event
                    .get_str("logstash.pipeline.plugin.output.flow.worker_millis_per_event.current")
                    == Some("Infinity")
            };
            if _cond {
                event
                    .remove("logstash.pipeline.plugin.output.flow.worker_millis_per_event.current");
            }

            let _cond = {
                event.get_str(
                    "logstash.pipeline.plugin.output.flow.worker_millis_per_event.last_1_minute",
                ) == Some("Infinity")
            };
            if _cond {
                event.remove(
                    "logstash.pipeline.plugin.output.flow.worker_millis_per_event.last_1_minute",
                );
            }

            event.remove(
                "logstash.pipeline.plugin.output.flow.worker_millis_per_event.last_5_minutes",
            );

            event.remove(
                "logstash.pipeline.plugin.output.flow.worker_millis_per_event.last_15_minutes",
            );

            event
                .remove("logstash.pipeline.plugin.output.flow.worker_millis_per_event.last_1_hour");

            event.remove(
                "logstash.pipeline.plugin.output.flow.worker_millis_per_event.last_24_hours",
            );

            event.remove("logstash.pipeline.plugin.output.flow.worker_utilization.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.plugin.output.flow.worker_utilization.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.plugin.output.flow.worker_utilization.current");
            }

            let _cond = {
                event.get_str(
                    "logstash.pipeline.plugin.output.flow.worker_utilization.last_1_minute",
                ) == Some("Infinity")
            };
            if _cond {
                event.remove(
                    "logstash.pipeline.plugin.output.flow.worker_utilization.last_1_minute",
                );
            }

            event.remove("logstash.pipeline.plugin.output.flow.worker_utilization.last_5_minutes");

            event.remove("logstash.pipeline.plugin.output.flow.worker_utilization.last_15_minutes");

            event.remove("logstash.pipeline.plugin.output.flow.worker_utilization.last_1_hour");

            event.remove("logstash.pipeline.plugin.output.flow.worker_utilization.last_24_hours");

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
