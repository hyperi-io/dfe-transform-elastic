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
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (?:(?:%{INT},%{INT}?,,%{DATA:rule.id},%{DATA:observer.ingress.interface.name},(?P<event_reason>(?:[a-zA-Z-]+)),%{WORD:event.action},%{WORD:network.direction},)(?:(?:(?P<network_type>(4)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.ecn}?,%{NONNEGINT:pfsense.ip.ttl:long},%{NONNEGINT:pfsense.ip.id:long},%{NONNEGINT:pfsense.ip.offset:long},(?:%{WORD:pfsense.ip.flags}|(?P<pfsense_ip_flags>(?:[+]))),%{INT:network.iana_number},%{WORD:network.transport},)|(?:(?P<network_type>(6)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.flow_label},%{WORD:pfsense.ip.flags},(?P<network_transport>(?:[0-9a-zA-Z-]+)),%{INT:network.iana_number},))(?:%{NONNEGINT:network.bytes:long},%{IP:source.address},%{IP:destination.address},)(?:(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.tcp.length:long},%{WORD:pfsense.tcp.flags}?,%{NONNEGINT:pfsense.tcp.seq:long}?:?%{NONNEGINT},%{NONNEGINT:pfsense.tcp.ack:long}?,%{NONNEGINT:pfsense.tcp.window:long}?,%{WORD:pfsense.tcp.urg}?,%{GREEDYDATA:pfsense.tcp.options})|(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.udp.length:long}$)|(?:(?:(?P<pfsense_icmp_type>(request|reply|unreachproto|unreachport|unreach|timeexceed|paramprob|redirect|maskreply|needfrag|tstamp|tstampreply)),)(?:(?:%{NONNEGINT:pfsense.icmp.id:long},%{NONNEGINT:pfsense.icmp.seq:long})|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?,\\[?%{NONNEGINT:pfsense.icmp.unreachable.port:long}\\]?)|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?)|(?:%{GREEDYDATA:pfsense.icmp.unreachable.other})|(?:%{IP:pfsense.icmp.destination.ip},%{NONNEGINT:pfsense.icmp.mtu:long})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq},%{INT:pfsense.icmp.otime},%{INT:pfsense.icmp.rtime},%{INT:pfsense.icmp.ttime})))|(?:datalength=%{NONNEGINT:network.packets:long})|(?:%{GREEDYDATA})|(?:))?)%{GREEDYDATA}
                    if !cached_grok_mapped!("(?:(?:%{INT},%{INT}?,,%{DATA:rule.id},%{DATA:observer.ingress.interface.name},(?P<event_reason>(?:[a-zA-Z-]+)),%{WORD:event.action},%{WORD:network.direction},)(?:(?:(?P<network_type>(4)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.ecn}?,%{NONNEGINT:pfsense.ip.ttl:long},%{NONNEGINT:pfsense.ip.id:long},%{NONNEGINT:pfsense.ip.offset:long},(?:%{WORD:pfsense.ip.flags}|(?P<pfsense_ip_flags>(?:[+]))),%{INT:network.iana_number},%{WORD:network.transport},)|(?:(?P<network_type>(6)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.flow_label},%{WORD:pfsense.ip.flags},(?P<network_transport>(?:[0-9a-zA-Z-]+)),%{INT:network.iana_number},))(?:%{NONNEGINT:network.bytes:long},%{IP:source.address},%{IP:destination.address},)(?:(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.tcp.length:long},%{WORD:pfsense.tcp.flags}?,%{NONNEGINT:pfsense.tcp.seq:long}?:?%{NONNEGINT},%{NONNEGINT:pfsense.tcp.ack:long}?,%{NONNEGINT:pfsense.tcp.window:long}?,%{WORD:pfsense.tcp.urg}?,%{GREEDYDATA:pfsense.tcp.options})|(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.udp.length:long}$)|(?:(?:(?P<pfsense_icmp_type>(request|reply|unreachproto|unreachport|unreach|timeexceed|paramprob|redirect|maskreply|needfrag|tstamp|tstampreply)),)(?:(?:%{NONNEGINT:pfsense.icmp.id:long},%{NONNEGINT:pfsense.icmp.seq:long})|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?,\\[?%{NONNEGINT:pfsense.icmp.unreachable.port:long}\\]?)|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?)|(?:%{GREEDYDATA:pfsense.icmp.unreachable.other})|(?:%{IP:pfsense.icmp.destination.ip},%{NONNEGINT:pfsense.icmp.mtu:long})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq},%{INT:pfsense.icmp.otime},%{INT:pfsense.icmp.rtime},%{INT:pfsense.icmp.ttime})))|(?:datalength=%{NONNEGINT:network.packets:long})|(?:%{GREEDYDATA})|(?:))?)%{GREEDYDATA}", [("event_reason", "event.reason"), ("pfsense_ip_flags", "pfsense.ip.flags"), ("network_transport", "network.transport"), ("network_type", "network.type"), ("network_type", "network.type"), ("pfsense_icmp_type", "pfsense.icmp.type")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("source.address") && event.has_value("destination.address") };
            if _cond {
                event.append_unique("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("event.action") == Some("block") };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = { event.get_str("event.action") == Some("pass") };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            if event.has_value("network.transport") {
                map_strings(event, "network.transport", "network.transport", str::to_lowercase)?;
            }

            let _cond = { !event.has_value("ack_number") || event.get_str("ack_number") == Some("") };
            if _cond {
                event.remove("ack_number");
            }

                // Classify network direction against the internal network ranges
                if let (Some(src), Some(dst)) = (event.get_string("source.ip"), event.get_string("destination.ip")) {
                    if let Some(listed) = event.get_array("_tmp.internal_networks") {
                    let listed: Vec<String> = listed.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect();
                    let networks: Vec<&str> = listed.iter().map(String::as_str).collect();
                    let direction = match (ip_in_networks(&src, &networks), ip_in_networks(&dst, &networks)) {
                        (true, false) => "outbound",
                        (false, true) => "inbound",
                        (true, true) => "internal",
                        (false, false) => "external",
                    };
                    event.set("network.direction", json!(direction))?;
                    }
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("pfsense.tcp.options") {
                if let Some(s) = event.get_string("pfsense.tcp.options") {
                    let mut parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("pfsense.tcp.options", Value::Array(parts))?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("pfsense.icmp.otime") {
                    match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "pfsense.icmp.otime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("pfsense.icmp.rtime") {
                    match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "pfsense.icmp.rtime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("pfsense.icmp.ttime") {
                    match parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "pfsense.icmp.ttime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
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
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
