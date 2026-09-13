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

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.error.time} %{IPORHOST:source.address} %{NUMBER:source.port:long} %{IPORHOST:destination.address} %{IPORHOST:destination.port:long} (?:HTTP/%{NUMBER:http.version}|-) (?:-|%{NOTSPACE:http.request.method}) (?:-|%{NOTSPACE:_tmp.url_orig}) (?:%{NUMBER}|-) (?:%{NUMBER:http.response.status_code:long}|-) (?:%{NUMBER}|-) (?:-|%{NOTSPACE:iis.error.reason_phrase}) (?:-|%{NOTSPACE:iis.error.queue_name})
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.error.time} %{IPORHOST:source.address} %{NUMBER:source.port:long} %{IPORHOST:destination.address} %{IPORHOST:destination.port:long} (?:HTTP/%{NUMBER:http.version}|-) (?:-|%{NOTSPACE:http.request.method}) (?:-|%{NOTSPACE:_tmp.url_orig}) (?:%{NUMBER:http.response.status_code:long}|-) (?:%{NUMBER}|-) (?:-|%{NOTSPACE:iis.error.reason_phrase}) (?:-|%{NOTSPACE:iis.error.queue_name})
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.error.time} %{IPORHOST:source.address} %{NUMBER:source.port:long} %{IPORHOST:destination.address} %{IPORHOST:destination.port:long} (?:HTTP/%{NUMBER:http.version}|-) (?:-|%{NOTSPACE:http.request.method}) (?:-|%{NOTSPACE:_tmp.url_orig}) (?:%{NUMBER}|-) (?:%{NUMBER:http.response.status_code:long}|-) (?:%{NUMBER}|-) (?:-|%{NOTSPACE:iis.error.reason_phrase}) (?:-|%{NOTSPACE:iis.error.queue_name})"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.error.time} %{IPORHOST:source.address} %{NUMBER:source.port:long} %{IPORHOST:destination.address} %{IPORHOST:destination.port:long} (?:HTTP/%{NUMBER:http.version}|-) (?:-|%{NOTSPACE:http.request.method}) (?:-|%{NOTSPACE:_tmp.url_orig}) (?:%{NUMBER:http.response.status_code:long}|-) (?:%{NUMBER}|-) (?:-|%{NOTSPACE:iis.error.reason_phrase}) (?:-|%{NOTSPACE:iis.error.queue_name})"
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
                uri_parts(event, "_tmp.url_orig", "url", true, false)?;
                Ok(())
            })();

            let _cond = { event.has_value("_tmp.url_orig") && !event.has_value("url.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "url.original",
                        json!(
                            event
                                .get("_tmp.url_orig")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            event.remove("_tmp");

            if event.has_value("@timestamp") {
                event.rename("@timestamp", "event.created")?;
            }

            if let Some(date_str) = event.get_as_string("iis.error.time") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "iis.error.time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("iis.error.time").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "iis.error.time".into(),
                });
            }

            if let Some(input) = event.get_string("destination.address") {
                // Grok pattern: (?P<destination_ip>(?:[^%]*))
                if !cached_grok_mapped!(
                    "(?P<destination_ip>(?:[^%]*))",
                    [("destination_ip", "destination.ip")]
                )
                .extract_into(&input, event)?
                {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            if let Some(input) = event.get_string("source.address") {
                // Grok pattern: (?P<source_ip>(?:[^%]*))
                if !cached_grok_mapped!("(?P<source_ip>(?:[^%]*))", [("source_ip", "source.ip")])
                    .extract_into(&input, event)?
                {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

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

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("web"))?;

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                event.append("event.type", json!("connection"))?;
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
