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
        let v = json!("logs");
        if !painless_is_empty_value(&v) {
            event.set("data_stream.type", v)?;
        }

        let v = json!("osquery_manager.result");
        if !painless_is_empty_value(&v) {
            event.set("data_stream.dataset", v)?;
        }

        Ok(TransformResult::Continue)
    }
}
