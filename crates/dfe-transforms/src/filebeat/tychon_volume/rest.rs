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
        let _cond = { !event.has_value("tychon.volume.block_size") };
        if _cond {
        event.set("tychon.volume.block_size", json!(0))?;
        }

        let _cond = { !event.has_value("tychon.volume.freespace") };
        if _cond {
        event.set("tychon.volume.freespace", json!(0))?;
        }

        let _cond = { !event.has_value("tychon.volume.percent_full") || event.get_str("tychon.volume.percent_full") == Some("NaN") };
        if _cond {
        event.set("tychon.volume.percent_full", json!(100))?;
        }

        let _cond = { !event.has_value("tychon.volume.size") };
        if _cond {
        event.set("tychon.volume.size", json!(0))?;
        }

        if event.has_value("tychon.volume.automount") {
            if let Some(val) = event.get("tychon.volume.automount") {
                let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.volume.automount".into(),
                        message,
                    })?;
                event.set("tychon.volume.automount", converted)?;
            }
        }

        if event.has_value("tychon.volume.block_size") {
            if let Some(val) = event.get("tychon.volume.block_size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.volume.block_size".into(),
                        message,
                    })?;
                event.set("tychon.volume.block_size", converted)?;
            }
        }

        if event.has_value("tychon.volume.freespace") {
            if let Some(val) = event.get("tychon.volume.freespace") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.volume.freespace".into(),
                        message,
                    })?;
                event.set("tychon.volume.freespace", converted)?;
            }
        }

        if event.has_value("tychon.volume.percent_full") {
            if let Some(val) = event.get("tychon.volume.percent_full") {
                let converted = convert_value(val, "float")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.volume.percent_full".into(),
                        message,
                    })?;
                event.set("tychon.volume.percent_full", converted)?;
            }
        }

        if event.has_value("tychon.volume.size") {
            if let Some(val) = event.get("tychon.volume.size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "tychon.volume.size".into(),
                        message,
                    })?;
                event.set("tychon.volume.size", converted)?;
            }
        }

        event.set("event.category", Value::Array(vec![json!("configuration")]))?;

        Ok(TransformResult::Continue)
    }
}
