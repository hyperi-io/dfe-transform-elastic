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
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { event.has_value("vsphere.virtualmachine.triggered_alarms") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def alerts = []; def warnings = []; for (alarm in ctx.vsphere.virtualmachine.triggered_alarms) {\n    if (alarm.status == 'red') {\n    alerts.add(alarm.name);\n    }\n    if (alarm.status == 'yellow') {\n    warnings.add(alarm.name);\n    }\n} ctx.alerts = alerts; ctx.warnings = warnings\n"#
                    ),
                )?;
            }

            if event.has_value("alerts") {
                event.rename("alerts", "vsphere.virtualmachine.alert.names")?;
            }

            if event.has_value("warnings") {
                event.rename("warnings", "vsphere.virtualmachine.warning.names")?;
            }

            // The pipeline's script, emitted as the runtime's equivalent
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
