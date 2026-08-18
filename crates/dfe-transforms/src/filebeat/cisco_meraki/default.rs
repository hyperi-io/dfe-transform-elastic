// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut Event) -> Result<TransformResult> {
        event.set("ecs.version", json!("8.11.0"))?;

        // TODO: conditional: ctx.event?.original == null
        {
            if event.has("message") {
                event.rename("message", "event.original")?;
            }
        }

        if let Some(input) = event.get_str("event.original").map(String::from) {
            let input = input.as_str();
            let mut remaining = input;
            if let Some(pos) = remaining.find(" ") {
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" ") {
                remaining = rest;
            }
            if let Some(pos) = remaining.find(" ") {
                event.set("_temp.ts_nano", &remaining[..pos])?;
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" ") {
                remaining = rest;
            }
            if let Some(pos) = remaining.find(" ") {
                event.set("observer.hostname", &remaining[..pos])?;
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" ") {
                remaining = rest;
            }
            if let Some(pos) = remaining.find(" ") {
                event.set("cisco_meraki.event_type", &remaining[..pos])?;
                remaining = &remaining[pos..];
            }
            if let Some(rest) = remaining.strip_prefix(" ") {
                remaining = rest;
            }
        }

        // TODO: conditional: ctx._conf?.tz_offset != null && ctx._conf?.tz_offset != "local"
        {
            if let Some(date_str) = event.get_str("_temp.ts_nano").map(String::from) {
                let date_str = date_str.as_str();
                // Try UNIX timestamp
                if let Ok(ts) = date_str.parse::<f64>() {
                    let secs = ts as i64;
                    let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                    if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
                        event.set("@timestamp", dt.to_rfc3339())?;
                    }
                }
            }
        }

        // TODO: conditional: ctx._conf?.tz_offset == null || ctx._conf?.tz_offset == "local"
        {
            if let Some(date_str) = event.get_str("_temp.ts_nano").map(String::from) {
                let date_str = date_str.as_str();
                // Try UNIX timestamp
                if let Ok(ts) = date_str.parse::<f64>() {
                    let secs = ts as i64;
                    let nsecs = ((ts - secs as f64) * 1_000_000_000.0) as u32;
                    if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
                        event.set("@timestamp", dt.to_rfc3339())?;
                    }
                }
            }
        }

        // TODO: conditional: ['flows', 'firewall', 'vpn_firewall', 'cellular_firewall', 'bridge_anyconnect_client_vpn_firewall'].contains(ctx.cisco_meraki.event_type)
        {
            // Begin nested pipeline: "flows"
            // Pattern definitions for grok
            // TYPE = flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: %{TYPE}( %{NOTSPACE:cisco_meraki.flows.op})? src=%{IP:source.ip:ip} dst=%{IP:destination.ip:ip}( mac=%{MAC:source.mac})? protocol=%{NOTSPACE:network.protocol}( type=%{NOTSPACE})?( sport=%{NONNEGINT:source.port:long})?( dport=%{NONNEGINT:destination.port:long})?( pattern: %{GREEDYDATA:cisco_meraki.firewall.pattern})?
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("%{TYPE}( %{NOTSPACE:cisco_meraki.flows.op})? src=%{IP:source.ip:ip} dst=%{IP:destination.ip:ip}( mac=%{MAC:source.mac})? protocol=%{NOTSPACE:network.protocol}( type=%{NOTSPACE})?( sport=%{NONNEGINT:source.port:long})?( dport=%{NONNEGINT:destination.port:long})?( pattern: %{GREEDYDATA:cisco_meraki.firewall.pattern})?")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
            // TODO: conditional: ctx.cisco_meraki?.firewall?.pattern != null && (ctx.cisco_meraki.firewall.pattern.startsWith('allow') || ctx.cisco_meraki.firewall.pattern.startsWith('deny'))
            {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event
                        .get_str("cisco_meraki.firewall.pattern")
                        .map(String::from)
                    {
                        let input = input.as_str();
                        // Grok pattern: %{NOTSPACE:cisco_meraki.firewall.action} %{GREEDYDATA:cisco_meraki.firewall.rule}
                        // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                        let grok_re = regex::Regex::new(&grok_to_regex("%{NOTSPACE:cisco_meraki.firewall.action} %{GREEDYDATA:cisco_meraki.firewall.rule}")).unwrap();
                        if let Some(caps) = grok_re.captures(input) {
                            for name in grok_re.capture_names().flatten() {
                                if let Some(m) = caps.name(name) {
                                    event.set(name, m.as_str())?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }
            // TODO: conditional: ctx.cisco_meraki?.firewall?.rule != null
            {
                event.remove("cisco_meraki.firewall.pattern");
            }
            if event.has("source.mac") {
                if let Some(s) = event.get_str("source.mac").map(String::from) {
                    let s = s.as_str();
                    let re = regex::Regex::new("[:.]").unwrap();
                    let replaced = re.replace_all(s, "-").into_owned();
                    event.set("source.mac", replaced)?;
                }
            }
            // TODO: conditional: ctx.cisco_meraki?.flows?.op == null
            {
                event.set("cisco_meraki.event_subtype", json!("ip_session_initiated"))?;
            }
            // TODO: conditional: ctx.cisco_meraki?.flows?.op == 'allow'
            {
                event.set("cisco_meraki.event_subtype", json!("flow_allowed"))?;
            }
            // TODO: conditional: ctx.cisco_meraki?.flows?.op == 'deny'
            {
                event.set("cisco_meraki.event_subtype", json!("flow_denied"))?;
            }
            // End nested pipeline: "flows"
        }

        // TODO: conditional: ctx.cisco_meraki.event_type == 'ip_flow_start' || ctx.cisco_meraki.event_type == 'ip_flow_end'
        {
            // Begin nested pipeline: "ipflows"
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
                if let Some(pos) = remaining.find(" ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("_temp.event_type", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                event.set("_temp.event", remaining)?;
            }
            // TODO: conditional: ctx._temp?.event != null
            {
                if let Some(kv_str) = event.get_str("_temp.event").map(String::from) {
                    let kv_str = kv_str.as_str();
                    for pair in kv_str.split(" ") {
                        if let Some((key, value)) = pair.split_once("=") {
                            if !key.is_empty() {
                                event.set(key, value)?;
                            }
                        }
                    }
                }
            }
            // TODO: conditional: ctx?.translated_src_ip != null
            {
                if let Some(s) = event.get_str("translated_src_ip").map(String::from) {
                    let s = s.as_str();
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "translated_src_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("source.ip", s)?;
                }
            }
            // TODO: conditional: ctx?.translated_src_ip == null && ctx?.src != null
            {
                if let Some(s) = event.get_str("src").map(String::from) {
                    let s = s.as_str();
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "src".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("source.ip", s)?;
                }
            }
            // TODO: conditional: ctx?.translated_src_ip != null && ctx?.translated_port != null
            {
                if let Some(val) = event.get("translated_port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "translated_port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "translated_port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "translated_port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.port", converted)?;
                }
            }
            // TODO: conditional: ctx?.translated_src_ip == null && ctx?.sport != null
            {
                if let Some(val) = event.get("sport") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "sport".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "sport".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "sport".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.port", converted)?;
                }
            }
            // TODO: conditional: ctx?.translated_dst_ip != null
            {
                if let Some(s) = event.get_str("translated_dst_ip").map(String::from) {
                    let s = s.as_str();
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "translated_dst_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("destination.ip", s)?;
                }
            }
            // TODO: conditional: ctx?.translated_dst_ip == null && ctx?.dst != null
            {
                if let Some(s) = event.get_str("dst").map(String::from) {
                    let s = s.as_str();
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "dst".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("destination.ip", s)?;
                }
            }
            // TODO: conditional: ctx?.translated_dst_ip != null && ctx?.translated_port != null
            {
                if let Some(val) = event.get("translated_port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "translated_port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "translated_port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "translated_port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("destination.port", converted)?;
                }
            }
            // TODO: conditional: ctx?.translated_dst_ip == null && ctx?.dport != null
            {
                if let Some(val) = event.get("dport") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "dport".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "dport".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "dport".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("destination.port", converted)?;
                }
            }
            event.rename("protocol", "network.protocol")?;
            // End nested pipeline: "ipflows"
        }

        // TODO: conditional: ctx.cisco_meraki.event_type == 'airmarshal_events'
        {
            // Begin nested pipeline: "airmarshal"
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
                if let Some(pos) = remaining.find(" airmarshal_events ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" airmarshal_events ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("type", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
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
            event.rename("type", "cisco_meraki.event_subtype")?;
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: %{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex(
                    "%{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}",
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
            if let Some(input) = event.get_str("_temp.ssid").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
                if let Some(rest) = remaining.strip_prefix("'") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("'") {
                    event.set("_temp.kv.ssid", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("'") {
                    remaining = rest;
                }
            }
            if let Some(kv_str) = event.get_str("_temp.kvline").map(String::from) {
                let kv_str = kv_str.as_str();
                for pair in kv_str.split(" ") {
                    if let Some((key, value)) = pair.split_once("=") {
                        if !key.is_empty() {
                            event.set(&format!("_temp.kv.{}", key), value)?;
                        }
                    }
                }
            }
            // TODO: conditional: ctx?._temp?.kv?.ssid != null
            {
                event.rename("_temp.kv.ssid", "network.name")?;
            }
            event.rename("_temp.kv.bssid", "cisco_meraki.bssid")?;
            // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'ssid_spoofing_detected'
            {
                event.rename("_temp.kv.vap", "cisco_meraki.vap")?;
            }
            if let Some(s) = event.get_str("_temp.kv.src").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("[-:.]").unwrap();
                let replaced = re.replace_all(s, "-").into_owned();
                event.set("source.mac", replaced)?;
            }
            if let Some(s) = event.get_str("_temp.kv.dst").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("[-:.]").unwrap();
                let replaced = re.replace_all(s, "-").into_owned();
                event.set("destination.mac", replaced)?;
            }
            // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'rogue_ssid_detected'
            {
                if let Some(s) = event.get_str("_temp.kv.wired_mac").map(String::from) {
                    let s = s.as_str();
                    let re = regex::Regex::new("[-:.]").unwrap();
                    let replaced = re.replace_all(s, "-").into_owned();
                    event.set("_temp.observer.mac", replaced)?;
                }
            }
            // TODO: conditional: ctx?._temp?.observer?.mac != null
            {
                event.append(
                    "observer.mac",
                    event
                        .get("_temp.observer.mac")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
            }
            // TODO: conditional: ctx?.cisco_meraki?.event_subtype == 'rogue_ssid_detected'
            {
                event.rename("_temp.kv.vlan_id", "network.vlan.id")?;
            }
            event.rename("_temp.kv.channel", "cisco_meraki.channel")?;
            event.rename("_temp.kv.fc_type", "cisco_meraki.fc_type")?;
            event.rename("_temp.kv.fc_subtype", "cisco_meraki.fc_subtype")?;
            // End nested pipeline: "airmarshal"
        }

        // TODO: conditional: ctx.cisco_meraki.event_type == 'security_event'
        {
            // Begin nested pipeline: "security"
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
                if let Some(pos) = remaining.find(" security_event ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" security_event ") {
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
            event.rename("type", "cisco_meraki.event_subtype")?;
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                // Grok pattern: ^%{DATA} (security_event|ids-alerts) (%{WORD}\\s)?%{DATA:_temp.kvs}(\\smessage:\\s?%{DATA:message})?$
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re = regex::Regex::new(&grok_to_regex("^%{DATA} (security_event|ids-alerts) (%{WORD}\\s)?%{DATA:_temp.kvs}(\\smessage:\\s?%{DATA:message})?$")).unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
            }
            if let Some(kv_str) = event.get_str("_temp.kvs").map(String::from) {
                let kv_str = kv_str.as_str();
                for pair in kv_str.split(" ") {
                    if let Some((key, value)) = pair.split_once("=") {
                        if !key.is_empty() {
                            event.set(key, value)?;
                        }
                    }
                }
            }
            if event.has("priority") {
                event.rename("priority", "cisco_meraki.security.priority")?;
            }
            if event.has("signature") {
                event.rename("signature", "cisco_meraki.security.signature")?;
            }
            if event.has("dhost") {
                if let Some(s) = event.get_str("dhost").map(String::from) {
                    let s = s.as_str();
                    let re = regex::Regex::new("[-:.]").unwrap();
                    let replaced = re.replace_all(s, "-").into_owned();
                    event.set("cisco_meraki.security.dhost", replaced)?;
                }
            }
            if event.has("direction") {
                event.rename("direction", "network.direction")?;
            }
            if event.has("protocol") {
                if let Some(s) = event.get_str("protocol").map(String::from) {
                    let s = s.as_str();
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
            }
            if event.has("decision") {
                event.rename("decision", "cisco_meraki.security.decision")?;
            }
            // TODO: conditional: ctx.url != null
            {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(uri_str) = event.get_str("url").map(String::from) {
                        let uri_str = uri_str.as_str();
                        if let Ok(url) = url::Url::parse(uri_str) {
                            event.set("url.scheme", url.scheme())?;
                            if let Some(host) = url.host_str() {
                                event.set("url.domain", host)?;
                            }
                            if let Some(port) = url.port() {
                                event.set("url.port", json!(port))?;
                            }
                            event.set("url.path", url.path())?;
                            if let Some(query) = url.query() {
                                event.set("url.query", query)?;
                            }
                            if let Some(fragment) = url.fragment() {
                                event.set("url.fragment", fragment)?;
                            }
                            if let Some(userinfo) = url.password() {
                                event.set(
                                    "url.user_info",
                                    format!("{}:{}", url.username(), userinfo),
                                )?;
                            } else if !url.username().is_empty() {
                                event.set("url.user_info", url.username())?;
                            }
                        }
                    }
                    Ok(())
                })();
            }
            if event.has("mac") {
                if let Some(s) = event.get_str("mac").map(String::from) {
                    let s = s.as_str();
                    let re = regex::Regex::new("[-:.]").unwrap();
                    let replaced = re.replace_all(s, "-").into_owned();
                    event.set("cisco_meraki.security.mac", replaced)?;
                }
            }
            if event.has("name") {
                event.rename("name", "file.name")?;
            }
            if event.has("sha256") {
                event.rename("sha256", "file.hash.sha256")?;
            }
            if event.has("disposition") {
                event.rename("disposition", "cisco_meraki.disposition")?;
            }
            if event.has("action") {
                event.rename("action", "cisco_meraki.security.action")?;
            }
            // TODO: conditional: ctx?.cisco_meraki?.event_subtype != 'security_filtering_disposition_change' && ctx?.src != null
            {
                // Pattern definitions for grok
                // IPV6PORTSEP = (?: port |[p#.])
                // PORT = [0-9]+
                // IPV6NOCOMPRESS = ([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}
                if let Some(input) = event.get_str("src").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^%{IPV4:_temp.src_ip}:%{PORT:sport}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re =
                        regex::Regex::new(&grok_to_regex("^%{IPV4:_temp.src_ip}:%{PORT:sport}$"))
                            .unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^\\[%{IPV6:_temp.src_ip}\\]:%{PORT:sport}$
                    // Additional grok pattern 2: ^%{IPV6NOCOMPRESS:_temp.src_ip}:%{PORT:sport}$
                    // Additional grok pattern 3: ^%{IPV6:_temp.src_ip}%{IPV6PORTSEP}%{PORT:sport}$
                }
            }
            if event.has("_temp.src_ip") {
                if let Some(s) = event.get_str("_temp.src_ip").map(String::from) {
                    let s = s.as_str();
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "_temp.src_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("source.ip", s)?;
                }
            }
            // TODO: conditional: ctx?.sport != "0"
            {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("sport") {
                        if let Some(val) = event.get("sport") {
                            let converted = match val {
                                Value::String(s) => {
                                    let s = s.trim();
                                    if let Some(hex) = s.strip_prefix("0x") {
                                        json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                            TransformError::ParseError {
                                                path: "sport".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    } else {
                                        json!(s.parse::<i64>().map_err(|_| {
                                            TransformError::ParseError {
                                                path: "sport".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    }
                                }
                                Value::Number(n) => {
                                    json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                                }
                                Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                                _ => {
                                    return Err(TransformError::ParseError {
                                        path: "sport".into(),
                                        message: "cannot convert to integer".into(),
                                    });
                                }
                            };
                            event.set("source.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }
            // TODO: conditional: ctx?.cisco_meraki?.event_subtype != 'security_filtering_disposition_change' && ctx?.dst != null
            {
                // Pattern definitions for grok
                // IPV6NOCOMPRESS = ([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}
                // PORT = [0-9]+
                // IPV6PORTSEP = (?: port |[p#.])
                if let Some(input) = event.get_str("dst").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^%{IPV4:_temp.dst_ip}:%{PORT:dport}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re =
                        regex::Regex::new(&grok_to_regex("^%{IPV4:_temp.dst_ip}:%{PORT:dport}$"))
                            .unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^\\[%{IPV6:_temp.dst_ip}\\]:%{PORT:dport}$
                    // Additional grok pattern 2: ^%{IPV6NOCOMPRESS:_temp.dst_ip}:%{PORT:dport}$
                    // Additional grok pattern 3: ^%{IPV6:_temp.dst_ip}%{IPV6PORTSEP}%{PORT:dport}$
                }
            }
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("_temp.dst_ip") {
                    if let Some(s) = event.get_str("_temp.dst_ip").map(String::from) {
                        let s = s.as_str();
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "_temp.dst_ip".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("destination.ip", s)?;
                    }
                }
                Ok(())
            })();
            // TODO: conditional: ctx?.dport != "0" && ctx?.cisco_meraki?.event_subtype != 'security_filtering_disposition_change'
            {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("dport") {
                        if let Some(val) = event.get("dport") {
                            let converted = match val {
                                Value::String(s) => {
                                    let s = s.trim();
                                    if let Some(hex) = s.strip_prefix("0x") {
                                        json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                            TransformError::ParseError {
                                                path: "dport".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    } else {
                                        json!(s.parse::<i64>().map_err(|_| {
                                            TransformError::ParseError {
                                                path: "dport".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    }
                                }
                                Value::Number(n) => {
                                    json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                                }
                                Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                                _ => {
                                    return Err(TransformError::ParseError {
                                        path: "dport".into(),
                                        message: "cannot convert to integer".into(),
                                    });
                                }
                            };
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }
            // End nested pipeline: "security"
        }

        // TODO: conditional: ctx.cisco_meraki.event_type == 'ids-alerts'
        {
            // Begin nested pipeline: "idsalerts"
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
                if let Some(pos) = remaining.find(" ids-alerts ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ids-alerts ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("sig", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("sig", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("pri", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("pri", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("ts", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("ts", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("dir", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("dir", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("prot", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("prot", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("src", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                event.set("src", remaining)?;
            }
            event.set("cisco_meraki.event_subtype", json!("ids_alerted"))?;
            event.rename("priority", "cisco_meraki.security.priority")?;
            event.rename("signature", "cisco_meraki.security.signature")?;
            event.rename("direction", "network.direction")?;
            if let Some(s) = event.get_str("protocol").map(String::from) {
                let s = s.as_str();
                let lowered = s.to_lowercase();
                event.set("network.protocol", lowered)?;
            }
            // TODO: conditional: ctx?.src != null
            {
                // Pattern definitions for grok
                // PORT = [0-9]+
                // IPV6PORTSEP = (?: port |[p#.])
                // IPV6NOCOMPRESS = ([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}
                if let Some(input) = event.get_str("src").map(String::from) {
                    let input = input.as_str();
                    // Grok pattern: ^%{IPV4:_temp.src_ip}:%{PORT:sport}$
                    // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                    let grok_re =
                        regex::Regex::new(&grok_to_regex("^%{IPV4:_temp.src_ip}:%{PORT:sport}$"))
                            .unwrap();
                    if let Some(caps) = grok_re.captures(input) {
                        for name in grok_re.capture_names().flatten() {
                            if let Some(m) = caps.name(name) {
                                event.set(name, m.as_str())?;
                            }
                        }
                    }
                    // Additional grok pattern 1: ^\\[%{IPV6:_temp.src_ip}\\]:%{PORT:sport}$
                    // Additional grok pattern 2: ^%{IPV6NOCOMPRESS:_temp.src_ip}:%{PORT:sport}$
                    // Additional grok pattern 3: ^%{IPV6:_temp.src_ip}%{IPV6PORTSEP}%{PORT:sport}$
                }
            }
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_str("_temp.src_ip").map(String::from) {
                    let s = s.as_str();
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "_temp.src_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("source.ip", s)?;
                }
                Ok(())
            })();
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("sport") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "sport".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "sport".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "sport".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.port", converted)?;
                }
                Ok(())
            })();
            // End nested pipeline: "idsalerts"
        }

        // TODO: conditional: ctx.cisco_meraki.event_type == 'events'
        {
            // Begin nested pipeline: "events"
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
                // SYSLOGVER = \b(?:\d{1,2})\b
                // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
                // SYSLOGHDR = %{SYSLOGPRI}%{SYSLOGVER}
                // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
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
                // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
                // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
                // BLOCKEDARP = Blocked ARP Packet
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
                // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
                // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
                // PORTACTION = (?:changed stp role|status changed)
                // SYSLOGHDR = %{SYSLOGPRI}%{SYSLOGVER}
                // SYSLOGVER = \b(?:\d{1,2})\b
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
                // SYSLOGHDR = %{SYSLOGPRI}%{SYSLOGVER}
                // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
                // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
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
                // SYSLOGVER = \b(?:\d{1,2})\b
                // SYSLOGPRI = <%{NONNEGINT:log.syslog.priority:long}>
                // WORDORHOST = (?:%{WORD}|%{HOSTNAME})
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
                                event
                                    .set(&format!("cisco_meraki.{event_subtype}.{}", key), value)?;
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
                        let grok_re = regex::Regex::new(&grok_to_regex("^%{IPV4:cisco_meraki.multiple_dhcp_servers_detected.original_server_ip}$")).unwrap();
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
            // End nested pipeline: "events"
        }

        // TODO: conditional: ctx.cisco_meraki.event_type == 'urls'
        {
            // Begin nested pipeline: "urls"
            if let Some(input) = event.get_str("event.original").map(String::from) {
                let input = input.as_str();
                let mut remaining = input;
                if let Some(pos) = remaining.find(" urls ") {
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" urls ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("src", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("src", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("dst", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("dst", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find("=") {
                    event.set("mac", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix("=") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" request: ") {
                    event.set("mac", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" request: ") {
                    remaining = rest;
                }
                if let Some(pos) = remaining.find(" ") {
                    event.set("http.request.method", &remaining[..pos])?;
                    remaining = &remaining[pos..];
                }
                if let Some(rest) = remaining.strip_prefix(" ") {
                    remaining = rest;
                }
                event.set("url.original", remaining)?;
            }
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("mac") {
                    if let Some(input) = event.get_str("mac").map(String::from) {
                        let input = input.as_str();
                        let mut remaining = input;
                        if let Some(pos) = remaining.find(" agent=") {
                            event.set("mac", &remaining[..pos])?;
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" agent=") {
                            remaining = rest;
                        }
                        event.set("user_agent.original", remaining)?;
                    }
                }
                Ok(())
            })();
            // Pattern definitions for grok
            // IPV6PORTSEP = (?: port |[p#.])
            // IPV6NOCOMPRESS = ([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}
            // PORT = [0-9]+
            if let Some(input) = event.get_str("src").map(String::from) {
                let input = input.as_str();
                // Grok pattern: ^%{IPV4:_temp.src_ip}:%{PORT:sport}$
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re =
                    regex::Regex::new(&grok_to_regex("^%{IPV4:_temp.src_ip}:%{PORT:sport}$"))
                        .unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
                // Additional grok pattern 1: ^\\[%{IPV6:_temp.src_ip}\\]:%{PORT:sport}$
                // Additional grok pattern 2: ^%{IPV6NOCOMPRESS:_temp.src_ip}:%{PORT:sport}$
                // Additional grok pattern 3: ^%{IPV6:_temp.src_ip}%{IPV6PORTSEP}%{PORT:sport}$
            }
            if let Some(s) = event.get_str("_temp.src_ip").map(String::from) {
                let s = s.as_str();
                // Validate IP format
                let s = s.trim();
                if s.parse::<std::net::IpAddr>().is_err() {
                    return Err(TransformError::ParseError {
                        path: "_temp.src_ip".into(),
                        message: format!("cannot convert '{}' to IP", s),
                    });
                }
                event.set("source.ip", s)?;
            }
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("sport") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "sport".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "sport".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "sport".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.port", converted)?;
                }
                Ok(())
            })();
            // Pattern definitions for grok
            // IPV6NOCOMPRESS = ([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4}
            // PORT = [0-9]+
            // IPV6PORTSEP = (?: port |[p#.])
            if let Some(input) = event.get_str("dst").map(String::from) {
                let input = input.as_str();
                // Grok pattern: ^%{IPV4:_temp.dst_ip}:%{PORT:dport}$
                // TODO: Replace with dfe-parse Layer 1/2/3 calls after grok analyser (2.1.2)
                let grok_re =
                    regex::Regex::new(&grok_to_regex("^%{IPV4:_temp.dst_ip}:%{PORT:dport}$"))
                        .unwrap();
                if let Some(caps) = grok_re.captures(input) {
                    for name in grok_re.capture_names().flatten() {
                        if let Some(m) = caps.name(name) {
                            event.set(name, m.as_str())?;
                        }
                    }
                }
                // Additional grok pattern 1: ^\\[%{IPV6:_temp.dst_ip}\\]:%{PORT:dport}$
                // Additional grok pattern 2: ^%{IPV6NOCOMPRESS:_temp.dst_ip}:%{PORT:dport}$
                // Additional grok pattern 3: ^%{IPV6:_temp.dst_ip}%{IPV6PORTSEP}%{PORT:dport}$
            }
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_str("_temp.dst_ip").map(String::from) {
                    let s = s.as_str();
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "_temp.dst_ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("destination.ip", s)?;
                }
                Ok(())
            })();
            // TODO: conditional: ctx?.dport != "0" && ctx?.cisco_meraki?.event_subtype != 'security_filtering_disposition_change'
            {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("dport") {
                        let converted = match val {
                            Value::String(s) => {
                                let s = s.trim();
                                if let Some(hex) = s.strip_prefix("0x") {
                                    json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                        TransformError::ParseError {
                                            path: "dport".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                } else {
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "dport".into(),
                                            message: format!("cannot convert '{}' to integer", s),
                                        }
                                    })?)
                                }
                            }
                            Value::Number(n) => {
                                json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                            }
                            Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                            _ => {
                                return Err(TransformError::ParseError {
                                    path: "dport".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("destination.port", converted)?;
                    }
                    Ok(())
                })();
            }
            if let Some(s) = event.get_str("mac").map(String::from) {
                let s = s.as_str();
                let re = regex::Regex::new("[-:.]").unwrap();
                let replaced = re.replace_all(s, "-").into_owned();
                event.set("cisco_meraki.urls.mac", replaced)?;
            }
            // TODO: conditional: ctx?.http?.request?.method.toLowerCase() != 'unknown'
            {
                event.set("cisco_meraki.event_subtype", json!("http_access"))?;
            }
            // TODO: conditional: ctx?.http?.request?.method.toLowerCase() == 'unknown'
            {
                event.set("cisco_meraki.event_subtype", json!("http_access_error"))?;
            }
            if event.has("user_agent.original") {
                if let Some(ua_str) = event.get_str("user_agent.original").map(String::from) {
                    let ua_str = ua_str.as_str();
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }
            // TODO: conditional: ctx.url?.original != null && ctx.url.original != ""
            {
                if let Some(uri_str) = event.get_str("url.original").map(String::from) {
                    let uri_str = uri_str.as_str();
                    if let Ok(url) = url::Url::parse(uri_str) {
                        event.set("url.scheme", url.scheme())?;
                        if let Some(host) = url.host_str() {
                            event.set("url.domain", host)?;
                        }
                        if let Some(port) = url.port() {
                            event.set("url.port", json!(port))?;
                        }
                        event.set("url.path", url.path())?;
                        if let Some(query) = url.query() {
                            event.set("url.query", query)?;
                        }
                        if let Some(fragment) = url.fragment() {
                            event.set("url.fragment", fragment)?;
                        }
                        if let Some(userinfo) = url.password() {
                            event
                                .set("url.user_info", format!("{}:{}", url.username(), userinfo))?;
                        } else if !url.username().is_empty() {
                            event.set("url.user_info", url.username())?;
                        }
                    }
                }
            }
            if event.has("url.domain") {
                if let Some(domain_str) = event.get_str("url.domain").map(String::from) {
                    let domain_str = domain_str.as_str();
                    let domain = domain_str.to_string();
                    event.set("url.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        event.set("url.registered_domain", json!(rd.registered_domain))?;
                        event.set("url.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("url.subdomain", json!(sub))?;
                        }
                    }
                }
            }
            // End nested pipeline: "urls"
        }

        event.append("event.category", json!("network"))?;

        event.append("event.type", json!("info"))?;

        // TODO: conditional: ctx?.cisco_meraki?.event_subtype != null
        {
            // Painless script
            // Source: def eventMap = params.get('eventmap');\ndef eventData = eventMap.get(ctx.cisco_meraki.event_subtype);\nif (eventData == null) {\n  ctx.event.action = ctx.cisco_meraki.event_subtype;\n  return;\n}\ndef eventCategory = eventData.get('category');\ndef eventType = eventData.get('type');\ndef eventAction = eventData.get('action');\nif (eventType != null) {\n  for (def t : eventType) {\n    ctx.event.type.add(t);\n  }\n}\nif (eventCategory != null) {\n  for (def c : eventCategory) {\n    ctx.event.category.add(c);\n  }\n}\nif (eventAction != null) {\n  ctx.event.action = eventAction;\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                r#"def eventMap = params.get('eventmap');\ndef eventData = eventMap.get(ctx.cisco_meraki.event_subtype);\nif (eventData == null) {\n  ctx.event.action = ctx.cisco_meraki.event_subtype;\n  return;\n}\ndef eventCategory = eventData.get('category');\ndef eventType = eventData.get('type');\ndef eventAction = eventData.get('action');\nif (eventType != null) {\n  for (def t : eventType) {\n    ctx.event.type.add(t);\n  }\n}\nif (eventCategory != null) {\n  for (def c : eventCategory) {\n    ctx.event.category.add(c);\n  }\n}\nif (eventAction != null) {\n  ctx.event.action = eventAction;\n}"#,
            )?;
        }

        // TODO: conditional: ctx.source?.geo == null && ctx?.source?.ip != null
        {
            if event.has("source.ip") {
                if let Some(ip_str) = event.get_str("source.ip").map(String::from) {
                    let ip_str = ip_str.as_str();
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
        }

        // TODO: conditional: ctx?.source?.ip != null
        {
            if event.has("source.ip") {
                if let Some(ip_str) = event.get_str("source.ip").map(String::from) {
                    let ip_str = ip_str.as_str();
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
        }

        if event.has("source.as.asn") {
            event.rename("source.as.asn", "source.as.number")?;
        }

        if event.has("source.as.organization_name") {
            event.rename("source.as.organization_name", "source.as.organization.name")?;
        }

        // TODO: conditional: ctx.destination?.geo == null && ctx?.destination?.ip != null
        {
            if event.has("destination.ip") {
                if let Some(ip_str) = event.get_str("destination.ip").map(String::from) {
                    let ip_str = ip_str.as_str();
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
        }

        // TODO: conditional: ctx?.destination?.ip != null
        {
            if event.has("destination.ip") {
                if let Some(ip_str) = event.get_str("destination.ip").map(String::from) {
                    let ip_str = ip_str.as_str();
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
        }

        if event.has("destination.as.asn") {
            event.rename("destination.as.asn", "destination.as.number")?;
        }

        if event.has("destination.as.organization_name") {
            event.rename(
                "destination.as.organization_name",
                "destination.as.organization.name",
            )?;
        }

        // TODO: conditional: ctx.client?.geo == null && ctx?.client?.ip != null
        {
            if event.has("client.ip") {
                if let Some(ip_str) = event.get_str("client.ip").map(String::from) {
                    let ip_str = ip_str.as_str();
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("client.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("client.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("client.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("client.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("client.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("client.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("client.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("client.geo.location", v.clone())?;
                        }
                    }
                }
            }
        }

        // TODO: conditional: ctx?.client?.ip != null
        {
            if event.has("client.ip") {
                if let Some(ip_str) = event.get_str("client.ip").map(String::from) {
                    let ip_str = ip_str.as_str();
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("client.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("client.as.organization_name", v.clone())?;
                        }
                    }
                }
            }
        }

        if event.has("client.as.asn") {
            event.rename("client.as.asn", "client.as.number")?;
        }

        if event.has("client.as.organization_name") {
            event.rename("client.as.organization_name", "client.as.organization.name")?;
        }

        event.remove("_temp");
        event.remove("_conf");
        event.remove("sport");
        event.remove("dport");
        event.remove("mac");
        event.remove("src");
        event.remove("dst");
        event.remove("translated_src_ip");
        event.remove("translated_dst_ip");
        event.remove("translated_port");
        event.remove("wired_mac");
        event.remove("rssi");
        event.remove("protocol");
        event.remove("dhost");
        event.remove("client_mac");
        event.remove("radio");
        event.remove("sts");
        event.remove("msgtype");
        event.remove("timestamp");

        // Painless script
        // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
        // TODO: Transpile Painless to Rust (2.2.3)
        painless_exec(
            event,
            r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#,
        )?;

        // TODO: conditional: ctx?.tags == null || !(ctx.tags.contains('preserve_original_event'))
        {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("event.original");
                Ok(())
            })();
        }

        Ok(TransformResult::Continue)
    }
}
