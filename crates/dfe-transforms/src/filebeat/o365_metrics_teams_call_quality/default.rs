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
        event.set("ecs.version", json!("8.17.0"))?;

        let _cond = { event.get_str("message") == Some("want_more") };
        if _cond {
            return Ok(TransformResult::Drop);
        }

        let _cond = { event.has_value("error.message") };
        if _cond {
            event.set("event.kind", json!("error"))?;
        }

        {
            let mut values = Vec::new();
            if let Some(v) = event.get("o365.metrics.teams.call.quality.call_record_id") {
                values.push(v.clone());
            } else {
                return Err(TransformError::FieldNotFound {
                    path: "o365.metrics.teams.call.quality.call_record_id".into(),
                });
            }
            if !values.is_empty() {
                event.set("_id", json!(fingerprint_default(&values)))?;
            }
        }

        if let Some(v) = event
            .get("o365.metrics.teams.call.quality.start_date_time")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("@timestamp", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
