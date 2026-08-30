// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event_tcc_modify` pipeline.
pub struct PipelineEventTccModify;

impl Transform for PipelineEventTccModify {
    fn name(&self) -> &str {
        "pipeline_event_tcc_modify"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("event.reason", json!("A Transparency Consent and Control (TCC) permission is granted or revoked."))?;

            event.append("event.type", json!("change"))?;

            event.append("event.category", json!("configuration"))?;

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.service") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.service", "jamf_protect.telemetry.tcc_service")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.identity") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.identity", "jamf_protect.telemetry.tcc_identity")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.identity_type") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.identity_type", "jamf_protect.telemetry.tcc_identity_type")?;
            }

            // Painless script
            // Source: if (ctx.jamf_protect?.telemetry?.event?.tcc_modify?.right != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.tcc_modify.right.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.tcc_right = itemTypeString;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(event, cached_painless!(r#"if (ctx.jamf_protect?.telemetry?.event?.tcc_modify?.right != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.tcc_modify.right.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.tcc_right = itemTypeString;\n}\n"#), cached_params!("{\"itemTypeMap\":{\"0\":\"denied\",\"1\":\"unknown\",\"2\":\"allowed\",\"3\":\"limited\",\"4\":\"add_modify_added\",\"5\":\"session_pid\",\"6\":\"learn_more\"}}"))?;

            // Painless script
            // Source: if (ctx.jamf_protect?.telemetry?.event?.tcc_modify?.reason != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.tcc_modify.reason.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.tcc_reason = itemTypeString;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(event, cached_painless!(r#"if (ctx.jamf_protect?.telemetry?.event?.tcc_modify?.reason != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.tcc_modify.reason.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.tcc_reason = itemTypeString;\n}\n"#), cached_params!("{\"itemTypeMap\":{\"0\":\"none\",\"1\":\"error\",\"2\":\"user_consent\",\"3\":\"user_set\",\"4\":\"system_set\",\"5\":\"service_policy\",\"6\":\"mdm_policy\",\"7\":\"service_override_policy\",\"8\":\"missing_usage_string\",\"9\":\"prompt_timeout\",\"10\":\"preflight_unknown\",\"11\":\"entitled\",\"12\":\"app_type_policy\",\"13\":\"prompt_cancel\"}}"))?;

            // Painless script
            // Source: if (ctx.jamf_protect?.telemetry?.event?.tcc_modify?.update_type != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.tcc_modify.update_type.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.tcc_update_type = itemTypeString;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(event, cached_painless!(r#"if (ctx.jamf_protect?.telemetry?.event?.tcc_modify?.update_type != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.tcc_modify.update_type.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.tcc_update_type = itemTypeString;\n}\n"#), cached_params!("{\"itemTypeMap\":{\"0\":\"unknown\",\"1\":\"create\",\"2\":\"modify\",\"3\":\"delete\"}}"))?;

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.start_time") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.start_time", "process.start")?;
            }

        if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.egid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.egid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.egid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.egid", converted)?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.euid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.euid", converted)?;
            }
        }

        let _cond = { event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.euid") };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("user.effective.id", json!(event.get("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.euid").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();
        }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.e_username", "process.user.name")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.is_platform_binary") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.is_platform_binary", "process.platform_binary")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.is_es_client") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.is_es_client", "process.endpoint_security_client")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.cdhash") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.cdhash", "process.hash.cdhash")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.executable.sha1") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.executable.sha1", "process.hash.sha1")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.executable.sha256") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.executable.sha256", "process.hash.sha256")?;
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

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.tty.path") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.tty.path", "jamf_protect.telemetry.tty")?;
            }

        let _cond = { event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.tty") };
        if _cond {
        event.set("process.interactive", json!(true))?;
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.tty") };
        if _cond {
        event.set("process.interactive", json!(false))?;
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.pid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.pid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.pid".into(),
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

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.audit_token.uuid", "process.entity_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.executable.path") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.executable.path", "process.executable")?;
            }

            // Painless script
            // Source: if (ctx.process?.executable != null) {\n    int lastSlashIndex = ctx.process.executable.lastIndexOf('/');\n    if (lastSlashIndex != -1) {\n        ctx.process.name = ctx.process.executable.substring(lastSlashIndex + 1);\n    } else {\n        ctx.process.name = ctx.process.executable; // Fallback if no slash is found\n    }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.process?.executable != null) {\n    int lastSlashIndex = ctx.process.executable.lastIndexOf('/');\n    if (lastSlashIndex != -1) {\n        ctx.process.name = ctx.process.executable.substring(lastSlashIndex + 1);\n    } else {\n        ctx.process.name = ctx.process.executable; // Fallback if no slash is found\n    }\n}\n"#))?;

            if event.has_value("jamf_protect.telemetry.thread.thread_id") {
                event.rename("jamf_protect.telemetry.thread.thread_id", "process.thread.id")?;
            }

        let _cond = { event.has_value("jamf_protect.telemetry.event.tcc_modify.db_path") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.tcc_modify.db_path") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.db_path", "process.working_directory")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.signing_id") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.signing_id", "process.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.team_id") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.team_id", "process.code_signature.team_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.codesigning_flags") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.codesigning_flags", "process.code_signature.flags")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.uuid", "process.parent.entity_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.pid") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.pid", "process.parent.pid")?;
            }

        if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.euid".into(),
                        message,
                    })?;
                event.set("process.parent.user.id", converted)?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.e_username", "process.parent.user.name")?;
            }

        if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.ruid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.ruid".into(),
                        message,
                    })?;
                event.set("process.parent.real_user.id", converted)?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.rgid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.rgid".into(),
                        message,
                    })?;
                event.set("process.parent.real_group.id", converted)?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.signing_id") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.signing_id", "process.parent.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.exec_path") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.parent_audit_token.exec_path", "process.parent.executable")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.uuid", "process.responsible.entity_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.pid") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.pid", "process.responsible.pid")?;
            }

        if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.euid".into(),
                        message,
                    })?;
                event.set("process.responsible.user.id", converted)?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.e_username", "process.responsible.user.name")?;
            }

        if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.ruid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.ruid".into(),
                        message,
                    })?;
                event.set("process.responsible.real_user.id", converted)?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.rgid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.rgid".into(),
                        message,
                    })?;
                event.set("process.responsible.real_group.id", converted)?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.signing_id") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.signing_id", "process.responsible.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.exec_path") {
                event.rename("jamf_protect.telemetry.event.tcc_modify.instigator.responsible_audit_token.exec_path", "process.responsible.executable")?;
            }

        Ok(TransformResult::Continue)
    }
}
