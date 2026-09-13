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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("metric"))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            event.set("event.module", json!("spring_boot"))?;

            event.set("event.dataset", json!("spring_boot.gc"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageBeforeGc.PS Eden Space") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.PS Eden Space",
                        "spring_boot.gc.last_info.memory_usage.before.ps_eden_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageBeforeGc.Code Cache") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.Code Cache",
                        "spring_boot.gc.last_info.memory_usage.before.code_cache",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageBeforeGc.CodeCache") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.CodeCache",
                        "spring_boot.gc.last_info.memory_usage.before.code_cache",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "jolokia.metrics.last_info.memoryUsageBeforeGc.Compressed Class Space",
                ) {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.Compressed Class Space",
                        "spring_boot.gc.last_info.memory_usage.before.compressed_class_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("jolokia.metrics.last_info.memoryUsageBeforeGc.PS Survivor Space")
                {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.PS Survivor Space",
                        "spring_boot.gc.last_info.memory_usage.before.ps_survivor_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageBeforeGc.PS Old Gen") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.PS Old Gen",
                        "spring_boot.gc.last_info.memory_usage.before.ps_old_gen",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageBeforeGc.Metaspace") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.Metaspace",
                        "spring_boot.gc.last_info.memory_usage.before.metaspace",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageBeforeGc.G1 Old Gen") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.G1 Old Gen",
                        "spring_boot.gc.last_info.memory_usage.before.g1_old_gen",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("jolokia.metrics.last_info.memoryUsageBeforeGc.G1 Survivor Space")
                {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.G1 Survivor Space",
                        "spring_boot.gc.last_info.memory_usage.before.g1_survivor_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageBeforeGc.G1 Eden Space") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageBeforeGc.G1 Eden Space",
                        "spring_boot.gc.last_info.memory_usage.before.g1_eden_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageAfterGc.PS Eden Space") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.PS Eden Space",
                        "spring_boot.gc.last_info.memory_usage.after.ps_eden_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageAfterGc.Code Cache") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.Code Cache",
                        "spring_boot.gc.last_info.memory_usage.after.code_cache",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value(
                    "jolokia.metrics.last_info.memoryUsageAfterGc.Compressed Class Space",
                ) {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.Compressed Class Space",
                        "spring_boot.gc.last_info.memory_usage.after.compressed_class_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageAfterGc.PS Survivor Space")
                {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.PS Survivor Space",
                        "spring_boot.gc.last_info.memory_usage.after.ps_survivor_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageAfterGc.PS Old Gen") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.PS Old Gen",
                        "spring_boot.gc.last_info.memory_usage.after.ps_old_gen",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageAfterGc.G1 Eden Space") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.G1 Eden Space",
                        "spring_boot.gc.last_info.memory_usage.after.g1_eden_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageAfterGc.CodeCache") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.CodeCache",
                        "spring_boot.gc.last_info.memory_usage.after.code_cache",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageAfterGc.G1 Old Gen") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.G1 Old Gen",
                        "spring_boot.gc.last_info.memory_usage.after.g1_old_gen",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageAfterGc.G1 Survivor Space")
                {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.G1 Survivor Space",
                        "spring_boot.gc.last_info.memory_usage.after.g1_survivor_space",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.memoryUsageAfterGc.Metaspace") {
                    event.rename(
                        "jolokia.metrics.last_info.memoryUsageAfterGc.Metaspace",
                        "spring_boot.gc.last_info.memory_usage.after.metaspace",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.GcThreadCount") {
                    event.rename(
                        "jolokia.metrics.last_info.GcThreadCount",
                        "spring_boot.gc.last_info.thread_count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.startTime") {
                    event.rename(
                        "jolokia.metrics.last_info.startTime",
                        "spring_boot.gc.last_info.time.start",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.endTime") {
                    event.rename(
                        "jolokia.metrics.last_info.endTime",
                        "spring_boot.gc.last_info.time.end",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.id") {
                    event.rename(
                        "jolokia.metrics.last_info.id",
                        "spring_boot.gc.last_info.id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.last_info.duration") {
                    event.rename(
                        "jolokia.metrics.last_info.duration",
                        "spring_boot.gc.last_info.time.duration",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics.name") {
                    event.rename("jolokia.metrics.name", "spring_boot.gc.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("jolokia");
                Ok(())
            })();

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
