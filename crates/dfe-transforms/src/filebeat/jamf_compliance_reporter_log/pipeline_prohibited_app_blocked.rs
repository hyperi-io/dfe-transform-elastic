// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_prohibited_app_blocked` pipeline.
pub struct PipelineProhibitedAppBlocked;

impl Transform for PipelineProhibitedAppBlocked {
    fn name(&self) -> &str {
        "pipeline_prohibited_app_blocked"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.header.action") {
                    event.rename("json.header.action", "jamf_compliance_reporter.log.header.action")?;
                }

                if event.has_value("json.exec_args.args") {
                    event.rename("json.exec_args.args", "json.args")?;
                }

                if event.has_value("json.exec_args.args_compiled") {
                    event.rename("json.exec_args.args_compiled", "jamf_compliance_reporter.log.exec_args.args_compiled")?;
                }

                if event.has_value("json.exec_env.env.PATH") {
                    event.rename("json.exec_env.env.PATH", "jamf_compliance_reporter.log.exec_env.env.path")?;
                }

                if event.has_value("json.exec_env.env.SHELL") {
                    event.rename("json.exec_env.env.SHELL", "jamf_compliance_reporter.log.exec_env.env.shell")?;
                }

                if event.has_value("json.exec_env.env.SSH_AUTH_SOCK") {
                    event.rename("json.exec_env.env.SSH_AUTH_SOCK", "jamf_compliance_reporter.log.exec_env.env.ssh_auth_sock")?;
                }

                if event.has_value("json.exec_env.env.TMPDIR") {
                    event.rename("json.exec_env.env.TMPDIR", "jamf_compliance_reporter.log.exec_env.env.tmpdir")?;
                }

            let _cond = { event.has_value("json.exec_env.env.USER") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.exec_env.env.USER").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.exec_env.env.USER") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.exec_env.env.USER").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                if event.has_value("json.exec_env.env.XPC_FLAGS") {
                    event.rename("json.exec_env.env.XPC_FLAGS", "jamf_compliance_reporter.log.exec_env.env.xpc.flags")?;
                }

                if event.has_value("json.exec_env.env.XPC_SERVICE_NAME") {
                    event.rename("json.exec_env.env.XPC_SERVICE_NAME", "jamf_compliance_reporter.log.exec_env.env.xpc.service_name")?;
                }

                if event.has_value("json.exec_env.env_compiled") {
                    event.rename("json.exec_env.env_compiled", "jamf_compliance_reporter.log.exec_env.env_compiled")?;
                }

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.identity.signer_id") {
                if let Some(val) = event.get("json.identity.signer_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.identity.signer_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.identity.signer.id", converted)?;
                }
            }
                Ok(())
            })();

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.identity.team_id") {
                if let Some(val) = event.get("json.identity.team_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.identity.team_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.identity.team.id", converted)?;
                }
            }
                Ok(())
            })();

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.audit_id") {
                if let Some(val) = event.get("json.subject.audit_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.audit_id".into(),
                            message,
                        })?;
                    event.set("process.real_user.id", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.subject.audit_user_name") {
                    event.rename("json.subject.audit_user_name", "process.real_user.name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.effective_group_id") {
                if let Some(val) = event.get("json.subject.effective_group_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.effective_group_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.effective.group.id", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.subject.effective_group_name") {
                    event.rename("json.subject.effective_group_name", "jamf_compliance_reporter.log.subject.effective.group.name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.effective_user_id") {
                if let Some(val) = event.get("json.subject.effective_user_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.effective_user_id".into(),
                            message,
                        })?;
                    event.set("user.effective.id", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if let Some(v) = event.get("user.effective.id").cloned() {
                event.set("jamf_compliance_reporter.log.subject.effective.user.id", v)?;
            }
                Ok(())
            })();

            let _cond = { event.has_value("json.subject.effective_user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.subject.effective_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.subject.effective_user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.subject.effective_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                if event.has_value("json.subject.effective_user_name") {
                    event.rename("json.subject.effective_user_name", "user.effective.name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if let Some(v) = event.get("user.effective.name").cloned() {
                event.set("jamf_compliance_reporter.log.subject.effective.user.name", v)?;
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.group_id") {
                if let Some(val) = event.get("json.subject.group_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.group_id".into(),
                            message,
                        })?;
                    event.set("user.group.id", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.subject.group_name") {
                    event.rename("json.subject.group_name", "user.group.name")?;
                }

                if event.has_value("json.subject.process_hash") {
                    event.rename("json.subject.process_hash", "process.hash.sha1")?;
                }

            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("process.hash.sha1").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.subject.process_id") {
                if let Some(val) = event.get("json.subject.process_id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.process_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.process.pid", converted)?;
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

                if event.has_value("json.subject.process_information") {
                    event.rename("json.subject.process_information", "jamf_compliance_reporter.log.subject.process.information")?;
                }

                if event.has_value("json.subject.process_name") {
                    event.rename("json.subject.process_name", "process.name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.responsible_process_id") {
                if let Some(val) = event.get("json.subject.responsible_process_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.responsible_process_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.responsible.process.id", converted)?;
                }
            }
                Ok(())
            })();

                if event.has_value("json.subject.responsible_process_name") {
                    event.rename("json.subject.responsible_process_name", "jamf_compliance_reporter.log.subject.responsible.process.name")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.session_id") {
                if let Some(val) = event.get("json.subject.session_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.session_id".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.session.id", converted)?;
                }
            }
                Ok(())
            })();

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.subject.terminal_id.ip_address") {
                if let Some(val) = event.get("json.subject.terminal_id.ip_address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.terminal_id.ip_address".into(),
                            message,
                        })?;
                    event.set("json.subject.terminal_id.ip_address", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("json.subject.terminal_id.ip_address");
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("host.ip", json!(event.get("json.subject.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.subject.terminal_id.ip_address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("json.subject.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.subject.terminal_id.port") {
                if let Some(val) = event.get("json.subject.terminal_id.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.terminal_id.port".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.terminal_id.port", converted)?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.terminal_id.type") {
                if let Some(val) = event.get("json.subject.terminal_id.type") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.terminal_id.type".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.subject.terminal_id.type", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.subject.user_id") {
                if let Some(val) = event.get("json.subject.user_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.subject.user_id".into(),
                            message,
                        })?;
                    event.set("user.id", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.has_value("json.subject.user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("json.subject.user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.subject.user_name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("json.subject.user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                if event.has_value("json.texts") {
                    event.rename("json.texts", "jamf_compliance_reporter.log.texts")?;
                }

                // Painless script
                // Source: def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def args_list = new ArrayList();\nctx.process.args = args_list;\nif (ctx.json?.args != null) {\n  for (Map.Entry m : ctx.json.args.entrySet()) {\n    ctx.process.args.add(m.getValue());\n  }\n}\n"#))?;

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
