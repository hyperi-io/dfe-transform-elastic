// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_network_action` pipeline.
pub struct PipelineNetworkAction;

impl Transform for PipelineNetworkAction {
    fn name(&self) -> &str {
        "pipeline_network_action"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.category", Value::Array(vec![json!("network")]))?;

            let _cond = {
                event.get_str("sentinel_one_cloud_funnel.event.type") == Some("IPConnect")
                    || event.get_str("sentinel_one_cloud_funnel.event.type") == Some("IP Connect")
            };
            if _cond {
                event.set(
                    "event.type",
                    Value::Array(vec![json!("start"), json!("connection")]),
                )?;
            }

            let _cond = {
                (event.get_str("sentinel_one_cloud_funnel.event.type") == Some("IPConnect")
                    || event.get_str("sentinel_one_cloud_funnel.event.type") == Some("IP Connect"))
                    && event.get_str("json.event.network.direction") == Some("OUTGOING")
            };
            if _cond {
                event.set(
                    "event.action",
                    Value::Array(vec![json!("connection_attempted")]),
                )?;
            }

            let _cond = {
                (event.get_str("sentinel_one_cloud_funnel.event.type") == Some("IPConnect")
                    || event.get_str("sentinel_one_cloud_funnel.event.type") == Some("IP Connect"))
                    && event.get_str("json.event.network.direction") == Some("INCOMING")
            };
            if _cond {
                event.set(
                    "event.action",
                    Value::Array(vec![json!("connection_accepted")]),
                )?;
            }

            if event.has_value("json.k8sCluster.containerId") {
                event.rename(
                    "json.k8sCluster.containerId",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.container.id",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.id", v)?;
            }

            if event.has_value("json.k8sCluster.containerImage.sha256") {
                event.rename(
                    "json.k8sCluster.containerImage.sha256",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256",
                )?;
            }

            let _cond = {
                event
                    .has_value("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256")
            };
            if _cond {
                event.append_unique(
                    "container.image.hash.all",
                    json!(
                        event
                            .get(
                                "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256"
                            )
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .has_value("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get(
                                "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.sha256"
                            )
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.k8sCluster.containerImage.value") {
                event.rename(
                    "json.k8sCluster.containerImage.value",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.image.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.image.name", v)?;
            }

            if event.has_value("json.k8sCluster.containerLabels") {
                event.rename(
                    "json.k8sCluster.containerLabels",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.container.labels",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.labels")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.labels", v)?;
            }

            if event.has_value("json.k8sCluster.containerName") {
                event.rename(
                    "json.k8sCluster.containerName",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.container.name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.k8s_cluster.container.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.name", v)?;
            }

            let _cond = { event.get_str("json.dst.ip.address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.dst.ip.address") {
                        if let Some(val) = event.get("json.dst.ip.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dst.ip.address".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("sentinel_one_cloud_funnel.event.dst.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_dst_ip_address",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.dst.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.dst.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.dst.ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.dst.ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.address", v)?;
            }

            let _cond = { event.get_str("json.dst.port.number") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.dst.port.number") {
                        if let Some(val) = event.get("json.dst.port.number") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dst.port.number".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.dst.port_number",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_dst_port_number",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.dst.port_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.event.network.connectionStatus") {
                event.rename(
                    "json.event.network.connectionStatus",
                    "sentinel_one_cloud_funnel.event.network.connection_status",
                )?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.network.connection_status")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.network.connection_status")
                        .is_some_and(|s| s.to_lowercase() == "blocked")
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = {
                event.get_str("sentinel_one_cloud_funnel.event.network.connection_status")
                    != Some("")
                    && event.get_str("event.outcome") != Some("unknown")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sentinel_one_cloud_funnel.event.network.connection_status")
                    {
                        map_strings(
                            event,
                            "sentinel_one_cloud_funnel.event.network.connection_status",
                            "event.outcome",
                            str::to_lowercase,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "lowercase")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "lowercase_sentinel_one_cloud_funnel_event_network_connection_status",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.event.network.direction") {
                event.rename(
                    "json.event.network.direction",
                    "sentinel_one_cloud_funnel.event.network.direction",
                )?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.network.direction")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.network.direction")
                        .is_some_and(|s| s.to_lowercase() == "incoming")
            };
            if _cond {
                event.set("network.direction", json!("ingress"))?;
            }

            let _cond = {
                event.has_value("sentinel_one_cloud_funnel.event.network.direction")
                    && event
                        .get_str("sentinel_one_cloud_funnel.event.network.direction")
                        .is_some_and(|s| s.to_lowercase() == "outgoing")
            };
            if _cond {
                event.set("network.direction", json!("egress"))?;
            }

            if event.has_value("json.event.network.protocolName") {
                event.rename(
                    "json.event.network.protocolName",
                    "sentinel_one_cloud_funnel.event.network.protocol_name",
                )?;
            }

            let _cond = {
                event.get_str("sentinel_one_cloud_funnel.event.network.protocol_name") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("sentinel_one_cloud_funnel.event.network.protocol_name") {
                        map_strings(
                            event,
                            "sentinel_one_cloud_funnel.event.network.protocol_name",
                            "network.protocol",
                            str::to_lowercase,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "lowercase")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "lowercase_sentinel_one_cloud_funnel_event_network_protocol_name",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.src.ip.address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.ip.address") {
                        if let Some(val) = event.get("json.src.ip.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.ip.address".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("sentinel_one_cloud_funnel.event.src.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_ip_address",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("sentinel_one_cloud_funnel.event.src.ip.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sentinel_one_cloud_funnel.event.src.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one_cloud_funnel.event.src.ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.get_str("json.src.port.number") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src.port.number") {
                        if let Some(val) = event.get("json.src.port.number") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src.port.number".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "sentinel_one_cloud_funnel.event.src.port_number",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_src_port_number",
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
                                .get("_ingest.pipeline")
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
                .get("sentinel_one_cloud_funnel.event.src.port_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.k8sCluster.controllerLabels") {
                event.rename(
                    "json.k8sCluster.controllerLabels",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.controller.labels",
                )?;
            }

            if event.has_value("json.k8sCluster.controllerName") {
                event.rename(
                    "json.k8sCluster.controllerName",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.controller.name",
                )?;
            }

            if event.has_value("json.k8sCluster.controllerType") {
                event.rename(
                    "json.k8sCluster.controllerType",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.controller.type",
                )?;
            }

            if event.has_value("json.k8sCluster.name") {
                event.rename(
                    "json.k8sCluster.name",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.name",
                )?;
            }

            if event.has_value("json.k8sCluster.namespace") {
                event.rename(
                    "json.k8sCluster.namespace",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.value",
                )?;
            }

            if event.has_value("json.k8sCluster.namespaceLabels") {
                event.rename(
                    "json.k8sCluster.namespaceLabels",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.namespace.labels",
                )?;
            }

            if event.has_value("json.k8sCluster.nodeName") {
                event.rename(
                    "json.k8sCluster.nodeName",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.node_name",
                )?;
            }

            if event.has_value("json.k8sCluster.podLabels") {
                event.rename(
                    "json.k8sCluster.podLabels",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.pod.labels",
                )?;
            }

            if event.has_value("json.k8sCluster.podName") {
                event.rename(
                    "json.k8sCluster.podName",
                    "sentinel_one_cloud_funnel.event.k8s_cluster.pod.name",
                )?;
            }

            let _cond = {
                (event.has_value("source.ip") || event.has_value("source.address"))
                    && event.has_value("source.port")
                    && (event.has_value("destination.ip") || event.has_value("destination.address"))
                    && event.has_value("destination.port")
            };
            if _cond {
                event.append_unique("event.type", json!("connection"))?;
            }

            if !event.has("event.type") {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
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
