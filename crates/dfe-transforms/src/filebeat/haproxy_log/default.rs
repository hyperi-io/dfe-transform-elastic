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
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: (?P<haproxy_request_date>(?:(%{MONTH} %{MONTHDAY} %{HOUR}:%{MINUTE}:%{SECOND})|%{SYSLOGTIMESTAMP})) %{IPORHOST:haproxy.source} %{PROG:process.name}(?:\\[%{POSINT:process.pid:long}\\])?: %{GREEDYDATA} (%{IPORHOST:source.address}|-):%{POSINT:source.port:long} %{WORD} %{IPORHOST:destination.ip}:%{POSINT:destination.port:long} \\(%{WORD:haproxy.frontend_name}/%{WORD:haproxy.mode}\\)
                    // Grok pattern: (%{NOTSPACE:process.name}\\[%{NUMBER:process.pid:long}\\]: )?(%{IP:source.address}|-):%{NUMBER:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} (%{IPORHOST:destination.address} )?%{NUMBER:haproxy.http.request.time_wait_ms:long}/%{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:haproxy.http.request.time_wait_without_data_ms:long}/%{NUMBER:temp.duration:long} %{NUMBER:http.response.status_code:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.http.request.captured_cookie} %{NOTSPACE:haproxy.http.response.captured_cookie} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long} (\\{%{DATA:haproxy.http.request.captured_headers}\\} \\{%{DATA:haproxy.http.response.captured_headers}\\} |\\{%{DATA}\\} )?\"%{GREEDYDATA:haproxy.http.request.raw_request_line}\"( (?:%{NUMBER:haproxy.connections.fc_err:long}/%{NUMBER:haproxy.connections.ssl_fc_err:long}/%{NUMBER:haproxy.connections.ssl_c_err:long}/%{NUMBER:haproxy.connections.ssl_c_ca_err:long}/%{NUMBER:temp.ssl_fc_is_resumed:long} %{HOSTNAME:tls.client.server_name}/%{DATA:tls.version_protocol}v%{NUMBER:tls.version}/%{NOTSPACE:tls.cipher}))?
                    // Grok pattern: (%{NOTSPACE:process.name}\\[%{NUMBER:process.pid:long}\\]: )?(%{IP:source.address}|-):%{NUMBER:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name}/%{NOTSPACE:haproxy.bind_name} %{GREEDYDATA:haproxy.error_message}
                    // Grok pattern: ((?:(%{MONTH} %{MONTHDAY} %{HOUR}:%{MINUTE}:%{SECOND})|%{SYSLOGTIMESTAMP}) %{IPORHOST:haproxy.source} )?(%{NOTSPACE:process.name}\\[%{NUMBER:process.pid:long}\\]: )?(%{IP:source.address}|-):%{NUMBER:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:temp.duration:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long}
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "(?P<haproxy_request_date>(?:(%{MONTH} %{MONTHDAY} %{HOUR}:%{MINUTE}:%{SECOND})|%{SYSLOGTIMESTAMP})) %{IPORHOST:haproxy.source} %{PROG:process.name}(?:\\[%{POSINT:process.pid:long}\\])?: %{GREEDYDATA} (%{IPORHOST:source.address}|-):%{POSINT:source.port:long} %{WORD} %{IPORHOST:destination.ip}:%{POSINT:destination.port:long} \\(%{WORD:haproxy.frontend_name}/%{WORD:haproxy.mode}\\)",
                                [("haproxy_request_date", "haproxy.request_date")]
                            ),
                            cached_grok!(
                                "(%{NOTSPACE:process.name}\\[%{NUMBER:process.pid:long}\\]: )?(%{IP:source.address}|-):%{NUMBER:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} (%{IPORHOST:destination.address} )?%{NUMBER:haproxy.http.request.time_wait_ms:long}/%{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:haproxy.http.request.time_wait_without_data_ms:long}/%{NUMBER:temp.duration:long} %{NUMBER:http.response.status_code:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.http.request.captured_cookie} %{NOTSPACE:haproxy.http.response.captured_cookie} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long} (\\{%{DATA:haproxy.http.request.captured_headers}\\} \\{%{DATA:haproxy.http.response.captured_headers}\\} |\\{%{DATA}\\} )?\"%{GREEDYDATA:haproxy.http.request.raw_request_line}\"( (?:%{NUMBER:haproxy.connections.fc_err:long}/%{NUMBER:haproxy.connections.ssl_fc_err:long}/%{NUMBER:haproxy.connections.ssl_c_err:long}/%{NUMBER:haproxy.connections.ssl_c_ca_err:long}/%{NUMBER:temp.ssl_fc_is_resumed:long} %{HOSTNAME:tls.client.server_name}/%{DATA:tls.version_protocol}v%{NUMBER:tls.version}/%{NOTSPACE:tls.cipher}))?"
                            ),
                            cached_grok!(
                                "(%{NOTSPACE:process.name}\\[%{NUMBER:process.pid:long}\\]: )?(%{IP:source.address}|-):%{NUMBER:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name}/%{NOTSPACE:haproxy.bind_name} %{GREEDYDATA:haproxy.error_message}"
                            ),
                            cached_grok!(
                                "((?:(%{MONTH} %{MONTHDAY} %{HOUR}:%{MINUTE}:%{SECOND})|%{SYSLOGTIMESTAMP}) %{IPORHOST:haproxy.source} )?(%{NOTSPACE:process.name}\\[%{NUMBER:process.pid:long}\\]: )?(%{IP:source.address}|-):%{NUMBER:source.port:long} \\[%{NOTSPACE:haproxy.request_date}\\] %{NOTSPACE:haproxy.frontend_name} %{NOTSPACE:haproxy.backend_name}/%{NOTSPACE:haproxy.server_name} %{NUMBER:haproxy.total_waiting_time_ms:long}/%{NUMBER:haproxy.connection_wait_time_ms:long}/%{NUMBER:temp.duration:long} %{NUMBER:haproxy.bytes_read:long} %{NOTSPACE:haproxy.termination_state} %{NUMBER:haproxy.connections.active:long}/%{NUMBER:haproxy.connections.frontend:long}/%{NUMBER:haproxy.connections.backend:long}/%{NUMBER:haproxy.connections.server:long}/%{NUMBER:haproxy.connections.retries:long} %{NUMBER:haproxy.server_queue:long}/%{NUMBER:haproxy.backend_queue:long}"
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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                if let Some(v) = event.get("_conf.tz_offset").cloned() {
                    event.set("event.timezone", v)?;
                }
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                    && event.get_str("haproxy.http.request.raw_request_line") != Some("<BADREQ>")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond = { event.has_value("url.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "url.original", "url", true, false)?;
                    Ok(())
                })();
            }

            if event.has_value("tls.version_protocol") {
                map_strings(
                    event,
                    "tls.version_protocol",
                    "tls.version_protocol",
                    str::to_lowercase,
                )?;
            }

            if let Some(v) = event
                .get("tls.client.server_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if let Some(v) = event
                .get("tls.client.server_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.domain", v)?;
            }

            let _cond = { event.get_i64("temp.ssl_fc_is_resumed") == Some(1) };
            if _cond {
                event.set("tls.resumed", json!(true))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("source.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("destination.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "destination.address".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
                Ok(())
            })();

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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
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
                    event.set("haproxy.http.request.captured_headers", Value::Array(parts))?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("haproxy.http.response.captured_headers") {
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
                Ok(())
            })();

            let _cond = { event.has_value("temp.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = Math.round(ctx.temp.duration * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.event.duration = Math.round(ctx.temp.duration * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1000000}"),
                )?;
            }

            let _cond = { event.has("http") };
            if _cond {
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
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
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
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

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.get_str("haproxy.mode") == Some("HTTP") || event.has_value("haproxy.http")
            };
            if _cond {
                event.append("event.category", json!("web"))?;
            }

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                event.append("event.type", json!("connection"))?;
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

            event.remove("temp");
            event.remove("_conf");
            event.remove("haproxy.request_date");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
