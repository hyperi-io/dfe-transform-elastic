// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `events` pipeline.
pub struct Events;

impl Transform for Events {
    fn name(&self) -> &str {
        "events"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(input) = event.get_string("event.original") {
                let mut remaining: &str = &input;
                if let Some(pos) = remaining.find(" events ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" events ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("msgtype", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "site-to-site")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("Site-to-Site VPN"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "client_vpn_connect")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("client_vpn_connect"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "blocked")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("blocked"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "auth")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("auth"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "port")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("port"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "carrier_change")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("carrier_change"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.starts_with("type="))
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    if let Some(pos) = remaining.find(" events type=") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" events type=") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("type", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                }
            }

            let _cond = { event.has_value("type") };
            if _cond {
                event.rename("type", "cisco_meraki.event_subtype")?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "dhcp")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    if let Some(pos) = remaining.find(" events dhcp ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" events dhcp ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("_temp.dhcp_op", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("_temp.dhcp_op2", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                }
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "dhcp")
            };
            if _cond {
                event.set("network.protocol", json!("dhcp"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "dhcp")
                    && event
                        .get_str("_temp.dhcp_op")
                        .is_some_and(|s| s.to_lowercase() == "lease")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    if let Some(pos) = remaining.find(" events dhcp lease of ip ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" events dhcp lease of ip ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" from ") {
                        event.set("_temp.client_ip", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" from ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" mac ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" mac ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" for client mac ") {
                        event.set("server.mac", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" for client mac ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("client.mac", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                }
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "dhcp")
                    && event
                        .get_str("_temp.dhcp_op")
                        .is_some_and(|s| s.to_lowercase() == "no")
                    && event
                        .get_str("_temp.dhcp_op2")
                        .is_some_and(|s| s.to_lowercase() == "offers")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    if let Some(pos) = remaining.find(" events dhcp no offers for mac ") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" events dhcp no offers for mac ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" ") {
                        event.set("client.mac", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" ") {
                        remaining = rest;
                    }
                }
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "dhcp")
                    && event.get_str("_temp.dhcp_op") == Some("lease")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("dhcp_offer"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "dhcp")
                    && event
                        .get_str("_temp.dhcp_op")
                        .is_some_and(|s| s.to_lowercase() == "no")
                    && event
                        .get_str("_temp.dhcp_op2")
                        .is_some_and(|s| s.to_lowercase() == "offers")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("dhcp_no_offer"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "dhcp")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: events dhcp %{GREEDYDATA:message}$
                    if !cached_grok!("events dhcp %{GREEDYDATA:message}$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("<"))
                    && event.get_str("cisco_meraki.event_subtype") == Some("Site-to-Site VPN")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?i)Site-to-Site VPN:%{GREEDYDATA:cisco_meraki.site_to_site_vpn.raw}
                    if !cached_grok!("(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?i)Site-to-Site VPN:%{GREEDYDATA:cisco_meraki.site_to_site_vpn.raw}").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("<"))
                    && event.get_str("cisco_meraki.event_subtype") == Some("blocked")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>(?P<_temp_blocked_arp>(?:Blocked ARP Packet)) from %{MAC:source.mac} with IP %{IP:source.ip} on %{NOTSPACE} %{GREEDYDATA:observer.ingress.vlan.id})$
                    if !cached_grok_mapped!("^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>(?P<_temp_blocked_arp>(?:Blocked ARP Packet)) from %{MAC:source.mac} with IP %{IP:source.ip} on %{NOTSPACE} %{GREEDYDATA:observer.ingress.vlan.id})$", [("_temp_blocked_arp", "_temp.blocked_arp")]).extract_into(&input, event)? {
                    }
                }
            }

            if event.has("source.mac") {
                if let Some(s) = event.get_string("source.mac") {
                    let re = cached_regex!("[:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("source.mac", replaced)?;
                }
            }

            if event.has("source.mac") {
                if let Some(s) = event.get_string("source.mac") {
                    let uppered = s.to_uppercase();
                    event.set("source.mac", uppered)?;
                }
            }

            let _cond = { event.has_value("_temp.blocked_arp") };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("arp_blocked"))?;
            }

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("<"))
                    && event.get_str("cisco_meraki.event_subtype") == Some("port")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^(?i)(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>port %{NOTSPACE:cisco_meraki.port} (?P<_temp_port_action>(?:(?:changed stp role|status changed)))(?: from %{NOTSPACE:cisco_meraki.old_port_status} to %{NOTSPACE:cisco_meraki.new_port_status}|.*))$
                    if !cached_grok_mapped!("^(?i)(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>port %{NOTSPACE:cisco_meraki.port} (?P<_temp_port_action>(?:(?:changed stp role|status changed)))(?: from %{NOTSPACE:cisco_meraki.old_port_status} to %{NOTSPACE:cisco_meraki.new_port_status}|.*))$", [("_temp_port_action", "_temp.port_action")]).extract_into(&input, event)? {
                    }
                }
            }

            if event.has("_temp.port_action") {
                if let Some(s) = event.get_string("_temp.port_action") {
                    let re = cached_regex!(" ");
                    let replaced = re.replace_all(&s, "_").into_owned();
                    event.set("_temp.port_action", replaced)?;
                }
            }

            if event.has("_temp.port_action") {
                if let Some(s) = event.get_string("_temp.port_action") {
                    let lowered = s.to_lowercase();
                    event.set("_temp.port_action", lowered)?;
                }
            }

            let _cond = { event.has_value("_temp.port_action") };
            if _cond {
                event.set(
                    "cisco_meraki.event_subtype",
                    json!(format!(
                        "port_{}",
                        event
                            .get("_temp.port_action")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
            }

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("<"))
                    && event.get_str("cisco_meraki.event_subtype") == Some("carrier_change")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^(?i)(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events carrier_change device%{SPACE}%{NOTSPACE:cisco_meraki.mxport} up %{NOTSPACE:_temp.up}.*$
                    if !cached_grok!("^(?i)(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events carrier_change device%{SPACE}%{NOTSPACE:cisco_meraki.mxport} up %{NOTSPACE:_temp.up}.*$").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("_temp.up") == Some("true") };
            if _cond {
                event.set("cisco_meraki.new_port_status", json!("up"))?;
            }

            let _cond = { event.get_str("_temp.up") == Some("false") };
            if _cond {
                event.set("cisco_meraki.new_port_status", json!("down"))?;
            }

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("<"))
                    && [
                        "dfs_event",
                        "association",
                        "disassociation",
                        "aps_association_reject",
                        "multiple_dhcp_servers_detected",
                        "wpa_deauth",
                        "wpa_auth",
                        "vpn_connectivity_change",
                        "8021x_eap_failure",
                        "8021x_auth",
                        "8021x_deauth",
                        "8021x_eap_success",
                        "splash_auth",
                        "device_packet_flood",
                    ]
                    .contains(&event.get_str("cisco_meraki.event_subtype").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}%{GREEDYDATA:_temp.rest}
                    if !cached_grok!("(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}%{GREEDYDATA:_temp.rest}").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = {
                event.has_value("_temp.rest")
                    && [
                        "dfs_event",
                        "association",
                        "disassociation",
                        "aps_association_reject",
                        "multiple_dhcp_servers_detected",
                        "wpa_deauth",
                        "wpa_auth",
                        "8021x_eap_failure",
                        "8021x_auth",
                        "8021x_deauth",
                        "8021x_eap_success",
                        "splash_auth",
                        "device_packet_flood",
                    ]
                    .contains(&event.get_str("cisco_meraki.event_subtype").unwrap_or(""))
            };
            if _cond {
                if let Some(kv_str) = event.get_string("_temp.rest") {
                    let mut kv_target_prefix = String::new();
                    kv_target_prefix.push_str("cisco_meraki.");
                    if let Some(segment) = event.get_str("cisco_meraki.event_subtype") {
                        kv_target_prefix.push_str(segment);
                        kv_target_prefix.push('.');
                    }
                    for pair in cached_regex!("[ \t]{1,}").split(&kv_str) {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_temp.rest".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let value = match (value.chars().next(), value.chars().last()) {
                                (Some('('), Some(')'))
                                | (Some('['), Some(']'))
                                | (Some('<'), Some('>'))
                                | (Some('"'), Some('"'))
                                | (Some('\''), Some('\''))
                                    if value.chars().count() > 1 =>
                                {
                                    &value[1..value.len() - 1]
                                }
                                _ => value,
                            };
                            if !key.is_empty() {
                                event.set(&format!("{}{}", kv_target_prefix, key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("_temp.rest")
                    && event.get_str("cisco_meraki.event_subtype")
                        == Some("vpn_connectivity_change")
            };
            if _cond {
                if let Some(kv_str) = event.get_string("_temp.rest") {
                    for pair in cached_regex!("[ \t]{1,}").split(&kv_str) {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_temp.rest".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let value = match (value.chars().next(), value.chars().last()) {
                                (Some('('), Some(')'))
                                | (Some('['), Some(']'))
                                | (Some('<'), Some('>'))
                                | (Some('"'), Some('"'))
                                | (Some('\''), Some('\''))
                                    if value.chars().count() > 1 =>
                                {
                                    &value[1..value.len() - 1]
                                }
                                _ => value,
                            };
                            if !key.is_empty() {
                                event.set(
                                    &format!(
                                        "cisco_meraki.site_to_site_vpn.connectivity_change.{}",
                                        key
                                    ),
                                    value,
                                )?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    == Some("multiple_dhcp_servers_detected")
            };
            if _cond {
                event.set("network.protocol", json!("dhcp"))?;
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    == Some("multiple_dhcp_servers_detected")
            };
            if _cond {
                event.rename(
                    "cisco_meraki.multiple_dhcp_servers_detected.original_server_mac",
                    "server.mac",
                )?;
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    == Some("multiple_dhcp_servers_detected")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string(
                        "cisco_meraki.multiple_dhcp_servers_detected.original_server_ip",
                    ) {
                        // Grok pattern: ^%{IPV4:cisco_meraki.multiple_dhcp_servers_detected.original_server_ip}$
                        if !cached_grok!("^%{IPV4:cisco_meraki.multiple_dhcp_servers_detected.original_server_ip}$").extract_into(&input, event)? {
                        // Grok pattern: ^%{IPV6:cisco_meraki.multiple_dhcp_servers_detected.original_server_ip}$
                        if !cached_grok!("^%{IPV6:cisco_meraki.multiple_dhcp_servers_detected.original_server_ip}$").extract_into(&input, event)? {
                        }
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    == Some("multiple_dhcp_servers_detected")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string(
                        "cisco_meraki.multiple_dhcp_servers_detected.original_server_ip",
                    ) {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path:
                                    "cisco_meraki.multiple_dhcp_servers_detected.original_server_ip"
                                        .into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("server.ip", s)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
                if event
                    .remove("cisco_meraki.multiple_dhcp_servers_detected.original_server_ip")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "cisco_meraki.multiple_dhcp_servers_detected.original_server_ip"
                            .into(),
                    });
                }
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    == Some("multiple_dhcp_servers_detected")
            };
            if _cond {
                event.append(
                    "related.ip",
                    event.get("server.ip").cloned().unwrap_or(Value::Null),
                )?;
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    == Some("multiple_dhcp_servers_detected")
            };
            if _cond {
                if let Some(input) =
                    event.get_string("cisco_meraki.multiple_dhcp_servers_detected.server_ip")
                {
                    // Grok pattern: ^%{IPV4:cisco_meraki.multiple_dhcp_servers_detected.server_ip}$
                    if !cached_grok!(
                        "^%{IPV4:cisco_meraki.multiple_dhcp_servers_detected.server_ip}$"
                    )
                    .extract_into(&input, event)?
                    {
                        // Grok pattern: ^%{IPV6:cisco_meraki.multiple_dhcp_servers_detected.server_ip}$
                        if !cached_grok!(
                            "^%{IPV6:cisco_meraki.multiple_dhcp_servers_detected.server_ip}$"
                        )
                        .extract_into(&input, event)?
                        {}
                    }
                }
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    == Some("multiple_dhcp_servers_detected")
            };
            if _cond {
                if let Some(s) =
                    event.get_string("cisco_meraki.multiple_dhcp_servers_detected.server_ip")
                {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "cisco_meraki.multiple_dhcp_servers_detected.server_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("cisco_meraki.multiple_dhcp_servers_detected.server_ip", s)?;
                }
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    == Some("multiple_dhcp_servers_detected")
            };
            if _cond {
                event.append(
                    "related.ip",
                    event
                        .get("cisco_meraki.multiple_dhcp_servers_detected.server_ip")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }

            let _cond = { event.get_str("cisco_meraki.event_subtype") == Some("wpa_deauth") };
            if _cond {
                event.rename("cisco_meraki.wpa_deauth.client_mac", "client.mac")?;
            }

            let _cond =
                { event.get_str("cisco_meraki.event_subtype") == Some("client_vpn_connect") };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    if let Some(pos) = remaining.find(" events client_vpn_connect user id '") {
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) =
                        remaining.strip_prefix(" events client_vpn_connect user id '")
                    {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find("' local ip ") {
                        event.set("user.name", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix("' local ip ") {
                        remaining = rest;
                    }
                    if let Some(pos) = remaining.find(" connected from ") {
                        event.set("network.forwarded_ip", &remaining[..pos])?;
                        remaining = &remaining[pos..];
                    }
                    if let Some(rest) = remaining.strip_prefix(" connected from ") {
                        remaining = rest;
                    }
                    event.set("_temp.client_ip", remaining)?;
                }
            }

            let _cond =
                { event.get_str("cisco_meraki.event_subtype") == Some("client_vpn_connect") };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: events client_vpn_connect %{GREEDYDATA:message}$
                    if !cached_grok!("events client_vpn_connect %{GREEDYDATA:message}$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            let _cond = { event.has_value("_temp.client_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp.client_ip") {
                        // Grok pattern: ^%{IPV4:_temp.client_ip}$
                        if !cached_grok!("^%{IPV4:_temp.client_ip}$").extract_into(&input, event)? {
                            // Grok pattern: ^%{IPV6:_temp.client_ip}$
                            if !cached_grok!("^%{IPV6:_temp.client_ip}$")
                                .extract_into(&input, event)?
                            {}
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.client_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("_temp.client_ip") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "_temp.client_ip".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("client.ip", s)?;
                    }
                    Ok(())
                })();
            }

            if event.has("client.mac") {
                if let Some(s) = event.get_string("client.mac") {
                    let re = cached_regex!("[:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("client.mac", replaced)?;
                }
            }

            if event.has("client.mac") {
                if let Some(s) = event.get_string("client.mac") {
                    let uppered = s.to_uppercase();
                    event.set("client.mac", uppered)?;
                }
            }

            if event.has("server.mac") {
                if let Some(s) = event.get_string("server.mac") {
                    let re = cached_regex!("[:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("server.mac", replaced)?;
                }
            }

            if event.has("server.mac") {
                if let Some(s) = event.get_string("server.mac") {
                    let uppered = s.to_uppercase();
                    event.set("server.mac", uppered)?;
                }
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
                    event
                        .get("_ingest.on_failure_message")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
