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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond =
                { event.has_value("event.original") && !event.has_value("kubernetes.audit") };
            if _cond {
                parse_json_field(event, "event.original", "kubernetes.audit")?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = {
                event.has_value("kubernetes.audit.category")
                    && event.has_value("kubernetes.audit.operationName")
                    && event.has_value("kubernetes.audit.resourceId")
                    && event.has_value("kubernetes.audit.time")
            };
            if _cond {
                if event.has_value("kubernetes.audit") {
                    event.rename("kubernetes.audit", "tmp.aks_audit")?;
                }
            }

            let _cond = {
                event
                    .get("tmp.aks_audit.properties.log")
                    .is_some_and(|v| v.is_string())
                    && !event.has_value("kubernetes.audit")
            };
            if _cond {
                parse_json_field(event, "tmp.aks_audit.properties.log", "kubernetes.audit")?;
            }

            let _cond = {
                event.has_value("tmp.aks_audit.properties.log")
                    && !event.has_value("kubernetes.audit")
            };
            if _cond {
                event.rename("tmp.aks_audit.properties.log", "kubernetes.audit")?;
            }

            event.remove("tmp.aks_audit.properties.log");

            if event.has_value("tmp.aks_audit") {
                event.rename("tmp.aks_audit", "kubernetes.audit.aks_metadata")?;
            }

            let _cond = { event.has_value("kubernetes.audit.aks_metadata.time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("kubernetes.audit.aks_metadata.time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "kubernetes.audit.aks_metadata.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("kubernetes.audit.aks_metadata.operationName") {
                event.rename(
                    "kubernetes.audit.aks_metadata.operationName",
                    "kubernetes.audit.aks_metadata.operation_name",
                )?;
            }

            if event.has_value("kubernetes.audit.aks_metadata.resourceId") {
                event.rename(
                    "kubernetes.audit.aks_metadata.resourceId",
                    "kubernetes.audit.aks_metadata.resource_id",
                )?;
            }

            if event.has_value("kubernetes.audit.aks_metadata.serviceBuild") {
                event.rename(
                    "kubernetes.audit.aks_metadata.serviceBuild",
                    "kubernetes.audit.aks_metadata.service_build",
                )?;
            }

            if event.has_value("kubernetes.audit.aks_metadata.properties.containerID") {
                event.rename(
                    "kubernetes.audit.aks_metadata.properties.containerID",
                    "kubernetes.audit.aks_metadata.container_id",
                )?;
            }

            if event.has_value("kubernetes.audit.aks_metadata.properties.pod") {
                event.rename(
                    "kubernetes.audit.aks_metadata.properties.pod",
                    "kubernetes.audit.aks_metadata.pod",
                )?;
            }

            if event.has_value("kubernetes.audit.aks_metadata.properties.stream") {
                event.rename(
                    "kubernetes.audit.aks_metadata.properties.stream",
                    "kubernetes.audit.aks_metadata.stream",
                )?;
            }

            let _cond = { event.has_value("kubernetes.audit.responseObject") };
            if _cond {
                event.remove("kubernetes.audit.responseObject.metadata");
            }

            let _cond = { event.has_value("kubernetes.audit.requestObject") };
            if _cond {
                event.remove("kubernetes.audit.requestObject.metadata");
            }

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("kubernetes.audit.verb") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.verb")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            let _cond = { !event.has_value("kubernetes.audit.auditID") };
            if _cond {
                if event.has_value("kubernetes.audit.insertId") {
                    event.rename("kubernetes.audit.insertId", "kubernetes.audit.auditID")?;
                }
            }

            let _cond = { !event.has_value("kubernetes.audit.requestReceivedTimestamp") };
            if _cond {
                if event.has_value("kubernetes.audit.receiveTimestamp") {
                    event.rename(
                        "kubernetes.audit.receiveTimestamp",
                        "kubernetes.audit.requestReceivedTimestamp",
                    )?;
                }
            }

            let _cond = { !event.has_value("kubernetes.audit.stageTimestamp") };
            if _cond {
                if event.has_value("kubernetes.audit.timestamp") {
                    event.rename(
                        "kubernetes.audit.timestamp",
                        "kubernetes.audit.stageTimestamp",
                    )?;
                }
            }

            let _cond = { !event.has_value("kubernetes.audit.responseStatus.code") };
            if _cond {
                if event.has_value("kubernetes.audit.protoPayload.status.code") {
                    event.rename(
                        "kubernetes.audit.protoPayload.status.code",
                        "kubernetes.audit.responseStatus.code",
                    )?;
                }
            }

            let _cond = { !event.has_value("kubernetes.audit.annotations") };
            if _cond {
                if event.has_value("kubernetes.audit.labels") {
                    event.rename("kubernetes.audit.labels", "kubernetes.audit.annotations")?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.annotations") };
            if _cond {
                // Painless script
                // Source: Map annotations = ctx['kubernetes']['audit']['annotations']; Map updatedAnnotation = new HashMap(); for (String key: annotations.keySet()) {\n    updatedAnnotation[key.replace('.', '_')] = annotations[key];\n    if (key == 'authorization.k8s.io/decision') {\n        if (annotations['authorization.k8s.io/decision'] == 'allow') {\n            ctx.event.outcome = 'success';\n        } else if (annotations['authorization.k8s.io/decision'] == 'forbid') {\n            ctx.event.outcome = 'failure';\n        }\n    }\n} ctx['kubernetes']['audit']['annotations'] = updatedAnnotation
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Map annotations = ctx['kubernetes']['audit']['annotations']; Map updatedAnnotation = new HashMap(); for (String key: annotations.keySet()) {\n    updatedAnnotation[key.replace('.', '_')] = annotations[key];\n    if (key == 'authorization.k8s.io/decision') {\n        if (annotations['authorization.k8s.io/decision'] == 'allow') {\n            ctx.event.outcome = 'success';\n        } else if (annotations['authorization.k8s.io/decision'] == 'forbid') {\n            ctx.event.outcome = 'failure';\n        }\n    }\n} ctx['kubernetes']['audit']['annotations'] = updatedAnnotation"#
                    ),
                )?;
            }

            let _cond = { event.has_value("kubernetes.audit.user.username") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.user.username")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.user.uid") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.user.uid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.userAgent") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.userAgent")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user_agent.original", v)?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.sourceIPs") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("kubernetes.audit.sourceIPs") {
                        if let Some(val) = event.get("kubernetes.audit.sourceIPs") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "kubernetes.audit.sourceIPs".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                if let Some(v) = event
                    .get("source.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client.ip", v)?;
                }
            }

            event.set("orchestrator.type", json!("kubernetes"))?;

            let _cond = { event.has_value("kubernetes.audit.apiVersion") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.apiVersion")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.api_version", v)?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.objectRef.resource") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.objectRef.resource")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.resource.type", v)?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.objectRef.namespace") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.objectRef.namespace")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.namespace", v)?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.objectRef.name") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.objectRef.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.resource.name", v)?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.protoPayload.resourceName") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.protoPayload.resourceName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("orchestrator.resource.name") {
                        event.set("orchestrator.resource.name", v)?;
                    }
                }
            }

            let _cond = { event.has_value("kubernetes.audit.resource.type") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.resource.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.resource.type", v)?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.resource.labels.cluster_name") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.resource.labels.cluster_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.cluster.name", v)?;
                }
            }

            let _cond = { event.has_value("kubernetes.audit.resource.labels.project_id") };
            if _cond {
                if let Some(v) = event
                    .get("kubernetes.audit.resource.labels.project_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.cluster.id", v)?;
                }
            }

            let _cond = { event.get_str("input.type") == Some("aws-cloudwatch") };
            if _cond {
                event.set("cloud.provider", json!("aws"))?;
            }

            let _cond = { event.get_str("input.type") == Some("azure-eventhub") };
            if _cond {
                event.set("cloud.provider", json!("azure"))?;
            }

            let _cond = { event.get_str("input.type") == Some("gcp-pubsub") };
            if _cond {
                event.set("cloud.provider", json!("gcp"))?;
            }

            let _cond = { event.has_value("kubernetes.audit.user.uid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("kubernetes.audit.user.uid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("kubernetes.audit.user.username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("kubernetes.audit.user.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.ip") && event.get("source.ip").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "source.ip", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}failed with message '{}",
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
