// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `config` pipeline.
pub struct Config;

impl Transform for Config {
    fn name(&self) -> &str {
        "config"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(csv_str) = event.get_string("message") {
                let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b',')
                    .quote(b'\"')
                    .has_headers(false)
                    .from_reader(csv_str.as_bytes());
                if let Some(Ok(record)) = rdr.records().next() {
                    if let Some(val) = record.get(0) {
                        if !val.is_empty() {
                            event.set("panw.panos.host.ip", val)?;
                        }
                    }
                    if let Some(val) = record.get(1) {
                        if !val.is_empty() {
                            event.set("panw.panos.virtual_sys", val)?;
                        }
                    }
                    if let Some(val) = record.get(2) {
                        if !val.is_empty() {
                            event.set("panw.panos.cmd", val)?;
                        }
                    }
                    if let Some(val) = record.get(3) {
                        if !val.is_empty() {
                            event.set("panw.panos.admin", val)?;
                        }
                    }
                    if let Some(val) = record.get(4) {
                        if !val.is_empty() {
                            event.set("panw.panos.client_type", val)?;
                        }
                    }
                    if let Some(val) = record.get(5) {
                        if !val.is_empty() {
                            event.set("panw.panos.result", val)?;
                        }
                    }
                    if let Some(val) = record.get(6) {
                        if !val.is_empty() {
                            event.set("panw.panos.path", val)?;
                        }
                    }
                    if let Some(val) = record.get(7) {
                        if !val.is_empty() {
                            event.set("_temp_.check_field", val)?;
                        }
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_temp_.check_field") {
                    if let Some(val) = event.get("_temp_.check_field") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "_temp_.check_field".into(),
                                message,
                            }
                        })?;
                        event.set("_temp_.check_field", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                !(event
                    .get("_temp_.check_field")
                    .is_some_and(|v| v.is_number()))
            };
            if _cond {
                if let Some(csv_str) = event.get_string("message") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.host.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.virtual_sys", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.cmd", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.admin", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.client_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.result", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.path", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.before_change_detail", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.after_change_detail", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.sequence_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.action_flags", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy1", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy2", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy3", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy4", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.comment", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("_temp_.future_use1", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("_temp_.high_res_timestamp", val)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("_temp_.check_field")
                    .is_some_and(|v| v.is_number())
            };
            if _cond {
                if let Some(csv_str) = event.get_string("message") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.host.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.virtual_sys", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.cmd", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.admin", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.client_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.result", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.path", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.sequence_number", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.action_flags", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy1", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy2", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy3", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_hierarchy4", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.vsys_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.device_group_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("panw.panos.comment", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("_temp_.future_use1", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            let val = val.trim();
                            if !val.is_empty() {
                                event.set("_temp_.high_res_timestamp", val)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("panw.panos.cmd") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event.action = params.get(ctx.panw.panos.cmd);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_params(
                        event,
                        cached_script!(r#"ctx.event.action = params.get(ctx.panw.panos.cmd);"#),
                        cached_params!(
                            "{\"add\":\"cmd-add\",\"clone\":\"cmd-clone\",\"commit\":\"cmd-commit\",\"delete\":\"cmd-delete\",\"edit\":\"cmd-edit\",\"move\":\"cmd-move\",\"rename\":\"cmd-rename\",\"set\":\"cmd-set\"}"
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("panw.panos.result") == Some("Succeeded") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("success"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("panw.panos.result") == Some("Failed") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("failure"))?;
                    Ok(())
                })();
            }

            let _cond = {
                !(["Succeeded", "Failed"]
                    .contains(&event.get_str("panw.panos.action").unwrap_or("")))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("unknown"))?;
                    Ok(())
                })();
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("configuration"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.host.ip").cloned() {
                    event.set("host.ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("panw.panos.device_name").cloned() {
                    event.set("observer.hostname", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("panw.panos.result") == Some("Succeeded") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("panw.panos.result") == Some("Failed") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond =
                { !event.has_value("event.outcome") || event.get_str("event.outcome") == Some("") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
        Ok(TransformResult::Continue)
    }
}
