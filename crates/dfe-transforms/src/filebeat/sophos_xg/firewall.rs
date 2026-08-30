// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `firewall` pipeline.
pub struct Firewall;

impl Transform for Firewall {
    fn name(&self) -> &str {
        "firewall"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("sophos.xg.log_subtype") };
            if _cond {
            event.set("event.action", json!(event.get("sophos.xg.log_subtype").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("sophos.xg.log_subtype") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { ["03001", "05001", "05151", "00003", "00004"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
            event.set("event.kind", json!("alert"))?;
            }

            let _cond = { ["03001", "05001", "05151", "00003", "00004"].contains(&event.get_str("event.code").unwrap_or("")) };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

                event.append("event.category", json!("network"))?;

            let _cond = { ["Start", "Interim"].contains(&event.get_str("sophos.xg.connevent").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("start"))?;
                event.append("event.type", json!("allowed"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("sophos.xg.connevent") == Some("Stop") };
            if _cond {
                event.append("event.type", json!("end"))?;
                event.append("event.type", json!("allowed"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("sophos.xg.status") == Some("Deny") };
            if _cond {
                event.append("event.type", json!("denied"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.has_value("sophos.xg.dst_ip") };
            if _cond {
                if event.has_value("sophos.xg.dst_ip") {
                    event.rename("sophos.xg.dst_ip", "destination.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.tran_dst_ip") };
            if _cond {
                if event.has_value("sophos.xg.tran_dst_ip") {
                    event.rename("sophos.xg.tran_dst_ip", "destination.nat.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.destinationip") };
            if _cond {
                if event.has_value("sophos.xg.destinationip") {
                    event.rename("sophos.xg.destinationip", "destination.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.dst_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.dst_port") {
                if let Some(val) = event.get("sophos.xg.dst_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.dst_port".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.tran_dst_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.tran_dst_port") {
                if let Some(val) = event.get("sophos.xg.tran_dst_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.tran_dst_port".into(),
                            message,
                        })?;
                    event.set("destination.nat.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.recv_pkts") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.recv_pkts") {
                if let Some(val) = event.get("sophos.xg.recv_pkts") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.recv_pkts".into(),
                            message,
                        })?;
                    event.set("destination.packets", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.packets_received") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.packets_received") {
                if let Some(val) = event.get("sophos.xg.packets_received") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.packets_received".into(),
                            message,
                        })?;
                    event.set("destination.packets", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.src_ip") };
            if _cond {
                if event.has_value("sophos.xg.src_ip") {
                    event.rename("sophos.xg.src_ip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.tran_src_ip") };
            if _cond {
                if event.has_value("sophos.xg.tran_src_ip") {
                    event.rename("sophos.xg.tran_src_ip", "source.nat.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.src_trans_ip") };
            if _cond {
                if event.has_value("sophos.xg.src_trans_ip") {
                    event.rename("sophos.xg.src_trans_ip", "source.nat.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.sourceip") };
            if _cond {
                if event.has_value("sophos.xg.sourceip") {
                    event.rename("sophos.xg.sourceip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.src_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.src_port") {
                if let Some(val) = event.get("sophos.xg.src_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.src_port".into(),
                            message,
                        })?;
                    event.set("source.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.tran_src_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.tran_src_port") {
                if let Some(val) = event.get("sophos.xg.tran_src_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.tran_src_port".into(),
                            message,
                        })?;
                    event.set("source.nat.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.src_mac") };
            if _cond {
                if event.has_value("sophos.xg.src_mac") {
                    event.rename("sophos.xg.src_mac", "source.mac")?;
                }
            }

            if event.has_value("sophos.xg.sent_pkts") {
                map_strings(event, "sophos.xg.sent_pkts", "sophos.xg.sent_pkts", |s| s.trim().to_string())?;
            }

            if event.has_value("sophos.xg.packets_sent") {
                map_strings(event, "sophos.xg.packets_sent", "sophos.xg.packets_sent", |s| s.trim().to_string())?;
            }

            let _cond = { event.has_value("sophos.xg.sent_pkts") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.sent_pkts") {
                if let Some(val) = event.get("sophos.xg.sent_pkts") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.sent_pkts".into(),
                            message,
                        })?;
                    event.set("source.packets", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.packets_sent") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("sophos.xg.packets_sent") {
                if let Some(val) = event.get("sophos.xg.packets_sent") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "sophos.xg.packets_sent".into(),
                            message,
                        })?;
                    event.set("source.packets", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("sophos.xg.user_name") };
            if _cond {
                if event.has_value("sophos.xg.user_name") {
                    event.rename("sophos.xg.user_name", "source.user.name")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.user_gp") };
            if _cond {
                if event.has_value("sophos.xg.user_gp") {
                    event.rename("sophos.xg.user_gp", "source.user.group.name")?;
                }
            }

            let _cond = { !event.has_value("rule.id") };
            if _cond {
                if event.has_value("sophos.xg.fw_rule_id") {
                    event.rename("sophos.xg.fw_rule_id", "rule.id")?;
                }
            }

            let _cond = { event.has_value("sophos.xg.policy_type") };
            if _cond {
                if event.has_value("sophos.xg.policy_type") {
                    event.rename("sophos.xg.policy_type", "rule.ruleset")?;
                }
            }

                if event.has_value("sophos.xg.application") {
                    event.rename("sophos.xg.application", "network.protocol")?;
                }

                if event.has_value("sophos.xg.protocol") {
                    event.rename("sophos.xg.protocol", "network.transport")?;
                }

            let _cond = { ["LAN", "DMZ", "VPN", "WiFi"].contains(&event.get_str("observer.egress.zone").unwrap_or("")) && event.get_str("observer.ingress.zone") == Some("WAN") };
            if _cond {
            event.set("network.direction", json!("inbound"))?;
            }

            let _cond = { ["LAN", "DMZ", "VPN", "WiFi"].contains(&event.get_str("observer.ingress.zone").unwrap_or("")) && event.get_str("observer.egress.zone") == Some("WAN") };
            if _cond {
            event.set("network.direction", json!("outbound"))?;
            }

            let _cond = { ["LAN", "DMZ", "VPN", "WiFi"].contains(&event.get_str("observer.ingress.zone").unwrap_or("")) && ["LAN", "DMZ", "VPN", "WiFi"].contains(&event.get_str("observer.egress.zone").unwrap_or("")) };
            if _cond {
            event.set("network.direction", json!("internal"))?;
            }

            let _cond = { event.get_str("observer.ingress.zone") == Some("WAN") && event.get_str("observer.egress.zone") == Some("WAN") };
            if _cond {
            event.set("network.direction", json!("external"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                Ok(())
            })();

                event.remove("sophos.xg.dst_port");
                event.remove("sophos.xg.tran_dst_port");
                event.remove("sophos.xg.recv_pkts");
                event.remove("sophos.xg.src_port");
                event.remove("sophos.xg.tran_src_port");
                event.remove("sophos.xg.sent_pkts");
                event.remove("sophos.xg.packets_received");
                event.remove("sophos.xg.packets_sent");

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
