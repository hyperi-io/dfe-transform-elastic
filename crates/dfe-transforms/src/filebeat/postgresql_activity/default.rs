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
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            {
                let mut values = Vec::new();
                if let Some(v) = event.get("postgresql.activity.query") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "postgresql.activity.query_id",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }
            Ok(())
        })();

        Ok(TransformResult::Continue)
    }
}
