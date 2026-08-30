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

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("apache_tomcat"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            let _cond = { event.get_i64("prometheus.metrics.java_lang_Memory_Verbose") == Some(1) };
            if _cond {
                event.set("apache_tomcat.memory.verbose", json!(true))?;
            }

            let _cond = { event.get_i64("prometheus.metrics.java_lang_Memory_Verbose") == Some(0) };
            if _cond {
                event.set("apache_tomcat.memory.verbose", json!(false))?;
            }

            if event.has_value("prometheus.metrics.java_lang_Memory_HeapMemoryUsage_max") {
                event.rename(
                    "prometheus.metrics.java_lang_Memory_HeapMemoryUsage_max",
                    "apache_tomcat.memory.heap.max.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Memory_HeapMemoryUsage_init") {
                event.rename(
                    "prometheus.metrics.java_lang_Memory_HeapMemoryUsage_init",
                    "apache_tomcat.memory.heap.init.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Memory_HeapMemoryUsage_used") {
                event.rename(
                    "prometheus.metrics.java_lang_Memory_HeapMemoryUsage_used",
                    "apache_tomcat.memory.heap.used.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Memory_HeapMemoryUsage_committed") {
                event.rename(
                    "prometheus.metrics.java_lang_Memory_HeapMemoryUsage_committed",
                    "apache_tomcat.memory.heap.committed.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Memory_NonHeapMemoryUsage_max") {
                event.rename(
                    "prometheus.metrics.java_lang_Memory_NonHeapMemoryUsage_max",
                    "apache_tomcat.memory.non_heap.max.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Memory_NonHeapMemoryUsage_init") {
                event.rename(
                    "prometheus.metrics.java_lang_Memory_NonHeapMemoryUsage_init",
                    "apache_tomcat.memory.non_heap.init.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Memory_NonHeapMemoryUsage_used") {
                event.rename(
                    "prometheus.metrics.java_lang_Memory_NonHeapMemoryUsage_used",
                    "apache_tomcat.memory.non_heap.used.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Memory_NonHeapMemoryUsage_committed") {
                event.rename(
                    "prometheus.metrics.java_lang_Memory_NonHeapMemoryUsage_committed",
                    "apache_tomcat.memory.non_heap.committed.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_Memory_ObjectPendingFinalizationCount")
            {
                event.rename(
                    "prometheus.metrics.java_lang_Memory_ObjectPendingFinalizationCount",
                    "apache_tomcat.memory.object_pending_finalization.count",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_G1_Old_Generation_CollectionCount") {
                event.rename(
                    "prometheus.metrics.java_lang_G1_Old_Generation_CollectionCount",
                    "apache_tomcat.memory.gc.collection.count",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_G1_Old_Generation_CollectionTime") {
                event.rename(
                    "prometheus.metrics.java_lang_G1_Old_Generation_CollectionTime",
                    "apache_tomcat.memory.gc.collection.time.ms",
                )?;
            }

            if event.has_value("prometheus.metrics.java_lang_G1_Old_Generation_Valid") {
                event.rename(
                    "prometheus.metrics.java_lang_G1_Old_Generation_Valid",
                    "apache_tomcat.memory.gc.valid",
                )?;
            }

            let _cond = {
                event.has_value("apache_tomcat.memory.heap")
                    || event.has_value("apache_tomcat.memory.non_heap")
            };
            if _cond {
                event.set("apache_tomcat.memory.doc_type", json!("memory"))?;
            }

            let _cond = { event.has_value("apache_tomcat.memory.gc") };
            if _cond {
                event.set("apache_tomcat.memory.doc_type", json!("gc"))?;
            }

            event.remove("prometheus");

            // Painless script
            // Source: boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n    if (o == null || o == \"\") {\n    return true;\n    } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n    } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n    }\n    return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

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
                event.append_unique("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
