// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_file_endpoint` pipeline.
pub struct PipelineFileEndpoint;

impl Transform for PipelineFileEndpoint {
    fn name(&self) -> &str {
        "pipeline_file_endpoint"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let _cond = {
            event
                .get("beyondtrust_epm.event.file.owner")
                .is_some_and(|v| v.is_string())
        };
        if _cond {
            event.rename(
                "beyondtrust_epm.event.file.owner",
                "beyondtrust_epm.event.file.owner_keyword",
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value(
                "beyondtrust_epm.event.EPMWinMac.AuthorizationRequest.ControlAuthorization",
            ) {
                if let Some(val) = event.get(
                    "beyondtrust_epm.event.EPMWinMac.AuthorizationRequest.ControlAuthorization",
                ) {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.EPMWinMac.AuthorizationRequest.ControlAuthorization".into(),
                        message,
                    })?;
                    event.set(
                        "beyondtrust_epm.event.EPMWinMac.AuthorizationRequest.ControlAuthorization",
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
                "convert_EPMWinMac_AuthorizationRequest_ControlAuthorization_to_boolean",
            )?;
            event.remove(
                "beyondtrust_epm.event.EPMWinMac.AuthorizationRequest.ControlAuthorization",
            );
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

        let _cond =
            { event.has_value("beyondtrust_epm.event.EPMWinMac.AuthorizingUser.Identifier") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.EPMWinMac.AuthorizingUser.Identifier")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.EPMWinMac.AuthorizingUser.Name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.EPMWinMac.AuthorizingUser.Name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event.has_value(
                "beyondtrust_epm.event.EPMWinMac.Configuration.Message.Authentication.User",
            )
        };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.EPMWinMac.Configuration.Message.Authentication.User").map_or_else(String::new, template_to_string)))?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.EPMWinMac.Configuration.Rule.MatchedChild") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.EPMWinMac.Configuration.Rule.MatchedChild")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.EPMWinMac.Configuration.Rule.MatchedChild"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.EPMWinMac.Configuration.Rule.MatchedChild",
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
                "convert_EPMWinMac_Configuration_Rule_MatchedChild_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.EPMWinMac.Configuration.Rule.MatchedChild");
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.EPMWinMac.Configuration.Rule.OnDemand") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.EPMWinMac.Configuration.Rule.OnDemand")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.EPMWinMac.Configuration.Rule.OnDemand"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.EPMWinMac.Configuration.Rule.OnDemand",
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
                "convert_EPMWinMac_Configuration_Rule_OnDemand_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.EPMWinMac.Configuration.Rule.OnDemand");
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value(
                "beyondtrust_epm.event.EPMWinMac.Configuration.RuleScript.Outcome.RuleAffected",
            ) {
                if let Some(val) = event.get(
                    "beyondtrust_epm.event.EPMWinMac.Configuration.RuleScript.Outcome.RuleAffected",
                ) {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.EPMWinMac.Configuration.RuleScript.Outcome.RuleAffected".into(),
                        message,
                    })?;
                    event.set("beyondtrust_epm.event.EPMWinMac.Configuration.RuleScript.Outcome.RuleAffected", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_EPMWinMac_Configuration_RuleScript_Outcome_RuleAffected_to_boolean",
            )?;
            event.remove(
                "beyondtrust_epm.event.EPMWinMac.Configuration.RuleScript.Outcome.RuleAffected",
            );
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.EPMWinMac.Session.Administrator") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.EPMWinMac.Session.Administrator")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.EPMWinMac.Session.Administrator".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.EPMWinMac.Session.Administrator",
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
                "convert_EPMWinMac_Session_Administrator_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.EPMWinMac.Session.Administrator");
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

        let _cond = {
            event.has_value("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Message.Authentication.User")
        };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Message.Authentication.User").map_or_else(String::new, template_to_string)))?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value(
                "beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.MatchedChild",
            ) {
                if let Some(val) = event.get("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.MatchedChild") {
                let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.MatchedChild".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.MatchedChild", converted)?;
            }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_EPMWinMac_Session_JITAdmin_Configuration_Rule_MatchedChild_to_boolean",
            )?;
            event.remove(
                "beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.MatchedChild",
            );
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value(
                "beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.OnDemand",
            ) {
                if let Some(val) = event.get(
                    "beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.OnDemand",
                ) {
                    let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.OnDemand".into(),
                        message,
                    })?;
                    event.set("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.OnDemand", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_EPMWinMac_Session_JITAdmin_Configuration_Rule_OnDemand_to_boolean",
            )?;
            event.remove(
                "beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.Rule.OnDemand",
            );
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.RuleScript.Outcome.RuleAffected") {
            if let Some(val) = event.get("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.RuleScript.Outcome.RuleAffected") {
                let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.RuleScript.Outcome.RuleAffected".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.RuleScript.Outcome.RuleAffected", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_EPMWinMac_Session_JITAdmin_Configuration_RuleScript_Outcome_RuleAffected_to_boolean")?;
            event.remove("beyondtrust_epm.event.EPMWinMac.Session.JITAdmin.Configuration.RuleScript.Outcome.RuleAffected");
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

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.EPMWinMac.Session.PowerUser") {
                if let Some(val) = event.get("beyondtrust_epm.event.EPMWinMac.Session.PowerUser") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.EPMWinMac.Session.PowerUser".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.EPMWinMac.Session.PowerUser",
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
                "convert_EPMWinMac_Session_PowerUser_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.EPMWinMac.Session.PowerUser");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.code_signature.digest_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.code_signature.digest_algorithm", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.dll.code_signature.exists") {
                if let Some(val) = event.get("beyondtrust_epm.event.dll.code_signature.exists") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.dll.code_signature.exists".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.dll.code_signature.exists", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_dll_code_signature_exists_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.dll.code_signature.exists");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.code_signature.exists")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.code_signature.exists", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.code_signature.signing_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.code_signature.signing_id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.code_signature.status")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.code_signature.status", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.code_signature.subject_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.code_signature.subject_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.code_signature.team_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.code_signature.team_id", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.dll.code_signature.timestamp")
                && event.get_str("beyondtrust_epm.event.dll.code_signature.timestamp") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.dll.code_signature.timestamp")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event
                            .set("beyondtrust_epm.event.dll.code_signature.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.dll.code_signature.timestamp".into(),
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
                    "date_dll_code_signature_timestamp",
                )?;
                event.remove("beyondtrust_epm.event.dll.code_signature.timestamp");
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
            .get("beyondtrust_epm.event.dll.code_signature.timestamp")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.code_signature.timestamp", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.dll.code_signature.trusted") {
                if let Some(val) = event.get("beyondtrust_epm.event.dll.code_signature.trusted") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.dll.code_signature.trusted".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.dll.code_signature.trusted",
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
                "convert_dll_code_signature_trusted_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.dll.code_signature.trusted");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.code_signature.trusted")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.code_signature.trusted", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.dll.code_signature.valid") {
                if let Some(val) = event.get("beyondtrust_epm.event.dll.code_signature.valid") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.dll.code_signature.valid".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.dll.code_signature.valid", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_dll_code_signature_valid_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.dll.code_signature.valid");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.code_signature.valid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.code_signature.valid", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.hash.md5")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.hash.md5", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.dll.hash.md5") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.dll.hash.md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.hash.sha1")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.hash.sha1", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.dll.hash.sha1") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.dll.hash.sha1")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.hash.sha256")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.hash.sha256", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.dll.hash.sha256") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.dll.hash.sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.hash.sha384")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.hash.sha384", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.dll.hash.sha384") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.dll.hash.sha384")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.hash.sha512")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.hash.sha512", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.dll.hash.sha512") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.dll.hash.sha512")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.hash.ssdeep")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.hash.ssdeep", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.hash.tlsh")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.hash.tlsh", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.path", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.pe.architecture")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.pe.architecture", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.pe.company")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.pe.company", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.pe.description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.pe.description", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.pe.file_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.pe.file_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.pe.imphash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.pe.imphash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.dll.pe.imphash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.dll.pe.imphash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.pe.original_file_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.pe.original_file_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.pe.pehash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.pe.pehash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.dll.pe.pehash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.dll.pe.pehash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.dll.pe.product")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("dll.pe.product", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.file.accessed")
                && event.get_str("beyondtrust_epm.event.file.accessed") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.file.accessed") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("beyondtrust_epm.event.file.accessed", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.file.accessed".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_accessed")?;
                event.remove("beyondtrust_epm.event.file.accessed");
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
            .get("beyondtrust_epm.event.file.accessed")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.accessed", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.attributes")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.attributes", |event| {
                event.append_unique(
                    "file.attributes",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.code_signature.digest_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.code_signature.digest_algorithm", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.file.code_signature.exists") {
                if let Some(val) = event.get("beyondtrust_epm.event.file.code_signature.exists") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.file.code_signature.exists".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.file.code_signature.exists",
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
                "convert_file_code_signature_exists_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.file.code_signature.exists");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.code_signature.exists")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.code_signature.exists", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.code_signature.signing_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.code_signature.signing_id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.code_signature.status")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.code_signature.status", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.code_signature.subject_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.code_signature.subject_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.code_signature.team_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.code_signature.team_id", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.file.code_signature.timestamp")
                && event.get_str("beyondtrust_epm.event.file.code_signature.timestamp") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.file.code_signature.timestamp")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.file.code_signature.timestamp",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.file.code_signature.timestamp".into(),
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
                    "date_file_code_signature_timestamp",
                )?;
                event.remove("beyondtrust_epm.event.file.code_signature.timestamp");
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
            .get("beyondtrust_epm.event.file.code_signature.timestamp")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.code_signature.timestamp", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.file.code_signature.trusted") {
                if let Some(val) = event.get("beyondtrust_epm.event.file.code_signature.trusted") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.file.code_signature.trusted".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.file.code_signature.trusted",
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
                "convert_file_code_signature_trusted_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.file.code_signature.trusted");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.code_signature.trusted")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.code_signature.trusted", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.file.code_signature.valid") {
                if let Some(val) = event.get("beyondtrust_epm.event.file.code_signature.valid") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.file.code_signature.valid".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.file.code_signature.valid", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_file_code_signature_valid_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.file.code_signature.valid");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.code_signature.valid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.code_signature.valid", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.file.created")
                && event.get_str("beyondtrust_epm.event.file.created") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.file.created") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("beyondtrust_epm.event.file.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.file.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_created")?;
                event.remove("beyondtrust_epm.event.file.created");
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
            .get("beyondtrust_epm.event.file.created")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.created", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.file.ctime")
                && event.get_str("beyondtrust_epm.event.file.ctime") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.file.ctime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("beyondtrust_epm.event.file.ctime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.file.ctime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_ctime")?;
                event.remove("beyondtrust_epm.event.file.ctime");
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
            .get("beyondtrust_epm.event.file.ctime")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.ctime", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.device")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.device", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.directory")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.directory", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.drive_letter")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.drive_letter", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.architecture")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.architecture", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.byte_order")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.byte_order", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.cpu_type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.cpu_type", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.file.elf.creation_date")
                && event.get_str("beyondtrust_epm.event.file.elf.creation_date") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.file.elf.creation_date")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.file.elf.creation_date", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.file.elf.creation_date".into(),
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
                    "date_file_elf_creation_date",
                )?;
                event.remove("beyondtrust_epm.event.file.elf.creation_date");
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
            .get("beyondtrust_epm.event.file.elf.creation_date")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.creation_date", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.header.abi_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.header.abi_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.header.class")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.header.class", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.header.data")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.header.data", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.file.elf.header.entrypoint") {
                if let Some(val) = event.get("beyondtrust_epm.event.file.elf.header.entrypoint") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.file.elf.header.entrypoint".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.file.elf.header.entrypoint",
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
                "convert_file_elf_header_entrypoint_to_long",
            )?;
            event.remove("beyondtrust_epm.event.file.elf.header.entrypoint");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.header.entrypoint")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.header.entrypoint", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.header.object_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.header.object_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.header.os_abi")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.header.os_abi", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.header.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.header.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.header.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.header.version", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.chi2") {
                        if let Some(val) = event.get("_ingest._value.chi2") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.chi2".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.chi2", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_file_elf_sections_chi2_to_long",
                    )?;
                    event.remove("_ingest._value.chi2");
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
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.append_unique(
                    "file.elf.sections.chi2",
                    json!(
                        event
                            .get("_ingest._value.chi2")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.entropy") {
                        if let Some(val) = event.get("_ingest._value.entropy") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.entropy".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.entropy", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_file_elf_sections_entropy_to_long",
                    )?;
                    event.remove("_ingest._value.entropy");
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
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.append_unique(
                    "file.elf.sections.entropy",
                    json!(
                        event
                            .get("_ingest._value.entropy")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.append_unique(
                    "file.elf.sections.flags",
                    json!(
                        event
                            .get("_ingest._value.flags")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.append_unique(
                    "file.elf.sections.name",
                    json!(
                        event
                            .get("_ingest._value.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.append_unique(
                    "file.elf.sections.physical_offset",
                    json!(
                        event
                            .get("_ingest._value.physical_offset")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.physical_size") {
                        if let Some(val) = event.get("_ingest._value.physical_size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.physical_size".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.physical_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_file_elf_sections_physical_size_to_long",
                    )?;
                    event.remove("_ingest._value.physical_size");
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
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.append_unique(
                    "file.elf.sections.physical_size",
                    json!(
                        event
                            .get("_ingest._value.physical_size")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.append_unique(
                    "file.elf.sections.type",
                    json!(
                        event
                            .get("_ingest._value.type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.virtual_address") {
                        if let Some(val) = event.get("_ingest._value.virtual_address") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.virtual_address".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.virtual_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_file_elf_sections_virtual_address_to_long",
                    )?;
                    event.remove("_ingest._value.virtual_address");
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
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.append_unique(
                    "file.elf.sections.virtual_address",
                    json!(
                        event
                            .get("_ingest._value.virtual_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.virtual_size") {
                        if let Some(val) = event.get("_ingest._value.virtual_size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.virtual_size".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.virtual_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_file_elf_sections_virtual_size_to_long",
                    )?;
                    event.remove("_ingest._value.virtual_size");
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
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.append_unique(
                    "file.elf.sections.virtual_size",
                    json!(
                        event
                            .get("_ingest._value.virtual_size")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.segments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.segments", |event| {
                event.append_unique(
                    "file.elf.segments.sections",
                    json!(
                        event
                            .get("_ingest._value.sections")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.segments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.segments", |event| {
                event.append_unique(
                    "file.elf.segments.type",
                    json!(
                        event
                            .get("_ingest._value.type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.shared_libraries")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.elf.shared_libraries",
                |event| {
                    event.append_unique(
                        "file.elf.shared_libraries",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.elf.telfhash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.elf.telfhash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.elf.telfhash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.elf.telfhash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.extension")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.extension", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.fork_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.fork_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.gid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.gid", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.group")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.group", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.hash.md5")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.hash.md5", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.hash.md5") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.hash.md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.hash.sha1")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.hash.sha1", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.hash.sha1") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.hash.sha1")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.hash.sha256")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.hash.sha256", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.hash.sha256") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.hash.sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.hash.sha384")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.hash.sha384", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.hash.sha384") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.hash.sha384")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.hash.sha512")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.hash.sha512", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.hash.sha512") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.hash.sha512")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.hash.ssdeep")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.hash.ssdeep", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.hash.ssdeep") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.hash.ssdeep")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.hash.tlsh")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.hash.tlsh", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.hash.tlsh") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.hash.tlsh")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.inode")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.inode", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.mime_type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.mime_type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.mode")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.mode", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.file.mtime")
                && event.get_str("beyondtrust_epm.event.file.mtime") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.file.mtime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("beyondtrust_epm.event.file.mtime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.file.mtime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_file_mtime")?;
                event.remove("beyondtrust_epm.event.file.mtime");
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
            .get("beyondtrust_epm.event.file.mtime")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.mtime", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.path", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.pe.architecture")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.pe.architecture", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.pe.company")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.pe.company", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.pe.description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.pe.description", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.pe.file_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.pe.file_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.pe.imphash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.pe.imphash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.pe.imphash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.pe.imphash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.pe.original_file_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.pe.original_file_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.pe.pehash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.pe.pehash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.pe.pehash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.pe.pehash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.pe.product")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.pe.product", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.file.size") {
                if let Some(val) = event.get("beyondtrust_epm.event.file.size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.file.size".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.file.size", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_file_size_to_long",
            )?;
            event.remove("beyondtrust_epm.event.file.size");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.size")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.size", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.target_path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.target_path", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.uid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.uid", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.alternative_names")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.alternative_names",
                |event| {
                    event.append_unique(
                        "file.x509.alternative_names",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.issuer.common_name")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.issuer.common_name",
                |event| {
                    event.append_unique(
                        "file.x509.issuer.common_name",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.issuer.country")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.issuer.country",
                |event| {
                    event.append_unique(
                        "file.x509.issuer.country",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.x509.issuer.distinguished_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.issuer.distinguished_name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.issuer.locality")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.issuer.locality",
                |event| {
                    event.append_unique(
                        "file.x509.issuer.locality",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.issuer.organization")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.issuer.organization",
                |event| {
                    event.append_unique(
                        "file.x509.issuer.organization",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.issuer.organizational_unit")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.issuer.organizational_unit",
                |event| {
                    event.append_unique(
                        "file.x509.issuer.organizational_unit",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.issuer.state_or_province")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.issuer.state_or_province",
                |event| {
                    event.append_unique(
                        "file.x509.issuer.state_or_province",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.file.x509.not_after")
                && event.get_str("beyondtrust_epm.event.file.x509.not_after") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.file.x509.not_after")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.file.x509.not_after", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.file.x509.not_after".into(),
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
                    "date_file_x509_not_after",
                )?;
                event.remove("beyondtrust_epm.event.file.x509.not_after");
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
            .get("beyondtrust_epm.event.file.x509.not_after")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.not_after", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.file.x509.not_before")
                && event.get_str("beyondtrust_epm.event.file.x509.not_before") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.file.x509.not_before")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.file.x509.not_before", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.file.x509.not_before".into(),
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
                    "date_file_x509_not_before",
                )?;
                event.remove("beyondtrust_epm.event.file.x509.not_before");
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
            .get("beyondtrust_epm.event.file.x509.not_before")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.not_before", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.x509.public_key_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.public_key_algorithm", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.x509.public_key_curve")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.public_key_curve", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.file.x509.public_key_exponent") {
                if let Some(val) = event.get("beyondtrust_epm.event.file.x509.public_key_exponent")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.file.x509.public_key_exponent".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.file.x509.public_key_exponent",
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
                "convert_file_x509_public_key_exponent_to_long",
            )?;
            event.remove("beyondtrust_epm.event.file.x509.public_key_exponent");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.x509.public_key_exponent")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.public_key_exponent", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.file.x509.public_key_size") {
                if let Some(val) = event.get("beyondtrust_epm.event.file.x509.public_key_size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.file.x509.public_key_size".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.file.x509.public_key_size", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_file_x509_public_key_size_to_long",
            )?;
            event.remove("beyondtrust_epm.event.file.x509.public_key_size");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.x509.public_key_size")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.public_key_size", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.x509.serial_number")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.serial_number", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.x509.signature_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.signature_algorithm", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.subject.common_name")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.subject.common_name",
                |event| {
                    event.append_unique(
                        "file.x509.subject.common_name",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.subject.country")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.subject.country",
                |event| {
                    event.append_unique(
                        "file.x509.subject.country",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.x509.subject.distinguished_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.subject.distinguished_name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.subject.locality")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.subject.locality",
                |event| {
                    event.append_unique(
                        "file.x509.subject.locality",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.subject.organization")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.subject.organization",
                |event| {
                    event.append_unique(
                        "file.x509.subject.organization",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.subject.organizational_unit")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.subject.organizational_unit",
                |event| {
                    event.append_unique(
                        "file.x509.subject.organizational_unit",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.x509.subject.state_or_province")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.file.x509.subject.state_or_province",
                |event| {
                    event.append_unique(
                        "file.x509.subject.state_or_province",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.x509.version_number")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("file.x509.version_number", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.architecture")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.architecture", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.build_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.build_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.checksum")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.checksum", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.description", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.install_scope")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.install_scope", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.package.installed")
                && event.get_str("beyondtrust_epm.event.package.installed") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.package.installed")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.package.installed", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.package.installed".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_package_installed")?;
                event.remove("beyondtrust_epm.event.package.installed");
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
            .get("beyondtrust_epm.event.package.installed")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.installed", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.license")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.license", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.path", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.reference")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.reference", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.package.size") {
                if let Some(val) = event.get("beyondtrust_epm.event.package.size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.package.size".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.package.size", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_package_size_to_long",
            )?;
            event.remove("beyondtrust_epm.event.package.size");
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

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.size")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.size", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.package.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("package.version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.registry.data.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("registry.data.bytes", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.registry.data.strings")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.registry.data.strings",
                |event| {
                    event.append_unique(
                        "registry.data.strings",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.registry.data.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("registry.data.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.registry.hive")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("registry.hive", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.registry.key")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("registry.key", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.registry.path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("registry.path", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.registry.value")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("registry.value", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.Owner.Name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("file.owner") {
                event.set("file.owner", v)?;
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.file.Owner.Name") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.file.Owner.Name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.file.SourceUrl")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("file.origin_url") {
                event.set("file.origin_url", v)?;
            }
        }

        let _cond = {
            event
                .get("file.elf.sections.chi2")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "file.elf.sections.chi2", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })();
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("file.elf.sections.entropy")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "file.elf.sections.entropy", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })();
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("file.elf.sections.physical_size")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "file.elf.sections.physical_size", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })();
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("file.elf.sections.virtual_address")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "file.elf.sections.virtual_address", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })();
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("file.elf.sections.virtual_size")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "file.elf.sections.virtual_size", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })();
                Ok(())
            })?;
        }

        let _cond = {
            event.has_value("file")
                && (!event.has_value("event.category")
                    || !(event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("file")),
                        serde_json::Value::String(s) => s.contains("file"),
                        _ => false,
                    })))
        };
        if _cond {
            event.append_unique("event.category", json!("file"))?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.sections", |event| {
                event.remove("_ingest._value.chi2");
                event.remove("_ingest._value.entropy");
                event.remove("_ingest._value.flags");
                event.remove("_ingest._value.name");
                event.remove("_ingest._value.physical_offset");
                event.remove("_ingest._value.physical_size");
                event.remove("_ingest._value.type");
                event.remove("_ingest._value.virtual_address");
                event.remove("_ingest._value.virtual_size");
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.file.elf.segments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.file.elf.segments", |event| {
                event.remove("_ingest._value.sections");
                event.remove("_ingest._value.type");
                Ok(())
            })?;
        }

        event.remove("beyondtrust_epm.event.dll.code_signature.digest_algorithm");
        event.remove("beyondtrust_epm.event.dll.code_signature.exists");
        event.remove("beyondtrust_epm.event.dll.code_signature.signing_id");
        event.remove("beyondtrust_epm.event.dll.code_signature.status");
        event.remove("beyondtrust_epm.event.dll.code_signature.subject_name");
        event.remove("beyondtrust_epm.event.dll.code_signature.team_id");
        event.remove("beyondtrust_epm.event.dll.code_signature.timestamp");
        event.remove("beyondtrust_epm.event.dll.code_signature.trusted");
        event.remove("beyondtrust_epm.event.dll.code_signature.valid");
        event.remove("beyondtrust_epm.event.dll.hash.md5");
        event.remove("beyondtrust_epm.event.dll.hash.sha1");
        event.remove("beyondtrust_epm.event.dll.hash.sha256");
        event.remove("beyondtrust_epm.event.dll.hash.sha384");
        event.remove("beyondtrust_epm.event.dll.hash.sha512");
        event.remove("beyondtrust_epm.event.dll.hash.ssdeep");
        event.remove("beyondtrust_epm.event.dll.hash.tlsh");
        event.remove("beyondtrust_epm.event.dll.name");
        event.remove("beyondtrust_epm.event.dll.path");
        event.remove("beyondtrust_epm.event.dll.pe.architecture");
        event.remove("beyondtrust_epm.event.dll.pe.company");
        event.remove("beyondtrust_epm.event.dll.pe.description");
        event.remove("beyondtrust_epm.event.dll.pe.file_version");
        event.remove("beyondtrust_epm.event.dll.pe.imphash");
        event.remove("beyondtrust_epm.event.dll.pe.original_file_name");
        event.remove("beyondtrust_epm.event.dll.pe.pehash");
        event.remove("beyondtrust_epm.event.dll.pe.product");
        event.remove("beyondtrust_epm.event.file.accessed");
        event.remove("beyondtrust_epm.event.file.attributes");
        event.remove("beyondtrust_epm.event.file.code_signature.digest_algorithm");
        event.remove("beyondtrust_epm.event.file.code_signature.exists");
        event.remove("beyondtrust_epm.event.file.code_signature.signing_id");
        event.remove("beyondtrust_epm.event.file.code_signature.status");
        event.remove("beyondtrust_epm.event.file.code_signature.subject_name");
        event.remove("beyondtrust_epm.event.file.code_signature.team_id");
        event.remove("beyondtrust_epm.event.file.code_signature.timestamp");
        event.remove("beyondtrust_epm.event.file.code_signature.trusted");
        event.remove("beyondtrust_epm.event.file.code_signature.valid");
        event.remove("beyondtrust_epm.event.file.created");
        event.remove("beyondtrust_epm.event.file.ctime");
        event.remove("beyondtrust_epm.event.file.device");
        event.remove("beyondtrust_epm.event.file.directory");
        event.remove("beyondtrust_epm.event.file.drive_letter");
        event.remove("beyondtrust_epm.event.file.elf.architecture");
        event.remove("beyondtrust_epm.event.file.elf.byte_order");
        event.remove("beyondtrust_epm.event.file.elf.cpu_type");
        event.remove("beyondtrust_epm.event.file.elf.creation_date");
        event.remove("beyondtrust_epm.event.file.elf.header.abi_version");
        event.remove("beyondtrust_epm.event.file.elf.header.class");
        event.remove("beyondtrust_epm.event.file.elf.header.data");
        event.remove("beyondtrust_epm.event.file.elf.header.entrypoint");
        event.remove("beyondtrust_epm.event.file.elf.header.object_version");
        event.remove("beyondtrust_epm.event.file.elf.header.os_abi");
        event.remove("beyondtrust_epm.event.file.elf.header.type");
        event.remove("beyondtrust_epm.event.file.elf.header.version");
        event.remove("beyondtrust_epm.event.file.elf.shared_libraries");
        event.remove("beyondtrust_epm.event.file.elf.telfhash");
        event.remove("beyondtrust_epm.event.file.extension");
        event.remove("beyondtrust_epm.event.file.fork_name");
        event.remove("beyondtrust_epm.event.file.gid");
        event.remove("beyondtrust_epm.event.file.group");
        event.remove("beyondtrust_epm.event.file.hash.md5");
        event.remove("beyondtrust_epm.event.file.hash.sha1");
        event.remove("beyondtrust_epm.event.file.hash.sha256");
        event.remove("beyondtrust_epm.event.file.hash.sha384");
        event.remove("beyondtrust_epm.event.file.hash.sha512");
        event.remove("beyondtrust_epm.event.file.hash.ssdeep");
        event.remove("beyondtrust_epm.event.file.hash.tlsh");
        event.remove("beyondtrust_epm.event.file.inode");
        event.remove("beyondtrust_epm.event.file.mime_type");
        event.remove("beyondtrust_epm.event.file.mode");
        event.remove("beyondtrust_epm.event.file.mtime");
        event.remove("beyondtrust_epm.event.file.name");
        event.remove("beyondtrust_epm.event.file.path");
        event.remove("beyondtrust_epm.event.file.pe.architecture");
        event.remove("beyondtrust_epm.event.file.pe.company");
        event.remove("beyondtrust_epm.event.file.pe.description");
        event.remove("beyondtrust_epm.event.file.pe.file_version");
        event.remove("beyondtrust_epm.event.file.pe.imphash");
        event.remove("beyondtrust_epm.event.file.pe.original_file_name");
        event.remove("beyondtrust_epm.event.file.pe.pehash");
        event.remove("beyondtrust_epm.event.file.pe.product");
        event.remove("beyondtrust_epm.event.file.size");
        event.remove("beyondtrust_epm.event.file.target_path");
        event.remove("beyondtrust_epm.event.file.type");
        event.remove("beyondtrust_epm.event.file.uid");
        event.remove("beyondtrust_epm.event.file.x509.alternative_names");
        event.remove("beyondtrust_epm.event.file.x509.issuer.common_name");
        event.remove("beyondtrust_epm.event.file.x509.issuer.country");
        event.remove("beyondtrust_epm.event.file.x509.issuer.distinguished_name");
        event.remove("beyondtrust_epm.event.file.x509.issuer.locality");
        event.remove("beyondtrust_epm.event.file.x509.issuer.organization");
        event.remove("beyondtrust_epm.event.file.x509.issuer.organizational_unit");
        event.remove("beyondtrust_epm.event.file.x509.issuer.state_or_province");
        event.remove("beyondtrust_epm.event.file.x509.not_after");
        event.remove("beyondtrust_epm.event.file.x509.not_before");
        event.remove("beyondtrust_epm.event.file.x509.public_key_algorithm");
        event.remove("beyondtrust_epm.event.file.x509.public_key_curve");
        event.remove("beyondtrust_epm.event.file.x509.public_key_exponent");
        event.remove("beyondtrust_epm.event.file.x509.public_key_size");
        event.remove("beyondtrust_epm.event.file.x509.serial_number");
        event.remove("beyondtrust_epm.event.file.x509.signature_algorithm");
        event.remove("beyondtrust_epm.event.file.x509.subject.common_name");
        event.remove("beyondtrust_epm.event.file.x509.subject.country");
        event.remove("beyondtrust_epm.event.file.x509.subject.distinguished_name");
        event.remove("beyondtrust_epm.event.file.x509.subject.locality");
        event.remove("beyondtrust_epm.event.file.x509.subject.organization");
        event.remove("beyondtrust_epm.event.file.x509.subject.organizational_unit");
        event.remove("beyondtrust_epm.event.file.x509.subject.state_or_province");
        event.remove("beyondtrust_epm.event.file.x509.version_number");
        event.remove("beyondtrust_epm.event.package.architecture");
        event.remove("beyondtrust_epm.event.package.build_version");
        event.remove("beyondtrust_epm.event.package.checksum");
        event.remove("beyondtrust_epm.event.package.description");
        event.remove("beyondtrust_epm.event.package.install_scope");
        event.remove("beyondtrust_epm.event.package.installed");
        event.remove("beyondtrust_epm.event.package.license");
        event.remove("beyondtrust_epm.event.package.name");
        event.remove("beyondtrust_epm.event.package.path");
        event.remove("beyondtrust_epm.event.package.reference");
        event.remove("beyondtrust_epm.event.package.size");
        event.remove("beyondtrust_epm.event.package.type");
        event.remove("beyondtrust_epm.event.package.version");
        event.remove("beyondtrust_epm.event.registry.data.bytes");
        event.remove("beyondtrust_epm.event.registry.data.strings");
        event.remove("beyondtrust_epm.event.registry.data.type");
        event.remove("beyondtrust_epm.event.registry.hive");
        event.remove("beyondtrust_epm.event.registry.key");
        event.remove("beyondtrust_epm.event.registry.path");
        event.remove("beyondtrust_epm.event.registry.value");

        Ok(TransformResult::Continue)
    }
}
