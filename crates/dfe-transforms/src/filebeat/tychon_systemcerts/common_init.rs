// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `common_init` pipeline.
pub struct CommonInit;

impl Transform for CommonInit {
    fn name(&self) -> &str {
        "common_init"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            parse_json_field(event, "message", "tychon")?;

            // Painless script, resolved to its runners at generation time
            drop_empty(event, &DropPolicy { nulls: true, empty_strings: true, shallow: true, ..DropPolicy::none() }, Some("tychon"));

            dot_expand(event, "tychon", "*")?;

        let _cond = { !event.has_value("event.original") };
        if _cond {
            if event.has_value("message") {
                event.rename("message", "event.original")?;
            }
        }

        let _cond = { event.has_value("event.original") };
        if _cond {
            event.remove("message");
        }

            // Painless script
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.tychon?.script?.current_duration != null)\n{\n  ctx.tychon.script.current_duration =\n    (long) Double.parseDouble(ctx.tychon.script.current_duration.toString());\n}\n"#))?;

        event.set("ecs.version", json!("8.17.0"))?;

        event.set("event.module", json!("tychon"))?;

        event.set("event.kind", json!("state"))?;

            event.append("event.type", json!("info"))?;

        Ok(TransformResult::Continue)
    }
}
