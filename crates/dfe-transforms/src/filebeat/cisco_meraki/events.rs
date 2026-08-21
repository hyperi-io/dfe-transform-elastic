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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" events ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" events ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("message", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

            if let Some(input) = event.get_string("event.original") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" events ") else {
                        break 'dissect false;
                    };
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" events ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else {
                        break 'dissect false;
                    };
                    captured.push(("msgtype", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else {
                        break 'dissect false;
                    };
                    remaining = rest;
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                } else {
                    return Err(TransformError::ParseError {
                        path: "event.original".into(),
                        message: "dissect pattern did not match".into(),
                    });
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
                    .is_some_and(|s| s.to_lowercase() == "anyconnect_vpn_connect")
            };
            if _cond {
                event.set(
                    "cisco_meraki.event_subtype",
                    json!("anyconnect_vpn_connect"),
                )?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "anyconnect_vpn_disconnect")
            };
            if _cond {
                event.set(
                    "cisco_meraki.event_subtype",
                    json!("anyconnect_vpn_disconnect"),
                )?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "splash_auth")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("splash_auth"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.to_lowercase() == "martian_vlan")
            };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("martian_vlan"))?;
            }

            let _cond = {
                event
                    .get_str("msgtype")
                    .is_some_and(|s| s.starts_with("type="))
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" events type=") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" events type=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "event.original".into(),
                            message: "dissect pattern did not match".into(),
                        });
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
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" events dhcp ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" events dhcp ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp.dhcp_op", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp.dhcp_op2", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "event.original".into(),
                            message: "dissect pattern did not match".into(),
                        });
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
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" events dhcp lease of ip ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" events dhcp lease of ip ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" from ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp.client_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" mac ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" mac ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" for client mac ") else {
                            break 'dissect false;
                        };
                        captured.push(("server.mac", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" for client mac ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("client.mac", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "event.original".into(),
                            message: "dissect pattern did not match".into(),
                        });
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
                    // Grok pattern: events dhcp no offers for mac %{MAC:client.mac}
                    if !cached_grok!("events dhcp no offers for mac %{MAC:client.mac}")
                        .extract_into(&input, event)?
                    {}
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
                        // Grok pattern: ^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>(?P<_temp_blocked_ra>(?:Blocked RA Packet)) from %{MAC:source.mac} \\(%{IP:source.ip}\\) on VLAN %{WORD:observer.ingress.vlan.id}(?: by default)?)$
                        if !cached_grok_mapped!("^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>(?P<_temp_blocked_ra>(?:Blocked RA Packet)) from %{MAC:source.mac} \\(%{IP:source.ip}\\) on VLAN %{WORD:observer.ingress.vlan.id}(?: by default)?)$", [("_temp_blocked_ra", "_temp.blocked_ra")]).extract_into(&input, event)? {
                            // Grok pattern: ^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>(?P<_temp_blocked_dhcp>(?:Blocked DHCP Packet)) from %{MAC:source.mac} \\(%{IP:source.ip}\\) on VLAN %{WORD:observer.ingress.vlan.id}(?: by default)?)$
                            if !cached_grok_mapped!("^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>(?P<_temp_blocked_dhcp>(?:Blocked DHCP Packet)) from %{MAC:source.mac} \\(%{IP:source.ip}\\) on VLAN %{WORD:observer.ingress.vlan.id}(?: by default)?)$", [("_temp_blocked_dhcp", "_temp.blocked_dhcp")]).extract_into(&input, event)? {
                            }
                        }
                    }
                }
            }

            if event.has_value("source.mac") {
                if let Some(s) = event.get_string("source.mac") {
                    let re = cached_regex!("[:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("source.mac", replaced)?;
                }
            }

            if event.has_value("source.mac") {
                if let Some(s) = event.get_string("source.mac") {
                    let uppered = s.to_uppercase();
                    event.set("source.mac", uppered)?;
                }
            }

            let _cond = { event.has_value("_temp.blocked_arp") };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("arp_blocked"))?;
            }

            let _cond = { event.has_value("_temp.blocked_ra") };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("ra_blocked"))?;
            }

            let _cond = { event.has_value("_temp.blocked_dhcp") };
            if _cond {
                event.set("cisco_meraki.event_subtype", json!("dhcp_blocked"))?;
            }

            let _cond = { event.has_value("_temp.blocked_dhcp") };
            if _cond {
                event.set("network.protocol", json!("dhcp"))?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.set(
                    "_temp.event_original_lower",
                    json!(
                        event
                            .get("event.original")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has_value("_temp.event_original_lower") {
                if let Some(s) = event.get_string("_temp.event_original_lower") {
                    let lowered = s.to_lowercase();
                    event.set("_temp.event_original_lower", lowered)?;
                }
            }

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("<"))
                    && event.get_str("cisco_meraki.event_subtype") == Some("port")
                    && event.has_value("_temp.event_original_lower")
                    && (event
                        .get("_temp.event_original_lower")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("status changed"))
                            }
                            serde_json::Value::String(s) => s.contains("status changed"),
                            _ => false,
                        })
                        || event
                            .get("_temp.event_original_lower")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("changed stp role"))
                                }
                                serde_json::Value::String(s) => s.contains("changed stp role"),
                                _ => false,
                            }))
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^(?i)(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>port %{NOTSPACE:cisco_meraki.port} (?P<_temp_port_action>(?:(?:changed stp role|status changed)))(?: from %{NOTSPACE:cisco_meraki.old_port_status} to %{NOTSPACE:cisco_meraki.new_port_status}|.*))$
                    if !cached_grok_mapped!("^(?i)(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:\\b(?:\\d{1,2})\\b))%{SPACE}%{NUMBER}%{SPACE}(?:(?:%{WORD}|%{HOSTNAME}))%{SPACE}events%{SPACE}(?P<message>port %{NOTSPACE:cisco_meraki.port} (?P<_temp_port_action>(?:(?:changed stp role|status changed)))(?: from %{NOTSPACE:cisco_meraki.old_port_status} to %{NOTSPACE:cisco_meraki.new_port_status}|.*))$", [("_temp_port_action", "_temp.port_action")]).extract_into(&input, event)? {
                    }
                }
            }

            if event.has_value("_temp.port_action") {
                if let Some(s) = event.get_string("_temp.port_action") {
                    let re = cached_regex!(" ");
                    let replaced = re.replace_all(&s, "_").into_owned();
                    event.set("_temp.port_action", replaced)?;
                }
            }

            if event.has_value("_temp.port_action") {
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
                        "8021x_client_deauth",
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
                        "8021x_client_deauth",
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
                    for pair in cached_regex!("[ \t]{1,}").split(&kv_str).into_iter() {
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
                    for pair in cached_regex!("[ \t]{1,}").split(&kv_str).into_iter() {
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
                event.has_value("cisco_meraki.event_subtype")
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
                        "8021x_client_deauth",
                        "8021x_eap_success",
                        "device_packet_flood",
                    ]
                    .contains(&event.get_str("cisco_meraki.event_subtype").unwrap_or(""))
            };
            if _cond {
                if let Some(from) = resolve_path(
                    event,
                    "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac",
                ) && let Some(to) = resolve_path(event, "client.mac")
                    && event.has(&from)
                {
                    event.rename(&from, &to)?;
                }
            }

            let _cond = {
                event.has_value("cisco_meraki.event_subtype")
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
                        "8021x_client_deauth",
                        "8021x_eap_success",
                        "splash_auth",
                        "device_packet_flood",
                    ]
                    .contains(&event.get_str("cisco_meraki.event_subtype").unwrap_or(""))
            };
            if _cond {
                if let Some(from) = resolve_path(
                    event,
                    "cisco_meraki.{{{cisco_meraki.event_subtype}}}.ip_src",
                ) && let Some(to) = resolve_path(event, "source.ip")
                    && event.has(&from)
                {
                    event.rename(&from, &to)?;
                }
            }

            let _cond = {
                event.has_value("cisco_meraki.event_subtype")
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
                        "8021x_client_deauth",
                        "8021x_eap_success",
                        "splash_auth",
                        "device_packet_flood",
                    ]
                    .contains(&event.get_str("cisco_meraki.event_subtype").unwrap_or(""))
            };
            if _cond {
                if let Some(from) = resolve_path(
                    event,
                    "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_ip",
                ) && let Some(to) = resolve_path(event, "_temp.client_ip")
                    && event.has(&from)
                {
                    event.rename(&from, &to)?;
                }
            }

            let _cond = {
                [
                    "association",
                    "disassociation",
                    "8021x_eap_failure",
                    "8021x_auth",
                    "8021x_deauth",
                    "8021x_client_deauth",
                    "8021x_eap_success",
                ]
                .contains(&event.get_str("cisco_meraki.event_subtype").unwrap_or(""))
            };
            if _cond {
                if let Some(from) = resolve_path(
                    event,
                    "cisco_meraki.{{{cisco_meraki.event_subtype}}}.identity",
                ) && let Some(to) = resolve_path(event, "user.name")
                    && event.has(&from)
                {
                    event.rename(&from, &to)?;
                }
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
                    json!(
                        event
                            .get("server.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
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
                    json!(
                        event
                            .get("cisco_meraki.multiple_dhcp_servers_detected.server_ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.get_str("cisco_meraki.event_subtype") == Some("client_vpn_connect") };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^%{DATA} events client_vpn_connect user id '%{DATA:user.name}' local ip %{IP:network.forwarded_ip} (reconnected from|connected from) %{IP:_temp.client_ip}$
                    if !cached_grok!("^%{DATA} events client_vpn_connect user id '%{DATA:user.name}' local ip %{IP:network.forwarded_ip} (reconnected from|connected from) %{IP:_temp.client_ip}$").extract_into(&input, event)? {
                        // Grok pattern: ^%{GREEDYDATA}$
                        if !cached_grok!("^%{GREEDYDATA}$").extract_into(&input, event)? {
                        }
                    }
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

            let _cond = {
                event.get_str("cisco_meraki.event_subtype")
                    == Some("anyconnect_vpn_session_manager")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: msg= ?'(?P<_temp_left>(?:[^:]*)): %{DATA:_temp.right}(?: Reason: %{DATA:cisco_meraki.anyconnect_vpn_session_manager.reason})? ?'
                        if !cached_grok_mapped!("msg= ?'(?P<_temp_left>(?:[^:]*)): %{DATA:_temp.right}(?: Reason: %{DATA:cisco_meraki.anyconnect_vpn_session_manager.reason})? ?'", [("_temp_left", "_temp.left")]).extract_into(&input, event)? {
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.left") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp.left") {
                        // Grok pattern: (?:Sess-ID\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_session_id>(?:[^\\]]*))\\])
                        if !cached_grok_mapped!("(?:Sess-ID\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_session_id>(?:[^\\]]*))\\])", [("cisco_meraki_anyconnect_vpn_session_manager_session_id", "cisco_meraki.anyconnect_vpn_session_manager.session_id")]).extract_into(&input, event)? {
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.left") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp.left") {
                        // Grok pattern: (?:User\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_user_name>(?:[^\\]]*))\\])
                        if !cached_grok_mapped!("(?:User\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_user_name>(?:[^\\]]*))\\])", [("cisco_meraki_anyconnect_vpn_session_manager_user_name", "cisco_meraki.anyconnect_vpn_session_manager.user_name")]).extract_into(&input, event)? {
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.left") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp.left") {
                        // Grok pattern: Peer IP=%{IP:cisco_meraki.anyconnect_vpn_session_manager.peer_ip}
                        if !cached_grok!(
                            "Peer IP=%{IP:cisco_meraki.anyconnect_vpn_session_manager.peer_ip}"
                        )
                        .extract_into(&input, event)?
                        {}
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.right") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_temp.right") {
                        // Grok pattern: ^(?:(?:(?:conn_id\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_conn_id>(?:[^\\]]*))\\]) (?P<cisco_meraki_anyconnect_vpn_session_manager_action>(?:Added)) (?:%{WORD:cisco_meraki.anyconnect_vpn_session_manager.tunnel_type} tunnel\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_tunnel_id>(?:[^\\]]*))\\]) to DB)|(?:(?P<cisco_meraki_anyconnect_vpn_session_manager_action>(?:Deleted)) (?:%{WORD:cisco_meraki.anyconnect_vpn_session_manager.tunnel_type} tunnel\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_tunnel_id>(?:[^\\]]*))\\]) from DB\\.)|(?:Applied VPN (?:filter\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_filter>(?:[^\\]]*))\\]) for assigned IP %{IP:cisco_meraki.anyconnect_vpn_session_manager.ip})|(?:Session (?P<cisco_meraki_anyconnect_vpn_session_manager_action>(?:disconnected))\\. Session Type: %{WORD:cisco_meraki.anyconnect_vpn_session_manager.session_type}, Duration: %{NOTSPACE:cisco_meraki.anyconnect_vpn_session_manager.duration}, Bytes xmt: %{NUMBER:cisco_meraki.anyconnect_vpn_session_manager.bytes_out}, Bytes rcv: %{NUMBER:cisco_meraki.anyconnect_vpn_session_manager.bytes_in},?))$
                        if !cached_grok_mapped!("^(?:(?:(?:conn_id\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_conn_id>(?:[^\\]]*))\\]) (?P<cisco_meraki_anyconnect_vpn_session_manager_action>(?:Added)) (?:%{WORD:cisco_meraki.anyconnect_vpn_session_manager.tunnel_type} tunnel\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_tunnel_id>(?:[^\\]]*))\\]) to DB)|(?:(?P<cisco_meraki_anyconnect_vpn_session_manager_action>(?:Deleted)) (?:%{WORD:cisco_meraki.anyconnect_vpn_session_manager.tunnel_type} tunnel\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_tunnel_id>(?:[^\\]]*))\\]) from DB\\.)|(?:Applied VPN (?:filter\\[(?P<cisco_meraki_anyconnect_vpn_session_manager_filter>(?:[^\\]]*))\\]) for assigned IP %{IP:cisco_meraki.anyconnect_vpn_session_manager.ip})|(?:Session (?P<cisco_meraki_anyconnect_vpn_session_manager_action>(?:disconnected))\\. Session Type: %{WORD:cisco_meraki.anyconnect_vpn_session_manager.session_type}, Duration: %{NOTSPACE:cisco_meraki.anyconnect_vpn_session_manager.duration}, Bytes xmt: %{NUMBER:cisco_meraki.anyconnect_vpn_session_manager.bytes_out}, Bytes rcv: %{NUMBER:cisco_meraki.anyconnect_vpn_session_manager.bytes_in},?))$", [("cisco_meraki_anyconnect_vpn_session_manager_action", "cisco_meraki.anyconnect_vpn_session_manager.action"), ("cisco_meraki_anyconnect_vpn_session_manager_action", "cisco_meraki.anyconnect_vpn_session_manager.action"), ("cisco_meraki_anyconnect_vpn_session_manager_action", "cisco_meraki.anyconnect_vpn_session_manager.action"), ("cisco_meraki_anyconnect_vpn_session_manager_conn_id", "cisco_meraki.anyconnect_vpn_session_manager.conn_id"), ("cisco_meraki_anyconnect_vpn_session_manager_tunnel_id", "cisco_meraki.anyconnect_vpn_session_manager.tunnel_id"), ("cisco_meraki_anyconnect_vpn_session_manager_tunnel_id", "cisco_meraki.anyconnect_vpn_session_manager.tunnel_id"), ("cisco_meraki_anyconnect_vpn_session_manager_filter", "cisco_meraki.anyconnect_vpn_session_manager.filter")]).extract_into(&input, event)? {
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("cisco_meraki.anyconnect_vpn_session_manager.action") == Some("Added")
            };
            if _cond {
                event.set(
                    "cisco_meraki.anyconnect_vpn_session_manager.action",
                    json!("added tunnel"),
                )?;
            }

            let _cond = {
                event.get_str("cisco_meraki.anyconnect_vpn_session_manager.action")
                    == Some("Deleted")
            };
            if _cond {
                event.set(
                    "cisco_meraki.anyconnect_vpn_session_manager.action",
                    json!("deleted tunnel"),
                )?;
            }

            let _cond = {
                event.get_str("cisco_meraki.anyconnect_vpn_session_manager.action")
                    == Some("disconnected")
            };
            if _cond {
                event.set(
                    "cisco_meraki.anyconnect_vpn_session_manager.action",
                    json!("session disconnected"),
                )?;
            }

            let _cond =
                { event.get_str("cisco_meraki.event_subtype") == Some("anyconnect_vpn_connect") };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^%{DATA} events anyconnect_vpn_connect user id '%{DATA:user.name}' local ip %{IP:network.forwarded_ip} (reconnected from|connected from) %{IP:_temp.client_ip}$
                    if !cached_grok!("^%{DATA} events anyconnect_vpn_connect user id '%{DATA:user.name}' local ip %{IP:network.forwarded_ip} (reconnected from|connected from) %{IP:_temp.client_ip}$").extract_into(&input, event)? {
                        // Grok pattern: ^%{GREEDYDATA}$
                        if !cached_grok!("^%{GREEDYDATA}$").extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond =
                { event.get_str("cisco_meraki.event_subtype") == Some("anyconnect_vpn_connect") };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: events anyconnect_vpn_connect %{GREEDYDATA:message}$
                    if !cached_grok!("events anyconnect_vpn_connect %{GREEDYDATA:message}$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype") == Some("anyconnect_vpn_disconnect")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) =
                            remaining.find(" events anyconnect_vpn_disconnect user id '")
                        else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" events anyconnect_vpn_disconnect user id '")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("' local ip ") else {
                            break 'dissect false;
                        };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("' local ip ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" connected from ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.forwarded_ip", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" connected from ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp.client_ip", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "event.original".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype") == Some("anyconnect_vpn_disconnect")
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: events anyconnect_vpn_disconnect %{GREEDYDATA:message}$
                    if !cached_grok!("events anyconnect_vpn_disconnect %{GREEDYDATA:message}$")
                        .extract_into(&input, event)?
                    {}
                }
            }

            let _cond = {
                event.get_str("cisco_meraki.event_subtype") == Some("splash_auth")
                    && event.has_value("cisco_meraki.splash_auth.mac")
            };
            if _cond {
                if let Some(v) = event.get("cisco_meraki.splash_auth.mac").cloned() {
                    event.set("client.mac", v)?;
                }
            }

            let _cond = { event.get_str("cisco_meraki.event_subtype") == Some("martian_vlan") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("martian_vlan ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("martian_vlan ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("_temp.martian_vlan", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "message".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.has_value("_temp.martian_vlan") };
            if _cond {
                if let Some(kv_str) = event.get_string("_temp.martian_vlan") {
                    for pair in kv_str.split("' ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("='") else {
                            return Err(TransformError::ParseError {
                                path: "_temp.martian_vlan".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                event.set(&format!("cisco_meraki.martian_vlan.{}", key), value)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("cisco_meraki.martian_vlan.Client") };
            if _cond {
                if let Some(v) = event.get("cisco_meraki.martian_vlan.Client").cloned() {
                    event.set("_temp.client_ip", v)?;
                }
            }

            let _cond = { event.has_value("cisco_meraki.martian_vlan.MAC") };
            if _cond {
                if let Some(v) = event.get("cisco_meraki.martian_vlan.MAC").cloned() {
                    event.set("client.mac", v)?;
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

            if event.has_value("client.mac") {
                if let Some(s) = event.get_string("client.mac") {
                    let re = cached_regex!("[:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("client.mac", replaced)?;
                }
            }

            if event.has_value("client.mac") {
                if let Some(s) = event.get_string("client.mac") {
                    let uppered = s.to_uppercase();
                    event.set("client.mac", uppered)?;
                }
            }

            if event.has_value("server.mac") {
                if let Some(s) = event.get_string("server.mac") {
                    let re = cached_regex!("[:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("server.mac", replaced)?;
                }
            }

            if event.has_value("server.mac") {
                if let Some(s) = event.get_string("server.mac") {
                    let uppered = s.to_uppercase();
                    event.set("server.mac", uppered)?;
                }
            }

            if event.has_value("user.name") {
                if let Some(s) = event.get_string("user.name") {
                    let lowered = s.to_lowercase();
                    event.set("user.name", lowered)?;
                }
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("user.name")
                    && event.get("user.name").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("\\")),
                        serde_json::Value::String(s) => s.contains("\\"),
                        _ => false,
                    })
            };
            if _cond {
                if event.has_value("user.name") {
                    if let Some(input) = event.get_string("user.name") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("\\") else {
                                break 'dissect false;
                            };
                            captured.push(("user.domain", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.name", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "user.name".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.email")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
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
