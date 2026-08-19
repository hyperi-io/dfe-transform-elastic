// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `userid` pipeline.
pub struct Userid;

impl Transform for Userid {
    fn name(&self) -> &str {
        "userid"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(csv_str) = event.get_string("message") {
                let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b',')
                    .quote(b'\"')
                    .has_headers(false)
                    .from_reader(csv_str.as_bytes());
                if let Some(Ok(record)) = rdr.records().next() {
                    if let Some(val) = record.get(0) {
                        if !val.is_empty() {
                            event.set("panw.panos.virtual_sys", val)?;
                        }
                    }
                    if let Some(val) = record.get(1) {
                        if !val.is_empty() {
                            event.set("panw.panos.source.ip", val)?;
                        }
                    }
                    if let Some(val) = record.get(2) {
                        if !val.is_empty() {
                            event.set("_temp_.srcuser", val)?;
                        }
                    }
                    if let Some(val) = record.get(3) {
                        if !val.is_empty() {
                            event.set("panw.panos.datasourcename", val)?;
                        }
                    }
                    if let Some(val) = record.get(4) {
                        if !val.is_empty() {
                            event.set("panw.panos.event.id", val)?;
                        }
                    }
                    if let Some(val) = record.get(5) {
                        if !val.is_empty() {
                            event.set("panw.panos.repeat_count", val)?;
                        }
                    }
                    if let Some(val) = record.get(6) {
                        if !val.is_empty() {
                            event.set("panw.panos.timeout", val)?;
                        }
                    }
                    if let Some(val) = record.get(7) {
                        if !val.is_empty() {
                            event.set("panw.panos.source.port", val)?;
                        }
                    }
                    if let Some(val) = record.get(8) {
                        if !val.is_empty() {
                            event.set("panw.panos.destination.port", val)?;
                        }
                    }
                    if let Some(val) = record.get(9) {
                        if !val.is_empty() {
                            event.set("panw.panos.datasource", val)?;
                        }
                    }
                    if let Some(val) = record.get(10) {
                        if !val.is_empty() {
                            event.set("panw.panos.datasourcetype", val)?;
                        }
                    }
                    if let Some(val) = record.get(11) {
                        if !val.is_empty() {
                            event.set("panw.panos.sequence_number", val)?;
                        }
                    }
                    if let Some(val) = record.get(12) {
                        if !val.is_empty() {
                            event.set("panw.panos.action_flags", val)?;
                        }
                    }
                    if let Some(val) = record.get(13) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_group_hierarchy1", val)?;
                        }
                    }
                    if let Some(val) = record.get(14) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_group_hierarchy2", val)?;
                        }
                    }
                    if let Some(val) = record.get(15) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_group_hierarchy3", val)?;
                        }
                    }
                    if let Some(val) = record.get(16) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_group_hierarchy4", val)?;
                        }
                    }
                    if let Some(val) = record.get(17) {
                        if !val.is_empty() {
                            event.set("panw.panos.vsys_name", val)?;
                        }
                    }
                    if let Some(val) = record.get(18) {
                        if !val.is_empty() {
                            event.set("panw.panos.device_name", val)?;
                        }
                    }
                    if let Some(val) = record.get(19) {
                        if !val.is_empty() {
                            event.set("panw.panos.vsys_id", val)?;
                        }
                    }
                    if let Some(val) = record.get(20) {
                        if !val.is_empty() {
                            event.set("panw.panos.factortype", val)?;
                        }
                    }
                    if let Some(val) = record.get(21) {
                        if !val.is_empty() {
                            event.set("panw.panos.factorcompletiontime", val)?;
                        }
                    }
                    if let Some(val) = record.get(22) {
                        if !val.is_empty() {
                            event.set("panw.panos.factorno", val)?;
                        }
                    }
                    if let Some(val) = record.get(23) {
                        if !val.is_empty() {
                            event.set("panw.panos.ugflags", val)?;
                        }
                    }
                    if let Some(val) = record.get(24) {
                        if !val.is_empty() {
                            event.set("panw.panos.user_by_source", val)?;
                        }
                    }
                    if let Some(val) = record.get(25) {
                        if !val.is_empty() {
                            event.set("panw.panos.tag.name", val)?;
                        }
                    }
                    if let Some(val) = record.get(26) {
                        if !val.is_empty() {
                            event.set("_temp_.high_res_timestamp", val)?;
                        }
                    }
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;
            event.append("event.category", json!("iam"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "destination.port",
                    event
                        .get("panw.panos.destination.port")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "event.code",
                    event
                        .get("panw.panos.event.id")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "panw.panos.received_time",
                    event.get("event.created").cloned().unwrap_or(Value::Null),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "observer.hostname",
                    event
                        .get("panw.panos.device_name")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "source.ip",
                    event
                        .get("panw.panos.source.ip")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "source.port",
                    event
                        .get("panw.panos.source.port")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set(
                    "source.user.name",
                    event
                        .get("panw.panos.user_by_source")
                        .cloned()
                        .unwrap_or(Value::Null),
                )?;
                Ok(())
            })();

            let _cond =
                { event.has("panw.panos.factorcompletiontime") && !event.has("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("panw.panos.factorcompletiontime") {
                        // Try Java datetime format: CustomTime(\"yyyy/MM/dd HH:mm:ss\")
                        // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                        // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy/MM/dd HH:mm:ss\")")
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        event
                            .get("_ingest.on_failure_message")
                            .cloned()
                            .unwrap_or(Value::Null),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                }
            }

            let _cond =
                { event.has("panw.panos.factorcompletiontime") && event.has("event.timezone") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("panw.panos.factorcompletiontime") {
                        // Try Java datetime format: CustomTime(\"yyyy/MM/dd HH:mm:ss\")
                        // TODO: Convert Java format to chrono strftime (date processor 2.2.3)
                        // chrono::NaiveDateTime::parse_from_str(&date_str, "CustomTime(\"yyyy/MM/dd HH:mm:ss\")")
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        event
                            .get("_ingest.on_failure_message")
                            .cloned()
                            .unwrap_or(Value::Null),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
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
                event.append(
                    "error.message",
                    json!(format!(
                        "error in User ID pipeline: error in [{}] processor{} with tag [{}]{} {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("#_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("/_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        // Final cleanup: remove null/empty fields created during processing
        painless_drop_empty(event.as_value_mut());

        Ok(TransformResult::Continue)
    }
}
