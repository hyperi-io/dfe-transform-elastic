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

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.append("event.category", json!("iam"))?;

            let _cond = { event.has_value("cef.name") };
            if _cond {
                // Painless script
                // Source: def schemaId = ctx.cef.name.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.category = schema.category;\n  ctx.event.type = schema.type;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def schemaId = ctx.cef.name.toString(); def schema = params[schemaId]; if (schema != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.category = schema.category;\n  ctx.event.type = schema.type;\n}\n"#
                    ),
                    cached_params!(
                        "{\"System Log\":{\"category\":[\"host\"],\"type\":[\"info\"]},\"CONFIGURATION - EDIT\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"USER - LOGIN\":{\"category\":[\"authentication\",\"iam\"],\"type\":[\"user\",\"access\",\"allowed\",\"start\"]},\"USER - LOGOUT\":{\"category\":[\"authentication\",\"iam\"],\"type\":[\"user\",\"access\",\"allowed\",\"end\"]},\"USER - LOGINFAILURE\":{\"category\":[\"authentication\",\"iam\"],\"type\":[\"user\",\"access\",\"denied\"]},\"USER - PASSWORDCHANGE\":{\"category\":[\"authentication\",\"iam\"],\"type\":[\"user\",\"access\",\"allowed\",\"end\"]},\"USER - CREATE\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"USER - DISABLE\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"USER - ENABLE\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"USER – LOCKOUT\":{\"category\":[\"authentication\"],\"type\":[\"user\",\"denied\"]},\"USER - ADDEDTOGROUP\":{\"category\":[\"iam\"],\"type\":[\"user\",\"group\",\"change\"]},\"USER - REMOVEDFROMGROUP\":{\"category\":[\"iam\"],\"type\":[\"user\",\"group\",\"change\"]},\"GROUP - OWNERS_MODIFIED\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"]},\"ROLE - CREATE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"ROLE - ASSIGNUSERORGROUP\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"ROLE - UNASSIGNUSERORGROUP\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"ROLEPERMISSION - ADDEDTOROLE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"ROLEPERMISSION - REMOVEDFROMROLE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"FOLDER - SECRETPOLICYCHANGE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"FOLDER - CREATE\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"FOLDER - DELETE\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"FOLDER - EDITPERMISSIONS\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"SECRET - CREATE\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"SECRET - DELETE\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"SECRET - UNDELETE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"SECRET - VIEW\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - EDIT\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"SECRET - LAUNCH\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - HEARTBEATFAILURE\":{\"category\":[\"iam\"],\"type\":[\"error\"]},\"SECRET - DEPENDENCYFAILURE\":{\"category\":[\"iam\"],\"type\":[\"error\"]},\"SECRET - EXPIREDTODAY\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - EXPIRES1DAY\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - EXPIRES7DAYS\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - EXPIRES15DAYS\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - EXPIRES3DAYS\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - SESSION RECORDING VIEW\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - COPY\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - CHECKIN\":{\"category\":[\"iam\"],\"type\":[\"info\",\"end\"]},\"SECRET - CHECKOUT\":{\"category\":[\"iam\"],\"type\":[\"info\",\"start\"]},\"SECRET - HEARTBEATSUCCESS\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - HOOKFAILURE\":{\"category\":[\"iam\"],\"type\":[\"info\",\"error\"]},\"SECRET - HOOKSUCCESS\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - HOOKCREATE\":{\"category\":[\"iam\"],\"type\":[\"info\",\"creation\"]},\"SECRET - HOOKEDIT\":{\"category\":[\"iam\"],\"type\":[\"info\",\"change\"]},\"SECRET - HOOKDELETE\":{\"category\":[\"iam\"],\"type\":[\"info\",\"deletion\"]},\"SECRET - CUSTOMAUDIT\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - PASSWORD_DISPLAYED\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - PASSWORD_COPIED_TO_CLIPBOARD\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRET - EDIT_VIEW\":{\"category\":[\"iam\"],\"type\":[\"info\",\"change\"]},\"SECRET - ACCESS_APPROVED\":{\"category\":[\"iam\"],\"type\":[\"info\",\"allowed\"]},\"SECRET - ACCESS_DENIED\":{\"category\":[\"iam\"],\"type\":[\"info\",\"denied\"]},\"SECRET - CUSTOM_PASSWORD_REQUIREMENT_ADDED\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"SECRET - CUSTOM_PASSWORD_REQUIREMENT_REMOVED\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"SECRET - DEPENDENCY_DELETED\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"SECRET - DEPENDENCY_ADDED\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"SECRET - SECRETPOLICYCHANGE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"SECRETTEMPLATE - CREATE\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"SECRETTEMPLATE - EDIT\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"SECRETTEMPLATE - TEMPLATE COPIED FROM\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"SECRETTEMPLATE - FIELD ENCRYPTED\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRETTEMPLATE - FIELD EXPOSED\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"SECRETPOLICY - CREATE\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"SECRETPOLICY - EDIT\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"EXPORTSECRETS - EXPORTED\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"IMPORTSECRETS - IMPORTED\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"UNLIMITEDADMIN - ENABLE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"UNLIMITEDADMIN - DISABLE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"USERAUDIT - EXPIRENOW\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"LICENSES - EXPIRES30DAYS\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"POWERSHELLSCRIPT - CREATE\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"POWERSHELLSCRIPT - DEACTIVATE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"POWERSHELLSCRIPT - EDIT\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"POWERSHELLSCRIPT - REACTIVATE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"POWERSHELLSCRIPT - VIEW\":{\"category\":[\"iam\"],\"type\":[\"info\"]}}"
                    ),
                )?;
            }

            if event.has_value("cef.name") {
                if let Some(input) = event.get_string("cef.name") {
                    // Grok pattern: %{DATA:event.provider} - %{GREEDYDATA:event.action}
                    // Grok pattern: %{DATA:event.provider} %{GREEDYDATA:event.action}
                    // Grok pattern: %{GREEDYDATA:event.action}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("%{DATA:event.provider} - %{GREEDYDATA:event.action}"),
                            cached_grok!("%{DATA:event.provider} %{GREEDYDATA:event.action}"),
                            cached_grok!("%{GREEDYDATA:event.action}"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            if event.has_value("event.provider") {
                map_strings(event, "event.provider", "event.provider", str::to_lowercase)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if event.has_value("cef.device.event_class_id") {
                if let Some(val) = event.get("cef.device.event_class_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.device.event_class_id".into(),
                            message,
                        }
                    })?;
                    event.set("event.code", converted)?;
                }
            }

            let _cond =
                { event.has_value("_tmp.pre_cef") && event.get_str("_tmp.pre_cef") != Some("") };
            if _cond {
                if event.has_value("_tmp.pre_cef") {
                    if let Some(input) = event.get_string("_tmp.pre_cef") {
                        // Grok pattern: %{TIME} %{HOSTNAME:log.syslog.hostname}
                        let _ = cached_grok!("%{TIME} %{HOSTNAME:log.syslog.hostname}")
                            .extract_into(&input, event)?;
                    }
                }
            }

            if event.has_value("log.syslog.hostname") {
                if let Some(val) = event.get("log.syslog.hostname") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "log.syslog.hostname".into(),
                            message,
                        }
                    })?;
                    event.set("host.name", converted)?;
                }
            }

            let _cond = { event.has_value("host.name") && event.get_str("host.name") != Some("") };
            if _cond {
                event.append(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("log.source.address") {
                    if let Some(input) = event.get_string("log.source.address") {
                        // Grok pattern: %{IP:_tmp.host.ip:ip}:%{NUMBER}
                        // Grok pattern: %{IP:_tmp.host.ip:ip}
                        let _ = extract_first_match(
                            &[
                                cached_grok!("%{IP:_tmp.host.ip:ip}:%{NUMBER}"),
                                cached_grok!("%{IP:_tmp.host.ip:ip}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                if event.remove("_tmp.host.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_tmp.host.ip".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond =
                { event.has_value("_tmp.host.ip") && event.get_str("_tmp.host.ip") != Some("") };
            if _cond {
                event.append(
                    "host.ip",
                    json!(
                        event
                            .get("_tmp.host.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("_tmp.host.ip") && event.get_str("_tmp.host.ip") != Some("") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("_tmp.host.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.sourceAddress")
                    && event.get_str("cef.extensions.sourceAddress") != Some("")
            };
            if _cond {
                if event.has_value("cef.extensions.sourceAddress") {
                    if let Some(input) = event.get_string("cef.extensions.sourceAddress") {
                        // Grok pattern: %{IP:source.ip}:%{NUMBER:source.port}
                        // Grok pattern: %{IP:source.ip}
                        let _ = extract_first_match(
                            &[
                                cached_grok!("%{IP:source.ip}:%{NUMBER:source.port}"),
                                cached_grok!("%{IP:source.ip}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("log.syslog.hostname") {
                if let Some(val) = event.get("log.syslog.hostname") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "log.syslog.hostname".into(),
                            message,
                        }
                    })?;
                    event.set("observer.hostname", converted)?;
                }
            }

            let _cond =
                { event.has_value("_tmp.host.ip") && event.get_str("_tmp.host.ip") != Some("") };
            if _cond {
                event.append(
                    "observer.ip",
                    json!(
                        event
                            .get("_tmp.host.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("cef.device.product") {
                if let Some(val) = event.get("cef.device.product") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.device.product".into(),
                            message,
                        }
                    })?;
                    event.set("observer.product", converted)?;
                }
            }

            if event.has_value("cef.device.vendor") {
                if let Some(val) = event.get("cef.device.vendor") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.device.vendor".into(),
                            message,
                        }
                    })?;
                    event.set("observer.vendor", converted)?;
                }
            }

            if event.has_value("cef.device.version") {
                if let Some(val) = event.get("cef.device.version") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.device.version".into(),
                            message,
                        }
                    })?;
                    event.set("observer.version", converted)?;
                }
            }

            if event.has_value("cef.extensions.sourceUserId") {
                if let Some(val) = event.get("cef.extensions.sourceUserId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.extensions.sourceUserId".into(),
                            message,
                        }
                    })?;
                    event.set("user.id", converted)?;
                }
            }

            let _cond = {
                event.has_value("cef.extensions.sourceUserName")
                    && event.get_str("cef.extensions.sourceUserName") != Some("")
            };
            if _cond {
                if event.has_value("cef.extensions.sourceUserName") {
                    if let Some(input) = event.get_string("cef.extensions.sourceUserName") {
                        // Grok pattern: %{DATA:_tmp.source_user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.domain}\\\\%{DATA:user.name}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "%{DATA:_tmp.source_user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}"
                                ),
                                cached_grok!("%{DATA:user.name}@%{GREEDYDATA:user.domain}"),
                                cached_grok!("%{DATA:user.domain}\\\\%{DATA:user.name}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.get_str("cef.extensions.deviceCustomString1Label") == Some("Role") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString1") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomString1") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomString1".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.role.name", converted)?;
                    }
                }
            }

            let _cond = {
                event.get_str("cef.extensions.deviceCustomString2Label") == Some("Group or User")
                    && event.get_str("cef.extensions.fileType") == Some("Role")
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString2") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomString2") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomString2".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.role.to", converted)?;
                    }
                }
            }

            let _cond = {
                event.get_str("cef.extensions.deviceCustomString2Label") == Some("Group or User")
                    && event.get_str("cef.extensions.fileType") == Some("User")
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString2") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomString2") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomString2".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.group.name", converted)?;
                    }
                }
            }

            let _cond =
                { event.get_str("cef.extensions.deviceCustomString3Label") == Some("Folder") };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString3") {
                    if let Some(val) = event.get("cef.extensions.deviceCustomString3") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.deviceCustomString3".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.secret.folder", converted)?;
                    }
                }
            }

            let _cond = {
                event.get_str("cef.extensions.deviceCustomString4Label")
                    == Some("suser Display Name")
            };
            if _cond {
                if event.has_value("cef.extensions.deviceCustomString4") {
                    if let Some(input) = event.get_string("cef.extensions.deviceCustomString4") {
                        // Grok pattern: %{DATA:_tmp.cs4_leading_domain}\\\\%{GREEDYDATA:user.full_name}
                        // Grok pattern: %{GREEDYDATA:user.full_name}
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "%{DATA:_tmp.cs4_leading_domain}\\\\%{GREEDYDATA:user.full_name}"
                                ),
                                cached_grok!("%{GREEDYDATA:user.full_name}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = { event.get_str("cef.extensions.fileType") == Some("Secret") };
            if _cond {
                if event.has_value("cef.extensions.filename") {
                    if let Some(val) = event.get("cef.extensions.filename") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.filename".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.secret.name", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("cef.extensions.fileType") == Some("Secret") };
            if _cond {
                if event.has_value("cef.extensions.fileId") {
                    if let Some(val) = event.get("cef.extensions.fileId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.fileId".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.secret.id", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("cef.extensions.fileType") == Some("Folder") };
            if _cond {
                if event.has_value("cef.extensions.filename") {
                    if let Some(val) = event.get("cef.extensions.filename") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.filename".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.folder.name", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("cef.extensions.fileType") == Some("Folder") };
            if _cond {
                if event.has_value("cef.extensions.fileId") {
                    if let Some(val) = event.get("cef.extensions.fileId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.fileId".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.folder.id", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("cef.extensions.fileType") == Some("Role") };
            if _cond {
                if event.has_value("cef.extensions.filename") {
                    if let Some(val) = event.get("cef.extensions.filename") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.filename".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.role.name", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("cef.extensions.fileType") == Some("Role") };
            if _cond {
                if event.has_value("cef.extensions.fileId") {
                    if let Some(val) = event.get("cef.extensions.fileId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.fileId".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.role.id", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("cef.extensions.fileType") == Some("Permission") };
            if _cond {
                if event.has_value("cef.extensions.filename") {
                    if let Some(val) = event.get("cef.extensions.filename") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.filename".into(),
                                message,
                            }
                        })?;
                        event.set("thycotic_ss.event.permission.name", converted)?;
                    }
                }
            }

            if event.has_value("cef.extensions.destinationUserId") {
                if let Some(val) = event.get("cef.extensions.destinationUserId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.extensions.destinationUserId".into(),
                            message,
                        }
                    })?;
                    event.set("thycotic_ss.event.user.id", converted)?;
                }
            }

            let _cond = {
                event.has_value("cef.extensions.destinationUserName")
                    && event.get_str("cef.extensions.destinationUserName") != Some("")
            };
            if _cond {
                if event.has_value("cef.extensions.destinationUserName") {
                    if let Some(input) = event.get_string("cef.extensions.destinationUserName") {
                        // Grok pattern: %{DATA:_tmp.destination_user_leading_domain}\\\\%{DATA:thycotic_ss.event.user.name}@%{GREEDYDATA:thycotic_ss.event.user.domain}
                        // Grok pattern: %{DATA:thycotic_ss.event.user.name}@%{GREEDYDATA:thycotic_ss.event.user.domain}
                        // Grok pattern: %{GREEDYDATA:thycotic_ss.event.user.domain}\\\\%{DATA:thycotic_ss.event.user.name}
                        // Grok pattern: %{GREEDYDATA:thycotic_ss.event.user.name}
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "%{DATA:_tmp.destination_user_leading_domain}\\\\%{DATA:thycotic_ss.event.user.name}@%{GREEDYDATA:thycotic_ss.event.user.domain}"
                                ),
                                cached_grok!(
                                    "%{DATA:thycotic_ss.event.user.name}@%{GREEDYDATA:thycotic_ss.event.user.domain}"
                                ),
                                cached_grok!(
                                    "%{GREEDYDATA:thycotic_ss.event.user.domain}\\\\%{DATA:thycotic_ss.event.user.name}"
                                ),
                                cached_grok!("%{GREEDYDATA:thycotic_ss.event.user.name}"),
                            ],
                            &input,
                            event,
                        )?;
                    }
                }
            }

            let _cond = {
                event.has_value("thycotic_ss.event.user.name")
                    && event.get_str("thycotic_ss.event.user.name") != Some("")
            };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("thycotic_ss.event.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { condition_eq(event.get("user.name"), event.get("thycotic_ss.event.user")) };
            if _cond {
                event.remove("thycotic_ss.event.user");
            }

            let _cond = {
                event.has_value("cef.extensions.deviceReceiptTime")
                    && event.get_str("cef.extensions.deviceReceiptTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cef.extensions.deviceReceiptTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("thycotic_ss.event.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cef.extensions.deviceReceiptTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_deviceReceiptTime")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("cef.extensions.message") {
                if let Some(val) = event.get("cef.extensions.message") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "cef.extensions.message".into(),
                            message,
                        }
                    })?;
                    event.set("message", converted)?;
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("preserve_cef"))
                        }
                        serde_json::Value::String(s) => s.contains("preserve_cef"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("cef");
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("preserve_log"))
                        }
                        serde_json::Value::String(s) => s.contains("preserve_log"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("log");
                    Ok(())
                })();
            }

            event.remove("_tmp");

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
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
