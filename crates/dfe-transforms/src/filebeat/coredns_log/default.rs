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
                if let Some(v) = event
                    .get("message")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.original", v)?;
                }
            }

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            // SKIPPED: condition not transpiled: ctx.event.original.charAt(0) == (char)("{")
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Begin nested pipeline: "json"
                parse_json_field(event, "message", "json")?;
                if let Some(v) = event
                    .get("json.message")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
                if event.has_value("json.kubernetes") {
                    event.rename("json.kubernetes", "kubernetes")?;
                }
                let _cond = { event.has_value("json.time") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                event.remove("json");
                // End nested pipeline: "json"
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("protocol"))?;

            let _cond = { !(event.get_str("message").is_some_and(|s| s.starts_with("["))) };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if event.has_value("message") {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: \\[%{LOGLEVEL:log.level}\\] \\[?%{IP:source.address}\\]?:%{POSINT:source.port:long} - %{POSINT:dns.id} \"%{WORD:dns.question.type} %{WORD:dns.question.class} %{IPORHOST:dns.question.name}\\. %{WORD:network.transport} %{POSINT:source.bytes:long} %{WORD:coredns.log.dnssec_ok:boolean} %{POSINT:coredns.log.buffer_size:long}\" %{WORD:dns.response_code} %{NOTSPACE:dns.header_flags} %{POSINT:destination.bytes:long} %{NOTSPACE:event.duration}s( \"%{NONNEGINT:dns.op_code}\")?
                    // Grok pattern: \\[%{LOGLEVEL:log.level}\\] %{DATA:log.logger}: (%{WORD:dns.response_code}|%{NONNEGINT:dns.response_code:long}) %{IPORHOST:dns.question.name}\\. %{WORD:dns.question.type}: %{GREEDYDATA:coredns.log.error.message}
                    // Grok pattern: \\[%{LOGLEVEL:log.level}\\] %{DATA:log.logger}: %{GREEDYDATA:coredns.log.error.message}
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "\\[%{LOGLEVEL:log.level}\\] \\[?%{IP:source.address}\\]?:%{POSINT:source.port:long} - %{POSINT:dns.id} \"%{WORD:dns.question.type} %{WORD:dns.question.class} %{IPORHOST:dns.question.name}\\. %{WORD:network.transport} %{POSINT:source.bytes:long} %{WORD:coredns.log.dnssec_ok:boolean} %{POSINT:coredns.log.buffer_size:long}\" %{WORD:dns.response_code} %{NOTSPACE:dns.header_flags} %{POSINT:destination.bytes:long} %{NOTSPACE:event.duration}s( \"%{NONNEGINT:dns.op_code}\")?"
                            ),
                            cached_grok!(
                                "\\[%{LOGLEVEL:log.level}\\] %{DATA:log.logger}: (%{WORD:dns.response_code}|%{NONNEGINT:dns.response_code:long}) %{IPORHOST:dns.question.name}\\. %{WORD:dns.question.type}: %{GREEDYDATA:coredns.log.error.message}"
                            ),
                            cached_grok!(
                                "\\[%{LOGLEVEL:log.level}\\] %{DATA:log.logger}: %{GREEDYDATA:coredns.log.error.message}"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            if event.has_value("log.level") {
                map_strings(event, "log.level", "log.level", str::to_lowercase)?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("network.transport")
                    && event.get_str("network.transport") == Some("tcp")
            };
            if _cond {
                event.set("network.iana_number", json!("6"))?;
            }

            let _cond = {
                event.has_value("network.transport")
                    && event.get_str("network.transport") == Some("udp")
            };
            if _cond {
                event.set("network.iana_number", json!("17"))?;
            }

            let _cond = {
                event.has_value("source.bytes")
                    && event.has_value("destination.bytes")
                    && !event.has_value("network.bytes")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            event.set("network.protocol", json!("dns"))?;

            let _cond = {
                event.has_value("dns.response_code")
                    && event
                        .get("dns.response_code")
                        .is_some_and(|v| v.is_number())
            };
            if _cond {
                // Painless script
                // Source: def response_code = ctx.dns.response_code;\nctx.dns.response_code = params.codes[response_code];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def response_code = ctx.dns.response_code;\nctx.dns.response_code = params.codes[response_code];\n"#
                    ),
                    cached_params!(
                        "{\"codes\":[\"NOERROR\",\"FORMERR\",\"SERVFAIL\",\"NXDOMAIN\",\"NOTIMP\",\"REFUSED\",\"YXDOMAIN\",\"XRRSET\",\"NOTAUTH\",\"NOTZONE\"]}"
                    ),
                )?;
            }

            if event.has_value("dns.question.name") {
                if let Some(domain_str) = event.get_string("dns.question.name") {
                    let domain = domain_str.to_string();
                    event.set("dns.question.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        if let Some(registered) = rd.registered_domain {
                            event.set("dns.question.registered_domain", json!(registered))?;
                        }
                        event.set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("dns.question.subdomain", json!(sub))?;
                        }
                    }
                }
            }

            event.remove("dns.question.domain");

            if event.has_value("dns.header_flags") {
                map_strings(
                    event,
                    "dns.header_flags",
                    "dns.header_flags",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("dns.header_flags") {
                if let Some(s) = event.get_string("dns.header_flags") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("dns.header_flags", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("dns.header_flags")
                    && event.get("dns.header_flags").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: for (int i=0; i<ctx.dns.header_flags.length; i++) {\n  if (ctx.dns.header_flags[i] == 'QR') {\n    ctx.dns.header_flags.remove(i);\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"for (int i=0; i<ctx.dns.header_flags.length; i++) {\n  if (ctx.dns.header_flags[i] == 'QR') {\n    ctx.dns.header_flags.remove(i);\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("coredns.log.dnssec_ok")
                    && event.get_bool("coredns.log.dnssec_ok") == Some(true)
            };
            if _cond {
                event.append("dns.header_flags", json!("DO"))?;
            }

            let _cond = {
                event.has_value("coredns.log.dnssec_ok")
                    && event.get_bool("coredns.log.dnssec_ok") == Some(true)
            };
            if _cond {
                if event.remove("dns.header_flags").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "dns.header_flags".into(),
                    });
                }
            }

            let _cond = {
                event.has_value("dns.response_code")
                    && event.get_str("dns.response_code") == Some("NOERROR")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("dns.response_code")
                    && event.get_str("dns.response_code") != Some("NOERROR")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.has_value("event.duration") };
            if _cond {
                // Painless script
                // Source: double f = Double.parseDouble(ctx.event.duration); ctx.event.duration = f * params.S_TO_NS;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"double f = Double.parseDouble(ctx.event.duration); ctx.event.duration = f * params.S_TO_NS;"#
                    ),
                    cached_params!("{\"S_TO_NS\":1000000000}"),
                )?;
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

            let _cond = {
                event.has_value("dns.question.name")
                    && event.get_str("dns.question.type") != Some("PTR")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("dns.question.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("dns.question.name")
                    && event.get_str("dns.question.type") == Some("PTR")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("dns.question.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("message");
            event.remove("_tmp");

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                ),
            )?;

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
