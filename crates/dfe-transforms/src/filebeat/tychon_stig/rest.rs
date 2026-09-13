// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `rest` pipeline.
pub struct Rest;

impl Transform for Rest {
    fn name(&self) -> &str {
        "rest"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            if event.has_value("tychon.rule.test_result") {
                event.rename("tychon.rule.test_result", "tychon.rule.result")?;
            }

        if event.has_value("tychon.rule.weight") {
            if let Some(val) = event.get("tychon.rule.weight") {
                let converted = convert_value(val, "float")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.rule.weight".into(),
                        message,
                    })?;
                event.set("tychon.rule.weight", converted)?;
            }
        }

        event.set("tychon.rule.result_score", json!(0))?;

        let _cond = { event.get_str("tychon.rule.result") == Some("fail") };
        if _cond {
        event.set("tychon.rule.result_score", json!(10))?;
        }

        event.set("event.category", Value::Array(vec![json!("vulnerability"), json!("configuration")]))?;

        let _cond = { event.has_value("tychon.benchmark.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("tychon.benchmark.hash").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("tychon.rule.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("rule.id", v)?;
        }

        if let Some(v) = event.get("tychon.rule.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("rule.name", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
