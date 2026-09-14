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
        if event.has_value("tychon.host.risk.calculated_score") {
            if let Some(val) = event.get("tychon.host.risk.calculated_score") {
                let converted = convert_value(val, "float")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.host.risk.calculated_score".into(),
                        message,
                    })?;
                event.set("tychon.host.risk.calculated_score", converted)?;
            }
        }

        let _cond = { !event.has_value("tychon.host.security.antivirus.exists") };
        if _cond {
        event.set("tychon.host.security.antivirus.exists", json!("false"))?;
        }

            map_strings(event, "tychon.host.security.antivirus.exists", "tychon.host.security.antivirus.exists", str::to_lowercase)?;

            if let Some(s) = event.get_string("tychon.host.security.antivirus.exists") {
                let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                if parts.len() > 1 {
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                }
                event.set("_tmp_av", Value::Array(parts))?;
            }

        event.set("tychon.host.security.antivirus.exists", json!(event.get("_tmp_av.0").map_or_else(String::new, template_to_string)))?;

            if event.remove("_tmp_av").is_none() {
                return Err(TransformError::FieldNotFound { path: "_tmp_av".into() });
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(val) = event.get("tychon.host.memory.size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.host.memory.size".into(),
                        message,
                    })?;
                event.set("tychon.host.memory.size", converted)?;
            }
            Ok(())
        })();

        event.set("event.category", Value::Array(vec![json!("host")]))?;

        let _cond = { event.has_value("tychon.host.risk.count.signature_hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("tychon.host.risk.count.signature_hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.host.risk.score.signature_hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("tychon.host.risk.score.signature_hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("tychon.host.risk.weight.signature_hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("tychon.host.risk.weight.signature_hash").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("tychon.host.architecture").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.architecture", v)?;
        }

        if let Some(v) = event.get("tychon.host.os.kernel").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.os.kernel", v)?;
        }

        if let Some(v) = event.get("tychon.host.risk.calculated_score").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.risk.calculated_score", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
