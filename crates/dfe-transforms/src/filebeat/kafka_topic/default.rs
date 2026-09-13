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
            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event
                        .get_str("jolokia.metrics.mbean")
                        .is_some_and(|s| s.starts_with("kafka.cluster:type=Partition"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.cluster:type=Partition,name=%{NOTSPACE:metric_name},topic=%{NOTSPACE:topic_name},partition=%{NUMBER:partition_id}
                        if !cached_grok!("kafka.cluster:type=Partition,name=%{NOTSPACE:metric_name},topic=%{NOTSPACE:topic_name},partition=%{NUMBER:partition_id}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event
                        .get_str("jolokia.metrics.mbean")
                        .is_some_and(|s| s.starts_with("kafka.log:type=Log"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.log:type=Log,name=%{NOTSPACE:metric_name},topic=%{NOTSPACE:topic_name},partition=%{NUMBER:partition_id}
                        if !cached_grok!("kafka.log:type=Log,name=%{NOTSPACE:metric_name},topic=%{NOTSPACE:topic_name},partition=%{NUMBER:partition_id}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("jolokia.metrics.mbean")
                    && event
                        .get_str("jolokia.metrics.mbean")
                        .is_some_and(|s| s.starts_with("kafka.server:type=BrokerTopicMetrics"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("jolokia.metrics.mbean") {
                        // Grok pattern: kafka.server:type=BrokerTopicMetrics,name=%{NOTSPACE:metric_name},topic=%{NOTSPACE:topic_name}
                        if !cached_grok!("kafka.server:type=BrokerTopicMetrics,name=%{NOTSPACE:metric_name},topic=%{NOTSPACE:topic_name}").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("topic_name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("topic_name").cloned() {
                        event.set("jolokia.metrics.topic.name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("partition_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("partition_id").cloned() {
                        event.set("jolokia.metrics.partition.id", v)?;
                    }
                    Ok(())
                })();
            }

            event.remove("topic_name");

            event.remove("partition_id");

            event.remove("metric_name");

            event.set("ecs.version", json!("8.11.0"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("jolokia.metrics") {
                    event.rename("jolokia.metrics", "kafka.topic")?;
                }
                Ok(())
            })();

            event.set("event.kind", json!("metric"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("service.type", json!("kafka"))?;

            // Painless script
            // Source: def queue = new ArrayList();\ndef fingerprint = new ArrayList();\n\nif (ctx.containsKey('kafka') && ctx.kafka.containsKey('topic')) {\n    queue.add(['p': 'kafka.topic', 'v': ctx.kafka.topic]);\n\n    while (!queue.isEmpty()) {\n        def item = queue.remove(0);\n        def path = item.p;\n        def val = item.v;\n\n        if (val instanceof Map) {\n            for (entry in val.entrySet()) {\n                def key = entry.getKey();\n                def child = entry.getValue();\n                def childPath = path + '.' + key;\n                queue.add(['p': childPath, 'v': child]);\n            }\n        } else {\n            fingerprint.add(path);\n        }\n    }\n\n    ctx.kafka_topic_metric_fingerprint = fingerprint;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def queue = new ArrayList();\ndef fingerprint = new ArrayList();\n\nif (ctx.containsKey('kafka') && ctx.kafka.containsKey('topic')) {\n    queue.add(['p': 'kafka.topic', 'v': ctx.kafka.topic]);\n\n    while (!queue.isEmpty()) {\n        def item = queue.remove(0);\n        def path = item.p;\n        def val = item.v;\n\n        if (val instanceof Map) {\n            for (entry in val.entrySet()) {\n                def key = entry.getKey();\n                def child = entry.getValue();\n                def childPath = path + '.' + key;\n                queue.add(['p': childPath, 'v': child]);\n            }\n        } else {\n            fingerprint.add(path);\n        }\n    }\n\n    ctx.kafka_topic_metric_fingerprint = fingerprint;\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("kafka_topic_metric_fingerprint") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "kafka_topic_metric_fingerprint".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set(
                            "kafka.topic.metric_fingerprint",
                            json!(fingerprint_default(&values)),
                        )?;
                    }
                }
                Ok(())
            })();

            event.remove("kafka_topic_metric_fingerprint");

            event.remove("kafka.topic.mbean");

            event.remove("jolokia.metrics");

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
