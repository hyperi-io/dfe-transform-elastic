// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_app_metrics` pipeline.
pub struct PipelineAppMetrics;

impl Transform for PipelineAppMetrics {
    fn name(&self) -> &str {
        "pipeline_app_metrics"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append("event.type", json!("info"))?;

            event.set("event.kind", json!("event"))?;

            event.set("jamf_compliance_reporter.log.dataset", json!("app_metrics"))?;

            event.set("host.os.type", json!("macos"))?;

                event.append("event.category", json!("process"))?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json._event_score") {
                if let Some(val) = event.get("json._event_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json._event_score".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.app_metric_info.cpu_percentage".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.app_metric_info.cpu_percentage", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.app_metric_info.cpu_time_seconds".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.app_metric_info.cpu_time_seconds", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.app_metric_info.interrupt_wakeups".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.app_metric_info.interrupt_wakeups", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.app_metric_info.platform_idle_wakeups".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.app_metric_info.platform_idle_wakeups", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                if let Some(val) = event.get("json.app_metric_info.resident_memory_size_mb") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.app_metric_info.resident_memory_size_mb".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.app_metric_info.resident_memory_size.mb", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                if let Some(val) = event.get("json.app_metric_info.virtual_memory_size_mb") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.app_metric_info.virtual_memory_size_mb".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.app_metric_info.virtual_memory_size.mb", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                if let Some(date_str) = event.get_as_string("json.header.time_seconds_epoch") {
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
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                event.append_unique("related.hosts", json!(event.get("host.hostname").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                if event.has_value("json.host_info.host_uuid") {
                    event.rename("json.host_info.host_uuid", "jamf_compliance_reporter.log.host_info.host.uuid")?;
                }

                if event.has_value("json.host_info.osversion") {
                    event.rename("json.host_info.osversion", "host.os.version")?;
                }

            let _cond = { event.has_value("json.host_info.primary_mac_address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("host.mac", json!(event.get("json.host_info.primary_mac_address").map_or_else(String::new, template_to_string)))?;
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
                painless_exec_plan(event, cached_painless!(r#"ctx.host.cpu = new HashMap();\nctx.host.cpu.usage = Math.round(ctx.json.app_metric_info.cpu_percentage * 10) / 1000.0;\n"#))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
