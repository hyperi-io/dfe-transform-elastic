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
            event.set("ecs.version", json!("8.16.0"))?;

            event.set("observer.vendor", json!("Google Workspace"))?;

            event.set("observer.product", json!("Vault"))?;

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

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "json")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            let _cond = {
                event
                    .get("json.events.parameters")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.google_workspace = ctx.google_workspace ?: [:]; ctx.google_workspace.vault = ctx.google_workspace.vault ?: [:]; for (def param : ctx.json.events.parameters) {\n  if (param.name == null) {\n    continue;\n  }\n  def lw_case_name = param.name.toLowerCase();\n  if (param.value != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.value;\n  } else if (param.boolValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.boolValue;\n  } else if (param.intValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.intValue;\n  } else if (param.multiValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.multiValue;\n  } else if (param.multiIntValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.multiIntValue;\n  } else if (param.multiBoolValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.multiBoolValue;\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.google_workspace = ctx.google_workspace ?: [:]; ctx.google_workspace.vault = ctx.google_workspace.vault ?: [:]; for (def param : ctx.json.events.parameters) {\n  if (param.name == null) {\n    continue;\n  }\n  def lw_case_name = param.name.toLowerCase();\n  if (param.value != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.value;\n  } else if (param.boolValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.boolValue;\n  } else if (param.intValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.intValue;\n  } else if (param.multiValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.multiValue;\n  } else if (param.multiIntValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.multiIntValue;\n  } else if (param.multiBoolValue != null) {\n    ctx.google_workspace.vault[lw_case_name] = param.multiBoolValue;\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_flatten_event_parameters",
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

            event.remove("json.events.parameters");

            if event.has_value("json.events.name") {
                event.rename("json.events.name", "google_workspace.vault.name")?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = { event.has_value("google_workspace.vault.name") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\ndef category = params.get(ctx.google_workspace.vault.name);\nif (category == null) {\n  ctx.event.remove('category');\n} else {\n  ctx.event.category = [category];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\ndef category = params.get(ctx.google_workspace.vault.name);\nif (category == null) {\n  ctx.event.remove('category');\n} else {\n  ctx.event.category = [category];\n}"#
                        ),
                        cached_params!(
                            "{\"add_collaborator_begin\":\"iam\",\"add_collaborator_end\":\"iam\",\"add_litigation_hold_begin\":\"configuration\",\"add_litigation_hold_end\":\"configuration\",\"add_preservation_rule_begin\":\"configuration\",\"add_preservation_rule_end\":\"configuration\",\"add_retention_rule_begin\":\"configuration\",\"add_retention_rule_end\":\"configuration\",\"cancel_accelerated_deletion_begin\":\"configuration\",\"cancel_accelerated_deletion_end\":\"configuration\",\"close_investigation_begin\":\"configuration\",\"close_investigation_end\":\"configuration\",\"convert_saved_query_to_collection_begin\":\"configuration\",\"convert_saved_query_to_collection_end\":\"configuration\",\"create_accelerated_deletion_begin\":\"configuration\",\"create_accelerated_deletion_end\":\"configuration\",\"create_export_begin\":\"configuration\",\"create_export_end\":\"configuration\",\"create_investigation_begin\":\"configuration\",\"create_investigation_end\":\"configuration\",\"create_saved_query_begin\":\"configuration\",\"create_saved_query_end\":\"configuration\",\"delete_export_begin\":\"configuration\",\"delete_export_end\":\"configuration\",\"delete_export_fail\":\"configuration\",\"delete_investigation_begin\":\"configuration\",\"delete_investigation_end\":\"configuration\",\"delete_preservation_rule_begin\":\"configuration\",\"delete_preservation_rule_end\":\"configuration\",\"delete_retention_rule_begin\":\"configuration\",\"delete_retention_rule_end\":\"configuration\",\"delete_saved_query_begin\":\"configuration\",\"delete_saved_query_end\":\"configuration\",\"deletion_search\":\"configuration\",\"download_cross_matter_litigation_hold_report\":\"configuration\",\"download_per_matter_litigation_hold_report\":\"configuration\",\"modify_default_retention_period_begin\":\"configuration\",\"modify_default_retention_period_end\":\"configuration\",\"obsolete_api_exports_list\":\"configuration\",\"obsolete_api_holds_insert\":\"configuration\",\"obsolete_api_holds_list\":\"configuration\",\"obsolete_api_matters_delete\":\"configuration\",\"obsolete_api_matters_get\":\"configuration\",\"obsolete_api_matters_insert\":\"configuration\",\"obsolete_api_matters_list\":\"configuration\",\"obsolete_api_matters_update\":\"configuration\",\"obsolete_preview_retention_rule_count\":\"configuration\",\"preview_retention_rule\":\"configuration\",\"remove_collaborator_begin\":\"iam\",\"remove_collaborator_end\":\"iam\",\"remove_litigation_hold_begin\":\"configuration\",\"remove_litigation_hold_end\":\"configuration\",\"reopen_investigation_begin\":\"configuration\",\"reopen_investigation_end\":\"configuration\",\"restore_investigation_begin\":\"configuration\",\"restore_investigation_end\":\"configuration\",\"update_investigation_details_begin\":\"configuration\",\"update_investigation_details_end\":\"configuration\",\"update_preservation_rule_add_holds_begin\":\"configuration\",\"update_preservation_rule_add_holds_end\":\"configuration\",\"update_preservation_rule_query_begin\":\"configuration\",\"update_preservation_rule_query_end\":\"configuration\",\"update_preservation_rule_remove_holds_begin\":\"configuration\",\"update_preservation_rule_remove_holds_end\":\"configuration\",\"update_retention_rule_begin\":\"configuration\",\"update_retention_rule_end\":\"configuration\",\"update_retention_settings\":\"configuration\",\"update_saved_query_details_begin\":\"configuration\",\"update_saved_query_details_end\":\"configuration\",\"view_cross_matter_litigation_hold_report\":\"configuration\",\"view_custodian_litigation_hold_report\":\"configuration\",\"view_investigation\":\"configuration\",\"view_matter_audit_log\":\"configuration\",\"view_per_matter_litigation_hold_report\":\"configuration\",\"view_retention_policy\":\"configuration\",\"view_retention_settings\":\"configuration\",\"view_system_audit_log\":\"configuration\",\"download_count_per_account_csv\":\"file\",\"export\":\"file\",\"export_file_download\":\"file\",\"legacy_export_download\":\"file\",\"view_document\":\"file\",\"view_document_information\":\"file\",\"view_external_document\":\"file\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_event_category_from_vault_name",
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

            let _cond = { event.has_value("google_workspace.vault.name") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\ndef type = params.get(ctx.google_workspace.vault.name);\nif (type == null) {\n  ctx.event.remove('type');\n} else {\n  ctx.event.type = [type];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\ndef type = params.get(ctx.google_workspace.vault.name);\nif (type == null) {\n  ctx.event.remove('type');\n} else {\n  ctx.event.type = [type];\n}"#
                        ),
                        cached_params!(
                            "{\"add_collaborator_begin\":\"info\",\"add_collaborator_end\":\"info\",\"add_litigation_hold_begin\":\"info\",\"add_litigation_hold_end\":\"info\",\"add_preservation_rule_begin\":\"info\",\"add_preservation_rule_end\":\"info\",\"add_retention_rule_begin\":\"info\",\"add_retention_rule_end\":\"info\",\"cancel_accelerated_deletion_begin\":\"deletion\",\"cancel_accelerated_deletion_end\":\"deletion\",\"close_investigation_begin\":\"change\",\"close_investigation_end\":\"change\",\"convert_saved_query_to_collection_begin\":\"change\",\"convert_saved_query_to_collection_end\":\"change\",\"create_accelerated_deletion_begin\":\"creation\",\"create_accelerated_deletion_end\":\"creation\",\"create_export_begin\":\"creation\",\"create_export_end\":\"creation\",\"create_investigation_begin\":\"creation\",\"create_investigation_end\":\"creation\",\"create_saved_query_begin\":\"creation\",\"create_saved_query_end\":\"creation\",\"delete_export_begin\":\"deletion\",\"delete_export_end\":\"deletion\",\"delete_export_fail\":\"info\",\"delete_investigation_begin\":\"deletion\",\"delete_investigation_end\":\"deletion\",\"delete_preservation_rule_begin\":\"deletion\",\"delete_preservation_rule_end\":\"deletion\",\"delete_retention_rule_begin\":\"deletion\",\"delete_retention_rule_end\":\"deletion\",\"delete_saved_query_begin\":\"deletion\",\"delete_saved_query_end\":\"deletion\",\"deletion_search\":\"deletion\",\"download_count_per_account_csv\":\"info\",\"download_cross_matter_litigation_hold_report\":\"info\",\"download_per_matter_litigation_hold_report\":\"info\",\"export\":\"info\",\"export_file_download\":\"info\",\"get_count_operation\":\"info\",\"legacy_export_download\":\"info\",\"modify_default_retention_period_begin\":\"change\",\"modify_default_retention_period_end\":\"change\",\"obsolete_api_exports_list\":\"change\",\"obsolete_api_holds_insert\":\"change\",\"obsolete_api_holds_list\":\"change\",\"obsolete_api_matters_delete\":\"change\",\"obsolete_api_matters_get\":\"change\",\"obsolete_api_matters_insert\":\"change\",\"obsolete_api_matters_list\":\"change\",\"obsolete_api_matters_update\":\"change\",\"obsolete_preview_retention_rule_count\":\"change\",\"preview_retention_rule\":\"info\",\"remove_collaborator_begin\":\"info\",\"remove_collaborator_end\":\"info\",\"remove_litigation_hold_begin\":\"info\",\"remove_litigation_hold_end\":\"info\",\"reopen_investigation_begin\":\"change\",\"reopen_investigation_end\":\"change\",\"restore_investigation_begin\":\"change\",\"restore_investigation_end\":\"change\",\"search\":\"info\",\"search_count\":\"info\",\"update_investigation_details_begin\":\"change\",\"update_investigation_details_end\":\"change\",\"update_preservation_rule_add_holds_begin\":\"change\",\"update_preservation_rule_add_holds_end\":\"change\",\"update_preservation_rule_query_begin\":\"change\",\"update_preservation_rule_query_end\":\"change\",\"update_preservation_rule_remove_holds_begin\":\"change\",\"update_preservation_rule_remove_holds_end\":\"change\",\"update_retention_rule_begin\":\"change\",\"update_retention_rule_end\":\"change\",\"update_retention_settings\":\"change\",\"update_saved_query_details_begin\":\"change\",\"update_saved_query_details_end\":\"change\",\"view_cross_matter_litigation_hold_report\":\"access\",\"view_custodian_litigation_hold_report\":\"access\",\"view_document\":\"access\",\"view_document_information\":\"access\",\"view_external_document\":\"access\",\"view_investigation\":\"access\",\"view_matter_audit_log\":\"access\",\"view_per_matter_litigation_hold_report\":\"access\",\"view_retention_policy\":\"access\",\"view_retention_settings\":\"access\",\"view_system_audit_log\":\"access\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_event_type_from_vault_name",
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

            if event.has_value("json.kind") {
                event.rename("json.kind", "google_workspace.kind")?;
            }

            if event.has_value("json.ownerDomain") {
                event.rename("json.ownerDomain", "google_workspace.organization.domain")?;
            }

            if event.has_value("json.etag") {
                event.rename("json.etag", "google_workspace.etag")?;
            }

            if event.has_value("json.actor.callerType") {
                event.rename(
                    "json.actor.callerType",
                    "google_workspace.actor.caller_type",
                )?;
            }

            if event.has_value("json.actor.email") {
                event.rename("json.actor.email", "google_workspace.actor.email")?;
            }

            if let Some(v) = event
                .get("google_workspace.actor.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.email", v)?;
            }

            if let Some(v) = event
                .get("google_workspace.actor.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            let _cond = {
                event.has_value("source.user.email")
                    && event.get("source.user.email").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("source.user.email") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("source.user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "source.user.email".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "dissect_user_email")?;
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

            if event.has_value("json.actor.key") {
                event.rename("json.actor.key", "google_workspace.actor.key")?;
            }

            if event.has_value("json.actor.profileId") {
                event.rename("json.actor.profileId", "google_workspace.actor.profile_id")?;
            }

            if let Some(v) = event
                .get("google_workspace.actor.profile_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
            }

            if let Some(v) = event
                .get("source.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user", v)?;
            }

            if event.has_value("json.id.applicationName") {
                event.rename(
                    "json.id.applicationName",
                    "google_workspace.id.application_name",
                )?;
            }

            if let Some(v) = event
                .get("google_workspace.id.application_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if event.has_value("json.id.customerId") {
                event.rename("json.id.customerId", "google_workspace.id.customer_id")?;
            }

            if let Some(v) = event
                .get("google_workspace.id.customer_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            let _cond =
                { event.has_value("json.id.time") && event.get_str("json.id.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.id.time") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => event.set("google_workspace.id.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.id.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_id_time")?;
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
                .get("google_workspace.id.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.id.uniqueQualifier") {
                event.rename(
                    "json.id.uniqueQualifier",
                    "google_workspace.id.unique_qualifier",
                )?;
            }

            if let Some(v) = event
                .get("google_workspace.id.unique_qualifier")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = { event.get_str("json.ipAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ipAddress") {
                        if let Some(val) = event.get("json.ipAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ipAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("google_workspace.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ipAddress_to_ip",
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
                .get("google_workspace.ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            if event.has_value("json.events.type") {
                event.rename("json.events.type", "google_workspace.vault.type")?;
            }

            if let Some(v) = event
                .get("google_workspace.vault.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        gsub_field(
                            event,
                            "event.action",
                            "event.action",
                            cached_regex!("_"),
                            "-",
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_event_action")?;
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
                .get("google_workspace.vault.target_user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.email", v)?;
            }

            if let Some(v) = event
                .get("google_workspace.vault.resource_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            if event.has_value("google_workspace.vault.query") {
                event.rename(
                    "google_workspace.vault.query",
                    "google_workspace.vault.query_raw",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.vault.query_raw") {
                    if let Some(kv_str) = event.get_string("google_workspace.vault.query_raw") {
                        for pair in kv_str.split(", ") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once(": ") else {
                                return Err(TransformError::ParseError {
                                    path: "google_workspace.vault.query_raw".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("google_workspace.vault.query.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "kv")?;
                event.set("_ingest.on_failure_processor_tag", "kv_vault_query_raw")?;
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

            if event.has_value("google_workspace.vault.query.Time zone") {
                event.rename(
                    "google_workspace.vault.query.Time zone",
                    "google_workspace.vault.query.time_zone",
                )?;
            }

            if event.has_value("google_workspace.vault.additional_details") {
                event.rename(
                    "google_workspace.vault.additional_details",
                    "google_workspace.vault.additional_details_raw",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.vault.additional_details_raw") {
                    if let Some(kv_str) =
                        event.get_string("google_workspace.vault.additional_details_raw")
                    {
                        for pair in cached_regex!("\\n").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once(": ") else {
                                return Err(TransformError::ParseError {
                                    path: "google_workspace.vault.additional_details_raw".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                let value = value.trim_matches(|c| "\\s\"".contains(c));
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!(
                                            "google_workspace.vault.additional_details.{}",
                                            key
                                        ),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "kv")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "kv_vault_additional_details_raw",
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
                if event.has_value(
                    "google_workspace.vault.additional_details.export_linked_drive_files",
                ) {
                    if let Some(val) = event
                        .get("google_workspace.vault.additional_details.export_linked_drive_files")
                    {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_workspace.vault.additional_details.export_linked_drive_files".into(),
                            message,
                        })?;
                        event.set(
                            "google_workspace.vault.additional_details.export_linked_drive_files",
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
                    "convert_export_linked_drive_files_to_boolean",
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
                if event.has_value("google_workspace.vault.additional_details.show_locker_content")
                {
                    if let Some(val) =
                        event.get("google_workspace.vault.additional_details.show_locker_content")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "google_workspace.vault.additional_details.show_locker_content"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "google_workspace.vault.additional_details.show_locker_content",
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
                    "convert_show_locker_content_to_boolean",
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
                if event.has_value("google_workspace.vault.additional_details.use_improved_export")
                {
                    if let Some(val) =
                        event.get("google_workspace.vault.additional_details.use_improved_export")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "google_workspace.vault.additional_details.use_improved_export"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "google_workspace.vault.additional_details.use_improved_export",
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
                    "convert_use_improved_export_to_boolean",
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

            let _cond = { event.has_value("google_workspace.actor.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_workspace.actor.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_workspace.vault.target_user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_workspace.vault.target_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_workspace.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("google_workspace.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
                event.remove("google_workspace.actor.email");
                event.remove("google_workspace.actor.profile_id");
                event.remove("google_workspace.id.application_name");
                event.remove("google_workspace.id.customer_id");
                event.remove("google_workspace.id.time");
                event.remove("google_workspace.id.unique_qualifier");
                event.remove("google_workspace.ip_address");
                event.remove("google_workspace.vault.resource_url");
                event.remove("google_workspace.vault.target_user");
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

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
                        "Processor '{}' {}failed with message '{}'",
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
