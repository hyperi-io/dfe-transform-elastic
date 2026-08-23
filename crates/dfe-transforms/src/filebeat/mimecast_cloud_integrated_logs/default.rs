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
            let _cond = { event.get_str("message") == Some("want_more") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

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

            parse_json_field(event, "event.original", "mimecast")?;

            let _cond =
                { !event.has_value("mimecast.datetime") && !event.has_value("mimecast.timestamp") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("event.category", Value::Array(vec![json!("email")]))?;

            let _cond = { event.has_value("mimecast.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("mimecast.timestamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            // SKIPPED: condition not transpiled: ctx['@timestamp'] != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.created", v)?;
                }
            }

            let _cond = {
                event.get("mimecast.tags").is_some_and(|v| v.is_array()) && event.get("mimecast.tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } != 0)
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has("mimecast.type") {
                event.rename("mimecast.type", "mimecast.log_type")?;
            }

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                    serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"),
                    _ => false,
                })
            };
            if _cond {
                if let Some(v) = event
                    .get("mimecast")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("original", v)?;
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("mimecast.messageId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.processingId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.aggregateId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.accountId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.action") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.log_type") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("mimecast.subtype") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event
                    .get("mimecast.recipients")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("mimecast.recipients").cloned() {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "email.to.address",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("mimecast.recipients", Value::Array(out))?;
                }
            }

            let _cond = {
                event.has_value("mimecast.recipients")
                    && !(event
                        .get("mimecast.recipients")
                        .is_some_and(|v| v.is_array()))
            };
            if _cond {
                event.append_unique(
                    "email.to.address",
                    json!(
                        event
                            .get("mimecast.recipients")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("mimecast.senderIp") {
                event.rename("mimecast.senderIp", "source.ip")?;
            }

            if event.has("mimecast.messageId") {
                event.rename("mimecast.messageId", "email.message_id")?;
            }

            let _cond = { event.has_value("mimecast.senderHeader") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.senderHeader")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("mimecast.senderEnvelope") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.senderEnvelope")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("mimecast.subject") {
                event.rename("mimecast.subject", "email.subject")?;
            }

            if event.has("mimecast.direction") {
                event.rename("mimecast.direction", "email.direction")?;
            }

            if event.has_value("email.direction") {
                if let Some(s) = event.get_string("email.direction") {
                    let lowered = s.to_lowercase();
                    event.set("email.direction", lowered)?;
                }
            }

            if event.has("mimecast.attachments") {
                event.rename("mimecast.attachments", "email.attachments.file.name")?;
            }

            if event.has("mimecast.action") {
                event.rename("mimecast.action", "mimecast.threatState")?;
            }

            if event.has("mimecast.action") {
                event.rename("mimecast.action", "event.action")?;
            }

            if event.has("mimecast.sourceIp") {
                event.rename("mimecast.sourceIp", "source.ip")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.ip") {
                    if let Some(val) = event.get("source.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("source.ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "source.ip".into(),
                        });
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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = {
                event
                    .get("email.from.address")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("email.from.address").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("email.from.address", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get("email.to.address").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(Value::Array(items)) = event.get("email.to.address").cloned() {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("email.to.address", Value::Array(out))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("email.from.address")
                    && !(event
                        .get("email.from.address")
                        .is_some_and(|v| v.is_array()))
                    && event.get_str("email.from.address") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("email.from.address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
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

            let _cond = {
                event
                    .get("email.attachments.file.hash")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                if let Some(Value::Array(items)) = event.get("email.attachments.file.hash").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("email.attachments.file.hash", Value::Array(out))?;
                }
            }

            let _cond = { event.has_value("email.attachments") };
            if _cond {
                // Painless script
                // Source: def attachments = [];\nattachments.add(ctx.email.attachments);\nctx.email.attachments = attachments;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def attachments = [];\nattachments.add(ctx.email.attachments);\nctx.email.attachments = attachments;\n"#
                    ),
                )?;
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
                event.remove("mimecast.recipients");
                event.remove("mimecast.senderEnvelope");
            }

            if event.has("original") {
                event.rename("original", "mimecast")?;
            }

            event.remove("mimecast._offset");
            event.remove("mimecast._partition");
            event.remove("mimecast.timestamp");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

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
