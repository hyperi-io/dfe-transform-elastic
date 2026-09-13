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
            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            parse_json_field(event, "event.original", "cato_networks.audit")?;

            event.append_unique("event.category", json!("configuration"))?;

            event.set("event.kind", json!("event"))?;

            event.append_unique("event.type", json!("info"))?;

            dot_expand(event, "cato_networks.audit", "*")?;

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.adminRoles")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "cato_networks.audit.change.After.adminRoles",
                    "cato_networks.audit.change.After.adminRolesKeyword",
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.allowedItems")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "cato_networks.audit.change.After.allowedItems",
                    "cato_networks.audit.change.After.allowedItemsKeyword",
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.allowedUpdatableItems")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "cato_networks.audit.change.After.allowedUpdatableItems",
                    "cato_networks.audit.change.After.allowedUpdatableItemsKeyword",
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.destType")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "cato_networks.audit.change.After.destType",
                    "cato_networks.audit.change.After.destTypeKeyword",
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.from")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("cato_networks.audit.change.After.from") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cato_networks.audit.change.After.from")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("cato_networks.audit.change.After.fromDate", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cato_networks.audit.change.After.from".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.from")
                    .is_some_and(|v| v.is_string())
                    && !event.has_value("cato_networks.audit.change.After.fromDate")
            };
            if _cond {
                event.rename(
                    "cato_networks.audit.change.After.from",
                    "cato_networks.audit.change.After.fromKeyword",
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.from")
                    .is_some_and(|v| v.is_string())
                    && (event.has_value("cato_networks.audit.change.After.fromDate")
                        || event.has_value("cato_networks.audit.change.After.fromKeyword"))
            };
            if _cond {
                event.remove("cato_networks.audit.change.After.from");
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.permissions")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "cato_networks.audit.change.After.permissions",
                    "cato_networks.audit.change.After.permissionsKeyword",
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.role")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "cato_networks.audit.change.After.role",
                    "cato_networks.audit.change.After.roleKeyword",
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.siteUsage")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "cato_networks.audit.change.After.siteUsage",
                    "cato_networks.audit.change.After.siteUsageKeyword",
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketConnectionConfiguration")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.rename(
                    "cato_networks.audit.change.After.socketConnectionConfiguration",
                    "cato_networks.audit.change.After.socketConnectionConfigurationKeyword",
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketInterfaces")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cato_networks.audit.change.After.socketInterfaces") {
                        if let Some(val) =
                            event.get("cato_networks.audit.change.After.socketInterfaces")
                        {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cato_networks.audit.change.After.socketInterfaces"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cato_networks.audit.change.After.socketInterfacesDouble",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_change_After_socketInterfaces_to_double",
                    )?;
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
                    event.remove("cato_networks.audit.change.After.socketInterfaces");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond =
                { event.has_value("cato_networks.audit.change.After.socketInterfacesDouble") };
            if _cond {
                event.remove("cato_networks.audit.change.After.socketInterfaces");
            }

            if let Some(v) = event
                .get("cato_networks.audit.admin")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("cato_networks.audit.admin_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("cato_networks.audit.admin_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cato_networks.audit.admin_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cato_networks.audit.admin") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cato_networks.audit.admin")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.backhaulingEnabled") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.backhaulingEnabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.backhaulingEnabled".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.backhaulingEnabled",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_backhaulingEnabled_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.backhaulingEnabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.disableAclForSip") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.disableAclForSip")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.disableAclForSip".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.disableAclForSip",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_disableAclForSip_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.disableAclForSip");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.downstreamBandwidth") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.downstreamBandwidth")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.downstreamBandwidth".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.downstreamBandwidth",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_downstreamBandwidth_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.downstreamBandwidth");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("cato_networks.audit.change.After.downstreamBandwidthMbpsPrecision")
                {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.downstreamBandwidthMbpsPrecision")
                    {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.downstreamBandwidthMbpsPrecision".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.downstreamBandwidthMbpsPrecision",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_downstreamBandwidthMbpsPrecision_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.downstreamBandwidthMbpsPrecision");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("cato_networks.audit.change.After.exceededThresholdsDurationEnabled")
                {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.exceededThresholdsDurationEnabled")
                    {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.exceededThresholdsDurationEnabled".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.exceededThresholdsDurationEnabled",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_exceededThresholdsDurationEnabled_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.exceededThresholdsDurationEnabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.expireInDaysAlert") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.expireInDaysAlert")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.expireInDaysAlert".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.expireInDaysAlert",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_expireInDaysAlert_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.expireInDaysAlert");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.forceSelectedLocations") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.forceSelectedLocations")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.forceSelectedLocations"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.forceSelectedLocations",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_forceSelectedLocations_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.forceSelectedLocations");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.isMacAuthOverride") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.isMacAuthOverride")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.isMacAuthOverride".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.isMacAuthOverride",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_isMacAuthOverride_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.isMacAuthOverride");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.isNatPolicyEnabled") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.isNatPolicyEnabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.isNatPolicyEnabled".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.isNatPolicyEnabled",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_isNatPolicyEnabled_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.isNatPolicyEnabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.isPrimary") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.isPrimary") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.isPrimary".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.isPrimary", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_isPrimary_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.isPrimary");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.keepAliveEnabled") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.keepAliveEnabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.keepAliveEnabled".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.keepAliveEnabled",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_keepAliveEnabled_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.keepAliveEnabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.l7LanFwEnforcement") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.l7LanFwEnforcement")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.l7LanFwEnforcement".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.l7LanFwEnforcement",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_l7LanFwEnforcement_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.l7LanFwEnforcement");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.lanFirewallRulesEnabled") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.lanFirewallRulesEnabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.lanFirewallRulesEnabled"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.lanFirewallRulesEnabled",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_lanFirewallRulesEnabled_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.lanFirewallRulesEnabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.latitude") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.latitude") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.latitude".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.latitude", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_latitude_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.latitude");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.longitude") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.longitude") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.longitude".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.longitude", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_longitude_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.longitude");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.maxBandwidth") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.maxBandwidth") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.maxBandwidth".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.maxBandwidth", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_maxBandwidth_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.maxBandwidth");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.model.automaticallyManaged") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.model.automaticallyManaged")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.model.automaticallyManaged"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.model.automaticallyManaged",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_model_automaticallyManaged_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.model.automaticallyManaged");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.naturalOrder") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.naturalOrder") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.naturalOrder".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.naturalOrder", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_naturalOrder_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.naturalOrder");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.offCloudTransportEnabled") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.offCloudTransportEnabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.offCloudTransportEnabled"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.offCloudTransportEnabled",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_offCloudTransportEnabled_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.offCloudTransportEnabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.overridePrimarySettings") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.overridePrimarySettings")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.overridePrimarySettings"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.overridePrimarySettings",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_overridePrimarySettings_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.overridePrimarySettings");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.physicalPort") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.physicalPort") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.physicalPort".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.physicalPort", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_physicalPort_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.physicalPort");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.preferHigherNaturalOrder") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.preferHigherNaturalOrder")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.preferHigherNaturalOrder"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.preferHigherNaturalOrder",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_preferHigherNaturalOrder_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.preferHigherNaturalOrder");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.preferPrimaryWAN") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.preferPrimaryWAN")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.preferPrimaryWAN".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.preferPrimaryWAN",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_preferPrimaryWAN_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.preferPrimaryWAN");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.primaryBWDownstream") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.primaryBWDownstream")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.primaryBWDownstream".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.primary_bw_downstream",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_primaryBWDownstream_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.primaryBWDownstream");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("cato_networks.audit.change.After.primaryBWDownstreamMbpsPrecision")
                {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.primaryBWDownstreamMbpsPrecision")
                    {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.primaryBWDownstreamMbpsPrecision".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.primary_bw_downstream_mbps_precision",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_primaryBWDownstreamMbpsPrecision_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.primaryBWDownstreamMbpsPrecision");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.primaryBWUpstream") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.primaryBWUpstream")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.primaryBWUpstream".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.primary_bw_upstream",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_primaryBWUpstream_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.primaryBWUpstream");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("cato_networks.audit.change.After.primaryBWUpstreamMbpsPrecision")
                {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.primaryBWUpstreamMbpsPrecision")
                    {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.primaryBWUpstreamMbpsPrecision".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.primary_bw_upstream_mbps_precision",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_primaryBWUpstreamMbpsPrecision_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.primaryBWUpstreamMbpsPrecision");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.publish") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.publish") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.publish".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.publish", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_publish_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.publish");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.pwK") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.pwK") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.pwK".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.pwk", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_pwK_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.pwK");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.regionalBandwidth") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.regionalBandwidth")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.regionalBandwidth".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.regionalBandwidth",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_regionalBandwidth_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.regionalBandwidth");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("cato_networks.audit.change.After.role.creationDateMilliseconds")
                    && event
                        .get_str("cato_networks.audit.change.After.role.creationDateMilliseconds")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "cato_networks.audit.change.After.role.creationDateMilliseconds",
                    ) {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "cato_networks.audit.change.After.role.creationDateMilliseconds",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "cato_networks.audit.change.After.role.creationDateMilliseconds".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_role_creationDateMilliseconds",
                    )?;
                    event.remove("cato_networks.audit.change.After.role.creationDateMilliseconds");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.role.isForToken") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.role.isForToken")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.role.isForToken".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.role.isForToken",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_role_isForToken_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.role.isForToken");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.role.isPredefined") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.role.isPredefined")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.role.isPredefined".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.role.isPredefined",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_role_isPredefined_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.role.isPredefined");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.role.isUsedOnExternalAccess") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.role.isUsedOnExternalAccess")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "cato_networks.audit.change.After.role.isUsedOnExternalAccess"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.role.isUsedOnExternalAccess",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_role_isUsedOnExternalAccess_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.role.isUsedOnExternalAccess");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.role.newSaveMethod") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.role.newSaveMethod")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.role.newSaveMethod".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.role.newSaveMethod",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_role_newSaveMethod_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.role.newSaveMethod");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.role.visible") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.role.visible") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.role.visible".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.role.visible", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_role_visible_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.role.visible");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.s2sEnabled") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.s2sEnabled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.s2sEnabled".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.s2sEnabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_s2sEnabled_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.s2sEnabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.secPwK") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.secPwK") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.secPwK".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.sec_pwk", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_secPwK_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.secPwK");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.secondaryBWDownstream") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.secondaryBWDownstream")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.secondaryBWDownstream"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.secondary_bw_downstream",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_secondaryBWDownstream_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.secondaryBWDownstream");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cato_networks.audit.change.After.secondaryBWDownstreamMbpsPrecision",
                ) {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.secondaryBWDownstreamMbpsPrecision")
                    {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.secondaryBWDownstreamMbpsPrecision".into(),
                            message,
                        })?;
                        event.set("cato_networks.audit.change.After.secondary_bw_downstream_mbps_precision", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_secondaryBWDownstreamMbpsPrecision_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.secondaryBWDownstreamMbpsPrecision");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.secondaryBWUpstream") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.secondaryBWUpstream")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.secondaryBWUpstream".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.secondary_bw_upstream",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_secondaryBWUpstream_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.secondaryBWUpstream");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("cato_networks.audit.change.After.secondaryBWUpstreamMbpsPrecision")
                {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.secondaryBWUpstreamMbpsPrecision")
                    {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.secondaryBWUpstreamMbpsPrecision".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.secondary_bw_upstream_mbps_precision",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_secondaryBWUpstreamMbpsPrecision_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.secondaryBWUpstreamMbpsPrecision");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.remove("cato_networks.audit.change.After.primaryBWDownstream");
            event.remove("cato_networks.audit.change.After.primaryBWDownstreamMbpsPrecision");
            event.remove("cato_networks.audit.change.After.primaryBWUpstream");
            event.remove("cato_networks.audit.change.After.primaryBWUpstreamMbpsPrecision");
            event.remove("cato_networks.audit.change.After.pwK");
            event.remove("cato_networks.audit.change.After.secPwK");
            event.remove("cato_networks.audit.change.After.secondaryBWDownstream");
            event.remove("cato_networks.audit.change.After.secondaryBWDownstreamMbpsPrecision");
            event.remove("cato_networks.audit.change.After.secondaryBWUpstream");
            event.remove("cato_networks.audit.change.After.secondaryBWUpstreamMbpsPrecision");

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.singleWanQosPolicy") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.singleWanQosPolicy")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.singleWanQosPolicy".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.singleWanQosPolicy",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_singleWanQosPolicy_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.singleWanQosPolicy");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.siteConnType.requirePassword")
                {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.siteConnType.requirePassword")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "cato_networks.audit.change.After.siteConnType.requirePassword"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteConnType.requirePassword",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteConnType_requirePassword_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.siteConnType.requirePassword");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cato_networks.audit.change.After.siteConnType.requirePrimaryBandwidth",
                ) {
                    if let Some(val) = event.get(
                        "cato_networks.audit.change.After.siteConnType.requirePrimaryBandwidth",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.siteConnType.requirePrimaryBandwidth".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteConnType.requirePrimaryBandwidth",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteConnType_requirePrimaryBandwidth_to_boolean",
                )?;
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
                event.remove(
                    "cato_networks.audit.change.After.siteConnType.requirePrimaryBandwidth",
                );
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.automaticallyManaged") {
                if let Some(val) = event.get("cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.automaticallyManaged") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.automaticallyManaged".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.automaticallyManaged", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_siteConnType_socketModelTypeConfiguration_automaticallyManaged_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.automaticallyManaged");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.siteConnType.supportBGP") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.siteConnType.supportBGP")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.siteConnType.supportBGP"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteConnType.supportBGP",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteConnType_supportBGP_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.siteConnType.supportBGP");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.siteConnType.supportKeepAlive")
                {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.siteConnType.supportKeepAlive")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "cato_networks.audit.change.After.siteConnType.supportKeepAlive"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteConnType.supportKeepAlive",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteConnType_supportKeepAlive_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.siteConnType.supportKeepAlive");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cato_networks.audit.change.After.siteConnType.supportMultiTunnelMode",
                ) {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.siteConnType.supportMultiTunnelMode")
                    {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.siteConnType.supportMultiTunnelMode".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteConnType.supportMultiTunnelMode",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteConnType_supportMultiTunnelMode_to_boolean",
                )?;
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
                event
                    .remove("cato_networks.audit.change.After.siteConnType.supportMultiTunnelMode");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("cato_networks.audit.change.After.siteConnType.supportSiteRecovery")
                {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.siteConnType.supportSiteRecovery")
                    {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.siteConnType.supportSiteRecovery".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteConnType.supportSiteRecovery",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteConnType_supportSiteRecovery_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.siteConnType.supportSiteRecovery");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value(
                    "cato_networks.audit.change.After.siteProfile.creationDateMilliseconds",
                ) && event.get_str(
                    "cato_networks.audit.change.After.siteProfile.creationDateMilliseconds",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "cato_networks.audit.change.After.siteProfile.creationDateMilliseconds",
                    ) {
                        match parse_date_out(&date_str, &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"], None, None) {
                        Some(parsed) => event.set("cato_networks.audit.change.After.siteProfile.creationDateMilliseconds", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cato_networks.audit.change.After.siteProfile.creationDateMilliseconds".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_siteProfile_creationDateMilliseconds",
                    )?;
                    event.remove(
                        "cato_networks.audit.change.After.siteProfile.creationDateMilliseconds",
                    );
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.siteProfile.newSaveMethod") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.siteProfile.newSaveMethod")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.siteProfile.newSaveMethod"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteProfile.newSaveMethod",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteProfile_newSaveMethod_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.siteProfile.newSaveMethod");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value(
                    "cato_networks.audit.change.After.siteUsage.creationDateMilliseconds",
                ) && event
                    .get_str("cato_networks.audit.change.After.siteUsage.creationDateMilliseconds")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "cato_networks.audit.change.After.siteUsage.creationDateMilliseconds",
                    ) {
                        match parse_date_out(&date_str, &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"], None, None) {
                        Some(parsed) => event.set("cato_networks.audit.change.After.siteUsage.creationDateMilliseconds", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cato_networks.audit.change.After.siteUsage.creationDateMilliseconds".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_siteUsage_creationDateMilliseconds",
                    )?;
                    event.remove(
                        "cato_networks.audit.change.After.siteUsage.creationDateMilliseconds",
                    );
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.siteUsage.maxBandwidth") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.siteUsage.maxBandwidth")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.siteUsage.maxBandwidth"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteUsage.maxBandwidth",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteUsage_maxBandwidth_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.siteUsage.maxBandwidth");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.siteUsage.newSaveMethod") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.siteUsage.newSaveMethod")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.siteUsage.newSaveMethod"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteUsage.newSaveMethod",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteUsage_newSaveMethod_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.siteUsage.newSaveMethod");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.siteUsage.regionalBandwidth") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.siteUsage.regionalBandwidth")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "cato_networks.audit.change.After.siteUsage.regionalBandwidth"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.siteUsage.regionalBandwidth",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_siteUsage_regionalBandwidth_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.siteUsage.regionalBandwidth");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // Painless script
            // Source: def removeFractional(def value) {\n  if (value instanceof Number) {\n    return ((Number)value).longValue();\n  }\n  if (value == '') {\n    return value;\n  }\n  return (long)Double.parseDouble(String.valueOf(value));\n}\nif (ctx.cato_networks?.audit?.change?.After?.startDate != null) {\n  ctx.cato_networks.audit.change.After.startDate = removeFractional(ctx.cato_networks.audit.change.After.startDate);\n}\nif (ctx.cato_networks?.audit?.change?.After?.siteUsage?.startDate != null) {\n  ctx.cato_networks.audit.change.After.siteUsage.startDate = removeFractional(ctx.cato_networks.audit.change.After.siteUsage.startDate);\n}\nif (ctx.cato_networks?.audit?.change?.After?.socketInterface?.lastModified != null) {\n  ctx.cato_networks.audit.change.After.socketInterface.lastModified = removeFractional(ctx.cato_networks.audit.change.After.socketInterface.lastModified);\n}\nif (ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.lastModified != null) {\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.lastModified = removeFractional(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.lastModified);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def removeFractional(def value) {\n  if (value instanceof Number) {\n    return ((Number)value).longValue();\n  }\n  if (value == '') {\n    return value;\n  }\n  return (long)Double.parseDouble(String.valueOf(value));\n}\nif (ctx.cato_networks?.audit?.change?.After?.startDate != null) {\n  ctx.cato_networks.audit.change.After.startDate = removeFractional(ctx.cato_networks.audit.change.After.startDate);\n}\nif (ctx.cato_networks?.audit?.change?.After?.siteUsage?.startDate != null) {\n  ctx.cato_networks.audit.change.After.siteUsage.startDate = removeFractional(ctx.cato_networks.audit.change.After.siteUsage.startDate);\n}\nif (ctx.cato_networks?.audit?.change?.After?.socketInterface?.lastModified != null) {\n  ctx.cato_networks.audit.change.After.socketInterface.lastModified = removeFractional(ctx.cato_networks.audit.change.After.socketInterface.lastModified);\n}\nif (ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.lastModified != null) {\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.lastModified = removeFractional(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.lastModified);\n}\n"#
                ),
            )?;

            let _cond = {
                event.has_value("cato_networks.audit.change.After.siteUsage.startDate")
                    && event.get_str("cato_networks.audit.change.After.siteUsage.startDate")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cato_networks.audit.change.After.siteUsage.startDate")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "cato_networks.audit.change.After.siteUsage.startDate",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cato_networks.audit.change.After.siteUsage.startDate"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_siteUsage_startDate",
                    )?;
                    event.remove("cato_networks.audit.change.After.siteUsage.startDate");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("cato_networks.audit.change.After.socketsSettings.primary.isPrimary")
                {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.socketsSettings.primary.isPrimary")
                    {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketsSettings.primary.isPrimary".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.socketsSettings.primary.isPrimary",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketsSettings_primary_isPrimary_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.socketsSettings.primary.isPrimary");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // Painless script
            // Source: def convert_to_array(def o){\n  if (o instanceof Map) {\n    def output = [];\n    for (entry in o.entrySet()) {\n      output.add(entry.getValue());\n    }\n    return output;\n  }\n  return o;\n}\nif(ctx.cato_networks?.audit?.change?.After?.adminRoles != null){\n  ctx.cato_networks.audit.change.After.adminRoles = convert_to_array(ctx.cato_networks.audit.change.After.adminRoles);\n}\nif(ctx.cato_networks?.audit?.change?.After?.allowedItems != null){\n  ctx.cato_networks.audit.change.After.allowedItems = convert_to_array(ctx.cato_networks.audit.change.After.allowedItems);\n}\nif(ctx.cato_networks?.audit?.change?.After?.allowedUpdatableItems != null){\n  ctx.cato_networks.audit.change.After.allowedUpdatableItems = convert_to_array(ctx.cato_networks.audit.change.After.allowedUpdatableItems);\n}\nif(ctx.cato_networks?.audit?.change?.After?.from != null){\n  ctx.cato_networks.audit.change.After.from = convert_to_array(ctx.cato_networks.audit.change.After.from);\n}\nif(ctx.cato_networks?.audit?.change?.After?.model?.links != null){\n  ctx.cato_networks.audit.change.After.model.links = convert_to_array(ctx.cato_networks.audit.change.After.model.links);\n}\nif(ctx.cato_networks?.audit?.change?.After?.model?.possiblePlatform != null){\n  ctx.cato_networks.audit.change.After.model.possiblePlatform = convert_to_array(ctx.cato_networks.audit.change.After.model.possiblePlatform);\n}\n if(ctx.cato_networks?.audit?.change?.After?.socketInterface?.metadata?.supportedDestinations != null){\n  ctx.cato_networks.audit.change.After.socketInterface.metadata.supportedDestinations = convert_to_array(ctx.cato_networks.audit.change.After.socketInterface.metadata.supportedDestinations);\n}\nif(ctx.cato_networks?.audit?.change?.After?.siteConnType?.connectionTypeFamilies != null){\n  ctx.cato_networks.audit.change.After.siteConnType.connectionTypeFamilies = convert_to_array(ctx.cato_networks.audit.change.After.siteConnType.connectionTypeFamilies);\n}\nif(ctx.cato_networks?.audit?.change?.After?.siteConnType?.socketModelTypeConfiguration?.links != null){\n  ctx.cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.links = convert_to_array(ctx.cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.links);\n}\nif(ctx.cato_networks?.audit?.change?.After?.siteConnType?.socketModelTypeConfiguration?.possiblePlatform != null){\n  ctx.cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.possiblePlatform = convert_to_array(ctx.cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.possiblePlatform);\n}\n\n// permissions.* converted to array\nif(ctx.cato_networks?.audit?.change?.After?.permissions != null){\n  ctx.cato_networks.audit.change.After.permissions = convert_to_array(ctx.cato_networks.audit.change.After.permissions);\n  for(o in ctx.cato_networks.audit.change.After.permissions){\n    if(o.actions != null){\n      o.actions = convert_to_array(o.actions);\n    }\n  }\n}\n\n// socketsSettings.* converted to array\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.links != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.links = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.links);\n  for(o in ctx.cato_networks.audit.change.After.socketsSettings.primary.links){\n    if(o.metadata?.supportedDestinations != null){\n      o.metadata.supportedDestinations = convert_to_array(o.metadata.supportedDestinations);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.supportedAddOns != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedAddOns = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedAddOns);\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.supportedMigrations != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations);\n  for(o in ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations){\n    if(o.links != null){\n      o.links = convert_to_array(o.links);\n    }\n    if(o.possibleAddOns != null){\n      o.possibleAddOns = convert_to_array(o.possibleAddOns);\n    }\n    if(o.possiblePlatform != null){\n      o.possiblePlatform = convert_to_array(o.possiblePlatform);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.type?.links != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.type.links = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.type.links);\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.type?.possiblePlatform != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.type.possiblePlatform = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.type.possiblePlatform);\n}\n\n// socketConnectionConfiguration.* converted to array\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketInterfaces != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces);\n  for(o in ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces){\n    if(o.metadata?.supportedDestinations != null){\n      o.metadata.supportedDestinations = convert_to_array(o.metadata.supportedDestinations);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.links != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links);\n  for(o in ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links){\n    if(o.metadata?.supportedDestinations != null){\n      o.metadata.supportedDestinations = convert_to_array(o.metadata.supportedDestinations);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.supportedAddOns != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedAddOns = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedAddOns);\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.supportedMigrations != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations);\n  for(o in ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations){\n    if(o.links != null){\n      o.links = convert_to_array(o.links);\n    }\n    if(o.possibleAddOns != null){\n      o.possibleAddOns = convert_to_array(o.possibleAddOns);\n    }\n    if(o.possiblePlatform != null){\n      o.possiblePlatform = convert_to_array(o.possiblePlatform);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.type?.links != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.links = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.links);\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.type?.possiblePlatform != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.possiblePlatform = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.possiblePlatform);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def convert_to_array(def o){\n  if (o instanceof Map) {\n    def output = [];\n    for (entry in o.entrySet()) {\n      output.add(entry.getValue());\n    }\n    return output;\n  }\n  return o;\n}\nif(ctx.cato_networks?.audit?.change?.After?.adminRoles != null){\n  ctx.cato_networks.audit.change.After.adminRoles = convert_to_array(ctx.cato_networks.audit.change.After.adminRoles);\n}\nif(ctx.cato_networks?.audit?.change?.After?.allowedItems != null){\n  ctx.cato_networks.audit.change.After.allowedItems = convert_to_array(ctx.cato_networks.audit.change.After.allowedItems);\n}\nif(ctx.cato_networks?.audit?.change?.After?.allowedUpdatableItems != null){\n  ctx.cato_networks.audit.change.After.allowedUpdatableItems = convert_to_array(ctx.cato_networks.audit.change.After.allowedUpdatableItems);\n}\nif(ctx.cato_networks?.audit?.change?.After?.from != null){\n  ctx.cato_networks.audit.change.After.from = convert_to_array(ctx.cato_networks.audit.change.After.from);\n}\nif(ctx.cato_networks?.audit?.change?.After?.model?.links != null){\n  ctx.cato_networks.audit.change.After.model.links = convert_to_array(ctx.cato_networks.audit.change.After.model.links);\n}\nif(ctx.cato_networks?.audit?.change?.After?.model?.possiblePlatform != null){\n  ctx.cato_networks.audit.change.After.model.possiblePlatform = convert_to_array(ctx.cato_networks.audit.change.After.model.possiblePlatform);\n}\n if(ctx.cato_networks?.audit?.change?.After?.socketInterface?.metadata?.supportedDestinations != null){\n  ctx.cato_networks.audit.change.After.socketInterface.metadata.supportedDestinations = convert_to_array(ctx.cato_networks.audit.change.After.socketInterface.metadata.supportedDestinations);\n}\nif(ctx.cato_networks?.audit?.change?.After?.siteConnType?.connectionTypeFamilies != null){\n  ctx.cato_networks.audit.change.After.siteConnType.connectionTypeFamilies = convert_to_array(ctx.cato_networks.audit.change.After.siteConnType.connectionTypeFamilies);\n}\nif(ctx.cato_networks?.audit?.change?.After?.siteConnType?.socketModelTypeConfiguration?.links != null){\n  ctx.cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.links = convert_to_array(ctx.cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.links);\n}\nif(ctx.cato_networks?.audit?.change?.After?.siteConnType?.socketModelTypeConfiguration?.possiblePlatform != null){\n  ctx.cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.possiblePlatform = convert_to_array(ctx.cato_networks.audit.change.After.siteConnType.socketModelTypeConfiguration.possiblePlatform);\n}\n\n// permissions.* converted to array\nif(ctx.cato_networks?.audit?.change?.After?.permissions != null){\n  ctx.cato_networks.audit.change.After.permissions = convert_to_array(ctx.cato_networks.audit.change.After.permissions);\n  for(o in ctx.cato_networks.audit.change.After.permissions){\n    if(o.actions != null){\n      o.actions = convert_to_array(o.actions);\n    }\n  }\n}\n\n// socketsSettings.* converted to array\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.links != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.links = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.links);\n  for(o in ctx.cato_networks.audit.change.After.socketsSettings.primary.links){\n    if(o.metadata?.supportedDestinations != null){\n      o.metadata.supportedDestinations = convert_to_array(o.metadata.supportedDestinations);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.supportedAddOns != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedAddOns = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedAddOns);\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.supportedMigrations != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations);\n  for(o in ctx.cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations){\n    if(o.links != null){\n      o.links = convert_to_array(o.links);\n    }\n    if(o.possibleAddOns != null){\n      o.possibleAddOns = convert_to_array(o.possibleAddOns);\n    }\n    if(o.possiblePlatform != null){\n      o.possiblePlatform = convert_to_array(o.possiblePlatform);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.type?.links != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.type.links = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.type.links);\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.type?.possiblePlatform != null){\n  ctx.cato_networks.audit.change.After.socketsSettings.primary.type.possiblePlatform = convert_to_array(ctx.cato_networks.audit.change.After.socketsSettings.primary.type.possiblePlatform);\n}\n\n// socketConnectionConfiguration.* converted to array\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketInterfaces != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces);\n  for(o in ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces){\n    if(o.metadata?.supportedDestinations != null){\n      o.metadata.supportedDestinations = convert_to_array(o.metadata.supportedDestinations);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.links != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links);\n  for(o in ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links){\n    if(o.metadata?.supportedDestinations != null){\n      o.metadata.supportedDestinations = convert_to_array(o.metadata.supportedDestinations);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.supportedAddOns != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedAddOns = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedAddOns);\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.supportedMigrations != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations);\n  for(o in ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations){\n    if(o.links != null){\n      o.links = convert_to_array(o.links);\n    }\n    if(o.possibleAddOns != null){\n      o.possibleAddOns = convert_to_array(o.possibleAddOns);\n    }\n    if(o.possiblePlatform != null){\n      o.possiblePlatform = convert_to_array(o.possiblePlatform);\n    }\n  }\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.type?.links != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.links = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.links);\n}\nif(ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketsSettings?.primary?.type?.possiblePlatform != null){\n  ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.possiblePlatform = convert_to_array(ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.possiblePlatform);\n}"#
                ),
            )?;

            // Painless script
            // Source: def removeFractional(def value) {\n  if (value instanceof Number) {\n    return ((Number)value).longValue();\n  }\n  if (value == '') {\n    return value;\n  }\n  return (long)Double.parseDouble(String.valueOf(value));\n}\nif (ctx.cato_networks?.audit?.change?.After?.allowedUpdatableItems instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.allowedUpdatableItems){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.permissions instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.permissions){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketInterfaces instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces){\n    if(item?.lastModified != null){\n      item.lastModified = removeFractional(item.lastModified);\n    }\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.links instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.socketsSettings.primary.links){\n    if(item?.lastModified != null){\n      item.lastModified = removeFractional(item.lastModified);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.adminRoles instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.adminRoles){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n    if(item?.role?.creationDateMilliseconds != null){\n      item.role.creationDateMilliseconds = removeFractional(item.role.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.allowedItems instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.allowedItems){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.from instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.from){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def removeFractional(def value) {\n  if (value instanceof Number) {\n    return ((Number)value).longValue();\n  }\n  if (value == '') {\n    return value;\n  }\n  return (long)Double.parseDouble(String.valueOf(value));\n}\nif (ctx.cato_networks?.audit?.change?.After?.allowedUpdatableItems instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.allowedUpdatableItems){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.permissions instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.permissions){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.socketConnectionConfiguration?.socketInterfaces instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces){\n    if(item?.lastModified != null){\n      item.lastModified = removeFractional(item.lastModified);\n    }\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.socketsSettings?.primary?.links instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.socketsSettings.primary.links){\n    if(item?.lastModified != null){\n      item.lastModified = removeFractional(item.lastModified);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.adminRoles instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.adminRoles){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n    if(item?.role?.creationDateMilliseconds != null){\n      item.role.creationDateMilliseconds = removeFractional(item.role.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.allowedItems instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.allowedItems){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\nif (ctx.cato_networks?.audit?.change?.After?.from instanceof List) {\n  for(def item : ctx.cato_networks.audit.change.After.from){\n    if(item?.creationDateMilliseconds != null){\n      item.creationDateMilliseconds = removeFractional(item.creationDateMilliseconds);\n    }\n  }\n}\n"#
                ),
            )?;

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketsSettings.primary.overridePrimarySettings") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketsSettings.primary.overridePrimarySettings") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketsSettings.primary.overridePrimarySettings".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketsSettings.primary.overridePrimarySettings", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_overridePrimarySettings_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.socketsSettings.primary.overridePrimarySettings");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketsSettings.primary.type.automaticallyManaged") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketsSettings.primary.type.automaticallyManaged") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketsSettings.primary.type.automaticallyManaged".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketsSettings.primary.type.automaticallyManaged", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_type_automaticallyManaged_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.socketsSettings.primary.type.automaticallyManaged");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cato_networks.audit.change.After.socketsSettings.primary.upgradesPaused",
                ) {
                    if let Some(val) = event.get(
                        "cato_networks.audit.change.After.socketsSettings.primary.upgradesPaused",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketsSettings.primary.upgradesPaused".into(),
                            message,
                        })?;
                        event.set("cato_networks.audit.change.After.socketsSettings.primary.upgradesPaused", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketsSettings_primary_upgradesPaused_to_boolean",
                )?;
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
                event.remove(
                    "cato_networks.audit.change.After.socketsSettings.primary.upgradesPaused",
                );
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value(
                    "cato_networks.audit.change.After.socketConnectionConfiguration.creationDate",
                ) && event.get_str(
                    "cato_networks.audit.change.After.socketConnectionConfiguration.creationDate",
                ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cato_networks.audit.change.After.socketConnectionConfiguration.creationDate") {
                    match parse_date_out(&date_str, &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"], None, None) {
                        Some(parsed) => event.set("cato_networks.audit.change.After.socketConnectionConfiguration.creationDate", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cato_networks.audit.change.After.socketConnectionConfiguration.creationDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_socketConnectionConfiguration_creationDate",
                    )?;
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
                    event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.creationDate");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("cato_networks.audit.change.After.socketConnectionConfiguration.creationDateMilliseconds") && event.get_str("cato_networks.audit.change.After.socketConnectionConfiguration.creationDateMilliseconds") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cato_networks.audit.change.After.socketConnectionConfiguration.creationDateMilliseconds") {
                    match parse_date_out(&date_str, &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"], None, None) {
                        Some(parsed) => event.set("cato_networks.audit.change.After.socketConnectionConfiguration.creationDateMilliseconds", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cato_networks.audit.change.After.socketConnectionConfiguration.creationDateMilliseconds".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_socketConnectionConfiguration_creationDateMilliseconds",
                    )?;
                    event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.creationDateMilliseconds");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketConnectionConfiguration.isMacAuthOverride") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.isMacAuthOverride") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketConnectionConfiguration.isMacAuthOverride".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketConnectionConfiguration.isMacAuthOverride", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_isMacAuthOverride_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.isMacAuthOverride");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value(
                    "cato_networks.audit.change.After.socketConnectionConfiguration.lastModified",
                ) && event.get_str(
                    "cato_networks.audit.change.After.socketConnectionConfiguration.lastModified",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cato_networks.audit.change.After.socketConnectionConfiguration.lastModified") {
                    match parse_date_out(&date_str, &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"], None, None) {
                        Some(parsed) => event.set("cato_networks.audit.change.After.socketConnectionConfiguration.lastModified", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cato_networks.audit.change.After.socketConnectionConfiguration.lastModified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_socketConnectionConfiguration_lastModified",
                    )?;
                    event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.lastModified");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketConnectionConfiguration.macAddressAuthenticationEnabledOnAccount") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.macAddressAuthenticationEnabledOnAccount") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketConnectionConfiguration.macAddressAuthenticationEnabledOnAccount".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketConnectionConfiguration.macAddressAuthenticationEnabledOnAccount", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_macAddressAuthenticationEnabledOnAccount_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.macAddressAuthenticationEnabledOnAccount");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cato_networks.audit.change.After.socketConnectionConfiguration.newSaveMethod",
                ) {
                    if let Some(val) = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.newSaveMethod") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketConnectionConfiguration.newSaveMethod".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketConnectionConfiguration.newSaveMethod", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketConnectionConfiguration_newSaveMethod_to_boolean",
                )?;
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
                event.remove(
                    "cato_networks.audit.change.After.socketConnectionConfiguration.newSaveMethod",
                );
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketConnectionConfiguration.offCloudTransportEnabled") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.offCloudTransportEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketConnectionConfiguration.offCloudTransportEnabled".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketConnectionConfiguration.offCloudTransportEnabled", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_offCloudTransportEnabled_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.offCloudTransportEnabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketConnectionConfiguration.preferHigherNaturalOrder") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.preferHigherNaturalOrder") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketConnectionConfiguration.preferHigherNaturalOrder".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketConnectionConfiguration.preferHigherNaturalOrder", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_preferHigherNaturalOrder_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.preferHigherNaturalOrder");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.isPrimary") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.isPrimary") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.isPrimary".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.isPrimary", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_isPrimary_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.isPrimary");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.automaticallyManaged") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.automaticallyManaged") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.automaticallyManaged".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.automaticallyManaged", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_type_automaticallyManaged_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.type.automaticallyManaged");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.upgradesPaused") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.upgradesPaused") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.upgradesPaused".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.upgradesPaused", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_upgradesPaused_to_boolean")?;
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
                event.remove("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.upgradesPaused");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("cato_networks.audit.change.After.socketInterface.creationDate")
                    && event
                        .get_str("cato_networks.audit.change.After.socketInterface.creationDate")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "cato_networks.audit.change.After.socketInterface.creationDate",
                    ) {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "cato_networks.audit.change.After.socketInterface.creationDate",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "cato_networks.audit.change.After.socketInterface.creationDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_socketInterface_creationDate",
                    )?;
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
                    event.remove("cato_networks.audit.change.After.socketInterface.creationDate");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value(
                    "cato_networks.audit.change.After.socketInterface.creationDateMilliseconds",
                ) && event.get_str(
                    "cato_networks.audit.change.After.socketInterface.creationDateMilliseconds",
                ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "cato_networks.audit.change.After.socketInterface.creationDateMilliseconds",
                    ) {
                        match parse_date_out(&date_str, &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"], None, None) {
                        Some(parsed) => event.set("cato_networks.audit.change.After.socketInterface.creationDateMilliseconds", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cato_networks.audit.change.After.socketInterface.creationDateMilliseconds".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_socketInterface_creationDateMilliseconds",
                    )?;
                    event.remove(
                        "cato_networks.audit.change.After.socketInterface.creationDateMilliseconds",
                    );
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cato_networks.audit.change.After.socketInterface.downstreamBandwidth",
                ) {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.socketInterface.downstreamBandwidth")
                    {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketInterface.downstreamBandwidth".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.socketInterface.downstreamBandwidth",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_downstreamBandwidth_to_double",
                )?;
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
                event
                    .remove("cato_networks.audit.change.After.socketInterface.downstreamBandwidth");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketInterface.downstreamBandwidthMbpsPrecision") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketInterface.downstreamBandwidthMbpsPrecision") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketInterface.downstreamBandwidthMbpsPrecision".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketInterface.downstreamBandwidthMbpsPrecision", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketInterface_downstreamBandwidthMbpsPrecision_to_double")?;
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
                event.remove("cato_networks.audit.change.After.socketInterface.downstreamBandwidthMbpsPrecision");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("cato_networks.audit.change.After.socketInterface.lastModified")
                    && event
                        .get_str("cato_networks.audit.change.After.socketInterface.lastModified")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "cato_networks.audit.change.After.socketInterface.lastModified",
                    ) {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "cato_networks.audit.change.After.socketInterface.lastModified",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "cato_networks.audit.change.After.socketInterface.lastModified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_socketInterface_lastModified",
                    )?;
                    event.remove("cato_networks.audit.change.After.socketInterface.lastModified");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cato_networks.audit.change.After.socketInterface.metadata.disabledDestType",
                ) {
                    if let Some(val) = event.get("cato_networks.audit.change.After.socketInterface.metadata.disabledDestType") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketInterface.metadata.disabledDestType".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketInterface.metadata.disabledDestType", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_metadata_disabledDestType_to_boolean",
                )?;
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
                event.remove(
                    "cato_networks.audit.change.After.socketInterface.metadata.disabledDestType",
                );
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cato_networks.audit.change.After.socketInterface.metadata.linkOrder",
                ) {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.socketInterface.metadata.linkOrder")
                    {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketInterface.metadata.linkOrder".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.socketInterface.metadata.linkOrder",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_metadata_linkOrder_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.socketInterface.metadata.linkOrder");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketInterface.naturalOrder")
                {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.socketInterface.naturalOrder")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "cato_networks.audit.change.After.socketInterface.naturalOrder"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.socketInterface.naturalOrder",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_naturalOrder_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.socketInterface.naturalOrder");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketInterface.newSaveMethod")
                {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.socketInterface.newSaveMethod")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "cato_networks.audit.change.After.socketInterface.newSaveMethod"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.socketInterface.newSaveMethod",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_newSaveMethod_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.socketInterface.newSaveMethod");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "cato_networks.audit.change.After.socketInterface.offCloudTransportEnabled",
                ) {
                    if let Some(val) = event.get(
                        "cato_networks.audit.change.After.socketInterface.offCloudTransportEnabled",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketInterface.offCloudTransportEnabled".into(),
                            message,
                        })?;
                        event.set("cato_networks.audit.change.After.socketInterface.offCloudTransportEnabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_offCloudTransportEnabled_to_boolean",
                )?;
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
                event.remove(
                    "cato_networks.audit.change.After.socketInterface.offCloudTransportEnabled",
                );
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketInterface.physicalPort")
                {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.socketInterface.physicalPort")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "cato_networks.audit.change.After.socketInterface.physicalPort"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.socketInterface.physicalPort",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_physicalPort_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.socketInterface.physicalPort");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketInterface.s2sEnabled") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.socketInterface.s2sEnabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.socketInterface.s2sEnabled"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.socketInterface.s2sEnabled",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_s2sEnabled_to_boolean",
                )?;
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
                event.remove("cato_networks.audit.change.After.socketInterface.s2sEnabled");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("cato_networks.audit.change.After.socketInterface.upstreamBandwidth")
                {
                    if let Some(val) = event
                        .get("cato_networks.audit.change.After.socketInterface.upstreamBandwidth")
                    {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketInterface.upstreamBandwidth".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.socketInterface.upstreamBandwidth",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_upstreamBandwidth_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.socketInterface.upstreamBandwidth");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.socketInterface.upstreamBandwidthMbpsPrecision") {
                if let Some(val) = event.get("cato_networks.audit.change.After.socketInterface.upstreamBandwidthMbpsPrecision") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.socketInterface.upstreamBandwidthMbpsPrecision".into(),
                            message,
                        })?;
                    event.set("cato_networks.audit.change.After.socketInterface.upstreamBandwidthMbpsPrecision", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_socketInterface_upstreamBandwidthMbpsPrecision_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.socketInterface.upstreamBandwidthMbpsPrecision");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("cato_networks.audit.change.After.startDate")
                    && event.get_str("cato_networks.audit.change.After.startDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cato_networks.audit.change.After.startDate")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("cato_networks.audit.change.After.startDate", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cato_networks.audit.change.After.startDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_change_After_startDate",
                    )?;
                    event.remove("cato_networks.audit.change.After.startDate");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.trialBandwidth") {
                    if let Some(val) = event.get("cato_networks.audit.change.After.trialBandwidth")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.trialBandwidth".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.audit.change.After.trialBandwidth", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_trialBandwidth_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.trialBandwidth");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cato_networks.audit.change.After.upstreamBandwidth") {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.upstreamBandwidth")
                    {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "cato_networks.audit.change.After.upstreamBandwidth".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.upstreamBandwidth",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_upstreamBandwidth_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.upstreamBandwidth");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("cato_networks.audit.change.After.upstreamBandwidthMbpsPrecision")
                {
                    if let Some(val) =
                        event.get("cato_networks.audit.change.After.upstreamBandwidthMbpsPrecision")
                    {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "cato_networks.audit.change.After.upstreamBandwidthMbpsPrecision".into(),
                            message,
                        })?;
                        event.set(
                            "cato_networks.audit.change.After.upstreamBandwidthMbpsPrecision",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_change_After_upstreamBandwidthMbpsPrecision_to_double",
                )?;
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
                event.remove("cato_networks.audit.change.After.upstreamBandwidthMbpsPrecision");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event
                .get("cato_networks.audit.change_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") == Some("created") };
            if _cond {
                event.append_unique("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("event.action") == Some("modified") };
            if _cond {
                event.append_unique("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("event.action") == Some("deleted") };
            if _cond {
                event.append_unique("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.has_value("cato_networks.audit.creation_date")
                    && event.get_str("cato_networks.audit.creation_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cato_networks.audit.creation_date")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("cato_networks.audit.creation_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cato_networks.audit.creation_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_creation_date")?;
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
                    event.remove("cato_networks.audit.creation_date");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cato_networks.audit.creation_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("cato_networks.audit.insertion_date")
                    && event.get_str("cato_networks.audit.insertion_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cato_networks.audit.insertion_date")
                    {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("cato_networks.audit.insertion_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cato_networks.audit.insertion_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_insertion_date")?;
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
                    event.remove("cato_networks.audit.insertion_date");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cato_networks.audit.insertion_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            let _cond = { !event.has_value("cato_networks.audit.creation_date") };
            if _cond {
                if let Some(v) = event
                    .get("cato_networks.audit.insertion_date")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.allowedUpdatableItems")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("cato_networks.audit.change.After.allowedUpdatableItems")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDateMilliseconds")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.creationDateMilliseconds",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDateMilliseconds"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_change_After_allowedUpdatableItems_creationDateMilliseconds_to_date")?;
                                event.remove("_ingest._value.creationDateMilliseconds");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "cato_networks.audit.change.After.allowedUpdatableItems",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.allowedUpdatableItems")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.allowedUpdatableItems",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.newSaveMethod") {
                                if let Some(val) = event.get("_ingest._value.newSaveMethod") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.newSaveMethod".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.newSaveMethod", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_allowedUpdatableItems_newSaveMethod_to_boolean")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest._value.newSaveMethod");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.adminRoles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("cato_networks.audit.change.After.adminRoles")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDateMilliseconds")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.creationDateMilliseconds",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDateMilliseconds"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_change_After_adminRoles_creationDateMilliseconds_to_date")?;
                                event.remove("_ingest._value.creationDateMilliseconds");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "cato_networks.audit.change.After.adminRoles",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.adminRoles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.adminRoles",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.newSaveMethod") {
                                if let Some(val) = event.get("_ingest._value.newSaveMethod") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.newSaveMethod".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.newSaveMethod", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_change_After_adminRoles_newSaveMethod_to_boolean",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest._value.newSaveMethod");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.adminRoles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("cato_networks.audit.change.After.adminRoles")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) =
                                (|| -> Result<()> {
                                    if let Some(date_str) = event.get_as_string(
                                        "_ingest._value.role.creationDateMilliseconds",
                                    ) {
                                        match parse_date_out(
                                            &date_str,
                                            &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                            None,
                                            None,
                                        ) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.role.creationDateMilliseconds",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                            path: "_ingest._value.role.creationDateMilliseconds".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                                            }
                                        }
                                    }
                                    Ok(())
                                })()
                            {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_change_After_adminRoles_role_creationDateMilliseconds_to_date")?;
                                event.remove("_ingest._value.role.creationDateMilliseconds");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "cato_networks.audit.change.After.adminRoles",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.adminRoles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.adminRoles",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.role.isForToken") {
                                if let Some(val) = event.get("_ingest._value.role.isForToken") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.role.isForToken".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.role.isForToken", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_change_After_adminRoles_role_isForToken_to_boolean",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest._value.role.isForToken");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.adminRoles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.adminRoles",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.role.isPredefined") {
                                if let Some(val) = event.get("_ingest._value.role.isPredefined") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.role.isPredefined".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.role.isPredefined", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_change_After_adminRoles_role_isPredefined_to_boolean",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest._value.role.isPredefined");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.adminRoles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.adminRoles",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.role.isUsedOnExternalAccess") {
                                if let Some(val) =
                                    event.get("_ingest._value.role.isUsedOnExternalAccess")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.role.isUsedOnExternalAccess"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.role.isUsedOnExternalAccess",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_adminRoles_role_isUsedOnExternalAccess_to_boolean")?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest._value.role.isUsedOnExternalAccess");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.adminRoles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.adminRoles",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.role.newSaveMethod") {
                                if let Some(val) = event.get("_ingest._value.role.newSaveMethod") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.role.newSaveMethod".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.role.newSaveMethod", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_change_After_adminRoles_role_newSaveMethod_to_boolean",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest._value.role.newSaveMethod");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.adminRoles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.adminRoles",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.role.visible") {
                                if let Some(val) = event.get("_ingest._value.role.visible") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.role.visible".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.role.visible", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_change_After_adminRoles_role_visible_to_boolean",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest._value.role.visible");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.allowedItems")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("cato_networks.audit.change.After.allowedItems")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDateMilliseconds")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.creationDateMilliseconds",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDateMilliseconds"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_change_After_allowedItems_creationDateMilliseconds",
                                )?;
                                event.remove("_ingest._value.creationDateMilliseconds");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "cato_networks.audit.change.After.allowedItems",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.allowedItems")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.allowedItems",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.newSaveMethod") {
                                if let Some(val) = event.get("_ingest._value.newSaveMethod") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.newSaveMethod".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.newSaveMethod", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_change_After_allowedItems_newSaveMethod_to_boolean",
                            )?;
                            event.remove("_ingest._value.newSaveMethod");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.from")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cato_networks.audit.change.After.from").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDateMilliseconds")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.creationDateMilliseconds",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDateMilliseconds"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_change_After_from_creationDateMilliseconds",
                                )?;
                                event.remove("_ingest._value.creationDateMilliseconds");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "cato_networks.audit.change.After.from",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.from")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "cato_networks.audit.change.After.from", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.newSaveMethod") {
                            if let Some(val) = event.get("_ingest._value.newSaveMethod") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.newSaveMethod".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.newSaveMethod", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_change_After_from_newSaveMethod_to_boolean",
                        )?;
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
                        event.remove("_ingest._value.newSaveMethod");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.permissions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("cato_networks.audit.change.After.permissions")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDateMilliseconds")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.creationDateMilliseconds",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDateMilliseconds"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_change_After_creationDateMilliseconds",
                                )?;
                                event.remove("_ingest._value.creationDateMilliseconds");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "cato_networks.audit.change.After.permissions",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.permissions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.permissions",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.newSaveMethod") {
                                if let Some(val) = event.get("_ingest._value.newSaveMethod") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.newSaveMethod".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.newSaveMethod", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_change_After_permissions_newSaveMethod_to_boolean",
                            )?;
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest._value.newSaveMethod");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDate")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.creationDate", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDate".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_change_After_socketConnectionConfiguration_socketInterfaces_creationDate")?;
                                event.remove("_ingest._value.creationDate");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDateMilliseconds")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.creationDateMilliseconds",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDateMilliseconds"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_change_After_socketConnectionConfiguration_socketInterfaces_creationDateMilliseconds")?;
                                event.remove("_ingest._value.creationDateMilliseconds");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.downstreamBandwidth") {
                                if let Some(val) = event.get("_ingest._value.downstreamBandwidth") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.downstreamBandwidth".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.downstreamBandwidth", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_downstreamBandwidth_to_double")?;
                            event.remove("_ingest._value.downstreamBandwidth");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.downstreamBandwidthMbpsPrecision") {
                                if let Some(val) =
                                    event.get("_ingest._value.downstreamBandwidthMbpsPrecision")
                                {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                    path: "_ingest._value.downstreamBandwidthMbpsPrecision".into(),
                    message,
                    }
                                        })?;
                                    event.set(
                                        "_ingest._value.downstreamBandwidthMbpsPrecision",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_downstreamBandwidthMbpsPrecision_to_double")?;
                            event.remove("_ingest._value.downstreamBandwidthMbpsPrecision");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.lastModified")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.lastModified", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.lastModified".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_change_After_socketConnectionConfiguration_socketInterfaces_lastModified")?;
                                event.remove("_ingest._value.lastModified");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.metadata.disabledDestType") {
                                if let Some(val) =
                                    event.get("_ingest._value.metadata.disabledDestType")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.metadata.disabledDestType"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.metadata.disabledDestType",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_metadata_disabledDestType_to_boolean")?;
                            event.remove("_ingest._value.metadata.disabledDestType");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.metadata.linkOrder") {
                                if let Some(val) = event.get("_ingest._value.metadata.linkOrder") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.metadata.linkOrder".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.metadata.linkOrder", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_metadata_linkOrder_to_double")?;
                            event.remove("_ingest._value.metadata.linkOrder");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.naturalOrder") {
                                if let Some(val) = event.get("_ingest._value.naturalOrder") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.naturalOrder".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.naturalOrder", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_naturalOrder_to_double")?;
                            event.remove("_ingest._value.naturalOrder");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.has_value("_ingest._value.metadata.speed") {
                                if let Some(input) =
                                    event.get_string("_ingest._value.metadata.speed")
                                {
                                    let mut remaining: &str = &input;
                                    let mut captured: Vec<(&str, &str)> = Vec::new();
                                    let matched = 'dissect: {
                                        let Some(rest) = remaining.strip_prefix("_") else {
                                            break 'dissect false;
                                        };
                                        remaining = rest;
                                        captured.push(("_ingest._value.metadata.speed", remaining));
                                        true
                                    };
                                    if matched {
                                        for (path, value) in captured {
                                            event.set(path, value)?;
                                        }
                                    }
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.newSaveMethod") {
                                if let Some(val) = event.get("_ingest._value.newSaveMethod") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.newSaveMethod".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.newSaveMethod", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_newSaveMethod_to_boolean")?;
                            event.remove("_ingest._value.newSaveMethod");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.offCloudTransportEnabled") {
                                if let Some(val) =
                                    event.get("_ingest._value.offCloudTransportEnabled")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.offCloudTransportEnabled"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.offCloudTransportEnabled",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_offCloudTransportEnabled_to_boolean")?;
                            event.remove("_ingest._value.offCloudTransportEnabled");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.physicalPort") {
                                if let Some(val) = event.get("_ingest._value.physicalPort") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.physicalPort".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.physicalPort", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_physicalPort_to_double")?;
                            event.remove("_ingest._value.physicalPort");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.s2sEnabled") {
                                if let Some(val) = event.get("_ingest._value.s2sEnabled") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.s2sEnabled".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.s2sEnabled", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_s2sEnabled_to_boolean")?;
                            event.remove("_ingest._value.s2sEnabled");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.upstreamBandwidth") {
                                if let Some(val) = event.get("_ingest._value.upstreamBandwidth") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.upstreamBandwidth".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.upstreamBandwidth", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_upstreamBandwidth_to_double")?;
                            event.remove("_ingest._value.upstreamBandwidth");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketInterfaces",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.upstreamBandwidthMbpsPrecision") {
                                if let Some(val) =
                                    event.get("_ingest._value.upstreamBandwidthMbpsPrecision")
                                {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path:
                                                    "_ingest._value.upstreamBandwidthMbpsPrecision"
                                                        .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.upstreamBandwidthMbpsPrecision",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketInterfaces_upstreamBandwidthMbpsPrecision_to_double")?;
                            event.remove("_ingest._value.upstreamBandwidthMbpsPrecision");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDate")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.creationDate", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDate".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_change_After_socketConnectionConfiguration_socketsSettings_primary_links_creationDate")?;
                                event.remove("_ingest._value.creationDate");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDateMilliseconds")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.creationDateMilliseconds",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDateMilliseconds"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_change_After_socketConnectionConfiguration_socketsSettings_primary_links_creationDateMilliseconds")?;
                                event.remove("_ingest._value.creationDateMilliseconds");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.downstreamBandwidth") {
                                if let Some(val) = event.get("_ingest._value.downstreamBandwidth") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.downstreamBandwidth".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.downstreamBandwidth", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_downstreamBandwidth_to_double")?;
                            event.remove("_ingest._value.downstreamBandwidth");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.downstreamBandwidthMbpsPrecision") {
                                if let Some(val) =
                                    event.get("_ingest._value.downstreamBandwidthMbpsPrecision")
                                {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                    path: "_ingest._value.downstreamBandwidthMbpsPrecision".into(),
                    message,
                    }
                                        })?;
                                    event.set(
                                        "_ingest._value.downstreamBandwidthMbpsPrecision",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_downstreamBandwidthMbpsPrecision_to_double")?;
                            event.remove("_ingest._value.downstreamBandwidthMbpsPrecision");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.lastModified")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.lastModified", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.lastModified".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_change_After_socketConnectionConfiguration_socketsSettings_primary_links_lastModified")?;
                                event.remove("_ingest._value.lastModified");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.metadata.disabledDestType") {
                                if let Some(val) =
                                    event.get("_ingest._value.metadata.disabledDestType")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.metadata.disabledDestType"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.metadata.disabledDestType",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_metadata_disabledDestType_to_boolean")?;
                            event.remove("_ingest._value.metadata.disabledDestType");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.metadata.linkOrder") {
                                if let Some(val) = event.get("_ingest._value.metadata.linkOrder") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.metadata.linkOrder".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.metadata.linkOrder", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_metadata_linkOrder_to_double")?;
                            event.remove("_ingest._value.metadata.linkOrder");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.has_value("_ingest._value.metadata.speed") {
                                if let Some(input) =
                                    event.get_string("_ingest._value.metadata.speed")
                                {
                                    let mut remaining: &str = &input;
                                    let mut captured: Vec<(&str, &str)> = Vec::new();
                                    let matched = 'dissect: {
                                        let Some(rest) = remaining.strip_prefix("_") else {
                                            break 'dissect false;
                                        };
                                        remaining = rest;
                                        captured.push(("_ingest._value.metadata.speed", remaining));
                                        true
                                    };
                                    if matched {
                                        for (path, value) in captured {
                                            event.set(path, value)?;
                                        }
                                    }
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.naturalOrder") {
                                if let Some(val) = event.get("_ingest._value.naturalOrder") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.naturalOrder".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.naturalOrder", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_naturalOrder_to_double")?;
                            event.remove("_ingest._value.naturalOrder");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.newSaveMethod") {
                                if let Some(val) = event.get("_ingest._value.newSaveMethod") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.newSaveMethod".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.newSaveMethod", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_newSaveMethod_to_boolean")?;
                            event.remove("_ingest._value.newSaveMethod");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.offCloudTransportEnabled") {
                                if let Some(val) =
                                    event.get("_ingest._value.offCloudTransportEnabled")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.offCloudTransportEnabled"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.offCloudTransportEnabled",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_offCloudTransportEnabled_to_boolean")?;
                            event.remove("_ingest._value.offCloudTransportEnabled");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.physicalPort") {
                                if let Some(val) = event.get("_ingest._value.physicalPort") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.physicalPort".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.physicalPort", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_physicalPort_to_double")?;
                            event.remove("_ingest._value.physicalPort");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.s2sEnabled") {
                                if let Some(val) = event.get("_ingest._value.s2sEnabled") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.s2sEnabled".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.s2sEnabled", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_s2sEnabled_to_boolean")?;
                            event.remove("_ingest._value.s2sEnabled");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.upstreamBandwidthMbpsPrecision") {
                                if let Some(val) =
                                    event.get("_ingest._value.upstreamBandwidthMbpsPrecision")
                                {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path:
                                                    "_ingest._value.upstreamBandwidthMbpsPrecision"
                                                        .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.upstreamBandwidthMbpsPrecision",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_upstreamBandwidthMbpsPrecision_to_double")?;
                            event.remove("_ingest._value.upstreamBandwidthMbpsPrecision");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.upstreamBandwidth") {
                                if let Some(val) = event.get("_ingest._value.upstreamBandwidth") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.upstreamBandwidth".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.upstreamBandwidth", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_links_upstreamBandwidth_to_double")?;
                            event.remove("_ingest._value.upstreamBandwidth");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedAddOns").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedAddOns",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.numberOfPorts") {
                                if let Some(val) = event.get("_ingest._value.numberOfPorts") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.numberOfPorts".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.numberOfPorts", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_supportedAddOns_numberOfPorts_to_double")?;
                            event.remove("_ingest._value.numberOfPorts");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.automaticallyManaged") {
                                if let Some(val) = event.get("_ingest._value.automaticallyManaged")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.automaticallyManaged".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.automaticallyManaged", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_supportedMigrations_automaticallyManaged_to_boolean")?;
                            event.remove("_ingest._value.automaticallyManaged");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations",
                    |event| {
                        if event.has_value("_ingest._value.possibleAddOns") {
                            foreach_array(event, "_ingest._value.possibleAddOns", |event| {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.numberOfPorts") {
                                        if let Some(val) = event.get("_ingest._value.numberOfPorts")
                                        {
                                            let converted = convert_value(val, "double").map_err(
                                                |message| TransformError::ParseError {
                                                    path: "_ingest._value.numberOfPorts".into(),
                                                    message,
                                                },
                                            )?;
                                            event.set("_ingest._value.numberOfPorts", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketConnectionConfiguration_socketsSettings_primary_supportedMigrations_possibleAddOns_numberOfPorts_to_double")?;
                                    event.remove("_ingest._value.numberOfPorts");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDate")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.creationDate", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDate".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_change_After_socketsSettings_primary_links_creationDate",
                                )?;
                                event.remove("_ingest._value.creationDate");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "cato_networks.audit.change.After.socketsSettings.primary.links",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.creationDateMilliseconds")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.creationDateMilliseconds",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.creationDateMilliseconds"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set("_ingest.on_failure_processor_tag", "date_change_After_socketsSettings_primary_links_creationDateMilliseconds")?;
                                event.remove("_ingest._value.creationDateMilliseconds");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "cato_networks.audit.change.After.socketsSettings.primary.links",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.downstreamBandwidth") {
                                if let Some(val) = event.get("_ingest._value.downstreamBandwidth") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.downstreamBandwidth".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.downstreamBandwidth", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_downstreamBandwidth_to_double")?;
                            event.remove("_ingest._value.downstreamBandwidth");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.downstreamBandwidthMbpsPrecision") {
                                if let Some(val) =
                                    event.get("_ingest._value.downstreamBandwidthMbpsPrecision")
                                {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                    path: "_ingest._value.downstreamBandwidthMbpsPrecision".into(),
                    message,
                    }
                                        })?;
                                    event.set(
                                        "_ingest._value.downstreamBandwidthMbpsPrecision",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_downstreamBandwidthMbpsPrecision_to_double")?;
                            event.remove("_ingest._value.downstreamBandwidthMbpsPrecision");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.lastModified")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &["UNIX_MS", "MMM d, yyyy h:mm:ss a", "ISO8601"],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.lastModified", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.lastModified".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_change_After_socketsSettings_primary_links_lastModified",
                                )?;
                                event.remove("_ingest._value.lastModified");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "cato_networks.audit.change.After.socketsSettings.primary.links",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.metadata.disabledDestType") {
                                if let Some(val) =
                                    event.get("_ingest._value.metadata.disabledDestType")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.metadata.disabledDestType"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.metadata.disabledDestType",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_metadata_disabledDestType_to_boolean")?;
                            event.remove("_ingest._value.metadata.disabledDestType");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.metadata.linkOrder") {
                                if let Some(val) = event.get("_ingest._value.metadata.linkOrder") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.metadata.linkOrder".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.metadata.linkOrder", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_metadata_linkOrder_to_double")?;
                            event.remove("_ingest._value.metadata.linkOrder");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.has_value("_ingest._value.metadata.speed") {
                                if let Some(input) =
                                    event.get_string("_ingest._value.metadata.speed")
                                {
                                    let mut remaining: &str = &input;
                                    let mut captured: Vec<(&str, &str)> = Vec::new();
                                    let matched = 'dissect: {
                                        let Some(rest) = remaining.strip_prefix("_") else {
                                            break 'dissect false;
                                        };
                                        remaining = rest;
                                        captured.push(("_ingest._value.metadata.speed", remaining));
                                        true
                                    };
                                    if matched {
                                        for (path, value) in captured {
                                            event.set(path, value)?;
                                        }
                                    }
                                }
                            }
                            Ok(())
                        })();
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.naturalOrder") {
                                if let Some(val) = event.get("_ingest._value.naturalOrder") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.naturalOrder".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.naturalOrder", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_naturalOrder_to_double")?;
                            event.remove("_ingest._value.naturalOrder");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.newSaveMethod") {
                                if let Some(val) = event.get("_ingest._value.newSaveMethod") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.newSaveMethod".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.newSaveMethod", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_newSaveMethod_to_boolean")?;
                            event.remove("_ingest._value.newSaveMethod");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.offCloudTransportEnabled") {
                                if let Some(val) =
                                    event.get("_ingest._value.offCloudTransportEnabled")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.offCloudTransportEnabled"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.offCloudTransportEnabled",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_offCloudTransportEnabled_to_boolean")?;
                            event.remove("_ingest._value.offCloudTransportEnabled");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.physicalPort") {
                                if let Some(val) = event.get("_ingest._value.physicalPort") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.physicalPort".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.physicalPort", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_physicalPort_to_double")?;
                            event.remove("_ingest._value.physicalPort");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.s2sEnabled") {
                                if let Some(val) = event.get("_ingest._value.s2sEnabled") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.s2sEnabled".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.s2sEnabled", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_s2sEnabled_to_boolean")?;
                            event.remove("_ingest._value.s2sEnabled");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.upstreamBandwidth") {
                                if let Some(val) = event.get("_ingest._value.upstreamBandwidth") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.upstreamBandwidth".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.upstreamBandwidth", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_upstreamBandwidth_to_double")?;
                            event.remove("_ingest._value.upstreamBandwidth");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.links")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.links",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.upstreamBandwidthMbpsPrecision") {
                                if let Some(val) =
                                    event.get("_ingest._value.upstreamBandwidthMbpsPrecision")
                                {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path:
                                                    "_ingest._value.upstreamBandwidthMbpsPrecision"
                                                        .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.upstreamBandwidthMbpsPrecision",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_links_upstreamBandwidthMbpsPrecision_to_double")?;
                            event.remove("_ingest._value.upstreamBandwidthMbpsPrecision");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("cato_networks.audit.change.After.socketsSettings.primary.supportedAddOns")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.supportedAddOns",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.numberOfPorts") {
                                if let Some(val) = event.get("_ingest._value.numberOfPorts") {
                                    let converted =
                                        convert_value(val, "double").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.numberOfPorts".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.numberOfPorts", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_supportedAddOns_numberOfPorts_to_double")?;
                            event.remove("_ingest._value.numberOfPorts");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.automaticallyManaged") {
                                if let Some(val) = event.get("_ingest._value.automaticallyManaged")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.automaticallyManaged".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.automaticallyManaged", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_supportedMigrations_automaticallyManaged_to_boolean")?;
                            event.remove("_ingest._value.automaticallyManaged");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get("cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "cato_networks.audit.change.After.socketsSettings.primary.supportedMigrations",
                    |event| {
                        if event.has_value("_ingest._value.possibleAddOns") {
                            foreach_array(event, "_ingest._value.possibleAddOns", |event| {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.numberOfPorts") {
                                        if let Some(val) = event.get("_ingest._value.numberOfPorts")
                                        {
                                            let converted = convert_value(val, "double").map_err(
                                                |message| TransformError::ParseError {
                                                    path: "_ingest._value.numberOfPorts".into(),
                                                    message,
                                                },
                                            )?;
                                            event.set("_ingest._value.numberOfPorts", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set("_ingest.on_failure_processor_tag", "convert_change_After_socketsSettings_primary_supportedMigrations_possible_add_ons_numberOfPorts_to_double")?;
                                    event.remove("_ingest._value.numberOfPorts");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                        }
                        Ok(())
                    },
                )?;
            }

            // Painless script
            // Source: // Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\nif (ctx.cato_networks.audit != null) {\n  ctx.cato_networks.audit = convertToSnakeCase(ctx.cato_networks.audit);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// Helper function to convert camelCase to snake_case\nString camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\n// Recursive function to handle nested fields\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    // Convert each key in the map\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      // Skip fields that contain '@' in their name\n      if (!entry.getKey().contains(\"@\")) {\n        String newKey = camelToSnake(entry.getKey());\n        newObj[newKey] = convertToSnakeCase(entry.getValue());\n      }\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    // If it's a list, process each item recursively\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\nif (ctx.cato_networks.audit != null) {\n  ctx.cato_networks.audit = convertToSnakeCase(ctx.cato_networks.audit);\n}\n"#
                ),
            )?;

            event.remove("cato_networks.audit.admin");
            event.remove("cato_networks.audit.admin_id");
            event.remove("cato_networks.audit.change_type");
            event.remove("cato_networks.audit.creation_date");
            event.remove("cato_networks.audit.insertion_date");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} {}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
