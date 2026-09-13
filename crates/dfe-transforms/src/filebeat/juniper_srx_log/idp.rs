// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `idp` pipeline.
pub struct Idp;

impl Transform for Idp {
    fn name(&self) -> &str {
        "idp"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("juniper.srx.tag") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

                event.append("event.category", json!("network"))?;

            let _cond = { ["IDP_ATTACK_LOG_EVENT", "IDP_APPDDOS_APP_STATE_EVENT", "IDP_APPDDOS_APP_ATTACK_EVENT", "IDP_ATTACK_LOG_EVENT_LS", "IDP_APPDDOS_APP_STATE_EVENT_LS", "IDP_APPDDOS_APP_ATTACK_EVENT_LS"].contains(&event.get_str("juniper.srx.tag").unwrap_or("")) };
            if _cond {
            event.set("event.kind", json!("alert"))?;
            }

            let _cond = { ["IDP_ATTACK_LOG_EVENT", "IDP_APPDDOS_APP_STATE_EVENT", "IDP_APPDDOS_APP_ATTACK_EVENT", "IDP_ATTACK_LOG_EVENT_LS", "IDP_APPDDOS_APP_STATE_EVENT_LS", "IDP_APPDDOS_APP_ATTACK_EVENT_LS"].contains(&event.get_str("juniper.srx.tag").unwrap_or("")) };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            let _cond = { ["IDP_ATTACK_LOG_EVENT", "IDP_APPDDOS_APP_STATE_EVENT", "IDP_APPDDOS_APP_ATTACK_EVENT", "IDP_ATTACK_LOG_EVENT_LS", "IDP_APPDDOS_APP_STATE_EVENT_LS", "IDP_APPDDOS_APP_ATTACK_EVENT_LS"].contains(&event.get_str("juniper.srx.tag").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("info"))?;
                event.append("event.type", json!("denied"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { !(["IDP_ATTACK_LOG_EVENT", "IDP_APPDDOS_APP_STATE_EVENT", "IDP_APPDDOS_APP_ATTACK_EVENT", "IDP_ATTACK_LOG_EVENT_LS", "IDP_APPDDOS_APP_STATE_EVENT_LS", "IDP_APPDDOS_APP_ATTACK_EVENT_LS"].contains(&event.get_str("juniper.srx.tag").unwrap_or(""))) };
            if _cond {
                event.append("event.type", json!("allowed"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { ["IDP_APPDDOS_APP_STATE_EVENT", "IDP_APPDDOS_APP_ATTACK_EVENT", "IDP_APPDDOS_APP_STATE_EVENT_LS", "IDP_APPDDOS_APP_ATTACK_EVENT_LS"].contains(&event.get_str("juniper.srx.tag").unwrap_or("")) };
            if _cond {
            event.set("event.action", json!("application_ddos"))?;
            }

            let _cond = { ["IDP_ATTACK_LOG_EVENT", "IDP_ATTACK_LOG_EVENT_LS"].contains(&event.get_str("juniper.srx.tag").unwrap_or("")) };
            if _cond {
            event.set("event.action", json!("security_threat"))?;
            }

            let _cond = { event.has_value("juniper.srx.destination_address") };
            if _cond {
                if event.has_value("juniper.srx.destination_address") {
                    event.rename("juniper.srx.destination_address", "destination.ip")?;
                }
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
            event.set("server.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("juniper.srx.nat_destination_address") };
            if _cond {
                if event.has_value("juniper.srx.nat_destination_address") {
                    event.rename("juniper.srx.nat_destination_address", "destination.nat.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.destination_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("juniper.srx.destination_port") {
                if let Some(val) = event.get("juniper.srx.destination_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "juniper.srx.destination_port".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("destination.port") };
            if _cond {
            event.set("server.port", json!(event.get("destination.port").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("server.port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("server.port") {
                if let Some(val) = event.get("server.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "server.port".into(),
                            message,
                        })?;
                    event.set("server.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("juniper.srx.nat_destination_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("juniper.srx.nat_destination_port") {
                if let Some(val) = event.get("juniper.srx.nat_destination_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "juniper.srx.nat_destination_port".into(),
                            message,
                        })?;
                    event.set("destination.nat.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("destination.nat.port") };
            if _cond {
            event.set("server.nat.port", json!(event.get("destination.nat.port").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("server.nat.port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("server.nat.port") {
                if let Some(val) = event.get("server.nat.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "server.nat.port".into(),
                            message,
                        })?;
                    event.set("server.nat.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("juniper.srx.inbound_bytes") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("juniper.srx.inbound_bytes") {
                if let Some(val) = event.get("juniper.srx.inbound_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "juniper.srx.inbound_bytes".into(),
                            message,
                        })?;
                    event.set("destination.bytes", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("destination.bytes") };
            if _cond {
            event.set("server.bytes", json!(event.get("destination.bytes").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("server.bytes") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("server.bytes") {
                if let Some(val) = event.get("server.bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "server.bytes".into(),
                            message,
                        })?;
                    event.set("server.bytes", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("juniper.srx.inbound_packets") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("juniper.srx.inbound_packets") {
                if let Some(val) = event.get("juniper.srx.inbound_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "juniper.srx.inbound_packets".into(),
                            message,
                        })?;
                    event.set("destination.packets", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("destination.packets") };
            if _cond {
            event.set("server.packets", json!(event.get("destination.packets").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("server.packets") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("server.packets") {
                if let Some(val) = event.get("server.packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "server.packets".into(),
                            message,
                        })?;
                    event.set("server.packets", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("juniper.srx.source_address") };
            if _cond {
                if event.has_value("juniper.srx.source_address") {
                    event.rename("juniper.srx.source_address", "source.ip")?;
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
            event.set("client.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("juniper.srx.nat_source_address") };
            if _cond {
                if event.has_value("juniper.srx.nat_source_address") {
                    event.rename("juniper.srx.nat_source_address", "source.nat.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.sourceip") };
            if _cond {
                if event.has_value("juniper.srx.sourceip") {
                    event.rename("juniper.srx.sourceip", "source.ip")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.source_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("juniper.srx.source_port") {
                if let Some(val) = event.get("juniper.srx.source_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "juniper.srx.source_port".into(),
                            message,
                        })?;
                    event.set("source.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("source.port") };
            if _cond {
            event.set("client.port", json!(event.get("source.port").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("client.port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("client.port") {
                if let Some(val) = event.get("client.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "client.port".into(),
                            message,
                        })?;
                    event.set("client.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("juniper.srx.nat_source_port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("juniper.srx.nat_source_port") {
                if let Some(val) = event.get("juniper.srx.nat_source_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "juniper.srx.nat_source_port".into(),
                            message,
                        })?;
                    event.set("source.nat.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("source.nat.port") };
            if _cond {
            event.set("client.nat.port", json!(event.get("source.nat.port").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("client.nat.port") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("client.nat.port") {
                if let Some(val) = event.get("client.nat.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "client.nat.port".into(),
                            message,
                        })?;
                    event.set("client.nat.port", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("juniper.srx.outbound_bytes") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("juniper.srx.outbound_bytes") {
                if let Some(val) = event.get("juniper.srx.outbound_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "juniper.srx.outbound_bytes".into(),
                            message,
                        })?;
                    event.set("source.bytes", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("source.bytes") };
            if _cond {
            event.set("client.bytes", json!(event.get("source.bytes").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("client.bytes") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("client.bytes") {
                if let Some(val) = event.get("client.bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "client.bytes".into(),
                            message,
                        })?;
                    event.set("client.bytes", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("juniper.srx.outbound_packets") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("juniper.srx.outbound_packets") {
                if let Some(val) = event.get("juniper.srx.outbound_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "juniper.srx.outbound_packets".into(),
                            message,
                        })?;
                    event.set("source.packets", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("source.packets") };
            if _cond {
            event.set("client.packets", json!(event.get("source.packets").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("client.packets") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("client.packets") {
                if let Some(val) = event.get("client.packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "client.packets".into(),
                            message,
                        })?;
                    event.set("client.packets", converted)?;
                }
            }
                Ok(())
            })();
            }

            let _cond = { event.has_value("juniper.srx.username") };
            if _cond {
                if event.has_value("juniper.srx.username") {
                    event.rename("juniper.srx.username", "source.user.name")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.rulebase_name") };
            if _cond {
                if event.has_value("juniper.srx.rulebase_name") {
                    event.rename("juniper.srx.rulebase_name", "rule.name")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.rule_name") };
            if _cond {
                if event.has_value("juniper.srx.rule_name") {
                    event.rename("juniper.srx.rule_name", "rule.id")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.protocol_name") };
            if _cond {
                if event.has_value("juniper.srx.protocol_name") {
                    event.rename("juniper.srx.protocol_name", "network.protocol")?;
                }
            }

            let _cond = { event.has_value("juniper.srx.message") };
            if _cond {
                if event.has_value("juniper.srx.message") {
                    event.rename("juniper.srx.message", "message")?;
                }
            }

                event.remove("juniper.srx.destination_port");
                event.remove("juniper.srx.nat_destination_port");
                event.remove("juniper.srx.outbound_bytes");
                event.remove("juniper.srx.outbound_packets");
                event.remove("juniper.srx.source_port");
                event.remove("juniper.srx.nat_source_port");
                event.remove("juniper.srx.inbound_bytes");
                event.remove("juniper.srx.inbound_packets");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
