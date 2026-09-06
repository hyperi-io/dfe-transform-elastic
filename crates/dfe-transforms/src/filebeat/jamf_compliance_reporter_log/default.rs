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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = { event.get_str("json.header.event_name") == Some("APP_METRICS") };
            if _cond {
                // Begin nested pipeline: "pipeline_app_metrics"
                event.append("event.type", json!("info"))?;
                event.set("event.kind", json!("event"))?;
                event.set("jamf_compliance_reporter.log.dataset", json!("app_metrics"))?;
                event.set("host.os.type", json!("macos"))?;
                event.append("event.category", json!("process"))?;
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json._event_score") {
                        if let Some(val) = event.get("json._event_score") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json._event_score".into(),
                                    message,
                                }
                            })?;
                            event.set("jamf_compliance_reporter.log.event_score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                    if event.has_value("json.app_metric_info.cpu_percentage") {
                        if let Some(val) = event.get("json.app_metric_info.cpu_percentage") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.app_metric_info.cpu_percentage".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.app_metric_info.cpu_percentage",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                    if event.has_value("json.app_metric_info.cpu_time_seconds") {
                        if let Some(val) = event.get("json.app_metric_info.cpu_time_seconds") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.app_metric_info.cpu_time_seconds".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.app_metric_info.cpu_time_seconds",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                    if event.has_value("json.app_metric_info.interrupt_wakeups") {
                        if let Some(val) = event.get("json.app_metric_info.interrupt_wakeups") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.app_metric_info.interrupt_wakeups".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.app_metric_info.interrupt_wakeups",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                    if event.has_value("json.app_metric_info.platform_idle_wakeups") {
                        if let Some(val) = event.get("json.app_metric_info.platform_idle_wakeups") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.app_metric_info.platform_idle_wakeups".into(),
                                    message,
                                }
                            })?;
                            event.set("jamf_compliance_reporter.log.app_metric_info.platform_idle_wakeups", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                    if event.has_value("json.app_metric_info.resident_memory_size_mb") {
                        if let Some(val) = event.get("json.app_metric_info.resident_memory_size_mb")
                        {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.app_metric_info.resident_memory_size_mb".into(),
                                    message,
                                }
                            })?;
                            event.set("jamf_compliance_reporter.log.app_metric_info.resident_memory_size.mb", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                    if event.has_value("json.app_metric_info.virtual_memory_size_mb") {
                        if let Some(val) = event.get("json.app_metric_info.virtual_memory_size_mb")
                        {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.app_metric_info.virtual_memory_size_mb".into(),
                                    message,
                                }
                            })?;
                            event.set("jamf_compliance_reporter.log.app_metric_info.virtual_memory_size.mb", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.header.event_name") {
                    event.rename("json.header.event_name", "event.action")?;
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = { event.get_i64("json.header.time_seconds_epoch") != Some(0) };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.header.time_seconds_epoch")
                        {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.header.time_seconds_epoch".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.host_info.host_name") {
                    event.rename("json.host_info.host_name", "host.hostname")?;
                }
                let _cond = { event.has_value("host.hostname") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("host.hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("json.host_info.host_uuid") {
                    event.rename(
                        "json.host_info.host_uuid",
                        "jamf_compliance_reporter.log.host_info.host.uuid",
                    )?;
                }
                if event.has_value("json.host_info.osversion") {
                    event.rename("json.host_info.osversion", "host.os.version")?;
                }
                let _cond = { event.has_value("json.host_info.primary_mac_address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "host.mac",
                            json!(
                                event
                                    .get("json.host_info.primary_mac_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("host.mac") {
                    gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
                }
                if event.has_value("host.mac") {
                    map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
                }
                if event.has_value("json.host_info.serial_number") {
                    event.rename("json.host_info.serial_number", "host.id")?;
                }
                let _cond = { event.has_value("json.app_metric_info.cpu_percentage") };
                if _cond {
                    // Painless script
                    // Source: ctx.host.cpu = new HashMap();\nctx.host.cpu.usage = Math.round(ctx.json.app_metric_info.cpu_percentage * 10) / 1000.0;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.host.cpu = new HashMap();\nctx.host.cpu.usage = Math.round(ctx.json.app_metric_info.cpu_percentage * 10) / 1000.0;\n"#
                        ),
                    )?;
                }
                // End nested pipeline: "pipeline_app_metrics"
            }

            let _cond = {
                event
                    .get_str("json.header.event_name")
                    .is_some_and(|s| s.starts_with("AUE_"))
            };
            if _cond {
                // Begin nested pipeline: "pipeline_audit"
                event.set("jamf_compliance_reporter.log.dataset", json!("audit"))?;
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json._event_score") {
                        if let Some(val) = event.get("json._event_score") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json._event_score".into(),
                                    message,
                                }
                            })?;
                            event.set("jamf_compliance_reporter.log.event_score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.header.event_id") {
                        if let Some(val) = event.get("json.header.event_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.header.event_id".into(),
                                    message,
                                }
                            })?;
                            event.set("event.code", converted)?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.header.event_modifier") {
                        if let Some(val) = event.get("json.header.event_modifier") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.header.event_modifier".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.header.event_modifier",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("json.header.event_name") {
                    event.rename("json.header.event_name", "event.action")?;
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = {
                    event.has_value("json.header.time_seconds_epoch")
                        && event.get_i64("json.header.time_seconds_epoch") != Some(0)
                };
                if _cond {
                    // Painless script
                    // Source: ctx.json.time_milliseconds = (long)ctx.json.header.time_seconds_epoch * 1000;\nif (ctx.json?.header?.time_milliseconds_offset != null && ctx.json.header.time_milliseconds_offset != 0) {\n  ctx.json.time_milliseconds = ctx.json.time_milliseconds + (long)ctx.json.header.time_milliseconds_offset;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.json.time_milliseconds = (long)ctx.json.header.time_seconds_epoch * 1000;\nif (ctx.json?.header?.time_milliseconds_offset != null && ctx.json.header.time_milliseconds_offset != 0) {\n  ctx.json.time_milliseconds = ctx.json.time_milliseconds + (long)ctx.json.header.time_milliseconds_offset;\n}\n"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("json.time_milliseconds")
                        && event.get_i64("json.time_milliseconds") != Some(0)
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.time_milliseconds") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.time_milliseconds".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.header.version") {
                        if let Some(val) = event.get("json.header.version") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.header.version".into(),
                                    message,
                                }
                            })?;
                            event.set("jamf_compliance_reporter.log.header.version", converted)?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("json.host_info.host_name") {
                    event.rename("json.host_info.host_name", "host.hostname")?;
                }
                let _cond = { event.has_value("host.hostname") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("host.hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("json.host_info.host_uuid") {
                    event.rename(
                        "json.host_info.host_uuid",
                        "jamf_compliance_reporter.log.host_info.host.uuid",
                    )?;
                }
                if event.has_value("json.host_info.osversion") {
                    event.rename("json.host_info.osversion", "host.os.version")?;
                }
                let _cond = { event.has_value("json.host_info.primary_mac_address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "host.mac",
                            json!(
                                event
                                    .get("json.host_info.primary_mac_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("host.mac") {
                    gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
                }
                if event.has_value("host.mac") {
                    map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
                }
                if event.has_value("json.host_info.serial_number") {
                    event.rename("json.host_info.serial_number", "host.id")?;
                }
                if event.has_value("json.return.description") {
                    event.rename(
                        "json.return.description",
                        "jamf_compliance_reporter.log.return.description",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.return.error") {
                        if let Some(val) = event.get("json.return.error") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.return.error".into(),
                                    message,
                                }
                            })?;
                            event.set("error.code", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.get_str("error.code") == Some("0") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("error.code") != Some("0") };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.return.return_value") {
                        if let Some(val) = event.get("json.return.return_value") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.return.return_value".into(),
                                    message,
                                }
                            })?;
                            event.set("process.exit_code", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.subject.audit_id") {
                        if let Some(val) = event.get("json.subject.audit_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.audit_id".into(),
                                    message,
                                }
                            })?;
                            event.set("process.real_user.id", converted)?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("json.subject.audit_user_name") {
                    event.rename("json.subject.audit_user_name", "process.real_user.name")?;
                }
                let _cond = { event.has_value("process.real_user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("process.real_user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("json.subject.audit_user_name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("json.subject.audit_user_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.subject.effective_group_id") {
                        if let Some(val) = event.get("json.subject.effective_group_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.effective_group_id".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.subject.effective.group.id",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("json.subject.effective_group_name") {
                    event.rename(
                        "json.subject.effective_group_name",
                        "jamf_compliance_reporter.log.subject.effective.group.name",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.subject.effective_user_id") {
                        if let Some(val) = event.get("json.subject.effective_user_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.effective_user_id".into(),
                                    message,
                                }
                            })?;
                            event.set("process.user.id", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("json.subject.effective_user_name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("json.subject.effective_user_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("json.subject.effective_user_name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("json.subject.effective_user_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("json.subject.effective_user_name") {
                    event.rename("json.subject.effective_user_name", "process.user.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.subject.group_id") {
                        if let Some(val) = event.get("json.subject.group_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.group_id".into(),
                                    message,
                                }
                            })?;
                            event.set("process.real_group.id", converted)?;
                        }
                    }
                    Ok(())
                })();
                if event.has_value("json.subject.group_name") {
                    event.rename("json.subject.group_name", "process.real_group.name")?;
                }
                if event.has_value("json.subject.process_hash") {
                    event.rename("json.subject.process_hash", "process.hash.sha1")?;
                }
                let _cond = { event.has_value("process.hash.sha1") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("process.hash.sha1")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.subject.process_id") {
                        if let Some(val) = event.get("json.subject.process_id") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.process_id".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.subject.process.pid",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.subject.process_name") {
                    event.rename(
                        "json.subject.process_name",
                        "jamf_compliance_reporter.log.subject.process.name",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.subject.session_id") {
                        if let Some(val) = event.get("json.subject.session_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.session_id".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.subject.session.id",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.subject.terminal_id.addr") {
                        if let Some(val) = event.get("json.subject.terminal_id.addr") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.terminal_id.addr".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.subject.terminal_id.addr",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })();
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.subject.terminal_id.ip_address") {
                        if let Some(val) = event.get("json.subject.terminal_id.ip_address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.terminal_id.ip_address".into(),
                                    message,
                                }
                            })?;
                            event.set("json.subject.terminal_id.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.remove("json.subject.terminal_id.ip_address");
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "host.ip",
                            json!(
                                event
                                    .get("json.subject.terminal_id.ip_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("json.subject.terminal_id.ip_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.subject.terminal_id.port") {
                        if let Some(val) = event.get("json.subject.terminal_id.port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.terminal_id.port".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.subject.terminal_id.port",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.subject.terminal_id.type") {
                        if let Some(val) = event.get("json.subject.terminal_id.type") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.terminal_id.type".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "jamf_compliance_reporter.log.subject.terminal_id.type",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.subject.user_id") {
                        if let Some(val) = event.get("json.subject.user_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.subject.user_id".into(),
                                    message,
                                }
                            })?;
                            event.set("user.id", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("json.subject.user_name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("json.subject.user_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("json.subject.user_name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("json.subject.user_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.append("event.type", json!("info"))?;
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("authentication"))?;
                let _cond = { event.get_str("event.action") == Some("aue_accept") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_accept"
                    if event.has_value("json.path") {
                        event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.socket_unix.family") {
                            if let Some(val) = event.get("json.socket_unix.family") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.socket_unix.family".into(),
                                            message,
                                        }
                                    })?;
                                event.set("json.inet_family", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.socket_unix.path") {
                        event.rename(
                            "json.socket_unix.path",
                            "jamf_compliance_reporter.log.socket.unix.path",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.fd") {
                            if let Some(val) = event.get("json.arguments.fd") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.fd".into(),
                                            message,
                                        }
                                    })?;
                                event
                                    .set("jamf_compliance_reporter.log.arguments.fd", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.inet_family") };
                    if _cond {
                        // Painless script
                        // Source: Map map = new HashMap();\nmap.put('0', 'AF_UNSPEC');\nmap.put('1', 'AF_LOCAL');\nmap.put('AF_LOCAL', 'AF_UNIX');\nmap.put('2', 'AF_INET');\nmap.put('3', 'AF_ImapPLINK');\nmap.put('4', 'AF_PUP');\nmap.put('5', 'AF_CHAOS');\nmap.put('6', 'AF_NS');\nmap.put('7', 'AF_ISO');\nmap.put('AF_ISO', 'AF_OSI');\nmap.put('8', 'AF_ECmapA');\nmap.put('9', 'AF_DATAKIT');\nmap.put('10', 'AF_CCITT');\nmap.put('11', 'AF_SNA');\nmap.put('12', 'AF_DECnet');\nmap.put('13', 'AF_DLI');\nmap.put('14', 'AF_LAT');\nmap.put('15', 'AF_HYLINK');\nmap.put('16', 'AF_APPLETALK');\nmap.put('17', 'AF_ROUTE');\nmap.put('18', 'AF_LINK');\nmap.put('19', 'pseudo_AF_XTP');\nmap.put('20', 'AF_COIP');\nmap.put('21', 'AF_CNT');\nmap.put('22', 'pseudo_AF_RTIP');\nmap.put('23', 'AF_IPX');\nmap.put('24', 'AF_SIP');\nmap.put('25', 'pseudo_AF_PIP');\nctx.jamf_compliance_reporter.log.socket.unix.family = map.get(ctx.json.inet_family);\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"Map map = new HashMap();\nmap.put('0', 'AF_UNSPEC');\nmap.put('1', 'AF_LOCAL');\nmap.put('AF_LOCAL', 'AF_UNIX');\nmap.put('2', 'AF_INET');\nmap.put('3', 'AF_ImapPLINK');\nmap.put('4', 'AF_PUP');\nmap.put('5', 'AF_CHAOS');\nmap.put('6', 'AF_NS');\nmap.put('7', 'AF_ISO');\nmap.put('AF_ISO', 'AF_OSI');\nmap.put('8', 'AF_ECmapA');\nmap.put('9', 'AF_DATAKIT');\nmap.put('10', 'AF_CCITT');\nmap.put('11', 'AF_SNA');\nmap.put('12', 'AF_DECnet');\nmap.put('13', 'AF_DLI');\nmap.put('14', 'AF_LAT');\nmap.put('15', 'AF_HYLINK');\nmap.put('16', 'AF_APPLETALK');\nmap.put('17', 'AF_ROUTE');\nmap.put('18', 'AF_LINK');\nmap.put('19', 'pseudo_AF_XTP');\nmap.put('20', 'AF_COIP');\nmap.put('21', 'AF_CNT');\nmap.put('22', 'pseudo_AF_RTIP');\nmap.put('23', 'AF_IPX');\nmap.put('24', 'AF_SIP');\nmap.put('25', 'pseudo_AF_PIP');\nctx.jamf_compliance_reporter.log.socket.unix.family = map.get(ctx.json.inet_family);\n"#
                            ),
                        )?;
                    }
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_accept"
                }
                let _cond = {
                    ["aue_auth_user", "aue_ssauthorize", "aue_ssauthmech"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_auth"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.texts") {
                        event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                    }
                    // End nested pipeline: "pipeline_aue_auth"
                }
                let _cond = {
                    ["aue_bind", "aue_connect"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_bind_and_aue_connect"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.socket_inet.addr") {
                            if let Some(val) = event.get("json.socket_inet.addr") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.socket_inet.addr".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.socket.inet.addr",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.fd") {
                            if let Some(val) = event.get("json.arguments.fd") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.fd".into(),
                                            message,
                                        }
                                    })?;
                                event
                                    .set("jamf_compliance_reporter.log.arguments.fd", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.socket_inet.family") {
                            if let Some(val) = event.get("json.socket_inet.family") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.socket_inet.family".into(),
                                            message,
                                        }
                                    })?;
                                event.set("json.inet_family", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.socket_inet.id") {
                            if let Some(val) = event.get("json.socket_inet.id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.socket_inet.id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.socket.inet.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.socket_inet.ip_address") {
                            if let Some(val) = event.get("json.socket_inet.ip_address") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.socket_inet.ip_address".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("json.socket_inet.ip_address");
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    let _cond = { event.has_value("server.ip") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("server.ip")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.socket_inet.port") {
                            if let Some(val) = event.get("json.socket_inet.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.socket_inet.port".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    let _cond = { event.has_value("json.inet_family") };
                    if _cond {
                        // Painless script
                        // Source: Map map = new HashMap();\nmap.put('0', 'AF_UNSPEC');\nmap.put('1', 'AF_LOCAL');\nmap.put('AF_LOCAL', 'AF_UNIX');\nmap.put('2', 'AF_INET');\nmap.put('3', 'AF_ImapPLINK');\nmap.put('4', 'AF_PUP');\nmap.put('5', 'AF_CHAOS');\nmap.put('6', 'AF_NS');\nmap.put('7', 'AF_ISO');\nmap.put('AF_ISO', 'AF_OSI');\nmap.put('8', 'AF_ECmapA');\nmap.put('9', 'AF_DATAKIT');\nmap.put('10', 'AF_CCITT');\nmap.put('11', 'AF_SNA');\nmap.put('12', 'AF_DECnet');\nmap.put('13', 'AF_DLI');\nmap.put('14', 'AF_LAT');\nmap.put('15', 'AF_HYLINK');\nmap.put('16', 'AF_APPLETALK');\nmap.put('17', 'AF_ROUTE');\nmap.put('18', 'AF_LINK');\nmap.put('19', 'pseudo_AF_XTP');\nmap.put('20', 'AF_COIP');\nmap.put('21', 'AF_CNT');\nmap.put('22', 'pseudo_AF_RTIP');\nmap.put('23', 'AF_IPX');\nmap.put('24', 'AF_SIP');\nmap.put('25', 'pseudo_AF_PIP');\nctx.jamf_compliance_reporter.log.socket.inet.family = map.get(ctx.json.inet_family);\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"Map map = new HashMap();\nmap.put('0', 'AF_UNSPEC');\nmap.put('1', 'AF_LOCAL');\nmap.put('AF_LOCAL', 'AF_UNIX');\nmap.put('2', 'AF_INET');\nmap.put('3', 'AF_ImapPLINK');\nmap.put('4', 'AF_PUP');\nmap.put('5', 'AF_CHAOS');\nmap.put('6', 'AF_NS');\nmap.put('7', 'AF_ISO');\nmap.put('AF_ISO', 'AF_OSI');\nmap.put('8', 'AF_ECmapA');\nmap.put('9', 'AF_DATAKIT');\nmap.put('10', 'AF_CCITT');\nmap.put('11', 'AF_SNA');\nmap.put('12', 'AF_DECnet');\nmap.put('13', 'AF_DLI');\nmap.put('14', 'AF_LAT');\nmap.put('15', 'AF_HYLINK');\nmap.put('16', 'AF_APPLETALK');\nmap.put('17', 'AF_ROUTE');\nmap.put('18', 'AF_LINK');\nmap.put('19', 'pseudo_AF_XTP');\nmap.put('20', 'AF_COIP');\nmap.put('21', 'AF_CNT');\nmap.put('22', 'pseudo_AF_RTIP');\nmap.put('23', 'AF_IPX');\nmap.put('24', 'AF_SIP');\nmap.put('25', 'pseudo_AF_PIP');\nctx.jamf_compliance_reporter.log.socket.inet.family = map.get(ctx.json.inet_family);\n"#
                            ),
                        )?;
                    }
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_bind_and_aue_connect"
                }
                let _cond = { event.get_str("event.action") == Some("aue_chdir") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_chdir"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.device") {
                            if let Some(val) = event.get("json.attributes.device") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.device".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.device",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event
                            .rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.file_system_id") {
                            if let Some(val) = event.get("json.attributes.file_system_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.file_system_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.file.system.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.node_id") {
                            if let Some(val) = event.get("json.attributes.node_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.node_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.node.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_group_id") {
                            if let Some(val) = event.get("json.attributes.owner_group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("user.group.id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.attributes.owner_group_name") {
                        event.rename("json.attributes.owner_group_name", "user.group.name")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_user_id") {
                            if let Some(val) = event.get("json.attributes.owner_user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("json.attributes.owner_user_id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.attributes.owner_user_id") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.id",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.name",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.path") {
                        event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                    }
                    let _cond = { event.has_value("json.file_access_mode") };
                    if _cond {
                        // Painless script, resolved to its runners at generation time
                        // Source: int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                        octal_string(
                            event,
                            &OctalString::new(
                                "json.file_access_mode",
                                "jamf_compliance_reporter.log.attributes.file.access_mode",
                            ),
                        );
                    }
                    // End nested pipeline: "pipeline_aue_chdir"
                }
                let _cond = { event.get_str("event.action") == Some("aue_chroot") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_chroot"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.device") {
                            if let Some(val) = event.get("json.attributes.device") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.device".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.device",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.attributes.file_access_mode") {
                        event
                            .rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.file_system_id") {
                            if let Some(val) = event.get("json.attributes.file_system_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.file_system_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.file.system.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.node_id") {
                            if let Some(val) = event.get("json.attributes.node_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.node_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.node.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_group_id") {
                            if let Some(val) = event.get("json.attributes.owner_group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("user.group.id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.attributes.owner_group_name") {
                        event.rename("json.attributes.owner_group_name", "user.group.name")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_user_id") {
                            if let Some(val) = event.get("json.attributes.owner_user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("json.attributes.owner_user_id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.attributes.owner_user_id") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.id",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.name",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.path") {
                        event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                    }
                    // Begin nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_chain_child.parent_path") {
                        event.rename(
                            "json.exec_chain_child.parent_path",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.path",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exec_chain_child.parent_pid") {
                            if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exec_chain_child.parent_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_chain_child.parent_uuid") {
                        event.rename(
                            "json.exec_chain_child.parent_uuid",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.uuid",
                        )?;
                    }
                    // End nested pipeline: "pipeline_exec_chain_child_object"
                    let _cond = { event.has_value("json.file_access_mode") };
                    if _cond {
                        // Painless script, resolved to its runners at generation time
                        // Source: int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                        octal_string(
                            event,
                            &OctalString::new(
                                "json.file_access_mode",
                                "jamf_compliance_reporter.log.attributes.file.access_mode",
                            ),
                        );
                    }
                    // End nested pipeline: "pipeline_aue_chroot"
                }
                let _cond = { event.get_str("event.action") == Some("aue_execve") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_execve"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.device") {
                            if let Some(val) = event.get("json.attributes.device") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.device".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.device",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event
                            .rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.file_system_id") {
                            if let Some(val) = event.get("json.attributes.file_system_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.file_system_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.file.system.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.node_id") {
                            if let Some(val) = event.get("json.attributes.node_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.node_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.node.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_group_id") {
                            if let Some(val) = event.get("json.attributes.owner_group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("user.group.id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.attributes.owner_group_name") {
                        event.rename("json.attributes.owner_group_name", "user.group.name")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_user_id") {
                            if let Some(val) = event.get("json.attributes.owner_user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("json.attributes.owner_user_id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.attributes.owner_user_id") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.id",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.name",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.path") {
                        event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                    }
                    // Begin nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_chain_child.parent_path") {
                        event.rename(
                            "json.exec_chain_child.parent_path",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.path",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exec_chain_child.parent_pid") {
                            if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exec_chain_child.parent_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_chain_child.parent_uuid") {
                        event.rename(
                            "json.exec_chain_child.parent_uuid",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.uuid",
                        )?;
                    }
                    // End nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_args.args") {
                        event.rename("json.exec_args.args", "json.args")?;
                    }
                    if event.has_value("json.exec_args.args_compiled") {
                        event.rename(
                            "json.exec_args.args_compiled",
                            "jamf_compliance_reporter.log.exec_args.args_compiled",
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.exec_env.env.ARCH")
                            && event.get_str("json.exec_env.env.ARCH") != Some("")
                    };
                    if _cond {
                        // Painless script
                        // Source: for (entry in params.replacements.entrySet()) {\n  if (ctx.json.exec_env.env.ARCH == entry.getKey()) {\n    ctx.json.exec_env.env.put('ARCH', entry.getValue());\n  }\n}\nif (!params.allowed.contains(ctx.json.exec_env.env.ARCH)) {\n  return;\n}\nif (ctx.host == null) {\n  HashMap hm = new HashMap();\n  ctx.put('host', hm);\n}\nif (ctx.host.os == null) {\n  HashMap hm = new HashMap();\n  ctx.host.put('os', hm);\n}\nctx.host.os.put('type', ctx.json.exec_env.env.ARCH);\nctx.json.exec_env.env.remove('ARCH');\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan_params(
                            event,
                            cached_painless!(
                                r#"for (entry in params.replacements.entrySet()) {\n  if (ctx.json.exec_env.env.ARCH == entry.getKey()) {\n    ctx.json.exec_env.env.put('ARCH', entry.getValue());\n  }\n}\nif (!params.allowed.contains(ctx.json.exec_env.env.ARCH)) {\n  return;\n}\nif (ctx.host == null) {\n  HashMap hm = new HashMap();\n  ctx.put('host', hm);\n}\nif (ctx.host.os == null) {\n  HashMap hm = new HashMap();\n  ctx.host.put('os', hm);\n}\nctx.host.os.put('type', ctx.json.exec_env.env.ARCH);\nctx.json.exec_env.env.remove('ARCH');\n"#
                            ),
                            cached_params!(
                                "{\"allowed\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"],\"replacements\":{\"macintosh\":\"macos\"}}"
                            ),
                        )?;
                    }
                    if event.has_value("json.exec_env.env.CPU") {
                        event.rename("json.exec_env.env.CPU", "host.architecture")?;
                    }
                    if event.has_value("json.exec_env.env.MALWAREBYTES_GROUP") {
                        event.rename(
                            "json.exec_env.env.MALWAREBYTES_GROUP",
                            "jamf_compliance_reporter.log.exec_env.env.malwarebytes_group",
                        )?;
                    }
                    if event.has_value("json.exec_env.env.PATH") {
                        event.rename(
                            "json.exec_env.env.PATH",
                            "jamf_compliance_reporter.log.exec_env.env.path",
                        )?;
                    }
                    if event.has_value("json.exec_env.env.XPC_FLAGS") {
                        event.rename(
                            "json.exec_env.env.XPC_FLAGS",
                            "jamf_compliance_reporter.log.exec_env.env.xpc.flags",
                        )?;
                    }
                    if event.has_value("json.exec_env.env.XPC_SERVICE_NAME") {
                        event.rename(
                            "json.exec_env.env.XPC_SERVICE_NAME",
                            "jamf_compliance_reporter.log.exec_env.env.xpc.service_name",
                        )?;
                    }
                    if event.has_value("json.exec_env.env_compiled") {
                        event.rename(
                            "json.exec_env.env_compiled",
                            "jamf_compliance_reporter.log.exec_env.env.compiled",
                        )?;
                    }
                    // Painless script
                    // Source: def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n"#
                        ),
                    )?;
                    let _cond = { event.has_value("json.file_access_mode") };
                    if _cond {
                        // Painless script, resolved to its runners at generation time
                        // Source: int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                        octal_string(
                            event,
                            &OctalString::new(
                                "json.file_access_mode",
                                "jamf_compliance_reporter.log.attributes.file.access_mode",
                            ),
                        );
                    }
                    // End nested pipeline: "pipeline_aue_execve"
                }
                let _cond = { event.get_str("event.action") == Some("aue_exit") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_exit"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // Begin nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_chain_child.parent_path") {
                        event.rename(
                            "json.exec_chain_child.parent_path",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.path",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exec_chain_child.parent_pid") {
                            if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exec_chain_child.parent_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_chain_child.parent_uuid") {
                        event.rename(
                            "json.exec_chain_child.parent_uuid",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.uuid",
                        )?;
                    }
                    // End nested pipeline: "pipeline_exec_chain_child_object"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exit.return_value") {
                            if let Some(val) = event.get("json.exit.return_value") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exit.return_value".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.exit.return.value",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.exit.status") {
                            if let Some(val) = event.get("json.exit.status") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.exit.status".into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.exit.status", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_aue_exit"
                }
                let _cond = { event.get_str("event.action") == Some("aue_kill") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_kill"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.signal") {
                            if let Some(val) = event.get("json.arguments.signal") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.signal".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.signal",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // Begin nested pipeline: "pipeline_process_object"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.process.audit_id") {
                            if let Some(val) = event.get("json.process.audit_id") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.process.audit_id".into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.effective_group_id") {
                            if let Some(val) = event.get("json.process.effective_group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.effective_group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.effective.group.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.process.effective_group_name") {
                        event.rename(
                            "json.process.effective_group_name",
                            "jamf_compliance_reporter.log.process.effective.group.name",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.effective_user_id") {
                            if let Some(val) = event.get("json.process.effective_user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.effective_user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.effective.user.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.process.effective_user_id") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.effective.id",
                                json!(
                                    event
                                        .get("json.process.effective_user_id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.process.effective_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.effective.name",
                                json!(
                                    event
                                        .get("json.process.effective_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.process.effective_user_name") {
                        event.rename(
                            "json.process.effective_user_name",
                            "jamf_compliance_reporter.log.process.effective.user.name",
                        )?;
                    }
                    let _cond = {
                        event.has_value("jamf_compliance_reporter.log.process.effective.user.name")
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.process.effective.user.name").map_or_else(String::new, template_to_string)))?;
                            Ok(())
                        })();
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.group_id") {
                            if let Some(val) = event.get("json.process.group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.group.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.process.group_name") {
                        event.rename(
                            "json.process.group_name",
                            "jamf_compliance_reporter.log.process.group.name",
                        )?;
                    }
                    let _cond = { event.has_value("json.process.process_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "process.hash.sha1",
                                json!(
                                    event
                                        .get("json.process.process_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.process.process_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("json.process.process_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.process.process_id") {
                            if let Some(val) = event.get("json.process.process_id") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.process.process_id".into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.process.process_name") {
                        event.rename("json.process.process_name", "process.name")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.session_id") {
                            if let Some(val) = event.get("json.process.session_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.session_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.session.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.terminal_id.addr") {
                            if let Some(val) = event.get("json.process.terminal_id.addr") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.terminal_id.addr".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.terminal_id.addr",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.process.terminal_id.ip_address") {
                            if let Some(val) = event.get("json.process.terminal_id.ip_address") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.process.terminal_id.ip_address".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.terminal_id.ip_address",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("json.process.terminal_id.ip_address");
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    let _cond = {
                        event.has_value(
                            "jamf_compliance_reporter.log.process.terminal_id.ip_address",
                        )
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique("related.ip", json!(event.get("jamf_compliance_reporter.log.process.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                            Ok(())
                        })();
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.process.terminal_id.port") {
                            if let Some(val) = event.get("json.process.terminal_id.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.process.terminal_id.port".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.terminal_id.port",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.terminal_id.type") {
                            if let Some(val) = event.get("json.process.terminal_id.type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.terminal_id.type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.terminal_id.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.user_id") {
                            if let Some(val) = event.get("json.process.user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.user.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.process.user_name") {
                        event.rename(
                            "json.process.user_name",
                            "jamf_compliance_reporter.log.process.user.name",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.process.user.name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.process.user.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // End nested pipeline: "pipeline_process_object"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_kill"
                }
                let _cond = { event.get_str("event.action") == Some("aue_mount") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_mount"
                    if event.has_value("json.texts") {
                        event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                    }
                    // Begin nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_chain_child.parent_path") {
                        event.rename(
                            "json.exec_chain_child.parent_path",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.path",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exec_chain_child.parent_pid") {
                            if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exec_chain_child.parent_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_chain_child.parent_uuid") {
                        event.rename(
                            "json.exec_chain_child.parent_uuid",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.uuid",
                        )?;
                    }
                    // End nested pipeline: "pipeline_exec_chain_child_object"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.path") {
                        event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.device") {
                            if let Some(val) = event.get("json.attributes.device") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.device".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.device",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.attributes.file_access_mode") {
                        event
                            .rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.file_system_id") {
                            if let Some(val) = event.get("json.attributes.file_system_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.file_system_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.file.system.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.node_id") {
                            if let Some(val) = event.get("json.attributes.node_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.node_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.node.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_group_id") {
                            if let Some(val) = event.get("json.attributes.owner_group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.owner.group.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.attributes.owner_group_name") {
                        event.rename(
                            "json.attributes.owner_group_name",
                            "jamf_compliance_reporter.log.attributes.owner.group.name",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_user_id") {
                            if let Some(val) = event.get("json.attributes.owner_user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("json.attributes.owner_user_id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.attributes.owner_user_id") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.id",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.name",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.flags") {
                            if let Some(val) = event.get("json.arguments.flags") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.flags".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.flags",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.file_access_mode") };
                    if _cond {
                        // Painless script, resolved to its runners at generation time
                        // Source: int temp = (int)ctx.json?.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                        octal_string(
                            event,
                            &OctalString::new(
                                "json.file_access_mode",
                                "jamf_compliance_reporter.log.attributes.file.access_mode",
                            ),
                        );
                    }
                    // End nested pipeline: "pipeline_aue_mount"
                }
                let _cond = { event.get_str("event.action") == Some("aue_posix_spawn") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_posix_spawn"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.arguments.child_PID") {
                            if let Some(val) = event.get("json.arguments.child_PID") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.child_PID".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.child.pid",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_args.args") {
                        event.rename("json.exec_args.args", "json.args")?;
                    }
                    if event.has_value("json.exec_args.args_compiled") {
                        event.rename(
                            "json.exec_args.args_compiled",
                            "jamf_compliance_reporter.log.exec_args.args_compiled",
                        )?;
                    }
                    if event.has_value("json.exec_env.env.XPC_FLAGS") {
                        event.rename(
                            "json.exec_env.env.XPC_FLAGS",
                            "jamf_compliance_reporter.log.exec_env.env.xpc.flags",
                        )?;
                    }
                    if event.has_value("json.exec_env.env_compiled") {
                        event.rename(
                            "json.exec_env.env_compiled",
                            "jamf_compliance_reporter.log.exec_env.env.compiled",
                        )?;
                    }
                    if event.has_value("json.path") {
                        event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                    }
                    if event.has_value("json.exec_chain_parent.uuid") {
                        event.rename(
                            "json.exec_chain_parent.uuid",
                            "jamf_compliance_reporter.log.exec_chain_parent.uuid",
                        )?;
                    }
                    // Painless script
                    // Source: def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n"#
                        ),
                    )?;
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_posix_spawn"
                }
                let _cond = {
                    ["aue_remove_from_group", "aue_mac_set_proc"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_remove_from_group_and_aue_mac_set_proc"
                    if event.has_value("json.texts") {
                        event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                    }
                    // Begin nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_chain_child.parent_path") {
                        event.rename(
                            "json.exec_chain_child.parent_path",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.path",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exec_chain_child.parent_pid") {
                            if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exec_chain_child.parent_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_chain_child.parent_uuid") {
                        event.rename(
                            "json.exec_chain_child.parent_uuid",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.uuid",
                        )?;
                    }
                    // End nested pipeline: "pipeline_exec_chain_child_object"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_remove_from_group_and_aue_mac_set_proc"
                }
                let _cond = {
                    [
                        "aue_session_end",
                        "aue_session_update",
                        "aue_session_close",
                        "aue_session_start",
                    ]
                    .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_session"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.am_failure") {
                            if let Some(val) = event.get("json.arguments.am_failure") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.am_failure".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.am_failure",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.am_success") {
                            if let Some(val) = event.get("json.arguments.am_success") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.am_success".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.am_success",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.sflags") {
                            if let Some(val) = event.get("json.arguments.sflags") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.sflags".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.sflags",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_aue_session"
                }
                let _cond = {
                    ["aue_setsockopt", "aue_shutdown"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_arguments"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.fd") {
                            if let Some(val) = event.get("json.arguments.fd") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.fd".into(),
                                            message,
                                        }
                                    })?;
                                event
                                    .set("jamf_compliance_reporter.log.arguments.fd", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_arguments"
                }
                let _cond = { event.get_str("event.action") == Some("aue_ssauthint") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_ssauthint"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.texts") {
                        event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.known_UID_") {
                            if let Some(val) = event.get("json.arguments.known_UID_") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.known_UID_".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.known_uid",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.arguments") {
                        event.rename(
                            "json.arguments",
                            "jamf_compliance_reporter.log.arguments.flattened",
                        )?;
                    }
                    // End nested pipeline: "pipeline_aue_ssauthint"
                }
                let _cond = { event.get_str("event.action") == Some("aue_tasknameforpid") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_tasknameforpid"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.process") {
                            if let Some(val) = event.get("json.arguments.process") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.process".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.process",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.arguments.target_port") {
                            if let Some(val) = event.get("json.arguments.target_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.target_port".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.target.port",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.arguments.task_port") {
                            if let Some(val) = event.get("json.arguments.task_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.task_port".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.task.port",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // End nested pipeline: "pipeline_aue_tasknameforpid"
                }
                let _cond = { event.get_str("event.action") == Some("aue_unmount") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_unmount"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // Begin nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_chain_child.parent_path") {
                        event.rename(
                            "json.exec_chain_child.parent_path",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.path",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exec_chain_child.parent_pid") {
                            if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exec_chain_child.parent_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_chain_child.parent_uuid") {
                        event.rename(
                            "json.exec_chain_child.parent_uuid",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.uuid",
                        )?;
                    }
                    // End nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.path") {
                        event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.device") {
                            if let Some(val) = event.get("json.attributes.device") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.device".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.device",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.attributes.file_access_mode") {
                        event
                            .rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.file_system_id") {
                            if let Some(val) = event.get("json.attributes.file_system_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.file_system_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.file.system.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.node_id") {
                            if let Some(val) = event.get("json.attributes.node_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.node_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.node.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_group_id") {
                            if let Some(val) = event.get("json.attributes.owner_group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.attributes.owner.group.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.attributes.owner_group_name") {
                        event.rename(
                            "json.attributes.owner_group_name",
                            "jamf_compliance_reporter.log.attributes.owner.group.name",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.attributes.owner_user_id") {
                            if let Some(val) = event.get("json.attributes.owner_user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.attributes.owner_user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("json.attributes.owner_user_id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.attributes.owner_user_id") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.id",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.name",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.attributes.owner_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("json.attributes.owner_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.file_access_mode") };
                    if _cond {
                        // Painless script, resolved to its runners at generation time
                        // Source: int temp = (int)ctx.json.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                        octal_string(
                            event,
                            &OctalString::new(
                                "json.file_access_mode",
                                "jamf_compliance_reporter.log.attributes.file.access_mode",
                            ),
                        );
                    }
                    // End nested pipeline: "pipeline_aue_unmount"
                }
                let _cond = { event.get_str("event.action") == Some("aue_fork") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_fork"
                    if event.has_value("json.exec_chain_parent.uuid") {
                        event.rename(
                            "json.exec_chain_parent.uuid",
                            "jamf_compliance_reporter.log.exec_chain_parent.uuid",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.arguments.child_PID") {
                            if let Some(val) = event.get("json.arguments.child_PID") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.child_PID".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.child.pid",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_fork"
                }
                let _cond = {
                    ["aue_getauid", "aue_lw_login", "aue_settimeofday"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                }
                let _cond = { event.get_str("event.action") == Some("aue_listen") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_listen"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.fd") {
                            if let Some(val) = event.get("json.arguments.fd") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.fd".into(),
                                            message,
                                        }
                                    })?;
                                event
                                    .set("jamf_compliance_reporter.log.arguments.fd", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // Begin nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_chain_child.parent_path") {
                        event.rename(
                            "json.exec_chain_child.parent_path",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.path",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exec_chain_child.parent_pid") {
                            if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exec_chain_child.parent_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_chain_child.parent_uuid") {
                        event.rename(
                            "json.exec_chain_child.parent_uuid",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.uuid",
                        )?;
                    }
                    // End nested pipeline: "pipeline_exec_chain_child_object"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_listen"
                }
                let _cond = { event.get_str("event.action") == Some("aue_logout") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_logout"
                    // Begin nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_chain_child.parent_path") {
                        event.rename(
                            "json.exec_chain_child.parent_path",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.path",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exec_chain_child.parent_pid") {
                            if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exec_chain_child.parent_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_chain_child.parent_uuid") {
                        event.rename(
                            "json.exec_chain_child.parent_uuid",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.uuid",
                        )?;
                    }
                    // End nested pipeline: "pipeline_exec_chain_child_object"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_logout"
                }
                let _cond = { event.get_str("event.action") == Some("aue_pidfortask") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_pidfortask"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.arguments.pid") {
                            if let Some(val) = event.get("json.arguments.pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.pid".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("jamf_compliance_reporter.log.arguments.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.arguments.port") {
                            if let Some(val) = event.get("json.arguments.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.port".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.port",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // End nested pipeline: "pipeline_aue_pidfortask"
                }
                let _cond = { event.get_str("event.action") == Some("aue_ptrace") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_ptrace"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.addr") {
                            if let Some(val) = event.get("json.arguments.addr") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.addr".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.addr",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.data") {
                            if let Some(val) = event.get("json.arguments.data") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.data".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.data",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.process") {
                            if let Some(val) = event.get("json.arguments.process") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.process".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.process",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.request") {
                            if let Some(val) = event.get("json.arguments.request") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.request".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.request",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // Begin nested pipeline: "pipeline_exec_chain_child_object"
                    if event.has_value("json.exec_chain_child.parent_path") {
                        event.rename(
                            "json.exec_chain_child.parent_path",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.path",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.exec_chain_child.parent_pid") {
                            if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.exec_chain_child.parent_pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.parent.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.exec_chain_child.parent_uuid") {
                        event.rename(
                            "json.exec_chain_child.parent_uuid",
                            "jamf_compliance_reporter.log.exec_chain_child.parent.uuid",
                        )?;
                    }
                    // End nested pipeline: "pipeline_exec_chain_child_object"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_ptrace"
                }
                let _cond = { event.get_str("event.action") == Some("aue_setpriority") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_setpriority"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.arguments.priority") {
                            if let Some(val) = event.get("json.arguments.priority") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.priority".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.priority",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.which") {
                            if let Some(val) = event.get("json.arguments.which") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.which".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.which",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.who") {
                            if let Some(val) = event.get("json.arguments.who") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.who".into(),
                                            message,
                                        }
                                    })?;
                                event
                                    .set("jamf_compliance_reporter.log.arguments.who", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_aue_setpriority"
                }
                let _cond = { event.get_str("event.action") == Some("aue_socketpair") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_socketpair"
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.domain") {
                            if let Some(val) = event.get("json.arguments.domain") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.domain".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.domain",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.protocol") {
                            if let Some(val) = event.get("json.arguments.protocol") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.protocol".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.protocol",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.arguments.type") {
                            if let Some(val) = event.get("json.arguments.type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.arguments.type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_aue_socketpair"
                }
                let _cond = { event.get_str("event.action") == Some("aue_taskforpid") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_taskforpid"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.arguments.target_port") {
                            if let Some(val) = event.get("json.arguments.target_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.target_port".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.target.port",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.arguments.task_port") {
                            if let Some(val) = event.get("json.arguments.task_port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.task_port".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.arguments.task.port",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // Begin nested pipeline: "pipeline_process_object"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.process.audit_id") {
                            if let Some(val) = event.get("json.process.audit_id") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.process.audit_id".into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.effective_group_id") {
                            if let Some(val) = event.get("json.process.effective_group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.effective_group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.effective.group.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.process.effective_group_name") {
                        event.rename(
                            "json.process.effective_group_name",
                            "jamf_compliance_reporter.log.process.effective.group.name",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.effective_user_id") {
                            if let Some(val) = event.get("json.process.effective_user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.effective_user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.effective.user.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.process.effective_user_id") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.effective.id",
                                json!(
                                    event
                                        .get("json.process.effective_user_id")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.process.effective_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.effective.name",
                                json!(
                                    event
                                        .get("json.process.effective_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.process.effective_user_name") {
                        event.rename(
                            "json.process.effective_user_name",
                            "jamf_compliance_reporter.log.process.effective.user.name",
                        )?;
                    }
                    let _cond = {
                        event.has_value("jamf_compliance_reporter.log.process.effective.user.name")
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.process.effective.user.name").map_or_else(String::new, template_to_string)))?;
                            Ok(())
                        })();
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.group_id") {
                            if let Some(val) = event.get("json.process.group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.group.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.process.group_name") {
                        event.rename(
                            "json.process.group_name",
                            "jamf_compliance_reporter.log.process.group.name",
                        )?;
                    }
                    let _cond = { event.has_value("json.process.process_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "process.hash.sha1",
                                json!(
                                    event
                                        .get("json.process.process_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.process.process_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("json.process.process_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.process.process_id") {
                            if let Some(val) = event.get("json.process.process_id") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.process.process_id".into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.process.process_name") {
                        event.rename("json.process.process_name", "process.name")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.session_id") {
                            if let Some(val) = event.get("json.process.session_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.session_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.session.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.terminal_id.addr") {
                            if let Some(val) = event.get("json.process.terminal_id.addr") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.terminal_id.addr".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.terminal_id.addr",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.process.terminal_id.ip_address") {
                            if let Some(val) = event.get("json.process.terminal_id.ip_address") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.process.terminal_id.ip_address".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.terminal_id.ip_address",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("json.process.terminal_id.ip_address");
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    let _cond = {
                        event.has_value(
                            "jamf_compliance_reporter.log.process.terminal_id.ip_address",
                        )
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique("related.ip", json!(event.get("jamf_compliance_reporter.log.process.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                            Ok(())
                        })();
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.process.terminal_id.port") {
                            if let Some(val) = event.get("json.process.terminal_id.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.process.terminal_id.port".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.terminal_id.port",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.terminal_id.type") {
                            if let Some(val) = event.get("json.process.terminal_id.type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.terminal_id.type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.terminal_id.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.process.user_id") {
                            if let Some(val) = event.get("json.process.user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.process.user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.process.user.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.process.user_name") {
                        event.rename(
                            "json.process.user_name",
                            "jamf_compliance_reporter.log.process.user.name",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.process.user.name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.process.user.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // End nested pipeline: "pipeline_process_object"
                    // End nested pipeline: "pipeline_aue_taskforpid"
                }
                let _cond = { event.get_str("event.action") == Some("aue_wait4") };
                if _cond {
                    // Begin nested pipeline: "pipeline_aue_wait4"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.arguments.pid") {
                            if let Some(val) = event.get("json.arguments.pid") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.arguments.pid".into(),
                                        message,
                                    }
                                })?;
                                event.set("process.pid", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // Begin nested pipeline: "pipeline_identity_object"
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.identity.signer_id") {
                        event.rename(
                            "json.identity.signer_id",
                            "jamf_compliance_reporter.log.identity.signer.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.identity.team_id") {
                        event.rename(
                            "json.identity.team_id",
                            "jamf_compliance_reporter.log.identity.team.id",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_identity_object"
                    // End nested pipeline: "pipeline_aue_wait4"
                }
                // End nested pipeline: "pipeline_audit"
            }

            let _cond = {
                [
                    "AUDIO_VIDEO_DEVICE_EVENT",
                    "AUDIT_CLASS_VERIFICATION_EVENT",
                    "COMPLIANCE_REPORTER_TAMPER_EVENT",
                    "FILE_EVENT",
                    "GATEKEEPER_INFO_EVENT",
                    "GATEKEEPER_MANUAL_OVERRIDES",
                    "GATEKEEPER_QUARANTINE_LOG",
                    "HARDWARE_EVENT",
                    "LICENSE_INFO_EVENT",
                    "PREFERENCE_LIST_EVENT",
                    "PRINT_EVENT_INFORMATION",
                    "PROHIBITED_APP_BLOCKED",
                    "SIGNAL_EVENT",
                    "UNIFIED_LOG_EVENT",
                    "XPROTECT_DEFINITIONS_VERSION_INFO",
                    "XPROTECT_EVENT_LOG",
                ]
                .contains(&event.get_str("json.header.event_name").unwrap_or(""))
            };
            if _cond {
                // Begin nested pipeline: "pipeline_event"
                event.set("event.kind", json!("event"))?;
                event.set("jamf_compliance_reporter.log.dataset", json!("event"))?;
                event.append("event.category", json!("process"))?;
                event.set("host.os.type", json!("macos"))?;
                let _cond = {
                    !(["UNIFIED_LOG_EVENT", "XPROTECT_EVENT_LOG"]
                        .contains(&event.get_str("json.header.event_name").unwrap_or("")))
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json._event_score") {
                        if let Some(val) = event.get("json._event_score") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json._event_score".into(),
                                    message,
                                }
                            })?;
                            event.set("jamf_compliance_reporter.log.event_score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("json.header.event_name") {
                    event.rename("json.header.event_name", "event.action")?;
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = { event.get_i64("json.header.time_seconds_epoch") != Some(0) };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.header.time_seconds_epoch")
                        {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.header.time_seconds_epoch".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.host_info.host_name") {
                    event.rename("json.host_info.host_name", "host.hostname")?;
                }
                let _cond = { event.has_value("host.hostname") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("host.hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("json.host_info.host_uuid") {
                    event.rename(
                        "json.host_info.host_uuid",
                        "jamf_compliance_reporter.log.host_info.host.uuid",
                    )?;
                }
                if event.has_value("json.host_info.osversion") {
                    event.rename("json.host_info.osversion", "host.os.version")?;
                }
                let _cond = { event.has_value("json.host_info.primary_mac_address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "host.mac",
                            json!(
                                event
                                    .get("json.host_info.primary_mac_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("host.mac") {
                    gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
                }
                if event.has_value("host.mac") {
                    map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
                }
                if event.has_value("json.host_info.serial_number") {
                    event.rename("json.host_info.serial_number", "host.id")?;
                }
                let _cond = { event.get_str("event.action") == Some("audio_video_device_event") };
                if _cond {
                    // Begin nested pipeline: "pipeline_audio_video_device_event"
                    if event.has_value("json.audio_video_device_info.audio_device_creator") {
                        event.rename("json.audio_video_device_info.audio_device_creator", "jamf_compliance_reporter.log.audio_video_device_info.audio_device.creator")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.audio_video_device_info.audio_device_hog_mode") {
                            if let Some(val) =
                                event.get("json.audio_video_device_info.audio_device_hog_mode")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path:
                                                "json.audio_video_device_info.audio_device_hog_mode"
                                                    .into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.audio_video_device_info.audio_device.hog_mode", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.audio_video_device_info.audio_device_id") {
                            if let Some(val) =
                                event.get("json.audio_video_device_info.audio_device_id")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.audio_video_device_info.audio_device_id"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.audio_video_device_info.audio_device.id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.audio_video_device_info.audio_device_manufacturer") {
                        event.rename("json.audio_video_device_info.audio_device_manufacturer", "jamf_compliance_reporter.log.audio_video_device_info.audio_device.manufacturer")?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.audio_video_device_info.audio_device_running") {
                            if let Some(val) =
                                event.get("json.audio_video_device_info.audio_device_running")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.audio_video_device_info.audio_device_running"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.audio_video_device_info.audio_device.running", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.audio_video_device_info.audio_device_uuid") {
                        event.rename("json.audio_video_device_info.audio_device_uuid", "jamf_compliance_reporter.log.audio_video_device_info.audio_device.uuid")?;
                    }
                    if event.has_value("json.audio_video_device_info.device_status") {
                        event.rename(
                            "json.audio_video_device_info.device_status",
                            "jamf_compliance_reporter.log.audio_video_device_info.device_status",
                        )?;
                    }
                    // End nested pipeline: "pipeline_audio_video_device_event"
                }
                let _cond =
                    { event.get_str("event.action") == Some("audit_class_verification_event") };
                if _cond {
                    // Begin nested pipeline: "pipeline_audit_class_verification_event"
                    if event.has_value("json.audit_class_verification_info.contents") {
                        event.rename(
                            "json.audit_class_verification_info.contents",
                            "jamf_compliance_reporter.log.audit_class_verification_info.contents",
                        )?;
                    }
                    if event.has_value("json.audit_class_verification_info.osversion") {
                        event.rename(
                            "json.audit_class_verification_info.osversion",
                            "jamf_compliance_reporter.log.audit_class_verification_info.os.version",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.audit_class_verification_info.restored_default") {
                            if let Some(val) =
                                event.get("json.audit_class_verification_info.restored_default")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                path: "json.audit_class_verification_info.restored_default".into(),
                message,
                }
                                    })?;
                                event.set("jamf_compliance_reporter.log.audit_class_verification_info.restored_default", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.audit_class_verification_info.status") {
                            if let Some(val) =
                                event.get("json.audit_class_verification_info.status")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.audit_class_verification_info.status"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.audit_class_verification_info.status", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.audit_class_verification_info.status_str") {
                        event.rename(
                            "json.audit_class_verification_info.status_str",
                            "jamf_compliance_reporter.log.audit_class_verification_info.status_str",
                        )?;
                    }
                    // End nested pipeline: "pipeline_audit_class_verification_event"
                }
                let _cond = {
                    ["compliance_reporter_tamper_event", "file_event"]
                        .contains(&event.get_str("event.action").unwrap_or(""))
                };
                if _cond {
                    // Begin nested pipeline: "pipeline_compliance_reporter_tamper_event_and_file_event_info"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.file_event_info.eventid_wrapped") {
                            if let Some(val) = event.get("json.file_event_info.eventid_wrapped") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.eventid_wrapped".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.eventid_wrapped",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.file_event_info.hash") {
                        event.rename("json.file_event_info.hash", "file.hash.sha1")?;
                    }
                    let _cond = { event.has_value("file.hash.sha1") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("file.hash.sha1")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.file_event_info.history_done") {
                            if let Some(val) = event.get("json.file_event_info.history_done") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.history_done".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.history_done",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_change_owner") {
                            if let Some(val) = event.get("json.file_event_info.item_change_owner") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_change_owner".into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.file_event_info.item.change_owner", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_cloned") {
                            if let Some(val) = event.get("json.file_event_info.item_cloned") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_cloned".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.item.cloned",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_created") {
                            if let Some(val) = event.get("json.file_event_info.item_created") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_created".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.item.created",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_extended_attribute_modified")
                        {
                            if let Some(val) =
                                event.get("json.file_event_info.item_extended_attribute_modified")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                path: "json.file_event_info.item_extended_attribute_modified".into(),
                message,
                }
                                    })?;
                                event.set("jamf_compliance_reporter.log.file_event_info.item.extended_attribute_modified", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_finder_info_modified") {
                            if let Some(val) =
                                event.get("json.file_event_info.item_finder_info_modified")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_finder_info_modified"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.file_event_info.item.finder_info_modified", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_inode_metadata_modified") {
                            if let Some(val) =
                                event.get("json.file_event_info.item_inode_metadata_modified")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path:
                                                "json.file_event_info.item_inode_metadata_modified"
                                                    .into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.file_event_info.item.inode_metadata_modified", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_is_directory") {
                            if let Some(val) = event.get("json.file_event_info.item_is_directory") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_is_directory".into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.file_event_info.item.is_directory", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_is_file") {
                            if let Some(val) = event.get("json.file_event_info.item_is_file") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_is_file".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.item.is_file",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_is_hard_link") {
                            if let Some(val) = event.get("json.file_event_info.item_is_hard_link") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_is_hard_link".into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.file_event_info.item.is_hard_link", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_is_last_hard_link") {
                            if let Some(val) =
                                event.get("json.file_event_info.item_is_last_hard_link")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_is_last_hard_link"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.file_event_info.item.is_last_hard_link", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_is_sym_link") {
                            if let Some(val) = event.get("json.file_event_info.item_is_sym_link") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_is_sym_link".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.item.is_sym_link",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_removed") {
                            if let Some(val) = event.get("json.file_event_info.item_removed") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_removed".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.item.removed",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_renamed") {
                            if let Some(val) = event.get("json.file_event_info.item_renamed") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_renamed".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.item.renamed",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.item_updated") {
                            if let Some(val) = event.get("json.file_event_info.item_updated") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.item_updated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.item.updated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.kernel_dropped") {
                            if let Some(val) = event.get("json.file_event_info.kernel_dropped") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.kernel_dropped".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.kernel_dropped",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.mount") {
                            if let Some(val) = event.get("json.file_event_info.mount") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.mount".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.mount",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.must_scan_sub_dir") {
                            if let Some(val) = event.get("json.file_event_info.must_scan_sub_dir") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.must_scan_sub_dir".into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.file_event_info.must_scan_sub_dir", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.none") {
                            if let Some(val) = event.get("json.file_event_info.none") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.none".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.none",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.own_event") {
                            if let Some(val) = event.get("json.file_event_info.own_event") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.own_event".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.own_event",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.file_event_info.path") {
                        event.rename("json.file_event_info.path", "file.path")?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.file_event_info.root_changed") {
                            if let Some(val) = event.get("json.file_event_info.root_changed") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.root_changed".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.root_changed",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.unmount") {
                            if let Some(val) = event.get("json.file_event_info.unmount") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.unmount".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.unmount",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.file_event_info.user_dropped") {
                            if let Some(val) = event.get("json.file_event_info.user_dropped") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.file_event_info.user_dropped".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.file_event_info.user_dropped",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // End nested pipeline: "pipeline_compliance_reporter_tamper_event_and_file_event_info"
                }
                let _cond = { event.get_str("event.action") == Some("gatekeeper_info_event") };
                if _cond {
                    // Begin nested pipeline: "pipeline_gatekeeper_info_event"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.assessments_enabled") {
                            if let Some(val) =
                                event.get("json.event_attributes.assessments_enabled")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.event_attributes.assessments_enabled".into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.event_attributes.assessments_enabled", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.event_attributes.dev_id_enabled") {
                            if let Some(val) = event.get("json.event_attributes.dev_id_enabled") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.event_attributes.dev_id_enabled".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.dev_id_enabled",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.event_attributes.opaque_version") {
                        event.rename(
                            "json.event_attributes.opaque_version",
                            "jamf_compliance_reporter.log.event_attributes.opaque_version",
                        )?;
                    }
                    if event.has_value("json.event_attributes.version") {
                        event.rename(
                            "json.event_attributes.version",
                            "jamf_compliance_reporter.log.event_attributes.version",
                        )?;
                    }
                    // End nested pipeline: "pipeline_gatekeeper_info_event"
                }
                let _cond =
                    { event.get_str("event.action") == Some("gatekeeper_manual_overrides") };
                if _cond {
                    // Begin nested pipeline: "pipeline_gatekeeper_manual_overrides"
                    let _cond = {
                        event
                            .get("json.event_attributes.attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.event_attributes.attributes").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(date_str) =
                                            event.get_as_string("_ingest._value.ctime")
                                        {
                                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                                Some(parsed) => {
                                                    event.set("_ingest._value.ctime", parsed)?
                                                }
                                                None => {
                                                    return Err(TransformError::ParseError {
                                                        path: "_ingest._value.ctime".into(),
                                                        message: format!(
                                                            "unable to parse date [{date_str}]"
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event.set("_ingest.on_failure_processor_type", "date")?;
                                        event.remove("_ingest._value.ctime");
                                        event.append(
                                            "error.message",
                                            json!(
                                                event
                                                    .get("_ingest.on_failure_message")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "json.event_attributes.attributes",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes.attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.event_attributes.attributes").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(date_str) =
                                            event.get_as_string("_ingest._value.mtime")
                                        {
                                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                                Some(parsed) => {
                                                    event.set("_ingest._value.mtime", parsed)?
                                                }
                                                None => {
                                                    return Err(TransformError::ParseError {
                                                        path: "_ingest._value.mtime".into(),
                                                        message: format!(
                                                            "unable to parse date [{date_str}]"
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event.set("_ingest.on_failure_processor_type", "date")?;
                                        event.remove("_ingest._value.mtime");
                                        event.append(
                                            "error.message",
                                            json!(
                                                event
                                                    .get("_ingest.on_failure_message")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "json.event_attributes.attributes",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    if event.has_value("json.event_attributes.attributes") {
                        event.rename(
                            "json.event_attributes.attributes",
                            "jamf_compliance_reporter.log.event_attributes.attributes",
                        )?;
                    }
                    if event.has_value("json.event_attributes.path") {
                        event.rename(
                            "json.event_attributes.path",
                            "jamf_compliance_reporter.log.event_attributes.path",
                        )?;
                    }
                    // End nested pipeline: "pipeline_gatekeeper_manual_overrides"
                }
                let _cond = { event.get_str("event.action") == Some("gatekeeper_quarantine_log") };
                if _cond {
                    // Begin nested pipeline: "pipeline_gatekeeper_quarantine_log"
                    let _cond = {
                        event
                            .get("json.event_attributes.attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes.attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.rename(
                                    "_ingest._value.QuarantineAgentBundleIdentifier",
                                    "_ingest._value.quarantine.agent_bundle_identifier",
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes.attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes.attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.rename(
                                    "_ingest._value.QuarantineAgentName",
                                    "_ingest._value.quarantine.agent_name",
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes.attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes.attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.rename(
                                    "_ingest._value.QuarantineDataURLString",
                                    "_ingest._value.quarantine.data_url_string",
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes.attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes.attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.rename(
                                    "_ingest._value.QuarantineEventIdentifier",
                                    "_ingest._value.quarantine.event_identifier",
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes.attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes.attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.rename(
                                    "_ingest._value.QuarantineOriginURLString",
                                    "_ingest._value.quarantine.origin_url_string",
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes.attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.event_attributes.attributes").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(date_str) = event
                                            .get_as_string("_ingest._value.QuarantineTimeStamp")
                                        {
                                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                                Some(parsed) => event.set(
                                                    "_ingest._value.quarantine.timestamp",
                                                    parsed,
                                                )?,
                                                None => {
                                                    return Err(TransformError::ParseError {
                                                        path: "_ingest._value.QuarantineTimeStamp"
                                                            .into(),
                                                        message: format!(
                                                            "unable to parse date [{date_str}]"
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event.set("_ingest.on_failure_processor_type", "date")?;
                                        event.remove("_ingest._value.QuarantineTimeStamp");
                                        event.append(
                                            "error.message",
                                            json!(
                                                event
                                                    .get("_ingest.on_failure_message")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "json.event_attributes.attributes",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes.attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.event_attributes.attributes").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if event
                                            .remove("_ingest._value.QuarantineTimeStamp")
                                            .is_none()
                                        {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value.QuarantineTimeStamp".into(),
                                            });
                                        }
                                        Ok(())
                                    })();
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "json.event_attributes.attributes",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    if event.has_value("json.event_attributes.attributes") {
                        event.rename(
                            "json.event_attributes.attributes",
                            "jamf_compliance_reporter.log.event_attributes.attributes",
                        )?;
                    }
                    if event.has_value("json.event_attributes.path") {
                        event.rename(
                            "json.event_attributes.path",
                            "jamf_compliance_reporter.log.event_attributes.path",
                        )?;
                    }
                    // End nested pipeline: "pipeline_gatekeeper_quarantine_log"
                }
                let _cond = { event.get_str("event.action") == Some("hardware_event") };
                if _cond {
                    // Begin nested pipeline: "pipeline_hardware_event"
                    if event.has_value("json.hardware_event_info.device_attributes.IOCFPlugInTypes")
                    {
                        event.rename("json.hardware_event_info.device_attributes.IOCFPlugInTypes", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.cf_plugin_types")?;
                    }
                    if event
                        .has_value("json.hardware_event_info.device_attributes.IOClassNameOverride")
                    {
                        event.rename("json.hardware_event_info.device_attributes.IOClassNameOverride", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.class_name_override")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.CapabilityFlags") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.CapabilityFlags") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.CapabilityFlags".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.capability_flags", converted)?;
                }
                }
                        Ok(())
                    })();
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.CurrentPowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.CurrentPowerState") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.CurrentPowerState".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.current_power_state", converted)?;
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.DevicePowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.DevicePowerState") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.DevicePowerState".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.device_power_state", converted)?;
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.DriverPowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.DriverPowerState") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.DriverPowerState".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.driver_power_state", converted)?;
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.MaxPowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.MaxPowerState") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.IOPowerManagement.MaxPowerState".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.max_power_state", converted)?;
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.hardware_event_info.device_attributes.Removable") {
                        event.rename("json.hardware_event_info.device_attributes.Removable", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.removable")?;
                    }
                    if event
                        .has_value("json.hardware_event_info.device_attributes.USB Product Name")
                    {
                        event.rename("json.hardware_event_info.device_attributes.USB Product Name", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.usb.product_name")?;
                    }
                    if event.has_value("json.hardware_event_info.device_attributes.USB Vendor Name")
                    {
                        event.rename("json.hardware_event_info.device_attributes.USB Vendor Name", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.usb.vendor_name")?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event
                            .has_value("json.hardware_event_info.device_attributes.iSerialNumber")
                        {
                            if let Some(val) = event
                                .get("json.hardware_event_info.device_attributes.iSerialNumber")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                path: "json.hardware_event_info.device_attributes.iSerialNumber".into(),
                message,
                }
                                })?;
                                event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.iserial_number", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.hardware_event_info.device_class") {
                        event.rename(
                            "json.hardware_event_info.device_class",
                            "jamf_compliance_reporter.log.hardware_event_info.device.class",
                        )?;
                    }
                    if event.has_value("json.hardware_event_info.device_name") {
                        event.rename(
                            "json.hardware_event_info.device_name",
                            "jamf_compliance_reporter.log.hardware_event_info.device.name",
                        )?;
                    }
                    if event.has_value("json.hardware_event_info.device_status") {
                        event.rename(
                            "json.hardware_event_info.device_status",
                            "jamf_compliance_reporter.log.hardware_event_info.device.status",
                        )?;
                    }
                    // End nested pipeline: "pipeline_hardware_event"
                }
                let _cond = { event.get_str("event.action") == Some("license_info_event") };
                if _cond {
                    // Begin nested pipeline: "pipeline_license_info_event"
                    if event.has_value("json.ComplianceReporter_license_info.email") {
                        event.rename("json.ComplianceReporter_license_info.email", "user.email")?;
                    }
                    let _cond = { event.has_value("user.email") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("user.email")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = {
                        event.has_value("json.ComplianceReporter_license_info.expiration_date")
                            && event.get_i64("json.ComplianceReporter_license_info.expiration_date")
                                != Some(0)
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string(
                                "json.ComplianceReporter_license_info.expiration_date",
                            ) {
                                match parse_date_out(&date_str, &["dd/MM/yyyy"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.compliancereporter_license_info.expiration_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.ComplianceReporter_license_info.expiration_date".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.ComplianceReporter_license_info.status") {
                        event.rename(
                            "json.ComplianceReporter_license_info.status",
                            "jamf_compliance_reporter.log.compliancereporter_license_info.status",
                        )?;
                    }
                    let _cond = {
                        event.has_value("json.ComplianceReporter_license_info.time_seconds_epoch")
                            && event
                                .get_str("json.ComplianceReporter_license_info.time_seconds_epoch")
                                != Some("0")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string(
                                "json.ComplianceReporter_license_info.time_seconds_epoch",
                            ) {
                                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.compliancereporter_license_info.time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.ComplianceReporter_license_info.time_seconds_epoch".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.ComplianceReporter_license_info.type") {
                        event.rename(
                            "json.ComplianceReporter_license_info.type",
                            "jamf_compliance_reporter.log.compliancereporter_license_info.type",
                        )?;
                    }
                    if event.has_value("json.ComplianceReporter_license_info.version") {
                        event.rename(
                            "json.ComplianceReporter_license_info.version",
                            "jamf_compliance_reporter.log.compliancereporter_license_info.version",
                        )?;
                    }
                    // End nested pipeline: "pipeline_license_info_event"
                }
                let _cond = { event.get_str("event.action") == Some("preference_list_event") };
                if _cond {
                    // Begin nested pipeline: "pipeline_preference_list_event"
                    if event.has_value("json.event_attributes.AuditEventExcludedProcesses") {
                        event.rename("json.event_attributes.AuditEventExcludedProcesses", "jamf_compliance_reporter.log.event_attributes.audit_event.excluded_processes")?;
                    }
                    if event.has_value("json.event_attributes.AuditEventExcludedUsers") {
                        event.rename("json.event_attributes.AuditEventExcludedUsers", "jamf_compliance_reporter.log.event_attributes.audit_event.excluded_users")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.AuditEventLogVerboseMessages") {
                            if let Some(val) =
                                event.get("json.event_attributes.AuditEventLogVerboseMessages")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path:
                                                "json.event_attributes.AuditEventLogVerboseMessages"
                                                    .into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.event_attributes.audit_event_log_verbose_messages", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.AuditLevel") {
                            if let Some(val) = event.get("json.event_attributes.AuditLevel") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.event_attributes.AuditLevel".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.audit_level",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.event_attributes.FileEventExclusionPaths") {
                        event.rename("json.event_attributes.FileEventExclusionPaths", "jamf_compliance_reporter.log.event_attributes.file_event.exclusion_paths")?;
                    }
                    if event.has_value("json.event_attributes.FileEventInclusionPaths") {
                        event.rename("json.event_attributes.FileEventInclusionPaths", "jamf_compliance_reporter.log.event_attributes.file_event.inclusion_paths")?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.FileEventUseFuzzyMatch") {
                            if let Some(val) =
                                event.get("json.event_attributes.FileEventUseFuzzyMatch")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.event_attributes.FileEventUseFuzzyMatch".into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.event_attributes.file_event.use_fuzzy_match", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.event_attributes.FileLicenseInfo.LicenseEmail") {
                        event.rename(
                            "json.event_attributes.FileLicenseInfo.LicenseEmail",
                            "user.email",
                        )?;
                    }
                    let _cond = { event.has_value("user.email") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("user.email")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = {
                        event.get_str("json.event_attributes.FileLicenseInfo.LicenseExpirationDate")
                            != Some("0")
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string(
                                "json.event_attributes.FileLicenseInfo.LicenseExpirationDate",
                            ) {
                                match parse_date_out(&date_str, &["dd/MM/yyyy"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.file_license_info.license_expiration_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.event_attributes.FileLicenseInfo.LicenseExpirationDate".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.event_attributes.FileLicenseInfo.LicenseKey") {
                        event.rename("json.event_attributes.FileLicenseInfo.LicenseKey", "jamf_compliance_reporter.log.event_attributes.file_license_info.license_key")?;
                    }
                    if event.has_value("json.event_attributes.FileLicenseInfo.LicenseType") {
                        event.rename("json.event_attributes.FileLicenseInfo.LicenseType", "jamf_compliance_reporter.log.event_attributes.file_license_info.license_type")?;
                    }
                    if event.has_value("json.event_attributes.FileLicenseInfo.LicenseVersion") {
                        event.rename("json.event_attributes.FileLicenseInfo.LicenseVersion", "jamf_compliance_reporter.log.event_attributes.file_license_info.license_version")?;
                    }
                    if event.has_value("json.event_attributes.LogFileLocation") {
                        event.rename(
                            "json.event_attributes.LogFileLocation",
                            "jamf_compliance_reporter.log.event_attributes.log.file.location",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.LogFileMaxNumberBackups") {
                            if let Some(val) =
                                event.get("json.event_attributes.LogFileMaxNumberBackups")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.event_attributes.LogFileMaxNumberBackups"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.event_attributes.log.file.max_number_backups", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                        if event.has_value("json.event_attributes.LogFileMaxSizeMegaBytes") {
                            if let Some(val) =
                                event.get("json.event_attributes.LogFileMaxSizeMegaBytes")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.event_attributes.LogFileMaxSizeMegaBytes"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.event_attributes.log.file.max_size_mega_bytes", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.event_attributes.LogFileOwnership") {
                        event.rename(
                            "json.event_attributes.LogFileOwnership",
                            "jamf_compliance_reporter.log.event_attributes.log.file.ownership",
                        )?;
                    }
                    if event.has_value("json.event_attributes.LogFilePermission") {
                        event.rename(
                            "json.event_attributes.LogFilePermission",
                            "jamf_compliance_reporter.log.event_attributes.log.file.permission",
                        )?;
                    }
                    if event.has_value("json.event_attributes.LogRemoteEndpointEnabled") {
                        event.rename("json.event_attributes.LogRemoteEndpointEnabled", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_enabled")?;
                    }
                    if event.has_value("json.event_attributes.LogRemoteEndpointType") {
                        event.rename("json.event_attributes.LogRemoteEndpointType", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type")?;
                    }
                    if event.has_value(
                        "json.event_attributes.LogRemoteEndpointTypeAWSKinesis.AccessKeyId",
                    ) {
                        event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.AccessKeyId", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.access_key_id")?;
                    }
                    if event
                        .has_value("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.Region")
                    {
                        event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.Region", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.region")?;
                    }
                    if event.has_value(
                        "json.event_attributes.LogRemoteEndpointTypeAWSKinesis.SecretKey",
                    ) {
                        event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.SecretKey", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.secret_key")?;
                    }
                    if event.has_value(
                        "json.event_attributes.LogRemoteEndpointTypeAWSKinesis.StreamName",
                    ) {
                        event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.StreamName", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.stream_name")?;
                    }
                    if event.has_value("json.event_attributes.LogRemoteEndpointURL") {
                        event.rename(
                            "json.event_attributes.LogRemoteEndpointURL",
                            "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_url",
                        )?;
                    }
                    if event.has_value("json.event_attributes.UnifiedLogPredicates") {
                        event.rename(
                            "json.event_attributes.UnifiedLogPredicates",
                            "jamf_compliance_reporter.log.event_attributes.unified_log_predicates",
                        )?;
                    }
                    if event.has_value("json.event_attributes.Version") {
                        event.rename(
                            "json.event_attributes.Version",
                            "jamf_compliance_reporter.log.event_attributes.version",
                        )?;
                    }
                    // End nested pipeline: "pipeline_preference_list_event"
                }
                let _cond = { event.get_str("event.action") == Some("print_event_information") };
                if _cond {
                    // Begin nested pipeline: "pipeline_print_event_information"
                    let _cond = {
                        event.has_value("json.event_attributes.job_completed_time")
                            && event.get_i64("json.event_attributes.job_completed_time") != Some(0)
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.event_attributes.job_completed_time")
                            {
                                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.job.completed_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.event_attributes.job_completed_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, template_to_string)
                                ),
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
                        event.has_value("json.event_attributes.job_creation_time")
                            && event.get_i64("json.event_attributes.job_creation_time") != Some(0)
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.event_attributes.job_creation_time")
                            {
                                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.job.creation_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.event_attributes.job_creation_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.event_attributes.job_destination") {
                        event.rename(
                            "json.event_attributes.job_destination",
                            "jamf_compliance_reporter.log.event_attributes.job.destination",
                        )?;
                    }
                    if event.has_value("json.event_attributes.job_format") {
                        event.rename(
                            "json.event_attributes.job_format",
                            "jamf_compliance_reporter.log.event_attributes.job.format",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.job_id") {
                            if let Some(val) = event.get("json.event_attributes.job_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.event_attributes.job_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.job.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = {
                        event.has_value("json.event_attributes.job_processing_time")
                            && event.get_i64("json.event_attributes.job_processing_time") != Some(0)
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.event_attributes.job_processing_time")
                            {
                                match parse_date_out(&date_str, &["UNIX"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.job.processing_time", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "json.event_attributes.job_processing_time".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.event_attributes.job_size") {
                        event.rename(
                            "json.event_attributes.job_size",
                            "jamf_compliance_reporter.log.event_attributes.job.size",
                        )?;
                    }
                    if event.has_value("json.event_attributes.job_state") {
                        event.rename(
                            "json.event_attributes.job_state",
                            "jamf_compliance_reporter.log.event_attributes.job.state",
                        )?;
                    }
                    if event.has_value("json.event_attributes.job_title") {
                        event.rename(
                            "json.event_attributes.job_title",
                            "jamf_compliance_reporter.log.event_attributes.job.title",
                        )?;
                    }
                    if event.has_value("json.event_attributes.job_user") {
                        event.rename(
                            "json.event_attributes.job_user",
                            "jamf_compliance_reporter.log.event_attributes.job.user",
                        )?;
                    }
                    let _cond = {
                        event.has_value("jamf_compliance_reporter.log.event_attributes.job.user")
                    };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get(
                                            "jamf_compliance_reporter.log.event_attributes.job.user"
                                        )
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // End nested pipeline: "pipeline_print_event_information"
                }
                let _cond = { event.get_str("event.action") == Some("prohibited_app_blocked") };
                if _cond {
                    // Begin nested pipeline: "pipeline_prohibited_app_blocked"
                    if event.has_value("json.header.action") {
                        event.rename(
                            "json.header.action",
                            "jamf_compliance_reporter.log.header.action",
                        )?;
                    }
                    if event.has_value("json.exec_args.args") {
                        event.rename("json.exec_args.args", "json.args")?;
                    }
                    if event.has_value("json.exec_args.args_compiled") {
                        event.rename(
                            "json.exec_args.args_compiled",
                            "jamf_compliance_reporter.log.exec_args.args_compiled",
                        )?;
                    }
                    if event.has_value("json.exec_env.env.PATH") {
                        event.rename(
                            "json.exec_env.env.PATH",
                            "jamf_compliance_reporter.log.exec_env.env.path",
                        )?;
                    }
                    if event.has_value("json.exec_env.env.SHELL") {
                        event.rename(
                            "json.exec_env.env.SHELL",
                            "jamf_compliance_reporter.log.exec_env.env.shell",
                        )?;
                    }
                    if event.has_value("json.exec_env.env.SSH_AUTH_SOCK") {
                        event.rename(
                            "json.exec_env.env.SSH_AUTH_SOCK",
                            "jamf_compliance_reporter.log.exec_env.env.ssh_auth_sock",
                        )?;
                    }
                    if event.has_value("json.exec_env.env.TMPDIR") {
                        event.rename(
                            "json.exec_env.env.TMPDIR",
                            "jamf_compliance_reporter.log.exec_env.env.tmpdir",
                        )?;
                    }
                    let _cond = { event.has_value("json.exec_env.env.USER") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.name",
                                json!(
                                    event
                                        .get("json.exec_env.env.USER")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.exec_env.env.USER") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("json.exec_env.env.USER")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.exec_env.env.XPC_FLAGS") {
                        event.rename(
                            "json.exec_env.env.XPC_FLAGS",
                            "jamf_compliance_reporter.log.exec_env.env.xpc.flags",
                        )?;
                    }
                    if event.has_value("json.exec_env.env.XPC_SERVICE_NAME") {
                        event.rename(
                            "json.exec_env.env.XPC_SERVICE_NAME",
                            "jamf_compliance_reporter.log.exec_env.env.xpc.service_name",
                        )?;
                    }
                    if event.has_value("json.exec_env.env_compiled") {
                        event.rename(
                            "json.exec_env.env_compiled",
                            "jamf_compliance_reporter.log.exec_env.env_compiled",
                        )?;
                    }
                    if event.has_value("json.identity.cd_hash") {
                        event.rename(
                            "json.identity.cd_hash",
                            "jamf_compliance_reporter.log.identity.cd_hash",
                        )?;
                    }
                    let _cond =
                        { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("jamf_compliance_reporter.log.identity.cd_hash")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id") {
                            if let Some(val) = event.get("json.identity.signer_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_id_truncated") {
                            if let Some(val) = event.get("json.identity.signer_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.signer_type") {
                            if let Some(val) = event.get("json.identity.signer_type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.signer_type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.signer.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id") {
                            if let Some(val) = event.get("json.identity.team_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.identity.team_id_truncated") {
                            if let Some(val) = event.get("json.identity.team_id_truncated") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.identity.team_id_truncated".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.identity.team.id_truncated",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.subject.audit_id") {
                            if let Some(val) = event.get("json.subject.audit_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.subject.audit_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("process.real_user.id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.subject.audit_user_name") {
                        event.rename("json.subject.audit_user_name", "process.real_user.name")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.subject.effective_group_id") {
                            if let Some(val) = event.get("json.subject.effective_group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.subject.effective_group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.subject.effective.group.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.subject.effective_group_name") {
                        event.rename(
                            "json.subject.effective_group_name",
                            "jamf_compliance_reporter.log.subject.effective.group.name",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.subject.effective_user_id") {
                            if let Some(val) = event.get("json.subject.effective_user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.subject.effective_user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("user.effective.id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event.get("user.effective.id").cloned() {
                            event
                                .set("jamf_compliance_reporter.log.subject.effective.user.id", v)?;
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.subject.effective_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.name",
                                json!(
                                    event
                                        .get("json.subject.effective_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.subject.effective_user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("json.subject.effective_user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.subject.effective_user_name") {
                        event.rename("json.subject.effective_user_name", "user.effective.name")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event.get("user.effective.name").cloned() {
                            event.set(
                                "jamf_compliance_reporter.log.subject.effective.user.name",
                                v,
                            )?;
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.subject.group_id") {
                            if let Some(val) = event.get("json.subject.group_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.subject.group_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("user.group.id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.subject.group_name") {
                        event.rename("json.subject.group_name", "user.group.name")?;
                    }
                    if event.has_value("json.subject.process_hash") {
                        event.rename("json.subject.process_hash", "process.hash.sha1")?;
                    }
                    let _cond = { event.has_value("process.hash.sha1") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("process.hash.sha1")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.subject.process_id") {
                            if let Some(val) = event.get("json.subject.process_id") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.subject.process_id".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.subject.process.pid",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.subject.process_information") {
                        event.rename(
                            "json.subject.process_information",
                            "jamf_compliance_reporter.log.subject.process.information",
                        )?;
                    }
                    if event.has_value("json.subject.process_name") {
                        event.rename("json.subject.process_name", "process.name")?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.subject.responsible_process_id") {
                            if let Some(val) = event.get("json.subject.responsible_process_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.subject.responsible_process_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.subject.responsible.process.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    if event.has_value("json.subject.responsible_process_name") {
                        event.rename(
                            "json.subject.responsible_process_name",
                            "jamf_compliance_reporter.log.subject.responsible.process.name",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.subject.session_id") {
                            if let Some(val) = event.get("json.subject.session_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.subject.session_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.subject.session.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.subject.terminal_id.ip_address") {
                            if let Some(val) = event.get("json.subject.terminal_id.ip_address") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.subject.terminal_id.ip_address".into(),
                                        message,
                                    }
                                })?;
                                event.set("json.subject.terminal_id.ip_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("json.subject.terminal_id.ip_address");
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "host.ip",
                                json!(
                                    event
                                        .get("json.subject.terminal_id.ip_address")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("json.subject.terminal_id.ip_address")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.subject.terminal_id.port") {
                            if let Some(val) = event.get("json.subject.terminal_id.port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.subject.terminal_id.port".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.subject.terminal_id.port",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.subject.terminal_id.type") {
                            if let Some(val) = event.get("json.subject.terminal_id.type") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.subject.terminal_id.type".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.subject.terminal_id.type",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.subject.user_id") {
                            if let Some(val) = event.get("json.subject.user_id") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.subject.user_id".into(),
                                            message,
                                        }
                                    })?;
                                event.set("user.id", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.subject.user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "user.name",
                                json!(
                                    event
                                        .get("json.subject.user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    let _cond = { event.has_value("json.subject.user_name") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("json.subject.user_name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                    }
                    if event.has_value("json.texts") {
                        event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                    }
                    // Painless script
                    // Source: def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n"#
                        ),
                    )?;
                    // End nested pipeline: "pipeline_prohibited_app_blocked"
                }
                let _cond = { event.get_str("event.action") == Some("signal_event") };
                if _cond {
                    // Begin nested pipeline: "pipeline_signal_event"
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.signal_event_info.signal") {
                            if let Some(val) = event.get("json.signal_event_info.signal") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.signal_event_info.signal".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.signal_event_info.signal",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    // End nested pipeline: "pipeline_signal_event"
                }
                let _cond = { event.get_str("event.action") == Some("unified_log_event") };
                if _cond {
                    // Begin nested pipeline: "pipeline_unified_log_event"
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.activityIdentifier") {
                            if let Some(val) = event.get("json.event_attributes.activityIdentifier")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.event_attributes.activityIdentifier".into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.event_attributes.activity_identifier", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = {
                        event
                            .get("json.event_attributes.backtrace.frames")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes.backtrace.frames", |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(val) = event.get("_ingest._value.imageOffset") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.imageOffset".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.image_offset", converted)?;
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.remove("_ingest._value.imageOffset");
                                event.append(
                                    "error.message",
                                    json!(
                                        event
                                            .get("_ingest.on_failure_message")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes.backtrace.frames")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject =
                                event.get("json.event_attributes.backtrace.frames").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if event.remove("_ingest._value.imageOffset").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value.imageOffset".into(),
                                            });
                                        }
                                        Ok(())
                                    })();
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "json.event_attributes.backtrace.frames",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes.backtrace.frames")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes.backtrace.frames", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.rename(
                                    "_ingest._value.imageUUID",
                                    "_ingest._value.image_uuid",
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    if event.has_value("json.event_attributes.backtrace.frames") {
                        event.rename(
                            "json.event_attributes.backtrace.frames",
                            "jamf_compliance_reporter.log.event_attributes.backtrace.frames",
                        )?;
                    }
                    if event.has_value("json.event_attributes.category") {
                        event.rename(
                            "json.event_attributes.category",
                            "jamf_compliance_reporter.log.event_attributes.category",
                        )?;
                    }
                    if event.has_value("json.event_attributes.eventMessage") {
                        event.rename(
                            "json.event_attributes.eventMessage",
                            "jamf_compliance_reporter.log.event_attributes.event.message",
                        )?;
                    }
                    if event.has_value("json.event_attributes.eventType") {
                        event.rename(
                            "json.event_attributes.eventType",
                            "jamf_compliance_reporter.log.event_attributes.event.type",
                        )?;
                    }
                    if event.has_value("json.event_attributes.formatString") {
                        event.rename(
                            "json.event_attributes.formatString",
                            "jamf_compliance_reporter.log.event_attributes.format_string",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.machTimestamp") {
                            if let Some(val) = event.get("json.event_attributes.machTimestamp") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.event_attributes.machTimestamp".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.mach_timestamp",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = { event.has_value("json.event_attributes.messageType") };
                    if _cond {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique("event.type", json!("info"))?;
                            Ok(())
                        })();
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.parentActivityIdentifier") {
                            if let Some(val) =
                                event.get("json.event_attributes.parentActivityIdentifier")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.event_attributes.parentActivityIdentifier"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event.set("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.processID") {
                            if let Some(val) = event.get("json.event_attributes.processID") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.event_attributes.processID".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.process.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.event_attributes.processImagePath") {
                        event.rename(
                            "json.event_attributes.processImagePath",
                            "jamf_compliance_reporter.log.event_attributes.process.image.path",
                        )?;
                    }
                    if event.has_value("json.event_attributes.processImageUUID") {
                        event.rename(
                            "json.event_attributes.processImageUUID",
                            "jamf_compliance_reporter.log.event_attributes.process.image.uuid",
                        )?;
                    }
                    if event.has_value("json.event_attributes.senderImagePath") {
                        event.rename(
                            "json.event_attributes.senderImagePath",
                            "jamf_compliance_reporter.log.event_attributes.sender.image.path",
                        )?;
                    }
                    if event.has_value("json.event_attributes.senderImageUUID") {
                        event.rename(
                            "json.event_attributes.senderImageUUID",
                            "jamf_compliance_reporter.log.event_attributes.sender.image.uuid",
                        )?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.senderProgramCounter") {
                            if let Some(val) =
                                event.get("json.event_attributes.senderProgramCounter")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.event_attributes.senderProgramCounter".into(),
                                        message,
                                    }
                                })?;
                                event.set("jamf_compliance_reporter.log.event_attributes.sender.program_counter", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    if event.has_value("json.event_attributes.source") {
                        event.rename(
                            "json.event_attributes.source",
                            "jamf_compliance_reporter.log.event_attributes.source",
                        )?;
                    }
                    if event.has_value("json.event_attributes.subsystem") {
                        event.rename(
                            "json.event_attributes.subsystem",
                            "jamf_compliance_reporter.log.event_attributes.subsystem",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.threadID") {
                            if let Some(val) = event.get("json.event_attributes.threadID") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.event_attributes.threadID".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.thread_id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = {
                        event.has_value("json.event_attributes.timestamp")
                            && event.get_i64("json.event_attributes.timestamp") != Some(0)
                    };
                    if _cond {
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("json.event_attributes.timestamp")
                            {
                                match parse_date_out(
                                    &date_str,
                                    &["yyyy-MM-dd HH:mm:ss.SSSSSSZ"],
                                    None,
                                    None,
                                ) {
                                    Some(parsed) => event.set(
                                        "jamf_compliance_reporter.log.event_attributes.timestamp",
                                        parsed,
                                    )?,
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "json.event_attributes.timestamp".into(),
                                            message: format!("unable to parse date [{date_str}]"),
                                        });
                                    }
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.append(
                                "error.message",
                                json!(
                                    event
                                        .get("_ingest.on_failure_message")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                    }
                    if event.has_value("json.event_attributes.timezoneName") {
                        event.rename(
                            "json.event_attributes.timezoneName",
                            "jamf_compliance_reporter.log.event_attributes.timezone_name",
                        )?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.event_attributes.traceID") {
                            if let Some(val) = event.get("json.event_attributes.traceID") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.event_attributes.traceID".into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.trace_id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_unified_log_event"
                }
                let _cond =
                    { event.get_str("event.action") == Some("xprotect_definitions_version_info") };
                if _cond {
                    // Begin nested pipeline: "pipeline_xprotect_definitions_version_info"
                    if event.has_value("json.event_attributes.BuildAliasOf") {
                        event.rename(
                            "json.event_attributes.BuildAliasOf",
                            "jamf_compliance_reporter.log.event_attributes.build_alias_of",
                        )?;
                    }
                    if event.has_value("json.event_attributes.BuildVersion") {
                        event.rename(
                            "json.event_attributes.BuildVersion",
                            "jamf_compliance_reporter.log.event_attributes.build_version",
                        )?;
                    }
                    if event.has_value("json.event_attributes.CFBundleShortVersionString") {
                        event.rename("json.event_attributes.CFBundleShortVersionString", "jamf_compliance_reporter.log.event_attributes.cf_bundle_short_version_string")?;
                    }
                    if event.has_value("json.event_attributes.CFBundleVersion") {
                        event.rename(
                            "json.event_attributes.CFBundleVersion",
                            "jamf_compliance_reporter.log.event_attributes.cf_bundle_version",
                        )?;
                    }
                    if event.has_value("json.event_attributes.ProjectName") {
                        event.rename(
                            "json.event_attributes.ProjectName",
                            "jamf_compliance_reporter.log.event_attributes.project_name",
                        )?;
                    }
                    if event.has_value("json.event_attributes.SourceVersion") {
                        event.rename(
                            "json.event_attributes.SourceVersion",
                            "jamf_compliance_reporter.log.event_attributes.source_version",
                        )?;
                    }
                    // End nested pipeline: "pipeline_xprotect_definitions_version_info"
                }
                let _cond = { event.get_str("event.action") == Some("xprotect_event_log") };
                if _cond {
                    // Begin nested pipeline: "pipeline_xprotect_event_log"
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique("jamf_compliance_reporter.log.event_attributes.activity_identifier", json!(event.get("_ingest._value.activityIdentifier").map_or_else(String::new, template_to_string)))?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value(
                            "jamf_compliance_reporter.log.event_attributes.activity_identifier",
                        ) {
                            if let Some(val) = event.get(
                                "jamf_compliance_reporter.log.event_attributes.activity_identifier",
                            ) {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.activity_identifier".into(),
                message,
                }
                                    })?;
                                event.set("jamf_compliance_reporter.log.event_attributes.activity_identifier", converted)?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            foreach_array(event, "_ingest._value.backtrace.frames", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset", json!(event.get("_ingest._value.imageOffset").map_or_else(String::new, template_to_string)))?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_offset", converted)?;
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            foreach_array(event, "_ingest._value.backtrace.frames", |event| {
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    event.append_unique("jamf_compliance_reporter.log.event_attributes.backtrace.frames.image_uuid", json!(event.get("_ingest._value.imageUUID").map_or_else(String::new, template_to_string)))?;
                                    Ok(())
                                })();
                                Ok(())
                            })?;
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.category",
                                    json!(
                                        event
                                            .get("_ingest._value.category")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.event.message",
                                    json!(
                                        event
                                            .get("_ingest._value.eventMessage")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.event.type",
                                    json!(
                                        event
                                            .get("_ingest._value.eventType")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.format_string",
                                    json!(
                                        event
                                            .get("_ingest._value.formatString")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(val) = event.get("_ingest._value.machTimestamp") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.machTimestamp".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("jamf_compliance_reporter.log.event_attributes.mach_timestamp", converted)?;
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.remove("_ingest._value.machTimestamp");
                                event.append(
                                    "error.message",
                                    json!(
                                        event
                                            .get("_ingest.on_failure_message")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique("event.type", json!("info"))?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier", json!(event.get("_ingest._value.parentActivityIdentifier").map_or_else(String::new, template_to_string)))?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier") {
                if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.parent_activity_identifier".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.parent_activity_identifier", converted)?;
                }
                }
                        Ok(())
                    })();
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.process.id",
                                    json!(
                                        event
                                            .get("_ingest._value.processID")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event
                            .has_value("jamf_compliance_reporter.log.event_attributes.process.id")
                        {
                            if let Some(val) = event
                                .get("jamf_compliance_reporter.log.event_attributes.process.id")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.process.id".into(),
                message,
                }
                                })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.process.id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique("jamf_compliance_reporter.log.event_attributes.process.image.path", json!(event.get("_ingest._value.processImagePath").map_or_else(String::new, template_to_string)))?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique("jamf_compliance_reporter.log.event_attributes.process.image.uuid", json!(event.get("_ingest._value.processImageUUID").map_or_else(String::new, template_to_string)))?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique("jamf_compliance_reporter.log.event_attributes.sender.image.path", json!(event.get("_ingest._value.senderImagePath").map_or_else(String::new, template_to_string)))?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique("jamf_compliance_reporter.log.event_attributes.sender.image.uuid", json!(event.get("_ingest._value.senderImageUUID").map_or_else(String::new, template_to_string)))?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique("jamf_compliance_reporter.log.event_attributes.sender.program_counter", json!(event.get("_ingest._value.senderProgramCounter").map_or_else(String::new, template_to_string)))?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value(
                            "jamf_compliance_reporter.log.event_attributes.sender.program_counter",
                        ) {
                            if let Some(val) = event.get("jamf_compliance_reporter.log.event_attributes.sender.program_counter") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.sender.program_counter".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.event_attributes.sender.program_counter", converted)?;
                }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.source",
                                    json!(
                                        event
                                            .get("_ingest._value.source")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.subsystem",
                                    json!(
                                        event
                                            .get("_ingest._value.subsystem")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.thread_id",
                                    json!(
                                        event
                                            .get("_ingest._value.threadID")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event
                            .has_value("jamf_compliance_reporter.log.event_attributes.thread_id")
                        {
                            if let Some(val) =
                                event.get("jamf_compliance_reporter.log.event_attributes.thread_id")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.thread_id".into(),
                message,
                }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.thread_id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("json.event_attributes").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                                Some(Value::Array(items)) => {
                                    items.into_iter().map(|v| (None, v)).collect()
                                }
                                Some(Value::Object(fields)) => {
                                    fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                                }
                                _ => Vec::new(),
                            };
                            if !entries.is_empty() {
                                // A NESTED loop borrows the same slots, so the enclosing
                                // entry is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let enclosing_key = event.get("_ingest._key").cloned();
                                let mut list = Vec::with_capacity(entries.len());
                                let mut fields = Map::new();
                                for (key, item) in entries {
                                    if let Some(key) = key.as_deref() {
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(date_str) =
                                            event.get_as_string("_ingest._value.timestamp")
                                        {
                                            match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSSSSSZ"], None, None) {
                Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.timestamp", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "_ingest._value.timestamp".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event.set("_ingest.on_failure_processor_type", "date")?;
                                        event.remove("_ingest._value.timestamp");
                                        event.append(
                                            "error.message",
                                            json!(
                                                event
                                                    .get("_ingest.on_failure_message")
                                                    .map_or_else(String::new, template_to_string)
                                            ),
                                        )?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    let left = event.remove("_ingest._value");
                                    match key {
                                        // An entry the body renamed AWAY is gone from the
                                        // object, which is how a foreach lifts fields up.
                                        Some(key) => {
                                            if let Some(value) = left {
                                                fields.insert(key, value);
                                            }
                                        }
                                        None => list.push(left.unwrap_or(Value::Null)),
                                    }
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                if let Some(previous) = enclosing_key {
                                    event.set("_ingest._key", previous)?;
                                }
                                event.set(
                                    "json.event_attributes",
                                    if keyed {
                                        Value::Object(fields)
                                    } else {
                                        Value::Array(list)
                                    },
                                )?;
                            }
                        }
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.timezone_name",
                                    json!(
                                        event
                                            .get("_ingest._value.timezone_name")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.timezoneName",
                                    json!(
                                        event
                                            .get("_ingest._value.timezoneName")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    let _cond = {
                        event
                            .get("json.event_attributes")
                            .is_some_and(|v| v.is_array())
                    };
                    if _cond {
                        foreach_array(event, "json.event_attributes", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                event.append_unique(
                                    "jamf_compliance_reporter.log.event_attributes.trace_id",
                                    json!(
                                        event
                                            .get("_ingest._value.traceID")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("jamf_compliance_reporter.log.event_attributes.trace_id")
                        {
                            if let Some(val) =
                                event.get("jamf_compliance_reporter.log.event_attributes.trace_id")
                            {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                path: "jamf_compliance_reporter.log.event_attributes.trace_id".into(),
                message,
                }
                                    })?;
                                event.set(
                                    "jamf_compliance_reporter.log.event_attributes.trace_id",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                    // End nested pipeline: "pipeline_xprotect_event_log"
                }
                // End nested pipeline: "pipeline_event"
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
