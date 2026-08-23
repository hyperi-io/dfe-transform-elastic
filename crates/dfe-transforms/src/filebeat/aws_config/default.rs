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
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

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

            event.set("cloud.provider", json!("aws"))?;

            event.set("observer.vendor", json!("AWS Config"))?;

            event.append("event.category", json!("configuration"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            if event.has("json.Annotation") {
                event.rename("json.Annotation", "aws.config.annotation")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.ConfigRuleInfo.ConfigRuleArn") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(":") else {
                            break 'dissect false;
                        };
                        captured.push(("cloud.account.id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(":") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("cloud.account.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.name", v)?;
            }

            if event.has("json.ConfigRuleInfo.ConfigRuleArn") {
                event.rename(
                    "json.ConfigRuleInfo.ConfigRuleArn",
                    "aws.config.rule_info.config_rule_arn",
                )?;
            }

            if let Some(v) = event
                .get("aws.config.rule_info.config_rule_arn")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.reference", v)?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("rule.reference") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("rule.uuid", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has("json.ConfigRuleInfo.ConfigRuleId") {
                event.rename(
                    "json.ConfigRuleInfo.ConfigRuleId",
                    "aws.config.rule_info.config_rule_id",
                )?;
            }

            if let Some(v) = event
                .get("aws.config.rule_info.config_rule_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has("json.ConfigRuleInfo.ConfigRuleName") {
                event.rename(
                    "json.ConfigRuleInfo.ConfigRuleName",
                    "aws.config.rule_info.config_rule_name",
                )?;
            }

            if let Some(v) = event
                .get("aws.config.rule_info.config_rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has("json.ConfigRuleInfo.ConfigRuleState") {
                event.rename(
                    "json.ConfigRuleInfo.ConfigRuleState",
                    "aws.config.rule_info.config_rule_state",
                )?;
            }

            let _cond = { event.has_value("json.ConfigRuleInfo.CreatedBy") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.ConfigRuleInfo.CreatedBy")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.ConfigRuleInfo.CreatedBy") {
                event.rename(
                    "json.ConfigRuleInfo.CreatedBy",
                    "aws.config.rule_info.created_by",
                )?;
            }

            if event.has("json.ConfigRuleInfo.Description") {
                event.rename(
                    "json.ConfigRuleInfo.Description",
                    "aws.config.rule_info.description",
                )?;
            }

            if let Some(v) = event
                .get("aws.config.rule_info.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
            }

            if let Some(v) = event
                .get("aws.config.rule_info.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event
                    .get("json.ConfigRuleInfo.EvaluationModes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.ConfigRuleInfo.EvaluationModes", |event| {
                    if event.has("_ingest._value.Mode") {
                        event.rename("_ingest._value.Mode", "_ingest._value.mode")?;
                    }
                    Ok(())
                })?;
            }

            if event.has("json.ConfigRuleInfo.EvaluationModes") {
                event.rename(
                    "json.ConfigRuleInfo.EvaluationModes",
                    "aws.config.rule_info.evaluation_modes",
                )?;
            }

            let _cond = {
                event.has_value("json.ConfigRuleInfo.InputParameters")
                    && event.get_str("json.ConfigRuleInfo.InputParameters") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "json.ConfigRuleInfo.InputParameters",
                        "aws.config.rule_info.input_parameters",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_ConfigRuleInfo_InputParameters",
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

            if event.has("json.ConfigRuleInfo.MaximumExecutionFrequency") {
                event.rename(
                    "json.ConfigRuleInfo.MaximumExecutionFrequency",
                    "aws.config.rule_info.maximum_execution_frequency",
                )?;
            }

            if event.has("json.ConfigRuleInfo.Scope.ComplianceResourceId") {
                event.rename(
                    "json.ConfigRuleInfo.Scope.ComplianceResourceId",
                    "aws.config.rule_info.scope.compliance_resource_id",
                )?;
            }

            if event.has("json.ConfigRuleInfo.Scope.ComplianceResourceTypes") {
                event.rename(
                    "json.ConfigRuleInfo.Scope.ComplianceResourceTypes",
                    "aws.config.rule_info.scope.compliance_resource_types",
                )?;
            }

            if event.has("json.ConfigRuleInfo.Scope.TagKey") {
                event.rename(
                    "json.ConfigRuleInfo.Scope.TagKey",
                    "aws.config.rule_info.scope.tag_key",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.ConfigRuleInfo.Source.CustomPolicyDetails.EnableDebugLogDelivery",
                ) {
                    if let Some(val) = event.get(
                        "json.ConfigRuleInfo.Source.CustomPolicyDetails.EnableDebugLogDelivery",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.ConfigRuleInfo.Source.CustomPolicyDetails.EnableDebugLogDelivery".into(),
                            message,
                        })?;
                        event.set("aws.config.rule_info.source.custom_policy_details.enable_debug_log_delivery", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ConfigRuleInfo_Source_CustomPolicyDetails_EnableDebugLogDelivery_to_boolean")?;
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

            if event.has("json.ConfigRuleInfo.Source.CustomPolicyDetails.PolicyRuntime") {
                event.rename(
                    "json.ConfigRuleInfo.Source.CustomPolicyDetails.PolicyRuntime",
                    "aws.config.rule_info.source.custom_policy_details.policy_runtime",
                )?;
            }

            if event.has("json.ConfigRuleInfo.Source.CustomPolicyDetails.PolicyText") {
                event.rename(
                    "json.ConfigRuleInfo.Source.CustomPolicyDetails.PolicyText",
                    "aws.config.rule_info.source.custom_policy_details.policy_text",
                )?;
            }

            if event.has("json.ConfigRuleInfo.Source.Owner") {
                event.rename(
                    "json.ConfigRuleInfo.Source.Owner",
                    "aws.config.rule_info.source.owner",
                )?;
            }

            let _cond = {
                event
                    .get("json.ConfigRuleInfo.Source.SourceDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.ConfigRuleInfo.Source.SourceDetails", |event| {
                    if event.has("_ingest._value.EventSource") {
                        event
                            .rename("_ingest._value.EventSource", "_ingest._value.event_source")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.ConfigRuleInfo.Source.SourceDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.ConfigRuleInfo.Source.SourceDetails", |event| {
                    if event.has("_ingest._value.MaximumExecutionFrequency") {
                        event.rename(
                            "_ingest._value.MaximumExecutionFrequency",
                            "_ingest._value.maximum_execution_frequency",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.ConfigRuleInfo.Source.SourceDetails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.ConfigRuleInfo.Source.SourceDetails", |event| {
                    if event.has("_ingest._value.MessageType") {
                        event
                            .rename("_ingest._value.MessageType", "_ingest._value.message_type")?;
                    }
                    Ok(())
                })?;
            }

            if event.has("json.ConfigRuleInfo.Source.SourceDetails") {
                event.rename(
                    "json.ConfigRuleInfo.Source.SourceDetails",
                    "aws.config.rule_info.source.source_details",
                )?;
            }

            if event.has("json.ConfigRuleInfo.Source.SourceIdentifier") {
                event.rename(
                    "json.ConfigRuleInfo.Source.SourceIdentifier",
                    "aws.config.rule_info.source.source_identifier",
                )?;
            }

            let _cond = {
                event.has_value("json.ConfigRuleInvokedTime")
                    && event.get_str("json.ConfigRuleInvokedTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ConfigRuleInvokedTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS", "UNIX"], None, None)
                        {
                            event.set("aws.config.config_rule_invoked_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_ConfigRuleInvokedTime",
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

            if event.has("json.EvaluationResultIdentifier.EvaluationResultQualifier.ConfigRuleName")
            {
                event.rename("json.EvaluationResultIdentifier.EvaluationResultQualifier.ConfigRuleName", "aws.config.evaluation_result_identifier.evaluation_result_qualifier.config_rule_name")?;
            }

            if event.has("json.EvaluationResultIdentifier.EvaluationResultQualifier.EvaluationMode")
            {
                event.rename("json.EvaluationResultIdentifier.EvaluationResultQualifier.EvaluationMode", "aws.config.evaluation_result_identifier.evaluation_result_qualifier.evaluation_mode")?;
            }

            let _cond = {
                event.has_value("json.EvaluationResultIdentifier.OrderingTimestamp")
                    && event.get_str("json.EvaluationResultIdentifier.OrderingTimestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.EvaluationResultIdentifier.OrderingTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS", "UNIX"], None, None)
                        {
                            event.set(
                                "aws.config.evaluation_result_identifier.ordering_timestamp",
                                parsed,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_EvaluationResultIdentifier_OrderingTimestamp",
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
                .get("aws.config.evaluation_result_identifier.ordering_timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has("json.EvaluationResultIdentifier.ResourceEvaluationId") {
                event.rename(
                    "json.EvaluationResultIdentifier.ResourceEvaluationId",
                    "aws.config.evaluation_result_identifier.resource_evaluation_id",
                )?;
            }

            if let Some(v) = event
                .get("aws.config.evaluation_result_identifier.resource_evaluation_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("json.ResultRecordedTime")
                    && event.get_str("json.ResultRecordedTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ResultRecordedTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS", "UNIX"], None, None)
                        {
                            event.set("aws.config.result_recorded_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_ResultRecordedTime",
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

            if event.has("json.ResultToken") {
                event.rename("json.ResultToken", "aws.config.result_token")?;
            }

            if event.has("json.EvaluationResultIdentifier.EvaluationResultQualifier.ResourceId") {
                event.rename("json.EvaluationResultIdentifier.EvaluationResultQualifier.ResourceId", "aws.config.evaluation_result_identifier.evaluation_result_qualifier.resource_id")?;
            }

            if let Some(v) = event.get("aws.config.evaluation_result_identifier.evaluation_result_qualifier.resource_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("resource.id", v)?;
            }

            if event.has("json.EvaluationResultIdentifier.EvaluationResultQualifier.ResourceType") {
                event.rename("json.EvaluationResultIdentifier.EvaluationResultQualifier.ResourceType", "aws.config.evaluation_result_identifier.evaluation_result_qualifier.resource_type")?;
            }

            if let Some(v) = event.get("aws.config.evaluation_result_identifier.evaluation_result_qualifier.resource_type").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("resource.type", v)?;
            }

            if event.has("json.ComplianceType") {
                event.rename("json.ComplianceType", "aws.config.compliance_type")?;
            }

            let _cond = {
                event
                    .get("aws.config.compliance_type")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // Painless script
                // Source: if (ctx.aws.config.compliance_type == 'NON_COMPLIANT') {\n  ctx.event.outcome = 'failure';\n} else if (ctx.aws.config.compliance_type == 'COMPLIANT') {\n  ctx.event.outcome = 'success';\n} else {\n  ctx.event.outcome = 'unknown';\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.aws.config.compliance_type == 'NON_COMPLIANT') {\n  ctx.event.outcome = 'failure';\n} else if (ctx.aws.config.compliance_type == 'COMPLIANT') {\n  ctx.event.outcome = 'success';\n} else {\n  ctx.event.outcome = 'unknown';\n}"#
                    ),
                )?;
            }

            let _cond = { event.get_str("aws.config.compliance_type") == Some("COMPLIANT") };
            if _cond {
                let v = json!("passed");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = { event.get_str("aws.config.compliance_type") == Some("NON_COMPLIANT") };
            if _cond {
                let v = json!("failed");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = { !event.has_value("result.evaluation") };
            if _cond {
                let v = json!("unknown");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            if event.has("json.ConfigRuleInfo.Scope.TagValue") {
                event.rename(
                    "json.ConfigRuleInfo.Scope.TagValue",
                    "aws.config.rule_info.scope.tag_value",
                )?;
            }

            if let Some(v) = event
                .get("aws.config.rule_info.scope.tag_value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.tags", v)?;
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
                event.remove("aws.config.evaluation_result_identifier.ordering_timestamp");
                event.remove("aws.config.evaluation_result_identifier.resource_evaluation_id");
                event.remove("aws.config.rule_info.config_rule_arn");
                event.remove("aws.config.rule_info.config_rule_id");
                event.remove("aws.config.rule_info.config_rule_name");
                event.remove("aws.config.rule_info.description");
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
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
