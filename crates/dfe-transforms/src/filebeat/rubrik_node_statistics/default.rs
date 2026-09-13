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

            if event.has_value("response.clusterName") {
                event.rename("response.clusterName", "rubrik.cluster.name")?;
            }

            if event.has_value("response.clusterId") {
                event.rename("response.clusterId", "rubrik.cluster.id")?;
            }

            if event.has_value("response.clusterType") {
                event.rename("response.clusterType", "rubrik.cluster.type")?;
            }

            if event.has_value("response.nodeId") {
                event.rename("response.nodeId", "rubrik.node_statistics.node_id")?;
            }

            if event.has_value("response.cpuStat") {
                event.rename("response.cpuStat", "rubrik.node_statistics.cpu_stat.pct")?;
            }

            if event.has_value("response.usedMemoryStat") {
                event.rename(
                    "response.usedMemoryStat",
                    "rubrik.node_statistics.used_memory.pct",
                )?;
            }

            if event.has_value("response.networkBytesReceived") {
                event.rename(
                    "response.networkBytesReceived",
                    "rubrik.node_statistics.network.received.bytes",
                )?;
            }

            if event.has_value("response.networkBytesTransmitted") {
                event.rename(
                    "response.networkBytesTransmitted",
                    "rubrik.node_statistics.network.transmitted.bytes",
                )?;
            }

            if event.has_value("response.iopsReadsPerSecond") {
                event.rename(
                    "response.iopsReadsPerSecond",
                    "rubrik.node_statistics.iops.reads",
                )?;
            }

            if event.has_value("response.iopsWritesPerSecond") {
                event.rename(
                    "response.iopsWritesPerSecond",
                    "rubrik.node_statistics.iops.writes",
                )?;
            }

            if event.has_value("response.readThroughputBytesPerSecond") {
                event.rename(
                    "response.readThroughputBytesPerSecond",
                    "rubrik.node_statistics.throughput.read.bytes",
                )?;
            }

            if event.has_value("response.writeThroughputBytesPerSecond") {
                event.rename(
                    "response.writeThroughputBytesPerSecond",
                    "rubrik.node_statistics.throughput.write.bytes",
                )?;
            }

            let _cond =
                { event.has_value("response.time") && event.get_str("response.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("response.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("rubrik.node_statistics.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "response.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("rubrik.node_statistics.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            event.remove("response");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);     \n
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
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}'",
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
                                "with tag '{}' ",
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
