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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("observer.vendor", json!("snort"))?;

            event.set("observer.product", json!("ids"))?;

            event.set("observer.type", json!("ids"))?;

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?P<_tmp_first_char>(?:.))
                if !cached_grok_mapped!(
                    "^(?P<_tmp_first_char>(?:.))",
                    [("_tmp_first_char", "_tmp.first_char")]
                )
                .extract_into(&input, event)?
                {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            let _cond = { event.get_str("_tmp.first_char") != Some("{") };
            if _cond {
                // Begin nested pipeline: "plaintext"
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^((?:<%{NONNEGINT:log.syslog.priority:long}>))?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:%{SYSLOGFACILITY} )?(?:(?:%{IP:observer.ip}|%{HOSTNAME:observer.name})) (?:%{PROG:process.name}(?:\\[%{POSINT:process.pid:int}\\])?):(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:%{SPACE}(?:(\\[Classification: %{DATA:rule.category}\\])?) (?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\{%{WORD:network.transport}\\} %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|))
                    // Grok pattern: (?:(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))(%{SPACE})?,%{NONNEGINT:snort.gid:long},%{NONNEGINT:rule.id},%{NONNEGINT:rule.version},(\"?%{DATA:rule.description}\"?|),%{WORD:network.transport},%{IP:source.address},(%{POSINT:source.port:long}|),%{IP:destination.address},(%{POSINT:destination.port:long}|)),%{NONNEGINT:snort.ip.id:long},(%{DATA:rule.category}|),%{NONNEGINT:event.severity:long},%{WORD},%{WORD:_tmp.action}
                    // Grok pattern: (?:(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))(%{SPACE})?,%{NONNEGINT:snort.gid:long},%{NONNEGINT:rule.id},%{NONNEGINT:rule.version},(\"?%{DATA:rule.description}\"?|),%{WORD:network.transport},%{IP:source.address},(%{POSINT:source.port:long}|),%{IP:destination.address},(%{POSINT:destination.port:long}|)),(%{MAC:source.mac}|),(%{MAC:destination.mac}|),(%{DATA:snort.eth.length}|),(%{DATA:snort.tcp.flags}|),(%{BASE16NUM:snort.tcp.seq}|),(%{BASE16NUM:snort.tcp.ack}|),(|%{DATA:snort.tcp.length}),(%{BASE16NUM:snort.tcp.window}|),(%{NONNEGINT:snort.ip.ttl:long}|),(%{NONNEGINT:snort.ip.tos:long}|),(%{NONNEGINT:snort.ip.id:long}|),(%{NONNEGINT:snort.dgm.length:long}|),(%{NONNEGINT:snort.ip.length:long}|),(%{NONNEGINT:snort.icmp.type:long}|),(%{NONNEGINT:snort.icmp.code:long}|),(%{NONNEGINT:snort.icmp.id:long}|),(%{NONNEGINT:snort.icmp.seq:long}|)
                    // Grok pattern: (?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))%{SPACE}(?:(?:(\\[\\*\\*\\]))(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:(\\[\\*\\*\\])))(?:%{SPACE}(?:(\\[Classification: %{DATA:rule.category}\\])?) (?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\{%{WORD:network.transport}\\} %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|))
                    // Grok pattern: (?:(?:(\\[\\*\\*\\]))(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:(\\[\\*\\*\\])))\\n((?:(\\[Classification: %{DATA:rule.category}\\])?) )?(?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\n(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME})) %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|)\\n%{WORD:network.transport} (TTL:%{NONNEGINT:snort.ip.ttl:long}|) (TOS:%{BASE16NUM:snort.ip.tos}|) (ID:%{NONNEGINT:snort.ip.id:long}|) (IpLen:%{NONNEGINT:snort.ip.length:long}|) (DgmLen:%{NONNEGINT:snort.dgm.length:long}|)(%{SPACE}%{NOTSPACE:snort.ip.flags})?\\n((?:(Len: %{NONNEGINT:snort.udp.length:long}))|(?:(Type:%{NONNEGINT:snort.icmp.type:long})%{SPACE}(Code:%{NONNEGINT:snort.icmp.code:long})%{SPACE}(ID:%{NONNEGINT:snort.icmp.id:long})%{SPACE}(Seq:%{NONNEGINT:snort.icmp.seq:long})%{GREEDYDATA})|(?:(%{NOTSPACE:snort.tcp.flags})%{SPACE}(Seq: %{BASE16NUM:snort.tcp.seq})%{SPACE}(Ack: %{BASE16NUM:snort.tcp.ack})%{SPACE}(Win: %{BASE16NUM:snort.tcp.window})%{SPACE}(TcpLen: %{NONNEGINT:snort.tcp.length:long})))
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^((?:<%{NONNEGINT:log.syslog.priority:long}>))?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:%{SYSLOGFACILITY} )?(?:(?:%{IP:observer.ip}|%{HOSTNAME:observer.name})) (?:%{PROG:process.name}(?:\\[%{POSINT:process.pid:int}\\])?):(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:%{SPACE}(?:(\\[Classification: %{DATA:rule.category}\\])?) (?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\{%{WORD:network.transport}\\} %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|))"
                            ),
                            cached_grok_mapped!(
                                "(?:(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))(%{SPACE})?,%{NONNEGINT:snort.gid:long},%{NONNEGINT:rule.id},%{NONNEGINT:rule.version},(\"?%{DATA:rule.description}\"?|),%{WORD:network.transport},%{IP:source.address},(%{POSINT:source.port:long}|),%{IP:destination.address},(%{POSINT:destination.port:long}|)),%{NONNEGINT:snort.ip.id:long},(%{DATA:rule.category}|),%{NONNEGINT:event.severity:long},%{WORD},%{WORD:_tmp.action}",
                                [("_tmp_timestamp", "_tmp.timestamp")]
                            ),
                            cached_grok_mapped!(
                                "(?:(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))(%{SPACE})?,%{NONNEGINT:snort.gid:long},%{NONNEGINT:rule.id},%{NONNEGINT:rule.version},(\"?%{DATA:rule.description}\"?|),%{WORD:network.transport},%{IP:source.address},(%{POSINT:source.port:long}|),%{IP:destination.address},(%{POSINT:destination.port:long}|)),(%{MAC:source.mac}|),(%{MAC:destination.mac}|),(%{DATA:snort.eth.length}|),(%{DATA:snort.tcp.flags}|),(%{BASE16NUM:snort.tcp.seq}|),(%{BASE16NUM:snort.tcp.ack}|),(|%{DATA:snort.tcp.length}),(%{BASE16NUM:snort.tcp.window}|),(%{NONNEGINT:snort.ip.ttl:long}|),(%{NONNEGINT:snort.ip.tos:long}|),(%{NONNEGINT:snort.ip.id:long}|),(%{NONNEGINT:snort.dgm.length:long}|),(%{NONNEGINT:snort.ip.length:long}|),(%{NONNEGINT:snort.icmp.type:long}|),(%{NONNEGINT:snort.icmp.code:long}|),(%{NONNEGINT:snort.icmp.id:long}|),(%{NONNEGINT:snort.icmp.seq:long}|)",
                                [("_tmp_timestamp", "_tmp.timestamp")]
                            ),
                            cached_grok_mapped!(
                                "(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME}))%{SPACE}(?:(?:(\\[\\*\\*\\]))(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:(\\[\\*\\*\\])))(?:%{SPACE}(?:(\\[Classification: %{DATA:rule.category}\\])?) (?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\{%{WORD:network.transport}\\} %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|))",
                                [("_tmp_timestamp", "_tmp.timestamp")]
                            ),
                            cached_grok_mapped!(
                                "(?:(?:(\\[\\*\\*\\]))(?:%{SPACE}\\[%{NONNEGINT:snort.gid:long}:%{NONNEGINT:rule.id}:%{NONNEGINT:rule.version}\\]%{SPACE}%{DATA:rule.description}%{SPACE})(?:(\\[\\*\\*\\])))\\n((?:(\\[Classification: %{DATA:rule.category}\\])?) )?(?:\\[Priority: %{NONNEGINT:event.severity:long}\\]) \\n(?P<_tmp_timestamp>(?:%{MONTHNUM}/%{MONTHDAY}(/%{YEAR})?-%{TIME})) %{IP:source.address}(:%{POSINT:source.port:long}|) -> %{IP:destination.address}(:%{POSINT:destination.port:long}|)\\n%{WORD:network.transport} (TTL:%{NONNEGINT:snort.ip.ttl:long}|) (TOS:%{BASE16NUM:snort.ip.tos}|) (ID:%{NONNEGINT:snort.ip.id:long}|) (IpLen:%{NONNEGINT:snort.ip.length:long}|) (DgmLen:%{NONNEGINT:snort.dgm.length:long}|)(%{SPACE}%{NOTSPACE:snort.ip.flags})?\\n((?:(Len: %{NONNEGINT:snort.udp.length:long}))|(?:(Type:%{NONNEGINT:snort.icmp.type:long})%{SPACE}(Code:%{NONNEGINT:snort.icmp.code:long})%{SPACE}(ID:%{NONNEGINT:snort.icmp.id:long})%{SPACE}(Seq:%{NONNEGINT:snort.icmp.seq:long})%{GREEDYDATA})|(?:(%{NOTSPACE:snort.tcp.flags})%{SPACE}(Seq: %{BASE16NUM:snort.tcp.seq})%{SPACE}(Ack: %{BASE16NUM:snort.tcp.ack})%{SPACE}(Win: %{BASE16NUM:snort.tcp.window})%{SPACE}(TcpLen: %{NONNEGINT:snort.tcp.length:long})))",
                                [("_tmp_timestamp", "_tmp.timestamp")]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                // Painless script
                // Source: if (ctx.snort?.ip?.tos != null && ctx.snort.ip.tos instanceof String) {\n    ctx.snort.ip.tos = Long.decode(ctx.snort.ip.tos);\n} if (ctx.snort?.eth?.length != null && ctx.snort.eth.length instanceof String) {\n    ctx.snort.eth.length = Long.decode(ctx.snort.eth.length);\n} if (ctx.snort?.tcp?.ack != null && ctx.snort.tcp.ack instanceof String) {\n    ctx.snort.tcp.ack = Long.decode(ctx.snort.tcp.ack);\n} if (ctx.snort?.tcp?.seq != null && ctx.snort.tcp.seq instanceof String) {\n    ctx.snort.tcp.seq = Long.decode(ctx.snort.tcp.seq);\n} if (ctx.snort?.tcp?.window != null && ctx.snort.tcp.window instanceof String) {\n    ctx.snort.tcp.window = Long.decode(ctx.snort.tcp.window);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.snort?.ip?.tos != null && ctx.snort.ip.tos instanceof String) {\n    ctx.snort.ip.tos = Long.decode(ctx.snort.ip.tos);\n} if (ctx.snort?.eth?.length != null && ctx.snort.eth.length instanceof String) {\n    ctx.snort.eth.length = Long.decode(ctx.snort.eth.length);\n} if (ctx.snort?.tcp?.ack != null && ctx.snort.tcp.ack instanceof String) {\n    ctx.snort.tcp.ack = Long.decode(ctx.snort.tcp.ack);\n} if (ctx.snort?.tcp?.seq != null && ctx.snort.tcp.seq instanceof String) {\n    ctx.snort.tcp.seq = Long.decode(ctx.snort.tcp.seq);\n} if (ctx.snort?.tcp?.window != null && ctx.snort.tcp.window instanceof String) {\n    ctx.snort.tcp.window = Long.decode(ctx.snort.tcp.window);\n}"#
                    ),
                )?;
                // End nested pipeline: "plaintext"
            }

            let _cond = { event.get_str("_tmp.first_char") == Some("{") };
            if _cond {
                // Begin nested pipeline: "json"
                parse_json_field(event, "event.original", "json")?;
                event.remove("json.b64_data");
                if event.has_value("json.timestamp") {
                    event.rename("json.timestamp", "_tmp.timestamp")?;
                }
                if event.has_value("json.src_port") {
                    if let Some(val) = event.get("json.src_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.src_port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                if event.has_value("json.dst_port") {
                    if let Some(val) = event.get("json.dst_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.dst_port".into(),
                                message,
                            }
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
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.eth_len".into(),
                                message,
                            }
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
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.rev".into(),
                                message,
                            }
                        })?;
                        event.set("rule.version", converted)?;
                    }
                }
                if event.has_value("json.sid") {
                    if let Some(val) = event.get("json.sid") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sid".into(),
                                message,
                            }
                        })?;
                        event.set("rule.id", converted)?;
                    }
                }
                if event.has_value("json.gid") {
                    if let Some(val) = event.get("json.gid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.gid".into(),
                                message,
                            }
                        })?;
                        event.set("snort.gid", converted)?;
                    }
                }
                if event.has_value("json.icmp_type") {
                    if let Some(val) = event.get("json.icmp_type") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.icmp_type".into(),
                                message,
                            }
                        })?;
                        event.set("snort.icmp.type", converted)?;
                    }
                }
                if event.has_value("json.icmp_code") {
                    if let Some(val) = event.get("json.icmp_code") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.icmp_code".into(),
                                message,
                            }
                        })?;
                        event.set("snort.icmp.code", converted)?;
                    }
                }
                if event.has_value("json.icmp_id") {
                    if let Some(val) = event.get("json.icmp_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.icmp_id".into(),
                                message,
                            }
                        })?;
                        event.set("snort.icmp.id", converted)?;
                    }
                }
                if event.has_value("json.icmp_seq") {
                    if let Some(val) = event.get("json.icmp_seq") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.icmp_seq".into(),
                                message,
                            }
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
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ip_id".into(),
                                message,
                            }
                        })?;
                        event.set("snort.ip.id", converted)?;
                    }
                }
                if event.has_value("json.tos") {
                    if let Some(val) = event.get("json.tos") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.tos".into(),
                                message,
                            }
                        })?;
                        event.set("snort.ip.tos", converted)?;
                    }
                }
                if event.has_value("json.ttl") {
                    if let Some(val) = event.get("json.ttl") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ttl".into(),
                                message,
                            }
                        })?;
                        event.set("snort.ip.ttl", converted)?;
                    }
                }
                if event.has_value("json.pkt_num") {
                    if let Some(val) = event.get("json.pkt_num") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.pkt_num".into(),
                                message,
                            }
                        })?;
                        event.set("network.packets", converted)?;
                    }
                }
                if event.has_value("json.pkt_len") {
                    if let Some(val) = event.get("json.pkt_len") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.pkt_len".into(),
                                message,
                            }
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
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.vlan".into(),
                                    message,
                                }
                            })?;
                            event.set("network.vlan.id", converted)?;
                        }
                    }
                }
                if event.has_value("json.priority") {
                    if let Some(val) = event.get("json.priority") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.priority".into(),
                                message,
                            }
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
                // End nested pipeline: "json"
            }

            let _cond = {
                event.has_value("_tmp.tz_offset")
                    && event.get_str("_tmp.tz_offset") != Some("local")
            };
            if _cond {
                event.set(
                    "event.timezone",
                    json!(
                        event
                            .get("_tmp.tz_offset")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "MM/dd-HH:mm:ss.SSSSSS",
                            "MM/dd/yy-HH:mm:ss.SSSSSS",
                            "MMM  d HH:mm:ss",
                            "MMM dd HH:mm:ss",
                        ],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "MM/dd-HH:mm:ss.SSSSSS",
                            "MM/dd/yy-HH:mm:ss.SSSSSS",
                            "MMM  d HH:mm:ss",
                            "MMM dd HH:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            if event.has_value("destination.address") {
                if let Some(val) = event.get("destination.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "destination.address".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }

            if event.has_value("destination.mac") {
                map_strings(
                    event,
                    "destination.mac",
                    "destination.mac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("[.:]"),
                    "-",
                )?;
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("[.:]"),
                    "-",
                )?;
            }

            if event.has_value("snort.tcp.flags") {
                gsub_field(
                    event,
                    "snort.tcp.flags",
                    "snort.tcp.flags",
                    cached_regex!("\\*"),
                    "",
                )?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            event.set("event.kind", json!("alert"))?;

            event.append_unique("event.category", json!("network"))?;

            let _cond = { !event.has_value("network.direction") };
            if _cond {
                event.set("network.direction", json!("unknown"))?;
            }

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = {
                event.has_value("source.ip")
                    && !(event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { event.has_value("network.transport") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def transport = ctx.network.transport;\nif (transport == 'udp') {\n    ctx.network.iana_number = '17';\n} else if (transport == 'tcp') {\n    ctx.network.iana_number = '6';\n} else if (transport == 'icmp') {\n    ctx.network.iana_number = '1';\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def transport = ctx.network.transport;\nif (transport == 'udp') {\n    ctx.network.iana_number = '17';\n} else if (transport == 'tcp') {\n    ctx.network.iana_number = '6';\n} else if (transport == 'icmp') {\n    ctx.network.iana_number = '1';\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            // Classify network direction against the internal network ranges
            if let (Some(src), Some(dst)) = (
                event.get_string("source.ip"),
                event.get_string("destination.ip"),
            ) {
                if let Some(listed) = event.get_array("_tmp.internal_networks") {
                    let listed: Vec<String> = listed
                        .iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect();
                    let networks: Vec<&str> = listed.iter().map(String::as_str).collect();
                    let direction = match (
                        ip_in_networks(&src, &networks),
                        ip_in_networks(&dst, &networks),
                    ) {
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
                if event.has_value("source.ip") {
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
                        let src_port =
                            u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                        let dst_port =
                            u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
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
                }
                Ok(())
            })();

            let _cond = {
                event
                    .get_str("_tmp.action")
                    .is_some_and(|s| s.to_lowercase() == "allow")
            };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = {
                event
                    .get_str("_tmp.action")
                    .is_some_and(|s| s.to_lowercase() == "block")
            };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
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

            event.remove("_tmp");
            event.remove("json");

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
