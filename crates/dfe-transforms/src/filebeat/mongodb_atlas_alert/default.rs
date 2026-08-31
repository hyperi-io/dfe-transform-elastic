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
            {
                let mut values = Vec::new();
                if let Some(v) = event.get("response.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("alert"))?;

            event.set("event.module", json!("mongodb_atlas"))?;

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("error message set and no data to process.").to_string(),
                });
            }

            let _cond = {
                !event.has_value("event.original")
                    && (event.has_value("tags")
                        && (event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_original_event")),
                            serde_json::Value::String(s) => s.contains("preserve_original_event"),
                            _ => false,
                        })))
            };
            if _cond {
                if let Some(v) = event
                    .get("response")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.original", v)?;
                }
            }

            if event.has_value("event.original") {
                if let Some(val) = event.get("event.original") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "event.original".into(),
                            message,
                        }
                    })?;
                    event.set("event.original", converted)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("response.created") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "response.created".into(),
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("response.id") {
                event.rename("response.id", "event.id")?;
            }

            if event.has_value("response.groupId") {
                event.rename("response.groupId", "group.id")?;
            }

            if event.has_value("response.orgId") {
                event.rename("response.orgId", "organization.id")?;
            }

            if event.has_value("response.acknowledgedUntil") {
                event.rename(
                    "response.acknowledgedUntil",
                    "mongodb_atlas.alert.acknowledged.time",
                )?;
            }

            if event.has_value("response.acknowledgementComment") {
                event.rename(
                    "response.acknowledgementComment",
                    "mongodb_atlas.alert.acknowledged.comment",
                )?;
            }

            if event.has_value("response.acknowledgingUsername") {
                event.rename(
                    "response.acknowledgingUsername",
                    "mongodb_atlas.alert.acknowledged.user.name",
                )?;
            }

            if event.has_value("response.alertConfigId") {
                event.rename("response.alertConfigId", "mongodb_atlas.alert.config.id")?;
            }

            if event.has_value("response.parentClusterId") {
                event.rename(
                    "response.parentClusterId",
                    "mongodb_atlas.alert.cluster.parent.id",
                )?;
            }

            if event.has_value("response.clusterId") {
                event.rename("response.clusterId", "mongodb_atlas.alert.cluster.id")?;
            }

            if event.has_value("response.clusterName") {
                event.rename("response.clusterName", "mongodb_atlas.alert.cluster.name")?;
            }

            if event.has_value("response.currentValue.number") {
                event.rename(
                    "response.currentValue.number",
                    "mongodb_atlas.alert.metric.value",
                )?;
            }

            if event.has_value("response.currentValue.units") {
                event.rename(
                    "response.currentValue.units",
                    "mongodb_atlas.alert.metric.unit",
                )?;
            }

            if event.has_value("response.metricName") {
                event.rename("response.metricName", "mongodb_atlas.alert.metric.name")?;
            }

            if event.has_value("response.eventTypeName") {
                event.rename(
                    "response.eventTypeName",
                    "mongodb_atlas.alert.event_type.name",
                )?;
            }

            if event.has_value("response.nonRunningHostIds") {
                event.rename(
                    "response.nonRunningHostIds",
                    "mongodb_atlas.alert.host.non_running.ids",
                )?;
            }

            if event.has_value("response.hostId") {
                event.rename("response.hostId", "mongodb_atlas.alert.host.id")?;
            }

            if event.has_value("response.hostnameAndPort") {
                event.rename(
                    "response.hostnameAndPort",
                    "mongodb_atlas.alert.host_name_and_port",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("mongodb_atlas.alert.host_name_and_port") {
                    if let Some(input) = event.get_string("mongodb_atlas.alert.host_name_and_port")
                    {
                        // Grok pattern: %{HOSTNAME:source.address}:%{POSINT:source.port}
                        if !cached_grok!("%{HOSTNAME:source.address}:%{POSINT:source.port}")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Failed to parse host_name_and_port: {}",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.port") {
                    if let Some(val) = event.get("source.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_address")?;
                if let Some(v) = event
                    .get("source.address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.domain", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("response.lastNotified") {
                event.rename(
                    "response.lastNotified",
                    "mongodb_atlas.alert.last_notified.time",
                )?;
            }

            if event.has_value("response.replicaSetName") {
                event.rename(
                    "response.replicaSetName",
                    "mongodb_atlas.alert.replicaset.name",
                )?;
            }

            if event.has_value("response.resolved") {
                event.rename("response.resolved", "mongodb_atlas.alert.resolved.time")?;
            }

            if event.has_value("response.status") {
                event.rename("response.status", "mongodb_atlas.alert.status")?;
            }

            if event.has_value("response.sourceTypeName") {
                event.rename(
                    "response.sourceTypeName",
                    "mongodb_atlas.alert.source_type.name",
                )?;
            }

            if event.has_value("response.updated") {
                event.rename("response.updated", "mongodb_atlas.alert.updated.time")?;
            }

            if event.has_value("response.instanceName") {
                event.rename(
                    "response.instanceName",
                    "mongodb_atlas.alert.processor.instance.name",
                )?;
            }

            if event.has_value("response.processorErrorMsg") {
                event.rename(
                    "response.processorErrorMsg",
                    "mongodb_atlas.alert.processor.error_msg",
                )?;
            }

            if event.has_value("response.processorName") {
                event.rename(
                    "response.processorName",
                    "mongodb_atlas.alert.processor.name",
                )?;
            }

            if event.has_value("response.processorState") {
                event.rename(
                    "response.processorState",
                    "mongodb_atlas.alert.processor.state",
                )?;
            }

            if event.has_value("response.userAlias") {
                event.rename("response.userAlias", "mongodb_atlas.alert.user.alias")?;
            }

            if event.has_value("response.tags") {
                event.rename("response.tags", "mongodb_atlas.alert.tags")?;
            }

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            event.remove("response");

            let _cond = { event.has_value("error.response") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
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
