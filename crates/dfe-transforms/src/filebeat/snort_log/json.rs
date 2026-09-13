// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `json` pipeline.
pub struct Json;

impl Transform for Json {
    fn name(&self) -> &str {
        "json"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                parse_json_field(event, "event.original", "json")?;

                event.remove("json.b64_data");

                if event.has_value("json.timestamp") {
                    event.rename("json.timestamp", "_tmp.timestamp")?;
                }

            if event.has_value("json.src_port") {
                if let Some(val) = event.get("json.src_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.src_port".into(),
                            message,
                        })?;
                    event.set("source.port", converted)?;
                }
            }

            if event.has_value("json.dst_port") {
                if let Some(val) = event.get("json.dst_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dst_port".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }

                if event.has_value("json.dst_addr") {
                    event.rename("json.dst_addr", "destination.address")?;
                }

                if event.has_value("json.src_addr") {
                    event.rename("json.src_addr", "source.address")?;
                }

                if event.has_value("json.eth_dst") {
                    event.rename("json.eth_dst", "destination.mac")?;
                }

                if event.has_value("json.eth_src") {
                    event.rename("json.eth_src", "source.mac")?;
                }

            if event.has_value("json.eth_len") {
                if let Some(val) = event.get("json.eth_len") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.eth_len".into(),
                            message,
                        })?;
                    event.set("snort.eth.length", converted)?;
                }
            }

                if event.has_value("json.class") {
                    event.rename("json.class", "rule.category")?;
                }

                if event.has_value("json.msg") {
                    event.rename("json.msg", "rule.description")?;
                }

            if event.has_value("json.rev") {
                if let Some(val) = event.get("json.rev") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.rev".into(),
                            message,
                        })?;
                    event.set("rule.version", converted)?;
                }
            }

            if event.has_value("json.sid") {
                if let Some(val) = event.get("json.sid") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.sid".into(),
                            message,
                        })?;
                    event.set("rule.id", converted)?;
                }
            }

            if event.has_value("json.gid") {
                if let Some(val) = event.get("json.gid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.gid".into(),
                            message,
                        })?;
                    event.set("snort.gid", converted)?;
                }
            }

            if event.has_value("json.icmp_type") {
                if let Some(val) = event.get("json.icmp_type") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.icmp_type".into(),
                            message,
                        })?;
                    event.set("snort.icmp.type", converted)?;
                }
            }

            if event.has_value("json.icmp_code") {
                if let Some(val) = event.get("json.icmp_code") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.icmp_code".into(),
                            message,
                        })?;
                    event.set("snort.icmp.code", converted)?;
                }
            }

            if event.has_value("json.icmp_id") {
                if let Some(val) = event.get("json.icmp_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.icmp_id".into(),
                            message,
                        })?;
                    event.set("snort.icmp.id", converted)?;
                }
            }

            if event.has_value("json.icmp_seq") {
                if let Some(val) = event.get("json.icmp_seq") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.icmp_seq".into(),
                            message,
                        })?;
                    event.set("snort.icmp.seq", converted)?;
                }
            }

                if event.has_value("json.tcp_flags") {
                    event.rename("json.tcp_flags", "snort.tcp.flags")?;
                }

                if event.has_value("json.tcp_len") {
                    event.rename("json.tcp_len", "snort.tcp.length")?;
                }

                if event.has_value("json.tcp_seq") {
                    event.rename("json.tcp_seq", "snort.tcp.seq")?;
                }

                if event.has_value("json.tcp_ack") {
                    event.rename("json.tcp_ack", "snort.tcp.ack")?;
                }

                if event.has_value("json.tcp_win") {
                    event.rename("json.tcp_win", "snort.tcp.window")?;
                }

                if event.has_value("json.udp_len") {
                    event.rename("json.udp_len", "snort.udp.length")?;
                }

            if event.has_value("json.ip_id") {
                if let Some(val) = event.get("json.ip_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.ip_id".into(),
                            message,
                        })?;
                    event.set("snort.ip.id", converted)?;
                }
            }

            if event.has_value("json.tos") {
                if let Some(val) = event.get("json.tos") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.tos".into(),
                            message,
                        })?;
                    event.set("snort.ip.tos", converted)?;
                }
            }

            if event.has_value("json.ttl") {
                if let Some(val) = event.get("json.ttl") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.ttl".into(),
                            message,
                        })?;
                    event.set("snort.ip.ttl", converted)?;
                }
            }

            if event.has_value("json.pkt_num") {
                if let Some(val) = event.get("json.pkt_num") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.pkt_num".into(),
                            message,
                        })?;
                    event.set("network.packets", converted)?;
                }
            }

            if event.has_value("json.pkt_len") {
                if let Some(val) = event.get("json.pkt_len") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.pkt_len".into(),
                            message,
                        })?;
                    event.set("network.bytes", converted)?;
                }
            }

                if event.has_value("json.proto") {
                    event.rename("json.proto", "network.transport")?;
                }

            let _cond = { event.get_str("json.service") != Some("unknown") };
            if _cond {
                if event.has_value("json.service") {
                    event.rename("json.service", "network.protocol")?;
                }
            }

            let _cond = { event.get_i64("json.vlan") != Some(0) };
            if _cond {
            if event.has_value("json.vlan") {
                if let Some(val) = event.get("json.vlan") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.vlan".into(),
                            message,
                        })?;
                    event.set("network.vlan.id", converted)?;
                }
            }
            }

            if event.has_value("json.priority") {
                if let Some(val) = event.get("json.priority") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.priority".into(),
                            message,
                        })?;
                    event.set("event.severity", converted)?;
                }
            }

                if event.has_value("json.action") {
                    event.rename("json.action", "_tmp.action")?;
                }

                if event.has_value("json.iface") {
                    event.rename("json.iface", "observer.ingress.interface.name")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
