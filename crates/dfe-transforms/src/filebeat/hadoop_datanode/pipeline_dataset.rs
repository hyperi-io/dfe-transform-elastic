// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_dataset` pipeline.
pub struct PipelineDataset;

impl Transform for PipelineDataset {
    fn name(&self) -> &str {
        "pipeline_dataset"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.CacheCapacity") {
                    event.rename("hadoop.datanode.temp.CacheCapacity", "hadoop.datanode.cache.capacity")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.CacheUsed") {
                    event.rename("hadoop.datanode.temp.CacheUsed", "hadoop.datanode.cache.used")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.NumBlocksCached") {
                    event.rename("hadoop.datanode.temp.NumBlocksCached", "hadoop.datanode.blocks.cached")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.EstimatedCapacityLostTotal") {
                    event.rename("hadoop.datanode.temp.EstimatedCapacityLostTotal", "hadoop.datanode.estimated_capacity_lost_total")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.Capacity") {
                    event.rename("hadoop.datanode.temp.Capacity", "hadoop.datanode.disk_space.capacity")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.Remaining") {
                    event.rename("hadoop.datanode.temp.Remaining", "hadoop.datanode.disk_space.remaining")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.DfsUsed") {
                    event.rename("hadoop.datanode.temp.DfsUsed", "hadoop.datanode.dfs_used")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.NumBlocksFailedToCache") {
                    event.rename("hadoop.datanode.temp.NumBlocksFailedToCache", "hadoop.datanode.blocks.failed.to_cache")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.NumBlocksFailedToUnCache") {
                    event.rename("hadoop.datanode.temp.NumBlocksFailedToUnCache", "hadoop.datanode.blocks.failed.to_uncache")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("hadoop.datanode.temp.NumFailedVolumes") {
                    event.rename("hadoop.datanode.temp.NumFailedVolumes", "hadoop.datanode.volumes.failed")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("hadoop.datanode.temp.LastVolumeFailureDate") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("hadoop.datanode.last_volume_failure_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "hadoop.datanode.temp.LastVolumeFailureDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
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
                event.set("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
