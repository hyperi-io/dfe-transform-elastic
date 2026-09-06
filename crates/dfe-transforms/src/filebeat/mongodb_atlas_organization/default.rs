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

            event.set("event.kind", json!("event"))?;

            event.set("event.module", json!("mongodb_atlas"))?;

            event.set("event.dataset", json!("mongodb_atlas.organization"))?;

            event.set(
                "event.category",
                Value::Array(vec![json!("configuration"), json!("database")]),
            )?;

            event.set(
                "event.type",
                Value::Array(vec![json!("info"), json!("access"), json!("change")]),
            )?;

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

            if event.has_value("response.remoteAddress") {
                event.rename("response.remoteAddress", "client.ip")?;
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

            let _cond = { event.has_value("response.targetUsername") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("response.targetUsername")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("response.port") {
                event.rename("response.port", "server.port")?;
            }

            if event.has_value("response.userId") {
                event.rename("response.userId", "user.id")?;
            }

            if event.has_value("response.username") {
                event.rename("response.username", "user.name")?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\nif(ctx.response?.raw != null) {\n  ctx.response['raw'] = keysToSnakeCase(ctx.response.raw);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map keysToSnakeCase(Map m) {\n  def regex = /_?([a-z])([A-Z]+)/;\n  def snakeCaseMap = [:];\n\n  for (entry in m.entrySet()) {\n    def k = entry.getKey();\n    def v = entry.getValue();\n\n    if (v instanceof Map) {\n      v = keysToSnakeCase(v);\n    } else if (v instanceof List) {\n      for (int i = 0; i < v.size(); i++) {\n        def item = v.get(i);\n        if (item instanceof Map) {\n          v.set(i, keysToSnakeCase(item));\n        }\n      }\n    }\n\n    k = regex.matcher(k).replaceAll('$1_$2').toLowerCase();\n    snakeCaseMap.put(k, v);\n  }\n  return snakeCaseMap;\n}\nif(ctx.response?.raw != null) {\n  ctx.response['raw'] = keysToSnakeCase(ctx.response.raw);\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "additional-info-keys-to-snake-case",
                )?;
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

            if event.has_value("response.raw") {
                event.rename("response.raw", "mongodb_atlas.organization.additional_info")?;
            }

            if event.has_value("response.alertConfigId") {
                event.rename(
                    "response.alertConfigId",
                    "mongodb_atlas.organization.alert.config.id",
                )?;
            }

            if event.has_value("response.alertId") {
                event.rename("response.alertId", "mongodb_atlas.organization.alert.id")?;
            }

            if event.has_value("response.apiKeyId") {
                event.rename("response.apiKeyId", "mongodb_atlas.organization.api_key.id")?;
            }

            if event.has_value("response.clusterId") {
                event.rename(
                    "response.clusterId",
                    "mongodb_atlas.organization.cluster.id",
                )?;
            }

            if event.has_value("response.clusterName") {
                event.rename(
                    "response.clusterName",
                    "mongodb_atlas.organization.cluster.name",
                )?;
            }

            if event.has_value("response.collection") {
                event.rename(
                    "response.collection",
                    "mongodb_atlas.organization.collection.name",
                )?;
            }

            if event.has_value("response.currentValue.number") {
                event.rename(
                    "response.currentValue.number",
                    "mongodb_atlas.organization.metric.value",
                )?;
            }

            if event.has_value("response.currentValue.units") {
                event.rename(
                    "response.currentValue.units",
                    "mongodb_atlas.organization.metric.unit",
                )?;
            }

            if event.has_value("response.database") {
                event.rename(
                    "response.database",
                    "mongodb_atlas.organization.database.name",
                )?;
            }

            if event.has_value("response.eventTypeName") {
                event.rename(
                    "response.eventTypeName",
                    "mongodb_atlas.organization.event_type.name",
                )?;
            }

            if event.has_value("response.hostId") {
                event.rename("response.hostId", "mongodb_atlas.organization.host.id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("response.hostname") {
                    if let Some(val) = event.get("response.hostname") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "response.hostname".into(),
                                message,
                            }
                        })?;
                        event.set("response.hostname", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("response.hostname")
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

            let _cond = { !event.has_value("related.hosts") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("response.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("response.hostname") {
                event.rename("response.hostname", "mongodb_atlas.organization.host.name")?;
            }

            if event.has_value("response.invoiceId") {
                event.rename(
                    "response.invoiceId",
                    "mongodb_atlas.organization.invoice.id",
                )?;
            }

            if event.has_value("response.isGlobalAdmin") {
                event.rename(
                    "response.isGlobalAdmin",
                    "mongodb_atlas.organization.is_global_admin",
                )?;
            }

            if event.has_value("response.metricName") {
                event.rename(
                    "response.metricName",
                    "mongodb_atlas.organization.metric.name",
                )?;
            }

            if event.has_value("response.opType") {
                event.rename(
                    "response.opType",
                    "mongodb_atlas.organization.operation.type",
                )?;
            }

            if event.has_value("response.paymentId") {
                event.rename(
                    "response.paymentId",
                    "mongodb_atlas.organization.payment.id",
                )?;
            }

            if event.has_value("response.publicKey") {
                event.rename(
                    "response.publicKey",
                    "mongodb_atlas.organization.public_key",
                )?;
            }

            if event.has_value("response.replicaSetName") {
                event.rename(
                    "response.replicaSetName",
                    "mongodb_atlas.organization.replicaset.name",
                )?;
            }

            if event.has_value("response.resourceId") {
                event.rename(
                    "response.resourceId",
                    "mongodb_atlas.organization.resource.id",
                )?;
            }

            if event.has_value("response.resourceType") {
                event.rename(
                    "response.resourceType",
                    "mongodb_atlas.organization.resource.type",
                )?;
            }

            if event.has_value("response.shardName") {
                event.rename(
                    "response.shardName",
                    "mongodb_atlas.organization.shard.name",
                )?;
            }

            if event.has_value("response.targetPublicKey") {
                event.rename(
                    "response.targetPublicKey",
                    "mongodb_atlas.organization.target_public_key",
                )?;
            }

            if event.has_value("response.targetUsername") {
                event.rename(
                    "response.targetUsername",
                    "mongodb_atlas.organization.target.username",
                )?;
            }

            if event.has_value("response.teamId") {
                event.rename("response.teamId", "mongodb_atlas.organization.team.id")?;
            }

            if event.has_value("response.whitelistEntry") {
                event.rename(
                    "response.whitelistEntry",
                    "mongodb_atlas.organization.whitelist_entry",
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);
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

            event.remove("response");

            let _cond = { event.has_value("error.message") };
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
