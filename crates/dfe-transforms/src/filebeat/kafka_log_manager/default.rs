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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics") {
                    event.rename("jolokia.metrics", "kafka.log_manager")?;
                }
                Ok(())
            })();

            event.set("event.kind", json!("metric"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("service.type", json!("kafka"))?;

            // Painless script
            // Source: def queue = new ArrayList();\ndef fingerprint = new ArrayList();\n\nif (ctx.containsKey('kafka') && ctx.kafka.containsKey('log_manager')) {\n    queue.add(['p': 'kafka.log_manager', 'v': ctx.kafka.log_manager]);\n\n    while (!queue.isEmpty()) {\n        def item = queue.remove(0);\n        def path = item.p;\n        def val = item.v;\n\n        if (val instanceof Map) {\n            for (entry in val.entrySet()) {\n                def key = entry.getKey();\n                def child = entry.getValue();\n                def childPath = path + '.' + key;\n                queue.add(['p': childPath, 'v': child]);\n            }\n        } else {\n            fingerprint.add(path);\n        }\n    }\n\n    ctx.kafka_log_manager_metric_fingerprint = fingerprint;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def queue = new ArrayList();\ndef fingerprint = new ArrayList();\n\nif (ctx.containsKey('kafka') && ctx.kafka.containsKey('log_manager')) {\n    queue.add(['p': 'kafka.log_manager', 'v': ctx.kafka.log_manager]);\n\n    while (!queue.isEmpty()) {\n        def item = queue.remove(0);\n        def path = item.p;\n        def val = item.v;\n\n        if (val instanceof Map) {\n            for (entry in val.entrySet()) {\n                def key = entry.getKey();\n                def child = entry.getValue();\n                def childPath = path + '.' + key;\n                queue.add(['p': childPath, 'v': child]);\n            }\n        } else {\n            fingerprint.add(path);\n        }\n    }\n\n    ctx.kafka_log_manager_metric_fingerprint = fingerprint;\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("kafka_log_manager_metric_fingerprint") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "kafka_log_manager_metric_fingerprint".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set(
                            "kafka.log_manager.metric_fingerprint",
                            json!(fingerprint_default(&values)),
                        )?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("kafka.log_manager.mbean")
                    && event
                        .get("kafka.log_manager.mbean")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("LogDirectoryOffline"))
                            }
                            serde_json::Value::String(s) => s.contains("LogDirectoryOffline"),
                            _ => false,
                        })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("kafka.log_manager.mbean") {
                        // Grok pattern: kafka.log:type=LogManager,name=LogDirectoryOffline,logDirectory=\"%{DATA:directory_offline_count_log_directory}\"
                        if !cached_grok!("kafka.log:type=LogManager,name=LogDirectoryOffline,logDirectory=\"%{DATA:directory_offline_count_log_directory}\"").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("directory_offline_count_log_directory") };
            if _cond {
                if let Some(v) = event.get("directory_offline_count_log_directory").cloned() {
                    event.set("kafka.log_manager.directory_offline_count.log_directory", v)?;
                }
            }

            let _cond = {
                event.has_value("kafka.log_manager.mbean")
                    && event
                        .get("kafka.log_manager.mbean")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("uncleanable-partitions-count")),
                            serde_json::Value::String(s) => {
                                s.contains("uncleanable-partitions-count")
                            }
                            _ => false,
                        })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("kafka.log_manager.mbean") {
                        // Grok pattern: kafka.log:type=LogCleanerManager,name=uncleanable-partitions-count,logDirectory=\"%{DATA:uncleanable_partitions_count_log_directory}\"
                        if !cached_grok!("kafka.log:type=LogCleanerManager,name=uncleanable-partitions-count,logDirectory=\"%{DATA:uncleanable_partitions_count_log_directory}\"").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("uncleanable_partitions_count_log_directory") };
            if _cond {
                if let Some(v) = event
                    .get("uncleanable_partitions_count_log_directory")
                    .cloned()
                {
                    event.set("kafka.log_manager.cleaner_manager.uncleanable_partitions_count.log_directory", v)?;
                }
            }

            let _cond = {
                event.has_value("kafka.log_manager.mbean")
                    && event
                        .get("kafka.log_manager.mbean")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("uncleanable-bytes"))
                            }
                            serde_json::Value::String(s) => s.contains("uncleanable-bytes"),
                            _ => false,
                        })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("kafka.log_manager.mbean") {
                        // Grok pattern: kafka.log:type=LogCleanerManager,name=uncleanable-bytes,logDirectory=\"%{DATA:uncleanable_bytes_log_directory}\"
                        if !cached_grok!("kafka.log:type=LogCleanerManager,name=uncleanable-bytes,logDirectory=\"%{DATA:uncleanable_bytes_log_directory}\"").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("uncleanable_bytes_log_directory") };
            if _cond {
                if let Some(v) = event.get("uncleanable_bytes_log_directory").cloned() {
                    event.set(
                        "kafka.log_manager.cleaner_manager.uncleanable_bytes.log_directory",
                        v,
                    )?;
                }
            }

            event.remove("directory_offline_count_log_directory");

            event.remove("uncleanable_partitions_count_log_directory");

            event.remove("uncleanable_bytes_log_directory");

            event.remove("kafka.log_manager.mbean");

            event.remove("kafka_log_manager_metric_fingerprint");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {} failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                " with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
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
