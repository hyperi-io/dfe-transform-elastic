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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(input) = event.get_string("message") {
                // Grok pattern: (?:(?:%{TIMESTAMP_ISO8601:timestamp}: %{BASE10NUM:elasticsearch.gc.jvm_runtime_sec}:)|(?:\\[%{TIMESTAMP_ISO8601:timestamp}\\]\\[%{POSINT:process.pid}\\](\\[%{DATA:log.level}%{SPACE}\\])?\\[%{DATA:elasticsearch.gc.tags}%{SPACE}\\])) Total time for which application threads were stopped: %{BASE10NUM:elasticsearch.gc.threads_total_stop_time_sec} seconds, Stopping threads took: %{BASE10NUM:elasticsearch.gc.stopping_threads_time_sec} seconds
                // Grok pattern: (?:(?:%{TIMESTAMP_ISO8601:timestamp}: %{BASE10NUM:elasticsearch.gc.jvm_runtime_sec}:)) \\[GC \\(%{DATA:elasticsearch.gc.phase.name}\\) \\[YG occupancy: %{BASE10NUM:elasticsearch.gc.young_gen.used_kb} K \\(%{BASE10NUM:elasticsearch.gc.young_gen.size_kb} K\\)\\]%{BASE10NUM}: \\[Rescan \\(parallel\\) , %{BASE10NUM:elasticsearch.gc.phase.parallel_rescan_time_sec} secs\\]%{BASE10NUM}: \\[weak refs processing, %{BASE10NUM:elasticsearch.gc.phase.weak_refs_processing_time_sec} secs\\]%{BASE10NUM}: \\[class unloading, %{BASE10NUM:elasticsearch.gc.phase.class_unload_time_sec} secs\\]%{BASE10NUM}: \\[scrub symbol table, %{BASE10NUM:elasticsearch.gc.phase.scrub_symbol_table_time_sec} secs\\]%{BASE10NUM}: \\[scrub string table, %{BASE10NUM:elasticsearch.gc.phase.scrub_string_table_time_sec} secs\\]\\[1 CMS-remark: %{BASE10NUM:elasticsearch.gc.old_gen.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.old_gen.size_kb}K\\)\\] %{BASE10NUM:elasticsearch.gc.heap.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.heap.size_kb}K\\), %{BASE10NUM:elasticsearch.gc.phase.duration_sec} secs\\] (?:\\[Times: user=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.user_sec} sys=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.sys_sec}, real=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.real_sec} secs\\])
                // Grok pattern: (?:(?:%{TIMESTAMP_ISO8601:timestamp}: %{BASE10NUM:elasticsearch.gc.jvm_runtime_sec}:)) \\[GC \\(%{DATA:elasticsearch.gc.phase.name}\\) \\[%{BASE10NUM} CMS-initial-mark: %{BASE10NUM:elasticsearch.gc.old_gen.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.old_gen.size_kb}K\\)\\] %{BASE10NUM:elasticsearch.gc.heap.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.heap.size_kb}K\\), %{BASE10NUM:elasticsearch.gc.phase.duration_sec} secs\\] (?:\\[Times: user=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.user_sec} sys=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.sys_sec}, real=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.real_sec} secs\\])
                // Grok pattern: (?:\\[%{TIMESTAMP_ISO8601:timestamp}\\]\\[%{POSINT:process.pid}\\](\\[%{DATA:log.level}%{SPACE}\\])?\\[%{DATA:elasticsearch.gc.tags}%{SPACE}\\]) GC\\(%{BASE10NUM}\\) ParNew: %{BASE10NUM}K-\\>%{BASE10NUM:elasticsearch.gc.young_gen.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.young_gen.size_kb}K\\)
                // Grok pattern: (?:\\[%{TIMESTAMP_ISO8601:timestamp}\\]\\[%{POSINT:process.pid}\\](\\[%{DATA:log.level}%{SPACE}\\])?\\[%{DATA:elasticsearch.gc.tags}%{SPACE}\\]) GC\\(%{BASE10NUM}\\) Old: %{BASE10NUM}K-\\>%{BASE10NUM:elasticsearch.gc.old_gen.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.old_gen.size_kb}K\\)
                // Grok pattern: (?:(?:%{TIMESTAMP_ISO8601:timestamp}: %{BASE10NUM:elasticsearch.gc.jvm_runtime_sec}:)|(?:\\[%{TIMESTAMP_ISO8601:timestamp}\\]\\[%{POSINT:process.pid}\\](\\[%{DATA:log.level}%{SPACE}\\])?\\[%{DATA:elasticsearch.gc.tags}%{SPACE}\\])) (?P<message>(?:(.|\n)*))
                let _ = extract_first_match(
                    &[
                        cached_grok!(
                            "(?:(?:%{TIMESTAMP_ISO8601:timestamp}: %{BASE10NUM:elasticsearch.gc.jvm_runtime_sec}:)|(?:\\[%{TIMESTAMP_ISO8601:timestamp}\\]\\[%{POSINT:process.pid}\\](\\[%{DATA:log.level}%{SPACE}\\])?\\[%{DATA:elasticsearch.gc.tags}%{SPACE}\\])) Total time for which application threads were stopped: %{BASE10NUM:elasticsearch.gc.threads_total_stop_time_sec} seconds, Stopping threads took: %{BASE10NUM:elasticsearch.gc.stopping_threads_time_sec} seconds"
                        ),
                        cached_grok!(
                            "(?:(?:%{TIMESTAMP_ISO8601:timestamp}: %{BASE10NUM:elasticsearch.gc.jvm_runtime_sec}:)) \\[GC \\(%{DATA:elasticsearch.gc.phase.name}\\) \\[YG occupancy: %{BASE10NUM:elasticsearch.gc.young_gen.used_kb} K \\(%{BASE10NUM:elasticsearch.gc.young_gen.size_kb} K\\)\\]%{BASE10NUM}: \\[Rescan \\(parallel\\) , %{BASE10NUM:elasticsearch.gc.phase.parallel_rescan_time_sec} secs\\]%{BASE10NUM}: \\[weak refs processing, %{BASE10NUM:elasticsearch.gc.phase.weak_refs_processing_time_sec} secs\\]%{BASE10NUM}: \\[class unloading, %{BASE10NUM:elasticsearch.gc.phase.class_unload_time_sec} secs\\]%{BASE10NUM}: \\[scrub symbol table, %{BASE10NUM:elasticsearch.gc.phase.scrub_symbol_table_time_sec} secs\\]%{BASE10NUM}: \\[scrub string table, %{BASE10NUM:elasticsearch.gc.phase.scrub_string_table_time_sec} secs\\]\\[1 CMS-remark: %{BASE10NUM:elasticsearch.gc.old_gen.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.old_gen.size_kb}K\\)\\] %{BASE10NUM:elasticsearch.gc.heap.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.heap.size_kb}K\\), %{BASE10NUM:elasticsearch.gc.phase.duration_sec} secs\\] (?:\\[Times: user=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.user_sec} sys=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.sys_sec}, real=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.real_sec} secs\\])"
                        ),
                        cached_grok!(
                            "(?:(?:%{TIMESTAMP_ISO8601:timestamp}: %{BASE10NUM:elasticsearch.gc.jvm_runtime_sec}:)) \\[GC \\(%{DATA:elasticsearch.gc.phase.name}\\) \\[%{BASE10NUM} CMS-initial-mark: %{BASE10NUM:elasticsearch.gc.old_gen.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.old_gen.size_kb}K\\)\\] %{BASE10NUM:elasticsearch.gc.heap.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.heap.size_kb}K\\), %{BASE10NUM:elasticsearch.gc.phase.duration_sec} secs\\] (?:\\[Times: user=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.user_sec} sys=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.sys_sec}, real=%{BASE10NUM:elasticsearch.gc.phase.cpu_time.real_sec} secs\\])"
                        ),
                        cached_grok!(
                            "(?:\\[%{TIMESTAMP_ISO8601:timestamp}\\]\\[%{POSINT:process.pid}\\](\\[%{DATA:log.level}%{SPACE}\\])?\\[%{DATA:elasticsearch.gc.tags}%{SPACE}\\]) GC\\(%{BASE10NUM}\\) ParNew: %{BASE10NUM}K-\\>%{BASE10NUM:elasticsearch.gc.young_gen.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.young_gen.size_kb}K\\)"
                        ),
                        cached_grok!(
                            "(?:\\[%{TIMESTAMP_ISO8601:timestamp}\\]\\[%{POSINT:process.pid}\\](\\[%{DATA:log.level}%{SPACE}\\])?\\[%{DATA:elasticsearch.gc.tags}%{SPACE}\\]) GC\\(%{BASE10NUM}\\) Old: %{BASE10NUM}K-\\>%{BASE10NUM:elasticsearch.gc.old_gen.used_kb}K\\(%{BASE10NUM:elasticsearch.gc.old_gen.size_kb}K\\)"
                        ),
                        cached_grok!(
                            "(?:(?:%{TIMESTAMP_ISO8601:timestamp}: %{BASE10NUM:elasticsearch.gc.jvm_runtime_sec}:)|(?:\\[%{TIMESTAMP_ISO8601:timestamp}\\]\\[%{POSINT:process.pid}\\](\\[%{DATA:log.level}%{SPACE}\\])?\\[%{DATA:elasticsearch.gc.tags}%{SPACE}\\])) (?P<message>(?:(.|\n)*))"
                        ),
                    ],
                    &input,
                    event,
                )?;
            }

            if event.has_value("process.pid") {
                if let Some(val) = event.get("process.pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.pid".into(),
                            message,
                        }
                    })?;
                    event.set("process.pid", converted)?;
                }
            }

            if let Some(date_str) = event.get_as_string("timestamp") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            if event.remove("timestamp").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "timestamp".into(),
                });
            }

            event.set("event.kind", json!("metric"))?;

            event.set("event.category", Value::Array(vec![json!("database")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            if event.has_value("elasticsearch.gc.tags") {
                if let Some(s) = event.get_string("elasticsearch.gc.tags") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("elasticsearch.gc.tags", Value::Array(parts))?;
                }
            }

            event.set("service.type", json!("elasticsearch"))?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
