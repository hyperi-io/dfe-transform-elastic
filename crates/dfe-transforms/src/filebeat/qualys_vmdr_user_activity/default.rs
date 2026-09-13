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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

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
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "qualys_vmdr.user_activity")?;

            let _cond = {
                event
                    .get("qualys_vmdr.user_activity")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: String underscore(String s) {\n  return /[ -]/.matcher(s).replaceAll('_');\n}\n\ndef out = [:];\nfor (def item : ctx.qualys_vmdr.user_activity.entrySet()) {\n  out[underscore(item.getKey())] = item.getValue();\n}\nctx.qualys_vmdr.user_activity = out;\n
                rewrite_keys(
                    event,
                    &RewriteKeys::new(
                        "qualys_vmdr.user_activity".into(),
                        "qualys_vmdr.user_activity".into(),
                        vec![KeyRewriteStep::ReplaceChars(" -".into(), Some('_'))],
                    ),
                );
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "event.original".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if let Some(date_str) = event.get_as_string("qualys_vmdr.user_activity.Date") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "qualys_vmdr.user_activity.Date".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if let Some(v) = event
                .get("qualys_vmdr.user_activity.Action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.user_activity.Module")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.user_activity.Details")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.user_activity.User_Name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.append(
                "user.roles",
                json!(
                    event
                        .get("qualys_vmdr.user_activity.User_Role")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("qualys_vmdr.user_activity.User_IP") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "qualys_vmdr.user_activity.User_IP".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
                Ok(())
            })();

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
                event.remove("qualys_vmdr.user_activity.Date");
                event.remove("qualys_vmdr.user_activity.Action");
                event.remove("qualys_vmdr.user_activity.Module");
                event.remove("qualys_vmdr.user_activity.Details");
                event.remove("qualys_vmdr.user_activity.User_Name");
                event.remove("qualys_vmdr.user_activity.User_Role");
                event.remove("qualys_vmdr.user_activity.User_IP");
            }

            let _cond = {
                event
                    .get("qualys_vmdr.user_activity")
                    .is_some_and(|v| v.is_object())
                    && event
                        .get("qualys_vmdr.user_activity")
                        .is_some_and(|v| match v {
                            serde_json::Value::String(s) => s.is_empty(),
                            serde_json::Value::Array(a) => a.is_empty(),
                            serde_json::Value::Object(o) => o.is_empty(),
                            serde_json::Value::Null => true,
                            _ => false,
                        })
            };
            if _cond {
                if event.remove("qualys_vmdr.user_activity").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "qualys_vmdr.user_activity".into(),
                    });
                }
            }

            let _cond = {
                event.get("qualys_vmdr").is_some_and(|v| v.is_object())
                    && event.get("qualys_vmdr").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                if event.remove("qualys_vmdr").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "qualys_vmdr".into(),
                    });
                }
            }

            event.remove("cloud");
            event.remove("host");

            event.set("event.kind", json!("event"))?;

            let _cond = { event.get_str("event.action") == Some("login") };
            if _cond {
                event.append_unique("event.category", json!("authentication"))?;
            }

            let _cond = { event.get_str("event.action") == Some("login") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("event.action") == Some("request") };
            if _cond {
                event.append_unique("event.category", json!("api"))?;
            }

            let _cond = { event.get_str("event.action") == Some("request") };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("add")
                    || event.get_str("event.action") == Some("create")
                    || event.get_str("event.action") == Some("set")
            };
            if _cond {
                event.append_unique("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("add")
                    || event.get_str("event.action") == Some("set")
            };
            if _cond {
                event.append_unique("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("event.action") == Some("create") };
            if _cond {
                event.append_unique("event.type", json!("creation"))?;
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
                        "Processor '{}' {}failed with message '{}'",
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
