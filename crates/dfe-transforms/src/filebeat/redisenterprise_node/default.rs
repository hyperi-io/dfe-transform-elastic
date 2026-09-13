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
        event.set("ecs.version", json!("8.11.0"))?;

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if event.has_value("prometheus") {
                event.rename("prometheus", "redisenterprise.node")?;
            }
            Ok(())
        })();

        foreach_array(event, "redisenterprise.node", |event| {
            gsub_field(
                event,
                "_ingest._key",
                "_ingest._key",
                cached_regex!("node_"),
                "",
            )?;
            Ok(())
        })?;

        Ok(TransformResult::Continue)
    }
}
