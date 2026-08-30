// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `webfirewall` pipeline.
pub struct Webfirewall;

impl Transform for Webfirewall {
    fn name(&self) -> &str {
        "webfirewall"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_temp.remMessage") {
                if let Some(input) = event.get_string("_temp.remMessage") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("barracuda.waf.severity_level", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("barracuda.waf.attack_description", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("_temp.clientIp", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("_temp.clientPort", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("_temp.destIp", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("_temp.destPort", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("rule.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("rule.category", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("barracuda.waf.action_taken", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("barracuda.waf.followup_action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("_temp.remMessage", remaining));
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

            if event.has_value("_temp.remMessage") {
                if let Some(input) = event.get_string("_temp.remMessage") {
                    // Grok pattern: \\[%{DATA:barracuda.waf.attack_details}\\] %{GREEDYDATA:_temp.remMessage}
                    let _ = cached_grok!("\\[%{DATA:barracuda.waf.attack_details}\\] %{GREEDYDATA:_temp.remMessage}").extract_into(&input, event)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_temp.remMessage") {
                if let Some(input) = event.get_string("_temp.remMessage") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("http.request.method", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("url.path", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("barracuda.waf.protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("barracuda.waf.sessionid", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("_temp.remMessage", remaining));
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

            if event.has_value("_temp.remMessage") {
                if let Some(input) = event.get_string("_temp.remMessage") {
                    // Grok pattern: \\\"%{DATA:user_agent.original}\\\"%{SPACE}%{GREEDYDATA:_temp.remMessage}
                    let _ = cached_grok!("\\\"%{DATA:user_agent.original}\\\"%{SPACE}%{GREEDYDATA:_temp.remMessage}").extract_into(&input, event)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("_temp.remMessage") {
                if let Some(input) = event.get_string("_temp.remMessage") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("_temp.proxyIp", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("_temp.proxyPort", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("http.request.referrer", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("user.id", remaining));
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

            if event.has_value("_temp.clientIp") {
                if let Some(val) = event.get("_temp.clientIp") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_temp.clientIp".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }

            if event.has_value("_temp.clientPort") {
                if let Some(val) = event.get("_temp.clientPort") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_temp.clientPort".into(),
                            message,
                        })?;
                    event.set("client.port", converted)?;
                }
            }

            if event.has_value("_temp.destIp") {
                if let Some(val) = event.get("_temp.destIp") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_temp.destIp".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }

            if event.has_value("_temp.destPort") {
                if let Some(val) = event.get("_temp.destPort") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_temp.destPort".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }

            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                        if let Some(version) = ua.version { event.set("user_agent.version", json!(version))?; }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                            }
                        }
                        if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                    }
                }
            }

            if event.has_value("_temp.proxyIp") {
                if let Some(val) = event.get("_temp.proxyIp") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_temp.proxyIp".into(),
                            message,
                        })?;
                    event.set("network.forwarded_ip", converted)?;
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

            if event.has_value("client.ip") {
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

            if let Some(v) = event.get("client").cloned() {
                event.set("source", v)?;
            }

            if let Some(v) = event.get("destination").cloned() {
                event.set("server", v)?;
            }

            event.set("event.category", Value::Array(vec![json!("network"), json!("intrusion_detection")]))?;

            if let Some(v) = event.get("barracuda.waf.action_taken").cloned() {
                event.set("event.action", v)?;
            }

            let _cond = { event.has_value("barracuda.waf.severitylevel") && ["ALER", "EMER", "CRIT", "ALERT", "CRITICAL", "EMERGENCY"].contains(&event.get_str("barracuda.waf.severity_level").unwrap_or("")) };
            if _cond {
            event.set("event.kind", json!("alert"))?;
            }

            let _cond = { !event.has_value("event.kind") };
            if _cond {
            event.set("event.kind", json!("event"))?;
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
