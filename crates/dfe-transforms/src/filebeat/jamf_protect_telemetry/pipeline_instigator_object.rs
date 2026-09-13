// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_instigator_object` pipeline.
pub struct PipelineInstigatorObject;

impl Transform for PipelineInstigatorObject {
    fn name(&self) -> &str {
        "pipeline_instigator_object"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            if event.has_value("jamf_protect.telemetry.process.start_time") {
                event.rename("jamf_protect.telemetry.process.start_time", "process.start")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("jamf_protect.telemetry.process.audit_token.egid") {
            if let Some(val) = event.get("jamf_protect.telemetry.process.audit_token.egid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.process.audit_token.egid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.process.audit_token.egid", converted)?;
            }
        }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("jamf_protect.telemetry.process.audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.process.audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.process.audit_token.euid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.process.audit_token.euid", converted)?;
            }
        }
            Ok(())
        })();

        let _cond = { event.has_value("jamf_protect.telemetry.process.audit_token.euid") };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("user.effective.id", json!(event.get("jamf_protect.telemetry.process.audit_token.euid").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();
        }

            if event.has_value("jamf_protect.telemetry.process.executable.sha1") {
                event.rename("jamf_protect.telemetry.process.executable.sha1", "process.hash.sha1")?;
            }

            if event.has_value("jamf_protect.telemetry.process.executable.sha256") {
                event.rename("jamf_protect.telemetry.process.executable.sha256", "process.hash.sha256")?;
            }

            if event.has_value("jamf_protect.telemetry.custom.tty.path") {
                event.rename("jamf_protect.telemetry.custom.tty.path", "jamf_protect.telemetry.tty")?;
            }

        let _cond = { event.has_value("jamf_protect.telemetry.process.tty") };
        if _cond {
        event.set("process.interactive", json!(true))?;
        }

        let _cond = { event.has_value("jamf_protect.telemetry.process.instigator.executable.sha1") };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hash", json!(event.get("jamf_protect.telemetry.process.executable.sha1").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("jamf_protect.telemetry.process.audit_token.pid") {
            if let Some(val) = event.get("jamf_protect.telemetry.process.audit_token.pid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.process.audit_token.pid".into(),
                        message,
                    })?;
                event.set("process.pid", converted)?;
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

            if event.has_value("jamf_protect.telemetry.process.audit_token.uuid") {
                event.rename("jamf_protect.telemetry.process.audit_token.uuid", "process.entity_id")?;
            }

            if event.has_value("jamf_protect.telemetry.process.executable.path") {
                event.rename("jamf_protect.telemetry.process.executable.path", "process.executable")?;
            }

            if event.has_value("jamf_protect.telemetry.thread.thread_id") {
                event.rename("jamf_protect.telemetry.thread.thread_id", "process.thread.id")?;
            }

            if event.has_value("jamf_protect.telemetry.process.signing_id") {
                event.rename("jamf_protect.telemetry.process.signing_id", "process.code_signature.signing_id")?;
            }

        Ok(TransformResult::Continue)
    }
}
