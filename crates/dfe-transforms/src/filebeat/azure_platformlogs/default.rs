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

            if event.has("azure") {
                event.rename("azure", "azure-eventhub")?;
            }

            event.remove("routing.category");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: ctx.message = ctx.message.replace(params.empty_field_name, '')
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"ctx.message = ctx.message.replace(params.empty_field_name, '')"#
                    ),
                    cached_params!("{\"empty_field_name\":\"\\\"\\\":\\\"\\\",\"}"),
                )?;
                Ok(())
            })();

            if event.has("azureLogForwarder") && !event.has("azure_log_forwarder") {
                event.rename("azureLogForwarder", "azure_log_forwarder")?;
            }

            if event.has("azure_log_forwarder.resourceType")
                && !event.has("azure_log_forwarder.resource_type")
            {
                event.rename(
                    "azure_log_forwarder.resourceType",
                    "azure_log_forwarder.resource_type",
                )?;
            }

            if event.has("azure_log_forwarder.serviceProvider")
                && !event.has("azure_log_forwarder.service_provider")
            {
                event.rename(
                    "azure_log_forwarder.serviceProvider",
                    "azure_log_forwarder.service_provider",
                )?;
            }

            if event.has("dataStream") && !event.has("data_stream") {
                event.rename("dataStream", "data_stream")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("timestamp") {
                    if let Some(parsed) = parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                        ],
                        None,
                        None,
                    ) {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();

            event.remove("timestamp");

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

            // on_failure: 5 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(s) = event.get_string("event.original") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("azure.platformlogs", parsed)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("event.original") {
                        if let Some(input) = event.get_string("event.original") {
                            // Grok pattern: resourceId\": ?\"%{DATA:azure.platformlogs.resourceId}\"
                            if !cached_grok!(
                                "resourceId\": ?\"%{DATA:azure.platformlogs.resourceId}\""
                            )
                            .extract_into(&input, event)?
                            {}
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("event.original") {
                        if let Some(input) = event.get_string("event.original") {
                            // Grok pattern: category\": ?\"%{DATA:azure.platformlogs.category}\"
                            if !cached_grok!("category\": ?\"%{DATA:azure.platformlogs.category}\"")
                                .extract_into(&input, event)?
                            {}
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("event.original") {
                        if let Some(input) = event.get_string("event.original") {
                            // Grok pattern: time\": ?\"%{DATA:azure.platformlogs.time}\"
                            if !cached_grok!("time\": ?\"%{DATA:azure.platformlogs.time}\"")
                                .extract_into(&input, event)?
                            {}
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("error.message", json!("Received invalid json from the Azure Cloud platform. Unable to parse the source log message"))?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append("tags", json!("preserve_original_event"))?;
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event
                    .get("azure.platformlogs.properties")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "temp_properties",
                        json!(
                            event
                                .get("azure.platformlogs.properties")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("temp_properties") };
            if _cond {
                event.remove("azure.platformlogs.properties");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx?.temp_properties != null) {\n  ctx.temp_properties = ctx.temp_properties.replace(\"'\", \"\\\"\");\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx?.temp_properties != null) {\n  ctx.temp_properties = ctx.temp_properties.replace(\"'\", \"\\\"\");\n}"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("temp_properties") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("temp_properties") {
                        let parsed: Value =
                            serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                                path: "temp_properties".into(),
                                message: format!("failed to parse JSON: {}", e),
                            })?;
                        event.set("azure.platformlogs.properties", parsed)?;
                    }
                    Ok(())
                })();
            }

            event.remove("temp_properties");

            if event.has("azure.platformlogs.Identity") {
                event.rename("azure.platformlogs.Identity", "azure.platformlogs.identity")?;
            }

            let _cond = {
                event
                    .get("azure.platformlogs.identity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has("azure.platformlogs.identity") {
                    event.rename(
                        "azure.platformlogs.identity",
                        "azure.platformlogs.identity_name",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("azure.platformlogs.time") {
                    if let Some(parsed) = parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                        ],
                        None,
                        None,
                    ) {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("azure.platformlogs.EventTimeString") {
                    if let Some(parsed) = parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "M/d/yyyy h:mm:ss a XXX",
                            "M/d/yyyy h:mm:ss a",
                            "M/d/yyyy H:mm:ss",
                            "yyyy-MM-dd'T'H:mm:ss.SSS'Z'",
                        ],
                        None,
                        None,
                    ) {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();

            event.remove("azure.platformlogs.time");

            let _cond = { !event.has_value("azure.resource_id") };
            if _cond {
                if event.has("azure.platformlogs.resourceId") {
                    event.rename("azure.platformlogs.resourceId", "azure.resource_id")?;
                }
            }

            let _cond = { !event.has_value("azure.resource_id") };
            if _cond {
                if event.has("azure.platformlogs.ResourceId") {
                    event.rename("azure.platformlogs.ResourceId", "azure.resource_id")?;
                }
            }

            if event.has("azure.platformlogs.Region") {
                event.rename("azure.platformlogs.Region", "cloud.region")?;
            }

            if event.has("azure.platformlogs.Host") {
                event.rename("azure.platformlogs.Host", "host.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("azure.platformlogs.EventProperties") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "azure.platformlogs.EventProperties".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("azure.platformlogs.properties", parsed)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("azure.platformlogs.properties") };
            if _cond {
                event.remove("azure.platformlogs.EventProperties");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("azure.platformlogs.properties.log") {
                    let parsed: Value =
                        serde_json::from_str(&s).map_err(|e| TransformError::ParseError {
                            path: "azure.platformlogs.properties.log".into(),
                            message: format!("failed to parse JSON: {}", e),
                        })?;
                    event.set("azure.platformlogs.properties.log", parsed)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("azure.platformlogs.properties.log")
                    && event
                        .get("azure.platformlogs.properties.log")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has("azure.platformlogs.properties.log") {
                    event.rename("azure.platformlogs.properties.log", "message")?;
                }
            }

            if event.has("azure.platformlogs.EventName") {
                event.rename("azure.platformlogs.EventName", "event.action")?;
            }

            if event.has("azure.platformlogs.EventIpAddress") {
                event.rename(
                    "azure.platformlogs.EventIpAddress",
                    "azure.platformlogs.callerIpAddress",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has("azure.platformlogs.callerIpAddress") {
                    if let Some(s) = event.get_string("azure.platformlogs.callerIpAddress") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "azure.platformlogs.callerIpAddress".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("source.ip", s)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("azure.platformlogs.callerIpAddress") {
                        event.rename("azure.platformlogs.callerIpAddress", "source.address")?;
                    }
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.remove("azure.platformlogs.callerIpAddress");
            }

            let v = json!(
                event
                    .get("source.ip")
                    .map_or_else(String::new, painless_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("client.ip", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("azure.platformlogs.level") {
                event.rename("azure.platformlogs.level", "log.level")?;
            }

            let _cond = {
                event.has_value("azure.platformlogs.durationMs")
                    && event
                        .get("azure.platformlogs.durationMs")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("azure.platformlogs.durationMs") {
                        if let Some(val) = event.get("azure.platformlogs.durationMs") {
                            let converted = match val {
                                Value::String(s) => {
                                    let s = s.trim();
                                    if let Some(hex) = s.strip_prefix("0x") {
                                        json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                            TransformError::ParseError {
                                                path: "azure.platformlogs.durationMs".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    } else {
                                        json!(s.parse::<i64>().map_err(|_| {
                                            TransformError::ParseError {
                                                path: "azure.platformlogs.durationMs".into(),
                                                message: format!(
                                                    "cannot convert '{}' to integer",
                                                    s
                                                ),
                                            }
                                        })?)
                                    }
                                }
                                Value::Number(n) => {
                                    json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                                }
                                Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                                _ => {
                                    return Err(TransformError::ParseError {
                                        path: "azure.platformlogs.durationMs".into(),
                                        message: "cannot convert to integer".into(),
                                    });
                                }
                            };
                            event.set("event.duration", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            event.remove("azure.platformlogs.durationMs");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.event.duration!= null) {ctx.event.duration = ctx.event.duration * params.param_nano;}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_params(
                    event,
                    cached_script!(
                        r#"if (ctx.event.duration!= null) {ctx.event.duration = ctx.event.duration * params.param_nano;}"#
                    ),
                    cached_params!("{\"param_nano\":1000000}"),
                )?;
                Ok(())
            })();

            if event.has("azure.platformlogs.location") {
                event.rename("azure.platformlogs.location", "geo.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx?.azure?.platformlogs?.properties?.eventCategory != null) {\n  ctx.azure.platformlogs.event_category = ctx.azure.platformlogs.properties.eventCategory;\n} else if (ctx?.azure?.platformlogs?.properties?.policies != null)  {\n  ctx.azure.platformlogs.event_category = 'Policy';\n} else {\n  ctx.azure.platformlogs.event_category = 'Administrative';\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"if (ctx?.azure?.platformlogs?.properties?.eventCategory != null) {\n  ctx.azure.platformlogs.event_category = ctx.azure.platformlogs.properties.eventCategory;\n} else if (ctx?.azure?.platformlogs?.properties?.policies != null)  {\n  ctx.azure.platformlogs.event_category = 'Policy';\n} else {\n  ctx.azure.platformlogs.event_category = 'Administrative';\n}"#
                    ),
                )?;
                Ok(())
            })();

            if event.has("azure.platformlogs.resultType") {
                event.rename(
                    "azure.platformlogs.resultType",
                    "azure.platformlogs.result_type",
                )?;
            }

            let _cond = {
                event.has_value("azure.platformlogs.result_type")
                    && event
                        .get("azure.platformlogs.result_type")
                        .is_some_and(|v| v.is_string())
                    && (event
                        .get_str("azure.platformlogs.result_type")
                        .is_some_and(|s| s.to_lowercase() == "success")
                        || event
                            .get_str("azure.platformlogs.result_type")
                            .is_some_and(|s| s.to_lowercase() == "failure"))
            };
            if _cond {
                if let Some(val) = event.get("azure.platformlogs.result_type") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("event.outcome", converted)?;
                }
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.has_value("azure.platformlogs.properties.result")
                    && event
                        .get("azure.platformlogs.properties.result")
                        .is_some_and(|v| v.is_string())
                    && ["success", "failure", "unknown"].contains(
                        &event
                            .get_str("azure.platformlogs.properties.result")
                            .unwrap_or(""),
                    )
            };
            if _cond {
                if let Some(val) = event.get("azure.platformlogs.properties.result") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("event.outcome", converted)?;
                }
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.has_value("azure.platformlogs.Status")
                    && event
                        .get("azure.platformlogs.Status")
                        .is_some_and(|v| v.is_string())
                    && ["success", "failure", "unknown", "Succeeded", "Failed"]
                        .contains(&event.get_str("azure.platformlogs.Status").unwrap_or(""))
            };
            if _cond {
                if let Some(val) = event.get("azure.platformlogs.Status") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("event.outcome", converted)?;
                }
            }

            let _cond = {
                event.has_value("event.outcome")
                    && event.get_str("event.outcome") == Some("Succeeded")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("event.outcome") && event.get_str("event.outcome") == Some("Failed")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has("azure.platformlogs.operationName") {
                event.rename(
                    "azure.platformlogs.operationName",
                    "azure.platformlogs.operation_name",
                )?;
            }

            if event.has("azure.platformlogs.operation_name") {
                if let Some(val) = event.get("azure.platformlogs.operation_name") {
                    let converted = match val {
                        Value::String(_) => val.clone(),
                        Value::Number(n) => json!(n.to_string()),
                        Value::Bool(b) => json!(b.to_string()),
                        Value::Null => json!("null"),
                        _ => json!(val.to_string()),
                    };
                    event.set("event.action", converted)?;
                }
            }

            if event.has("azure.platformlogs.resultSignature") {
                event.rename(
                    "azure.platformlogs.resultSignature",
                    "azure.platformlogs.result_signature",
                )?;
            }

            if event.has("azure.platformlogs.correlationId") {
                event.rename("azure.platformlogs.correlationId", "azure.correlation_id")?;
            }

            if event.has("azure.platformlogs.properties.statusCode") {
                event.rename(
                    "azure.platformlogs.properties.statusCode",
                    "azure.platformlogs.properties.status_code",
                )?;
            }

            if event.has("azure.platformlogs.Status") {
                event.rename("azure.platformlogs.Status", "azure.platformlogs.status")?;
            }

            if event.has("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("geo.location", v.clone())?;
                        }
                    }
                }
            }

            // Painless script
            // Source: if (ctx?.azure?.platformlogs?.category == null) {\n  return;\n} def category = ctx.azure.platformlogs.category.toLowerCase(); if (params.get(category) == null) {\n  return;\n} def hm = new HashMap(params.get(category)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_params(
                event,
                cached_script!(
                    r#"if (ctx?.azure?.platformlogs?.category == null) {\n  return;\n} def category = ctx.azure.platformlogs.category.toLowerCase(); if (params.get(category) == null) {\n  return;\n} def hm = new HashMap(params.get(category)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"action\":{\"type\":[\"change\"]},\"delete\":{\"type\":[\"deletion\"]},\"read\":{\"type\":[\"access\"]},\"write\":{\"type\":[\"change\"]}}"
                ),
            )?;

            if event.has("source.ip") {
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

            if event.has("source.ip") {
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.kind", json!("event"))?;
                Ok(())
            })();

            // Begin nested pipeline: "azure-shared-pipeline"
            event.set("cloud.provider", json!("azure"))?;
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("azure.resource_id") {
                    // Grok pattern: /(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))
                    if !cached_grok_mapped!("/(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_namespace", "azure.resource.namespace"), ("azure_resource_authorization_rule", "azure.resource.authorization_rule")]).extract_into(&input, event)? {
                }
                }
                Ok(())
            })();
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                        if !cached_grok_mapped!("/(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                        if !cached_grok_mapped!("/(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group"), ("azure_resource_provider", "azure.resource.provider"), ("azure_resource_name", "azure.resource.name")]).extract_into(&input, event)? {
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /providers/(?P<azure_resource_provider>(?:.+))
                        if !cached_grok_mapped!(
                            "/providers/(?P<azure_resource_provider>(?:.+))",
                            [("azure_resource_provider", "azure.resource.provider")]
                        )
                        .extract_into(&input, event)?
                        {
                            // Grok pattern: /PROVIDERS/(?P<azure_resource_provider>(?:.+))
                            if !cached_grok_mapped!(
                                "/PROVIDERS/(?P<azure_resource_provider>(?:.+))",
                                [("azure_resource_provider", "azure.resource.provider")]
                            )
                            .extract_into(&input, event)?
                            {}
                        }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                        if !cached_grok_mapped!("/(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_provider", "azure.resource.provider")]).extract_into(&input, event)? {
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))
                        if !cached_grok_mapped!("/(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))", [("azure_subscription_id", "azure.subscription_id"), ("azure_resource_group", "azure.resource.group")]).extract_into(&input, event)? {
                }
                    }
                    Ok(())
                })();
            }
            let _cond = { !event.has_value("azure.subscription_id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("azure.resource_id") {
                        // Grok pattern: /(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                        if !cached_grok_mapped!("/(?i)subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))", [("azure_subscription_id", "azure.subscription_id")]).extract_into(&input, event)? {
                }
                    }
                    Ok(())
                })();
            }
            if event.has("azure.resource_id") {
                event.rename("azure.resource_id", "azure.resource.id")?;
            }
            if event.has("event.outcome") {
                if let Some(s) = event.get_string("event.outcome") {
                    let lowered = s.to_lowercase();
                    event.set("event.outcome", lowered)?;
                }
            }
            // End nested pipeline: "azure-shared-pipeline"

            let _cond = {
                event.has_value("azure_log_forwarder.resource_type")
                    && (event.get_str("azure_log_forwarder.resource_type")
                        == Some("Microsoft.AppPlatform/Spring")
                        || event.get_str("azure_log_forwarder.resource_type")
                            == Some("MICROSOFT.APPPLATFORM/SPRING"))
            };
            if _cond {
                // Begin nested pipeline: "springcloudlogs-inner-pipeline"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx['_index'] = ctx['_index'].replace('platformlogs', 'springcloudlogs')
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"ctx['_index'] = ctx['_index'].replace('platformlogs', 'springcloudlogs')"#
                        ),
                    )?;
                    Ok(())
                })();
                if event.has("azure.platformlogs") {
                    event.rename("azure.platformlogs", "azure.springcloudlogs")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.dataset", json!("azure.springcloudlogs"))?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("data_stream.dataset", json!("azure.springcloudlogs"))?;
                    Ok(())
                })();
                let _cond = {
                    event.get_str("azure.springcloudlogs.category") != Some("SystemLogs")
                        && event.get_str("azure.springcloudlogs.category")
                            != Some("ApplicationConsole")
                        && event.get_str("azure.springcloudlogs.category") != Some("IngressLogs")
                        && event.get_str("azure.springcloudlogs.category") != Some("BuildLogs")
                        && event.get_str("azure.springcloudlogs.category")
                            != Some("ContainerEventLogs")
                };
                if _cond {
                    return Ok(TransformResult::Drop);
                }
                if event.has("azure.springcloudlogs.LogFormat") {
                    event.rename(
                        "azure.springcloudlogs.LogFormat",
                        "azure.springcloudlogs.log_format",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.InstanceName") {
                    event.rename(
                        "azure.springcloudlogs.properties.InstanceName",
                        "azure.springcloudlogs.properties.instance_name",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.Log") {
                    event.rename(
                        "azure.springcloudlogs.properties.Log",
                        "azure.springcloudlogs.properties.log",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.ServiceName") {
                    event.rename(
                        "azure.springcloudlogs.properties.ServiceName",
                        "azure.springcloudlogs.properties.service_name",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.Stream") {
                    event.rename(
                        "azure.springcloudlogs.properties.Stream",
                        "azure.springcloudlogs.properties.stream",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.AppName") {
                    event.rename(
                        "azure.springcloudlogs.properties.AppName",
                        "azure.springcloudlogs.properties.app_name",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.ServiceId") {
                    event.rename(
                        "azure.springcloudlogs.properties.ServiceId",
                        "azure.springcloudlogs.properties.service_id",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.Type") {
                    event.rename(
                        "azure.springcloudlogs.properties.Type",
                        "azure.springcloudlogs.properties.type",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.Level") {
                    event.rename(
                        "azure.springcloudlogs.properties.Level",
                        "azure.springcloudlogs.level",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.Logger") {
                    event.rename(
                        "azure.springcloudlogs.properties.Logger",
                        "azure.springcloudlogs.properties.logger",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.Stack") {
                    event.rename(
                        "azure.springcloudlogs.properties.Stack",
                        "azure.springcloudlogs.properties.stack",
                    )?;
                }
                if event.has("azure.springcloudlogs.properties.Thread") {
                    event.rename(
                        "azure.springcloudlogs.properties.Thread",
                        "azure.springcloudlogs.properties.thread",
                    )?;
                }
                if event.has("azure.springcloudlogs.level") {
                    event.rename("azure.springcloudlogs.level", "log.level")?;
                }
                if event.has("azure.springcloudlogs.operationName") {
                    event.rename(
                        "azure.springcloudlogs.operationName",
                        "azure.springcloudlogs.operation_name",
                    )?;
                }
                if event.has("azure.springcloudlogs.operation_name") {
                    if let Some(val) = event.get("azure.springcloudlogs.operation_name") {
                        let converted = match val {
                            Value::String(_) => val.clone(),
                            Value::Number(n) => json!(n.to_string()),
                            Value::Bool(b) => json!(b.to_string()),
                            Value::Null => json!("null"),
                            _ => json!(val.to_string()),
                        };
                        event.set("event.action", converted)?;
                    }
                }
                // End nested pipeline: "springcloudlogs-inner-pipeline"
            }

            if event.has("azure.platformlogs.resultDescription") {
                event.rename(
                    "azure.platformlogs.resultDescription",
                    "azure.platformlogs.result_description",
                )?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                        "Processor '{}' {}with tag '{}' {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("#_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("/_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
