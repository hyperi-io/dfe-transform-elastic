// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `journald` pipeline.
pub struct Journald;

impl Transform for Journald {
    fn name(&self) -> &str {
        "journald"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(v) = event.get("_ingest.timestamp").cloned() {
                event.set("event.ingested", v)?;
            }

            if let Some(v) = event.get("event.original").cloned() {
                event.set("message", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if let Some(v) = event.get("journald.pid").cloned() {
                event.set("process.pid", v)?;
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("process.name", json!(event.get("journald.process.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("host.hostname") && event.get_str("host.hostname") != Some("") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.hostname").map_or_else(String::new, template_to_string)))?;
            }

                event.remove("journald");
                event.remove("process.thread");
                event.remove("syslog");
                event.remove("systemd");
                event.remove("message_id");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
