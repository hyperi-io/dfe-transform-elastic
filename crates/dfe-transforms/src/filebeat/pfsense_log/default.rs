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

            event.set("observer.vendor", json!("netgate"))?;

            event.set("observer.type", json!("firewall"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("event.kind", json!("event"))?;

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

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?(?:(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}(%{SPACE}(?:(?:\\b(?P<process_name>(?:[[[:alnum:]]_-]+))|\\((?P<process_name>(?:[[[:alnum:]]_-]+))\\)))|%{SPACE}(?:(?:%{IP:observer.ip}|%{HOSTNAME:observer.name}))%{SPACE}(?:(?:\\b(?P<process_name>(?:[[[:alnum:]]_-]+))|\\((?P<process_name>(?:[[[:alnum:]]_-]+))\\))))(\\[%{POSINT:process.pid:long}\\])?:)|(?:(?P<_tmp_timestamp8601>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:event.timezone}?))%{SPACE}(?:(?:%{IP:observer.ip}|%{HOSTNAME:observer.name}))%{SPACE}(?:(\\(%{DATA:process.name}\\)|(?:(?:(/([\\w_%!$@:.,+~-]+|\\\\.)*)*))(?P<process_name>(?:[[[:alnum:]]_%!$@:.,+~-]+))))%{SPACE}(%{POSINT:process.pid:long}|-) - (-|(?:\\[[^\\]]*\\]))))) %{GREEDYDATA:message}
                if !cached_grok_mapped!("^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?(?:(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}(%{SPACE}(?:(?:\\b(?P<process_name>(?:[[[:alnum:]]_-]+))|\\((?P<process_name>(?:[[[:alnum:]]_-]+))\\)))|%{SPACE}(?:(?:%{IP:observer.ip}|%{HOSTNAME:observer.name}))%{SPACE}(?:(?:\\b(?P<process_name>(?:[[[:alnum:]]_-]+))|\\((?P<process_name>(?:[[[:alnum:]]_-]+))\\))))(\\[%{POSINT:process.pid:long}\\])?:)|(?:(?P<_tmp_timestamp8601>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:event.timezone}?))%{SPACE}(?:(?:%{IP:observer.ip}|%{HOSTNAME:observer.name}))%{SPACE}(?:(\\(%{DATA:process.name}\\)|(?:(?:(/([\\w_%!$@:.,+~-]+|\\\\.)*)*))(?P<process_name>(?:[[[:alnum:]]_%!$@:.,+~-]+))))%{SPACE}(%{POSINT:process.pid:long}|-) - (-|(?:\\[[^\\]]*\\]))))) %{GREEDYDATA:message}", [("_tmp_timestamp8601", "_tmp.timestamp8601"), ("process_name", "process.name"), ("process_name", "process.name"), ("process_name", "process.name"), ("process_name", "process.name"), ("process_name", "process.name")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
            }

            let _cond = { event.has_value("_tmp.timestamp8601") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp8601") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp8601".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("event.timezone") && event.has_value("_tmp.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &["MMM  d HH:mm:ss", "MMM d HH:mm:ss", "MMM dd HH:mm:ss"],
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

            if let Some(input) = event.get_string("process.name") {
                // Grok pattern: ^(?P<event_provider>(?:\\b[A-Za-z0-9_]+(-[A-Za-z_]+)*\\b))
                if !cached_grok_mapped!(
                    "^(?P<event_provider>(?:\\b[A-Za-z0-9_]+(-[A-Za-z_]+)*\\b))",
                    [("event_provider", "event.provider")]
                )
                .extract_into(&input, event)?
                {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            let _cond = { event.get_str("event.provider") == Some("filterlog") };
            if _cond {
                // Begin nested pipeline: "firewall"
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (?:(?:%{INT},%{INT}?,,%{DATA:rule.id},%{DATA:observer.ingress.interface.name},(?P<event_reason>(?:[a-zA-Z-]+)),%{WORD:event.action},%{WORD:network.direction},)(?:(?:(?P<network_type>(4)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.ecn}?,%{NONNEGINT:pfsense.ip.ttl:long},%{NONNEGINT:pfsense.ip.id:long},%{NONNEGINT:pfsense.ip.offset:long},(?:%{WORD:pfsense.ip.flags}|(?P<pfsense_ip_flags>(?:[+]))),%{INT:network.iana_number},%{WORD:network.transport},)|(?:(?P<network_type>(6)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.flow_label},%{WORD:pfsense.ip.flags},(?P<network_transport>(?:[0-9a-zA-Z-]+)),%{INT:network.iana_number},))(?:%{NONNEGINT:network.bytes:long},%{IP:source.address},%{IP:destination.address},)(?:(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.tcp.length:long},%{WORD:pfsense.tcp.flags}?,%{NONNEGINT:pfsense.tcp.seq:long}?:?%{NONNEGINT},%{NONNEGINT:pfsense.tcp.ack:long}?,%{NONNEGINT:pfsense.tcp.window:long}?,%{WORD:pfsense.tcp.urg}?,%{GREEDYDATA:pfsense.tcp.options})|(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.udp.length:long}$)|(?:(?:(?P<pfsense_icmp_type>(request|reply|unreachproto|unreachport|unreach|timeexceed|paramprob|redirect|maskreply|needfrag|tstamp|tstampreply)),)(?:(?:%{NONNEGINT:pfsense.icmp.id:long},%{NONNEGINT:pfsense.icmp.seq:long})|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?,\\[?%{NONNEGINT:pfsense.icmp.unreachable.port:long}\\]?)|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?)|(?:%{GREEDYDATA:pfsense.icmp.unreachable.other})|(?:%{IP:pfsense.icmp.destination.ip},%{NONNEGINT:pfsense.icmp.mtu:long})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq},%{INT:pfsense.icmp.otime},%{INT:pfsense.icmp.rtime},%{INT:pfsense.icmp.ttime})))|(?:datalength=%{NONNEGINT:network.packets:long})|(?:%{GREEDYDATA})|(?:))?)%{GREEDYDATA}
                    if !cached_grok_mapped!("(?:(?:%{INT},%{INT}?,,%{DATA:rule.id},%{DATA:observer.ingress.interface.name},(?P<event_reason>(?:[a-zA-Z-]+)),%{WORD:event.action},%{WORD:network.direction},)(?:(?:(?P<network_type>(4)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.ecn}?,%{NONNEGINT:pfsense.ip.ttl:long},%{NONNEGINT:pfsense.ip.id:long},%{NONNEGINT:pfsense.ip.offset:long},(?:%{WORD:pfsense.ip.flags}|(?P<pfsense_ip_flags>(?:[+]))),%{INT:network.iana_number},%{WORD:network.transport},)|(?:(?P<network_type>(6)),%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.flow_label},%{WORD:pfsense.ip.flags},(?P<network_transport>(?:[0-9a-zA-Z-]+)),%{INT:network.iana_number},))(?:%{NONNEGINT:network.bytes:long},%{IP:source.address},%{IP:destination.address},)(?:(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.tcp.length:long},%{WORD:pfsense.tcp.flags}?,%{NONNEGINT:pfsense.tcp.seq:long}?:?%{NONNEGINT},%{NONNEGINT:pfsense.tcp.ack:long}?,%{NONNEGINT:pfsense.tcp.window:long}?,%{WORD:pfsense.tcp.urg}?,%{GREEDYDATA:pfsense.tcp.options})|(?:%{INT:source.port:long},%{INT:destination.port:long},%{NONNEGINT:pfsense.udp.length:long}$)|(?:(?:(?P<pfsense_icmp_type>(request|reply|unreachproto|unreachport|unreach|timeexceed|paramprob|redirect|maskreply|needfrag|tstamp|tstampreply)),)(?:(?:%{NONNEGINT:pfsense.icmp.id:long},%{NONNEGINT:pfsense.icmp.seq:long})|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?,\\[?%{NONNEGINT:pfsense.icmp.unreachable.port:long}\\]?)|(?:\\[?%{IP:pfsense.icmp.destination.ip}\\]?,\\[?%{WORD:pfsense.icmp.unreachable.protocol_id}\\]?)|(?:%{GREEDYDATA:pfsense.icmp.unreachable.other})|(?:%{IP:pfsense.icmp.destination.ip},%{NONNEGINT:pfsense.icmp.mtu:long})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq})|(?:%{INT:pfsense.icmp.id},%{INT:pfsense.icmp.seq},%{INT:pfsense.icmp.otime},%{INT:pfsense.icmp.rtime},%{INT:pfsense.icmp.ttime})))|(?:datalength=%{NONNEGINT:network.packets:long})|(?:%{GREEDYDATA})|(?:))?)%{GREEDYDATA}", [("event_reason", "event.reason"), ("pfsense_ip_flags", "pfsense.ip.flags"), ("network_transport", "network.transport"), ("network_type", "network.type"), ("network_type", "network.type"), ("pfsense_icmp_type", "pfsense.icmp.type")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                }
                event.set("event.kind", json!("event"))?;
                let _cond =
                    { event.has_value("source.address") && event.has_value("destination.address") };
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
                    map_strings(
                        event,
                        "network.transport",
                        "network.transport",
                        str::to_lowercase,
                    )?;
                }
                let _cond =
                    { !event.has_value("ack_number") || event.get_str("ack_number") == Some("") };
                if _cond {
                    event.remove("ack_number");
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
                    if event.has_value("pfsense.tcp.options") {
                        if let Some(s) = event.get_string("pfsense.tcp.options") {
                            let mut parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
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
                // End nested pipeline: "firewall"
            }

            let _cond = { event.get_str("event.provider") == Some("openvpn") };
            if _cond {
                // Begin nested pipeline: "openvpn"
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{SPACE}peer%{SPACE}info:%{SPACE}%{GREEDYDATA:pfsense.openvpn.peer_info}
                    // Grok pattern: (?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{SPACE}\\[(?P<user_name>(?:[a-zA-Z0-9._-]+))\\]%{SPACE}%{GREEDYDATA}
                    // Grok pattern: user%{SPACE}'(?P<user_name>(?:[a-zA-Z0-9._-]+))'%{GREEDYDATA}
                    // Grok pattern: (?P<user_name>(?:[a-zA-Z0-9._-]+))/(?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{DATA}IPv4=(%{IP:source.nat.ip}|%{GREEDYDATA}),%{SPACE}IPv6=(%{IP:source.nat.ip}|%{GREEDYDATA})
                    // Grok pattern: %{GREEDYDATA}(?:%{IP:source.address}:%{NONNEGINT:source.port:long})
                    // Grok pattern: %{GREEDYDATA}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "(?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{SPACE}peer%{SPACE}info:%{SPACE}%{GREEDYDATA:pfsense.openvpn.peer_info}"
                            ),
                            cached_grok_mapped!(
                                "(?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{SPACE}\\[(?P<user_name>(?:[a-zA-Z0-9._-]+))\\]%{SPACE}%{GREEDYDATA}",
                                [("user_name", "user.name")]
                            ),
                            cached_grok_mapped!(
                                "user%{SPACE}'(?P<user_name>(?:[a-zA-Z0-9._-]+))'%{GREEDYDATA}",
                                [("user_name", "user.name")]
                            ),
                            cached_grok_mapped!(
                                "(?P<user_name>(?:[a-zA-Z0-9._-]+))/(?:%{IP:source.address}:%{NONNEGINT:source.port:long})%{DATA}IPv4=(%{IP:source.nat.ip}|%{GREEDYDATA}),%{SPACE}IPv6=(%{IP:source.nat.ip}|%{GREEDYDATA})",
                                [("user_name", "user.name")]
                            ),
                            cached_grok!(
                                "%{GREEDYDATA}(?:%{IP:source.address}:%{NONNEGINT:source.port:long})"
                            ),
                            cached_grok!("%{GREEDYDATA}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("auth")),
                        serde_json::Value::String(s) => s.contains("auth"),
                        _ => false,
                    })
                };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = { event.has_value("source.address") };
                if _cond {
                    event.append_unique("event.type", json!("connection"))?;
                }
                event.append_unique("event.type", json!("info"))?;
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("error"))
                        || event
                            .get_str("message")
                            .is_some_and(|s| s.to_lowercase().contains("not auth"))
                };
                if _cond {
                    event.append_unique("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("initiat"))
                };
                if _cond {
                    event.append_unique("event.type", json!("start"))?;
                }
                let v = json!(
                    event
                        .get("source.address")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.ip", v)?;
                }
                event.set("network.protocol", json!("openvpn"))?;
                // End nested pipeline: "openvpn"
            }

            let _cond = { event.get_str("event.provider") == Some("charon") };
            if _cond {
                // Begin nested pipeline: "ipsec"
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (?:\\d+\\[%{WORD}\\])%{GREEDYDATA}(?:%{IP:source.address}\\[%{NONNEGINT:source.port:long}\\]) to (?:%{IP:destination.address}\\[%{NONNEGINT:destination.port:long}\\]) \\(%{NONNEGINT:network.bytes:long} bytes\\)
                    // Grok pattern: %{GREEDYDATA}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "(?:\\d+\\[%{WORD}\\])%{GREEDYDATA}(?:%{IP:source.address}\\[%{NONNEGINT:source.port:long}\\]) to (?:%{IP:destination.address}\\[%{NONNEGINT:destination.port:long}\\]) \\(%{NONNEGINT:network.bytes:long} bytes\\)"
                            ),
                            cached_grok!("%{GREEDYDATA}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                let _cond = { event.has_value("source.address") };
                if _cond {
                    event.append_unique("event.type", json!("connection"))?;
                }
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("disconnected"))
                };
                if _cond {
                    event.append_unique("event.type", json!("end"))?;
                }
                let v = json!(
                    event
                        .get("source.address")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("source.ip", v)?;
                }
                let v = json!(
                    event
                        .get("destination.address")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("destination.ip", v)?;
                }
                event.set("network.protocol", json!("ipsec"))?;
                // End nested pipeline: "ipsec"
            }

            let _cond = {
                ["dhcpd", "dhclient", "dhcp6c", "dnsmasq-dhcp"]
                    .contains(&event.get_str("event.provider").unwrap_or(""))
            };
            if _cond {
                // Begin nested pipeline: "dhcp"
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{DATA:_tmp.action}\\(%{DATA:observer.ingress.interface.name}\\)(?: %{IP:client.ip})? %{MAC:client.mac}(?: %{HOSTNAME:pfsense.dhcp.hostname})?
                    // Grok pattern: %{DATA:_tmp.action}/(?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))/(?P<server_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})))/%{NOTSPACE:pfsense.dhcp.subnet}
                    // Grok pattern: %{DATA:_tmp.action} %{IPV6:client.address}(/%{NUMBER})? on (?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))
                    // Grok pattern: %{DATA:_tmp.action} (from|to) %{IPV6:client.address} port %{POSINT:client.port:long}(, transaction ID %{NOTSPACE:pfsense.dhcp.transaction_id})?
                    // Grok pattern: %{DATA:_tmp.action} for: %{IPV6:client.address}(, age %{POSINT:pfsense.dhcp.age:long} secs)?%{GREEDYDATA}
                    // Grok pattern: %{DATA:_tmp.action}: address %{IPV6:client.address} to client with duid (?P<pfsense_dhcp_duid>(?:(?i)[0-9a-f]{2}(:[0-9a-f]{2})+)) iaid = -%{NOTSPACE:pfsense.dhcp.iaid} valid for %{POSINT:pfsense.dhcp.lease_time:long} seconds
                    // Grok pattern: %{WORD:event.action} (?:(?:(?:from (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))))|(?:on %{IP:client.address} to (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))) \\(%{HOSTNAME:pfsense.dhcp.hostname}\\))|(?:for %{IP:client.address} \\(%{IP:server.address}\\)? from (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))) \\(%{HOSTNAME:pfsense.dhcp.hostname}\\)))) via (?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))
                    // Grok pattern: %{DATA:_tmp.action} %{IPV6:client.address}
                    // Grok pattern: %{GREEDYDATA}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "%{DATA:_tmp.action}\\(%{DATA:observer.ingress.interface.name}\\)(?: %{IP:client.ip})? %{MAC:client.mac}(?: %{HOSTNAME:pfsense.dhcp.hostname})?"
                            ),
                            cached_grok_mapped!(
                                "%{DATA:_tmp.action}/(?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))/(?P<server_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})))/%{NOTSPACE:pfsense.dhcp.subnet}",
                                [
                                    (
                                        "observer_ingress_interface_name",
                                        "observer.ingress.interface.name"
                                    ),
                                    ("server_mac", "server.mac")
                                ]
                            ),
                            cached_grok_mapped!(
                                "%{DATA:_tmp.action} %{IPV6:client.address}(/%{NUMBER})? on (?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))",
                                [(
                                    "observer_ingress_interface_name",
                                    "observer.ingress.interface.name"
                                )]
                            ),
                            cached_grok!(
                                "%{DATA:_tmp.action} (from|to) %{IPV6:client.address} port %{POSINT:client.port:long}(, transaction ID %{NOTSPACE:pfsense.dhcp.transaction_id})?"
                            ),
                            cached_grok!(
                                "%{DATA:_tmp.action} for: %{IPV6:client.address}(, age %{POSINT:pfsense.dhcp.age:long} secs)?%{GREEDYDATA}"
                            ),
                            cached_grok_mapped!(
                                "%{DATA:_tmp.action}: address %{IPV6:client.address} to client with duid (?P<pfsense_dhcp_duid>(?:(?i)[0-9a-f]{2}(:[0-9a-f]{2})+)) iaid = -%{NOTSPACE:pfsense.dhcp.iaid} valid for %{POSINT:pfsense.dhcp.lease_time:long} seconds",
                                [("pfsense_dhcp_duid", "pfsense.dhcp.duid")]
                            ),
                            cached_grok_mapped!(
                                "%{WORD:event.action} (?:(?:(?:from (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))))|(?:on %{IP:client.address} to (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))) \\(%{HOSTNAME:pfsense.dhcp.hostname}\\))|(?:for %{IP:client.address} \\(%{IP:server.address}\\)? from (?P<client_mac>(?:([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2}))) \\(%{HOSTNAME:pfsense.dhcp.hostname}\\)))) via (?P<observer_ingress_interface_name>(?:[a-z0-9\\.]+))",
                                [
                                    (
                                        "observer_ingress_interface_name",
                                        "observer.ingress.interface.name"
                                    ),
                                    ("client_mac", "client.mac"),
                                    ("client_mac", "client.mac"),
                                    ("client_mac", "client.mac")
                                ]
                            ),
                            cached_grok!("%{DATA:_tmp.action} %{IPV6:client.address}"),
                            cached_grok!("%{GREEDYDATA}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                event.append_unique("event.type", json!("connection"))?;
                event.append_unique("event.type", json!("protocol"))?;
                event.append_unique("event.type", json!("info"))?;
                event.set("network.protocol", json!("dhcp"))?;
                let _cond = {
                    event.get_str("event.provider") == Some("dhcp6c")
                        || (event.has_value("server.address")
                            && event.get("server.address").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(":"))
                                }
                                serde_json::Value::String(s) => s.contains(":"),
                                _ => false,
                            }))
                        || (event.has_value("client.address")
                            && event.get("client.address").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(":"))
                                }
                                serde_json::Value::String(s) => s.contains(":"),
                                _ => false,
                            }))
                };
                if _cond {
                    event.set("network.protocol", json!("dhcpv6"))?;
                }
                event.set("network.transport", json!("udp"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("client.address") {
                        if let Some(val) = event.get("client.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "client.address".into(),
                                    message,
                                }
                            })?;
                            event.set("client.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("server.address") {
                        if let Some(val) = event.get("server.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "server.address".into(),
                                    message,
                                }
                            })?;
                            event.set("server.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("client.mac") {
                    map_strings(event, "client.mac", "client.mac", str::to_uppercase)?;
                }
                if event.has_value("client.mac") {
                    gsub_field(event, "client.mac", "client.mac", cached_regex!("[:]"), "-")?;
                }
                if event.has_value("server.mac") {
                    map_strings(event, "server.mac", "server.mac", str::to_uppercase)?;
                }
                if event.has_value("server.mac") {
                    gsub_field(event, "server.mac", "server.mac", cached_regex!("[:]"), "-")?;
                }
                if event.has_value("_tmp.action") {
                    map_strings(event, "_tmp.action", "_tmp.action", str::to_lowercase)?;
                }
                if event.has_value("_tmp.action") {
                    gsub_field(
                        event,
                        "_tmp.action",
                        "event.action",
                        cached_regex!(" "),
                        "-",
                    )?;
                }
                if let Some(v) = event
                    .get("client")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source", v)?;
                }
                if let Some(v) = event
                    .get("server")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination", v)?;
                }
                gsub_field(
                    event,
                    "event.provider",
                    "event.provider",
                    cached_regex!("dnsmasq-dhcp"),
                    "dhcpd",
                )?;
                let _cond = { event.has_value("pfsense.log.dhcp.hostname") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("pfsense.dhcp.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "dhcp"
            }

            let _cond = { event.get_str("event.provider") == Some("unbound") };
            if _cond {
                // Begin nested pipeline: "unbound"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: %{LOGLEVEL:log.level}: %{IP:source.address} %{HOSTNAME:_tmp.question.name}(\\.) %{WORD:_tmp.question.type} %{WORD:_tmp.question.class}
                        if !cached_grok!("%{LOGLEVEL:log.level}: %{IP:source.address} %{HOSTNAME:_tmp.question.name}(\\.) %{WORD:_tmp.question.type} %{WORD:_tmp.question.class}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_message_88ceaee5")?;
                    return Ok(TransformResult::Drop);
                }
                let _cond = { event.has_value("source.address") };
                if _cond {
                    event.append_unique("event.type", json!("connection"))?;
                }
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("disconnected"))
                };
                if _cond {
                    event.append_unique("event.type", json!("end"))?;
                }
                event.set("network.protocol", json!("dns"))?;
                let _cond = { event.has_value("_tmp.question.name") };
                if _cond {
                    event.set("dns.type", json!("question"))?;
                }
                if event.has_value("_tmp.question.name") {
                    if let Some(domain) = event.get_string("_tmp.question.name") {
                        // Public suffix list lookup for registered domain extraction.
                        // A failed lookup writes NO target field, which is what
                        // Elasticsearch does.
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            event.set("dns.question.domain", json!(domain))?;
                            if let Some(registered) = rd.registered_domain {
                                event.set("dns.question.registered_domain", json!(registered))?;
                            }
                            event
                                .set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                            if let Some(sub) = rd.subdomain {
                                event.set("dns.question.subdomain", json!(sub))?;
                            }
                        }
                    }
                }
                if event.has_value("dns.question.domain") {
                    event.rename("dns.question.domain", "dns.question.name")?;
                }
                if event.has_value("_tmp.question.type") {
                    event.rename("_tmp.question.type", "dns.question.type")?;
                }
                if event.has_value("_tmp.question.class") {
                    event.rename("_tmp.question.class", "dns.question.class")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.address") {
                        if let Some(val) = event.get("source.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "source.address".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                if let Some(v) = event
                    .get("source")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("client", v)?;
                }
                // End nested pipeline: "unbound"
            }

            let _cond = { event.get_str("event.provider") == Some("haproxy") };
            if _cond {
                // Begin nested pipeline: "haproxy"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: Connect from (%{IPORHOST:source.address}|-):%{POSINT:source.port:long} %{WORD} %{IPORHOST:destination.address}:%{POSINT:destination.port:long} \\(%{NOTSPACE:haproxy.frontend_name}/%{WORD:haproxy.mode}\\)
                        // Grok pattern: (%{IPORHOST:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.http.request.time_wait_ms:long}/%{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:haproxy.http.request.time_wait_without_data_ms:long}/%{NUMBER:_temp.duration:long} %{NUMBER:http.response.status_code:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.http.request.captured_cookie} %{NOTSPACE:haproxy.http.response.captured_cookie} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long} (\\{%{DATA:haproxy.http.request.captured_headers}\\} \\{%{DATA:haproxy.http.response.captured_headers}\\} |\\{%{DATA}\\} )?\"%{GREEDYDATA:haproxy.http.request.raw_request_line}\"
                        // Grok pattern: (%{IP:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:_temp.duration:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long}
                        // Grok pattern: (%{IP:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name}/(?P<haproxy_bind_name>(?:((%{IP:destination.address})?(:%{POSINT:destination.port:long})?|%{NOTSPACE}))):? %{GREEDYDATA:haproxy.error_message}
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "Connect from (%{IPORHOST:source.address}|-):%{POSINT:source.port:long} %{WORD} %{IPORHOST:destination.address}:%{POSINT:destination.port:long} \\(%{NOTSPACE:haproxy.frontend_name}/%{WORD:haproxy.mode}\\)"
                                ),
                                cached_grok!(
                                    "(%{IPORHOST:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.http.request.time_wait_ms:long}/%{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:haproxy.http.request.time_wait_without_data_ms:long}/%{NUMBER:_temp.duration:long} %{NUMBER:http.response.status_code:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.http.request.captured_cookie} %{NOTSPACE:haproxy.http.response.captured_cookie} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long} (\\{%{DATA:haproxy.http.request.captured_headers}\\} \\{%{DATA:haproxy.http.response.captured_headers}\\} |\\{%{DATA}\\} )?\"%{GREEDYDATA:haproxy.http.request.raw_request_line}\""
                                ),
                                cached_grok!(
                                    "(%{IP:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:_temp.duration:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long}"
                                ),
                                cached_grok_mapped!(
                                    "(%{IP:source.address}|-):%{POSINT:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name}/(?P<haproxy_bind_name>(?:((%{IP:destination.address})?(:%{POSINT:destination.port:long})?|%{NOTSPACE}))):? %{GREEDYDATA:haproxy.error_message}",
                                    [("haproxy_bind_name", "haproxy.bind_name")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_message_27190f7e")?;
                    return Ok(TransformResult::Drop);
                }
                let _cond = {
                    event.has_value("haproxy.request_date") && !event.has_value("event.timezone")
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("haproxy.request_date") {
                        match parse_date_out(
                            &date_str,
                            &["dd/MMM/yyyy:HH:mm:ss.SSS", "MMM dd HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "haproxy.request_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = {
                    event.has_value("haproxy.request_date") && event.has_value("event.timezone")
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("haproxy.request_date") {
                        match parse_date_out(
                            &date_str,
                            &["dd/MMM/yyyy:HH:mm:ss.SSS", "MMM dd HH:mm:ss"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "haproxy.request_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                event.remove("haproxy.request_date");
                let _cond = {
                    event.has_value("haproxy.http.request.raw_request_line")
                        && event
                            .get("haproxy.http.request.raw_request_line")
                            .is_some_and(|v| !match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            })
                        && event.get_str("haproxy.http.request.raw_request_line")
                            != Some("<BADREQ>")
                };
                if _cond {
                    if event.has_value("haproxy.http.request.raw_request_line") {
                        if let Some(input) =
                            event.get_string("haproxy.http.request.raw_request_line")
                        {
                            // Grok pattern: %{WORD:http.request.method}%{SPACE}%{URIPATHPARAM:url.original}%{SPACE}HTTP/%{NUMBER:http.version}
                            if !cached_grok!("%{WORD:http.request.method}%{SPACE}%{URIPATHPARAM:url.original}%{SPACE}HTTP/%{NUMBER:http.version}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                    }
                }
                let _cond = { event.has_value("url.original") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        uri_parts(event, "url.original", "url", true, false)?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("haproxy.http.request.captured_headers") {
                        if let Some(s) = event.get_string("haproxy.http.request.captured_headers") {
                            let mut parts: Vec<Value> = cached_regex!("\\|")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                            }
                            event.set(
                                "haproxy.http.request.captured_headers",
                                Value::Array(parts),
                            )?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("haproxy.http.response.captured_headers") {
                        if let Some(s) = event.get_string("haproxy.http.response.captured_headers")
                        {
                            let mut parts: Vec<Value> = cached_regex!("\\|")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                            }
                            event.set(
                                "haproxy.http.response.captured_headers",
                                Value::Array(parts),
                            )?;
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("_temp.duration") };
                if _cond {
                    // Painless script
                    // Source: ctx.event.duration = Math.round(ctx._temp.duration * params.scale)
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.event.duration = Math.round(ctx._temp.duration * params.scale)"#
                        ),
                        cached_params!("{\"scale\":1000000}"),
                    )?;
                }
                let _cond = { event.has("http") };
                if _cond {
                    if event.has_value("haproxy.bytes_read") {
                        if let Some(val) = event.get("haproxy.bytes_read") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "haproxy.bytes_read".into(),
                                    message,
                                }
                            })?;
                            event.set("http.response.bytes", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.get_str("haproxy.mode") == Some("HTTP") || event.has_value("haproxy.http")
                };
                if _cond {
                    event.append("event.category", json!("web"))?;
                }
                let _cond =
                    { event.has_value("source.address") && event.has_value("destination.address") };
                if _cond {
                    event.append("event.type", json!("access"))?;
                }
                let _cond = {
                    event.has_value("http.response.status_code")
                        && event
                            .get_i64("http.response.status_code")
                            .is_some_and(|n| n < 400)
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.has_value("http.response.status_code")
                        && event
                            .get_i64("http.response.status_code")
                            .is_some_and(|n| n >= 400)
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                event.remove("_temp");
                event.remove("haproxy.request_date");
                // End nested pipeline: "haproxy"
            }

            let _cond = { event.get_str("event.provider") == Some("php-fpm") };
            if _cond {
                // Begin nested pipeline: "php-fpm"
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{DATA}: (?:((?:(%{DATA:_tmp.action}) for user '%{USER:user.name}' from: %{IP:source.address} \\(%{DATA}\\))|(?:User (%{DATA:_tmp.action}) for user '%{USER:user.name}' from: %{IP:source.address})|(?:webConfigurator %{DATA:_tmp.action} for user '%{DATA:user.name}' from: %{IP:source.address})))
                    // Grok pattern: ^%{GREEDYDATA}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA}: (?:((?:(%{DATA:_tmp.action}) for user '%{USER:user.name}' from: %{IP:source.address} \\(%{DATA}\\))|(?:User (%{DATA:_tmp.action}) for user '%{USER:user.name}' from: %{IP:source.address})|(?:webConfigurator %{DATA:_tmp.action} for user '%{DATA:user.name}' from: %{IP:source.address})))"
                            ),
                            cached_grok!("^%{GREEDYDATA}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                event.append_unique("event.category", json!("authentication"))?;
                let _cond = {
                    event
                        .get_str("_tmp.action")
                        .is_some_and(|s| s.to_lowercase().contains("success"))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event
                        .get_str("_tmp.action")
                        .is_some_and(|s| s.to_lowercase().contains("authentication error"))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.address") {
                        if let Some(val) = event.get("source.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "source.address".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("observer.ip") {
                    event.rename("observer.ip", "host.ip")?;
                }
                if event.has_value("observer.name") {
                    event.rename("observer.name", "host.name")?;
                }
                // End nested pipeline: "php-fpm"
            }

            let _cond = { event.get_str("event.provider") == Some("squid") };
            if _cond {
                // Begin nested pipeline: "squid"
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{IPORHOST:source.address} %{NOTSPACE:squid.request_status}/%{NUMBER:http.response.status_code:long} %{NUMBER:http.response.bytes:long} %{NOTSPACE:http.request.method} (?:%{URI:url.original}|(%{IPORHOST:url.domain}(?::%{DATA:url.port})?))?%{SPACE}%{NOTSPACE:http.request.referrer}%{SPACE}%{NOTSPACE:squid.hierarchy_status}/(?:%{IPORHOST:destination.address}|-)%{SPACE}%{NOTSPACE:http.response.mime_type}
                    if !cached_grok!("%{IPORHOST:source.address} %{NOTSPACE:squid.request_status}/%{NUMBER:http.response.status_code:long} %{NUMBER:http.response.bytes:long} %{NOTSPACE:http.request.method} (?:%{URI:url.original}|(%{IPORHOST:url.domain}(?::%{DATA:url.port})?))?%{SPACE}%{NOTSPACE:http.request.referrer}%{SPACE}%{NOTSPACE:squid.hierarchy_status}/(?:%{IPORHOST:destination.address}|-)%{SPACE}%{NOTSPACE:http.response.mime_type}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                }
                let _cond = { event.has_value("url.original") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        uri_parts(event, "url.original", "url", true, false)?;
                        Ok(())
                    })();
                }
                if event.has_value("url.port") {
                    if let Some(val) = event.get("url.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "url.port".into(),
                                message,
                            }
                        })?;
                        event.set("url.port", converted)?;
                    }
                }
                event.append("event.category", json!("web"))?;
                let _cond = {
                    event.has_value("http.response.status_code")
                        && event
                            .get_i64("http.response.status_code")
                            .is_some_and(|n| n < 400)
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.has_value("http.response.status_code")
                        && event
                            .get_i64("http.response.status_code")
                            .is_some_and(|n| n >= 400)
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                // End nested pipeline: "squid"
            }

            let _cond = { event.get_str("event.provider") == Some("snort") };
            if _cond {
                // Begin nested pipeline: "snort"
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: \\[%{NUMBER:snort.generator_id}:%{NUMBER:snort.signature_id}:%{NUMBER:snort.signature_revision}\\] \\(%{DATA:snort.preprocessor}\\) %{GREEDYDATA:snort.alert_message} \\[Classification: %{DATA:snort.classification}\\] \\[Priority: %{NONNEGINT:snort.priority:long}\\] \\{%{WORD:network.protocol}\\} %{IP:source.address}:%{NUMBER:source.port:long} -> %{IP:destination.address}:%{NUMBER:destination.port:long}
                    if !cached_grok!("\\[%{NUMBER:snort.generator_id}:%{NUMBER:snort.signature_id}:%{NUMBER:snort.signature_revision}\\] \\(%{DATA:snort.preprocessor}\\) %{GREEDYDATA:snort.alert_message} \\[Classification: %{DATA:snort.classification}\\] \\[Priority: %{NONNEGINT:snort.priority:long}\\] \\{%{WORD:network.protocol}\\} %{IP:source.address}:%{NUMBER:source.port:long} -> %{IP:destination.address}:%{NUMBER:destination.port:long}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                }
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                // End nested pipeline: "snort"
            }

            let _cond = {
                !([
                    "filterlog",
                    "openvpn",
                    "charon",
                    "dhcpd",
                    "dnsmasq-dhcp",
                    "dhclient",
                    "dhcp6c",
                    "unbound",
                    "haproxy",
                    "php-fpm",
                    "squid",
                    "snort",
                ]
                .contains(&event.get_str("event.provider").unwrap_or("")))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.has_value("network") };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("destination.address") {
                    if let Some(val) = event.get("destination.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.address".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("observer.ingress.interface.name") {
                    if let Some(input) = event.get_string("observer.ingress.interface.name") {
                        // Grok pattern: %{DATA}.%{NONNEGINT:observer.ingress.vlan.id}
                        if !cached_grok!("%{DATA}.%{NONNEGINT:observer.ingress.vlan.id}")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("observer.ingress.vlan.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.vlan.id", v)?;
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

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("network.direction")
                    && event
                        .get_str("network.direction")
                        .is_some_and(|s| cached_regex!(r"^(in|out)$").is_match(s))
            };
            if _cond {
                event.set(
                    "network.direction",
                    json!(format!(
                        "{}bound",
                        event
                            .get("network.direction")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("_tmp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_tmp".into(),
                    });
                }
                Ok(())
            })();

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || (v instanceof String && v == \"-\"));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    sentinels: vec!["-".into()],
                    ..DropPolicy::none()
                },
                None,
            );

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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("_tmp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_tmp".into(),
                        });
                    }
                    Ok(())
                })();
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
