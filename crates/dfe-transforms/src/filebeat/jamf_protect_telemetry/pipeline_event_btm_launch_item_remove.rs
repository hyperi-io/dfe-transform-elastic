// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event_btm_launch_item_remove` pipeline.
pub struct PipelineEventBtmLaunchItemRemove;

impl Transform for PipelineEventBtmLaunchItemRemove {
    fn name(&self) -> &str {
        "pipeline_event_btm_launch_item_remove"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("event.reason", json!("Apple’s Background Task Manager notified that an existing item has been removed"))?;

            event.append("event.type", json!("change"))?;

            event.append("event.category", json!("configuration"))?;

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.item.app_url") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.item.app_url") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.item.app_url".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.btm_item_app_url", converted)?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.item.item_url") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.item.item_url") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.item.item_url".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.btm_item_url", converted)?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.item.uid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.item.uid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.item.uid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.btm_item_user_uid", converted)?;
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.item.legacy") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.item.legacy") {
                let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.item.legacy".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.btm_item_is_legacy", converted)?;
            }
        }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.item.managed") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.item.managed") {
                let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.item.managed".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.btm_item_is_managed", converted)?;
            }
        }
            Ok(())
        })();

            // Painless script
            // Source: if (ctx.jamf_protect?.telemetry?.event?.btm_launch_item_remove?.item?.item_type != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.btm_launch_item_remove.item.item_type.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.btm_item_type = itemTypeString;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(event, cached_painless!(r#"if (ctx.jamf_protect?.telemetry?.event?.btm_launch_item_remove?.item?.item_type != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.btm_launch_item_remove.item.item_type.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.btm_item_type = itemTypeString;\n}\n"#), cached_params!("{\"itemTypeMap\":{\"0\":\"User_Item\",\"1\":\"App\",\"2\":\"LoginItem\",\"3\":\"LaunchAgent\",\"4\":\"LaunchDaemon\"}}"))?;

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.start_time") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.start_time", "process.start")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.start_time") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.start_time", "process.start")?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.egid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.egid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.egid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.egid", converted)?;
            }
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.egid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.egid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.egid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.egid", converted)?;
            }
        }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.e_username", "process.user.name")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.e_username", "process.user.name")?;
            }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.euid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.euid", converted)?;
            }
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.euid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.euid", converted)?;
            }
        }
        }

        let _cond = { event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.euid") };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("user.effective.id", json!(event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.euid").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();
        }

        let _cond = { event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.euid") && !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("user.effective.id", json!(event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.euid").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.executable.sha1") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.executable.sha1", "process.hash.sha1")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.executable.sha1") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.executable.sha1", "process.hash.sha1")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.is_platform_binary") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.is_platform_binary", "process.platform_binary")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.is_platform_binary") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.is_platform_binary", "process.platform_binary")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.is_es_client") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.is_es_client", "process.endpoint_security_client")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.is_es_client") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.is_es_client", "process.endpoint_security_client")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.cdhash") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.cdhash", "process.hash.cdhash")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.cdhash") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.cdhash", "process.hash.cdhash")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.executable.sha1") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.executable.sha1", "process.hash.sha1")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.executable.sha1") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.executable.sha1", "process.hash.sha1")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.executable.sha256") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.executable.sha256", "process.hash.sha256")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.executable.sha256") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.executable.sha256", "process.hash.sha256")?;
            }
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

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.tty.path") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.tty.path", "jamf_protect.telemetry.tty")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.tty.path") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.tty.path", "jamf_protect.telemetry.tty")?;
            }
        }

        let _cond = { event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.tty") };
        if _cond {
        event.set("process.interactive", json!(true))?;
        }

        let _cond = { event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.tty") && !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        event.set("process.interactive", json!(true))?;
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.tty") };
        if _cond {
        event.set("process.interactive", json!(false))?;
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.tty") && !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        event.set("process.interactive", json!(false))?;
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.pid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.pid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.pid".into(),
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

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.pid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.pid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.pid".into(),
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
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.audit_token.uuid", "process.entity_id")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.audit_token.uuid", "process.entity_id")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.executable.path") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.executable.path", "process.executable")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.executable.path") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.executable.path", "process.executable")?;
            }
        }

            // Painless script
            // Source: if (ctx.process?.executable != null) {\n    int lastSlashIndex = ctx.process.executable.lastIndexOf('/');\n    if (lastSlashIndex != -1) {\n        ctx.process.name = ctx.process.executable.substring(lastSlashIndex + 1);\n    } else {\n        ctx.process.name = ctx.process.executable; // Fallback if no slash is found\n    }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.process?.executable != null) {\n    int lastSlashIndex = ctx.process.executable.lastIndexOf('/');\n    if (lastSlashIndex != -1) {\n        ctx.process.name = ctx.process.executable.substring(lastSlashIndex + 1);\n    } else {\n        ctx.process.name = ctx.process.executable; // Fallback if no slash is found\n    }\n}\n"#))?;

            if event.has_value("jamf_protect.telemetry.thread.thread_id") {
                event.rename("jamf_protect.telemetry.thread.thread_id", "process.thread.id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.signing_id") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.signing_id", "process.code_signature.signing_id")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.signing_id") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.signing_id", "process.code_signature.signing_id")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.team_id") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.team_id", "process.code_signature.team_id")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.team_id") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.team_id", "process.code_signature.team_id")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.codesigning_flags") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.codesigning_flags", "process.code_signature.flags")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.codesigning_flags") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.codesigning_flags", "process.code_signature.flags")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.uuid", "process.parent.entity_id")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.uuid", "process.parent.entity_id")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.pid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.pid", "process.parent.pid")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.pid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.pid", "process.parent.pid")?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.euid".into(),
                        message,
                    })?;
                event.set("process.parent.user.id", converted)?;
            }
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.euid".into(),
                        message,
                    })?;
                event.set("process.parent.user.id", converted)?;
            }
        }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.e_username", "process.parent.user.name")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.e_username", "process.parent.user.name")?;
            }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.ruid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.ruid".into(),
                        message,
                    })?;
                event.set("process.parent.real_user.id", converted)?;
            }
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.ruid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.ruid".into(),
                        message,
                    })?;
                event.set("process.parent.real_user.id", converted)?;
            }
        }
        }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.rgid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.rgid".into(),
                        message,
                    })?;
                event.set("process.parent.real_group.id", converted)?;
            }
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.rgid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.rgid".into(),
                        message,
                    })?;
                event.set("process.parent.real_group.id", converted)?;
            }
        }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.signing_id") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.signing_id", "process.parent.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.signing_id") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.signing_id", "process.parent.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.exec_path") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.parent_audit_token.exec_path", "process.parent.executable")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.exec_path") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.parent_audit_token.exec_path", "process.parent.executable")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.uuid", "process.responsible.entity_id")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.uuid", "process.responsible.entity_id")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.pid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.pid", "process.responsible.pid")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.pid") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.pid", "process.responsible.pid")?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.euid".into(),
                        message,
                    })?;
                event.set("process.responsible.user.id", converted)?;
            }
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.euid".into(),
                        message,
                    })?;
                event.set("process.responsible.user.id", converted)?;
            }
        }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.e_username", "process.responsible.user.name")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.e_username", "process.responsible.user.name")?;
            }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.ruid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.ruid".into(),
                        message,
                    })?;
                event.set("process.responsible.real_user.id", converted)?;
            }
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.ruid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.ruid".into(),
                        message,
                    })?;
                event.set("process.responsible.real_user.id", converted)?;
            }
        }
        }

        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.rgid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.rgid".into(),
                        message,
                    })?;
                event.set("process.responsible.real_group.id", converted)?;
            }
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
        if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.rgid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.rgid".into(),
                        message,
                    })?;
                event.set("process.responsible.real_group.id", converted)?;
            }
        }
        }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.signing_id") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.signing_id", "process.responsible.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.signing_id") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.signing_id", "process.responsible.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.exec_path") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.app.responsible_audit_token.exec_path", "process.responsible.executable")?;
            }

            if event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.exec_path") {
                event.rename("jamf_protect.telemetry.event.btm_launch_item_remove.instigator.responsible_audit_token.exec_path", "process.responsible.executable")?;
            }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.instigator") && !event.has_value("jamf_protect.telemetry.event.btm_launch_item_remove.app") };
        if _cond {
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
        }

        Ok(TransformResult::Continue)
    }
}
