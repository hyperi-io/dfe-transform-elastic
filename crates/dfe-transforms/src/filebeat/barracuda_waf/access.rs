// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `access` pipeline.
pub struct Access;

impl Transform for Access {
    fn name(&self) -> &str {
        "access"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("_temp.remMessage") {
                if let Some(input) = event.get_string("_temp.remMessage") {
                    // Grok pattern: ^%{IP:destination.ip} %{NUMBER:destination.port:long} %{IP:client.ip} %{NUMBER:client.port:long} %{NOTSPACE:client.user.id} %{GREEDYDATA:client.user.name} %{NOTSPACE:http.request.method} %{NOTSPACE:network.protocol} %{DATA:_temp.hostIp} (HTTP/)?%{NOTSPACE:http.version} %{NONNEGINT:http.response.status_code:long} %{NUMBER:http.response.bytes:long} %{NONNEGINT:http.request.bytes:long} %{NONNEGINT:barracuda.waf.cache_hit:long} %{NONNEGINT:barracuda.waf.response_timetaken:long} %{IP:_temp.serverIp} %{NUMBER:_temp.serverPort:long} %{NONNEGINT:barracuda.waf.server_time:long} %{NOTSPACE:barracuda.waf.sessionid}\\s+((?P<barracuda_waf_response_type>(?:(?:INTERNAL|SERVER))) )?((?P<barracuda_waf_profile_matched>(?:(?:DEFAULT|PROFILED))) )?((?P<barracuda_waf_protected>(?:(?:PASSIVE|PROTECTED|UNPROTECTED))) )?((?P<barracuda_waf_wf_matched>(?:(?:IN)?VALID)) )?(%{DATA:url.path})?\\s+%{NOTSPACE:url.query}\\s+%{NOTSPACE:http.request.referrer} %{DATA:barracuda.waf.request_cookie} \"?\"%{DATA:user_agent.original}\"\"? %{IP:barracuda.waf.proxy.ip} %{NONNEGINT:barracuda.waf.proxy.port:long} (\"-\"|%{DATA:user.name}) %{DATA:_temp.raw_custom_headers}\\s+(?P<event_id>(?:[A-Za-z0-9\\-]+))\\s*$
                    let _ = cached_grok_mapped!("^%{IP:destination.ip} %{NUMBER:destination.port:long} %{IP:client.ip} %{NUMBER:client.port:long} %{NOTSPACE:client.user.id} %{GREEDYDATA:client.user.name} %{NOTSPACE:http.request.method} %{NOTSPACE:network.protocol} %{DATA:_temp.hostIp} (HTTP/)?%{NOTSPACE:http.version} %{NONNEGINT:http.response.status_code:long} %{NUMBER:http.response.bytes:long} %{NONNEGINT:http.request.bytes:long} %{NONNEGINT:barracuda.waf.cache_hit:long} %{NONNEGINT:barracuda.waf.response_timetaken:long} %{IP:_temp.serverIp} %{NUMBER:_temp.serverPort:long} %{NONNEGINT:barracuda.waf.server_time:long} %{NOTSPACE:barracuda.waf.sessionid}\\s+((?P<barracuda_waf_response_type>(?:(?:INTERNAL|SERVER))) )?((?P<barracuda_waf_profile_matched>(?:(?:DEFAULT|PROFILED))) )?((?P<barracuda_waf_protected>(?:(?:PASSIVE|PROTECTED|UNPROTECTED))) )?((?P<barracuda_waf_wf_matched>(?:(?:IN)?VALID)) )?(%{DATA:url.path})?\\s+%{NOTSPACE:url.query}\\s+%{NOTSPACE:http.request.referrer} %{DATA:barracuda.waf.request_cookie} \"?\"%{DATA:user_agent.original}\"\"? %{IP:barracuda.waf.proxy.ip} %{NONNEGINT:barracuda.waf.proxy.port:long} (\"-\"|%{DATA:user.name}) %{DATA:_temp.raw_custom_headers}\\s+(?P<event_id>(?:[A-Za-z0-9\\-]+))\\s*$", [("barracuda_waf_response_type", "barracuda.waf.response_type"), ("barracuda_waf_profile_matched", "barracuda.waf.profile_matched"), ("barracuda_waf_protected", "barracuda.waf.protected"), ("barracuda_waf_wf_matched", "barracuda.waf.wf_matched"), ("event_id", "event.id")]).extract_into(&input, event)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_temp_remMessage_access")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("_temp.hostIp") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(val) = event.get("_temp.hostIp") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_temp.hostIp".into(),
                            message,
                        })?;
                    event.set("_temp.hostIp", converted)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                    let _cond = { event.has_value("_temp.hostIp") && (event.get_str("_temp.hostIp") != Some("-") || event.get_str("_temp.hostIp") != Some("")) };
                    if _cond {
                        event.append_unique("host.name", json!(event.get("_temp.hostIp").map_or_else(String::new, template_to_string)))?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.remove("_temp.hostIp");
                        Ok(())
                    })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("_temp.hostIp") };
            if _cond {
                event.append_unique("host.ip", json!(event.get("_temp.hostIp").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("_temp.raw_custom_headers") && event.get_str("_temp.raw_custom_headers") != Some("") };
            if _cond {
                // Painless script
                // Source: def headers = ctx._temp.raw_custom_headers.splitOnToken(' ');\nif (ctx.barracuda.waf.custom_header == null) {\n    ctx.barracuda.waf.custom_header = new HashMap();\n}\nfor (int i = 0; i < headers.length; i++) {\n  ctx.barracuda.waf.custom_header[params[(i+1).toString()]] = headers[i];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def headers = ctx._temp.raw_custom_headers.splitOnToken(' ');\nif (ctx.barracuda.waf.custom_header == null) {\n    ctx.barracuda.waf.custom_header = new HashMap();\n}\nfor (int i = 0; i < headers.length; i++) {\n  ctx.barracuda.waf.custom_header[params[(i+1).toString()]] = headers[i];\n}\n"#), cached_params!("{\"1\":\"accept_encoding\",\"2\":\"host\",\"3\":\"connection\",\"4\":\"cache_control\",\"5\":\"user_agent\",\"6\":\"content_type\"}"))?;
            }

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

            if event.has_value("_temp.serverIp") {
                if let Some(val) = event.get("_temp.serverIp") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "_temp.serverIp".into(),
                            message,
                        })?;
                    event.set("server.ip", converted)?;
                }
            }

            if event.has_value("_temp.serverPort") {
                if let Some(val) = event.get("_temp.serverPort") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "_temp.serverPort".into(),
                            message,
                        })?;
                    event.set("server.port", converted)?;
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

            if event.has_value("barracuda.waf.cache_hit") {
                if let Some(val) = event.get("barracuda.waf.cache_hit") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "barracuda.waf.cache_hit".into(),
                            message,
                        })?;
                    event.set("barracuda.waf.cache_hit", converted)?;
                }
            }

            if event.has_value("barracuda.waf.response_timetaken") {
                if let Some(val) = event.get("barracuda.waf.response_timetaken") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "barracuda.waf.response_timetaken".into(),
                            message,
                        })?;
                    event.set("barracuda.waf.response_timetaken", converted)?;
                }
            }

            if event.has_value("barracuda.waf.server_time") {
                if let Some(val) = event.get("barracuda.waf.server_time") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "barracuda.waf.server_time".into(),
                            message,
                        })?;
                    event.set("barracuda.waf.server_time", converted)?;
                }
            }

            if event.has_value("http.request.bytes") {
                if let Some(val) = event.get("http.request.bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "http.request.bytes".into(),
                            message,
                        })?;
                    event.set("http.request.bytes", converted)?;
                }
            }

            if event.has_value("http.response.bytes") {
                if let Some(val) = event.get("http.response.bytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "http.response.bytes".into(),
                            message,
                        })?;
                    event.set("http.response.bytes", converted)?;
                }
            }

            if event.has_value("http.response.status_code") {
                if let Some(val) = event.get("http.response.status_code") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "http.response.status_code".into(),
                            message,
                        })?;
                    event.set("http.response.status_code", converted)?;
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

            if event.has_value("server.ip") {
                if let Some(ip_str) = event.get_string("server.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("server.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("server.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("server.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("server.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("server.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("server.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("server.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("server.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if let Some(v) = event.get("client").cloned() {
                event.set("source", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("barracuda.waf.proxy.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("barracuda.waf.proxy.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("server.ip").map_or_else(String::new, template_to_string)))?;
            }

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("access"), json!("connection")]))?;

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
