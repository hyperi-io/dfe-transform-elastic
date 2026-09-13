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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "_temp_")?;

            let _cond = { !event.has_value("_temp_.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.rename("_temp_", "zeek.kerberos")?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.append("event.category", json!("network"))?;
            event.append("event.category", json!("authentication"))?;

            event.append("event.type", json!("connection"))?;

            event.append("event.type", json!("protocol"))?;

            event.append("event.type", json!("access"))?;

            event.set("network.transport", json!("tcp"))?;

            event.set("network.protocol", json!("kerberos"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "id.orig_p")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "id.orig_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "id.resp_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "id.resp_p")?;
                Ok(())
            })();

            if event.has_value("zeek.kerberos.id.orig_h") {
                event.rename("zeek.kerberos.id.orig_h", "source.address")?;
            }

            if event.has_value("zeek.kerberos.id.orig_p") {
                event.rename("zeek.kerberos.id.orig_p", "source.port")?;
            }

            if event.has_value("zeek.kerberos.id.resp_h") {
                event.rename("zeek.kerberos.id.resp_h", "destination.address")?;
            }

            if event.has_value("zeek.kerberos.id.resp_p") {
                event.rename("zeek.kerberos.id.resp_p", "destination.port")?;
            }

            if event.has_value("zeek.kerberos.uid") {
                event.rename("zeek.kerberos.uid", "zeek.session_id")?;
            }

            let _cond = { event.has_value("zeek.session_id") };
            if _cond {
                if let Some(v) = event.get("zeek.session_id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("source.ip", v)?;
                }
            }

            let _cond = { event.has_value("source.address") };
            if _cond {
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("client.address", v)?;
                }
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("destination.ip", v)?;
                }
            }

            let _cond = { event.has_value("destination.address") };
            if _cond {
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("server.address", v)?;
                }
            }

            let _cond = { event.has_value("zeek.kerberos.request_type") };
            if _cond {
                if let Some(v) = event.get("zeek.kerberos.request_type").cloned() {
                    event.set("event.action", v)?;
                }
            }

            if event.has_value("zeek.kerberos.till") {
                event.rename("zeek.kerberos.till", "zeek.kerberos.valid.until")?;
            }

            if event.has_value("zeek.kerberos.from") {
                event.rename("zeek.kerberos.from", "zeek.kerberos.valid.from")?;
            }

            if event.has_value("zeek.kerberos.error_code") {
                event.rename("zeek.kerberos.error_code", "zeek.kerberos.error.code")?;
            }

            if event.has_value("zeek.kerberos.error_msg") {
                event.rename("zeek.kerberos.error_msg", "zeek.kerberos.error.msg")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "cert.client")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "cert.client_subject")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "cert.client_fuid")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "cert.server")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "cert.server_subject")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.kerberos", "cert.server_fuid")?;
                Ok(())
            })();

            if event.has_value("zeek.kerberos.cert.client") {
                event.rename(
                    "zeek.kerberos.cert.client",
                    "zeek.kerberos.cert.client.value",
                )?;
            }

            if event.has_value("zeek.kerberos.cert.client_subject") {
                event.rename(
                    "zeek.kerberos.cert.client_subject",
                    "zeek.kerberos.cert.client.subject",
                )?;
            }

            if event.has_value("zeek.kerberos.cert.client_fuid") {
                event.rename(
                    "zeek.kerberos.cert.client_fuid",
                    "zeek.kerberos.cert.client.fuid",
                )?;
            }

            if event.has_value("zeek.kerberos.cert.server") {
                event.rename(
                    "zeek.kerberos.cert.server",
                    "zeek.kerberos.cert.server.value",
                )?;
            }

            if event.has_value("zeek.kerberos.cert.server_subject") {
                event.rename(
                    "zeek.kerberos.cert.server_subject",
                    "zeek.kerberos.cert.server.subject",
                )?;
            }

            if event.has_value("zeek.kerberos.cert.server_fuid") {
                event.rename(
                    "zeek.kerberos.cert.server_fuid",
                    "zeek.kerberos.cert.server.fuid",
                )?;
            }

            if event.has_value("zeek.kerberos.auth_ticket") {
                event.rename("zeek.kerberos.auth_ticket", "zeek.kerberos.ticket.auth")?;
            }

            if event.has_value("zeek.kerberos.new_ticket") {
                event.rename("zeek.kerberos.new_ticket", "zeek.kerberos.ticket.new")?;
            }

            let _cond = {
                event.get("zeek.kerberos.client").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("/")),
                    serde_json::Value::String(s) => s.contains("/"),
                    _ => false,
                })
            };
            if _cond {
                if event.has_value("zeek.kerberos.client") {
                    if let Some(input) = event.get_string("zeek.kerberos.client") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("/") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("/") else {
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
                        } else {
                            return Err(TransformError::ParseError {
                                path: "zeek.kerberos.client".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
            }

            if let Some(date_str) = event.get_as_string("zeek.kerberos.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.kerberos.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.kerberos.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.kerberos.ts".into(),
                });
            }

            let _cond = {
                event.has_value("zeek.kerberos.valid.from")
                    && event.has_value("zeek.kerberos.valid.until")
            };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.zeek.kerberos.valid.days = Math.round( (ctx.zeek.kerberos.valid.until - ctx.zeek.kerberos.valid.from) / 86400 )
                scalar_expression(
                    event,
                    &ScalarExpression::new(
                        "zeek.kerberos.valid.days",
                        Expr::Round(Box::new(Expr::Binary(
                            Box::new(Expr::Binary(
                                Box::new(Expr::Field("zeek.kerberos.valid.until".into())),
                                Op::Sub,
                                Box::new(Expr::Field("zeek.kerberos.valid.from".into())),
                            )),
                            Op::Div,
                            Box::new(Expr::Int(86400)),
                        ))),
                    ),
                );
            }

            let _cond = { event.has_value("zeek.kerberos.valid.until") };
            if _cond {
                if let Some(date_str) = event.get_as_string("zeek.kerberos.valid.until") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("zeek.kerberos.valid.until", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zeek.kerberos.valid.until".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("zeek.kerberos.valid.from") };
            if _cond {
                if let Some(date_str) = event.get_as_string("zeek.kerberos.valid.from") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("zeek.kerberos.valid.from", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "zeek.kerberos.valid.from".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.get_bool("zeek.kerberos.success") == Some(true) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_bool("zeek.kerberos.success") == Some(false) };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
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

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("zeek.kerberos.cert.client.subject") {
                gsub_field(
                    event,
                    "zeek.kerberos.cert.client.subject",
                    "zeek.kerberos.cert.client.subject",
                    cached_regex!("\\\\,"),
                    "",
                )?;
            }

            if event.has_value("zeek.kerberos.cert.client.subject") {
                if let Some(kv_str) = event.get_string("zeek.kerberos.cert.client.subject") {
                    for pair in kv_str.split(",") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "zeek.kerberos.cert.client.subject".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(
                                    event,
                                    &format!("zeek.kerberos.cert.client.kv_sub.{}", key),
                                    value,
                                )?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.client.kv_sub.C")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.country",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.client.kv_sub.C")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.client.kv_sub.CN")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.client.kv_sub.CN")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.client.kv_sub.L")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.locality",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.client.kv_sub.L")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.client.kv_sub.O")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.organization",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.client.kv_sub.O")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.client.kv_sub.OU")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.organizational_unit",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.client.kv_sub.OU")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.client.kv_sub.ST")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.client.x509.subject.state_or_province",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.client.kv_sub.ST")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            event.remove("zeek.kerberos.cert.client.kv_sub");

            if event.has_value("zeek.kerberos.cert.server.subject") {
                gsub_field(
                    event,
                    "zeek.kerberos.cert.server.subject",
                    "zeek.kerberos.cert.server.subject",
                    cached_regex!("\\\\,"),
                    "",
                )?;
            }

            if event.has_value("zeek.kerberos.cert.server.subject") {
                if let Some(kv_str) = event.get_string("zeek.kerberos.cert.server.subject") {
                    for pair in kv_str.split(",") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "zeek.kerberos.cert.server.subject".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(
                                    event,
                                    &format!("zeek.kerberos.cert.server.kv_sub.{}", key),
                                    value,
                                )?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.server.kv_sub.C")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.country",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.server.kv_sub.C")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.server.kv_sub.CN")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.common_name",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.server.kv_sub.CN")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.server.kv_sub.L")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.locality",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.server.kv_sub.L")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.server.kv_sub.O")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.organization",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.server.kv_sub.O")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.server.kv_sub.OU")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.organizational_unit",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.server.kv_sub.OU")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event
                    .get("zeek.kerberos.cert.server.kv_sub.ST")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "tls.server.x509.subject.state_or_province",
                    Value::Array(vec![json!(
                        event
                            .get("zeek.kerberos.cert.server.kv_sub.ST")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            event.remove("zeek.kerberos.cert.server.kv_sub");

            // Community ID v1 hash
            if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                event.get_string("source.ip"),
                event.get_string("destination.ip"),
                event
                    .get_as_string("network.iana_number")
                    .or_else(|| event.get_as_string("network.transport")),
            ) {
                let icmp = matches!(
                    protocol.to_ascii_lowercase().as_str(),
                    "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                );
                let (src_field, dst_field) = if icmp {
                    ("icmp.type", "icmp.code")
                } else {
                    ("source.port", "destination.port")
                };
                let src_port = u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                let dst_port = u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                    Ok(cid) => event.set("network.community_id", cid)?,
                    Err(message) => {
                        return Err(TransformError::ParseError {
                            path: "network.community_id".into(),
                            message,
                        });
                    }
                }
            }

            event.remove("message");
            event.remove("json");
            event.remove("zeek.kerberos.id");

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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
