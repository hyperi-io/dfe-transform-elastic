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
            event.set("ecs.version", json!("9.4.0"))?;

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

            let _cond = { !event.has_value("json") && event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            let _cond = { event.has_value("json.snapshot") };
            if _cond {
                event.set("event.action", json!("snapshot"))?;
            }

            let _cond = { event.has_value("json.diffResults") };
            if _cond {
                event.set("event.action", json!("differential"))?;
            }

            if let Some(v) = event
                .get("json.numerics")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("kolide.osquery_result.numerics", v)?;
            }

            let _cond = { event.has_value("json.unixTime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.unixTime") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.unixTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed to parse json.unixTime: {}",
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

            if event.has_value("json.calendarTime") {
                event.rename("json.calendarTime", "kolide.osquery_result.calendar_time")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.name") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("pack:kolide_log_pipeline:") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("kolide.osquery_result.pack_name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("kolide.osquery_result.query_name", remaining));
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

            if let Some(v) = event
                .get("kolide.osquery_result.query_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.epoch") {
                event.rename("json.epoch", "kolide.osquery_result.epoch")?;
            }

            if event.has_value("json.counter") {
                event.rename("json.counter", "kolide.osquery_result.counter")?;
            }

            if let Some(v) = event
                .get("kolide.osquery_result.counter")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.sequence", v)?;
            }

            if event.has_value("json.hostIdentifier") {
                event.rename(
                    "json.hostIdentifier",
                    "kolide.osquery_result.host_identifier",
                )?;
            }

            if event.has_value("json.request_id") {
                event.rename("json.request_id", "kolide.osquery_result.request_id")?;
            }

            if event.has_value("json.kolide_decorations.device_id") {
                if let Some(val) = event.get("json.kolide_decorations.device_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.kolide_decorations.device_id".into(),
                            message,
                        }
                    })?;
                    event.set("host.id", converted)?;
                }
            }

            if event.has_value("json.kolide_decorations.device_display_name") {
                event.rename("json.kolide_decorations.device_display_name", "host.name")?;
            }

            if let Some(v) = event
                .get("host.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if event.has_value("json.kolide_decorations.serial_number") {
                event.rename(
                    "json.kolide_decorations.serial_number",
                    "kolide.osquery_result.device_serial",
                )?;
            }

            if event.has_value("json.kolide_decorations.hardware_uuid") {
                event.rename(
                    "json.kolide_decorations.hardware_uuid",
                    "kolide.osquery_result.hardware_uuid",
                )?;
            }

            if event.has_value("json.kolide_decorations.enrolled_at") {
                event.rename(
                    "json.kolide_decorations.enrolled_at",
                    "kolide.osquery_result.enrolled_at",
                )?;
            }

            if event.has_value("json.kolide_decorations.device_registered_at") {
                event.rename(
                    "json.kolide_decorations.device_registered_at",
                    "kolide.osquery_result.device_registered_at",
                )?;
            }

            if event.has_value("json.kolide_decorations.device_registered_owner_id") {
                if let Some(val) = event.get("json.kolide_decorations.device_registered_owner_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.kolide_decorations.device_registered_owner_id".into(),
                            message,
                        }
                    })?;
                    event.set("user.id", converted)?;
                }
            }

            if event.has_value("json.kolide_decorations.device_registered_owner_name") {
                event.rename(
                    "json.kolide_decorations.device_registered_owner_name",
                    "user.full_name",
                )?;
            }

            if event.has_value("json.kolide_decorations.device_registered_owner_email") {
                event.rename(
                    "json.kolide_decorations.device_registered_owner_email",
                    "user.email",
                )?;
            }

            let _cond = { event.has_value("json.kolide_decorations.remote_ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("json.kolide_decorations.remote_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.decorations.hardware_vendor") {
                event.rename("json.decorations.hardware_vendor", "device.manufacturer")?;
            }

            if event.has_value("json.decorations.hardware_model") {
                event.rename("json.decorations.hardware_model", "device.model.name")?;
            }

            if event.has_value("json.decorations.hardware_version") {
                event.rename(
                    "json.decorations.hardware_version",
                    "kolide.osquery_result.hardware_version",
                )?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if let Some(v) = event
                    .get("json.decorations.hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { !event.has_value("kolide.osquery_result.hardware_uuid") };
            if _cond {
                if let Some(v) = event
                    .get("json.decorations.uuid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.osquery_result.hardware_uuid", v)?;
                }
            }

            let _cond = { !event.has_value("kolide.osquery_result.device_serial") };
            if _cond {
                if let Some(v) = event
                    .get("json.decorations.hardware_serial")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.osquery_result.device_serial", v)?;
                }
            }

            let _cond = {
                event.has_value("host.ip") && event.get("host.ip").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                if event.has_value("host.ip") {
                    if let Some(ip_str) = event.get_string("host.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("host.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("host.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("host.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("host.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("host.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("host.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("host.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("host.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("host.ip") && event.get("host.ip").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("host.ip.0")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.snapshot") {
                event.rename("json.snapshot", "kolide.osquery_result.snapshot")?;
            }

            if event.has_value("json.diffResults.added") {
                event.rename("json.diffResults.added", "kolide.osquery_result.added")?;
            }

            if event.has_value("json.diffResults.removed") {
                event.rename("json.diffResults.removed", "kolide.osquery_result.removed")?;
            }

            // Begin nested pipeline: "categorize"
            let _cond = { event.get_str("event.action") == Some("snapshot") };
            if _cond {
                event.set("event.kind", json!("state"))?;
            }
            let _cond = { event.get_str("event.action") == Some("snapshot") };
            if _cond {
                event.append("event.category", json!("host"))?;
            }
            let _cond = { event.get_str("event.action") == Some("snapshot") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }
            let _cond = { event.get_str("event.action") == Some("differential") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }
            let _cond = { event.get_str("event.action") == Some("differential") };
            if _cond {
                event.append("event.category", json!("host"))?;
            }
            let _cond = { event.get_str("event.action") == Some("differential") };
            if _cond {
                event.append("event.type", json!("change"))?;
            }
            let _cond = {
                event.get_str("event.action") != Some("snapshot")
                    && event.get_str("event.action") != Some("differential")
            };
            if _cond {
                event.set("event.kind", json!("state"))?;
            }
            let _cond = {
                event.get_str("event.action") != Some("snapshot")
                    && event.get_str("event.action") != Some("differential")
            };
            if _cond {
                event.append("event.category", json!("host"))?;
            }
            let _cond = {
                event.get_str("event.action") != Some("snapshot")
                    && event.get_str("event.action") != Some("differential")
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }
            // End nested pipeline: "categorize"

            if let Some(v) = event
                .get("kolide.osquery_result.request_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = { event.has_value("event.id") && event.get_str("event.id") != Some("") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.id") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "event.id".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set(
                            "_id",
                            json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?),
                        )?;
                    }
                }
            }

            let _cond = { !event.has_value("event.id") && event.has_value("event.original") };
            if _cond {
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
                        event.set(
                            "_id",
                            json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?),
                        )?;
                    }
                }
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

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
