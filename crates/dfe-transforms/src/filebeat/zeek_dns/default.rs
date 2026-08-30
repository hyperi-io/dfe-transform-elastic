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

            event.rename("_temp_", "zeek.dns")?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("connection"))?;

            event.append("event.type", json!("protocol"))?;

            event.append("event.type", json!("info"))?;

            event.set("network.protocol", json!("dns"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.dns", "id.orig_p")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.dns", "id.orig_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.dns", "id.resp_h")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                dot_expand(event, "zeek.dns", "id.resp_p")?;
                Ok(())
            })();

            if event.has_value("zeek.dns.id.orig_h") {
                event.rename("zeek.dns.id.orig_h", "source.address")?;
            }

            if event.has_value("zeek.dns.id.orig_p") {
                event.rename("zeek.dns.id.orig_p", "source.port")?;
            }

            if event.has_value("zeek.dns.id.resp_h") {
                event.rename("zeek.dns.id.resp_h", "destination.address")?;
            }

            if event.has_value("zeek.dns.id.resp_p") {
                event.rename("zeek.dns.id.resp_p", "destination.port")?;
            }

            if event.has_value("zeek.dns.uid") {
                event.rename("zeek.dns.uid", "zeek.session_id")?;
            }

            if event.has_value("zeek.dns.proto") {
                event.rename("zeek.dns.proto", "network.transport")?;
            }

            if let Some(v) = event
                .get("zeek.session_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("source.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event
                .get("destination.address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.get_bool("zeek.dns.AA") == Some(true) };
            if _cond {
                event.append("dns.header_flags", json!("AA"))?;
            }

            let _cond = { event.get_bool("zeek.dns.TC") == Some(true) };
            if _cond {
                event.append("dns.header_flags", json!("TC"))?;
            }

            let _cond = { event.get_bool("zeek.dns.RD") == Some(true) };
            if _cond {
                event.append("dns.header_flags", json!("RD"))?;
            }

            let _cond = { event.get_bool("zeek.dns.RA") == Some(true) };
            if _cond {
                event.append("dns.header_flags", json!("RA"))?;
            }

            let _cond = { event.get_i64("zeek.dns.qclass") == Some(1) };
            if _cond {
                event.set("dns.question.class", json!("IN"))?;
            }

            let _cond = { event.get_i64("zeek.dns.qclass") == Some(3) };
            if _cond {
                event.set("dns.question.class", json!("CH"))?;
            }

            let _cond = { event.get_i64("zeek.dns.qclass") == Some(4) };
            if _cond {
                event.set("dns.question.class", json!("HS"))?;
            }

            let _cond = { event.get_i64("zeek.dns.qclass") == Some(254) };
            if _cond {
                event.set("dns.question.class", json!("NONE"))?;
            }

            let _cond = { event.get_i64("zeek.dns.qclass") == Some(255) };
            if _cond {
                event.set("dns.question.class", json!("ANY"))?;
            }

            let _cond = { event.has_value("zeek.dns.rcode_name") };
            if _cond {
                event.set("dns.type", json!("answer"))?;
            }

            let _cond = { !event.has_value("dns.type") };
            if _cond {
                event.set("dns.type", json!("query"))?;
            }

            let _cond = { event.has_value("zeek.dns.rtt") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = ctx.zeek.dns.rtt * 1000000000L;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.event.duration = ctx.zeek.dns.rtt * 1000000000L;"#),
                )?;
            }

            let _cond = { event.has_value("zeek.dns.answers") && event.has_value("zeek.dns.TTLs") };
            if _cond {
                // Painless script
                // Source: def answers = ctx.zeek.dns.answers; def ttls = ctx.zeek.dns.TTLs; if (answers.isEmpty() || ttls.isEmpty() || answers.length != ttls.length) {\n  return;\n} def lst = new ArrayList(); for (def i = 0; i < answers.length; i++) {\n  lst.add([\n    \"data\": answers[i],\n    \"ttl\": (int)ttls[i]\n  ])\n} if (ctx.dns == null) {\n  ctx.dns = new HashMap();\n} ctx.dns.answers = lst;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def answers = ctx.zeek.dns.answers; def ttls = ctx.zeek.dns.TTLs; if (answers.isEmpty() || ttls.isEmpty() || answers.length != ttls.length) {\n  return;\n} def lst = new ArrayList(); for (def i = 0; i < answers.length; i++) {\n  lst.add([\n    \"data\": answers[i],\n    \"ttl\": (int)ttls[i]\n  ])\n} if (ctx.dns == null) {\n  ctx.dns = new HashMap();\n} ctx.dns.answers = lst;"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("dns.answers")
                    && event.get("dns.answers").is_some_and(|v| !match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                foreach_array(event, "dns.answers", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_ingest._value.data") {
                            if let Some(val) = event.get("_ingest._value.data") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.data".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.tmpip", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("dns.answers")
                    && event.get("dns.answers").is_some_and(|v| !match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: def answers = ctx.dns.answers; def iplist = new ArrayList(); for (def i = 0; i < ctx.dns.answers.length; i++) {\n  if (answers[i].containsKey(\"tmpip\")) {\n    iplist.add(answers[i].tmpip);\n    answers[i].remove(\"tmpip\");\n  }\n} ctx.dns.resolved_ip = iplist;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def answers = ctx.dns.answers; def iplist = new ArrayList(); for (def i = 0; i < ctx.dns.answers.length; i++) {\n  if (answers[i].containsKey(\"tmpip\")) {\n    iplist.add(answers[i].tmpip);\n    answers[i].remove(\"tmpip\");\n  }\n} ctx.dns.resolved_ip = iplist;"#
                    ),
                )?;
            }

            let _cond = { event.get_i64("dns.rcode") == Some(0) };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            if event.has_value("zeek.dns.trans_id") {
                if let Some(val) = event.get("zeek.dns.trans_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "zeek.dns.trans_id".into(),
                            message,
                        }
                    })?;
                    event.set("zeek.dns.trans_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("zeek.dns.trans_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.id", v)?;
            }

            if let Some(v) = event
                .get("zeek.dns.qtype_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.type", v)?;
            }

            if let Some(v) = event
                .get("zeek.dns.rcode_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.response_code", v)?;
            }

            if event.has_value("zeek.dns.query") {
                if let Some(domain_str) = event.get_string("zeek.dns.query") {
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

            if event.has_value("dns.question.domain") {
                event.rename("dns.question.domain", "dns.question.name")?;
            }

            if let Some(date_str) = event.get_as_string("zeek.dns.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.dns.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.dns.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.dns.ts".into(),
                });
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

            let _cond = { event.has_value("source.address") };
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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.remove("zeek.dns.Z");
            event.remove("zeek.dns.auth");
            event.remove("zeek.dns.addl");
            event.remove("zeek.dns.id");

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
