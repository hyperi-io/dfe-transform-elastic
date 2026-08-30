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

            event.set("event.kind", json!("event"))?;

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

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("#"))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            parse_json_field(event, "event.original", "gitlab.auth")?;

            let _cond = { event.has_value("gitlab.auth.time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("gitlab.auth.time") {
                        match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "gitlab.auth.time".into(),
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
                        "date_event_created_time_epoch",
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
            }

            event.remove("gitlab.auth.time");

            let _cond = { event.get_str("gitlab.auth.severity") == Some("DEBUG") };
            if _cond {
                event.set("event.severity", json!(0))?;
            }

            let _cond = { event.get_str("gitlab.auth.severity") == Some("INFO") };
            if _cond {
                event.set("event.severity", json!(1))?;
            }

            let _cond = { event.get_str("gitlab.auth.severity") == Some("WARN") };
            if _cond {
                event.set("event.severity", json!(2))?;
            }

            let _cond = { event.get_str("gitlab.auth.severity") == Some("ERROR") };
            if _cond {
                event.set("event.severity", json!(3))?;
            }

            let _cond = { event.get_str("gitlab.auth.severity") == Some("FATAL") };
            if _cond {
                event.set("event.severity", json!(4))?;
            }

            let _cond = { event.get_str("gitlab.auth.severity") == Some("UNKNOWN") };
            if _cond {
                event.set("event.severity", json!(5))?;
            }

            event.remove("gitlab.auth.severity");

            if event.has_value("gitlab.auth.correlation_id") {
                event.rename("gitlab.auth.correlation_id", "event.id")?;
            }

            dot_expand(event, "gitlab.auth", "meta.user")?;

            if event.has_value("gitlab.auth.user_id") {
                if let Some(val) = event.get("gitlab.auth.user_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "gitlab.auth.user_id".into(),
                            message,
                        }
                    })?;
                    event.set("gitlab.auth.user_id", converted)?;
                }
            }

            if event.has_value("gitlab.auth.user_id") {
                event.rename("gitlab.auth.user_id", "user.id")?;
            }

            if event.has_value("gitlab.auth.request_method") {
                event.rename("gitlab.auth.request_method", "http.request.method")?;
            }

            if event.has_value("gitlab.auth.status") {
                event.rename("gitlab.auth.status", "http.response.status_code")?;
            }

            if event.has_value("gitlab.auth.pid") {
                event.rename("gitlab.auth.pid", "process.pid")?;
            }

            let _cond = { event.has_value("gitlab.auth.path") };
            if _cond {
                event.set(
                    "tmp.url_orig",
                    json!(format!(
                        "https://tmp.gitlab{}",
                        event
                            .get("gitlab.auth.path")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if event.has_value("tmp.url_orig") {
                uri_parts(event, "tmp.url_orig", "tmp.url", false, false)?;
            }

            if event.has_value("tmp.url.path") {
                event.rename("tmp.url.path", "url.path")?;
            }

            if event.has_value("tmp.url.query") {
                event.rename("tmp.url.query", "url.query")?;
            }

            if event.has_value("tmp.url.fragment") {
                event.rename("tmp.url.fragment", "url.frament")?;
            }

            event.remove("tmp");
            event.remove("gitlab.auth.path");

            if event.has_value("gitlab.auth.remote_ip") {
                event.rename("gitlab.auth.remote_ip", "client.ip")?;
            }

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

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                if let Some(v) = event.get("client.ip").cloned() {
                    event.set("client.address", v)?;
                }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                if let Some(v) = event.get("client").cloned() {
                    event.set("source", v)?;
                }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.append_unique("event.category", json!("web"))?;

            event.append_unique("event.type", json!("info"))?;

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
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
                event.append_unique("tags", json!("preserve_original_event"))?;
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
            }
        }

        Ok(TransformResult::Continue)
    }
}
