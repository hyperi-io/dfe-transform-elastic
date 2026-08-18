// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `events` pipeline.
pub struct Events;

impl Transform for Events {
    fn name(&self) -> &str {
        "events"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        if let Some(input) = event.get_str("event.original").map(String::from) {
            let input = input.as_str();
            let mut remaining = input;
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

        // TODO: conditional: ctx.msgtype.toLowerCase() == "site-to-site"
        {
            event.set("cisco_meraki.event_subtype", json!("Site-to-Site VPN"))?;
        }

        // TODO: conditional: ctx.msgtype.toLowerCase() == "client_vpn_connect"
        {
            event.set("cisco_meraki.event_subtype", json!("client_vpn_connect"))?;
        }

        // TODO: conditional: ctx.msgtype.toLowerCase() == "blocked"
        {
            event.set("cisco_meraki.event_subtype", json!("blocked"))?;
        }

        // TODO: conditional: ctx.msgtype.toLowerCase() == "auth"
        {
            event.set("cisco_meraki.event_subtype", json!("auth"))?;
        }

        // TODO: conditional: ctx.msgtype.toLowerCase() == "port"
        {
            event.set("cisco_meraki.event_subtype", json!("port"))?;
        }

        // TODO: conditional: ctx.msgtype.toLowerCase() == "carrier_change"
        {
            event.set("cisco_meraki.event_subtype", json!("carrier_change"))?;
        }

        // TODO: conditional: ctx?.msgtype.startsWith("type=")
        {
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx?.type != null
        {
            event.rename("type", "cisco_meraki.event_subtype")?;
        }

        // TODO: conditional: ctx?.msgtype.toLowerCase() == "dhcp"
        {
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx?.msgtype.toLowerCase() == "dhcp"
        {
            event.set("network.protocol", json!("dhcp"))?;
        }

        // TODO: conditional: ctx?.msgtype.toLowerCase() == "dhcp" && ctx?._temp?.dhcp_op.toLowerCase() == 'lease'
        {
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx?.msgtype.toLowerCase() == "dhcp" && ctx?._temp?.dhcp_op.toLowerCase() == 'no' && ctx?._temp?.dhcp_op2.toLowerCase() == 'offers'
        {
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
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

        // TODO: conditional: ctx?.msgtype.toLowerCase() == "dhcp" && ctx?._temp?.dhcp_op == 'lease'
        {
            event.set("cisco_meraki.event_subtype", json!("dhcp_offer"))?;
        }

        // TODO: conditional: ctx?.msgtype.toLowerCase() == "dhcp" && ctx?._temp?.dhcp_op.toLowerCase() == 'no' && ctx?._temp?.dhcp_op2.toLowerCase() == 'offers'
        {
            event.set("cisco_meraki.event_subtype", json!("dhcp_no_offer"))?;
        }

        // TODO: conditional: ctx?.msgtype.toLowerCase() == "dhcp"
        {
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: events dhcp %{GREEDYDATA:message}$
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re =
                    regex::Regex::new(&grok_to_regex("events dhcp %{GREEDYDATA:message}$"))
                        .unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx.event.original.startsWith('<') && ctx?.cisco_meraki?.event_subtype == "Site-to-Site VPN"
        {
            // Pattern definitions for grok
            // SYSLOGHDR = %{SYSLOGPRI}%{SYSLOGVER}
            // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
            // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
            // SYSLOGVER = \b(?:\d{1,2})\b
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: %{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events%{SPACE}(?i)Site-to-Site VPN:%{GREEDYDATA:cisco_meraki.site_to_site_vpn.raw}
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("%{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events%{SPACE}(?i)Site-to-Site VPN:%{GREEDYDATA:cisco_meraki.site_to_site_vpn.raw}")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx.event.original.startsWith('<') && ctx?.cisco_meraki?.event_subtype == "blocked"
        {
            // Pattern definitions for grok
            // SYSLOGHDR = %{SYSLOGPRI}%{SYSLOGVER}
            // BLOCKEDARP = Blocked ARP Packet
            // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
            // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
            // SYSLOGVER = \b(?:\d{1,2})\b
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: ^%{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events%{SPACE}(?<message>%{BLOCKEDARP:_temp.blocked_arp} from %{MAC:source.mac} with IP %{IP:source.ip} on %{NOTSPACE} %{GREEDYDATA:observer.ingress.vlan.id})$
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("^%{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events%{SPACE}(?<message>%{BLOCKEDARP:_temp.blocked_arp} from %{MAC:source.mac} with IP %{IP:source.ip} on %{NOTSPACE} %{GREEDYDATA:observer.ingress.vlan.id})$")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
        }

        if event.has("source.mac") {
            if let Some(s) = event.get_str("source.mac").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("[:.]").unwrap();
                let replaced = re.replace_all(s, "-").into_owned();
                event.set("source.mac", replaced)?;
            }
        }

        if event.has("source.mac") {
            if let Some(s) = event.get_str("source.mac").map(String::from) {
                let s = s.as_str();
                let uppered = s.to_uppercase();
                event.set("source.mac", uppered)?;
            }
        }

        // TODO: conditional: ctx._temp?.blocked_arp != null
        {
            event.set("cisco_meraki.event_subtype", json!("arp_blocked"))?;
        }

        // TODO: conditional: ctx.event.original.startsWith('<') && ctx.cisco_meraki?.event_subtype == "port"
        {
            // Pattern definitions for grok
            // PORTACTION = (?:changed stp role|status changed)
            // SYSLOGVER = \b(?:\d{1,2})\b
            // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
            // SYSLOGHDR = %{SYSLOGPRI}%{SYSLOGVER}
            // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: ^(?i)%{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events%{SPACE}(?<message>port %{NOTSPACE:cisco_meraki.port} %{PORTACTION:_temp.port_action}(?: from %{NOTSPACE:cisco_meraki.old_port_status} to %{NOTSPACE:cisco_meraki.new_port_status}|.*))$
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("^(?i)%{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events%{SPACE}(?<message>port %{NOTSPACE:cisco_meraki.port} %{PORTACTION:_temp.port_action}(?: from %{NOTSPACE:cisco_meraki.old_port_status} to %{NOTSPACE:cisco_meraki.new_port_status}|.*))$")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
        }

        if event.has("_temp.port_action") {
            if let Some(s) = event.get_str("_temp.port_action").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new(" ").unwrap();
                let replaced = re.replace_all(s, "_").into_owned();
                event.set("_temp.port_action", replaced)?;
            }
        }

        if event.has("_temp.port_action") {
            if let Some(s) = event.get_str("_temp.port_action").map(String::from) {
                let s = s.as_str();
                let lowered = s.to_lowercase();
                event.set("_temp.port_action", lowered)?;
            }
        }

        // TODO: conditional: ctx._temp?.port_action != null
        {
            event.set(
                "cisco_meraki.event_subtype",
                json!(format!(
                    "port_{}",
                    event.get_str("_temp.port_action").unwrap_or("")
                )),
            )?;
        }

        // TODO: conditional: ctx.event.original.startsWith('<') && ctx.cisco_meraki?.event_subtype == "carrier_change"
        {
            // Pattern definitions for grok
            // SYSLOGVER = \b(?:\d{1,2})\b
            // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
            // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
            // SYSLOGHDR = %{SYSLOGPRI}%{SYSLOGVER}
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: ^(?i)%{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events carrier_change device%{SPACE}%{NOTSPACE:cisco_meraki.mxport} up %{NOTSPACE:_temp.up}.*$
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("^(?i)%{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events carrier_change device%{SPACE}%{NOTSPACE:cisco_meraki.mxport} up %{NOTSPACE:_temp.up}.*$")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx._temp?.up == 'true'
        {
            event.set("cisco_meraki.new_port_status", json!("up"))?;
        }

        // TODO: conditional: ctx._temp?.up == 'false'
        {
            event.set("cisco_meraki.new_port_status", json!("down"))?;
        }

        // TODO: conditional: ctx.event.original.startsWith('<') && ['dfs_event', 'association', 'disassociation', 'aps_association_reject', 'multiple_dhcp_servers_detected', 'wpa_deauth', 'wpa_auth', 'vpn_connectivity_change', '8021x_eap_failure', '8021x_auth', '8021x_deauth', '8021x_eap_success', 'splash_auth', 'device_packet_flood'].contains(ctx.cisco_meraki.event_subtype)
        {
            // Pattern definitions for grok
            // SYSLOGHDR = %{SYSLOGPRI}%{SYSLOGVER}
            // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
            // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
            // SYSLOGVER = \b(?:\d{1,2})\b
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: %{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events%{SPACE}%{GREEDYDATA:_temp.rest}
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("%{SYSLOGHDR}%{SPACE}%{NUMBER}%{SPACE}%{WORDORHOST}%{SPACE}events%{SPACE}%{GREEDYDATA:_temp.rest}")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx?._temp?.rest != null && ['dfs_event', 'association', 'disassociation', 'aps_association_reject', 'multiple_dhcp_servers_detected', 'wpa_deauth', 'wpa_auth', '8021x_eap_failure', '8021x_auth', '8021x_deauth', '8021x_eap_success', 'splash_auth', 'device_packet_flood'].contains(ctx.cisco_meraki.event_subtype)
        {
            if let Some(kv_str) = event.get_str("_temp.rest").map(String::from) {
                let kv_str = kv_str.as_str();
                for pair in kv_str.split("[ \t]{1,}") {
                    if let Some((key, value)) = pair.split_once("=") {
                        if !key.is_empty() {
                            let event_subtype = event
                                .get_str("cisco_meraki.event_subtype")
                                .unwrap_or("unknown");
                            event.set(&format!("cisco_meraki.{event_subtype}.{}", key), value)?;
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx?._temp?.rest != null && ctx?.cisco_meraki?.event_subtype == 'vpn_connectivity_change'
        {
            if let Some(kv_str) = event.get_str("_temp.rest").map(String::from) {
                let kv_str = kv_str.as_str();
                for pair in kv_str.split("[ \t]{1,}") {
                    if let Some((key, value)) = pair.split_once("=") {
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

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'multiple_dhcp_servers_detected'
        {
            event.set("network.protocol", json!("dhcp"))?;
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'multiple_dhcp_servers_detected'
        {
            event.rename(
                "cisco_meraki.multiple_dhcp_servers_detected.original_server_mac",
                "server.mac",
            )?;
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'multiple_dhcp_servers_detected'
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event
                    .get_str("cisco_meraki.multiple_dhcp_servers_detected.original_server_ip")
                    .map(String::from)
                {
                    let input = input.as_str();
                    // Grok pattern: ^%{IPV4:cisco_meraki.multiple_dhcp_servers_detected.original_server_ip}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re = regex::Regex::new(&grok_to_regex(
                        "^%{IPV4:cisco_meraki.multiple_dhcp_servers_detected.original_server_ip}$",
                    ))
                    .unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^%{IPV6:cisco_meraki.multiple_dhcp_servers_detected.original_server_ip}$
                }
                Ok(())
            })();
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'multiple_dhcp_servers_detected'
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event
                    .get_str("cisco_meraki.multiple_dhcp_servers_detected.original_server_ip")
                    .map(String::from)
                {
                    let s = s.as_str();
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "cisco_meraki.multiple_dhcp_servers_detected.original_server_ip"
                                .into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("server.ip", s)?;
                }
                Ok(())
            })();
        }

        // TODO: conditional: ctx?.server?.ip != null
        {
            event.remove("cisco_meraki.multiple_dhcp_servers_detected.original_server_ip");
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'multiple_dhcp_servers_detected'
        {
            event.append(
                "related.ip",
                event.get("server.ip").cloned().unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'multiple_dhcp_servers_detected'
        {
            if let Some(input) = event
                .get_str("cisco_meraki.multiple_dhcp_servers_detected.server_ip")
                .map(String::from)
            {
                let input = input.as_str();
                // Grok pattern: ^%{IPV4:cisco_meraki.multiple_dhcp_servers_detected.server_ip}$
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex(
                    "^%{IPV4:cisco_meraki.multiple_dhcp_servers_detected.server_ip}$",
                ))
                .unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
                // Additional grok pattern 1: ^%{IPV6:cisco_meraki.multiple_dhcp_servers_detected.server_ip}$
            }
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'multiple_dhcp_servers_detected'
        {
            if let Some(s) = event
                .get_str("cisco_meraki.multiple_dhcp_servers_detected.server_ip")
                .map(String::from)
            {
                let s = s.as_str();
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

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'multiple_dhcp_servers_detected'
        {
            event.append(
                "related.ip",
                event
                    .get("cisco_meraki.multiple_dhcp_servers_detected.server_ip")
                    .cloned()
                    .unwrap_or(Value::Null),
            )?;
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'wpa_deauth'
        {
            event.rename("cisco_meraki.wpa_deauth.client_mac", "client.mac")?;
        }

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == "client_vpn_connect"
        {
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
                if let Some(pos) = remaining.find(" events client_vpn_connect user id '") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" events client_vpn_connect user id '") {
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

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype == "client_vpn_connect"
        {
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: events client_vpn_connect %{GREEDYDATA:message}$
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex(
                    "events client_vpn_connect %{GREEDYDATA:message}$",
                ))
                .unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx?._temp?.client_ip != null
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_str("_temp.client_ip").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^%{IPV4:_temp.client_ip}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re =
                        regex::Regex::new(&grok_to_regex("^%{IPV4:_temp.client_ip}$")).unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^%{IPV6:_temp.client_ip}$
                }
                Ok(())
            })();
        }

        // TODO: conditional: ctx?._temp?.client_ip != null
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_str("_temp.client_ip").map(String::from) {
                    let s = s.as_str();
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
            if let Some(s) = event.get_str("client.mac").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("[:.]").unwrap();
                let replaced = re.replace_all(s, "-").into_owned();
                event.set("client.mac", replaced)?;
            }
        }

        if event.has("client.mac") {
            if let Some(s) = event.get_str("client.mac").map(String::from) {
                let s = s.as_str();
                let uppered = s.to_uppercase();
                event.set("client.mac", uppered)?;
            }
        }

        if event.has("server.mac") {
            if let Some(s) = event.get_str("server.mac").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("[:.]").unwrap();
                let replaced = re.replace_all(s, "-").into_owned();
                event.set("server.mac", replaced)?;
            }
        }

        if event.has("server.mac") {
            if let Some(s) = event.get_str("server.mac").map(String::from) {
                let s = s.as_str();
                let uppered = s.to_uppercase();
                event.set("server.mac", uppered)?;
            }
        }

        Ok(TransformResult::Continue)
    }
}
