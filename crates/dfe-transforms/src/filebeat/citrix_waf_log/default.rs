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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|(?P<_tmp_timestamp8601>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:event.timezone}?)))) (?:<?(?P<citrix_facility>(?:[a-zA-Z][a-zA-Z0-9]*))\\.(?P<citrix_priority>(?:[a-zA-Z][a-zA-Z0-9]*))>?) %{IP:client.ip:ip} %{GREEDYDATA:citrix.detail}
                // Grok pattern: ^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)%{NONNEGINT:log.syslog.version} +(?:-|(?P<_tmp_timestamp8601>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:event.timezone}?))) +(?:-|%{IPORHOST:log.syslog.hostname}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.appname}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.procid}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.msgid}) +(?:-|%{SYSLOG5424SD})?) +%{GREEDYDATA:citrix.detail}
                // Grok pattern: ^%{GREEDYDATA:citrix.detail}
                let _ = extract_first_match(
                    &[
                        cached_grok_mapped!(
                            "^(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|(?P<_tmp_timestamp8601>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:event.timezone}?)))) (?:<?(?P<citrix_facility>(?:[a-zA-Z][a-zA-Z0-9]*))\\.(?P<citrix_priority>(?:[a-zA-Z][a-zA-Z0-9]*))>?) %{IP:client.ip:ip} %{GREEDYDATA:citrix.detail}",
                            [
                                ("_tmp_timestamp8601", "_tmp.timestamp8601"),
                                ("citrix_facility", "citrix.facility"),
                                ("citrix_priority", "citrix.priority")
                            ]
                        ),
                        cached_grok_mapped!(
                            "^(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)%{NONNEGINT:log.syslog.version} +(?:-|(?P<_tmp_timestamp8601>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?%{ISO8601_TIMEZONE:event.timezone}?))) +(?:-|%{IPORHOST:log.syslog.hostname}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.appname}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.procid}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.msgid}) +(?:-|%{SYSLOG5424SD})?) +%{GREEDYDATA:citrix.detail}",
                            [("_tmp_timestamp8601", "_tmp.timestamp8601")]
                        ),
                        cached_grok!("^%{GREEDYDATA:citrix.detail}"),
                    ],
                    &input,
                    event,
                )?;
            }

            let _cond =
                { event.has_value("log.syslog.hostname") && !event.has_value("observer.hostname") };
            if _cond {
                if let Some(v) = event.get("log.syslog.hostname").cloned() {
                    event.set("observer.hostname", v)?;
                }
            }

            let _cond = {
                event.has_value("citrix.detail")
                    && event
                        .get_str("citrix.detail")
                        .is_some_and(|s| s.starts_with("CEF:"))
            };
            if _cond {
                // Begin nested pipeline: "cef"
                event.set("citrix.cef_format", json!(true))?;
                if let Some(input) = event.get_string("citrix.detail") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("CEF:") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.cef_version", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.device_vendor", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.device_product", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.device_version", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.device_event_class_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("citrix.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("|") else {
                            break 'dissect false;
                        };
                        captured.push(("event.severity", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("|") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("citrix.extended.message", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "citrix.detail".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
                if event.has_value("citrix.extended.message") {
                    if let Some(kv_str) = event.get_string("citrix.extended.message") {
                        for pair in cached_regex!(" (?=[a-zA-Z][a-zA-Z0-9]*=)")
                            .split(&kv_str)
                            .into_iter()
                        {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "citrix.extended.message".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("citrix.extended_kv.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.has_value("citrix.extended_kv") };
                if _cond {
                    if event.remove("citrix.extended").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "citrix.extended".into(),
                        });
                    }
                }
                if event.has_value("citrix.extended_kv.src") {
                    if let Some(val) = event.get("citrix.extended_kv.src") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "citrix.extended_kv.src".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                event.remove("citrix.extended_kv.src");
                if event.has_value("citrix.extended_kv.spt") {
                    if let Some(val) = event.get("citrix.extended_kv.spt") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "citrix.extended_kv.spt".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                event.remove("citrix.extended_kv.spt");
                if event.has_value("citrix.extended_kv.method") {
                    event.rename("citrix.extended_kv.method", "http.request.method")?;
                }
                if event.has_value("citrix.extended_kv.request") {
                    event.rename("citrix.extended_kv.request", "url.original")?;
                }
                if event.has_value("citrix.extended_kv.act") {
                    event.rename("citrix.extended_kv.act", "event.action")?;
                }
                if event.has_value("citrix.extended_kv.msg") {
                    event.rename("citrix.extended_kv.msg", "message")?;
                }
                if event.has_value("citrix.extended_kv.cn1") {
                    event.rename("citrix.extended_kv.cn1", "event.id")?;
                }
                if event.has_value("citrix.extended_kv.cn2") {
                    event.rename("citrix.extended_kv.cn2", "http.request.id")?;
                }
                if event.has_value("citrix.extended_kv.cs1") {
                    event.rename("citrix.extended_kv.cs1", "citrix.profile_name")?;
                }
                if event.has_value("citrix.extended_kv.cs2") {
                    event.rename("citrix.extended_kv.cs2", "citrix.ppe_id")?;
                }
                if event.has_value("citrix.extended_kv.cs3") {
                    event.rename("citrix.extended_kv.cs3", "citrix.session_id")?;
                }
                if event.has_value("citrix.extended_kv.cs4") {
                    event.rename("citrix.extended_kv.cs4", "citrix.severity")?;
                }
                if event.has_value("citrix.extended_kv.cs5") {
                    event.rename("citrix.extended_kv.cs5", "citrix.event_year")?;
                }
                if event.has_value("citrix.extended_kv.cs6") {
                    event.rename(
                        "citrix.extended_kv.cs6",
                        "citrix.signature_violation_category",
                    )?;
                }
                if event.has_value("citrix.extended_kv") {
                    event.rename("citrix.extended_kv", "citrix.extended")?;
                }
                // End nested pipeline: "cef"
            }

            let _cond = {
                event.has_value("citrix.detail")
                    && !(event
                        .get_str("citrix.detail")
                        .is_some_and(|s| s.starts_with("CEF:")))
            };
            if _cond {
                // Begin nested pipeline: "native"
                event.set("citrix.cef_format", json!(false))?;
                if let Some(input) = event.get_string("citrix.detail") {
                    // Grok pattern: ^(?:(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone} (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : )?%{GREEDYDATA:_tmp.details} : +\"%{GREEDYDATA:citrix.extended.message}\"
                    // Grok pattern: ^(?:(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone} (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : )?%{GREEDYDATA:_tmp.details} : +%{GREEDYDATA:citrix.extended.message}
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^(?:(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone} (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : )?%{GREEDYDATA:_tmp.details} : +\"%{GREEDYDATA:citrix.extended.message}\"",
                                [("_tmp_timestamp_native", "_tmp.timestamp_native")]
                            ),
                            cached_grok_mapped!(
                                "^(?:(?:(?:<%{NUMBER}>%{SPACE})?(?P<_tmp_timestamp_native>(?:(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}|%{YEAR}/%{MONTHNUM}/%{MONTHDAY}):%{HOUR}:%{MINUTE}:%{SECOND})) %{WORD:event.timezone} (?:%{SYSLOGHOST:citrix.host} )?%{INT}-PPE-%{INT}) : )?%{GREEDYDATA:_tmp.details} : +%{GREEDYDATA:citrix.extended.message}",
                                [("_tmp_timestamp_native", "_tmp.timestamp_native")]
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
                let _cond = { !event.has_value("log.syslog.appname") };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.details") {
                        // Grok pattern: ^(?P<_tmp_default>(?:default ))?%{WORD:citrix.device_event_class_id} %{GREEDYDATA:citrix.name} %{INT:event.id} %{INT:event.severity}$
                        let _ = cached_grok_mapped!("^(?P<_tmp_default>(?:default ))?%{WORD:citrix.device_event_class_id} %{GREEDYDATA:citrix.name} %{INT:event.id} %{INT:event.severity}$", [("_tmp_default", "_tmp.default")]).extract_into(&input, event)?;
                    }
                }
                let _cond = { event.has_value("log.syslog.appname") };
                if _cond {
                    if let Some(input) = event.get_string("_tmp.details") {
                        // Grok pattern: ^(?P<_tmp_default>(?:default ))?%{GREEDYDATA:citrix.name} %{INT:event.id} %{INT:event.severity}$
                        let _ = cached_grok_mapped!("^(?P<_tmp_default>(?:default ))?%{GREEDYDATA:citrix.name} %{INT:event.id} %{INT:event.severity}$", [("_tmp_default", "_tmp.default")]).extract_into(&input, event)?;
                    }
                }
                let _cond = {
                    !event.has_value("citrix.device_event_class_id")
                        && event.has_value("log.syslog.appname")
                };
                if _cond {
                    if let Some(v) = event.get("log.syslog.appname").cloned() {
                        event.set("citrix.device_event_class_id", v)?;
                    }
                }
                let _cond = { event.get_str("_tmp.default") == Some("default ") };
                if _cond {
                    event.set("citrix.default_class", json!(true))?;
                }
                // End nested pipeline: "native"
            }

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

            if !event.has("_conf.tz_offset") {
                event.set("_conf.tz_offset", json!("UTC"))?;
            }

            let _cond = {
                !event.has_value("event.timezone") || event.get_str("event.timezone") == Some("")
            };
            if _cond {
                if let Some(v) = event.get("_conf.tz_offset").cloned() {
                    event.set("event.timezone", v)?;
                }
            }

            let _cond = { event.has_value("_tmp.timestamp8601") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp8601") {
                    match parse_date_out(
                        &date_str,
                        &["ISO8601"],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp8601".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond =
                { event.has_value("_tmp.timestamp") && event.has_value("citrix.event_year") };
            if _cond {
                event.set(
                    "_tmp.timestamp",
                    json!(format!(
                        "{} {}",
                        event
                            .get("citrix.event_year")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_tmp.timestamp")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "MMM d HH:mm:ss",
                            "MMM  d HH:mm:ss",
                            "MMM dd HH:mm:ss",
                            "MMMM d HH:mm:ss",
                            "MMMM  d HH:mm:ss",
                            "MMMM dd HH:mm:ss",
                            "yyyy MMM d HH:mm:ss",
                            "yyyy MMM  d HH:mm:ss",
                            "yyyy MMM dd HH:mm:ss",
                            "yyyy MMMM d HH:mm:ss",
                            "yyyy MMMM  d HH:mm:ss",
                            "yyyy MMMM dd HH:mm:ss",
                        ],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("_tmp.timestamp_native") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp_native") {
                    match parse_date_out(
                        &date_str,
                        &["MM/dd/yyyy:HH:mm:ss", "yyyy/MM/dd:HH:mm:ss"],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp_native".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("citrix.event_year").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "citrix.event_year".into(),
                    });
                }
                Ok(())
            })();

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

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("client.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("client.as.organization_name", v.clone())?;
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

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            let _cond =
                { event.has_value("url.original") && event.get_str("url.original") != Some("") };
            if _cond {
                uri_parts(event, "url.original", "url", true, false)?;
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            event.remove("_tmp");
            event.remove("_conf");

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
                event.remove("_conf");
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
