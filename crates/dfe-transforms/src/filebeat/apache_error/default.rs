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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("ecs.version", json!("8.11.0"))?;

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

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: \\[(?P<apache_error_timestamp>(?:((%{DAY} %{MONTH} %{MONTHDAY} %{TIME} %{YEAR})|(%{YEAR}\\-%{MONTHNUM}\\-%{MONTHDAY} %{TIME} %{ISO8601_TIMEZONE}))))\\] \\[%{LOGLEVEL:log.level}\\]( \\[client %{IPORHOST:source.address}(:%{POSINT:source.port})?\\])? %{GREEDYDATA:message}
                    // Grok pattern: (\\[(?P<apache_error_timestamp>(?:((%{DAY} %{MONTH} %{MONTHDAY} %{TIME} %{YEAR})|(%{YEAR}\\-%{MONTHNUM}\\-%{MONTHDAY} %{TIME} %{ISO8601_TIMEZONE}))))\\] )?(\\[%{DATA:apache.error.module}:(?P<log_level>(?:%{LOGLEVEL}[0-9]*))\\] \\[pid %{NUMBER:process.pid:long}(:tid %{NUMBER:process.thread.id:long})?\\]( \\[client %{IPORHOST:source.address}(:%{POSINT:source.port})?\\])? )?%{GREEDYDATA:message}
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "\\[(?P<apache_error_timestamp>(?:((%{DAY} %{MONTH} %{MONTHDAY} %{TIME} %{YEAR})|(%{YEAR}\\-%{MONTHNUM}\\-%{MONTHDAY} %{TIME} %{ISO8601_TIMEZONE}))))\\] \\[%{LOGLEVEL:log.level}\\]( \\[client %{IPORHOST:source.address}(:%{POSINT:source.port})?\\])? %{GREEDYDATA:message}",
                                [("apache_error_timestamp", "apache.error.timestamp")]
                            ),
                            cached_grok_mapped!(
                                "(\\[(?P<apache_error_timestamp>(?:((%{DAY} %{MONTH} %{MONTHDAY} %{TIME} %{YEAR})|(%{YEAR}\\-%{MONTHNUM}\\-%{MONTHDAY} %{TIME} %{ISO8601_TIMEZONE}))))\\] )?(\\[%{DATA:apache.error.module}:(?P<log_level>(?:%{LOGLEVEL}[0-9]*))\\] \\[pid %{NUMBER:process.pid:long}(:tid %{NUMBER:process.thread.id:long})?\\]( \\[client %{IPORHOST:source.address}(:%{POSINT:source.port})?\\])? )?%{GREEDYDATA:message}",
                                [
                                    ("apache_error_timestamp", "apache.error.timestamp"),
                                    ("log_level", "log.level")
                                ]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: File does not exist: %{URIPATH:file.path}, referer: %{URI:http.request.referrer}
                        // Grok pattern: File does not exist: %{URIPATH:file.path}
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "File does not exist: %{URIPATH:file.path}, referer: %{URI:http.request.referrer}"
                                ),
                                cached_grok!("File does not exist: %{URIPATH:file.path}"),
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

            let _cond =
                { !event.has_value("event.timezone") && event.has_value("apache.error.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("apache.error.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss.SSSSSS Z",
                                "EEE MMM dd H:m:s yyyy",
                                "EEE MMM dd H:m:s.SSSSSS yyyy",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "apache.error.timestamp".into(),
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

            let _cond =
                { event.has_value("event.timezone") && event.has_value("apache.error.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("apache.error.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss.SSSSSS Z",
                                "EEE MMM dd H:m:s yyyy",
                                "EEE MMM dd H:m:s.SSSSSS yyyy",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "apache.error.timestamp".into(),
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("apache.error.timestamp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "apache.error.timestamp".into(),
                    });
                }
                Ok(())
            })();

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("web"))?;

            let _cond = { event.has_value("log.level") };
            if _cond {
                // Painless script
                // Source: def err_levels = [\"emerg\", \"alert\", \"crit\", \"error\", \"warn\"]; if (err_levels.contains(ctx.log.level)) {\n  ctx.event.type = [\"error\"];\n} else {\n  ctx.event.type = [\"info\"];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def err_levels = [\"emerg\", \"alert\", \"crit\", \"error\", \"warn\"]; if (err_levels.contains(ctx.log.level)) {\n  ctx.event.type = [\"error\"];\n} else {\n  ctx.event.type = [\"info\"];\n}"#
                    ),
                )?;
            }

            if event.has_value("source.address") {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^(%{IP:source.ip}|%{HOSTNAME:source.domain})$
                    if !cached_grok!("^(%{IP:source.ip}|%{HOSTNAME:source.domain})$")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("source.port") {
                if let Some(val) = event.get("source.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v.equals(\"\") || (v instanceof Map && v.values().isEmpty()));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_collections: true,
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
