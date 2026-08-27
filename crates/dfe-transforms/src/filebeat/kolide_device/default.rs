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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { event.get("event").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("event") {
                    event.rename("event", "json.event")?;
                }
            }

            let _cond = { event.has_value("json.event") && event.has_value("data") };
            if _cond {
                // Painless script
                // Source: if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }"#
                    ),
                )?;
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

            let _cond = { !event.has_value("json") && event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("json.timestamp") };
            if _cond {
                if let Some(v) = event.get("_ingest.timestamp").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { event.has_value("json.registered_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.registered_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.registered_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
                // Begin nested pipeline: "webhook"
                if event.has_value("json.id") {
                    if let Some(val) = event.get("json.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.id".into(),
                                message,
                            }
                        })?;
                        event.set("event.id", converted)?;
                    }
                }
                let _cond = { event.has_value("json.event") };
                if _cond {
                    if let Some(v) = event.get("json.event").cloned() {
                        event.set("event.action", v)?;
                    }
                }
                if event.has_value("json.data.device_name") {
                    event.rename("json.data.device_name", "host.name")?;
                }
                if event.has_value("json.data.device_id") {
                    if let Some(val) = event.get("json.data.device_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.device_id".into(),
                                message,
                            }
                        })?;
                        event.set("host.id", converted)?;
                    }
                }
                if event.has_value("json.data.registered_owner.email") {
                    event.rename("json.data.registered_owner.email", "user.email")?;
                }
                if event.has_value("json.data.registered_owner.name") {
                    event.rename("json.data.registered_owner.name", "user.name")?;
                }
                if event.has_value("json.data.registered_owner.id") {
                    if let Some(val) = event.get("json.data.registered_owner.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.registered_owner.id".into(),
                                message,
                            }
                        })?;
                        event.set("user.id", converted)?;
                    }
                }
                if event.has_value("json.data.device_status") {
                    event.rename("json.data.device_status", "kolide.device.device_status")?;
                }
                if event.has_value("json.data.device_url") {
                    event.rename("json.data.device_url", "kolide.device.device_url")?;
                }
                let _cond = {
                    !event.has_value("event.original")
                        && event.has_value("event.id")
                        && event.get_str("event.id") != Some("")
                };
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
                                json!(fingerprint_with(&values, "SHA-256", "").map_err(
                                    |message| TransformError::ParseError {
                                        path: "_id".into(),
                                        message
                                    }
                                )?),
                            )?;
                        }
                    }
                }
                // End nested pipeline: "webhook"
            }

            let _cond = { !event.has_value("json.event") };
            if _cond {
                event.set("event.action", json!("device"))?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if event.has_value("json.name") {
                    event.rename("json.name", "host.name")?;
                }
            }

            let _cond = { !event.has_value("host.id") };
            if _cond {
                if event.has_value("json.id") {
                    if let Some(val) = event.get("json.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.id".into(),
                                message,
                            }
                        })?;
                        event.set("host.id", converted)?;
                    }
                }
            }

            if event.has_value("json.operating_system") {
                event.rename("json.operating_system", "host.os.full")?;
            }

            let _cond = { event.has_value("host.os.full") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("host.os.full") {
                        if let Some(input) = event.get_string("host.os.full") {
                            // Grok pattern: ^%{DATA:host.os.name} %{NOTSPACE:host.os.version}$
                            let _ =
                                cached_grok!("^%{DATA:host.os.name} %{NOTSPACE:host.os.version}$")
                                    .extract_into(&input, event)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.device_type") };
            if _cond {
                if let Some(v) = event.get("json.device_type").cloned() {
                    event.set("host.os.platform", v)?;
                }
            }

            let _cond = { event.get_str("json.form_factor") == Some("phone") };
            if _cond {
                event.set("host.type", json!("mobile"))?;
            }

            let _cond = { event.get_str("json.form_factor") == Some("tablet") };
            if _cond {
                event.set("host.type", json!("tablet"))?;
            }

            let _cond = {
                event.has_value("json.form_factor")
                    && event
                        .get_str("json.form_factor")
                        .is_some_and(|s| s.eq_ignore_ascii_case("Computer"))
            };
            if _cond {
                event.set("host.type", json!("desktop"))?;
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
                if event.has_value("json.registered_owner_info.identifier") {
                    if let Some(val) = event.get("json.registered_owner_info.identifier") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.registered_owner_info.identifier".into(),
                                message,
                            }
                        })?;
                        event.set("user.id", converted)?;
                    }
                }
            }

            let _cond = { event.has_value("json.device_type") };
            if _cond {
                if let Some(v) = event.get("json.device_type").cloned() {
                    event.set("kolide.device.type", v)?;
                }
            }

            event.remove("json.device_type");

            let _cond = { event.has_value("json.serial") };
            if _cond {
                if event.has_value("json.serial") {
                    event.rename("json.serial", "kolide.device.serial")?;
                }
            }

            let _cond = { event.has_value("json.hardware_uuid") };
            if _cond {
                if event.has_value("json.hardware_uuid") {
                    event.rename("json.hardware_uuid", "kolide.device.hardware_uuid")?;
                }
            }

            if event.has_value("json.hardware_model") {
                event.rename("json.hardware_model", "kolide.device.hardware_model")?;
            }

            let _cond = { event.has_value("json.note") };
            if _cond {
                if event.has_value("json.note") {
                    event.rename("json.note", "kolide.device.note")?;
                }
            }

            if event.has_value("json.form_factor") {
                event.rename("json.form_factor", "kolide.device.form_factor")?;
            }

            if event.has_value("json.product_image_url") {
                event.rename("json.product_image_url", "kolide.device.product_image_url")?;
            }

            if event.has_value("json.auth_state") {
                event.rename("json.auth_state", "kolide.device.auth_state")?;
            }

            let _cond = { event.has_value("json.registered_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.registered_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.device.registered_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.registered_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.last_authenticated_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.last_authenticated_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.device.last_authenticated_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_authenticated_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.last_seen_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.last_seen_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.device.last_seen_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_seen_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.will_block_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.will_block_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.device.will_block_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.will_block_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.registered_owner_info.identifier") {
                event.rename(
                    "json.registered_owner_info.identifier",
                    "kolide.device.registered_owner_info.identifier",
                )?;
            }

            let _cond = { event.has_value("json.registered_owner_info.link") };
            if _cond {
                if event.has_value("json.registered_owner_info.link") {
                    event.rename(
                        "json.registered_owner_info.link",
                        "kolide.device.registered_owner_info.location",
                    )?;
                }
            }

            if event.has_value("json.auth_configuration.device_id") {
                event.rename(
                    "json.auth_configuration.device_id",
                    "kolide.device.auth_configuration.device_id",
                )?;
            }

            if event.has_value("json.auth_configuration.authentication_mode") {
                event.rename(
                    "json.auth_configuration.authentication_mode",
                    "kolide.device.auth_configuration.authentication_mode",
                )?;
            }

            if event.has_value("json.auth_configuration.person_groups") {
                event.rename(
                    "json.auth_configuration.person_groups",
                    "kolide.device.auth_configuration.person_groups",
                )?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                if let Some(v) = event.get("host.name").cloned() {
                    event.set("host.hostname", v)?;
                }
            }

            let _cond = { event.has_value("kolide.device.device_status") };
            if _cond {
                if let Some(v) = event.get("kolide.device.device_status").cloned() {
                    event.set("kolide.device.status", v)?;
                }
            }

            let _cond = {
                !event.has_value("kolide.device.status")
                    && event.has_value("kolide.device.auth_state")
            };
            if _cond {
                if let Some(v) = event.get("kolide.device.auth_state").cloned() {
                    event.set("kolide.device.status", v)?;
                }
            }

            if event.has_value("kolide.device.status") {
                map_strings(
                    event,
                    "kolide.device.status",
                    "kolide.device.status",
                    str::to_lowercase,
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

            // Begin nested pipeline: "extended-mappings"
            if event.has_value("host.os.platform") {
                map_strings(
                    event,
                    "host.os.platform",
                    "host.os.platform",
                    str::to_lowercase,
                )?;
            }
            let _cond = { event.get_str("host.os.platform") == Some("android") };
            if _cond {
                event.set("host.os.type", json!("android"))?;
            }
            let _cond = { event.get_str("host.os.platform") == Some("mac") };
            if _cond {
                event.set("host.os.type", json!("macos"))?;
            }
            let _cond = { event.get_str("host.os.platform") == Some("windows") };
            if _cond {
                event.set("host.os.type", json!("windows"))?;
            }
            let _cond = { event.get_str("host.os.platform") == Some("linux") };
            if _cond {
                event.set("host.os.type", json!("linux"))?;
            }
            let _cond = { event.get_str("host.os.platform") == Some("ios") };
            if _cond {
                event.set("host.os.type", json!("ios"))?;
            }
            // End nested pipeline: "extended-mappings"

            // Begin nested pipeline: "categorize"
            let _cond = { event.has_value("event.action") };
            if _cond {
                // Painless script
                // Source: def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome') && ctx.event.outcome == null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.kind = 'event';\n  ctx.event.category = ['host'];\n  ctx.event.type = ['change'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome') && ctx.event.outcome == null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.kind = 'event';\n  ctx.event.category = ['host'];\n  ctx.event.type = ['change'];\n}"#
                    ),
                    cached_params!(
                        "{\"exact\":{\"device\":{\"kind\":\"state\",\"category\":[\"host\"],\"type\":[\"info\"]},\"devices.created\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"change\"]},\"devices.registered\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"change\"],\"outcome\":\"success\"},\"devices.destroyed\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"change\"]},\"device_trust.status_changed\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"change\"]}}}"
                    ),
                )?;
            }
            // End nested pipeline: "categorize"

            let _cond = {
                event.has_value("event.original")
                    && event.get_str("event.original") != Some("")
                    && !event.has_value("json.event")
            };
            if _cond {
                parse_json_field(event, "event.original", "_tmp.fingerprint_source")?;
            }

            let _cond = { event.has_value("_tmp.fingerprint_source") };
            if _cond {
                event.remove("_tmp.fingerprint_source.last_seen_at");
            }

            let _cond = { event.has_value("_tmp.fingerprint_source") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("_tmp.fingerprint_source") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "_tmp.fingerprint_source".into(),
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
            event.remove("data");
            event.remove("_tmp");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

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
