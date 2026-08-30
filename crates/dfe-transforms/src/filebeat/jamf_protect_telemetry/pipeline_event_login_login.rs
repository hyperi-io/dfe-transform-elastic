// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event_login_login` pipeline.
pub struct PipelineEventLoginLogin;

impl Transform for PipelineEventLoginLogin {
    fn name(&self) -> &str {
        "pipeline_event_login_login"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("event.reason", json!("A user attempted to log in using /usr/bin/login"))?;

            event.append("event.type", json!("start"))?;

            event.append("event.category", json!("authentication"))?;

            // Painless script
            // Source: ctx.event = ctx.event != null ? ctx.event : new HashMap(); if (ctx.jamf_protect?.telemetry?.event?.login_login?.success instanceof boolean) {\n  if (ctx.jamf_protect.telemetry.event.login_login.success) {\n    ctx.event.outcome = 'success';\n  } else {\n    ctx.event.outcome = 'failure';\n  }\n} if (ctx.event.outcome == null) {\n  ctx.event.outcome = 'unknown';\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"ctx.event = ctx.event != null ? ctx.event : new HashMap(); if (ctx.jamf_protect?.telemetry?.event?.login_login?.success instanceof boolean) {\n  if (ctx.jamf_protect.telemetry.event.login_login.success) {\n    ctx.event.outcome = 'success';\n  } else {\n    ctx.event.outcome = 'failure';\n  }\n} if (ctx.event.outcome == null) {\n  ctx.event.outcome = 'unknown';\n}\n"#))?;

            if event.has_value("jamf_protect.telemetry.event.login_login.username") {
                event.rename("jamf_protect.telemetry.event.login_login.username", "user.name")?;
            }

        if event.has_value("jamf_protect.telemetry.event.login_login.uid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.login_login.uid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.login_login.uid".into(),
                        message,
                    })?;
                event.set("user.id", converted)?;
            }
        }

            // Begin nested pipeline: "pipeline_object_process"
            if event.has_value("jamf_protect.telemetry.process.start_time") {
            event.rename("jamf_protect.telemetry.process.start_time", "process.start")?;
            }
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
            let _cond = { event.has_value("jamf_protect.telemetry.process.audit_token.euid") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.append_unique("user.effective.id", json!(event.get("jamf_protect.telemetry.process.audit_token.euid").map_or_else(String::new, template_to_string)))?;
            Ok(())
            })();
            }
            if event.has_value("jamf_protect.telemetry.process.audit_token.e_username") {
            event.rename("jamf_protect.telemetry.process.audit_token.e_username", "process.user.name")?;
            }
            if event.has_value("jamf_protect.telemetry.process.is_platform_binary") {
            event.rename("jamf_protect.telemetry.process.is_platform_binary", "process.platform_binary")?;
            }
            if event.has_value("jamf_protect.telemetry.process.is_es_client") {
            event.rename("jamf_protect.telemetry.process.is_es_client", "process.endpoint_security_client")?;
            }
            if event.has_value("jamf_protect.telemetry.process.cdhash") {
            event.rename("jamf_protect.telemetry.process.cdhash", "process.hash.cdhash")?;
            }
            if event.has_value("jamf_protect.telemetry.process.executable.sha1") {
            event.rename("jamf_protect.telemetry.process.executable.sha1", "process.hash.sha1")?;
            }
            if event.has_value("jamf_protect.telemetry.process.executable.sha256") {
            event.rename("jamf_protect.telemetry.process.executable.sha256", "process.hash.sha256")?;
            }
            let _cond = { event.has_value("process.hash.sha1") };
            if _cond {
            event.append_unique("related.hash", json!(event.get("process.hash.sha1").map_or_else(String::new, template_to_string)))?;
            }
            let _cond = { event.has_value("process.hash.sha256") };
            if _cond {
            event.append_unique("related.hash", json!(event.get("process.hash.sha256").map_or_else(String::new, template_to_string)))?;
            }
            let _cond = { event.has_value("process.hash.cdhash") && event.get_str("process.hash.cdhash") != Some("") };
            if _cond {
            event.append_unique("related.hash", json!(event.get("process.hash.cdhash").map_or_else(String::new, template_to_string)))?;
            }
            if event.has_value("jamf_protect.telemetry.custom.tty.path") {
            event.rename("jamf_protect.telemetry.custom.tty.path", "jamf_protect.telemetry.tty")?;
            }
            let _cond = { event.has_value("jamf_protect.telemetry.process.tty") };
            if _cond {
            event.set("process.interactive", json!(true))?;
            }
            let _cond = { !event.has_value("jamf_protect.telemetry.process.tty") };
            if _cond {
            event.set("process.interactive", json!(false))?;
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
            // Painless script
            // Source: if (ctx.process?.executable != null) {\n    int lastSlashIndex = ctx.process.executable.lastIndexOf('/');\n    if (lastSlashIndex != -1) {\n        ctx.process.name = ctx.process.executable.substring(lastSlashIndex + 1);\n    } else {\n        ctx.process.name = ctx.process.executable; // Fallback if no slash is found\n    }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.process?.executable != null) {\n    int lastSlashIndex = ctx.process.executable.lastIndexOf('/');\n    if (lastSlashIndex != -1) {\n        ctx.process.name = ctx.process.executable.substring(lastSlashIndex + 1);\n    } else {\n        ctx.process.name = ctx.process.executable; // Fallback if no slash is found\n    }\n}\n"#))?;
            if event.has_value("jamf_protect.telemetry.thread.thread_id") {
            event.rename("jamf_protect.telemetry.thread.thread_id", "process.thread.id")?;
            }
            if event.has_value("jamf_protect.telemetry.process.signing_id") {
            event.rename("jamf_protect.telemetry.process.signing_id", "process.code_signature.signing_id")?;
            }
            if event.has_value("jamf_protect.telemetry.process.team_id") {
            event.rename("jamf_protect.telemetry.process.team_id", "process.code_signature.team_id")?;
            }
            if event.has_value("jamf_protect.telemetry.process.codesigning_flags") {
            event.rename("jamf_protect.telemetry.process.codesigning_flags", "process.code_signature.flags")?;
            }
            if event.has_value("jamf_protect.telemetry.process.parent_audit_token.uuid") {
            event.rename("jamf_protect.telemetry.process.parent_audit_token.uuid", "process.parent.entity_id")?;
            }
            if event.has_value("jamf_protect.telemetry.process.parent_audit_token.pid") {
            event.rename("jamf_protect.telemetry.process.parent_audit_token.pid", "process.parent.pid")?;
            }
            if event.has_value("jamf_protect.telemetry.process.parent_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.process.parent_audit_token.euid") {
            let converted = convert_value(val, "string")
            .map_err(|message| TransformError::ParseError {
            path: "jamf_protect.telemetry.process.parent_audit_token.euid".into(),
            message,
            })?;
            event.set("process.parent.user.id", converted)?;
            }
            }
            if event.has_value("jamf_protect.telemetry.process.responsible_audit_token.e_username") {
            event.rename("jamf_protect.telemetry.process.responsible_audit_token.e_username", "process.parent.user.name")?;
            }
            if event.has_value("jamf_protect.telemetry.process.parent_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.process.parent_audit_token.ruid") {
            let converted = convert_value(val, "string")
            .map_err(|message| TransformError::ParseError {
            path: "jamf_protect.telemetry.process.parent_audit_token.ruid".into(),
            message,
            })?;
            event.set("process.parent.real_user.id", converted)?;
            }
            }
            if event.has_value("jamf_protect.telemetry.process.parent_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.process.parent_audit_token.rgid") {
            let converted = convert_value(val, "string")
            .map_err(|message| TransformError::ParseError {
            path: "jamf_protect.telemetry.process.parent_audit_token.rgid".into(),
            message,
            })?;
            event.set("process.parent.real_group.id", converted)?;
            }
            }
            if event.has_value("jamf_protect.telemetry.process.parent_audit_token.signing_id") {
            event.rename("jamf_protect.telemetry.process.parent_audit_token.signing_id", "process.parent.code_signature.signing_id")?;
            }
            if event.has_value("jamf_protect.telemetry.process.parent_audit_token.exec_path") {
            event.rename("jamf_protect.telemetry.process.parent_audit_token.exec_path", "process.parent.executable")?;
            }
            if event.has_value("jamf_protect.telemetry.process.responsible_audit_token.uuid") {
            event.rename("jamf_protect.telemetry.process.responsible_audit_token.uuid", "process.responsible.entity_id")?;
            }
            if event.has_value("jamf_protect.telemetry.process.responsible_audit_token.pid") {
            event.rename("jamf_protect.telemetry.process.responsible_audit_token.pid", "process.responsible.pid")?;
            }
            if event.has_value("jamf_protect.telemetry.process.responsible_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.process.responsible_audit_token.euid") {
            let converted = convert_value(val, "string")
            .map_err(|message| TransformError::ParseError {
            path: "jamf_protect.telemetry.process.responsible_audit_token.euid".into(),
            message,
            })?;
            event.set("process.responsible.user.id", converted)?;
            }
            }
            if event.has_value("jamf_protect.telemetry.process.responsible_audit_token.e_username") {
            event.rename("jamf_protect.telemetry.process.responsible_audit_token.e_username", "process.responsible.user.name")?;
            }
            if event.has_value("jamf_protect.telemetry.process.responsible_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.process.responsible_audit_token.ruid") {
            let converted = convert_value(val, "string")
            .map_err(|message| TransformError::ParseError {
            path: "jamf_protect.telemetry.process.responsible_audit_token.ruid".into(),
            message,
            })?;
            event.set("process.responsible.real_user.id", converted)?;
            }
            }
            if event.has_value("jamf_protect.telemetry.process.responsible_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.process.responsible_audit_token.rgid") {
            let converted = convert_value(val, "string")
            .map_err(|message| TransformError::ParseError {
            path: "jamf_protect.telemetry.process.responsible_audit_token.rgid".into(),
            message,
            })?;
            event.set("process.responsible.real_group.id", converted)?;
            }
            }
            if event.has_value("jamf_protect.telemetry.process.responsible_audit_token.signing_id") {
            event.rename("jamf_protect.telemetry.process.responsible_audit_token.signing_id", "process.responsible.code_signature.signing_id")?;
            }
            if event.has_value("jamf_protect.telemetry.process.responsible_audit_token.exec_path") {
            event.rename("jamf_protect.telemetry.process.responsible_audit_token.exec_path", "process.responsible.executable")?;
            }
            // End nested pipeline: "pipeline_object_process"

        Ok(TransformResult::Continue)
    }
}
