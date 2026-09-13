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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "_temp_")?;

            let _cond = { !event.has_value("_temp_.ts") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.rename("_temp_", "zeek.files")?;

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.set("ecs.version", json!("8.17.0"))?;

            event.append("event.category", json!("file"))?;

            event.append("event.type", json!("info"))?;

            if event.has_value("zeek.files.conn_uids") {
                event.rename("zeek.files.conn_uids", "zeek.files.session_ids")?;
            }

            let _cond = { event.has_value("zeek.files.mime_type") };
            if _cond {
                if let Some(v) = event.get("zeek.files.mime_type").cloned() {
                    event.set("file.mime_type", v)?;
                }
            }

            if event.has_value("zeek.files.filename") {
                event.rename("zeek.files.filename", "file.name")?;
            }

            if event.has_value("zeek.files.total_bytes") {
                event.rename("zeek.files.total_bytes", "file.size")?;
            }

            let _cond = { event.has_value("zeek.files.md5") };
            if _cond {
                if let Some(v) = event.get("zeek.files.md5").cloned() {
                    event.set("file.hash.md5", v)?;
                }
            }

            let _cond = { event.has_value("zeek.files.sha1") };
            if _cond {
                if let Some(v) = event.get("zeek.files.sha1").cloned() {
                    event.set("file.hash.sha1", v)?;
                }
            }

            let _cond = { event.has_value("zeek.files.sha256") };
            if _cond {
                if let Some(v) = event.get("zeek.files.sha256").cloned() {
                    event.set("file.hash.sha256", v)?;
                }
            }

            if let Some(date_str) = event.get_as_string("zeek.files.ts") {
                match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "zeek.files.ts".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("zeek.files.ts").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "zeek.files.ts".into(),
                });
            }

            let _cond = { event.has_value("zeek.files.session_ids") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.zeek.session_id = ctx.zeek.files.session_ids[0];
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(r#"ctx.zeek.session_id = ctx.zeek.files.session_ids[0];"#),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("zeek.session_id") };
            if _cond {
                if let Some(v) = event.get("zeek.session_id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            if event.has_value("zeek.files.tx_hosts") {
                foreach_array(event, "zeek.files.tx_hosts", |event| {
                    event.append(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: ctx.zeek.files.tx_host = ctx.zeek.files.tx_hosts[0]; ctx.zeek.files.remove('tx_hosts');
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.zeek.files.tx_host = ctx.zeek.files.tx_hosts[0]; ctx.zeek.files.remove('tx_hosts');"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("zeek.files.tx_host") };
            if _cond {
                if let Some(v) = event.get("zeek.files.tx_host").cloned() {
                    event.set("server.ip", v)?;
                }
            }

            if event.has_value("zeek.files.rx_hosts") {
                foreach_array(event, "zeek.files.rx_hosts", |event| {
                    event.append(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: ctx.zeek.files.rx_host = ctx.zeek.files.rx_hosts[0]; ctx.zeek.files.remove('rx_hosts');
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.zeek.files.rx_host = ctx.zeek.files.rx_hosts[0]; ctx.zeek.files.remove('rx_hosts');"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("zeek.files.rx_host") };
            if _cond {
                event.set(
                    "client.ip",
                    json!(
                        event
                            .get("zeek.files.rx_host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("zeek.files.x509");

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
