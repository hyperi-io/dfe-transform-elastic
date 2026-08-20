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
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                let mut remaining: &str = &input;
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

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp.ts_nano") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["UNIX"],
                            event.get_str("_conf.tz_offset"),
                            None,
                        ) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed to parse time field ({}): {}",
                            event
                                .get("_temp.ts_nano")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                !event.has_value("_conf.tz_offset")
                    || event.get_str("_conf.tz_offset") == Some("local")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_temp.ts_nano") {
                        if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], None, None) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed to parse time field ({}): {}",
                            event
                                .get("_temp.ts_nano")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                [
                    "flows",
                    "firewall",
                    "vpn_firewall",
                    "cellular_firewall",
                    "bridge_anyconnect_client_vpn_firewall",
                ]
                .contains(&event.get_str("cisco_meraki.event_type").unwrap_or(""))
            };
            if _cond {
                // Begin nested pipeline: "flows"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall) %{GREEDYDATA:message}
                        if !cached_grok!("(?:flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall) %{GREEDYDATA:message}").extract_into(&input, event)? {
                }
                    }
                    Ok(())
                })();
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (?:flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall)( %{NOTSPACE:cisco_meraki.flows.op})? src=%{IP:source.ip:ip} dst=%{IP:destination.ip:ip}( mac=%{MAC:source.mac})? protocol=%{NOTSPACE:network.protocol}( type=%{NOTSPACE})?( sport=%{NONNEGINT:source.port:long})?( dport=%{NONNEGINT:destination.port:long})?( pattern: %{GREEDYDATA:cisco_meraki.firewall.pattern})?
                    if !cached_grok!("(?:flows|firewall|vpn_firewall|cellular_firewall|bridge_anyconnect_client_vpn_firewall)( %{NOTSPACE:cisco_meraki.flows.op})? src=%{IP:source.ip:ip} dst=%{IP:destination.ip:ip}( mac=%{MAC:source.mac})? protocol=%{NOTSPACE:network.protocol}( type=%{NOTSPACE})?( sport=%{NONNEGINT:source.port:long})?( dport=%{NONNEGINT:destination.port:long})?( pattern: %{GREEDYDATA:cisco_meraki.firewall.pattern})?").extract_into(&input, event)? {
                }
                }
                let _cond = {
                    event.has_value("cisco_meraki.firewall.pattern")
                        && (event
                            .get_str("cisco_meraki.firewall.pattern")
                            .is_some_and(|s| s.starts_with("allow"))
                            || event
                                .get_str("cisco_meraki.firewall.pattern")
                                .is_some_and(|s| s.starts_with("deny")))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_meraki.firewall.pattern") {
                            // Grok pattern: %{NOTSPACE:cisco_meraki.firewall.action} %{GREEDYDATA:cisco_meraki.firewall.rule}
                            if !cached_grok!("%{NOTSPACE:cisco_meraki.firewall.action} %{GREEDYDATA:cisco_meraki.firewall.rule}").extract_into(&input, event)? {
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_meraki.firewall.rule") };
                if _cond {
                    if event.remove("cisco_meraki.firewall.pattern").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "cisco_meraki.firewall.pattern".into(),
                        });
                    }
                }
                if event.has("source.mac") {
                    if let Some(s) = event.get_string("source.mac") {
                        let re = cached_regex!("[:.]");
                        let replaced = re.replace_all(&s, "-").into_owned();
                        event.set("source.mac", replaced)?;
                    }
                }
                let _cond = { !event.has_value("cisco_meraki.flows.op") };
                if _cond {
                    event.set("cisco_meraki.event_subtype", json!("ip_session_initiated"))?;
                }
                let _cond = { event.get_str("cisco_meraki.flows.op") == Some("allow") };
                if _cond {
                    event.set("cisco_meraki.event_subtype", json!("flow_allowed"))?;
                }
                let _cond = { event.get_str("cisco_meraki.flows.op") == Some("deny") };
                if _cond {
                    event.set("cisco_meraki.event_subtype", json!("flow_denied"))?;
                }
                // End nested pipeline: "flows"
            }

            let _cond = {
                event.get_str("cisco_meraki.event_type") == Some("ip_flow_start")
                    || event.get_str("cisco_meraki.event_type") == Some("ip_flow_end")
            };
            if _cond {
                // Begin nested pipeline: "ipflows"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:ip_flow_start|ip_flow_end) %{GREEDYDATA:message}
                        if !cached_grok!("(?:ip_flow_start|ip_flow_end) %{GREEDYDATA:message}")
                            .extract_into(&input, event)?
                        {}
                    }
                    Ok(())
                })();
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
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
                let _cond = { event.has_value("_temp.event") };
                if _cond {
                    if let Some(kv_str) = event.get_string("_temp.event") {
                        for pair in kv_str.split(" ") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "_temp.event".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    event.set(key, value)?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.has_value("src") };
                if _cond {
                    if let Some(s) = event.get_string("src") {
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
                let _cond = { event.has_value("sport") };
                if _cond {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "sport".into(),
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
                                    path: "sport".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("source.port", converted)?;
                    }
                }
                let _cond = { event.has_value("translated_src_ip") };
                if _cond {
                    if let Some(s) = event.get_string("translated_src_ip") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "translated_src_ip".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("source.nat.ip", s)?;
                    }
                }
                let _cond =
                    { event.has_value("translated_port") && event.has_value("source.nat.ip") };
                if _cond {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "translated_port".into(),
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
                                    path: "translated_port".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("source.nat.port", converted)?;
                    }
                }
                let _cond = { event.has_value("dst") };
                if _cond {
                    if let Some(s) = event.get_string("dst") {
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
                let _cond = { event.has_value("dport") };
                if _cond {
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
                }
                let _cond = { event.has_value("translated_dst_ip") };
                if _cond {
                    if let Some(s) = event.get_string("translated_dst_ip") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "translated_dst_ip".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("destination.nat.ip", s)?;
                    }
                }
                let _cond =
                    { event.has_value("translated_port") && event.has_value("destination.nat.ip") };
                if _cond {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "translated_port".into(),
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
                                    path: "translated_port".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("destination.nat.port", converted)?;
                    }
                }
                event.rename("protocol", "network.protocol")?;
                // End nested pipeline: "ipflows"
            }

            let _cond = { event.get_str("cisco_meraki.event_type") == Some("airmarshal_events") };
            if _cond {
                // Begin nested pipeline: "airmarshal"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        let mut remaining: &str = &input;
                        if let Some(pos) = remaining.find(" airmarshal_events ") {
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" airmarshal_events ") {
                            remaining = rest;
                        }
                        event.set("message", remaining)?;
                    }
                    Ok(())
                })();
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
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
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: %{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}
                    if !cached_grok!(
                        "%{GREEDYDATA} ssid=%{QS:_temp.ssid}%{SPACE}%{GREEDYDATA:_temp.kvline}"
                    )
                    .extract_into(&input, event)?
                    {}
                }
                if let Some(input) = event.get_string("_temp.ssid") {
                    let mut remaining: &str = &input;
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
                if let Some(kv_str) = event.get_string("_temp.kvline") {
                    for pair in kv_str.split(" ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_temp.kvline".into(),
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
                                event.set(&format!("_temp.kv.{}", key), value)?;
                            }
                        }
                    }
                }
                let _cond = { event.has_value("_temp.kv.ssid") };
                if _cond {
                    event.rename("_temp.kv.ssid", "network.name")?;
                }
                event.rename("_temp.kv.bssid", "cisco_meraki.bssid")?;
                let _cond = {
                    event.get_str("cisco_meraki.event_subtype") == Some("ssid_spoofing_detected")
                };
                if _cond {
                    event.rename("_temp.kv.vap", "cisco_meraki.vap")?;
                }
                if let Some(s) = event.get_string("_temp.kv.src") {
                    let re = cached_regex!("[-:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("source.mac", replaced)?;
                }
                if let Some(s) = event.get_string("_temp.kv.dst") {
                    let re = cached_regex!("[-:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("destination.mac", replaced)?;
                }
                let _cond =
                    { event.get_str("cisco_meraki.event_subtype") == Some("rogue_ssid_detected") };
                if _cond {
                    if let Some(s) = event.get_string("_temp.kv.wired_mac") {
                        let re = cached_regex!("[-:.]");
                        let replaced = re.replace_all(&s, "-").into_owned();
                        event.set("_temp.observer.mac", replaced)?;
                    }
                }
                let _cond = { event.has_value("_temp.observer.mac") };
                if _cond {
                    event.append(
                        "observer.mac",
                        event
                            .get("_temp.observer.mac")
                            .cloned()
                            .unwrap_or(Value::Null),
                    )?;
                }
                let _cond =
                    { event.get_str("cisco_meraki.event_subtype") == Some("rogue_ssid_detected") };
                if _cond {
                    event.rename("_temp.kv.vlan_id", "network.vlan.id")?;
                }
                event.rename("_temp.kv.channel", "cisco_meraki.channel")?;
                event.rename("_temp.kv.fc_type", "cisco_meraki.fc_type")?;
                event.rename("_temp.kv.fc_subtype", "cisco_meraki.fc_subtype")?;
                // End nested pipeline: "airmarshal"
            }

            let _cond = { event.get_str("cisco_meraki.event_type") == Some("security_event") };
            if _cond {
                // Begin nested pipeline: "security"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        let mut remaining: &str = &input;
                        if let Some(pos) = remaining.find(" security_event ") {
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" security_event ") {
                            remaining = rest;
                        }
                        event.set("message", remaining)?;
                    }
                    Ok(())
                })();
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
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
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^%{DATA} (security_event|ids-alerts) (%{WORD}\\s)?%{DATA:_temp.kvs}(\\smessage:\\s?%{DATA:message})?$
                    if !cached_grok!("^%{DATA} (security_event|ids-alerts) (%{WORD}\\s)?%{DATA:_temp.kvs}(\\smessage:\\s?%{DATA:message})?$").extract_into(&input, event)? {
                }
                }
                if let Some(kv_str) = event.get_string("_temp.kvs") {
                    for pair in kv_str.split(" ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_temp.kvs".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let value = value.trim_matches(|c| " '\"".contains(c));
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
                    if let Some(s) = event.get_string("dhost") {
                        let re = cached_regex!("[-:.]");
                        let replaced = re.replace_all(&s, "-").into_owned();
                        event.set("cisco_meraki.security.dhost", replaced)?;
                    }
                }
                if event.has("direction") {
                    event.rename("direction", "network.direction")?;
                }
                if event.has("protocol") {
                    if let Some(s) = event.get_string("protocol") {
                        let lowered = s.to_lowercase();
                        event.set("network.protocol", lowered)?;
                    }
                }
                if event.has("decision") {
                    event.rename("decision", "cisco_meraki.security.decision")?;
                }
                let _cond = { event.has_value("url") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(uri_str) = event.get_string("url") {
                            if let Ok(url) = url::Url::parse(&uri_str) {
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
                    if let Some(s) = event.get_string("mac") {
                        let re = cached_regex!("[-:.]");
                        let replaced = re.replace_all(&s, "-").into_owned();
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
                let _cond = {
                    event.get_str("cisco_meraki.event_subtype")
                        != Some("security_filtering_disposition_change")
                        && event.has_value("src")
                };
                if _cond {
                    if let Some(input) = event.get_string("src") {
                        // Grok pattern: ^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$
                        if !cached_grok!("^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$")
                            .extract_into(&input, event)?
                        {
                            // Grok pattern: ^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$
                            if !cached_grok!("^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$")
                                .extract_into(&input, event)?
                            {
                                // Grok pattern: ^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$
                                if !cached_grok_mapped!("^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$", [("_temp_src_ip", "_temp.src_ip")]).extract_into(&input, event)? {
                // Grok pattern: ^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$
                if !cached_grok!("^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$").extract_into(&input, event)? {
                }
                }
                            }
                        }
                    }
                }
                if event.has("_temp.src_ip") {
                    if let Some(s) = event.get_string("_temp.src_ip") {
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
                let _cond = { event.get_str("sport") != Some("0") };
                if _cond {
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
                                    Value::Number(n) => json!(
                                        n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64)
                                    ),
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
                let _cond = {
                    event.get_str("cisco_meraki.event_subtype")
                        != Some("security_filtering_disposition_change")
                        && event.has_value("dst")
                };
                if _cond {
                    if let Some(input) = event.get_string("dst") {
                        // Grok pattern: ^%{IPV4:_temp.dst_ip}:(?P<dport>(?:[0-9]+))$
                        if !cached_grok!("^%{IPV4:_temp.dst_ip}:(?P<dport>(?:[0-9]+))$")
                            .extract_into(&input, event)?
                        {
                            // Grok pattern: ^\\[%{IPV6:_temp.dst_ip}\\]:(?P<dport>(?:[0-9]+))$
                            if !cached_grok!("^\\[%{IPV6:_temp.dst_ip}\\]:(?P<dport>(?:[0-9]+))$")
                                .extract_into(&input, event)?
                            {
                                // Grok pattern: ^(?P<_temp_dst_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<dport>(?:[0-9]+))$
                                if !cached_grok_mapped!("^(?P<_temp_dst_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<dport>(?:[0-9]+))$", [("_temp_dst_ip", "_temp.dst_ip")]).extract_into(&input, event)? {
                // Grok pattern: ^%{IPV6:_temp.dst_ip}(?:(?: port |[p#.]))(?P<dport>(?:[0-9]+))$
                if !cached_grok!("^%{IPV6:_temp.dst_ip}(?:(?: port |[p#.]))(?P<dport>(?:[0-9]+))$").extract_into(&input, event)? {
                }
                }
                            }
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("_temp.dst_ip") {
                        if let Some(s) = event.get_string("_temp.dst_ip") {
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
                let _cond = {
                    event.get_str("dport") != Some("0")
                        && event.get_str("cisco_meraki.event_subtype")
                            != Some("security_filtering_disposition_change")
                };
                if _cond {
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
                                    Value::Number(n) => json!(
                                        n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64)
                                    ),
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

            let _cond = { event.get_str("cisco_meraki.event_type") == Some("ids-alerts") };
            if _cond {
                // Begin nested pipeline: "idsalerts"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        let mut remaining: &str = &input;
                        if let Some(pos) = remaining.find(" ids-alerts ") {
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" ids-alerts ") {
                            remaining = rest;
                        }
                        event.set("message", remaining)?;
                    }
                    Ok(())
                })();
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
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
                if let Some(s) = event.get_string("protocol") {
                    let lowered = s.to_lowercase();
                    event.set("network.protocol", lowered)?;
                }
                let _cond = { event.has_value("src") };
                if _cond {
                    if let Some(input) = event.get_string("src") {
                        // Grok pattern: ^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$
                        if !cached_grok!("^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$")
                            .extract_into(&input, event)?
                        {
                            // Grok pattern: ^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$
                            if !cached_grok!("^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$")
                                .extract_into(&input, event)?
                            {
                                // Grok pattern: ^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$
                                if !cached_grok_mapped!("^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$", [("_temp_src_ip", "_temp.src_ip")]).extract_into(&input, event)? {
                // Grok pattern: ^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$
                if !cached_grok!("^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$").extract_into(&input, event)? {
                }
                }
                            }
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("_temp.src_ip") {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "sport".into(),
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

            let _cond = { event.get_str("cisco_meraki.event_type") == Some("events") };
            if _cond {
                // Begin nested pipeline: "events"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        let mut remaining: &str = &input;
                        if let Some(pos) = remaining.find(" events ") {
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" events ") {
                            remaining = rest;
                        }
                        event.set("message", remaining)?;
                    }
                    Ok(())
                })();
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
                        event.get("event.original").cloned().unwrap_or(Value::Null),
                    )?;
                }
                if event.has("_temp.event_original_lower") {
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
                    if event.has("cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac") {
                        event.rename(
                            "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac",
                            "client.mac",
                        )?;
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
                    if event.has("cisco_meraki.{{{cisco_meraki.event_subtype}}}.ip_src") {
                        event.rename(
                            "cisco_meraki.{{{cisco_meraki.event_subtype}}}.ip_src",
                            "source.ip",
                        )?;
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
                    if event.has("cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_ip") {
                        event.rename(
                            "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_ip",
                            "_temp.client_ip",
                        )?;
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
                    if event.has("cisco_meraki.{{{cisco_meraki.event_subtype}}}.identity") {
                        event.rename(
                            "cisco_meraki.{{{cisco_meraki.event_subtype}}}.identity",
                            "user.name",
                        )?;
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
                                return Err(TransformError::ParseError { path: "cisco_meraki.multiple_dhcp_servers_detected.original_server_ip".into(), message: format!("cannot convert '{}' to IP", s) });
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
                                path: "cisco_meraki.multiple_dhcp_servers_detected.server_ip"
                                    .into(),
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
                    event.get_str("cisco_meraki.anyconnect_vpn_session_manager.action")
                        == Some("Added")
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
                let _cond = {
                    event.get_str("cisco_meraki.event_subtype") == Some("anyconnect_vpn_connect")
                };
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
                let _cond = {
                    event.get_str("cisco_meraki.event_subtype") == Some("anyconnect_vpn_connect")
                };
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
                        if let Some(pos) =
                            remaining.find(" events anyconnect_vpn_disconnect user id '")
                        {
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) =
                            remaining.strip_prefix(" events anyconnect_vpn_disconnect user id '")
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
                        if let Some(pos) = remaining.find("martian_vlan ") {
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix("martian_vlan ") {
                            remaining = rest;
                        }
                        event.set("_temp.martian_vlan", remaining)?;
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
                                    event.set(
                                        &format!("cisco_meraki.martian_vlan.{}", key),
                                        value,
                                    )?;
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
                            if !cached_grok!("^%{IPV4:_temp.client_ip}$")
                                .extract_into(&input, event)?
                            {
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
                if event.has("user.name") {
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
                        if event.has("user.email") {
                            if let Some(input) = event.get_string("user.email") {
                                let mut remaining: &str = &input;
                                if let Some(pos) = remaining.find("@") {
                                    event.set("user.name", &remaining[..pos])?;
                                    remaining = &remaining[pos..];
                                }
                                if let Some(rest) = remaining.strip_prefix("@") {
                                    remaining = rest;
                                }
                                event.set("user.domain", remaining)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("user.name")
                        && event.get("user.name").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
                };
                if _cond {
                    if event.has("user.name") {
                        if let Some(input) = event.get_string("user.name") {
                            let mut remaining: &str = &input;
                            if let Some(pos) = remaining.find("\\") {
                                event.set("user.domain", &remaining[..pos])?;
                                remaining = &remaining[pos..];
                            }
                            if let Some(rest) = remaining.strip_prefix("\\") {
                                remaining = rest;
                            }
                            event.set("user.name", remaining)?;
                        }
                    }
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append(
                            "related.user",
                            event.get("user.name").cloned().unwrap_or(Value::Null),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("user.email") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append(
                            "related.user",
                            event.get("user.email").cloned().unwrap_or(Value::Null),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    event.append(
                        "related.ip",
                        event.get("source.ip").cloned().unwrap_or(Value::Null),
                    )?;
                }
                let _cond = { event.has_value("client.ip") };
                if _cond {
                    event.append(
                        "related.ip",
                        event.get("client.ip").cloned().unwrap_or(Value::Null),
                    )?;
                }
                // End nested pipeline: "events"
            }

            let _cond = { event.get_str("cisco_meraki.event_type") == Some("urls") };
            if _cond {
                // Begin nested pipeline: "urls"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        let mut remaining: &str = &input;
                        if let Some(pos) = remaining.find(" urls ") {
                            remaining = &remaining[pos..];
                        }
                        if let Some(rest) = remaining.strip_prefix(" urls ") {
                            remaining = rest;
                        }
                        event.set("message", remaining)?;
                    }
                    Ok(())
                })();
                if let Some(input) = event.get_string("event.original") {
                    let mut remaining: &str = &input;
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
                        if let Some(input) = event.get_string("mac") {
                            let mut remaining: &str = &input;
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
                if let Some(input) = event.get_string("src") {
                    // Grok pattern: ^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$
                    if !cached_grok!("^%{IPV4:_temp.src_ip}:(?P<sport>(?:[0-9]+))$")
                        .extract_into(&input, event)?
                    {
                        // Grok pattern: ^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$
                        if !cached_grok!("^\\[%{IPV6:_temp.src_ip}\\]:(?P<sport>(?:[0-9]+))$")
                            .extract_into(&input, event)?
                        {
                            // Grok pattern: ^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$
                            if !cached_grok_mapped!("^(?P<_temp_src_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<sport>(?:[0-9]+))$", [("_temp_src_ip", "_temp.src_ip")]).extract_into(&input, event)? {
                // Grok pattern: ^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$
                if !cached_grok!("^%{IPV6:_temp.src_ip}(?:(?: port |[p#.]))(?P<sport>(?:[0-9]+))$").extract_into(&input, event)? {
                }
                }
                        }
                    }
                }
                if let Some(s) = event.get_string("_temp.src_ip") {
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
                                    json!(s.parse::<i64>().map_err(|_| {
                                        TransformError::ParseError {
                                            path: "sport".into(),
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
                                    path: "sport".into(),
                                    message: "cannot convert to integer".into(),
                                });
                            }
                        };
                        event.set("source.port", converted)?;
                    }
                    Ok(())
                })();
                if let Some(input) = event.get_string("dst") {
                    // Grok pattern: ^%{IPV4:_temp.dst_ip}:(?P<dport>(?:[0-9]+))$
                    if !cached_grok!("^%{IPV4:_temp.dst_ip}:(?P<dport>(?:[0-9]+))$")
                        .extract_into(&input, event)?
                    {
                        // Grok pattern: ^\\[%{IPV6:_temp.dst_ip}\\]:(?P<dport>(?:[0-9]+))$
                        if !cached_grok!("^\\[%{IPV6:_temp.dst_ip}\\]:(?P<dport>(?:[0-9]+))$")
                            .extract_into(&input, event)?
                        {
                            // Grok pattern: ^(?P<_temp_dst_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<dport>(?:[0-9]+))$
                            if !cached_grok_mapped!("^(?P<_temp_dst_ip>(?:([0-9A-Fa-f]{1,4}:){7}[0-9A-Fa-f]{1,4})):(?P<dport>(?:[0-9]+))$", [("_temp_dst_ip", "_temp.dst_ip")]).extract_into(&input, event)? {
                // Grok pattern: ^%{IPV6:_temp.dst_ip}(?:(?: port |[p#.]))(?P<dport>(?:[0-9]+))$
                if !cached_grok!("^%{IPV6:_temp.dst_ip}(?:(?: port |[p#.]))(?P<dport>(?:[0-9]+))$").extract_into(&input, event)? {
                }
                }
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("_temp.dst_ip") {
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
                let _cond = {
                    event.get_str("dport") != Some("0")
                        && event.get_str("cisco_meraki.event_subtype")
                            != Some("security_filtering_disposition_change")
                };
                if _cond {
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
                        Ok(())
                    })();
                }
                if let Some(s) = event.get_string("mac") {
                    let re = cached_regex!("[-:.]");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("cisco_meraki.urls.mac", replaced)?;
                }
                let _cond = {
                    !(event
                        .get_str("http.request.method")
                        .is_some_and(|s| s.to_lowercase() == "unknown"))
                };
                if _cond {
                    event.set("cisco_meraki.event_subtype", json!("http_access"))?;
                }
                let _cond = {
                    event
                        .get_str("http.request.method")
                        .is_some_and(|s| s.to_lowercase() == "unknown")
                };
                if _cond {
                    event.set("cisco_meraki.event_subtype", json!("http_access_error"))?;
                }
                if event.has("user_agent.original") {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
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
                let _cond = {
                    event.has_value("url.original")
                        && event.get_str("url.original").is_some_and(|s| !s.is_empty())
                };
                if _cond {
                    if let Some(uri_str) = event.get_string("url.original") {
                        if let Ok(url) = url::Url::parse(&uri_str) {
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
                }
                if event.has("url.domain") {
                    if let Some(domain_str) = event.get_string("url.domain") {
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

            let _cond = { event.has_value("cisco_meraki.event_subtype") };
            if _cond {
                // Painless script
                // Source: def eventMap = params.get('eventmap');\ndef eventData = eventMap.get(ctx.cisco_meraki.event_subtype);\nif (eventData == null) {\n  ctx.event.action = ctx.cisco_meraki.event_subtype;\n  return;\n}\ndef eventCategory = eventData.get('category');\ndef eventType = eventData.get('type');\ndef eventAction = eventData.get('action');\nif (eventType != null) {\n  for (def t : eventType) {\n    ctx.event.type.add(t);\n  }\n}\nif (eventCategory != null) {\n  for (def c : eventCategory) {\n    ctx.event.category.add(c);\n  }\n}\nif (eventAction != null) {\n  ctx.event.action = eventAction;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"def eventMap = params.get('eventmap');\ndef eventData = eventMap.get(ctx.cisco_meraki.event_subtype);\nif (eventData == null) {\n  ctx.event.action = ctx.cisco_meraki.event_subtype;\n  return;\n}\ndef eventCategory = eventData.get('category');\ndef eventType = eventData.get('type');\ndef eventAction = eventData.get('action');\nif (eventType != null) {\n  for (def t : eventType) {\n    ctx.event.type.add(t);\n  }\n}\nif (eventCategory != null) {\n  for (def c : eventCategory) {\n    ctx.event.category.add(c);\n  }\n}\nif (eventAction != null) {\n  ctx.event.action = eventAction;\n}"#
                    ),
                    cached_params!(
                        "{\"eventmap\":{\"8021x_client_deauth\":{\"action\":\"wifi-8021x-client-deauth\",\"category\":[\"authentication\"],\"type\":[\"end\"]},\"8021x_deauth\":{\"action\":\"wifi-8021x-failed-auth-or-deauth\",\"category\":[\"authentication\"],\"type\":[\"end\",\"denied\"]},\"8021x_eap_failure\":{\"action\":\"wifi-8021x-failed-authentication-attempt\",\"category\":[\"authentication\"],\"type\":[\"end\",\"denied\"]},\"8021x_eap_success\":{\"action\":\"wifi-8021x-auth\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"Site-to-Site VPN\":{\"action\":\"site-to-site-vpn\",\"type\":[\"access\"]},\"anyconnect_vpn_connect\":{\"action\":\"anyconnect_vpn_connect\",\"category\":[\"session\"],\"type\":[\"access\",\"allowed\",\"start\"]},\"aps_association_reject\":{\"action\":\"association-rejected-for-load-balancing\"},\"arp_blocked\":{\"action\":\"arp_blocked\",\"type\":[\"denied\"]},\"association\":{\"action\":\"wifi-association-request\",\"type\":[\"access\",\"connection\"]},\"client_vpn_connect\":{\"action\":\"site-to-site-vpn\",\"category\":[\"session\"],\"type\":[\"access\",\"allowed\",\"start\"]},\"device_packet_flood\":{\"action\":\"wireless-packet-flood-detected\"},\"dfs_event\":{\"action\":\"dynamic-frequency-selection-detected\"},\"dhcp_blocked\":{\"action\":\"dhcp_blocked\",\"type\":[\"denied\"]},\"dhcp_no_offer\":{\"action\":\"dhcp-no-offer\",\"type\":[\"access\",\"denied\"]},\"dhcp_offer\":{\"action\":\"dhcp-offer\",\"type\":[\"access\",\"allowed\"]},\"disassociation\":{\"action\":\"wifi-disassociation-request\",\"category\":[\"session\"],\"type\":[\"access\",\"end\"]},\"flow_allowed\":{\"action\":\"layer3-firewall-allowed-flow\",\"type\":[\"connection\",\"start\"]},\"flow_denied\":{\"action\":\"layer3-firewall-denied-flow\",\"type\":[\"access\",\"denied\"]},\"http_access\":{\"action\":\"http-access\",\"category\":[\"web\"],\"type\":[\"access\"]},\"http_access_error\":{\"action\":\"http-access-error\",\"category\":[\"web\"],\"type\":[\"error\"]},\"ids_alerted\":{\"action\":\"ids-signature-matched\",\"category\":[\"intrusion_detection\"]},\"ip_session_initiated\":{\"action\":\"ip-session-initiated\",\"type\":[\"access\",\"start\"]},\"multiple_dhcp_servers_detected\":{\"action\":[\"multiple_dhcp_servers_detected\"],\"type\":[\"protocol\"]},\"ra_blocked\":{\"action\":\"ra_blocked\",\"type\":[\"denied\"]},\"rogue_ssid_detected\":{\"action\":\"rogue-ssid-detected\"},\"security_filtering_disposition_change\":{\"action\":\"issued-retrospective-malicious-disposition\",\"category\":[\"file\",\"malware\"]},\"security_filtering_file_scanned\":{\"action\":\"malicious-file-actioned\",\"category\":[\"file\",\"malware\"]},\"splash_auth\":{\"action\":\"splash-authentication\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"ssid_spoofing_detected\":{\"action\":\"ssid-spoofing-detected\"},\"vpn_connectivity_change\":{\"action\":\"vpn-connectivity-change\",\"category\":[\"session\"],\"type\":[\"connection\"]},\"wpa_auth\":{\"action\":\"wifi-wpa-authentication\",\"category\":[\"authentication\"],\"type\":[\"start\",\"access\"]},\"wpa_deauth\":{\"action\":\"wifi-wpa-failed-auth-or-deauth\",\"category\":[\"authentication\"],\"type\":[\"end\",\"denied\"]}}}"
                    ),
                )?;
            }

            let _cond = { !event.has_value("source.geo") && event.has_value("source.ip") };
            if _cond {
                if event.has("source.ip") {
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
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                if event.has("source.ip") {
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
            }

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond =
                { !event.has_value("destination.geo") && event.has_value("destination.ip") };
            if _cond {
                if event.has("destination.ip") {
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
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                if event.has("destination.ip") {
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

            let _cond = { !event.has_value("client.geo") && event.has_value("client.ip") };
            if _cond {
                if event.has("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
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

            let _cond = { event.has_value("client.ip") };
            if _cond {
                if event.has("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
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
                cached_script!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("tags", json!("preserve_original_event"))?;
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
