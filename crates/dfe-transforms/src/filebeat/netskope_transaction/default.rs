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

            event.set("event.kind", json!("event"))?;

            if event.has_value("message") {
                event.rename("message", "event.original")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "netskope.transaction")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: // Helper function to convert kebab-case to snake_case\nString kebabToSnake(String str) {\n  return str.replace(\"-\", \"_\");\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = kebabToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nif (ctx.netskope?.transaction != null) {\n  ctx.netskope.transaction = convertToSnakeCase(ctx.netskope.transaction);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Helper function to convert kebab-case to snake_case\nString kebabToSnake(String str) {\n  return str.replace(\"-\", \"_\");\n}\n\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = kebabToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\n\n// Apply the conversion\nif (ctx.netskope?.transaction != null) {\n  ctx.netskope.transaction = convertToSnakeCase(ctx.netskope.transaction);\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_convert_kebab_case_to_snake_case",
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

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == '-' || v == 'N/A' || v == 'NotChecked' || v == 'NotAvailable' || v == 'NoSSL'\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == '-' || v == 'N/A' || v == 'NotChecked' || v == 'NotAvailable' || v == 'NoSSL'\n  });\n}\nhandleMap(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    prune_lists: true,
                    sentinels: vec![
                        "-".into(),
                        "N/A".into(),
                        "NotChecked".into(),
                        "NotAvailable".into(),
                        "NoSSL".into(),
                    ],
                    ..DropPolicy::none()
                },
                None,
            );

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("netskope.transaction.x_transaction_id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if let Some(v) = event
                .get("netskope.transaction.x_policy_action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond =
                { event.has_value("event.action") && event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let mut parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("event.action", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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

            let _cond =
                { event.has_value("event.action") && event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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

            if let Some(v) = event
                .get("netskope.transaction.x_transaction_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            event.append("event.category", json!("network"))?;

            let _cond = { event.get_str("event.action") == Some("allow") };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { event.get_str("event.action") == Some("block") };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.bytes") {
                    if let Some(val) = event.get("netskope.transaction.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_bytes_to_long",
                )?;
                if event.remove("netskope.transaction.bytes").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.bytes".into(),
                    });
                }
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

            let _cond = { event.get_str("netskope.transaction.c_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.c_ip") {
                        if let Some(val) = event.get("netskope.transaction.c_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.c_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.c_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_c_ip_to_ip",
                    )?;
                    if event.remove("netskope.transaction.c_ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.c_ip".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("netskope.transaction.c_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.ip", v)?;
            }

            let _cond = { event.has_value("netskope.transaction.c_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.c_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.cs_bytes") {
                    if let Some(val) = event.get("netskope.transaction.cs_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.cs_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.cs_bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_cs_bytes_to_long",
                )?;
                if event.remove("netskope.transaction.cs_bytes").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.cs_bytes".into(),
                    });
                }
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

            if let Some(v) = event
                .get("netskope.transaction.cs_bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.bytes", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_c_country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.geo.country_name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_c_latitude") {
                    if let Some(val) = event.get("netskope.transaction.x_c_latitude") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_c_latitude".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_c_latitude", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_x_c_latitude_to_double",
                )?;
                if event.remove("netskope.transaction.x_c_latitude").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_c_latitude".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_c_longitude") {
                    if let Some(val) = event.get("netskope.transaction.x_c_longitude") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_c_longitude".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_c_longitude", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_x_c_longitude_to_double",
                )?;
                if event.remove("netskope.transaction.x_c_longitude").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_c_longitude".into(),
                    });
                }
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

            if let Some(v) = event
                .get("netskope.transaction.x_c_latitude")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.geo.location.lat", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_c_longitude")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.geo.location.lon", v)?;
            }

            let _cond = {
                !(event
                    .get("client.geo.location.lat")
                    .is_some_and(|v| v.is_number()))
                    || !(event
                        .get("client.geo.location.lon")
                        .is_some_and(|v| v.is_number()))
                    || event
                        .get_f64("client.geo.location.lat")
                        .is_some_and(|n| n < -90.0)
                    || event
                        .get_f64("client.geo.location.lat")
                        .is_some_and(|n| n > 90.0)
                    || event
                        .get_f64("client.geo.location.lon")
                        .is_some_and(|n| n < -180.0)
                    || event
                        .get_f64("client.geo.location.lon")
                        .is_some_and(|n| n > 180.0)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("client.geo.location").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "client.geo.location".into(),
                        });
                    }
                    Ok(())
                })();
            }

            if let Some(v) = event
                .get("netskope.transaction.x_c_location")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.geo.city_name", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_c_region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.geo.region_name", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_c_zipcode")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.geo.postal_code", v)?;
            }

            if let Some(v) = event
                .get("client.geo")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.rs_status") {
                    if let Some(val) = event.get("netskope.transaction.rs_status") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.rs_status".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.rs_status", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_rs_status_to_long",
                )?;
                if event.remove("netskope.transaction.rs_status").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.rs_status".into(),
                    });
                }
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

            let _cond = { !event.has_value("http.response.status_code") };
            if _cond {
                if let Some(v) = event
                    .get("netskope.transaction.rs_status")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.response.status_code", v)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.sc_status") {
                    if let Some(val) = event.get("netskope.transaction.sc_status") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.sc_status".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.sc_status", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_sc_status_to_long",
                )?;
                if event.remove("netskope.transaction.sc_status").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.sc_status".into(),
                    });
                }
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

            if let Some(v) = event
                .get("netskope.transaction.sc_status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.cs_method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.cs_referer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.referrer", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("netskope.transaction.x_cs_http_version") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("HTTP") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("http.version", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("netskope.transaction.s_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.s_ip") {
                        if let Some(val) = event.get("netskope.transaction.s_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.s_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.s_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_s_ip_to_ip",
                    )?;
                    if event.remove("netskope.transaction.s_ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.s_ip".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("netskope.transaction.s_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server.ip", v)?;
            }

            let _cond = { event.has_value("netskope.transaction.s_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.s_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.sc_bytes") {
                    if let Some(val) = event.get("netskope.transaction.sc_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.sc_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.sc_bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_sc_bytes_to_long",
                )?;
                if event.remove("netskope.transaction.sc_bytes").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.sc_bytes".into(),
                    });
                }
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

            if let Some(v) = event
                .get("netskope.transaction.sc_bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.bytes", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_cs_app_cci") {
                    if let Some(val) = event.get("netskope.transaction.x_cs_app_cci") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_cs_app_cci".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_cs_app_cci", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_x_cs_app_cci_to_long",
                )?;
                if event.remove("netskope.transaction.x_cs_app_cci").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_cs_app_cci".into(),
                    });
                }
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

            let _cond = { event.get_str("netskope.transaction.x_cs_dst_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_cs_dst_ip") {
                        if let Some(val) = event.get("netskope.transaction.x_cs_dst_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.x_cs_dst_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.x_cs_dst_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_x_cs_dst_ip_to_ip",
                    )?;
                    if event.remove("netskope.transaction.x_cs_dst_ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_cs_dst_ip".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("netskope.transaction.x_cs_dst_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("netskope.transaction.x_cs_dst_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_dst_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_cs_dst_port") {
                    if let Some(val) = event.get("netskope.transaction.x_cs_dst_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_cs_dst_port".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_cs_dst_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_x_cs_dst_port_to_long",
                )?;
                if event.remove("netskope.transaction.x_cs_dst_port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_cs_dst_port".into(),
                    });
                }
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

            if let Some(v) = event
                .get("netskope.transaction.x_cs_dst_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.cs_dns")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_s_country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.geo.country_name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_s_latitude") {
                    if let Some(val) = event.get("netskope.transaction.x_s_latitude") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_s_latitude".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_s_latitude", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_x_s_latitude_to_double",
                )?;
                if event.remove("netskope.transaction.x_s_latitude").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_s_latitude".into(),
                    });
                }
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_s_longitude") {
                    if let Some(val) = event.get("netskope.transaction.x_s_longitude") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_s_longitude".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_s_longitude", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_x_s_longitude_to_double",
                )?;
                if event.remove("netskope.transaction.x_s_longitude").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_s_longitude".into(),
                    });
                }
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

            if let Some(v) = event
                .get("netskope.transaction.x_s_latitude")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.geo.location.lat", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_s_longitude")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.geo.location.lon", v)?;
            }

            let _cond = {
                !(event
                    .get("destination.geo.location.lat")
                    .is_some_and(|v| v.is_number()))
                    || !(event
                        .get("destination.geo.location.lon")
                        .is_some_and(|v| v.is_number()))
                    || event
                        .get_f64("destination.geo.location.lat")
                        .is_some_and(|n| n < -90.0)
                    || event
                        .get_f64("destination.geo.location.lat")
                        .is_some_and(|n| n > 90.0)
                    || event
                        .get_f64("destination.geo.location.lon")
                        .is_some_and(|n| n < -180.0)
                    || event
                        .get_f64("destination.geo.location.lon")
                        .is_some_and(|n| n > 180.0)
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("destination.geo.location").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "destination.geo.location".into(),
                        });
                    }
                    Ok(())
                })();
            }

            if let Some(v) = event
                .get("netskope.transaction.x_s_location")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.geo.city_name", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_s_region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.geo.region_name", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_s_zipcode")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.geo.postal_code", v)?;
            }

            if let Some(v) = event
                .get("destination.geo")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server.geo", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.cs_uri_port") {
                    if let Some(val) = event.get("netskope.transaction.cs_uri_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.cs_uri_port".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.cs_uri_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_cs_uri_port_to_long",
                )?;
                if event.remove("netskope.transaction.cs_uri_port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.cs_uri_port".into(),
                    });
                }
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

            if let Some(v) = event
                .get("netskope.transaction.cs_uri_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.port", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.cs_uri_query")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.query", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.cs_uri_scheme")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.scheme", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_cs_uri_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.path", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_cs_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_cs_ip_connect_xff") {
                    if let Some(val) = event.get("netskope.transaction.x_cs_ip_connect_xff") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_cs_ip_connect_xff".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_cs_ip_connect_xff", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_x_cs_ip_connect_xff_to_ip",
                )?;
                if event
                    .remove("netskope.transaction.x_cs_ip_connect_xff")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_cs_ip_connect_xff".into(),
                    });
                }
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

            let _cond = { event.has_value("netskope.transaction.x_cs_ip_connect_xff") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_ip_connect_xff")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("netskope.transaction.x_cs_ip_xff") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_cs_ip_xff") {
                        if let Some(val) = event.get("netskope.transaction.x_cs_ip_xff") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.x_cs_ip_xff".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.x_cs_ip_xff", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_x_cs_ip_xff_to_ip",
                    )?;
                    if event.remove("netskope.transaction.x_cs_ip_xff").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_cs_ip_xff".into(),
                        });
                    }
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

            let _cond = { event.has_value("netskope.transaction.x_cs_ip_xff") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_ip_xff")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("netskope.transaction.x_cs_src_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_cs_src_ip") {
                        if let Some(val) = event.get("netskope.transaction.x_cs_src_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.x_cs_src_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.x_cs_src_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_x_cs_src_ip_to_ip",
                    )?;
                    if event.remove("netskope.transaction.x_cs_src_ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_cs_src_ip".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("netskope.transaction.x_cs_src_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("netskope.transaction.x_cs_src_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_src_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("netskope.transaction.x_cs_src_ip_egress") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_cs_src_ip_egress") {
                        if let Some(val) = event.get("netskope.transaction.x_cs_src_ip_egress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.x_cs_src_ip_egress".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.x_cs_src_ip_egress", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_x_cs_src_ip_egress_to_ip",
                    )?;
                    if event
                        .remove("netskope.transaction.x_cs_src_ip_egress")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_cs_src_ip_egress".into(),
                        });
                    }
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

            let _cond = { event.has_value("netskope.transaction.x_cs_src_ip_egress") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_src_ip_egress")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_cs_src_port") {
                    if let Some(val) = event.get("netskope.transaction.x_cs_src_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_cs_src_port".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_cs_src_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_x_cs_src_port_to_long",
                )?;
                if event.remove("netskope.transaction.x_cs_src_port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_cs_src_port".into(),
                    });
                }
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

            if let Some(v) = event
                .get("netskope.transaction.x_cs_src_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            let _cond = { event.get_str("netskope.transaction.x_cs_userip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_cs_userip") {
                        if let Some(val) = event.get("netskope.transaction.x_cs_userip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.x_cs_userip".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.x_cs_userip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_x_cs_userip_to_ip",
                    )?;
                    if event.remove("netskope.transaction.x_cs_userip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_cs_userip".into(),
                        });
                    }
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

            let _cond = { event.has_value("netskope.transaction.x_cs_userip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_userip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("netskope.transaction.x_sr_dst_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_sr_dst_ip") {
                        if let Some(val) = event.get("netskope.transaction.x_sr_dst_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.x_sr_dst_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.x_sr_dst_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_x_sr_dst_ip_to_ip",
                    )?;
                    if event.remove("netskope.transaction.x_sr_dst_ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_sr_dst_ip".into(),
                        });
                    }
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

            let _cond = { event.has_value("netskope.transaction.x_sr_dst_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_sr_dst_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_sr_dst_port") {
                    if let Some(val) = event.get("netskope.transaction.x_sr_dst_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_sr_dst_port".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_sr_dst_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_x_sr_dst_port_to_long",
                )?;
                if event.remove("netskope.transaction.x_sr_dst_port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_sr_dst_port".into(),
                    });
                }
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

            let _cond = { event.get_str("netskope.transaction.x_sr_src_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_sr_src_ip") {
                        if let Some(val) = event.get("netskope.transaction.x_sr_src_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.x_sr_src_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.x_sr_src_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_x_sr_src_ip_to_ip",
                    )?;
                    if event.remove("netskope.transaction.x_sr_src_ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_sr_src_ip".into(),
                        });
                    }
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

            let _cond = { event.has_value("netskope.transaction.x_sr_src_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_sr_src_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("netskope.transaction.x_sr_src_port") {
                    if let Some(val) = event.get("netskope.transaction.x_sr_src_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "netskope.transaction.x_sr_src_port".into(),
                                message,
                            }
                        })?;
                        event.set("netskope.transaction.x_sr_src_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_netskope_transaction_x_sr_src_port_to_long",
                )?;
                if event.remove("netskope.transaction.x_sr_src_port").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "netskope.transaction.x_sr_src_port".into(),
                    });
                }
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

            let _cond = { event.get_str("netskope.transaction.x_ssl_policy_dst_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_ssl_policy_dst_ip") {
                        if let Some(val) = event.get("netskope.transaction.x_ssl_policy_dst_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.x_ssl_policy_dst_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.x_ssl_policy_dst_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_x_ssl_policy_dst_ip_to_ip",
                    )?;
                    if event
                        .remove("netskope.transaction.x_ssl_policy_dst_ip")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_ssl_policy_dst_ip".into(),
                        });
                    }
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

            let _cond = { event.has_value("netskope.transaction.x_ssl_policy_dst_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_ssl_policy_dst_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("netskope.transaction.x_ssl_policy_src_ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_ssl_policy_src_ip") {
                        if let Some(val) = event.get("netskope.transaction.x_ssl_policy_src_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "netskope.transaction.x_ssl_policy_src_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("netskope.transaction.x_ssl_policy_src_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_netskope_transaction_x_ssl_policy_src_ip_to_ip",
                    )?;
                    if event
                        .remove("netskope.transaction.x_ssl_policy_src_ip")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_ssl_policy_src_ip".into(),
                        });
                    }
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

            let _cond = { event.has_value("netskope.transaction.x_ssl_policy_src_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_ssl_policy_src_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("netskope.transaction.date")
                    && event.get_str("netskope.transaction.date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("netskope.transaction.date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("netskope.transaction.date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "netskope.transaction.date".into(),
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
                        "date_netskope_transaction_date",
                    )?;
                    if event.remove("netskope.transaction.date").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.date".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("netskope.transaction.x_c_local_time")
                    && event.get_str("netskope.transaction.x_c_local_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("netskope.transaction.x_c_local_time")
                    {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                            Some(parsed) => {
                                event.set("netskope.transaction.x_c_local_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "netskope.transaction.x_c_local_time".into(),
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
                        "date_netskope_transaction_x_c_local_time",
                    )?;
                    if event
                        .remove("netskope.transaction.x_c_local_time")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_c_local_time".into(),
                        });
                    }
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

            let _cond = {
                event.has_value("netskope.transaction.x_cs_timestamp")
                    && event.get_str("netskope.transaction.x_cs_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("netskope.transaction.x_cs_timestamp")
                    {
                        match parse_date_out(&date_str, &["epoch_second"], None, None) {
                            Some(parsed) => {
                                event.set("netskope.transaction.x_cs_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "netskope.transaction.x_cs_timestamp".into(),
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
                        "date_netskope_transaction_x_cs_timestamp",
                    )?;
                    if event
                        .remove("netskope.transaction.x_cs_timestamp")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_cs_timestamp".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("netskope.transaction.x_cs_timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("netskope.transaction.x_r_cert_enddate")
                    && event.get_str("netskope.transaction.x_r_cert_enddate") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("netskope.transaction.x_r_cert_enddate")
                    {
                        match parse_date_out(&date_str, &["MMM dd HH:mm:ss yyyy z"], None, None) {
                            Some(parsed) => {
                                event.set("netskope.transaction.x_r_cert_enddate", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "netskope.transaction.x_r_cert_enddate".into(),
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
                        "date_netskope_transaction_x_r_cert_enddate",
                    )?;
                    if event
                        .remove("netskope.transaction.x_r_cert_enddate")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_r_cert_enddate".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("netskope.transaction.x_r_cert_enddate")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.not_after", v)?;
            }

            let _cond = {
                event.has_value("netskope.transaction.x_r_cert_startdate")
                    && event.get_str("netskope.transaction.x_r_cert_startdate") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("netskope.transaction.x_r_cert_startdate")
                    {
                        match parse_date_out(&date_str, &["MMM dd HH:mm:ss yyyy z"], None, None) {
                            Some(parsed) => {
                                event.set("netskope.transaction.x_r_cert_startdate", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "netskope.transaction.x_r_cert_startdate".into(),
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
                        "date_netskope_transaction_x_r_cert_startdate",
                    )?;
                    if event
                        .remove("netskope.transaction.x_r_cert_startdate")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_r_cert_startdate".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("netskope.transaction.x_r_cert_startdate")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.not_before", v)?;
            }

            let _cond = { event.get_str("netskope.transaction.x_cs_app_tags") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_cs_app_tags") {
                        if let Some(s) = event.get_string("netskope.transaction.x_cs_app_tags") {
                            let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            event.set("netskope.transaction.x_cs_app_tags", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
                    if event.remove("netskope.transaction.x_cs_app_tags").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_cs_app_tags".into(),
                        });
                    }
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

            let _cond =
                { event.get_str("netskope.transaction.x_ssl_policy_categories") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_ssl_policy_categories") {
                        if let Some(s) =
                            event.get_string("netskope.transaction.x_ssl_policy_categories")
                        {
                            let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            event.set(
                                "netskope.transaction.x_ssl_policy_categories",
                                Value::Array(parts),
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
                    if event
                        .remove("netskope.transaction.x_ssl_policy_categories")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_ssl_policy_categories".into(),
                        });
                    }
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

            let _cond = { event.get_str("netskope.transaction.x_sr_headers_name") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_sr_headers_name") {
                        if let Some(s) = event.get_string("netskope.transaction.x_sr_headers_name")
                        {
                            let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            event.set(
                                "netskope.transaction.x_sr_headers_name",
                                Value::Array(parts),
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
                    if event
                        .remove("netskope.transaction.x_sr_headers_name")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_sr_headers_name".into(),
                        });
                    }
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

            let _cond = { event.get_str("netskope.transaction.x_sr_headers_value") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("netskope.transaction.x_sr_headers_value") {
                        if let Some(s) = event.get_string("netskope.transaction.x_sr_headers_value")
                        {
                            let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            event.set(
                                "netskope.transaction.x_sr_headers_value",
                                Value::Array(parts),
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
                    if event
                        .remove("netskope.transaction.x_sr_headers_value")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "netskope.transaction.x_sr_headers_value".into(),
                        });
                    }
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

            if let Some(v) = event
                .get("netskope.transaction.x_policy_justification_reason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_cs_app_instance_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.instance.id", v)?;
            }

            let _cond = { event.has_value("netskope.transaction.x_cs_app_instance_id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_app_instance_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.get_str("netskope.transaction.x_cs_app_object_type") == Some("file") };
            if _cond {
                if let Some(v) = event
                    .get("netskope.transaction.x_cs_app_object_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.name", v)?;
                }
            }

            if let Some(v) = event
                .get("netskope.transaction.x_rs_file_md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            let _cond = { event.has_value("netskope.transaction.x_rs_file_md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("netskope.transaction.x_rs_file_md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_rs_file_sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.has_value("netskope.transaction.x_rs_file_sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("netskope.transaction.x_rs_file_sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_c_os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_cs_sni")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.server_name", v)?;
            }

            let _cond = { event.has_value("netskope.transaction.x_cs_sni") };
            if _cond {
                if let Some(v) = event
                    .get("netskope.transaction.x_cs_sni")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.domain", v)?;
                }
            }

            if let Some(v) = event
                .get("netskope.transaction.x_cs_ssl_cipher")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.cipher", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_cs_ssl_ja3")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.ja3", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("netskope.transaction.x_cs_ssl_version") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("v") else {
                            break 'dissect false;
                        };
                        captured.push(("tls.version_protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("v") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("tls.version", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("tls.version_protocol") {
                map_strings(
                    event,
                    "tls.version_protocol",
                    "tls.version_protocol",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("netskope.transaction.x_r_cert_valid") == Some("yes") };
            if _cond {
                event.set("tls.established", json!(true))?;
            }

            let _cond = { event.get_str("netskope.transaction.x_r_cert_valid") == Some("no") };
            if _cond {
                event.set("tls.established", json!(false))?;
            }

            if let Some(v) = event
                .get("netskope.transaction.x_sr_ssl_ja3s")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.server.ja3s", v)?;
            }

            if let Some(v) = event
                .get("netskope.transaction.cs_username")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("netskope.transaction.cs_username") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("netskope.transaction.cs_username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("netskope.transaction.x_cs_app_to_user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_app_to_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("netskope.transaction.x_cs_app_from_user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_app_from_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("netskope.transaction.x_cs_connect_host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("netskope.transaction.x_cs_connect_host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("netskope.transaction.x_policy_dst_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_policy_dst_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("netskope.transaction.x_policy_src_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netskope.transaction.x_policy_src_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("netskope.transaction.x_ssl_policy_dst_host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("netskope.transaction.x_ssl_policy_dst_host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("netskope.transaction.cs_user_agent")
                    && event.get_str("netskope.transaction.cs_user_agent") != Some("")
            };
            if _cond {
                if let Some(ua_str) = event.get_string("netskope.transaction.cs_user_agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            let _cond = { event.has_value("aws.s3.bucket") && event.has_value("aws.s3.object") };
            if _cond {
                event.remove("log.file.path");
                event.remove("log.offset");
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("netskope.transaction.c_ip");
                event.remove("netskope.transaction.cs_bytes");
                event.remove("netskope.transaction.cs_dns");
                event.remove("netskope.transaction.cs_method");
                event.remove("netskope.transaction.cs_referer");
                event.remove("netskope.transaction.cs_uri_port");
                event.remove("netskope.transaction.cs_uri_query");
                event.remove("netskope.transaction.cs_uri_scheme");
                event.remove("netskope.transaction.cs_username");
                event.remove("netskope.transaction.rs_status");
                event.remove("netskope.transaction.s_ip");
                event.remove("netskope.transaction.sc_bytes");
                event.remove("netskope.transaction.sc_status");
                event.remove("netskope.transaction.x_c_country");
                event.remove("netskope.transaction.x_c_location");
                event.remove("netskope.transaction.x_c_os");
                event.remove("netskope.transaction.x_c_region");
                event.remove("netskope.transaction.x_c_zipcode");
                event.remove("netskope.transaction.x_cs_app_instance_id");
                event.remove("netskope.transaction.x_cs_app_object_name");
                event.remove("netskope.transaction.x_cs_dst_ip");
                event.remove("netskope.transaction.x_cs_dst_port");
                event.remove("netskope.transaction.x_cs_http_version");
                event.remove("netskope.transaction.x_cs_sni");
                event.remove("netskope.transaction.x_cs_src_port");
                event.remove("netskope.transaction.x_cs_ssl_cipher");
                event.remove("netskope.transaction.x_cs_ssl_ja3");
                event.remove("netskope.transaction.x_cs_ssl_version");
                event.remove("netskope.transaction.x_cs_timestamp");
                event.remove("netskope.transaction.x_cs_uri_path");
                event.remove("netskope.transaction.x_cs_url");
                event.remove("netskope.transaction.x_policy_action");
                event.remove("netskope.transaction.x_policy_justification_reason");
                event.remove("netskope.transaction.x_r_cert_enddate");
                event.remove("netskope.transaction.x_r_cert_startdate");
                event.remove("netskope.transaction.x_r_cert_valid");
                event.remove("netskope.transaction.x_rs_file_md5");
                event.remove("netskope.transaction.x_rs_file_sha256");
                event.remove("netskope.transaction.x_s_country");
                event.remove("netskope.transaction.x_s_location");
                event.remove("netskope.transaction.x_s_region");
                event.remove("netskope.transaction.x_s_zipcode");
                event.remove("netskope.transaction.x_cs_src_ip");
                event.remove("netskope.transaction.x_sr_ssl_ja3s");
                event.remove("netskope.transaction.x_transaction_id");
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
