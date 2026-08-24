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

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?:(?:(?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) (?:%{NOTSPACE:aws.elb.name}) (?:%{IP:source.address}:%{POSINT:source.port:long}) (?:(?:-|%{IP:aws.elb.backend.ip}:%{POSINT:aws.elb.backend.port})) (?:(?:-1|%{NUMBER:aws.elb.request_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.backend_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.response_processing_time.sec:float}))) %{NUMBER:http.response.status_code:long} (?:-|%{NUMBER:aws.elb.backend.http.response.status_code:long}) %{NUMBER:http.request.body.bytes:long} %{NUMBER:http.response.body.bytes:long} \\\"(?:-|%{WORD:http.request.method}) (?:-|%{DATA:_tmp.uri_orig})(?: (?:-|HTTP/%{NOTSPACE:http.version}))?\\\" \\\"%{DATA:_tmp.user_agent}\\\" (?:(?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NOTSPACE:aws.elb.ssl_protocol})))
                // Grok pattern: (?:(?:(?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) (?:%{NOTSPACE:aws.elb.name}) (?:%{IP:source.address}:%{POSINT:source.port:long}) (?:(?:-|%{IP:aws.elb.backend.ip}:%{POSINT:aws.elb.backend.port})) (?:(?:-1|%{NUMBER:aws.elb.request_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.backend_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.response_processing_time.sec:float}))) - - %{NUMBER:source.bytes:long} %{NUMBER:destination.bytes:long} \\\"- - - \\\" \\\"-\\\" (?:(?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NOTSPACE:aws.elb.ssl_protocol})))
                // Grok pattern: (?:%{WORD:aws.elb.type}) (?:(?:(?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) (?:%{NOTSPACE:aws.elb.name}) (?:%{IP:source.address}:%{POSINT:source.port:long}) (?:(?:-|%{IP:aws.elb.backend.ip}:%{POSINT:aws.elb.backend.port})) (?:(?:-1|%{NUMBER:aws.elb.request_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.backend_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.response_processing_time.sec:float}))) %{NUMBER:http.response.status_code:long} (?:-|%{NUMBER:aws.elb.backend.http.response.status_code:long}) %{NUMBER:http.request.body.bytes:long} %{NUMBER:http.response.body.bytes:long} \\\"(?:-|%{WORD:http.request.method}) (?:-|%{DATA:_tmp.uri_orig})(?: (?:-|HTTP/%{NOTSPACE:http.version}))?\\\" \\\"%{DATA:_tmp.user_agent}\\\" (?:(?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NOTSPACE:aws.elb.ssl_protocol}))) %{NOTSPACE:aws.elb.target_group.arn} \\\"%{DATA:aws.elb.trace_id}\\\" \\\"(?:-|%{DATA:destination.domain})\\\" \\\"(?:-|%{DATA:aws.elb.chosen_cert.arn})\\\" (?:-1|%{NUMBER:aws.elb.matched_rule_priority}) %{TIMESTAMP_ISO8601:event.start} \\\"(?:-|%{DATA:_tmp.actions_executed})\\\" \\\"(?:-|%{DATA:aws.elb.redirect_url})\\\" \\\"(?:-|%{DATA:aws.elb.error.reason})\\\"( \\\"(?:-|%{DATA:_tmp.target_port})\\\")?( \\\"(?:-|%{DATA:_tmp.target_status_code})\\\")?( \\\"(?:-|%{DATA:aws.elb.classification})\\\")?( \\\"(?:-|%{DATA:aws.elb.classification_reason})\\\")?
                // Grok pattern: (?:%{WORD:aws.elb.type}) (?:%{NOTSPACE}) (?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) (?:%{NOTSPACE:aws.elb.name}) %{NOTSPACE:aws.elb.listener} (?:%{IP:source.address}:%{POSINT:source.port:long}) (?:(?:-|%{IP:aws.elb.backend.ip}:%{POSINT:aws.elb.backend.port})) (?:-|%{NUMBER:aws.elb.connection_time.ms:float}) (?:-|%{NUMBER:aws.elb.tls_handshake_time.ms:float}) %{NUMBER:source.bytes:long} %{NUMBER:destination.bytes:long} (?:-|%{NUMBER:aws.elb.incoming_tls_alert}) (?:-|%{NOTSPACE:aws.elb.chosen_cert.arn}) (?:-|%{NOTSPACE:aws.elb.chosen_cert.serial}) (?:(?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NOTSPACE:aws.elb.ssl_protocol})) (?:-|%{NOTSPACE:aws.elb.ssl_named_group}) (?:-|%{NOTSPACE:destination.domain}) (?:-|%{NOTSPACE:aws.elb.alpn_fe_protocol}) (?:-|%{NOTSPACE:aws.elb.alpn_be_protocol}) (?:-|\\\\?\\\"%{DATA:aws.elb.alpn_client_preference_list}\\\\?\\\") (?:%{TIMESTAMP_ISO8601:aws.elb.tls_connection_creation_time_str}|-)
                // Grok pattern: (?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) %{IP:client.ip} %{NUMBER:client.port:long} %{NOTSPACE:aws.elb.listener} (?:-|%{NOTSPACE:aws.elb.ssl_protocol}) (?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NUMBER:aws.elb.tls_handshake_latency:float}) \"%{DATA:aws.elb.leaf_client_cert_subject}\" (?:-|NotBefore=%{TIMESTAMP_ISO8601:aws.elb.leaf_client_cert_not_before_str};NotAfter=%{TIMESTAMP_ISO8601:aws.elb.leaf_client_cert_not_after_str}) (?:-|%{NOTSPACE:aws.elb.leaf_client_cert_serial_number}) (%{WORD:aws.elb.tls_verify_status})?(?::%{NOTSPACE:aws.elb.tls_error_code})?
                let _ = extract_first_match(
                    &[
                        cached_grok!(
                            "(?:(?:(?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) (?:%{NOTSPACE:aws.elb.name}) (?:%{IP:source.address}:%{POSINT:source.port:long}) (?:(?:-|%{IP:aws.elb.backend.ip}:%{POSINT:aws.elb.backend.port})) (?:(?:-1|%{NUMBER:aws.elb.request_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.backend_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.response_processing_time.sec:float}))) %{NUMBER:http.response.status_code:long} (?:-|%{NUMBER:aws.elb.backend.http.response.status_code:long}) %{NUMBER:http.request.body.bytes:long} %{NUMBER:http.response.body.bytes:long} \\\"(?:-|%{WORD:http.request.method}) (?:-|%{DATA:_tmp.uri_orig})(?: (?:-|HTTP/%{NOTSPACE:http.version}))?\\\" \\\"%{DATA:_tmp.user_agent}\\\" (?:(?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NOTSPACE:aws.elb.ssl_protocol})))"
                        ),
                        cached_grok!(
                            "(?:(?:(?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) (?:%{NOTSPACE:aws.elb.name}) (?:%{IP:source.address}:%{POSINT:source.port:long}) (?:(?:-|%{IP:aws.elb.backend.ip}:%{POSINT:aws.elb.backend.port})) (?:(?:-1|%{NUMBER:aws.elb.request_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.backend_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.response_processing_time.sec:float}))) - - %{NUMBER:source.bytes:long} %{NUMBER:destination.bytes:long} \\\"- - - \\\" \\\"-\\\" (?:(?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NOTSPACE:aws.elb.ssl_protocol})))"
                        ),
                        cached_grok!(
                            "(?:%{WORD:aws.elb.type}) (?:(?:(?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) (?:%{NOTSPACE:aws.elb.name}) (?:%{IP:source.address}:%{POSINT:source.port:long}) (?:(?:-|%{IP:aws.elb.backend.ip}:%{POSINT:aws.elb.backend.port})) (?:(?:-1|%{NUMBER:aws.elb.request_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.backend_processing_time.sec:float}) (?:-1|%{NUMBER:aws.elb.response_processing_time.sec:float}))) %{NUMBER:http.response.status_code:long} (?:-|%{NUMBER:aws.elb.backend.http.response.status_code:long}) %{NUMBER:http.request.body.bytes:long} %{NUMBER:http.response.body.bytes:long} \\\"(?:-|%{WORD:http.request.method}) (?:-|%{DATA:_tmp.uri_orig})(?: (?:-|HTTP/%{NOTSPACE:http.version}))?\\\" \\\"%{DATA:_tmp.user_agent}\\\" (?:(?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NOTSPACE:aws.elb.ssl_protocol}))) %{NOTSPACE:aws.elb.target_group.arn} \\\"%{DATA:aws.elb.trace_id}\\\" \\\"(?:-|%{DATA:destination.domain})\\\" \\\"(?:-|%{DATA:aws.elb.chosen_cert.arn})\\\" (?:-1|%{NUMBER:aws.elb.matched_rule_priority}) %{TIMESTAMP_ISO8601:event.start} \\\"(?:-|%{DATA:_tmp.actions_executed})\\\" \\\"(?:-|%{DATA:aws.elb.redirect_url})\\\" \\\"(?:-|%{DATA:aws.elb.error.reason})\\\"( \\\"(?:-|%{DATA:_tmp.target_port})\\\")?( \\\"(?:-|%{DATA:_tmp.target_status_code})\\\")?( \\\"(?:-|%{DATA:aws.elb.classification})\\\")?( \\\"(?:-|%{DATA:aws.elb.classification_reason})\\\")?"
                        ),
                        cached_grok!(
                            "(?:%{WORD:aws.elb.type}) (?:%{NOTSPACE}) (?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) (?:%{NOTSPACE:aws.elb.name}) %{NOTSPACE:aws.elb.listener} (?:%{IP:source.address}:%{POSINT:source.port:long}) (?:(?:-|%{IP:aws.elb.backend.ip}:%{POSINT:aws.elb.backend.port})) (?:-|%{NUMBER:aws.elb.connection_time.ms:float}) (?:-|%{NUMBER:aws.elb.tls_handshake_time.ms:float}) %{NUMBER:source.bytes:long} %{NUMBER:destination.bytes:long} (?:-|%{NUMBER:aws.elb.incoming_tls_alert}) (?:-|%{NOTSPACE:aws.elb.chosen_cert.arn}) (?:-|%{NOTSPACE:aws.elb.chosen_cert.serial}) (?:(?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NOTSPACE:aws.elb.ssl_protocol})) (?:-|%{NOTSPACE:aws.elb.ssl_named_group}) (?:-|%{NOTSPACE:destination.domain}) (?:-|%{NOTSPACE:aws.elb.alpn_fe_protocol}) (?:-|%{NOTSPACE:aws.elb.alpn_be_protocol}) (?:-|\\\\?\\\"%{DATA:aws.elb.alpn_client_preference_list}\\\\?\\\") (?:%{TIMESTAMP_ISO8601:aws.elb.tls_connection_creation_time_str}|-)"
                        ),
                        cached_grok!(
                            "(?:%{TIMESTAMP_ISO8601:_tmp.timestamp}) %{IP:client.ip} %{NUMBER:client.port:long} %{NOTSPACE:aws.elb.listener} (?:-|%{NOTSPACE:aws.elb.ssl_protocol}) (?:-|%{NOTSPACE:aws.elb.ssl_cipher}) (?:-|%{NUMBER:aws.elb.tls_handshake_latency:float}) \"%{DATA:aws.elb.leaf_client_cert_subject}\" (?:-|NotBefore=%{TIMESTAMP_ISO8601:aws.elb.leaf_client_cert_not_before_str};NotAfter=%{TIMESTAMP_ISO8601:aws.elb.leaf_client_cert_not_after_str}) (?:-|%{NOTSPACE:aws.elb.leaf_client_cert_serial_number}) (%{WORD:aws.elb.tls_verify_status})?(?::%{NOTSPACE:aws.elb.tls_error_code})?"
                        ),
                    ],
                    &input,
                    event,
                )?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("cloud.provider", json!("aws"))?;

            let _cond = { event.has_value("http") };
            if _cond {
                event.set("aws.elb.protocol", json!("http"))?;
            }

            let _cond = { event.has_value("_tmp.uri_orig") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "_tmp.uri_orig", "url", true, false)?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_tmp.user_agent") };
            if _cond {
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
            }

            let _cond = { event.has_value("http") };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("web")]))?;
            }

            let _cond = { !event.has_value("http") };
            if _cond {
                event.set("aws.elb.protocol", json!("tcp"))?;
            }

            let _cond = { !event.has_value("http") };
            if _cond {
                event.set("event.category", Value::Array(vec![json!("network")]))?;
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

            let _cond = { event.has_value("aws.elb.trace_id") };
            if _cond {
                event.set(
                    "trace.id",
                    json!(
                        event
                            .get("aws.elb.trace_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("_tmp.actions_executed") {
                if let Some(s) = event.get_string("_tmp.actions_executed") {
                    let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    event.set("aws.elb.action_executed", Value::Array(parts))?;
                }
            }

            if event.has_value("_tmp.target_port") {
                if let Some(s) = event.get_string("_tmp.target_port") {
                    let parts: Vec<Value> = s.split(" ").map(|p| json!(p)).collect();
                    event.set("aws.elb.target_port", Value::Array(parts))?;
                }
            }

            if event.has_value("_tmp.target_status_code") {
                if let Some(s) = event.get_string("_tmp.target_status_code") {
                    let parts: Vec<Value> = s.split(" ").map(|p| json!(p)).collect();
                    event.set("aws.elb.target_status_code", Value::Array(parts))?;
                }
            }

            if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                    event.set("@timestamp", parsed)?;
                }
            }

            event.set(
                "event.end",
                json!(
                    event
                        .get("@timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

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
                if let Some(val) = event.get("source.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = { event.has_value("aws.elb.ssl_cipher") };
            if _cond {
                event.set(
                    "tls.cipher",
                    json!(
                        event
                            .get("aws.elb.ssl_cipher")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("aws.elb.ssl_protocol") };
            if _cond {
                // Painless script
                // Source: def parts = ctx.aws.elb.ssl_protocol.splitOnToken(\"v\"); if (parts.length != 2) {\n  return;\n} if (parts[1].contains(\".\")) {\n  ctx.tls.version = parts[1];\n} else {\n  ctx.tls.version = parts[1].substring(0,1) + \".\" + parts[1].substring(1);\n} ctx.tls.version_protocol = parts[0].toLowerCase();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def parts = ctx.aws.elb.ssl_protocol.splitOnToken(\"v\"); if (parts.length != 2) {\n  return;\n} if (parts[1].contains(\".\")) {\n  ctx.tls.version = parts[1];\n} else {\n  ctx.tls.version = parts[1].substring(0,1) + \".\" + parts[1].substring(1);\n} ctx.tls.version_protocol = parts[0].toLowerCase();"#
                    ),
                )?;
            }

            event.remove("_tmp");

            let _cond = {
                event.has_value("aws.elb.tls_connection_creation_time_str")
                    && event.get_str("aws.elb.tls_connection_creation_time_str") != Some("-")
                    && event.get_str("aws.elb.tls_connection_creation_time_str") != Some("")
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("aws.elb.tls_connection_creation_time_str")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("aws.elb.tls_connection_creation_time", parsed)?;
                    }
                }
            }

            event.remove("aws.elb.tls_connection_creation_time_str");

            let _cond = {
                event.has_value("aws.elb.leaf_client_cert_not_after_str")
                    && event.get_str("aws.elb.leaf_client_cert_not_after_str") != Some("-")
                    && event.get_str("aws.elb.leaf_client_cert_not_after_str") != Some("")
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("aws.elb.leaf_client_cert_not_after_str")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("aws.elb.leaf_client_cert_not_after", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("aws.elb.leaf_client_cert_not_before_str")
                    && event.get_str("aws.elb.leaf_client_cert_not_before_str") != Some("-")
                    && event.get_str("aws.elb.leaf_client_cert_not_before_str") != Some("")
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("aws.elb.leaf_client_cert_not_before_str")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("aws.elb.leaf_client_cert_not_before", parsed)?;
                    }
                }
            }

            event.remove("aws.elb.leaf_client_cert_not_after_str");
            event.remove("aws.elb.leaf_client_cert_not_before_str");

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
