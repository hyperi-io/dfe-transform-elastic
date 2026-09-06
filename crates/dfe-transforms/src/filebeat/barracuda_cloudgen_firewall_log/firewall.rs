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
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }

            let _cond = { event.get_str("json.src_ip") != Some("-") };
            if _cond {
                if event.has_value("json.src_ip") {
                    event.rename("json.src_ip", "source.address")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })();

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

                if event.has_value("json.src_iface") {
                    event.rename("json.src_iface", "observer.ingress.interface.name")?;
                }

            let _cond = { event.get_str("json.src_mac") != Some("00:00:00:00:00:00") };
            if _cond {
            if event.has_value("json.src_mac") {
                gsub_field(event, "json.src_mac", "source.mac", cached_regex!("[-:.]"), "-")?;
            }
            }

            let _cond = { event.get_str("json.src_ip_nat") != Some("0.0.0.0") };
            if _cond {
            if event.has_value("json.src_ip_nat") {
                if let Some(val) = event.get("json.src_ip_nat") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.src_ip_nat".into(),
                            message,
                        })?;
                    event.set("source.nat.ip", converted)?;
                }
            }
            }

            if event.has_value("json.fwd_bytes") {
                if let Some(val) = event.get("json.fwd_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.fwd_bytes".into(),
                            message,
                        })?;
                    event.set("source.bytes", converted)?;
                }
            }

            if event.has_value("json.fwd_packets") {
                if let Some(val) = event.get("json.fwd_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.fwd_packets".into(),
                            message,
                        })?;
                    event.set("source.packets", converted)?;
                }
            }

            let _cond = { event.get_str("json.dst_ip") != Some("-") };
            if _cond {
                if event.has_value("json.dst_ip") {
                    event.rename("json.dst_ip", "destination.address")?;
                }
            }

            if event.has_value("destination.address") {
                if let Some(val) = event.get("destination.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "destination.address".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
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

                if event.has_value("json.dst_iface") {
                    event.rename("json.dst_iface", "observer.egress.interface.name")?;
                }

            let _cond = { event.get_str("json.dst_mac") != Some("00:00:00:00:00:00") };
            if _cond {
            if event.has_value("json.dst_mac") {
                gsub_field(event, "json.dst_mac", "destination.mac", cached_regex!("[-:.]"), "-")?;
            }
            }

            let _cond = { event.get_str("json.dst_ip_nat") != Some("0.0.0.0") };
            if _cond {
            if event.has_value("json.dst_ip_nat") {
                if let Some(val) = event.get("json.dst_ip_nat") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dst_ip_nat".into(),
                            message,
                        })?;
                    event.set("destination.nat.ip", converted)?;
                }
            }
            }

            if event.has_value("destination.mac") {
                map_strings(event, "destination.mac", "destination.mac", str::to_uppercase)?;
            }

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            if event.has_value("destination.mac") {
                gsub_field(event, "destination.mac", "destination.mac", cached_regex!("[.:]"), "-")?;
            }

            if event.has_value("source.mac") {
                gsub_field(event, "source.mac", "source.mac", cached_regex!("[.:]"), "-")?;
            }

            if event.has_value("json.rev_bytes") {
                if let Some(val) = event.get("json.rev_bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.rev_bytes".into(),
                            message,
                        })?;
                    event.set("destination.bytes", converted)?;
                }
            }

            if event.has_value("json.rev_packets") {
                if let Some(val) = event.get("json.rev_packets") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.rev_packets".into(),
                            message,
                        })?;
                    event.set("destination.packets", converted)?;
                }
            }

                if event.has_value("json.fw_rule") {
                    event.rename("json.fw_rule", "rule.name")?;
                }

                if event.has_value("json.app_rule") {
                    event.rename("json.app_rule", "barracuda_cloudgen_firewall.log.app_rule")?;
                }

                if event.has_value("json.apps") {
                    event.rename("json.apps", "barracuda_cloudgen_firewall.log.apps")?;
                }

                if event.has_value("json.protos") {
                    event.rename("json.protos", "barracuda_cloudgen_firewall.log.protos")?;
                }

            let _cond = { event.get_str("json.fw_info") != Some("-") };
            if _cond {
                if event.has_value("json.fw_info") {
                    event.rename("json.fw_info", "barracuda_cloudgen_firewall.log.fw_info")?;
                }
            }

                if event.has_value("json.action") {
                    event.rename("json.action", "event.action")?;
                }

            let _cond = { event.has_value("json.duration") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.event.duration = (long)ctx.json.duration * 1000000;
                scale_field(event, &ScaleField::new("json.duration", "event.duration", Factor::Long(1000000)));
            }

            let _cond = { event.has_value("source.bytes") && event.has_value("destination.bytes") && !event.has_value("network.bytes") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes
                sum_directions(event, &["bytes"]);
                Ok(())
            })();
            }

            let _cond = { event.has_value("source.packets") && event.has_value("destination.packets") && !event.has_value("network.packets") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.network.packets = ctx.source.packets + ctx.destination.packets
                sum_directions(event, &["packets"]);
                Ok(())
            })();
            }

            if event.has_value("json.ip_proto") {
                if let Some(val) = event.get("json.ip_proto") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.ip_proto".into(),
                            message,
                        })?;
                    event.set("network.iana_number", converted)?;
                }
            }

                if event.has_value("json.user") {
                    event.rename("json.user", "user.name")?;
                }

            let _cond = { event.has_value("network.iana_number") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: def iana_number = ctx.network.iana_number;\nif (iana_number == '0') {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number == '2') {\n    ctx.network.transport = 'igmp';\n} else if (iana_number == '6') {\n    ctx.network.transport = 'tcp';\n} else if (iana_number == '8') {\n    ctx.network.transport = 'egp';\n} else if (iana_number == '17') {\n    ctx.network.transport = 'udp';\n} else if (iana_number == '47') {\n    ctx.network.transport = 'gre';\n} else if (iana_number == '50') {\n    ctx.network.transport = 'esp';\n} else if (iana_number == '58') {\n    ctx.network.transport = 'ipv6-icmp';\n} else if (iana_number == '112') {\n    ctx.network.transport = 'vrrp';\n} else if (iana_number == '132') {\n    ctx.network.transport = 'sctp';\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def iana_number = ctx.network.iana_number;\nif (iana_number == '0') {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number == '2') {\n    ctx.network.transport = 'igmp';\n} else if (iana_number == '6') {\n    ctx.network.transport = 'tcp';\n} else if (iana_number == '8') {\n    ctx.network.transport = 'egp';\n} else if (iana_number == '17') {\n    ctx.network.transport = 'udp';\n} else if (iana_number == '47') {\n    ctx.network.transport = 'gre';\n} else if (iana_number == '50') {\n    ctx.network.transport = 'esp';\n} else if (iana_number == '58') {\n    ctx.network.transport = 'ipv6-icmp';\n} else if (iana_number == '112') {\n    ctx.network.transport = 'vrrp';\n} else if (iana_number == '132') {\n    ctx.network.transport = 'sctp';\n}\n"#))?;
                Ok(())
            })();
            }

            event.set("event.kind", json!("event"))?;

                event.append_unique("event.category", json!("network"))?;

            let _cond = { event.get_str("event.action") == Some("AppBlock") };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = { event.get_str("event.action") == Some("End") };
            if _cond {
                event.append_unique("event.type", json!("end"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
