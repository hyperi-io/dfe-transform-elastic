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
            event.set("ecs.version", json!("8.16.0"))?;

            event.set("event.kind", json!("metric"))?;

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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "response")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "error.message",
                        json!("Received invalid JSON. Unable to parse the source log message"),
                    )?;
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("response.archiveJobs.count") {
                event.rename(
                    "response.archiveJobs.count",
                    "rubrik.monitoring_jobs.archive_jobs.count",
                )?;
            }

            if event.has_value("response.backupJobs.count") {
                event.rename(
                    "response.backupJobs.count",
                    "rubrik.monitoring_jobs.backup_jobs.count",
                )?;
            }

            if event.has_value("response.conversionJobs.count") {
                event.rename(
                    "response.conversionJobs.count",
                    "rubrik.monitoring_jobs.conversion_jobs.count",
                )?;
            }

            if event.has_value("response.logBackupJobs.count") {
                event.rename(
                    "response.logBackupJobs.count",
                    "rubrik.monitoring_jobs.log_backup_jobs.count",
                )?;
            }

            if event.has_value("response.recoveryJobs.count") {
                event.rename(
                    "response.recoveryJobs.count",
                    "rubrik.monitoring_jobs.recovery_jobs.count",
                )?;
            }

            if event.has_value("response.replicationJobs.count") {
                event.rename(
                    "response.replicationJobs.count",
                    "rubrik.monitoring_jobs.replication_jobs.count",
                )?;
            }

            if event.has_value("response.name") {
                event.rename("response.name", "rubrik.cluster.name")?;
            }

            if event.has_value("response.id") {
                event.rename("response.id", "rubrik.cluster.id")?;
            }

            if event.has_value("response.allJobs") {
                event.rename("response.allJobs", "rubrik.monitoring_jobs.all_jobs.count")?;
            }

            event.remove("response");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
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
