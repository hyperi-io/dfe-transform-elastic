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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^<%{NONNEGINT:log.syslog.priority:long}>%{NONNEGINT} %{TIMESTAMP_ISO8601:_tmp.syslog_ts} %{SYSLOGHOST:_tmp.hostname} (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))
                    // Grok pattern: ^%{SYSLOGTIMESTAMP:_tmp.syslog_ts} %{SYSLOGHOST:_tmp.hostname} (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))
                    // Grok pattern: (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))
                    if !extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^<%{NONNEGINT:log.syslog.priority:long}>%{NONNEGINT} %{TIMESTAMP_ISO8601:_tmp.syslog_ts} %{SYSLOGHOST:_tmp.hostname} (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))",
                                [("_tmp_payload", "_tmp.payload")]
                            ),
                            cached_grok_mapped!(
                                "^%{SYSLOGTIMESTAMP:_tmp.syslog_ts} %{SYSLOGHOST:_tmp.hostname} (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))",
                                [("_tmp_payload", "_tmp.payload")]
                            ),
                            cached_grok_mapped!(
                                "(?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))",
                                [("_tmp_payload", "_tmp.payload")]
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_event_original")?;
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: (format!(
                        "unexpected event format: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ))
                    .to_string(),
                });
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "_tmp.payload", "_tmp.json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_tmp_payload")?;
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: (format!(
                        "malformed JSON event: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ))
                    .to_string(),
                });
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                event.rename("_tmp.json.syslog.monitor_record", "cyberarkpas.monitor")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "rename")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "rename_tmp_json_syslog_monitor_record",
                )?;
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: (format!(
                        "unexpected event structure: {}",
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ))
                    .to_string(),
                });
            }

            // Painless script
            // Source: ctx.cyberarkpas.monitor.entrySet().removeIf(entry -> entry.getValue() == \"\");
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx.cyberarkpas.monitor.entrySet().removeIf(entry -> entry.getValue() == \"\");"#
                ),
            )?;

            if event.has_value("_tmp.json.raw") {
                event.rename("_tmp.json.raw", "cyberarkpas.monitor.raw")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("cyberarkpas.monitor.IsoTimestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cyberarkpas.monitor.IsoTimestamp".into(),
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
                    "date_cyberarkpas_monitor_isotimestamp",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "failed to parse ISO timestamp field: {}: {}",
                        event
                            .get("cyberarkpas.monitor.IsoTimestamp")
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

            // Painless script
            // Source: String to_snake_case(String s) {\n  /* faster code path for strings that won't need an underscore */\n  if (s.chars().skip(1).noneMatch(Character::isUpperCase)) {\n    return s.toLowerCase();\n  }\n  int run = 0;\n  boolean first = true;\n  StringBuilder result = new StringBuilder();\n  for (char c : s.toCharArray()) {\n    char o = Character.toLowerCase(c);\n    if (c != o) {\n      if (run == 0 && !first) {\n        result.append('_');\n      }\n      run ++;\n    } else {\n      if (run > 1) {\n        char prev = result.charAt(result.length()-1);\n        result.setCharAt(result.length()-1, (char)'_');\n        result.append(prev);\n      }\n      run = 0;\n      first = false;\n    }\n    result.append(o);\n  }\n  return result.toString();\n} def keys_to_snake_case_recursive(Map object) {\n  return object.entrySet().stream().collect(\n    Collectors.toMap(\n      e -> to_snake_case(e.getKey()),\n      e -> e.getValue() instanceof Map ? keys_to_snake_case_recursive(e.getValue()) : e.getValue()\n    )\n  );\n} ctx.cyberarkpas.monitor = keys_to_snake_case_recursive(ctx.cyberarkpas.monitor);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"String to_snake_case(String s) {\n  /* faster code path for strings that won't need an underscore */\n  if (s.chars().skip(1).noneMatch(Character::isUpperCase)) {\n    return s.toLowerCase();\n  }\n  int run = 0;\n  boolean first = true;\n  StringBuilder result = new StringBuilder();\n  for (char c : s.toCharArray()) {\n    char o = Character.toLowerCase(c);\n    if (c != o) {\n      if (run == 0 && !first) {\n        result.append('_');\n      }\n      run ++;\n    } else {\n      if (run > 1) {\n        char prev = result.charAt(result.length()-1);\n        result.setCharAt(result.length()-1, (char)'_');\n        result.append(prev);\n      }\n      run = 0;\n      first = false;\n    }\n    result.append(o);\n  }\n  return result.toString();\n} def keys_to_snake_case_recursive(Map object) {\n  return object.entrySet().stream().collect(\n    Collectors.toMap(\n      e -> to_snake_case(e.getKey()),\n      e -> e.getValue() instanceof Map ? keys_to_snake_case_recursive(e.getValue()) : e.getValue()\n    )\n  );\n} ctx.cyberarkpas.monitor = keys_to_snake_case_recursive(ctx.cyberarkpas.monitor);\n"#
                ),
            )?;

            if event.has_value("cyberarkpas.monitor.average_execution_time") {
                if let Some(val) = event.get("cyberarkpas.monitor.average_execution_time") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.average_execution_time".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.average_execution_time", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.max_execution_time") {
                if let Some(val) = event.get("cyberarkpas.monitor.max_execution_time") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.max_execution_time".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.max_execution_time", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.average_queue_time") {
                if let Some(val) = event.get("cyberarkpas.monitor.average_queue_time") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.average_queue_time".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.average_queue_time", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.max_queue_time") {
                if let Some(val) = event.get("cyberarkpas.monitor.max_queue_time") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.max_queue_time".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.max_queue_time", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.number_of_parallel_tasks") {
                if let Some(val) = event.get("cyberarkpas.monitor.number_of_parallel_tasks") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.number_of_parallel_tasks".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.number_of_parallel_tasks", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.max_parallel_tasks") {
                if let Some(val) = event.get("cyberarkpas.monitor.max_parallel_tasks") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.max_parallel_tasks".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.max_parallel_tasks", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.transaction_count") {
                if let Some(val) = event.get("cyberarkpas.monitor.transaction_count") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.transaction_count".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.transaction_count", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.cpu_usage") {
                if let Some(val) = event.get("cyberarkpas.monitor.cpu_usage") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.cpu_usage".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.cpu_usage", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.memory_usage") {
                if let Some(val) = event.get("cyberarkpas.monitor.memory_usage") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.memory_usage".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.memory_usage", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.drive_free_space_in_gb") {
                if let Some(val) = event.get("cyberarkpas.monitor.drive_free_space_in_gb") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.drive_free_space_in_gb".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.drive_free_space_in_gb", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.drive_total_space_in_gb") {
                if let Some(val) = event.get("cyberarkpas.monitor.drive_total_space_in_gb") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.drive_total_space_in_gb".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.drive_total_space_in_gb", converted)?;
                }
            }

            if event.has_value("cyberarkpas.monitor.syslog_queue_size") {
                if let Some(val) = event.get("cyberarkpas.monitor.syslog_queue_size") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cyberarkpas.monitor.syslog_queue_size".into(),
                            message,
                        }
                    })?;
                    event.set("cyberarkpas.monitor.syslog_queue_size", converted)?;
                }
            }

            event.set("event.kind", json!("metric"))?;

            let _cond = { event.get("tags").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def i = ctx.tags.indexOf(\"cyberarkpas-audit\"); if (i != -1) ctx.tags.set(i, \"cyberarkpas-monitor\");
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def i = ctx.tags.indexOf(\"cyberarkpas-audit\"); if (i != -1) ctx.tags.set(i, \"cyberarkpas-monitor\");"#
                    ),
                )?;
            }

            if event.has_value("cyberarkpas.monitor.vendor") {
                event.rename("cyberarkpas.monitor.vendor", "observer.vendor")?;
            }

            if event.has_value("cyberarkpas.monitor.product") {
                event.rename("cyberarkpas.monitor.product", "observer.product")?;
            }

            if let Some(v) = event
                .get("cyberarkpas.monitor.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            if event.has_value("cyberarkpas.monitor.hostname") {
                event.rename("cyberarkpas.monitor.hostname", "observer.hostname")?;
            }

            let _cond = { event.has_value("observer.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("observer.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("host.cpu.usage") };
            if _cond {
                // Painless script
                // Source: if (ctx.host == null) ctx.host = [:]; if (ctx.host.cpu == null) ctx.host.cpu = [:]; ctx.host.cpu.usage = ctx.cyberarkpas.monitor.cpu_usage/100.0;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.host == null) ctx.host = [:]; if (ctx.host.cpu == null) ctx.host.cpu = [:]; ctx.host.cpu.usage = ctx.cyberarkpas.monitor.cpu_usage/100.0;"#
                    ),
                )?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                let v = json!(
                    event
                        .get("observer.hostname")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("host.name", v)?;
                }
            }

            event.remove("_tmp");

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
                event.remove("_tmp");
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
