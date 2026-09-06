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

            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
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

            parse_json_field(event, "event.original", "json")?;

            event.set("event.kind", json!("alert"))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.uid") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.uid".into(),
                    });
                }
                if let Some(v) = event.get("json.updatedDate") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.updatedDate".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.account.inPlatformIdentifier") {
                event.rename(
                    "json.account.inPlatformIdentifier",
                    "cyera.issue.account.in_platform_identifier",
                )?;
            }

            if let Some(v) = event
                .get("cyera.issue.account.in_platform_identifier")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if event.has_value("json.account.name") {
                event.rename("json.account.name", "cyera.issue.account.name")?;
            }

            if let Some(v) = event
                .get("cyera.issue.account.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.name", v)?;
            }

            if event.has_value("json.classificationGroups") {
                event.rename(
                    "json.classificationGroups",
                    "cyera.issue.classification_groups",
                )?;
            }

            if event.has_value("json.cloudProviderTags") {
                event.rename("json.cloudProviderTags", "cyera.issue.cloud_provider_tags")?;
            }

            let _cond = {
                event.has_value("json.createdDate") && event.get_str("json.createdDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("cyera.issue.created_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.createdDate".into(),
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
                        "date_createdDate_b3dcb772",
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cyera.issue.created_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.affectedRecords") {
                    if let Some(val) = event.get("json.data.affectedRecords") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.affectedRecords".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.issue.data.affected_records", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_data_affectedRecords_to_long_df98bb11",
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
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.data.dataClassesUids") {
                event.rename(
                    "json.data.dataClassesUids",
                    "cyera.issue.data.data_classes_uids",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.objectsAtRisk") {
                    if let Some(val) = event.get("json.data.objectsAtRisk") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.objectsAtRisk".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.issue.data.objects_at_risk", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_data_objectsAtRisk_to_long_7d5cf7a1",
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
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.recordsAtRisk") {
                    if let Some(val) = event.get("json.data.recordsAtRisk") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.recordsAtRisk".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.issue.data.records_at_risk", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_data_recordsAtRisk_to_long_7cf515b3",
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
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.datastoreCloudProviderTags") {
                event.rename(
                    "json.datastoreCloudProviderTags",
                    "cyera.issue.datastore_cloud_provider_tags",
                )?;
            }

            if event.has_value("json.datastoreName") {
                event.rename("json.datastoreName", "cyera.issue.datastore_name")?;
            }

            let _cond = {
                event
                    .get("json.datastoreOwners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.datastoreOwners", |event| {
                    if event.has_value("_ingest._value.datastoreOwnerUid") {
                        event.rename(
                            "_ingest._value.datastoreOwnerUid",
                            "_ingest._value.datastore_owner_uid",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.datastoreOwners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.datastoreOwners", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.datastore_owner_uid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.datastoreOwners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.datastoreOwners", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.datastoreOwners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.datastoreOwners", |event| {
                    if event.has_value("_ingest._value.ownerType") {
                        event.rename("_ingest._value.ownerType", "_ingest._value.owner_type")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.datastoreOwners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.datastoreOwners", |event| {
                    event.append_unique(
                        "user.roles",
                        json!(
                            event
                                .get("_ingest._value.owner_type")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.datastoreOwners") {
                event.rename("json.datastoreOwners", "cyera.issue.datastore_owners")?;
            }

            if event.has_value("json.datastoreUid") {
                event.rename("json.datastoreUid", "cyera.issue.datastore_uid")?;
            }

            if event.has_value("json.datastoreUserTags") {
                event.rename("json.datastoreUserTags", "cyera.issue.datastore_user_tags")?;
            }

            if event.has_value("json.engine") {
                event.rename("json.engine", "cyera.issue.engine")?;
            }

            if event.has_value("json.infrastructure") {
                event.rename("json.infrastructure", "cyera.issue.infrastructure")?;
            }

            if let Some(v) = event
                .get("cyera.issue.infrastructure")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.service.name", v)?;
            }

            let _cond = { event.get("json.itsmTickets").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.itsmTickets", |event| {
                    if event.has_value("_ingest._value.vendorLink") {
                        event.rename("_ingest._value.vendorLink", "_ingest._value.vendor.link")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.itsmTickets").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.itsmTickets", |event| {
                    if event.has_value("_ingest._value.vendorStatus") {
                        event.rename(
                            "_ingest._value.vendorStatus",
                            "_ingest._value.vendor.status",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.itsmTickets").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.itsmTickets", |event| {
                    if event.has_value("_ingest._value.vendorTicketId") {
                        event.rename(
                            "_ingest._value.vendorTicketId",
                            "_ingest._value.vendor.ticket_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.itsmTickets") {
                event.rename("json.itsmTickets", "cyera.issue.itsm_tickets")?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "cyera.issue.name")?;
            }

            if let Some(v) = event
                .get("cyera.issue.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("json.owner") {
                event.rename("json.owner", "cyera.issue.owner")?;
            }

            let _cond = {
                event.has_value("cyera.issue.owner")
                    && event.get("cyera.issue.owner").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(v) = event
                    .get("cyera.issue.owner")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { !event.has_value("user.email") };
            if _cond {
                if let Some(v) = event
                    .get("cyera.issue.owner")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event.has_value("cyera.issue.owner")
                    && event.get("cyera.issue.owner").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                if event.has_value("cyera.issue.owner") {
                    if let Some(input) = event.get_string("cyera.issue.owner") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "cyera.issue.owner".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("cyera.issue.owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cyera.issue.owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.policyUid") {
                event.rename("json.policyUid", "cyera.issue.policy_uid")?;
            }

            if event.has_value("json.provider") {
                event.rename("json.provider", "cyera.issue.provider")?;
            }

            if let Some(v) = event
                .get("cyera.issue.provider")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if event.has_value("cloud.provider") {
                map_strings(event, "cloud.provider", "cloud.provider", str::to_lowercase)?;
            }

            let _cond = { event.get("json.regions").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.regions", |event| {
                    event.append_unique(
                        "cloud.region",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.regions") {
                event.rename("json.regions", "cyera.issue.regions")?;
            }

            if event.has_value("json.remediationAdvice") {
                event.rename("json.remediationAdvice", "cyera.issue.remediation_advice")?;
            }

            if event.has_value("json.resolution") {
                event.rename("json.resolution", "cyera.issue.resolution")?;
            }

            if event.has_value("json.resolutionNote") {
                event.rename("json.resolutionNote", "cyera.issue.resolution_note")?;
            }

            if event.has_value("json.risk.description") {
                event.rename("json.risk.description", "cyera.issue.risk.description")?;
            }

            if event.has_value("json.risk.frameworks") {
                event.rename("json.risk.frameworks", "cyera.issue.risk.frameworks")?;
            }

            if event.has_value("json.risk.policyUid") {
                event.rename("json.risk.policyUid", "cyera.issue.risk.policy_uid")?;
            }

            if event.has_value("json.risk.useCases") {
                event.rename("json.risk.useCases", "cyera.issue.risk.use_cases")?;
            }

            if event.has_value("json.riskStatus") {
                event.rename("json.riskStatus", "cyera.issue.risk_status")?;
            }

            if event.has_value("json.severity") {
                event.rename("json.severity", "cyera.issue.severity")?;
            }

            let _cond = {
                event
                    .get("cyera.issue.severity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\nString severity_in_text = ctx.cyera.issue.severity;\nif (severity_in_text.equalsIgnoreCase(\"Low\")) {\n  ctx.event.severity = 21;\n} else if (severity_in_text.equalsIgnoreCase(\"Medium\")) {\n  ctx.event.severity = 47;\n} else if (severity_in_text.equalsIgnoreCase(\"High\")) {\n  ctx.event.severity = 73;\n} else if (severity_in_text.equalsIgnoreCase(\"Critical\")) {\n  ctx.event.severity = 99;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event = ctx.event ?: [:];\nString severity_in_text = ctx.cyera.issue.severity;\nif (severity_in_text.equalsIgnoreCase(\"Low\")) {\n  ctx.event.severity = 21;\n} else if (severity_in_text.equalsIgnoreCase(\"Medium\")) {\n  ctx.event.severity = 47;\n} else if (severity_in_text.equalsIgnoreCase(\"High\")) {\n  ctx.event.severity = 73;\n} else if (severity_in_text.equalsIgnoreCase(\"Critical\")) {\n  ctx.event.severity = 99;\n}"#
                    ),
                )?;
            }

            if event.has_value("json.status") {
                event.rename("json.status", "cyera.issue.status")?;
            }

            if event.has_value("json.uid") {
                event.rename("json.uid", "cyera.issue.uid")?;
            }

            if let Some(v) = event
                .get("cyera.issue.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("json.updatedDate") && event.get_str("json.updatedDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updatedDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("cyera.issue.updated_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.updatedDate".into(),
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
                        "date_updatedDate_30b90811",
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("cyera.issue.updated_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event
                    .get("cyera.issue.datastore_owners")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "cyera.issue.datastore_owners", |event| {
                    let _cond = {
                        !event.has_value("tags")
                            || !(event.get("tags").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => a.iter().any(|x| {
                                    x.as_str() == Some("preserve_duplicate_custom_fields")
                                }),
                                serde_json::Value::String(s) => {
                                    s.contains("preserve_duplicate_custom_fields")
                                }
                                _ => false,
                            }))
                    };
                    if _cond {
                        event.remove("_ingest._value.owner_type");
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("cyera.issue.account.in_platform_identifier");
                event.remove("cyera.issue.account.name");
                event.remove("cyera.issue.created_date");
                event.remove("cyera.issue.infrastructure");
                event.remove("cyera.issue.name");
                event.remove("cyera.issue.provider");
                event.remove("cyera.issue.regions");
                event.remove("cyera.issue.uid");
                event.remove("cyera.issue.updated_date");
            }

            event.remove("json");

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
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
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
