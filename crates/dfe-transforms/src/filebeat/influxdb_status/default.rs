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
            if event.has_value("prometheus.metrics") {
                event.rename("prometheus.metrics", "influxdb.status")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.go_memstats_alloc_bytes") {
                    event.rename(
                        "influxdb.status.go_memstats_alloc_bytes",
                        "influxdb.status.go_runtime.memstats_alloc_bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.go_memstats_alloc_bytes_total") {
                    event.rename(
                        "influxdb.status.go_memstats_alloc_bytes_total",
                        "influxdb.status.go_runtime.memstats_alloc_bytes_total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.go_memstats_heap_alloc_bytes") {
                    event.rename(
                        "influxdb.status.go_memstats_heap_alloc_bytes",
                        "influxdb.status.go_runtime.memstats_heap_alloc_bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.go_memstats_heap_idle_bytes") {
                    event.rename(
                        "influxdb.status.go_memstats_heap_idle_bytes",
                        "influxdb.status.go_runtime.memstats_heap_idle_bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.go_memstats_heap_inuse_bytes") {
                    event.rename(
                        "influxdb.status.go_memstats_heap_inuse_bytes",
                        "influxdb.status.go_runtime.memstats_heap_inuse_bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.go_threads") {
                    event.rename(
                        "influxdb.status.go_threads",
                        "influxdb.status.go_runtime.threads",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.qc_all_active") {
                    event.rename(
                        "influxdb.status.qc_all_active",
                        "influxdb.status.query_controller.all_active",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.qc_compiling_active") {
                    event.rename(
                        "influxdb.status.qc_compiling_active",
                        "influxdb.status.query_controller.compiling_active",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.qc_executing_active") {
                    event.rename(
                        "influxdb.status.qc_executing_active",
                        "influxdb.status.query_controller.executing_active",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_bucket_measurement_num") {
                    event.rename(
                        "influxdb.status.storage_bucket_measurement_num",
                        "influxdb.status.storage.bucket_measurement_num",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_bucket_series_num") {
                    event.rename(
                        "influxdb.status.storage_bucket_series_num",
                        "influxdb.status.storage.bucket_series_num",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_compactions_failed") {
                    event.rename(
                        "influxdb.status.storage_compactions_failed",
                        "influxdb.status.storage.compactions_failed",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_shard_disk_size") {
                    event.rename(
                        "influxdb.status.storage_shard_disk_size",
                        "influxdb.status.storage.shard_disk_size",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_shard_write_count") {
                    event.rename(
                        "influxdb.status.storage_shard_write_count",
                        "influxdb.status.storage.shard_write_count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_shard_write_dropped_sum") {
                    event.rename(
                        "influxdb.status.storage_shard_write_dropped_sum",
                        "influxdb.status.storage.shard_write_dropped_sum",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_shard_write_err_count") {
                    event.rename(
                        "influxdb.status.storage_shard_write_err_count",
                        "influxdb.status.storage.shard_write_err_count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_tsm_files_disk_bytes") {
                    event.rename(
                        "influxdb.status.storage_tsm_files_disk_bytes",
                        "influxdb.status.storage.tsm_files_disk_bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_tsm_files_total") {
                    event.rename(
                        "influxdb.status.storage_tsm_files_total",
                        "influxdb.status.storage.tsm_files_total",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_wal_size") {
                    event.rename(
                        "influxdb.status.storage_wal_size",
                        "influxdb.status.storage.wal_size",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_wal_writes") {
                    event.rename(
                        "influxdb.status.storage_wal_writes",
                        "influxdb.status.storage.wal_writes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_wal_writes_err") {
                    event.rename(
                        "influxdb.status.storage_wal_writes_err",
                        "influxdb.status.storage.wal_writes_err",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.storage_writer_timeouts") {
                    event.rename(
                        "influxdb.status.storage_writer_timeouts",
                        "influxdb.status.storage.writer_timeouts",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_executor_errors_counter") {
                    event.rename(
                        "influxdb.status.task_executor_errors_counter",
                        "influxdb.status.tasks.executor_errors_counter",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_executor_total_runs_active") {
                    event.rename(
                        "influxdb.status.task_executor_total_runs_active",
                        "influxdb.status.tasks.executor_total_runs_active",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_executor_total_runs_complete") {
                    event.rename(
                        "influxdb.status.task_executor_total_runs_complete",
                        "influxdb.status.tasks.executor_total_runs_complete",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_executor_workers_busy") {
                    event.rename(
                        "influxdb.status.task_executor_workers_busy",
                        "influxdb.status.tasks.executor_workers_busy",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_scheduler_current_execution") {
                    event.rename(
                        "influxdb.status.task_scheduler_current_execution",
                        "influxdb.status.tasks.scheduler_current_execution",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_scheduler_total_execute_failure") {
                    event.rename(
                        "influxdb.status.task_scheduler_total_execute_failure",
                        "influxdb.status.tasks.scheduler_total_execute_failure",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_scheduler_total_execution_calls") {
                    event.rename(
                        "influxdb.status.task_scheduler_total_execution_calls",
                        "influxdb.status.tasks.scheduler_total_execution_calls",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_scheduler_total_release_calls") {
                    event.rename(
                        "influxdb.status.task_scheduler_total_release_calls",
                        "influxdb.status.tasks.scheduler_total_release_calls",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_scheduler_total_schedule_calls") {
                    event.rename(
                        "influxdb.status.task_scheduler_total_schedule_calls",
                        "influxdb.status.tasks.scheduler_total_schedule_calls",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("influxdb.status.task_scheduler_total_schedule_fails") {
                    event.rename(
                        "influxdb.status.task_scheduler_total_schedule_fails",
                        "influxdb.status.tasks.scheduler_total_schedule_fails",
                    )?;
                }
                Ok(())
            })();

            if event.has_value("prometheus.labels.instance") {
                event.rename("prometheus.labels.instance", "influxdb.status.instance")?;
            }

            if event.has_value("prometheus.labels.method") {
                event.rename(
                    "prometheus.labels.method",
                    "influxdb.status.http_api.method",
                )?;
            }

            if event.has_value("prometheus.labels.path") {
                event.rename("prometheus.labels.path", "influxdb.status.http_api.path")?;
            }

            if event.has_value("prometheus.labels.status") {
                event.rename(
                    "prometheus.labels.status",
                    "influxdb.status.http_api.http_status",
                )?;
            }

            if event.has_value("prometheus.labels.response_code") {
                event.rename(
                    "prometheus.labels.response_code",
                    "influxdb.status.http_api.response_code",
                )?;
            }

            if event.has_value("influxdb.status.influxdb_buckets_total") {
                event.rename(
                    "influxdb.status.influxdb_buckets_total",
                    "influxdb.status.buckets_total",
                )?;
            }

            if event.has_value("influxdb.status.influxdb_dashboards_total") {
                event.rename(
                    "influxdb.status.influxdb_dashboards_total",
                    "influxdb.status.dashboards_total",
                )?;
            }

            if event.has_value("influxdb.status.influxdb_organizations_total") {
                event.rename(
                    "influxdb.status.influxdb_organizations_total",
                    "influxdb.status.organizations_total",
                )?;
            }

            if event.has_value("influxdb.status.influxdb_scrapers_total") {
                event.rename(
                    "influxdb.status.influxdb_scrapers_total",
                    "influxdb.status.scrapers_total",
                )?;
            }

            if event.has_value("influxdb.status.influxdb_tokens_total") {
                event.rename(
                    "influxdb.status.influxdb_tokens_total",
                    "influxdb.status.tokens_total",
                )?;
            }

            if event.has_value("influxdb.status.influxdb_uptime_seconds") {
                event.rename(
                    "influxdb.status.influxdb_uptime_seconds",
                    "influxdb.status.uptime_seconds",
                )?;
            }

            if event.has_value("influxdb.status.influxdb_users_total") {
                event.rename(
                    "influxdb.status.influxdb_users_total",
                    "influxdb.status.users_total",
                )?;
            }

            if event.has_value("prometheus.labels.org") {
                event.rename("prometheus.labels.org", "influxdb.status.org")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("prometheus.labels") {
                    event.rename("prometheus.labels", "influxdb.status.label")?;
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
