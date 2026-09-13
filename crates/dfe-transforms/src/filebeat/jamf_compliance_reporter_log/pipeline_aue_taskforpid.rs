// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_aue_taskforpid` pipeline.
pub struct PipelineAueTaskforpid;

impl Transform for PipelineAueTaskforpid {
    fn name(&self) -> &str {
        "pipeline_aue_taskforpid"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.arguments.target_port") {
                if let Some(val) = event.get("json.arguments.target_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.arguments.target_port".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.arguments.target.port", converted)?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.arguments.task_port") {
                if let Some(val) = event.get("json.arguments.task_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.arguments.task_port".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.arguments.task.port", converted)?;
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

                // Begin nested pipeline: "pipeline_process_object"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.audit_id") {
                if let Some(val) = event.get("json.process.audit_id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.audit_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
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
                if event.has_value("json.process.effective_group_id") {
                if let Some(val) = event.get("json.process.effective_group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.effective_group_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.effective.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.process.effective_group_name") {
                event.rename("json.process.effective_group_name", "jamf_compliance_reporter.log.process.effective.group.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.effective_user_id") {
                if let Some(val) = event.get("json.process.effective_user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.effective_user_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.effective.user.id", converted)?;
                }
                }
                Ok(())
                })();
                let _cond = { event.has_value("json.process.effective_user_id") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.effective.id", json!(event.get("json.process.effective_user_id").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.process.effective_user_name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("user.effective.name", json!(event.get("json.process.effective_user_name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                if event.has_value("json.process.effective_user_name") {
                event.rename("json.process.effective_user_name", "jamf_compliance_reporter.log.process.effective.user.name")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.process.effective.user.name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.process.effective.user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.group_id") {
                if let Some(val) = event.get("json.process.group_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.group_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.group.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.process.group_name") {
                event.rename("json.process.group_name", "jamf_compliance_reporter.log.process.group.name")?;
                }
                let _cond = { event.has_value("json.process.process_hash") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("process.hash.sha1", json!(event.get("json.process.process_hash").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                let _cond = { event.has_value("json.process.process_hash") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.hash", json!(event.get("json.process.process_hash").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.process_id") {
                if let Some(val) = event.get("json.process.process_id") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.process_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.pid", converted)?;
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
                if event.has_value("json.process.process_name") {
                event.rename("json.process.process_name", "process.name")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.session_id") {
                if let Some(val) = event.get("json.process.session_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.session_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.session.id", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.addr") {
                if let Some(val) = event.get("json.process.terminal_id.addr") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.addr".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.addr", converted)?;
                }
                }
                Ok(())
                })();
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.ip_address") {
                if let Some(val) = event.get("json.process.terminal_id.ip_address") {
                let converted = convert_value(val, "ip")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.ip_address".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.ip_address", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.remove("json.process.terminal_id.ip_address");
                event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
                }
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.process.terminal_id.ip_address") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("jamf_compliance_reporter.log.process.terminal_id.ip_address").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.process.terminal_id.port") {
                if let Some(val) = event.get("json.process.terminal_id.port") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.port".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.port", converted)?;
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
                if event.has_value("json.process.terminal_id.type") {
                if let Some(val) = event.get("json.process.terminal_id.type") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.terminal_id.type".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.terminal_id.type", converted)?;
                }
                }
                Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                if event.has_value("json.process.user_id") {
                if let Some(val) = event.get("json.process.user_id") {
                let converted = convert_value(val, "string")
                .map_err(|message| TransformError::ParseError {
                path: "json.process.user_id".into(),
                message,
                })?;
                event.set("jamf_compliance_reporter.log.process.user.id", converted)?;
                }
                }
                Ok(())
                })();
                if event.has_value("json.process.user_name") {
                event.rename("json.process.user_name", "jamf_compliance_reporter.log.process.user.name")?;
                }
                let _cond = { event.has_value("jamf_compliance_reporter.log.process.user.name") };
                if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("jamf_compliance_reporter.log.process.user.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                }
                // End nested pipeline: "pipeline_process_object"

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
