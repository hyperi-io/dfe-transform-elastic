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

            event.set("observer.vendor", json!("SonicWall"))?;

            event.set("observer.product", json!("SonicOS"))?;

            event.set("observer.type", json!("firewall"))?;

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                event.set(
                    "event.timezone",
                    json!(
                        event
                            .get("_conf.tz_offset")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^%{DATA:_temp_.header}%{SPACE}(?P<_temp__serialized_kv>(?:id=.*))
                    if !cached_grok_mapped!(
                        "^%{DATA:_temp_.header}%{SPACE}(?P<_temp__serialized_kv>(?:id=.*))",
                        [("_temp__serialized_kv", "_temp_.serialized_kv")]
                    )
                    .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "grok_event_original_882a4bdd",
                )?;
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: (format!(
                        "unable to extract key-values from log message: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ))
                    .to_string(),
                });
            }

            if let Some(input) = event.get_string("_temp_.header") {
                // Grok pattern: ^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP}?%{SPACE}%{DATA:_temp_.host}?$
                if !cached_grok!("^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP}?%{SPACE}%{DATA:_temp_.host}?$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
            }

            let _cond = { event.has_value("_temp_.host") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("_temp_.host") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "_temp_.host".into(),
                                message,
                            }
                        })?;
                        event.set("host.ip", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_string()) };
            if _cond {
                event.set(
                    "host.ip",
                    Value::Array(vec![json!(
                        event
                            .get("host.ip")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                !event.has_value("host.ip")
                    && event.has_value("_temp_.host")
                    && event.get_str("_temp_.host") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("_temp_.host", "host.name")?;
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("_temp_.serialized_kv") {
                    for pair in cached_regex!(" +(?=[a-zA-Z0-9_-]+=)")
                        .split(&kv_str)
                        .into_iter()
                    {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_temp_.serialized_kv".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let value = value.trim_matches(|c| "\"'".contains(c));
                            if !key.is_empty() {
                                kv_put(event, &format!("sonicwall.firewall.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "kv")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "kv__temp__serialized_kv_d4d79ba0",
                )?;
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: (format!(
                        "unable to process key-values from log message: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ))
                    .to_string(),
                });
            }

            let _cond = { event.has_value("sonicwall.firewall") };
            if _cond {
                // Painless script
                // Source: List sets = ctx._temp_.computeIfAbsent(\"sets\", k -> new ArrayList());\nList removes = ctx._temp_.computeIfAbsent(\"removes\", k -> new ArrayList());\nfor (def src_field : ctx.sonicwall.firewall.entrySet()) {\n  def key = src_field.getKey();\n  if (params[key] != null) {\n    boolean mapped = false;\n    for (def action : params[key]) {\n      def value = action.map == null? src_field.getValue() : action.map[src_field.getValue()];\n      if (value != null) {\n        sets.add([\n          \"target\": action.to,\n          \"value\": value\n        ]);\n      }\n    }\n    removes.add(key);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"List sets = ctx._temp_.computeIfAbsent(\"sets\", k -> new ArrayList());\nList removes = ctx._temp_.computeIfAbsent(\"removes\", k -> new ArrayList());\nfor (def src_field : ctx.sonicwall.firewall.entrySet()) {\n  def key = src_field.getKey();\n  if (params[key] != null) {\n    boolean mapped = false;\n    for (def action : params[key]) {\n      def value = action.map == null? src_field.getValue() : action.map[src_field.getValue()];\n      if (value != null) {\n        sets.add([\n          \"target\": action.to,\n          \"value\": value\n        ]);\n      }\n    }\n    removes.add(key);\n  }\n}\n"#
                    ),
                    cached_params!(
                        "{\"arg\":[{\"to\":\"url.path\"}],\"dpi\":[{\"to\":\"sonicwall.firewall.dpi\",\"map\":{\"0\":\"false\",\"1\":\"true\"}}],\"dstMac\":[{\"to\":\"destination.mac\"}],\"dstname\":[{\"to\":\"url.domain\"}],\"dstZone\":[{\"to\":\"observer.egress.zone\"}],\"fw\":[{\"to\":\"observer.hostname\"}],\"fw_action\":[{\"to\":\"event.action\",\"map\":{\"forward\":\"packet-forwarded\",\"drop\":\"packet-dropped\",\"mgmt\":\"packet-management\"}}],\"gcat\":[{\"to\":\"sonicwall.firewall.gcat\"},{\"to\":\"sonicwall.firewall.event_group_category\",\"map\":{\"1\":\"System\",\"2\":\"Log\",\"3\":\"Security Services\",\"4\":\"Users\",\"5\":\"Firewall Settings\",\"6\":\"Network\",\"7\":\"VPN\",\"8\":\"High Availability\",\"9\":\"WWAN Modem\",\"10\":\"Firewall\",\"11\":\"Wireless\",\"12\":\"VoIP\",\"13\":\"SSL VPN\",\"14\":\"Anti-Spam\",\"15\":\"WAN Acceleration\",\"16\":\"Object\",\"17\":\"SD-WAN\",\"18\":\"Multi-Instance\",\"19\":\"Unified Policy Engine\",\"20\":\"WireGuard\",\"21\":\"Cloud Secure Edge\"}}],\"id\":[{\"to\":\"observer.name\"}],\"m\":[{\"to\":\"event.code\"}],\"msg\":[{\"to\":\"message\"}],\"n\":[{\"to\":\"event.sequence\"}],\"natDst\":[{\"to\":\"_temp_.destination_nat_ip\"}],\"natDstV6\":[{\"to\":\"_temp_.destination_nat_ip\"}],\"natSrc\":[{\"to\":\"_temp_.source_nat_ip\"}],\"natSrcV6\":[{\"to\":\"_temp_.source_nat_ip\"}],\"op\":[{\"to\":\"http.request.method\",\"map\":{\"1\":\"GET\",\"2\":\"POST\",\"3\":\"HEAD\"}}],\"pri\":[{\"to\":\"event.severity\"},{\"to\":\"log.level\",\"map\":{\"0\":\"emergency\",\"1\":\"alert\",\"2\":\"critical\",\"3\":\"error\",\"4\":\"warning\",\"5\":\"notice\",\"6\":\"info\",\"7\":\"debug\"}}],\"proto\":[{\"to\":\"network.transport\"}],\"rcvd\":[{\"to\":\"destination.bytes\"}],\"rpkt\":[{\"to\":\"destination.packets\"}],\"rule\":[{\"to\":\"rule.id\"}],\"uuid\":[{\"to\":\"rule.uuid\"}],\"sent\":[{\"to\":\"source.bytes\"}],\"spkt\":[{\"to\":\"source.packets\"}],\"srcMac\":[{\"to\":\"source.mac\"}],\"srcZone\":[{\"to\":\"observer.ingress.zone\"}],\"sn\":[{\"to\":\"observer.serial_number\"}],\"time\":[{\"to\":\"@timestamp\"}],\"user\":[{\"to\":\"user.name\"}],\"usr\":[{\"to\":\"user.name\"}]}"
                    ),
                )?;
            }

            // Painless script
            // Source: List sets = ctx._temp_.computeIfAbsent(\"sets\", k -> new ArrayList());\nList removes = ctx._temp_.computeIfAbsent(\"removes\", k -> new ArrayList());\nfor (def field : params.entrySet()) {\n  String value = ctx.sonicwall.firewall[field.getKey()];\n  if (value == null) continue;\n  String[] parts = value.splitOnToken(\":\");\n  List mapping = field.getValue();\n  for ( int i = (int)Math.min(parts.length, mapping.size()) - 1\n      ; i>=0\n      ; i--) {\n    sets.add([\n      \"target\": mapping[i],\n      \"value\": parts[i]\n    ]);\n  }\n  removes.add(field.getKey());\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"List sets = ctx._temp_.computeIfAbsent(\"sets\", k -> new ArrayList());\nList removes = ctx._temp_.computeIfAbsent(\"removes\", k -> new ArrayList());\nfor (def field : params.entrySet()) {\n  String value = ctx.sonicwall.firewall[field.getKey()];\n  if (value == null) continue;\n  String[] parts = value.splitOnToken(\":\");\n  List mapping = field.getValue();\n  for ( int i = (int)Math.min(parts.length, mapping.size()) - 1\n      ; i>=0\n      ; i--) {\n    sets.add([\n      \"target\": mapping[i],\n      \"value\": parts[i]\n    ]);\n  }\n  removes.add(field.getKey());\n}\n"#
                ),
                cached_params!(
                    "{\"src\":[\"source.address\",\"source.port\",\"observer.ingress.interface.name\",\"source.domain\"],\"dst\":[\"destination.address\",\"destination.port\",\"observer.egress.interface.name\",\"destination.domain\"]}"
                ),
            )?;

            // Painless script
            // Source: List sets = ctx._temp_.computeIfAbsent(\"sets\", k -> new ArrayList());\nList removes = ctx._temp_.computeIfAbsent(\"removes\", k -> new ArrayList());\nMap base = ctx.sonicwall?.firewall;\nif (base == null) return;\nfor (def entry : params.sources) {\n  if (base.containsKey(entry.field)) {\n    sets.add([\n        \"target\": params.destination,\n        \"value\": base[entry.field] + entry.append\n    ]);\n  }\n  removes.add(entry.field);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"List sets = ctx._temp_.computeIfAbsent(\"sets\", k -> new ArrayList());\nList removes = ctx._temp_.computeIfAbsent(\"removes\", k -> new ArrayList());\nMap base = ctx.sonicwall?.firewall;\nif (base == null) return;\nfor (def entry : params.sources) {\n  if (base.containsKey(entry.field)) {\n    sets.add([\n        \"target\": params.destination,\n        \"value\": base[entry.field] + entry.append\n    ]);\n  }\n  removes.add(entry.field);\n}\n"#
                ),
                cached_params!(
                    "{\"destination\":\"event.duration\",\"sources\":[{\"field\":\"dur\",\"append\":\"000000000\"},{\"field\":\"cdur\",\"append\":\"000000\"}]}"
                ),
            )?;

            foreach_array(event, "_temp_.removes", |event| {
                event.remove("sonicwall.firewall.{{{ _ingest._value }}}");
                Ok(())
            })?;

            foreach_array(event, "_temp_.sets", |event| {
                event.set(
                    "{{{ _ingest._value.target }}}",
                    json!(
                        event
                            .get("_ingest._value.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("sonicwall.firewall.srcV6").cloned() {
                    event.set("source.address", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("sonicwall.firewall.dstV6").cloned() {
                    event.set("destination.address", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("@timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss VV", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                            event.get_str("_conf.tz_offset"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "@timestamp".into(),
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
                        "date_@timestamp_f835b139",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed to parse time field ({}): {}",
                            event
                                .get("@timestamp")
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

            let _cond = {
                !event.has_value("_conf.tz_offset")
                    || event.get_str("_conf.tz_offset") == Some("local")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("@timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss VV", "yyyy-MM-dd HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "@timestamp".into(),
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
                        "date_@timestamp_0d6a6c2f",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed to parse time field ({}): {}",
                            event
                                .get("@timestamp")
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("observer.hostname") {
                    if let Some(val) = event.get("observer.hostname") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "observer.hostname".into(),
                                message,
                            }
                        })?;
                        event.set("_temp_.observer.ip", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("_temp_.observer.ip") };
            if _cond {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("_temp_.observer.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("observer.ip") };
            if _cond {
                if event.remove("observer.hostname").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "observer.hostname".into(),
                    });
                }
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
                if event.remove("source.address").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.address".into(),
                    });
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

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                if event.remove("destination.address").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.address".into(),
                    });
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

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            if event.has_value("source.mac") {
                gsub_field(event, "source.mac", "source.mac", cached_regex!(":"), "-")?;
            }

            if event.has_value("destination.mac") {
                map_strings(
                    event,
                    "destination.mac",
                    "destination.mac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!(":"),
                    "-",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("network.transport") {
                    // Grok pattern: ^(?P<network_transport>(?:[^/]*))/%{NUMBER}$
                    // Grok pattern: ^(?P<network_transport>(?:[^/]*))/(?P<network_protocol>(?:[^/]*))$
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^(?P<network_transport>(?:[^/]*))/%{NUMBER}$",
                                [("network_transport", "network.transport")]
                            ),
                            cached_grok_mapped!(
                                "^(?P<network_transport>(?:[^/]*))/(?P<network_protocol>(?:[^/]*))$",
                                [
                                    ("network_transport", "network.transport"),
                                    ("network_protocol", "network.protocol")
                                ]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_temp_.source_nat_ip") {
                    if let Some(input) = event.get_string("_temp_.source_nat_ip") {
                        // Grok pattern: ^%{IPV4:source.nat.ip}(:?:%{POSINT:source.nat.port})?$
                        // Grok pattern: ^%{IPV6:source.nat.ip}$
                        // Grok pattern: ^\\[%{IPV6:source.nat.ip}\\]:%{POSINT:source.nat.port}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{IPV4:source.nat.ip}(:?:%{POSINT:source.nat.port})?$"
                                ),
                                cached_grok!("^%{IPV6:source.nat.ip}$"),
                                cached_grok!(
                                    "^\\[%{IPV6:source.nat.ip}\\]:%{POSINT:source.nat.port}$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_temp_.destination_nat_ip") {
                    if let Some(input) = event.get_string("_temp_.destination_nat_ip") {
                        // Grok pattern: ^%{IPV4:destination.nat.ip}(:?:%{POSINT:destination.nat.port})?$
                        // Grok pattern: ^%{IPV6:destination.nat.ip}$
                        // Grok pattern: ^\\[%{IPV6:destination.nat.ip}\\]:%{POSINT:destination.nat.port}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{IPV4:destination.nat.ip}(:?:%{POSINT:destination.nat.port})?$"
                                ),
                                cached_grok!("^%{IPV6:destination.nat.ip}$"),
                                cached_grok!(
                                    "^\\[%{IPV6:destination.nat.ip}\\]:%{POSINT:destination.nat.port}$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.bytes") {
                    if let Some(val) = event.get("source.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("source.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_bytes_1c4305d3",
                )?;
                if event.remove("source.bytes").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.bytes".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.port") {
                    if let Some(val) = event.get("source.port") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_port_4955d6a9",
                )?;
                if event.remove("source.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.port".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.nat.port") {
                    if let Some(val) = event.get("source.nat.port") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.nat.port".into(),
                                message,
                            }
                        })?;
                        event.set("source.nat.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_nat_port_cae51fbb",
                )?;
                if event.remove("source.nat.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.nat.port".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.packets") {
                    if let Some(val) = event.get("source.packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.packets".into(),
                                message,
                            }
                        })?;
                        event.set("source.packets", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_packets_7718e13b",
                )?;
                if event.remove("source.packets").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.packets".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.bytes") {
                    if let Some(val) = event.get("destination.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("destination.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_bytes_9a9d8ca5",
                )?;
                if event.remove("destination.bytes").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.bytes".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.port") {
                    if let Some(val) = event.get("destination.port") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_port_cdbd1c15",
                )?;
                if event.remove("destination.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.port".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.nat.port") {
                    if let Some(val) = event.get("destination.nat.port") {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.nat.port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.nat.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_nat_port_d61b8981",
                )?;
                if event.remove("destination.nat.port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.nat.port".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.packets") {
                    if let Some(val) = event.get("destination.packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.packets".into(),
                                message,
                            }
                        })?;
                        event.set("destination.packets", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_destination_packets_7faf12a5",
                )?;
                if event.remove("destination.packets").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.packets".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("event.duration") {
                    if let Some(val) = event.get("event.duration") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "event.duration".into(),
                                message,
                            }
                        })?;
                        event.set("event.duration", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_duration_dfe9fd73",
                )?;
                if event.remove("event.duration").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "event.duration".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("event.sequence") {
                    if let Some(val) = event.get("event.sequence") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "event.sequence".into(),
                                message,
                            }
                        })?;
                        event.set("event.sequence", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_sequence_562f390f",
                )?;
                if event.remove("event.sequence").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "event.sequence".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("event.severity") {
                    if let Some(val) = event.get("event.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "event.severity".into(),
                                message,
                            }
                        })?;
                        event.set("event.severity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_severity_4d6b0993",
                )?;
                if event.remove("event.severity").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "event.severity".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // Painless script
            // Source: for (def src : params.from) {\n  for (def key : params.keys) {\n    def v = null;\n    if (ctx[src] != null && (v = ctx[src][key]) != null && v instanceof Long) {\n      if (ctx[params.to] == null || !(ctx[params.to] instanceof Map)) {\n        ctx[params.to] = new HashMap();\n      }\n      if (ctx[params.to][key] == null || !(ctx[params.to][key] instanceof Long)) {\n        ctx[params.to][key] = v;\n      } else {\n        ctx[params.to][key] += v;\n      }\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"for (def src : params.from) {\n  for (def key : params.keys) {\n    def v = null;\n    if (ctx[src] != null && (v = ctx[src][key]) != null && v instanceof Long) {\n      if (ctx[params.to] == null || !(ctx[params.to] instanceof Map)) {\n        ctx[params.to] = new HashMap();\n      }\n      if (ctx[params.to][key] == null || !(ctx[params.to][key] instanceof Long)) {\n        ctx[params.to][key] = v;\n      } else {\n        ctx[params.to][key] += v;\n      }\n    }\n  }\n}\n"#
                ),
                cached_params!(
                    "{\"keys\":[\"bytes\",\"packets\"],\"from\":[\"source\",\"destination\"],\"to\":\"network\"}"
                ),
            )?;

            let _cond =
                { event.has_value("message") && event.has_value("sonicwall.firewall.note") };
            if _cond {
                event.set(
                    "message",
                    json!(format!(
                        "{} ({})",
                        event
                            .get("message")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("sonicwall.firewall.note")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let v = json!(
                event
                    .get("sonicwall.firewall.note")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                if !event.has("message") {
                    event.set("message", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def clone(def val) {\n  return val instanceof List? new ArrayList(val) : val;\n}\ndef evtype = params.message_codes[ctx.event?.code];\nif (evtype == null) return;\ndef actions = params.event_types[evtype];\nif (actions == null) {\n  throw new Exception(\"message code \" + ctx.event.code + \" references missing event type \" + evtype);\n}\ndef event = ctx.computeIfAbsent('event', k -> new HashMap());\nfor (def entry : actions.entrySet()) {\n  event[entry.getKey()] = clone(entry.getValue());\n}\nevent[\"action\"] = evtype;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def clone(def val) {\n  return val instanceof List? new ArrayList(val) : val;\n}\ndef evtype = params.message_codes[ctx.event?.code];\nif (evtype == null) return;\ndef actions = params.event_types[evtype];\nif (actions == null) {\n  throw new Exception(\"message code \" + ctx.event.code + \" references missing event type \" + evtype);\n}\ndef event = ctx.computeIfAbsent('event', k -> new HashMap());\nfor (def entry : actions.entrySet()) {\n  event[entry.getKey()] = clone(entry.getValue());\n}\nevent[\"action\"] = evtype;\n"#
                    ),
                    cached_params!(
                        "{\"event_types\":{\"internal-log-success\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"internal-log-failure\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"login-success\":{\"kind\":\"event\",\"category\":[\"authentication\"],\"type\":[\"start\",\"info\"],\"outcome\":\"success\"},\"login-failure\":{\"kind\":\"event\",\"category\":[\"authentication\"],\"type\":[\"start\",\"info\"],\"outcome\":\"failure\"},\"logout\":{\"kind\":\"event\",\"category\":[\"authentication\"],\"type\":[\"end\",\"info\"],\"outcome\":\"success\"},\"user-account-locked\":{\"kind\":\"event\",\"category\":[\"iam\"],\"type\":[\"info\",\"user\"],\"outcome\":\"success\"},\"user-account-unlocked\":{\"kind\":\"event\",\"category\":[\"iam\"],\"type\":[\"info\",\"user\"],\"outcome\":\"success\"},\"user-account-enabled\":{\"kind\":\"event\",\"category\":[\"iam\"],\"type\":[\"info\",\"user\"],\"outcome\":\"success\"},\"user-account-disabled\":{\"kind\":\"event\",\"category\":[\"iam\"],\"type\":[\"info\",\"user\"],\"outcome\":\"success\"},\"user-account-created\":{\"kind\":\"event\",\"category\":[\"iam\"],\"type\":[\"info\",\"user\",\"deletion\"],\"outcome\":\"success\"},\"user-account-changed\":{\"kind\":\"event\",\"category\":[\"iam\"],\"type\":[\"info\",\"user\",\"change\"],\"outcome\":\"success\"},\"user-account-change-failure\":{\"kind\":\"event\",\"category\":[\"iam\"],\"type\":[\"info\",\"user\",\"change\"],\"outcome\":\"failure\"},\"admin-account-changed\":{\"kind\":\"event\",\"category\":[\"iam\"],\"type\":[\"info\",\"user\",\"change\",\"admin\"],\"outcome\":\"success\"},\"user-account-deleted\":{\"kind\":\"event\",\"category\":[\"iam\"],\"type\":[\"info\",\"user\",\"deletion\"],\"outcome\":\"success\"},\"session-start\":{\"kind\":\"event\",\"category\":[\"session\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"session-end\":{\"kind\":\"event\",\"category\":[\"session\"],\"type\":[\"end\"],\"outcome\":\"success\"},\"attack-detected\":{\"kind\":\"alert\",\"category\":[\"intrusion_detection\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"attack-blocked\":{\"kind\":\"alert\",\"category\":[\"intrusion_detection\"],\"type\":[\"denied\"],\"outcome\":\"success\"},\"connection-start\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"connection\",\"start\"],\"outcome\":\"success\"},\"connection-end\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"connection\",\"end\"],\"outcome\":\"success\"},\"connection-denied\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"connection\",\"denied\"],\"outcome\":\"success\"},\"packet-dropped\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"denied\"],\"outcome\":\"success\"},\"connection-info\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"connection\",\"info\"],\"outcome\":\"success\"},\"malware-info\":{\"kind\":\"alert\",\"category\":[\"malware\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"config-change\":{\"kind\":\"event\",\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"config-change-failure\":{\"kind\":\"event\",\"category\":[\"configuration\"],\"type\":[\"change\"],\"outcome\":\"failure\"},\"config-info\":{\"kind\":\"event\",\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"config-delete\":{\"kind\":\"event\",\"category\":[\"configuration\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},\"config-add\":{\"kind\":\"event\",\"category\":[\"configuration\"],\"type\":[\"creation\"],\"outcome\":\"success\"}},\"message_codes\":{\"646\":\"packet-dropped\",\"647\":\"packet-dropped\",\"734\":\"connection-info\",\"735\":\"packet-dropped\",\"45\":\"connection-info\",\"815\":\"connection-info\",\"428\":\"packet-dropped\",\"1473\":\"packet-dropped\",\"1573\":\"packet-dropped\",\"1576\":\"packet-dropped\",\"41\":\"packet-dropped\",\"46\":\"packet-dropped\",\"98\":\"connection-start\",\"347\":\"packet-dropped\",\"537\":\"connection-end\",\"590\":\"packet-dropped\",\"714\":\"packet-dropped\",\"1304\":\"packet-dropped\",\"883\":\"packet-dropped\",\"884\":\"packet-dropped\",\"885\":\"packet-dropped\",\"886\":\"packet-dropped\",\"1448\":\"packet-dropped\",\"1449\":\"packet-dropped\",\"1198\":\"connection-denied\",\"1199\":\"connection-denied\",\"1474\":\"connection-denied\",\"1475\":\"connection-denied\",\"38\":\"packet-dropped\",\"63\":\"packet-dropped\",\"175\":\"packet-dropped\",\"182\":\"connection-info\",\"188\":\"connection-info\",\"523\":\"packet-dropped\",\"597\":\"connection-info\",\"598\":\"connection-info\",\"1254\":\"packet-dropped\",\"1255\":\"connection-info\",\"1256\":\"connection-info\",\"1257\":\"packet-dropped\",\"1431\":\"connection-info\",\"1433\":\"packet-dropped\",\"1458\":\"connection-info\",\"28\":\"packet-dropped\",\"522\":\"packet-dropped\",\"910\":\"packet-dropped\",\"1301\":\"packet-dropped\",\"1302\":\"packet-dropped\",\"1303\":\"packet-dropped\",\"1429\":\"packet-dropped\",\"1430\":\"packet-dropped\",\"651\":\"packet-dropped\",\"652\":\"packet-dropped\",\"653\":\"packet-dropped\",\"1253\":\"packet-dropped\",\"683\":\"packet-dropped\",\"690\":\"packet-dropped\",\"694\":\"packet-dropped\",\"1233\":\"packet-dropped\",\"339\":\"packet-dropped\",\"1197\":\"connection-info\",\"1436\":\"packet-dropped\",\"1313\":\"config-add\",\"1314\":\"config-change\",\"1315\":\"config-delete\",\"36\":\"packet-dropped\",\"48\":\"packet-dropped\",\"173\":\"connection-denied\",\"181\":\"packet-dropped\",\"524\":\"connection-denied\",\"580\":\"packet-dropped\",\"708\":\"packet-dropped\",\"709\":\"packet-dropped\",\"712\":\"connection-denied\",\"713\":\"connection-denied\",\"760\":\"connection-denied\",\"887\":\"packet-dropped\",\"888\":\"packet-dropped\",\"889\":\"packet-dropped\",\"890\":\"packet-dropped\",\"891\":\"packet-dropped\",\"892\":\"packet-dropped\",\"893\":\"packet-dropped\",\"894\":\"packet-dropped\",\"895\":\"packet-dropped\",\"896\":\"packet-dropped\",\"1029\":\"packet-dropped\",\"1030\":\"packet-dropped\",\"1031\":\"packet-dropped\",\"1384\":\"packet-dropped\",\"1385\":\"packet-dropped\",\"1628\":\"packet-dropped\",\"1629\":\"packet-dropped\",\"14\":\"connection-denied\",\"16\":\"connection-info\",\"1599\":\"config-add\",\"1600\":\"config-change\",\"1601\":\"config-change\",\"797\":\"connection-denied\",\"798\":\"connection-denied\",\"22\":\"attack-blocked\",\"23\":\"attack-blocked\",\"27\":\"attack-blocked\",\"81\":\"attack-blocked\",\"82\":\"attack-detected\",\"83\":\"attack-detected\",\"177\":\"attack-detected\",\"178\":\"attack-detected\",\"179\":\"attack-detected\",\"267\":\"attack-blocked\",\"606\":\"attack-blocked\",\"1316\":\"attack-detected\",\"1373\":\"attack-detected\",\"1374\":\"attack-detected\",\"1375\":\"attack-detected\",\"1376\":\"attack-blocked\",\"1387\":\"attack-blocked\",\"1471\":\"attack-detected\",\"229\":\"attack-blocked\",\"1098\":\"attack-detected\",\"1099\":\"attack-blocked\",\"1593\":\"attack-detected\",\"446\":\"attack-blocked\",\"527\":\"attack-blocked\",\"528\":\"attack-blocked\",\"538\":\"attack-blocked\",\"789\":\"attack-detected\",\"790\":\"attack-blocked\",\"608\":\"attack-detected\",\"609\":\"attack-blocked\",\"25\":\"attack-detected\",\"856\":\"config-change\",\"857\":\"config-change\",\"858\":\"config-change\",\"859\":\"attack-detected\",\"860\":\"attack-detected\",\"862\":\"config-change\",\"863\":\"config-change\",\"864\":\"attack-blocked\",\"897\":\"attack-detected\",\"898\":\"attack-blocked\",\"901\":\"attack-blocked\",\"904\":\"attack-detected\",\"905\":\"attack-detected\",\"1180\":\"attack-blocked\",\"1213\":\"attack-detected\",\"1214\":\"attack-detected\",\"1366\":\"attack-blocked\",\"1369\":\"attack-detected\",\"1450\":\"attack-detected\",\"1451\":\"attack-detected\",\"1452\":\"attack-detected\",\"879\":\"attack-detected\",\"1363\":\"attack-detected\",\"546\":\"attack-detected\",\"548\":\"attack-detected\",\"24\":\"logout\",\"29\":\"login-success\",\"30\":\"login-failure\",\"31\":\"login-success\",\"32\":\"login-failure\",\"33\":\"login-failure\",\"34\":\"login-failure\",\"35\":\"login-failure\",\"199\":\"login-success\",\"200\":\"login-failure\",\"235\":\"login-success\",\"236\":\"login-success\",\"237\":\"login-success\",\"238\":\"login-success\",\"246\":\"login-failure\",\"261\":\"logout\",\"262\":\"logout\",\"263\":\"logout\",\"264\":\"logout\",\"265\":\"logout\",\"328\":\"admin-account-changed\",\"329\":\"login-failure\",\"438\":\"user-account-unlocked\",\"439\":\"user-account-unlocked\",\"486\":\"login-failure\",\"506\":\"config-change\",\"507\":\"config-change\",\"508\":\"config-change\",\"509\":\"config-change\",\"520\":\"logout\",\"549\":\"login-failure\",\"550\":\"session-end\",\"551\":\"session-end\",\"557\":\"login-failure\",\"558\":\"user-account-created\",\"559\":\"user-account-deleted\",\"560\":\"user-account-disabled\",\"561\":\"user-account-enabled\",\"562\":\"user-account-deleted\",\"564\":\"session-end\",\"583\":\"login-failure\",\"728\":\"config-change\",\"729\":\"config-change\",\"759\":\"login-failure\",\"986\":\"login-failure\",\"987\":\"login-failure\",\"994\":\"session-start\",\"995\":\"session-end\",\"996\":\"session-start\",\"997\":\"session-start\",\"998\":\"session-end\",\"1008\":\"logout\",\"1035\":\"login-failure\",\"1048\":\"login-failure\",\"1080\":\"login-success\",\"1117\":\"login-failure\",\"1118\":\"login-failure\",\"1119\":\"login-failure\",\"1120\":\"login-failure\",\"1121\":\"login-failure\",\"1122\":\"login-failure\",\"1123\":\"login-failure\",\"1124\":\"logout\",\"1157\":\"user-account-disabled\",\"1158\":\"user-account-deleted\",\"1243\":\"login-failure\",\"1333\":\"user-account-created\",\"1334\":\"user-account-changed\",\"1335\":\"user-account-deleted\",\"1341\":\"user-account-changed\",\"1342\":\"user-account-changed\",\"1517\":\"login-failure\",\"1570\":\"user-account-locked\",\"1571\":\"user-account-unlocked\",\"1572\":\"login-failure\",\"1585\":\"login-failure\",\"1627\":\"user-account-disabled\",\"1655\":\"login-failure\",\"1672\":\"login-failure\",\"243\":\"login-failure\",\"244\":\"login-failure\",\"245\":\"login-failure\",\"744\":\"login-failure\",\"745\":\"login-failure\",\"746\":\"login-failure\",\"747\":\"login-failure\",\"748\":\"login-failure\",\"749\":\"login-failure\",\"750\":\"login-failure\",\"751\":\"login-failure\",\"753\":\"login-failure\",\"754\":\"login-failure\",\"755\":\"login-failure\",\"756\":\"login-failure\",\"757\":\"login-failure\",\"1011\":\"user-account-change-failure\",\"988\":\"login-failure\",\"989\":\"login-failure\",\"990\":\"login-failure\",\"991\":\"login-failure\",\"794\":\"malware-info\",\"795\":\"malware-info\",\"796\":\"malware-info\",\"123\":\"malware-info\",\"124\":\"malware-info\",\"125\":\"malware-info\",\"159\":\"malware-info\",\"408\":\"malware-info\",\"482\":\"malware-info\",\"1559\":\"malware-info\",\"1560\":\"malware-info\",\"1561\":\"malware-info\",\"1562\":\"malware-info\",\"1154\":\"malware-info\",\"1155\":\"malware-info\",\"793\":\"malware-info\",\"1654\":\"malware-info\",\"440\":\"config-add\",\"441\":\"config-change\",\"442\":\"config-delete\",\"340\":\"config-change\",\"341\":\"config-change\",\"1590\":\"config-info\",\"1195\":\"attack-detected\",\"1200\":\"attack-blocked\",\"1201\":\"attack-blocked\",\"1476\":\"attack-blocked\",\"1477\":\"attack-blocked\",\"1518\":\"attack-blocked\",\"1519\":\"attack-blocked\",\"1511\":\"internal-log-success\",\"1512\":\"internal-log-failure\",\"1513\":\"internal-log-success\",\"1514\":\"internal-log-failure\",\"1515\":\"internal-log-success\",\"1516\":\"internal-log-failure\",\"93\":\"internal-log-failure\",\"94\":\"internal-log-failure\",\"95\":\"internal-log-failure\",\"164\":\"internal-log-failure\",\"599\":\"internal-log-failure\",\"600\":\"internal-log-failure\",\"601\":\"internal-log-failure\",\"1046\":\"internal-log-success\",\"1047\":\"internal-log-success\",\"1392\":\"internal-log-success\",\"1393\":\"internal-log-success\",\"573\":\"internal-log-failure\",\"574\":\"internal-log-failure\",\"1049\":\"internal-log-success\",\"1065\":\"internal-log-success\",\"1066\":\"internal-log-failure\",\"1160\":\"internal-log-failure\",\"1161\":\"internal-log-failure\",\"1268\":\"internal-log-failure\",\"1269\":\"config-change\",\"1336\":\"config-change\",\"1337\":\"user-account-changed\",\"1338\":\"user-account-changed\",\"1339\":\"config-change\",\"1340\":\"config-change\",\"1432\":\"config-change\",\"1494\":\"internal-log-success\",\"1520\":\"internal-log-success\",\"1521\":\"internal-log-failure\",\"1565\":\"internal-log-success\",\"1566\":\"internal-log-failure\",\"1567\":\"internal-log-success\",\"1568\":\"internal-log-failure\",\"1636\":\"internal-log-failure\",\"1637\":\"internal-log-failure\",\"1149\":\"internal-log-failure\",\"1152\":\"internal-log-failure\",\"4\":\"internal-log-success\",\"53\":\"internal-log-failure\",\"521\":\"internal-log-success\",\"1107\":\"internal-log-failure\",\"1196\":\"internal-log-failure\",\"1332\":\"config-change\",\"1495\":\"internal-log-success\",\"1496\":\"internal-log-success\",\"1382\":\"config-change\",\"1383\":\"config-change-failure\",\"1674\":\"config-change\",\"58\":\"connection-denied\",\"999\":\"connection-info\",\"1001\":\"connection-info\",\"1002\":\"connection-info\",\"1003\":\"connection-info\",\"1004\":\"connection-info\",\"1005\":\"connection-info\",\"1006\":\"connection-info\",\"1081\":\"connection-info\"}}"
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_9079cbb3")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "internal ECS categorization error: {}",
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

            let _cond = { event.has_value("url.path") };
            if _cond {
                let v = json!(
                    event
                        .get("network.protocol")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("url.scheme", v)?;
                }
            }

            let _cond = { !event.has_value("url.path") };
            if _cond {
                if event.has_value("url.domain") {
                    event.rename("url.domain", "sonicwall.firewall.dstname")?;
                }
            }

            let _cond = { event.has_value("url.scheme") && event.has_value("url.domain") };
            if _cond {
                event.set(
                    "url.full",
                    json!(format!(
                        "{}://{}{}",
                        event
                            .get("url.scheme")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("url.path")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { !event.has_value("url.scheme") && event.has_value("url.domain") };
            if _cond {
                event.set(
                    "url.full",
                    json!(format!(
                        "//{}{}",
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("url.path")
                            .map_or_else(String::new, template_to_string)
                    )),
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

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
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

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.observer.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("_temp_.observer.ip")
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_conf");
                event.remove("_temp_");
                event.remove("sonicwall.firewall.srcV6");
                event.remove("sonicwall.firewall.dstV6");
                event.remove("sonicwall.firewall.note");
                event.remove("sonicwall.firewall.c");
                Ok(())
            })();

            let _cond = {
                event.get("sonicwall.firewall").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                if event.remove("sonicwall").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "sonicwall".into(),
                    });
                }
            }

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
