// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_aue_mount` pipeline.
pub struct PipelineAueMount;

impl Transform for PipelineAueMount {
    fn name(&self) -> &str {
        "pipeline_aue_mount"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.texts") {
                    event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                }

                // Begin nested pipeline: "pipeline_exec_chain_child_object"
                if event.has_value("json.exec_chain_child.parent_path") {
                event.rename("json.exec_chain_child.parent_path", "jamf_compliance_reporter.log.exec_chain_child.parent.path")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.exec_chain_child.parent_pid") {
                if let Some(val) = event.get("json.exec_chain_child.parent_pid") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.exec_chain_child.parent_pid".into(),
                message,
                })?;
                event.set("process.parent.pid", converted)?;
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
                if event.has_value("json.exec_chain_child.parent_uuid") {
                event.rename("json.exec_chain_child.parent_uuid", "jamf_compliance_reporter.log.exec_chain_child.parent.uuid")?;
                }
                // End nested pipeline: "pipeline_exec_chain_child_object"

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

                if event.has_value("json.path") {
                    event.rename("json.path", "jamf_compliance_reporter.log.path")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.attributes.device") {
                if let Some(val) = event.get("json.attributes.device") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.device".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.attributes.device", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.attributes.file_access_mode") {
                    event.rename("json.attributes.file_access_mode", "json.file_access_mode")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.attributes.file_system_id") {
                if let Some(val) = event.get("json.attributes.file_system_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.file_system_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.attributes.file.system.id", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.attributes.node_id") {
                if let Some(val) = event.get("json.attributes.node_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.node_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.attributes.node.id", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.attributes.owner_group_id") {
                if let Some(val) = event.get("json.attributes.owner_group_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.owner_group_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.attributes.owner.group.id", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.attributes.owner_group_name") {
                    event.rename("json.attributes.owner_group_name", "jamf_compliance_reporter.log.attributes.owner.group.name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.attributes.owner_user_id") {
                if let Some(val) = event.get("json.attributes.owner_user_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.owner_user_id".into(),
                            message,
                        })?;
                    event.set("json.attributes.owner_user_id", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("json.attributes.owner_user_id") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.id", json!(event.get("json.attributes.owner_user_id").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.attributes.owner_user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.attributes.owner_user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.attributes.owner_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.arguments.flags") {
                if let Some(val) = event.get("json.arguments.flags") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.arguments.flags".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.arguments.flags", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("json.file_access_mode") };
            if _cond {
                // Painless script
                // Source: int temp = (int)ctx.json?.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"int temp = (int)ctx.json?.file_access_mode;\nctx.jamf_compliance_reporter.log.attributes.file.access_mode = Integer.toOctalString(temp);\n"#))?;
            }

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
