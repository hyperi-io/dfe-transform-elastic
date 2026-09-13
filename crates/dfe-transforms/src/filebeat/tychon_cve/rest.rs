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
        let _cond = { !event.has_value("tychon.vulnerability.due_date") };
        if _cond {
        event.set("tychon.vulnerability.due_date", json!("1970-01-01T00:00:01Z"))?;
        }

            if let Some(date_str) = event.get_as_string("tychon.vulnerability.due_date") {
                match parse_date_out(&date_str, &["strict_date_optional_time", "epoch_millis", "date", "MM/dd/yyyy"], None, Some("yyyy-MM-dd'T'HH:mm:ss.SSSXXX")) {
                    Some(parsed) => event.set("tychon.vulnerability.due_date", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "tychon.vulnerability.due_date".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

        let _cond = { !event.has_value("tychon.vulnerability.score.base") };
        if _cond {
        event.set("tychon.vulnerability.score.base", json!("0.0"))?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(val) = event.get("tychon.vulnerability.score.base") {
                let converted = convert_value(val, "float")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.vulnerability.score.base".into(),
                        message,
                    })?;
                event.set("tychon.vulnerability.score.base", converted)?;
            }
            Ok(())
        })();

            if let Some(val) = event.get("tychon.vulnerability.year") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.vulnerability.year".into(),
                        message,
                    })?;
                event.set("tychon.vulnerability.year", converted)?;
            }

        event.set("tychon.vulnerability.scanner.vendor", json!("tychon"))?;

        event.set("tychon.vulnerability.category", Value::Array(vec![json!("oval")]))?;

        event.set("tychon.vulnerability.enumeration", json!("CVE"))?;

        event.set("event.category", Value::Array(vec![json!("vulnerability")]))?;

            // Painless script
            // Source: if (ctx.tychon.vulnerability?.result == \"fail\") {\n  ctx.event.outcome = \"failure\"\n} else if (ctx.tychon.vulnerability?.result == \"pass\") {\n  ctx.event.outcome = \"success\"\n} else {\n  ctx.event.outcome = \"unknown\"\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.tychon.vulnerability?.result == \"fail\") {\n  ctx.event.outcome = \"failure\"\n} else if (ctx.tychon.vulnerability?.result == \"pass\") {\n  ctx.event.outcome = \"success\"\n} else {\n  ctx.event.outcome = \"unknown\"\n}\n"#))?;

        if let Some(v) = event.get("tychon.vulnerability.category").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("vulnerability.category", v)?;
        }

        if let Some(v) = event.get("tychon.vulnerability.classification").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("vulnerability.classification", v)?;
        }

        if let Some(v) = event.get("tychon.vulnerability.enumeration").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("vulnerability.enumeration", v)?;
        }

        if let Some(v) = event.get("tychon.vulnerability.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("vulnerability.id", v)?;
        }

        if let Some(v) = event.get("tychon.vulnerability.reference").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("vulnerability.reference", v)?;
        }

        if let Some(v) = event.get("tychon.vulnerability.scanner.vendor").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("vulnerability.scanner.vendor", v)?;
        }

        if let Some(v) = event.get("tychon.vulnerability.score.base").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("vulnerability.score.base", v)?;
        }

        if let Some(v) = event.get("tychon.vulnerability.severity").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("vulnerability.severity", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
