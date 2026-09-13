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

            let _cond = { event.has_value("message") && event.get_str("message") == Some("retry") };
            if _cond {
                return Ok(TransformResult::Drop);
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.Administrator") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.Description") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.EventTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.InternalSessionId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.LoggedAt") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.LoggedFrom") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.Feature") {
                event.rename("json.Feature", "cyberark_epm.admin_audit.feature")?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("cyberark_epm.admin_audit.feature")
                    && (event
                        .get_str("cyberark_epm.admin_audit.feature")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Agent Config"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Policies"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("My Computers")))
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.admin_audit.feature")
                    && (event
                        .get_str("cyberark_epm.admin_audit.feature")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Public API")))
            };
            if _cond {
                event.append("event.category", json!("api"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.admin_audit.feature")
                    && (event
                        .get_str("cyberark_epm.admin_audit.feature")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Sets")))
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.admin_audit.feature")
                    && (event
                        .get_str("cyberark_epm.admin_audit.feature")
                        .is_some_and(|s| s.eq_ignore_ascii_case("WebPage"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("End-User UI"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Application Groups"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Reports")))
            };
            if _cond {
                event.append("event.category", json!("web"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.admin_audit.feature")
                    && (event
                        .get_str("cyberark_epm.admin_audit.feature")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Agent Config"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Public API"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("WebPage"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("End-User UI"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Application Groups"))
                        || event
                            .get_str("cyberark_epm.admin_audit.feature")
                            .is_some_and(|s| s.eq_ignore_ascii_case("Policies")))
            };
            if _cond {
                event.append("event.type", json!("access"))?;
            }

            let _cond = {
                event.has_value("cyberark_epm.admin_audit.feature")
                    && (event
                        .get_str("cyberark_epm.admin_audit.feature")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Sets")))
            };
            if _cond {
                event.append("event.type", json!("admin"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            event.set("observer.vendor", json!("CyberArk"))?;

            event.set("observer.product", json!("Endpoint Privilege Manager"))?;

            let _cond = {
                event.has_value("json.EventTime") && event.get_str("json.EventTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.EventTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("cyberark_epm.admin_audit.event_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.EventTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_EventTime")?;
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
                .get("cyberark_epm.admin_audit.event_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.Description") {
                event.rename("json.Description", "cyberark_epm.admin_audit.description")?;
            }

            if let Some(v) = event
                .get("cyberark_epm.admin_audit.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { event.get_str("json.LoggedFrom") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.LoggedFrom") {
                        if let Some(val) = event.get("json.LoggedFrom") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.LoggedFrom".into(),
                                    message,
                                }
                            })?;
                            event.set("cyberark_epm.admin_audit.logged_from", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_LoggedFrom_to_ip",
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

            if let Some(v) = event
                .get("cyberark_epm.admin_audit.logged_from")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("cyberark_epm.admin_audit.logged_from")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.Administrator") {
                event.rename(
                    "json.Administrator",
                    "cyberark_epm.admin_audit.administrator",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cyberark_epm.admin_audit.administrator") {
                    if let Some(input) = event.get_string("cyberark_epm.admin_audit.administrator")
                    {
                        // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        if !extract_first_match(
                            &[
                                cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
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

            let _cond = {
                event.has_value("cyberark_epm.admin_audit.administrator")
                    && event
                        .get_str("cyberark_epm.admin_audit.administrator")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event
                    .get("cyberark_epm.admin_audit.administrator")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
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

            if event.has_value("json.Role") {
                event.rename("json.Role", "cyberark_epm.admin_audit.role")?;
            }

            event.append_unique(
                "user.roles",
                json!(
                    event
                        .get("cyberark_epm.admin_audit.role")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.InternalSessionId") {
                if let Some(val) = event.get("json.InternalSessionId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.InternalSessionId".into(),
                            message,
                        }
                    })?;
                    event.set("cyberark_epm.admin_audit.internal_session_id", converted)?;
                }
            }

            let _cond =
                { event.has_value("json.LoggedAt") && event.get_str("json.LoggedAt") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.LoggedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("cyberark_epm.admin_audit.logged_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LoggedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_LoggedAt")?;
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

            if event.has_value("json.PermissionDescription") {
                event.rename(
                    "json.PermissionDescription",
                    "cyberark_epm.admin_audit.permission_description",
                )?;
            }

            if event.has_value("json.SetName") {
                event.rename("json.SetName", "cyberark_epm.admin_audit.set_name")?;
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
                event.remove("cyberark_epm.admin_audit.event_time");
                event.remove("cyberark_epm.admin_audit.logged_from");
                event.remove("cyberark_epm.admin_audit.administrator");
                event.remove("cyberark_epm.admin_audit.role");
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
