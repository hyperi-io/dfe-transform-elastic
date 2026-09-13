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
                if event.has_value("prometheus") {
                    event.rename("prometheus", "influxdb.advstatus")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.labels.instance") {
                    event.rename(
                        "influxdb.advstatus.labels.instance",
                        "influxdb.advstatus.instance",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.labels.org") {
                    event.rename("influxdb.advstatus.labels.org", "influxdb.advstatus.org")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.labels.taskID") {
                    event.rename(
                        "influxdb.advstatus.labels.taskID",
                        "influxdb.advstatus.labels.taskid",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.qc_all_duration_seconds.histogram") {
                    event.rename(
                        "influxdb.advstatus.qc_all_duration_seconds.histogram",
                        "influxdb.advstatus.query_controller.all_duration_seconds.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.qc_compiling_duration_seconds.histogram") {
                    event.rename(
                        "influxdb.advstatus.qc_compiling_duration_seconds.histogram",
                        "influxdb.advstatus.query_controller.compiling_duration_seconds.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.qc_executing_duration_seconds.histogram") {
                    event.rename(
                        "influxdb.advstatus.qc_executing_duration_seconds.histogram",
                        "influxdb.advstatus.query_controller.executing_duration_seconds.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.qc_executing_duration_seconds.histogram") {
                    event.rename(
                        "influxdb.advstatus.qc_executing_duration_seconds.histogram",
                        "influxdb.advstatus.query_controller.executing_duration_seconds.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event
                    .has_value("influxdb.advstatus.storage_compactions_duration_seconds.histogram")
                {
                    event.rename(
                        "influxdb.advstatus.storage_compactions_duration_seconds.histogram",
                        "influxdb.advstatus.storage.compactions_duration_seconds.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.storage_retention_check_duration.histogram")
                {
                    event.rename(
                        "influxdb.advstatus.storage_retention_check_duration.histogram",
                        "influxdb.advstatus.storage.retention_check_duration.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.storage_writer_dropped_points.histogram") {
                    event.rename(
                        "influxdb.advstatus.storage_writer_dropped_points.histogram",
                        "influxdb.advstatus.storage.writer_dropped_points.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.storage_writer_err_points.histogram") {
                    event.rename(
                        "influxdb.advstatus.storage_writer_err_points.histogram",
                        "influxdb.advstatus.storage.writer_err_points.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.storage_writer_ok_points.histogram") {
                    event.rename(
                        "influxdb.advstatus.storage_writer_ok_points.histogram",
                        "influxdb.advstatus.storage.writer_ok_points.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.storage_writer_req_points.histogram") {
                    event.rename(
                        "influxdb.advstatus.storage_writer_req_points.histogram",
                        "influxdb.advstatus.storage.writer_req_points.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.task_executor_run_latency_seconds.histogram")
                {
                    event.rename(
                        "influxdb.advstatus.task_executor_run_latency_seconds.histogram",
                        "influxdb.advstatus.tasks.executor_run_latency_seconds.histogram",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.advstatus.task_executor_run_queue_delta.histogram") {
                    event.rename(
                        "influxdb.advstatus.task_executor_run_queue_delta.histogram",
                        "influxdb.advstatus.tasks.executor_run_queue_delta.histogram",
                    )?;
                }
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
