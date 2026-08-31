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

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "cif3")?;

            let _cond = { event.has_value("cif3.firsttime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cif3.firsttime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cif3.firsttime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date-indicator_first_seen",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("cif3.lasttime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cif3.lasttime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cif3.lasttime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date-indicator_last_seen",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
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

            let _cond = { event.has_value("cif3.indicator") };
            if _cond {
                if let Some(v) = event.get("cif3.indicator").cloned() {
                    event.set("threat.indicator.name", v)?;
                }
            }

            if event.has_value("cif3.reporttime") {
                event.rename("cif3.reporttime", "threat.indicator.modified_at")?;
            }

            if event.has_value("cif3.provider") {
                event.rename("cif3.provider", "threat.indicator.provider")?;
            }

            if event.has_value("cif3.reference") {
                event.rename("cif3.reference", "threat.indicator.reference")?;
            }

            if event.has_value("cif3.count") {
                event.rename("cif3.count", "threat.indicator.sightings")?;
            }

            let _cond = { event.get_str("cif3.description") != Some("") };
            if _cond {
                if event.has_value("cif3.description") {
                    event.rename("cif3.description", "threat.indicator.description")?;
                }
            }

            let _cond = { event.has_value("cif3.tlp") };
            if _cond {
                if event.has_value("cif3.tlp") {
                    map_strings(
                        event,
                        "cif3.tlp",
                        "threat.indicator.marking.tlp",
                        str::to_uppercase,
                    )?;
                }
            }

            // SKIPPED: condition not transpiled: ctx.cif3?.tags?.contains('ja3') != true && ['md5', 'sha1', 'sha256', 'sha512', 'ssdeep'].contains(ctx.cif3?.itype)
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.set("threat.indicator.type", json!("file"))?;
            }

            let _cond = {
                event.get_str("cif3.itype") == Some("md5")
                    && event.get("cif3.tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("ja3")),
                        serde_json::Value::String(s) => s.contains("ja3"),
                        _ => false,
                    })
            };
            if _cond {
                if event.has_value("cif3.indicator") {
                    event.rename("cif3.indicator", "threat.indicator.tls.client.ja3")?;
                }
            }

            let _cond = {
                event.get_str("cif3.itype") == Some("md5")
                    && event.get("cif3.tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("imphash"))
                        }
                        serde_json::Value::String(s) => s.contains("imphash"),
                        _ => false,
                    })
            };
            if _cond {
                if event.has_value("cif3.indicator") {
                    event.rename("cif3.indicator", "threat.indicator.file.pe.imphash")?;
                }
            }

            let _cond = { event.has_value("threat.indicator.file.pe.imphash") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("threat.indicator.file.hash.pe.imphash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("file") };
            if _cond {
                if event.has_value("cif3.indicator") {
                    event.rename("cif3.indicator", "_tmp.hashvalue")?;
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("file") };
            if _cond {
                event.set(
                    "threat.indicator.file.hash.{{{cif3.itype}}}",
                    json!(
                        event
                            .get("_tmp.hashvalue")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("file")
                    && !event.has_value("threat.indicator.file.pe.imphash")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append(
                        "related.hash",
                        json!(
                            event
                                .get("_tmp.hashvalue")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("cif3.itype") == Some("asn") };
            if _cond {
                event.set("threat.indicator.type", json!("autonomous-system"))?;
            }

            let _cond = { event.get_str("cif3.itype") == Some("asn") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cif3.indicator") {
                        // Grok pattern: as(?:%{INT:threat.indicator.as.number})
                        if !cached_grok!("as(?:%{INT:threat.indicator.as.number})")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("cif3.itype") == Some("ipv4") };
            if _cond {
                event.set("threat.indicator.type", json!("ipv4-addr"))?;
            }

            let _cond = { event.get_str("cif3.itype") == Some("ipv6") };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = {
                (event.has_value("cif3.indicator_ipv4_mask")
                    || event.has_value("cif3.indicator_ipv6_mask"))
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cif3.indicator") {
                    event.rename("cif3.indicator", "threat.indicator.network.cidr")?;
                }
            }

            let _cond = {
                !event.has_value("cif3.indicator_ipv4_mask")
                    && !event.has_value("cif3.indicator_ipv6_mask")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cif3.indicator") {
                    if let Some(val) = event.get("cif3.indicator") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "cif3.indicator".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.ip", converted)?;
                    }
                }
            }

            let _cond = { event.has_value("threat.indicator.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("threat.indicator.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cif3.cc")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cif3.cc") {
                    event.rename("cif3.cc", "threat.indicator.geo.country_iso_code")?;
                }
            }

            let _cond = {
                event.has_value("cif3.asn")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cif3.asn") {
                    event.rename("cif3.asn", "threat.indicator.as.number")?;
                }
            }

            let _cond = {
                event.has_value("cif3.asn_desc")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cif3.asn_desc") {
                    event.rename("cif3.asn_desc", "threat.indicator.as.organization.name")?;
                }
            }

            let _cond = {
                event.has_value("cif3.latitude")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cif3.latitude") {
                    event.rename("cif3.latitude", "threat.indicator.geo.location.lat")?;
                }
            }

            let _cond = {
                event.has_value("cif3.longitude")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cif3.longitude") {
                    event.rename("cif3.longitude", "threat.indicator.geo.location.lon")?;
                }
            }

            let _cond = {
                event.has_value("cif3.region")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cif3.region") {
                    event.rename("cif3.region", "threat.indicator.geo.region_name")?;
                }
            }

            let _cond = {
                event.has_value("cif3.timezone")
                    && ["ipv4-addr", "ipv6-addr"]
                        .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("cif3.timezone") {
                    event.rename("cif3.timezone", "threat.indicator.geo.timezone")?;
                }
            }

            let _cond = { event.get_str("cif3.itype") == Some("url") };
            if _cond {
                event.set("threat.indicator.type", json!("url"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("url") };
            if _cond {
                uri_parts(event, "cif3.indicator", "threat.indicator.url", true, true)?;
            }

            let _cond = { event.get_str("cif3.itype") == Some("url") };
            if _cond {
                let v = json!(
                    event
                        .get("threat.indicator.url.original")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("threat.indicator.url.full", v)?;
                }
            }

            let _cond = { event.get_str("cif3.itype") == Some("url") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cif3.indicator") {
                        // Grok pattern: %{URIPROTO:threat.indicator.url.scheme}://(?:%{IPV4:threat.indicator.ip}|\\[?%{IPV6:threat.indicator.ip}\\]?|%{HOSTNAME:threat.indicator.url.domain})(?::%{POSINT:threat.indicator.url.port})?(?:%{URIPATH:threat.indicator.url.path})?.*
                        if !cached_grok!("%{URIPROTO:threat.indicator.url.scheme}://(?:%{IPV4:threat.indicator.ip}|\\[?%{IPV6:threat.indicator.ip}\\]?|%{HOSTNAME:threat.indicator.url.domain})(?::%{POSINT:threat.indicator.url.port})?(?:%{URIPATH:threat.indicator.url.path})?.*").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("cif3.itype") == Some("email") };
            if _cond {
                event.set("threat.indicator.type", json!("email-addr"))?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("email-addr") };
            if _cond {
                if event.has_value("cif3.indicator") {
                    event.rename("cif3.indicator", "threat.indicator.email.address")?;
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("email-addr") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("threat.indicator.email.address") {
                        // Grok pattern: %{USERNAME}@%{GREEDYDATA:threat.indicator.url.domain}
                        if !cached_grok!("%{USERNAME}@%{GREEDYDATA:threat.indicator.url.domain}")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("cif3.itype") == Some("fqdn") };
            if _cond {
                event.set("threat.indicator.type", json!("domain-name"))?;
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("domain-name")
                    && !event.has_value("threat.indicator.url.domain")
            };
            if _cond {
                if event.has_value("cif3.indicator") {
                    event.rename("cif3.indicator", "threat.indicator.url.domain")?;
                }
            }

            let _cond = { event.has_value("threat.indicator.url.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    json!(
                        event
                            .get("threat.indicator.url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cif3.confidence") };
            if _cond {
                // Painless script
                // Source: def value = ctx.cif3.confidence; if (value < 0.0 || value > 10.0) {\n  ctx.threat.indicator.confidence = \"None\";\n  return;\n} if (value >= 0.0 && value < 3.0) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} if (value >= 3.0 && value < 7.0) {\n  ctx.threat.indicator.confidence = \"Med\";\n  return;\n} if (value >= 7.0 && value <= 10.0) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def value = ctx.cif3.confidence; if (value < 0.0 || value > 10.0) {\n  ctx.threat.indicator.confidence = \"None\";\n  return;\n} if (value >= 0.0 && value < 3.0) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} if (value >= 3.0 && value < 7.0) {\n  ctx.threat.indicator.confidence = \"Med\";\n  return;\n} if (value >= 7.0 && value <= 10.0) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("cif3.tags") };
            if _cond {
                if event.has_value("cif3.tags") {
                    foreach_array(event, "cif3.tags", |event| {
                        event.append_unique(
                            "tags",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.has_value("cif3.protocol") };
            if _cond {
                event.rename("cif3.protocol", "network.transport")?;
            }

            let _cond = { event.has_value("cif3.application") };
            if _cond {
                event.rename("cif3.application", "network.protocol")?;
            }

            let _cond = { event.has_value("cif3.port") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("cif3.port", "threat.indicator.port")?;
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                event.set("threat.indicator.type", json!("unknown"))?;
            }

            let _cond = {
                !event.has_value("cif3.deleted_at")
                    && event.has_value("_conf.ioc_expiration_duration")
                    && event.get_str("_conf.ioc_expiration_duration") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_lasttime_at; if (ctx.threat.indicator.last_seen != null) {\n    _tmp_lasttime_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n} else {\n    _tmp_lasttime_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n} if (ctx.threat.indicator.type == 'ipv4-addr' || ctx.threat.indicator.type == 'ipv6-addr') {\n    _tmp_deleted_at = _tmp_lasttime_at.plusDays(45L);\n    ctx.cif3.expiration_duration = \"45d\";\n} else if (ctx.threat.indicator.type == 'domain-name') {\n    _tmp_deleted_at = _tmp_lasttime_at.plusDays(90L);\n    ctx.cif3.expiration_duration = \"90d\";\n} else if (ctx.threat.indicator.type == 'url' || ctx.threat.indicator.type == 'file') {\n    _tmp_deleted_at = _tmp_lasttime_at.plusDays(365L);\n    ctx.cif3.expiration_duration = \"365d\";\n} else {\n    def dur = ctx._conf.ioc_expiration_duration;\n    ctx.cif3.expiration_duration = ctx._conf.ioc_expiration_duration;\n    if (dur instanceof String){\n        String time_unit = dur.substring(dur.length() -  1, dur.length());\n        String time_value = dur.substring(0, dur.length() - 1);\n        if (time_unit == 'd') {\n            _tmp_deleted_at = _tmp_lasttime_at.plusDays(Long.parseLong(time_value));\n        } else if (time_unit == 'h') {\n            _tmp_deleted_at = _tmp_lasttime_at.plusHours(Long.parseLong(time_value));\n        } else if (time_unit == 'm') {\n            _tmp_deleted_at = _tmp_lasttime_at.plusMinutes(Long.parseLong(time_value));\n        } else {\n            _tmp_deleted_at = _tmp_lasttime_at.plusDays(90L);\n            if (ctx.error == null) {\n            ctx.error = new HashMap();\n            }\n            if (ctx.error.message == null) {\n            ctx.error.message = new ArrayList();\n            }\n            ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n        }\n    }\n} ctx.cif3.deleted_at = _tmp_deleted_at;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_lasttime_at; if (ctx.threat.indicator.last_seen != null) {\n    _tmp_lasttime_at = ZonedDateTime.parse(ctx.threat.indicator.last_seen);\n} else {\n    _tmp_lasttime_at = ZonedDateTime.parse(ctx.threat.indicator.first_seen);\n} if (ctx.threat.indicator.type == 'ipv4-addr' || ctx.threat.indicator.type == 'ipv6-addr') {\n    _tmp_deleted_at = _tmp_lasttime_at.plusDays(45L);\n    ctx.cif3.expiration_duration = \"45d\";\n} else if (ctx.threat.indicator.type == 'domain-name') {\n    _tmp_deleted_at = _tmp_lasttime_at.plusDays(90L);\n    ctx.cif3.expiration_duration = \"90d\";\n} else if (ctx.threat.indicator.type == 'url' || ctx.threat.indicator.type == 'file') {\n    _tmp_deleted_at = _tmp_lasttime_at.plusDays(365L);\n    ctx.cif3.expiration_duration = \"365d\";\n} else {\n    def dur = ctx._conf.ioc_expiration_duration;\n    ctx.cif3.expiration_duration = ctx._conf.ioc_expiration_duration;\n    if (dur instanceof String){\n        String time_unit = dur.substring(dur.length() -  1, dur.length());\n        String time_value = dur.substring(0, dur.length() - 1);\n        if (time_unit == 'd') {\n            _tmp_deleted_at = _tmp_lasttime_at.plusDays(Long.parseLong(time_value));\n        } else if (time_unit == 'h') {\n            _tmp_deleted_at = _tmp_lasttime_at.plusHours(Long.parseLong(time_value));\n        } else if (time_unit == 'm') {\n            _tmp_deleted_at = _tmp_lasttime_at.plusMinutes(Long.parseLong(time_value));\n        } else {\n            _tmp_deleted_at = _tmp_lasttime_at.plusDays(90L);\n            if (ctx.error == null) {\n            ctx.error = new HashMap();\n            }\n            if (ctx.error.message == null) {\n            ctx.error.message = new ArrayList();\n            }\n            ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n        }\n    }\n} ctx.cif3.deleted_at = _tmp_deleted_at;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script-default-deleted_at",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("cif3.deleted_at") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("cif3.deleted_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cif3.deleted_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_deleted_at")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
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

            let _cond = { !event.has_value("cif3.expiration_duration") };
            if _cond {
                event.rename("_conf.ioc_expiration_duration", "cif3.expiration_duration")?;
            }

            let _cond = { event.has_value("cif3") };
            if _cond {
                // Painless script
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
            }

            let _cond = { event.get_str("cif3.rdata") == Some("") };
            if _cond {
                event.remove("cif3.rdata");
            }

            let _cond = { event.has_value("threat.indicator.type") };
            if _cond {
                event.remove("cif3.confidence");
                event.remove("cif3.indicator_ipv4");
                event.remove("cif3.indicator_ipv6");
                event.remove("cif3.group");
                event.remove("cif3.latitude");
                event.remove("cif3.longitude");
                event.remove("cif3.location");
                event.remove("cif3.city");
                event.remove("cif3.region");
                event.remove("cif3.tags");
                event.remove("cif3.tlp");
                event.remove("cif3.firsttime");
                event.remove("cif3.lasttime");
                event.remove("message");
                event.remove("_tmp");
                event.remove("_conf");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
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
