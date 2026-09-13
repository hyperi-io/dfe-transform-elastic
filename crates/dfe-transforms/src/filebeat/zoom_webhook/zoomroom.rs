// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `zoomroom` pipeline.
pub struct Zoomroom;

impl Transform for Zoomroom {
    fn name(&self) -> &str {
        "zoomroom"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { ["zoomroom.checked_in", "zoomroom.checked_out"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("event.action") == Some("zoomroom.checked_in") };
            if _cond {
                event.append("event.type", json!("start"))?;
            }

            let _cond = { event.get_str("event.action") == Some("zoomroom.checked_out") };
            if _cond {
                event.append("event.type", json!("end"))?;
            }

                if event.has_value("zoom.object") {
                    event.rename("zoom.object", "zoom.zoomroom")?;
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
