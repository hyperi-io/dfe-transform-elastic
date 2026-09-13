// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_aue_posix_spawn` pipeline.
pub struct PipelineAuePosixSpawn;

impl Transform for PipelineAuePosixSpawn {
    fn name(&self) -> &str {
        "pipeline_aue_posix_spawn"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.arguments.child_PID") {
                if let Some(val) = event.get("json.arguments.child_PID") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.arguments.child_PID".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.arguments.child.pid", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.exec_args.args") {
                    event.rename("json.exec_args.args", "json.args")?;
                }

                if event.has_value("json.exec_args.args_compiled") {
                    event.rename("json.exec_args.args_compiled", "jamf_compliance_reporter.log.exec_args.args_compiled")?;
                }

                if event.has_value("json.exec_env.env.XPC_FLAGS") {
                    event.rename("json.exec_env.env.XPC_FLAGS", "jamf_compliance_reporter.log.exec_env.env.xpc.flags")?;
                }

                if event.has_value("json.exec_env.env_compiled") {
                    event.rename("json.exec_env.env_compiled", "jamf_compliance_reporter.log.exec_env.env.compiled")?;
                }

                if event.has_value("json.path") {
                    event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                }

                if event.has_value("json.exec_chain_parent.uuid") {
                    event.rename("json.exec_chain_parent.uuid", "jamf_compliance_reporter.log.exec_chain_parent.uuid")?;
                }

                // Painless script
                // Source: def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n"#))?;

                // Begin nested pipeline: "pipeline_identity_object"
                if event.has_value("json.identity.cd_hash") {
                event.rename("json.identity.cd_hash", "jamf_compliance_reporter.log.identity.cd_hash")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.identity.cd_hash") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("jamf_compliance_reporter.log.identity.cd_hash").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                if event.has_value("json.identity.signer_id") {
                event.rename("json.identity.signer_id", "jamf_compliance_reporter.log.identity.signer.id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.signer_id_truncated") {
                if let Some(val) = event.get("json.identity.signer_id_truncated") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.signer_id_truncated".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.signer.id_truncated", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.signer_type") {
                if let Some(val) = event.get("json.identity.signer_type") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.signer_type".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.signer.type", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.identity.team_id") {
                event.rename("json.identity.team_id", "jamf_compliance_reporter.log.identity.team.id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.identity.team_id_truncated") {
                if let Some(val) = event.get("json.identity.team_id_truncated") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.identity.team_id_truncated".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.identity.team.id_truncated", converted)?;
                }
                }
                Ok(())
                })();
                // End nested pipeline: "pipeline_identity_object"

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
