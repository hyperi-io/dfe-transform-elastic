// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event_od_modify_password` pipeline.
pub struct PipelineEventOdModifyPassword;

impl Transform for PipelineEventOdModifyPassword {
    fn name(&self) -> &str {
        "pipeline_event_od_modify_password"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("event.reason", json!("A user password is modified via Open Directory"))?;

            event.append("event.type", json!("change"))?;

            event.append("event.category", json!("configuration"))?;

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.group_name") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.group_name", "group.name")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.account_name") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.account_name", "user.name")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.db_path") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.db_path", "file.path")?;
            }

            // Painless script
            // Source: if (ctx.jamf_protect?.telemetry?.event?.od_modify_password?.account_type != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.od_modify_password.account_type.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.account_type = itemTypeString;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(event, cached_painless!(r#"if (ctx.jamf_protect?.telemetry?.event?.od_modify_password?.account_type != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.od_modify_password.account_type.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.account_type = itemTypeString;\n}\n"#), cached_params!("{\"itemTypeMap\":{\"0\":\"User\",\"1\":\"Computer\"}}"))?;

            // Painless script
            // Source: if (ctx.jamf_protect?.telemetry?.event?.od_modify_password?.error_code != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.od_modify_password.error_code.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.error_message = itemTypeString;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(event, cached_painless!(r#"if (ctx.jamf_protect?.telemetry?.event?.od_modify_password?.error_code != null) {\n    String itemType = ctx.jamf_protect.telemetry.event.od_modify_password.error_code.toString();\n    def itemTypeString = params.itemTypeMap.containsKey(itemType) ? params.itemTypeMap[itemType] : 'Unknown';\n    ctx.jamf_protect = ctx.jamf_protect != null ? ctx.jamf_protect : new HashMap();\n    ctx.jamf_protect.telemetry.error_message = itemTypeString;\n}\n"#), cached_params!("{\"itemTypeMap\":{\"0\":\"kODErrorSuccess\",\"1000\":\"kODErrorSessionLocalOnlyDaemonInUse\",\"1001\":\"kODErrorSessionNormalDaemonInUse\",\"1002\":\"kODErrorSessionDaemonNotRunning\",\"1003\":\"kODErrorSessionDaemonRefused\",\"1100\":\"kODErrorSessionProxyCommunicationError\",\"1101\":\"kODErrorSessionProxyVersionMismatch\",\"1102\":\"kODErrorSessionProxyIPUnreachable\",\"1103\":\"kODErrorSessionProxyUnknownHost\",\"2000\":\"kODErrorNodeUnknownName\",\"2001\":\"kODErrorNodeUnknownType\",\"2002\":\"kODErrorNodeDisabled\",\"2100\":\"kODErrorNodeConnectionFailed\",\"2200\":\"kODErrorNodeUnknownHost\",\"3000\":\"kODErrorQuerySynchronize\",\"3100\":\"kODErrorQueryInvalidMatchType\",\"3101\":\"kODErrorQueryUnsupportedMatchType\",\"3102\":\"kODErrorQueryTimeout\",\"4000\":\"kODErrorRecordReadOnlyNode\",\"4001\":\"kODErrorRecordPermissionError\",\"4100\":\"kODErrorRecordParameterError\",\"4101\":\"kODErrorRecordInvalidType\",\"4102\":\"kODErrorRecordAlreadyExists\",\"4103\":\"kODErrorRecordTypeDisabled\",\"4104\":\"kODErrorRecordNoLongerExists\",\"4200\":\"kODErrorRecordAttributeUnknownType\",\"4201\":\"kODErrorRecordAttributeNotFound\",\"4202\":\"kODErrorRecordAttributeValueSchemaError\",\"4203\":\"kODErrorRecordAttributeValueNotFound\",\"5000\":\"kODErrorCredentialsInvalid\",\"5100\":\"kODErrorCredentialsMethodNotSupported\",\"5101\":\"kODErrorCredentialsNotAuthorized\",\"5102\":\"kODErrorCredentialsParameterError\",\"5103\":\"kODErrorCredentialsOperationFailed\",\"5200\":\"kODErrorCredentialsServerUnreachable\",\"5201\":\"kODErrorCredentialsServerNotFound\",\"5202\":\"kODErrorCredentialsServerError\",\"5203\":\"kODErrorCredentialsServerTimeout\",\"5204\":\"kODErrorCredentialsContactPrimary\",\"5205\":\"kODErrorCredentialsContactPrimary\",\"5206\":\"kODErrorCredentialsServerCommunicationError\",\"5300\":\"kODErrorCredentialsAccountNotFound\",\"5301\":\"kODErrorCredentialsAccountDisabled\",\"5302\":\"kODErrorCredentialsAccountExpired\",\"5303\":\"kODErrorCredentialsAccountInactive\",\"5304\":\"kODErrorCredentialsAccountTemporarilyLocked\",\"5305\":\"kODErrorCredentialsAccountLocked\",\"5400\":\"kODErrorCredentialsPasswordExpired\",\"5401\":\"kODErrorCredentialsPasswordChangeRequired\",\"5402\":\"kODErrorCredentialsPasswordQualityFailed\",\"5403\":\"kODErrorCredentialsPasswordTooShort\",\"5404\":\"kODErrorCredentialsPasswordTooLong\",\"5405\":\"kODErrorCredentialsPasswordNeedsLetter\",\"5406\":\"kODErrorCredentialsPasswordNeedsDigit\",\"5407\":\"kODErrorCredentialsPasswordChangeTooSoon\",\"5408\":\"kODErrorCredentialsPasswordUnrecoverable\",\"5500\":\"kODErrorCredentialsInvalidLogonHours\",\"5501\":\"kODErrorCredentialsInvalidComputer\",\"6000\":\"kODErrorPolicyUnsupported\",\"6001\":\"kODErrorPolicyOutOfRange\",\"10000\":\"kODErrorPluginOperationNotSupported\",\"10001\":\"kODErrorPluginError\",\"10002\":\"kODErrorDaemonError\",\"10003\":\"kODErrorPluginOperationTimeout\"}}"))?;

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.start_time") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.start_time", "process.start")?;
            }

        if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.egid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.egid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.egid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.egid", converted)?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.euid".into(),
                        message,
                    })?;
                event.set("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.euid", converted)?;
            }
        }

        let _cond = { event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.euid") };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("user.effective.id", json!(event.get("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.euid").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();
        }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.e_username", "process.user.name")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.is_platform_binary") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.is_platform_binary", "process.platform_binary")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.is_es_client") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.is_es_client", "process.endpoint_security_client")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.cdhash") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.cdhash", "process.hash.cdhash")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.executable.sha1") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.executable.sha1", "process.hash.sha1")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.executable.sha256") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.executable.sha256", "process.hash.sha256")?;
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

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.tty.path") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.tty.path", "jamf_protect.telemetry.tty")?;
            }

        let _cond = { event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.tty") };
        if _cond {
        event.set("process.interactive", json!(true))?;
        }

        let _cond = { !event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.tty") };
        if _cond {
        event.set("process.interactive", json!(false))?;
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.pid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.pid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.pid".into(),
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

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.audit_token.uuid", "process.entity_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.executable.path") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.executable.path", "process.executable")?;
            }

            // Painless script
            // Source: if (ctx.process?.executable != null) {\n    int lastSlashIndex = ctx.process.executable.lastIndexOf('/');\n    if (lastSlashIndex != -1) {\n        ctx.process.name = ctx.process.executable.substring(lastSlashIndex + 1);\n    } else {\n        ctx.process.name = ctx.process.executable; // Fallback if no slash is found\n    }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.process?.executable != null) {\n    int lastSlashIndex = ctx.process.executable.lastIndexOf('/');\n    if (lastSlashIndex != -1) {\n        ctx.process.name = ctx.process.executable.substring(lastSlashIndex + 1);\n    } else {\n        ctx.process.name = ctx.process.executable; // Fallback if no slash is found\n    }\n}\n"#))?;

            if event.has_value("jamf_protect.telemetry.thread.thread_id") {
                event.rename("jamf_protect.telemetry.thread.thread_id", "process.thread.id")?;
            }

        let _cond = { event.has_value("jamf_protect.telemetry.event.od_modify_password.db_path") };
        if _cond {
            if event.has_value("jamf_protect.telemetry.event.od_modify_password.db_path") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.db_path", "process.working_directory")?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.signing_id") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.signing_id", "process.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.team_id") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.team_id", "process.code_signature.team_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.codesigning_flags") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.codesigning_flags", "process.code_signature.flags")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.uuid", "process.parent.entity_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.pid") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.pid", "process.parent.pid")?;
            }

        if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.euid".into(),
                        message,
                    })?;
                event.set("process.parent.user.id", converted)?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.e_username", "process.parent.user.name")?;
            }

        if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.ruid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.ruid".into(),
                        message,
                    })?;
                event.set("process.parent.real_user.id", converted)?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.rgid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.rgid".into(),
                        message,
                    })?;
                event.set("process.parent.real_group.id", converted)?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.signing_id") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.signing_id", "process.parent.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.exec_path") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.parent_audit_token.exec_path", "process.parent.executable")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.uuid") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.uuid", "process.responsible.entity_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.pid") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.pid", "process.responsible.pid")?;
            }

        if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.euid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.euid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.euid".into(),
                        message,
                    })?;
                event.set("process.responsible.user.id", converted)?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.e_username") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.e_username", "process.responsible.user.name")?;
            }

        if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.ruid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.ruid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.ruid".into(),
                        message,
                    })?;
                event.set("process.responsible.real_user.id", converted)?;
            }
        }

        if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.rgid") {
            if let Some(val) = event.get("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.rgid") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.rgid".into(),
                        message,
                    })?;
                event.set("process.responsible.real_group.id", converted)?;
            }
        }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.signing_id") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.signing_id", "process.responsible.code_signature.signing_id")?;
            }

            if event.has_value("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.exec_path") {
                event.rename("jamf_protect.telemetry.event.od_modify_password.instigator.responsible_audit_token.exec_path", "process.responsible.executable")?;
            }

        Ok(TransformResult::Continue)
    }
}
