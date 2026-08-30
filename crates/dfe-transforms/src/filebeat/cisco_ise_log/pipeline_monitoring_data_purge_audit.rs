// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_monitoring_data_purge_audit` pipeline.
pub struct PipelineMonitoringDataPurgeAudit;

impl Transform for PipelineMonitoringDataPurgeAudit {
    fn name(&self) -> &str {
        "pipeline_monitoring_data_purge_audit"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (?P<_tmp_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[ ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?)) %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},
                    let _ = cached_grok_mapped!("(?P<_tmp_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[ ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?)) %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},", [("_tmp_timestamp", "_tmp.timestamp")]).extract_into(&input, event)?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "cisco_ise.log.log_details_raw", "cisco_ise.log.log_details_raw", |s| s.trim().to_string())?;
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.message.description") && event.get_str("cisco_ise.log.message.description") != Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                    // Grok pattern: ^%{DATA:event.action}:
                    let _ = cached_grok!("^%{DATA:event.action}:").extract_into(&input, event)?;
                }
                Ok(())
            })();
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

                gsub_field(event, "cisco_ise.log.log_details_raw", "cisco_ise.log.log_details_raw", cached_regex!("\\\\,"), "")?;

                // Painless script
                // Source: def m = ctx.cisco_ise.log[\"log_details\"];\nif (!(m instanceof Map)) {\n  m = new HashMap();\n}\nint pos = ctx.cisco_ise.log.log_details_raw.indexOf(\"=\");\nif (pos == -1) {\n    m[ctx.cisco_ise.log.log_details_raw] = null;\n} else {\n    m[ctx.cisco_ise.log.log_details_raw.substring(0,pos)] = ctx.cisco_ise.log.log_details_raw.substring(pos+1);\n}\nctx.cisco_ise.log[\"log_details\"] = m;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def m = ctx.cisco_ise.log[\"log_details\"];\nif (!(m instanceof Map)) {\n  m = new HashMap();\n}\nint pos = ctx.cisco_ise.log.log_details_raw.indexOf(\"=\");\nif (pos == -1) {\n    m[ctx.cisco_ise.log.log_details_raw] = null;\n} else {\n    m[ctx.cisco_ise.log.log_details_raw.substring(0,pos)] = ctx.cisco_ise.log.log_details_raw.substring(pos+1);\n}\nctx.cisco_ise.log[\"log_details\"] = m;"#))?;

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSSSSS", "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date__tmp_timestamp_9ef85c6a")?;
                        event.remove("_tmp.timestamp");
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("event.timezone") && event.get_str("event.timezone") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSSSSS", "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date__tmp_timestamp_1d2a12b9")?;
                        event.remove("_tmp.timestamp");
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
