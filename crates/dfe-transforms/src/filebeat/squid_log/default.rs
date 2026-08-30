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

            if let Some(v) = event.get("message").cloned() {
                event.set("event.original", v)?;
            }

            if event.remove("message").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "message".into(),
                });
            }

            event.set("observer.vendor", json!("Squid"))?;

            event.set("observer.product", json!("Squid"))?;

            event.set("observer.type", json!("proxy"))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("web")]))?;

            event.set("event.type", Value::Array(vec![json!("access")]))?;

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?P<_tmp_time_s>(?:[0-9]+))\\.(?P<_tmp_time_ms>(?:[0-9]+))%{SPACE}(?P<_tmp_elapsed>(?:[0-9]+)) %{NOTSPACE:_tmp.source_ip} (?P<_tmp_code>(?:[^/]+))/(?P<_tmp_status>(?:[0-9]+)) (?P<_tmp_destination_bytes>(?:[0-9]+)) %{NOTSPACE:_tmp.method} %{NOTSPACE:_tmp.url} %{NOTSPACE:_tmp.user_name} (?P<_tmp_peer_status>(?:[^/]+))/%{NOTSPACE:_tmp.peer_host} %{NOTSPACE:_tmp.content_type}$
                // Grok pattern: ^(?P<_tmp_time_s>(?:[0-9]+))\\.(?P<_tmp_time_ms>(?:[0-9]+))%{SPACE}(?P<_tmp_elapsed>(?:[0-9]+)) %{NOTSPACE:_tmp.source_ip} (?P<_tmp_source_port>(?:[0-9]+)) (?P<_tmp_code>(?:[^/]+))/(?P<_tmp_status>(?:[0-9]+)) %{NOTSPACE:_tmp.reply_size} %{NOTSPACE:_tmp.request_size} %{NOTSPACE:_tmp.reply_header_size} %{NOTSPACE:_tmp.request_header_size} %{NOTSPACE:_tmp.reply_body_size} %{NOTSPACE:_tmp.method} %{NOTSPACE:_tmp.url} %{NOTSPACE:_tmp.http_version} %{NOTSPACE:_tmp.user_name} (?P<_tmp_peer_status>(?:[^/]+))/%{NOTSPACE:_tmp.peer_host} %{NOTSPACE:_tmp.destination_port} %{NOTSPACE:_tmp.content_type} %{NOTSPACE:_tmp.err_code} %{NOTSPACE:_tmp.err_detail} \"%{DATA:_tmp.referer}\" \"%{DATA:_tmp.user_agent}\" \"%{DATA:_tmp.host_header}\" \"%{DATA:_tmp.xff}\" \"%{DATA:_tmp.sni}\" %{GREEDYDATA:_tmp.note}$
                // Grok pattern: ^(?P<_tmp_time_s>(?:[0-9]+))\\.(?P<_tmp_time_ms>(?:[0-9]+))%{SPACE}(?P<_tmp_elapsed>(?:[0-9]+)) %{NOTSPACE:_tmp.source_ip} (?P<_tmp_source_port>(?:[0-9]+)) (?P<_tmp_code>(?:[^/]+))/(?P<_tmp_status>(?:[0-9]+)) %{NOTSPACE:_tmp.reply_size} %{NOTSPACE:_tmp.request_size} %{NOTSPACE:_tmp.reply_header_size} %{NOTSPACE:_tmp.request_header_size} %{NOTSPACE:_tmp.reply_body_size} %{NOTSPACE:_tmp.method} %{NOTSPACE:_tmp.url} %{NOTSPACE:_tmp.http_version} %{NOTSPACE:_tmp.user_name} (?P<_tmp_peer_status>(?:[^/]+))/%{NOTSPACE:_tmp.peer_host} %{NOTSPACE:_tmp.destination_port} %{NOTSPACE:_tmp.content_type} %{NOTSPACE:_tmp.err_code} %{NOTSPACE:_tmp.err_detail} \"%{DATA:_tmp.referer}\" \"%{DATA:_tmp.user_agent}\" \"%{DATA:_tmp.host_header}\" \"%{DATA:_tmp.xff}\" %{GREEDYDATA:_tmp.note}$
                let _ = extract_first_match(
                    &[
                        cached_grok_mapped!(
                            "^(?P<_tmp_time_s>(?:[0-9]+))\\.(?P<_tmp_time_ms>(?:[0-9]+))%{SPACE}(?P<_tmp_elapsed>(?:[0-9]+)) %{NOTSPACE:_tmp.source_ip} (?P<_tmp_code>(?:[^/]+))/(?P<_tmp_status>(?:[0-9]+)) (?P<_tmp_destination_bytes>(?:[0-9]+)) %{NOTSPACE:_tmp.method} %{NOTSPACE:_tmp.url} %{NOTSPACE:_tmp.user_name} (?P<_tmp_peer_status>(?:[^/]+))/%{NOTSPACE:_tmp.peer_host} %{NOTSPACE:_tmp.content_type}$",
                            [
                                ("_tmp_time_s", "_tmp.time_s"),
                                ("_tmp_time_ms", "_tmp.time_ms"),
                                ("_tmp_elapsed", "_tmp.elapsed"),
                                ("_tmp_code", "_tmp.code"),
                                ("_tmp_status", "_tmp.status"),
                                ("_tmp_destination_bytes", "_tmp.destination_bytes"),
                                ("_tmp_peer_status", "_tmp.peer_status")
                            ]
                        ),
                        cached_grok_mapped!(
                            "^(?P<_tmp_time_s>(?:[0-9]+))\\.(?P<_tmp_time_ms>(?:[0-9]+))%{SPACE}(?P<_tmp_elapsed>(?:[0-9]+)) %{NOTSPACE:_tmp.source_ip} (?P<_tmp_source_port>(?:[0-9]+)) (?P<_tmp_code>(?:[^/]+))/(?P<_tmp_status>(?:[0-9]+)) %{NOTSPACE:_tmp.reply_size} %{NOTSPACE:_tmp.request_size} %{NOTSPACE:_tmp.reply_header_size} %{NOTSPACE:_tmp.request_header_size} %{NOTSPACE:_tmp.reply_body_size} %{NOTSPACE:_tmp.method} %{NOTSPACE:_tmp.url} %{NOTSPACE:_tmp.http_version} %{NOTSPACE:_tmp.user_name} (?P<_tmp_peer_status>(?:[^/]+))/%{NOTSPACE:_tmp.peer_host} %{NOTSPACE:_tmp.destination_port} %{NOTSPACE:_tmp.content_type} %{NOTSPACE:_tmp.err_code} %{NOTSPACE:_tmp.err_detail} \"%{DATA:_tmp.referer}\" \"%{DATA:_tmp.user_agent}\" \"%{DATA:_tmp.host_header}\" \"%{DATA:_tmp.xff}\" \"%{DATA:_tmp.sni}\" %{GREEDYDATA:_tmp.note}$",
                            [
                                ("_tmp_time_s", "_tmp.time_s"),
                                ("_tmp_time_ms", "_tmp.time_ms"),
                                ("_tmp_elapsed", "_tmp.elapsed"),
                                ("_tmp_source_port", "_tmp.source_port"),
                                ("_tmp_code", "_tmp.code"),
                                ("_tmp_status", "_tmp.status"),
                                ("_tmp_peer_status", "_tmp.peer_status")
                            ]
                        ),
                        cached_grok_mapped!(
                            "^(?P<_tmp_time_s>(?:[0-9]+))\\.(?P<_tmp_time_ms>(?:[0-9]+))%{SPACE}(?P<_tmp_elapsed>(?:[0-9]+)) %{NOTSPACE:_tmp.source_ip} (?P<_tmp_source_port>(?:[0-9]+)) (?P<_tmp_code>(?:[^/]+))/(?P<_tmp_status>(?:[0-9]+)) %{NOTSPACE:_tmp.reply_size} %{NOTSPACE:_tmp.request_size} %{NOTSPACE:_tmp.reply_header_size} %{NOTSPACE:_tmp.request_header_size} %{NOTSPACE:_tmp.reply_body_size} %{NOTSPACE:_tmp.method} %{NOTSPACE:_tmp.url} %{NOTSPACE:_tmp.http_version} %{NOTSPACE:_tmp.user_name} (?P<_tmp_peer_status>(?:[^/]+))/%{NOTSPACE:_tmp.peer_host} %{NOTSPACE:_tmp.destination_port} %{NOTSPACE:_tmp.content_type} %{NOTSPACE:_tmp.err_code} %{NOTSPACE:_tmp.err_detail} \"%{DATA:_tmp.referer}\" \"%{DATA:_tmp.user_agent}\" \"%{DATA:_tmp.host_header}\" \"%{DATA:_tmp.xff}\" %{GREEDYDATA:_tmp.note}$",
                            [
                                ("_tmp_time_s", "_tmp.time_s"),
                                ("_tmp_time_ms", "_tmp.time_ms"),
                                ("_tmp_elapsed", "_tmp.elapsed"),
                                ("_tmp_source_port", "_tmp.source_port"),
                                ("_tmp_code", "_tmp.code"),
                                ("_tmp_status", "_tmp.status"),
                                ("_tmp_peer_status", "_tmp.peer_status")
                            ]
                        ),
                    ],
                    &input,
                    event,
                )?;
            }

            // SKIPPED: condition not transpiled: ctx._tmp?.values() != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Painless script
                // Source: ctx._tmp?.values().removeIf(value -> value == \"-\");
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx._tmp?.values().removeIf(value -> value == \"-\");"#),
                )?;
            }

            let _cond = {
                event.get("_tmp.time_s").is_some_and(|v| v.is_number())
                    && event.get("_tmp.time_ms").is_some_and(|v| v.is_number())
            };
            if _cond {
                // Painless script
                // Source: ctx[\"@timestamp\"] = new Date(ctx._tmp.time_s * 1000 + ctx._tmp.time_ms);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx[\"@timestamp\"] = new Date(ctx._tmp.time_s * 1000 + ctx._tmp.time_ms);"#
                    ),
                )?;
            }

            let _cond = { event.get("_tmp.elapsed").is_some_and(|v| v.is_number()) };
            if _cond {
                // Painless script
                // Source: ctx.event[\"duration\"] = ctx._tmp.elapsed * 1000000;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.event[\"duration\"] = ctx._tmp.elapsed * 1000000;"#),
                )?;
            }

            if event.has_value("_tmp.user_name") {
                event.rename("_tmp.user_name", "source.user.name")?;
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("_tmp.source_ip") {
                event.rename("_tmp.source_ip", "source.address")?;
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

            if event.has_value("_tmp.source_port") {
                event.rename("_tmp.source_port", "source.port")?;
            }

            if event.has_value("_tmp.destination_bytes") {
                event.rename("_tmp.destination_bytes", "destination.bytes")?;
            }

            if event.has_value("_tmp.reply_size") {
                if let Some(val) = event.get("_tmp.reply_size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reply_size".into(),
                            message,
                        }
                    })?;
                    event.set("destination.bytes", converted)?;
                }
            }

            if event.has_value("_tmp.request_size") {
                if let Some(val) = event.get("_tmp.request_size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.request_size".into(),
                            message,
                        }
                    })?;
                    event.set("source.bytes", converted)?;
                }
            }

            if event.has_value("_tmp.reply_header_size") {
                if let Some(val) = event.get("_tmp.reply_header_size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reply_header_size".into(),
                            message,
                        }
                    })?;
                    event.set("squid.reply_header_size", converted)?;
                }
            }

            if event.has_value("_tmp.request_header_size") {
                if let Some(val) = event.get("_tmp.request_header_size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.request_header_size".into(),
                            message,
                        }
                    })?;
                    event.set("squid.request_header_size", converted)?;
                }
            }

            if event.has_value("_tmp.reply_body_size") {
                if let Some(val) = event.get("_tmp.reply_body_size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.reply_body_size".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.body.bytes", converted)?;
                }
            }

            if event.has_value("_tmp.method") {
                event.rename("_tmp.method", "http.request.method")?;
            }

            if event.has_value("_tmp.http_version") {
                event.rename("_tmp.http_version", "http.version")?;
            }

            let _cond = { event.get_str("http.request.method") != Some("CONNECT") };
            if _cond {
                if event.has_value("_tmp.url") {
                    uri_parts(event, "_tmp.url", "url", true, false)?;
                }
            }

            let _cond = {
                event.has_value("_tmp.url")
                    && event.get_str("http.request.method") == Some("CONNECT")
            };
            if _cond {
                event.rename("_tmp.url", "url.original")?;
            }

            let _cond = {
                event.get("_tmp.status").is_some_and(|v| v.is_number())
                    && event.get_i64("_tmp.status").is_some_and(|n| n < 400)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get("_tmp.status").is_some_and(|v| v.is_number())
                    && event.get_i64("_tmp.status").is_some_and(|n| n >= 400)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has_value("_tmp.status") {
                event.rename("_tmp.status", "squid.status_code")?;
            }

            if event.has_value("_tmp.code") {
                event.rename("_tmp.code", "squid.result_code")?;
            }

            if event.has_value("_tmp.peer_status") {
                event.rename("_tmp.peer_status", "squid.peer_status")?;
            }

            if event.has_value("_tmp.content_type") {
                event.rename("_tmp.content_type", "squid.content_type")?;
            }

            if event.has_value("_tmp.destination_port") {
                if let Some(val) = event.get("_tmp.destination_port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.destination_port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            if event.has_value("_tmp.err_code") {
                event.rename("_tmp.err_code", "error.code")?;
            }

            if event.has_value("_tmp.err_detail") {
                event.rename("_tmp.err_detail", "squid.error_detail")?;
            }

            if event.has_value("_tmp.referer") {
                event.rename("_tmp.referer", "http.request.referrer")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_tmp.user_agent") {
                    if let Some(ua_str) = event.get_string("_tmp.user_agent") {
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
                Ok(())
            })();

            if event.has_value("_tmp.xff") {
                event.rename("_tmp.xff", "network.forwarded_ip")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("network.forwarded_ip") {
                    if let Some(val) = event.get("network.forwarded_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "network.forwarded_ip".into(),
                                message,
                            }
                        })?;
                        event.set("network.forwarded_ip", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has_value("_tmp.sni") {
                event.rename("_tmp.sni", "tls.client.server_name")?;
            }

            if event.has_value("_tmp.note") {
                event.rename("_tmp.note", "squid.note")?;
            }

            if event.has_value("_tmp.host_header") {
                event.rename("_tmp.host_header", "squid.host_header")?;
            }

            if event.has_value("_tmp.peer_host") {
                event.rename("_tmp.peer_host", "destination.address")?;
            }

            let _cond = { !event.has_value("destination.address") };
            if _cond {
                if let Some(v) = event
                    .get("url.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.address", v)?;
                }
            }

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

            if event.has_value("url.domain") {
                if let Some(domain_str) = event.get_string("url.domain") {
                    let domain = domain_str.to_string();
                    event.set("url.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        if let Some(registered) = rd.registered_domain {
                            event.set("url.registered_domain", json!(registered))?;
                        }
                        event.set("url.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("url.subdomain", json!(sub))?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("squid.host_header") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("squid.host_header")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("tls.client.server_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("tls.client.server_name")
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

            let _cond = { event.has_value("network.forwarded_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("network.forwarded_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("_tmp");

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
                event.remove("_tmp");
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
