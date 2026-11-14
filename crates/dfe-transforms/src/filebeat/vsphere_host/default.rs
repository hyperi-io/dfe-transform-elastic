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

            let _cond = { event.has_value("vsphere.host.triggered_alarms") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "91c801ae13ded4f3911e1f3abfae0c9e16caa25795b791bc50717cd6e01747a8",
                        r#"{"params":null,"known":[{"CollectByLiteral":{"list":"vsphere.host.triggered_alarms","tested":"status","arms":[{"literal":"red","member":"name","target":"alerts"},{"literal":"yellow","member":"name","target":"warnings"}]}}]}"#
                    ),
                )?;
            }

            if event.has_value("alerts") {
                event.rename("alerts", "vsphere.host.alert.names")?;
            }

            if event.has_value("warnings") {
                event.rename("warnings", "vsphere.host.warning.names")?;
            }

            let _cond = {
                event.has_value("vsphere.host.memory.used.bytes")
                    && event.has_value("vsphere.host.memory.total.bytes")
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "4104c9bd9e63fc91ff3fb49b403eebdc4e9ea521e31114dcd7342d5fb09ad3cf",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("vsphere.host.cpu.used.mhz")
                    && event.has_value("vsphere.host.cpu.total.mhz")
            };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "d67a7eb6c4e28b369f83d828f296e77959500b0a0a4e201d7c6dd058bd12dce5",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
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
