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
            event.remove("logstash.pipeline.total.flow.queue_backpressure.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.queue_backpressure.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.queue_backpressure.current");
            }

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.queue_backpressure.last_1_minute")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.queue_backpressure.last_1_minute");
            }

            event.remove("logstash.pipeline.total.flow.queue_backpressure.last_5_minutes");

            event.remove("logstash.pipeline.total.flow.queue_backpressure.last_15_minutes");

            event.remove("logstash.pipeline.total.flow.queue_backpressure.last_1_hour");

            event.remove("logstash.pipeline.total.flow.queue_backpressure.last_24_hours");

            event.remove("logstash.pipeline.total.flow.queue_persisted_growth_bytes.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.queue_persisted_growth_bytes.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.queue_persisted_growth_bytes.current");
            }

            let _cond = {
                event.get_str(
                    "logstash.pipeline.total.flow.queue_persisted_growth_bytes.last_1_minute",
                ) == Some("Infinity")
            };
            if _cond {
                event.remove(
                    "logstash.pipeline.total.flow.queue_persisted_growth_bytes.last_1_minute",
                );
            }

            event
                .remove("logstash.pipeline.total.flow.queue_persisted_growth_bytes.last_5_minutes");

            event.remove(
                "logstash.pipeline.total.flow.queue_persisted_growth_bytes.last_15_minutes",
            );

            event.remove("logstash.pipeline.total.flow.queue_persisted_growth_bytes.last_1_hour");

            event.remove("logstash.pipeline.total.flow.queue_persisted_growth_bytes.last_24_hours");

            event.remove("logstash.pipeline.total.flow.queue_persisted_growth_events.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.queue_persisted_growth_events.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.queue_persisted_growth_events.current");
            }

            let _cond = {
                event.get_str(
                    "logstash.pipeline.total.flow.queue_persisted_growth_events.last_1_minute",
                ) == Some("Infinity")
            };
            if _cond {
                event.remove(
                    "logstash.pipeline.total.flow.queue_persisted_growth_events.last_1_minute",
                );
            }

            event.remove(
                "logstash.pipeline.total.flow.queue_persisted_growth_events.last_5_minutes",
            );

            event.remove(
                "logstash.pipeline.total.flow.queue_persisted_growth_events.last_15_minutes",
            );

            event.remove("logstash.pipeline.total.flow.queue_persisted_growth_events.last_1_hour");

            event
                .remove("logstash.pipeline.total.flow.queue_persisted_growth_events.last_24_hours");

            event.remove("logstash.pipeline.total.flow.worker_concurrency.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.worker_concurrency.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.worker_concurrency.current");
            }

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.worker_concurrency.last_1_minute")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.worker_concurrency.last_1_minute");
            }

            event.remove("logstash.pipeline.total.flow.worker_concurrency.last_5_minutes");

            event.remove("logstash.pipeline.total.flow.worker_concurrency.last_15_minutes");

            event.remove("logstash.pipeline.total.flow.worker_concurrency.last_1_hour");

            event.remove("logstash.pipeline.total.flow.worker_concurrency.last_24_hours");

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.worker_concurrency.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.worker_concurrency.current");
            }

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.worker_utilization.last_1_minute")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.worker_utilization.last_1_minute");
            }

            event.remove("logstash.pipeline.total.flow.worker_utilization.last_5_minutes");

            event.remove("logstash.pipeline.total.flow.worker_utilization.last_15_minutes");

            event.remove("logstash.pipeline.total.flow.worker_utilization.last_1_hour");

            event.remove("logstash.pipeline.total.flow.worker_utilization.last_24_hours");

            event.remove("logstash.pipeline.total.flow.filter_throughput.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.filter_throughput.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.filter_throughput.current");
            }

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.filter_throughput.last_1_minute")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.filter_throughput.last_1_minute");
            }

            event.remove("logstash.pipeline.total.flow.filter_throughput.last_5_minutes");

            event.remove("logstash.pipeline.total.flow.filter_throughput.last_15_minutes");

            event.remove("logstash.pipeline.total.flow.filter_throughput.last_1_hour");

            event.remove("logstash.pipeline.total.flow.filter_throughput.last_24_hours");

            event.remove("logstash.pipeline.total.flow.output_throughput.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.output_throughput.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.output_throughput.current");
            }

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.output_throughput.last_1_minute")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.output_throughput.last_1_minute");
            }

            event.remove("logstash.pipeline.total.flow.output_throughput.last_5_minutes");

            event.remove("logstash.pipeline.total.flow.output_throughput.last_15_minutes");

            event.remove("logstash.pipeline.total.flow.output_throughput.last_1_hour");

            event.remove("logstash.pipeline.total.flow.output_throughput.last_24_hours");

            event.remove("logstash.pipeline.total.flow.input_throughput.lifetime");

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.input_throughput.current")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.input_throughput.current");
            }

            let _cond = {
                event.get_str("logstash.pipeline.total.flow.input_throughput.last_1_minute")
                    == Some("Infinity")
            };
            if _cond {
                event.remove("logstash.pipeline.total.flow.input_throughput.last_1_minute");
            }

            event.remove("logstash.pipeline.total.flow.input_throughput.last_5_minutes");

            event.remove("logstash.pipeline.total.flow.input_throughput.last_15_minutes");

            event.remove("logstash.pipeline.total.flow.input_throughput.last_1_hour");

            event.remove("logstash.pipeline.total.flow.input_throughput.last_24_hours");

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
