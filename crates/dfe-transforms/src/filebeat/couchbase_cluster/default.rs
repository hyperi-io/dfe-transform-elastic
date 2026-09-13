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
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("info")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("metric");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("database")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.category", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("couchbase");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.maxBucketCount") {
                    event.rename(
                        "http.cluster.maxBucketCount",
                        "couchbase.cluster.buckets.max.count",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.hdd.free") {
                    event.rename(
                        "http.cluster.storageTotals.hdd.free",
                        "couchbase.cluster.hdd.free.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.hdd.quotaTotal") {
                    event.rename(
                        "http.cluster.storageTotals.hdd.quotaTotal",
                        "couchbase.cluster.hdd.quota.total.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.hdd.total") {
                    event.rename(
                        "http.cluster.storageTotals.hdd.total",
                        "couchbase.cluster.hdd.total.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.hdd.usedByData") {
                    event.rename(
                        "http.cluster.storageTotals.hdd.usedByData",
                        "couchbase.cluster.hdd.used.data.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.hdd.used") {
                    event.rename(
                        "http.cluster.storageTotals.hdd.used",
                        "couchbase.cluster.hdd.used.value.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.indexMemoryQuota") {
                    event.rename(
                        "http.cluster.indexMemoryQuota",
                        "couchbase.cluster.memory.quota.index.mb",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.memoryQuota") {
                    event.rename(
                        "http.cluster.memoryQuota",
                        "couchbase.cluster.memory.quota.mb",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.ram.quotaTotalPerNode") {
                    event.rename(
                        "http.cluster.storageTotals.ram.quotaTotalPerNode",
                        "couchbase.cluster.ram.quota.total.per_node.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.ram.quotaTotal") {
                    event.rename(
                        "http.cluster.storageTotals.ram.quotaTotal",
                        "couchbase.cluster.ram.quota.total.value.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.ram.quotaUsedPerNode") {
                    event.rename(
                        "http.cluster.storageTotals.ram.quotaUsedPerNode",
                        "couchbase.cluster.ram.quota.used.per_node.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.ram.quotaUsed") {
                    event.rename(
                        "http.cluster.storageTotals.ram.quotaUsed",
                        "couchbase.cluster.ram.quota.used.value.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.ram.total") {
                    event.rename(
                        "http.cluster.storageTotals.ram.total",
                        "couchbase.cluster.ram.total.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.ram.usedByData") {
                    event.rename(
                        "http.cluster.storageTotals.ram.usedByData",
                        "couchbase.cluster.ram.used.data.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("http.cluster.storageTotals.ram.used") {
                    event.rename(
                        "http.cluster.storageTotals.ram.used",
                        "couchbase.cluster.ram.used.value.bytes",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("http");
                Ok(())
            })();

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
