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

            parse_json_field(event, "event.original", "_temp_")?;

            let _cond = { !event.has_value("_temp_.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.rename("_temp_", "zeek.dhcp")?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("ecs.version", json!("8.17.0"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("connection"))?;

            event.append("event.type", json!("protocol"))?;

            event.append("event.type", json!("info"))?;

            event.set("event.kind", json!("event"))?;

            event.set("network.transport", json!("udp"))?;

            event.set("network.protocol", json!("dhcp"))?;

            if event.has_value("zeek.dhcp.uids") {
                event.rename("zeek.dhcp.uids", "zeek.session_id")?;
            }

            if event.has_value("zeek.dhcp.assigned_addr") {
                event.rename("zeek.dhcp.assigned_addr", "zeek.dhcp.address.assigned")?;
            }

            if event.has_value("zeek.dhcp.client_addr") {
                event.rename("zeek.dhcp.client_addr", "zeek.dhcp.address.client")?;
            }

            if event.has_value("zeek.dhcp.mac") {
                event.rename("zeek.dhcp.mac", "zeek.dhcp.address.mac")?;
            }

            if event.has_value("zeek.dhcp.requested_addr") {
                event.rename("zeek.dhcp.requested_addr", "zeek.dhcp.address.requested")?;
            }

            if event.has_value("zeek.dhcp.server_addr") {
                event.rename("zeek.dhcp.server_addr", "zeek.dhcp.address.server")?;
            }

            if event.has_value("zeek.dhcp.host_name") {
                event.rename("zeek.dhcp.host_name", "zeek.dhcp.hostname")?;
            }

            if event.has_value("zeek.dhcp.client_message") {
                event.rename("zeek.dhcp.client_message", "zeek.dhcp.msg.client")?;
            }

            if event.has_value("zeek.dhcp.server_message") {
                event.rename("zeek.dhcp.server_message", "zeek.dhcp.msg.server")?;
            }

            if event.has_value("zeek.dhcp.msg_types") {
                event.rename("zeek.dhcp.msg_types", "zeek.dhcp.msg.types")?;
            }

            if event.has_value("zeek.dhcp.msg_orig") {
                event.rename("zeek.dhcp.msg_orig", "zeek.dhcp.msg.origin")?;
            }

            if event.has_value("zeek.dhcp.client_software") {
                event.rename("zeek.dhcp.client_software", "zeek.dhcp.software.client")?;
            }

            if event.has_value("zeek.dhcp.server_software") {
                event.rename("zeek.dhcp.server_software", "zeek.dhcp.software.server")?;
            }

            if event.has_value("zeek.dhcp.circuit_id") {
                event.rename("zeek.dhcp.circuit_id", "zeek.dhcp.id.circuit")?;
            }

            if event.has_value("zeek.dhcp.agent_remote_id") {
                event.rename("zeek.dhcp.agent_remote_id", "zeek.dhcp.id.remote_agent")?;
            }

            if event.has_value("zeek.dhcp.subscriber_id") {
                event.rename("zeek.dhcp.subscriber_id", "zeek.dhcp.id.subscriber")?;
            }

            if event.has_value("zeek.dhcp.client_port") {
                event.rename("zeek.dhcp.client_port", "source.port")?;
            }

            if event.has_value("zeek.dhcp.server_port") {
                event.rename("zeek.dhcp.server_port", "destination.port")?;
            }

            let _cond = { event.has_value("zeek.dhcp.domain") };
            if _cond {
                if let Some(v) = event.get("zeek.dhcp.domain").cloned() {
                    event.set("network.name", v)?;
                }
            }

            let _cond = { !event.has_value("source.port") };
            if _cond {
                event.set("source.port", json!(68))?;
            }

            let _cond = { !event.has_value("destination.port") };
            if _cond {
                event.set("destination.port", json!(67))?;
            }

            if let Some(v) = event
                .get("zeek.dhcp.address.client")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.address", v)?;
            }

            if let Some(v) = event
                .get("zeek.dhcp.address.client")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.address", v)?;
            }

            if let Some(v) = event
                .get("zeek.dhcp.address.client")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event
                .get("zeek.dhcp.address.server")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.address", v)?;
            }

            if let Some(v) = event
                .get("zeek.dhcp.address.server")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event
                .get("zeek.dhcp.address.server")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server.address", v)?;
            }

            if let Some(date_str) = event.get_as_string("zeek.dhcp.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.dhcp.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.dhcp.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.dhcp.ts".into(),
                });
            }

            let _cond = { event.has_value("zeek.session_id") };
            if _cond {
                if let Some(v) = event.get("zeek.session_id").cloned() {
                    event.set("event.id", v)?;
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

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Community ID v1 hash
            if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                event.get_string("source.ip"),
                event.get_string("destination.ip"),
                event
                    .get_as_string("network.iana_number")
                    .or_else(|| event.get_as_string("network.transport")),
            ) {
                let icmp = matches!(
                    protocol.to_ascii_lowercase().as_str(),
                    "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                );
                let (src_field, dst_field) = if icmp {
                    ("icmp.type", "icmp.code")
                } else {
                    ("source.port", "destination.port")
                };
                let src_port = u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                let dst_port = u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                    Ok(cid) => event.set("network.community_id", cid)?,
                    Err(message) => {
                        return Err(TransformError::ParseError {
                            path: "network.community_id".into(),
                            message,
                        });
                    }
                }
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

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
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
