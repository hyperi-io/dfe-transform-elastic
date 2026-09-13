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
            event.set("ecs.version", json!("9.3.0"))?;

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

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.get_bool("_conf.enable_deduplication") == Some(false)
                    && event.get_str("input.type") == Some("aws-s3")
            };
            if _cond {
                event.remove("_id");
            }

            let _cond = {
                event.has_value("json.Timestamp") && event.get_str("json.Timestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.Timestamp") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Timestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.Timestamp", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.Timestamp")
                    && event.get("json.Timestamp").is_some_and(|v| v.is_number())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: long t = (long)(ctx.json.Timestamp);\nif (t > (long)(1e18)) {\n  ctx.json.Timestamp = t/(long)(1e6)\n} else if (t < (long)(1e10))  {\n  ctx.json.Timestamp = t*(long)(1e3)\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"long t = (long)(ctx.json.Timestamp);\nif (t > (long)(1e18)) {\n  ctx.json.Timestamp = t/(long)(1e6)\n} else if (t < (long)(1e10))  {\n  ctx.json.Timestamp = t*(long)(1e3)\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "painless_timestamp_to_milli",
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

            let _cond = {
                event.has_value("json.Timestamp") && event.get_str("json.Timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601", "yyyy-MM-dd'T'HH:mm:ssZ"],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.Timestamp".into(),
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
                        "date_json_Timestamp_70be028a",
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
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloudflare_logpush.device_posture.timestamp", v)?;
            }

            if event.has_value("json.ClientVersion") {
                event.rename(
                    "json.ClientVersion",
                    "cloudflare_logpush.device_posture.version",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.version", v)?;
            }

            if event.has_value("json.DeviceID") {
                event.rename("json.DeviceID", "cloudflare_logpush.device_posture.host.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.host.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.host.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if event.has_value("json.DeviceManufacturer") {
                event.rename(
                    "json.DeviceManufacturer",
                    "cloudflare_logpush.device_posture.host.manufacturer",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.host.manufacturer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.manufacturer", v)?;
            }

            if event.has_value("json.DeviceModel") {
                event.rename(
                    "json.DeviceModel",
                    "cloudflare_logpush.device_posture.host.model",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.host.model")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.name", v)?;
            }

            if event.has_value("json.DeviceName") {
                event.rename(
                    "json.DeviceName",
                    "cloudflare_logpush.device_posture.host.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.host.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if event.has_value("json.DeviceSerialNumber") {
                event.rename(
                    "json.DeviceSerialNumber",
                    "cloudflare_logpush.device_posture.host.serial",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.host.serial")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.serial_number", v)?;
            }

            if event.has_value("json.DeviceType") {
                event.rename(
                    "json.DeviceType",
                    "cloudflare_logpush.device_posture.host.os.family",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.host.os.family")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.family", v)?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.host.os.family")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.type", v)?;
            }

            if event.has_value("json.OSVersion") {
                event.rename(
                    "json.OSVersion",
                    "cloudflare_logpush.device_posture.host.os.version",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.host.os.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has_value("json.PolicyID") {
                event.rename("json.PolicyID", "cloudflare_logpush.device_posture.rule.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.rule.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has_value("json.PostureCheckName") {
                event.rename(
                    "json.PostureCheckName",
                    "cloudflare_logpush.device_posture.rule.name",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.rule.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.PostureCheckType") {
                event.rename(
                    "json.PostureCheckType",
                    "cloudflare_logpush.device_posture.rule.category",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.rule.category")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.category", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.PostureEvaluatedResult") {
                    if let Some(val) = event.get("json.PostureEvaluatedResult") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.PostureEvaluatedResult".into(),
                                message,
                            }
                        })?;
                        event.set("cloudflare_logpush.device_posture.eval.result", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_postureevaluatedresult_to_boolean",
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

            let _cond = {
                event.has_value("cloudflare_logpush.device_posture.eval.result")
                    && event.get_bool("cloudflare_logpush.device_posture.eval.result") == Some(true)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.device_posture.eval.result")
                    && event.get_bool("cloudflare_logpush.device_posture.eval.result")
                        == Some(false)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has_value("json.PostureExpectedJSON") {
                event.rename(
                    "json.PostureExpectedJSON",
                    "cloudflare_logpush.device_posture.eval.expected",
                )?;
            }

            if event.has_value("json.PostureReceivedJSON") {
                event.rename(
                    "json.PostureReceivedJSON",
                    "cloudflare_logpush.device_posture.eval.received",
                )?;
            }

            if event.has_value("json.RegistrationID") {
                event.rename(
                    "json.RegistrationID",
                    "cloudflare_logpush.device_posture.registration_id",
                )?;
            }

            if event.has_value("json.UserUID") {
                event.rename("json.UserUID", "cloudflare_logpush.device_posture.user.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("json.Email") {
                event.rename("json.Email", "cloudflare_logpush.device_posture.user.email")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.device_posture.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("cloudflare_logpush.device_posture.host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.device_posture.host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.device_posture.host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.device_posture.host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.device_posture.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.device_posture.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.device_posture.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.device_posture.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");
            event.remove("_conf");

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
                event.remove("cloudflare_logpush.device_posture.timestamp");
                event.remove("cloudflare_logpush.device_posture.version");
                event.remove("cloudflare_logpush.device_posture.host.id");
                event.remove("cloudflare_logpush.device_posture.host.manufacturer");
                event.remove("cloudflare_logpush.device_posture.host.model");
                event.remove("cloudflare_logpush.device_posture.host.name");
                event.remove("cloudflare_logpush.device_posture.host.serial");
                event.remove("cloudflare_logpush.device_posture.host.os.family");
                event.remove("cloudflare_logpush.device_posture.host.os.version");
                event.remove("cloudflare_logpush.device_posture.rule.id");
                event.remove("cloudflare_logpush.device_posture.rule.name");
                event.remove("cloudflare_logpush.device_posture.rule.category");
                event.remove("cloudflare_logpush.device_posture.user.id");
                event.remove("cloudflare_logpush.device_posture.user.email");
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["N/A".into()],
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
