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

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("vulnerability"))?;

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("Amazon Inspector"))?;

            event.set("vulnerability.scanner.vendor", json!("Amazon Inspector"))?;

            event.remove("cloud");

            event.set("cloud.provider", json!("aws"))?;

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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.has_value("json.updatedAt") && event.get_str("json.updatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updatedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.inspector.updated_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_updatedAt")?;
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
                .get("aws.inspector.updated_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has("json.description") {
                event.rename("json.description", "aws.inspector.description")?;
            }

            if let Some(v) = event
                .get("aws.inspector.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if let Some(v) = event
                .get("aws.inspector.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.description", v)?;
            }

            if event.has("json.awsAccountId") {
                event.rename("json.awsAccountId", "aws.inspector.aws_account_id")?;
            }

            if let Some(v) = event
                .get("aws.inspector.aws_account_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if event.has("json.severity") {
                event.rename("json.severity", "aws.inspector.severity")?;
            }

            let _cond = { event.has_value("aws.inspector.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String severity = ctx.aws.inspector.severity.toLowerCase(); if (severity == 'untriaged') {\n  ctx.vulnerability.put('severity', 'Unknown');\n} else if (severity == 'informational') {\n  ctx.vulnerability.put('severity', 'Low');\n} else if (severity == 'low') {\n  ctx.vulnerability.put('severity', 'Low');\n} else if (severity == 'medium') {\n  ctx.vulnerability.put('severity', 'Medium');\n} else if (severity == 'high') {\n  ctx.vulnerability.put('severity', 'High');\n} else if (severity == 'critical') {\n  ctx.vulnerability.put('severity', 'Critical');\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String severity = ctx.aws.inspector.severity.toLowerCase(); if (severity == 'untriaged') {\n  ctx.vulnerability.put('severity', 'Unknown');\n} else if (severity == 'informational') {\n  ctx.vulnerability.put('severity', 'Low');\n} else if (severity == 'low') {\n  ctx.vulnerability.put('severity', 'Low');\n} else if (severity == 'medium') {\n  ctx.vulnerability.put('severity', 'Medium');\n} else if (severity == 'high') {\n  ctx.vulnerability.put('severity', 'High');\n} else if (severity == 'critical') {\n  ctx.vulnerability.put('severity', 'Critical');\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_map_severity_to_CVSS",
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

            if event.has("json.networkReachabilityDetails.protocol") {
                event.rename(
                    "json.networkReachabilityDetails.protocol",
                    "aws.inspector.network_reachability_details.protocol",
                )?;
            }

            if let Some(v) = event
                .get("aws.inspector.network_reachability_details.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.transport", v)?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            if event.has("json.codeVulnerabilityDetails.cwes") {
                event.rename(
                    "json.codeVulnerabilityDetails.cwes",
                    "aws.inspector.code_vulnerability_details.cwes",
                )?;
            }

            if event.has("json.codeVulnerabilityDetails.detectorId") {
                event.rename(
                    "json.codeVulnerabilityDetails.detectorId",
                    "aws.inspector.code_vulnerability_details.detector_id",
                )?;
            }

            if event.has("json.codeVulnerabilityDetails.detectorName") {
                event.rename(
                    "json.codeVulnerabilityDetails.detectorName",
                    "aws.inspector.code_vulnerability_details.detector_name",
                )?;
            }

            if event.has("json.codeVulnerabilityDetails.detectorTags") {
                event.rename(
                    "json.codeVulnerabilityDetails.detectorTags",
                    "aws.inspector.code_vulnerability_details.detector_tags",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.codeVulnerabilityDetails.filePath.endLine") {
                    if let Some(val) = event.get("json.codeVulnerabilityDetails.filePath.endLine") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.codeVulnerabilityDetails.filePath.endLine".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.inspector.code_vulnerability_details.file_path.end_line",
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
                    "convert_codeVulnerabilityDetails_filePath_endLine_to_long",
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

            if event.has("json.codeVulnerabilityDetails.filePath.fileName") {
                event.rename(
                    "json.codeVulnerabilityDetails.filePath.fileName",
                    "aws.inspector.code_vulnerability_details.file_path.name",
                )?;
            }

            if event.has("json.codeVulnerabilityDetails.filePath.filePath") {
                event.rename(
                    "json.codeVulnerabilityDetails.filePath.filePath",
                    "aws.inspector.code_vulnerability_details.file_path.path",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.codeVulnerabilityDetails.filePath.startLine") {
                    if let Some(val) = event.get("json.codeVulnerabilityDetails.filePath.startLine")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.codeVulnerabilityDetails.filePath.startLine".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.inspector.code_vulnerability_details.file_path.start_line",
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
                    "convert_codeVulnerabilityDetails_filePath_startLine_to_long",
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

            if event.has("json.codeVulnerabilityDetails.referenceUrls") {
                event.rename(
                    "json.codeVulnerabilityDetails.referenceUrls",
                    "aws.inspector.code_vulnerability_details.reference_urls",
                )?;
            }

            if event.has("json.codeVulnerabilityDetails.ruleId") {
                event.rename(
                    "json.codeVulnerabilityDetails.ruleId",
                    "aws.inspector.code_vulnerability_details.rule_id",
                )?;
            }

            if event.has("json.codeVulnerabilityDetails.sourceLambdaLayerArn") {
                event.rename(
                    "json.codeVulnerabilityDetails.sourceLambdaLayerArn",
                    "aws.inspector.code_vulnerability_details.source_lambda_layer_arn",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.epss.score") {
                    if let Some(val) = event.get("json.epss.score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.epss.score".into(),
                                message,
                            }
                        })?;
                        event.set("aws.inspector.epss.score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_epss_score_to_double",
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

            let _cond = {
                event.has_value("json.exploitabilityDetails.lastKnownExploitAt")
                    && event.get_str("json.exploitabilityDetails.lastKnownExploitAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.exploitabilityDetails.lastKnownExploitAt")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.inspector.exploitability_details.last_known_exploit_at",
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
                        "date_exploitabilityDetails_lastKnownExploitAt",
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

            if event.has("json.exploitAvailable") {
                event.rename("json.exploitAvailable", "aws.inspector.exploit_available")?;
            }

            if event.has("json.packageVulnerabilityDetails.referenceUrls") {
                event.rename(
                    "json.packageVulnerabilityDetails.referenceUrls",
                    "aws.inspector.package_vulnerability_details.reference_urls",
                )?;
            }

            if let Some(v) = event
                .get("aws.inspector.package_vulnerability_details.reference_urls")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.reference", v)?;
            }

            if event.has("json.packageVulnerabilityDetails.vulnerabilityId") {
                event.rename(
                    "json.packageVulnerabilityDetails.vulnerabilityId",
                    "aws.inspector.package_vulnerability_details.vulnerability_id",
                )?;
            }

            if let Some(v) = event
                .get("aws.inspector.package_vulnerability_details.vulnerability_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.id", v)?;
            }

            let _cond = {
                event.get_str("aws.inspector.type") == Some("PACKAGE_VULNERABILITY")
                    && event
                        .get_str("aws.inspector.package_vulnerability_details.vulnerability_id")
                        .is_some_and(|s| s.starts_with("CVE"))
            };
            if _cond {
                if let Some(v) = event
                    .get("aws.inspector.package_vulnerability_details.vulnerability_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.cve", v)?;
                }
            }

            let _cond = { event.has_value("vulnerability.cve") };
            if _cond {
                event.set("vulnerability.enumeration", json!("CVE"))?;
            }

            if event.has("json.findingArn") {
                event.rename("json.findingArn", "aws.inspector.finding_arn")?;
            }

            let _cond = {
                event.has_value("json.firstObservedAt")
                    && event.get_str("json.firstObservedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.firstObservedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.inspector.first_observed_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_firstObservedAt")?;
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

            if event.has("json.fixAvailable") {
                event.rename("json.fixAvailable", "aws.inspector.fix_available")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.inspectorScore") {
                    if let Some(val) = event.get("json.inspectorScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.inspectorScore".into(),
                                message,
                            }
                        })?;
                        event.set("aws.inspector.inspector_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_inspectorScore_to_double",
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

            if event.has("json.inspectorScoreDetails.adjustedCvss.adjustments") {
                event.rename(
                    "json.inspectorScoreDetails.adjustedCvss.adjustments",
                    "aws.inspector.inspector_score_details.adjusted_cvss.adjustments",
                )?;
            }

            if event.has("json.inspectorScoreDetails.adjustedCvss.cvssSource") {
                event.rename(
                    "json.inspectorScoreDetails.adjustedCvss.cvssSource",
                    "aws.inspector.inspector_score_details.adjusted_cvss.cvss_source",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.inspectorScoreDetails.adjustedCvss.score") {
                    if let Some(val) = event.get("json.inspectorScoreDetails.adjustedCvss.score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.inspectorScoreDetails.adjustedCvss.score".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.inspector.inspector_score_details.adjusted_cvss.score.value",
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
                    "convert_inspectorScoreDetails_adjustedCvss_score_to_double",
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

            if let Some(v) = event
                .get("aws.inspector.inspector_score_details.adjusted_cvss.score.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.base", v)?;
            }

            let _cond = {
                event.get_str("aws.inspector.type") == Some("PACKAGE_VULNERABILITY")
                    && event.has_value(
                        "aws.inspector.inspector_score_details.adjusted_cvss.score.value",
                    )
            };
            if _cond {
                event.set("vulnerability.classification", json!("CVSS"))?;
            }

            if event.has("json.inspectorScoreDetails.adjustedCvss.scoreSource") {
                event.rename(
                    "json.inspectorScoreDetails.adjustedCvss.scoreSource",
                    "aws.inspector.inspector_score_details.adjusted_cvss.score.source",
                )?;
            }

            if event.has("json.inspectorScoreDetails.adjustedCvss.scoringVector") {
                event.rename(
                    "json.inspectorScoreDetails.adjustedCvss.scoringVector",
                    "aws.inspector.inspector_score_details.adjusted_cvss.scoring_vector",
                )?;
            }

            if event.has("json.inspectorScoreDetails.adjustedCvss.version") {
                event.rename(
                    "json.inspectorScoreDetails.adjustedCvss.version",
                    "aws.inspector.inspector_score_details.adjusted_cvss.version",
                )?;
            }

            if let Some(v) = event
                .get("aws.inspector.inspector_score_details.adjusted_cvss.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.version", v)?;
            }

            let _cond = {
                event.has_value("json.lastObservedAt")
                    && event.get_str("json.lastObservedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastObservedAt") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set("aws.inspector.last_observed_at", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastObservedAt")?;
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
                    .get("json.networkReachabilityDetails.networkPath.steps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.networkReachabilityDetails.networkPath.steps",
                    |event| {
                        if event.has("_ingest._value.componentId") {
                            event.rename(
                                "_ingest._value.componentId",
                                "_ingest._value.component.id",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.networkReachabilityDetails.networkPath.steps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.networkReachabilityDetails.networkPath.steps",
                    |event| {
                        if event.has("_ingest._value.componentType") {
                            event.rename(
                                "_ingest._value.componentType",
                                "_ingest._value.component.type",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.networkReachabilityDetails.networkPath.steps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.networkReachabilityDetails.networkPath.steps",
                    |event| {
                        if event.has("_ingest._value.componentArn") {
                            event.rename(
                                "_ingest._value.componentArn",
                                "_ingest._value.component.arn",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has("json.networkReachabilityDetails.networkPath.steps") {
                event.rename(
                    "json.networkReachabilityDetails.networkPath.steps",
                    "aws.inspector.network_reachability_details.network_path.steps",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.networkReachabilityDetails.openPortRange.begin") {
                    if let Some(val) =
                        event.get("json.networkReachabilityDetails.openPortRange.begin")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.networkReachabilityDetails.openPortRange.begin".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.inspector.network_reachability_details.open_port_range.begin",
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
                    "convert_networkReachabilityDetails_openPortRange_begin_to_long",
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
                if event.has_value("json.networkReachabilityDetails.openPortRange.end") {
                    if let Some(val) =
                        event.get("json.networkReachabilityDetails.openPortRange.end")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.networkReachabilityDetails.openPortRange.end".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "aws.inspector.network_reachability_details.open_port_range.end",
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
                    "convert_networkReachabilityDetails_openPortRange_end_to_long",
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

            let _cond = {
                event
                    .get("json.packageVulnerabilityDetails.cvss")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.packageVulnerabilityDetails.cvss", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.baseScore") {
                            if let Some(val) = event.get("_ingest._value.baseScore") {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.baseScore".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.base_score", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_packageVulnerabilityDetails_cvss_baseScore_to_double",
                        )?;
                        event.remove("_ingest._value.baseScore");
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
                    .get("json.packageVulnerabilityDetails.cvss")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.packageVulnerabilityDetails.cvss", |event| {
                    event.remove("_ingest._value.baseScore");
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.packageVulnerabilityDetails.cvss")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.packageVulnerabilityDetails.cvss", |event| {
                    if event.has("_ingest._value.scoringVector") {
                        event.rename(
                            "_ingest._value.scoringVector",
                            "_ingest._value.scoring_vector",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has("json.packageVulnerabilityDetails.cvss") {
                event.rename(
                    "json.packageVulnerabilityDetails.cvss",
                    "aws.inspector.package_vulnerability_details.cvss",
                )?;
            }

            if event.has("json.networkReachabilityDetails.networkPath.steps") {
                event.rename(
                    "json.networkReachabilityDetails.networkPath.steps",
                    "aws.inspector.network_reachability_details.network_path.steps",
                )?;
            }

            if event.has("json.packageVulnerabilityDetails.relatedVulnerabilities") {
                event.rename(
                    "json.packageVulnerabilityDetails.relatedVulnerabilities",
                    "aws.inspector.package_vulnerability_details.related_vulnerabilities",
                )?;
            }

            if event.has("json.packageVulnerabilityDetails.source") {
                event.rename(
                    "json.packageVulnerabilityDetails.source",
                    "aws.inspector.package_vulnerability_details.source.value",
                )?;
            }

            let _cond = { event.has_value("json.packageVulnerabilityDetails.sourceUrl") };
            if _cond {
                uri_parts(
                    event,
                    "json.packageVulnerabilityDetails.sourceUrl",
                    "aws.inspector.package_vulnerability_details.source.url",
                    true,
                    false,
                )?;
            }

            let _cond = {
                event.has_value("json.packageVulnerabilityDetails.vendorCreatedAt")
                    && event.get_str("json.packageVulnerabilityDetails.vendorCreatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.packageVulnerabilityDetails.vendorCreatedAt")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.inspector.package_vulnerability_details.vendor.created_at",
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
                        "date_packageVulnerabilityDetails_vendorCreatedAt",
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
                .get("aws.inspector.package_vulnerability_details.vendor.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.published_date", v)?;
            }

            if event.has("json.packageVulnerabilityDetails.vendorSeverity") {
                event.rename(
                    "json.packageVulnerabilityDetails.vendorSeverity",
                    "aws.inspector.package_vulnerability_details.vendor.severity",
                )?;
            }

            let _cond = {
                event.has_value("json.packageVulnerabilityDetails.vendorUpdatedAt")
                    && event.get_str("json.packageVulnerabilityDetails.vendorUpdatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.packageVulnerabilityDetails.vendorUpdatedAt")
                    {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                            None,
                            None,
                        ) {
                            event.set(
                                "aws.inspector.package_vulnerability_details.vendor.updated_at",
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
                        "date_packageVulnerabilityDetails_vendorUpdatedAt",
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

            let _cond = {
                event
                    .get("json.packageVulnerabilityDetails.vulnerablePackages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.packageVulnerabilityDetails.vulnerablePackages",
                    |event| {
                        if event.has("_ingest._value.filePath") {
                            event.rename("_ingest._value.filePath", "_ingest._value.file_path")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.packageVulnerabilityDetails.vulnerablePackages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.packageVulnerabilityDetails.vulnerablePackages",
                    |event| {
                        if event.has("_ingest._value.fixedInVersion") {
                            event.rename(
                                "_ingest._value.fixedInVersion",
                                "_ingest._value.fixed_in_version",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.packageVulnerabilityDetails.vulnerablePackages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.packageVulnerabilityDetails.vulnerablePackages",
                    |event| {
                        if event.has("_ingest._value.packageManager") {
                            event.rename(
                                "_ingest._value.packageManager",
                                "_ingest._value.package_manager",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.packageVulnerabilityDetails.vulnerablePackages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.packageVulnerabilityDetails.vulnerablePackages",
                    |event| {
                        if event.has("_ingest._value.sourceLambdaLayerArn") {
                            event.rename(
                                "_ingest._value.sourceLambdaLayerArn",
                                "_ingest._value.source_lambda_layer_arn",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.packageVulnerabilityDetails.vulnerablePackages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.packageVulnerabilityDetails.vulnerablePackages",
                    |event| {
                        if event.has("_ingest._value.sourceLayerHash") {
                            event.rename(
                                "_ingest._value.sourceLayerHash",
                                "_ingest._value.source_layer_hash",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.packageVulnerabilityDetails.vulnerablePackages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.packageVulnerabilityDetails.vulnerablePackages",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.source_layer_hash")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if event.has("json.packageVulnerabilityDetails.vulnerablePackages") {
                event.rename(
                    "json.packageVulnerabilityDetails.vulnerablePackages",
                    "aws.inspector.package_vulnerability_details.vulnerable_packages",
                )?;
            }

            if let Some(v) = event
                .get("aws.inspector.package_vulnerability_details.vulnerable_packages")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("aws.inspector.package_nested", v)?;
            }

            let _cond = {
                event
                    .get("aws.inspector.package_vulnerability_details.vulnerable_packages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.inspector.package_vulnerability_details.vulnerable_packages",
                    |event| {
                        event.append_unique(
                            "package.architecture",
                            json!(
                                event
                                    .get("_ingest._value.arch")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("aws.inspector.package_vulnerability_details.vulnerable_packages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.inspector.package_vulnerability_details.vulnerable_packages",
                    |event| {
                        event.append_unique(
                            "package.name",
                            json!(
                                event
                                    .get("_ingest._value.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("aws.inspector.package_vulnerability_details.vulnerable_packages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.inspector.package_vulnerability_details.vulnerable_packages",
                    |event| {
                        event.append_unique(
                            "package.version",
                            json!(
                                event
                                    .get("_ingest._value.version")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("aws.inspector.package_vulnerability_details.vulnerable_packages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.inspector.package_vulnerability_details.vulnerable_packages",
                    |event| {
                        event.append_unique(
                            "package.path",
                            json!(
                                event
                                    .get("_ingest._value.file_path")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("aws.inspector.package_vulnerability_details.vulnerable_packages")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "aws.inspector.package_vulnerability_details.vulnerable_packages",
                    |event| {
                        event.append_unique(
                            "package.fixed_version",
                            json!(
                                event
                                    .get("_ingest._value.fixed_in_version")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if event.has("json.remediation.recommendation.text") {
                event.rename(
                    "json.remediation.recommendation.text",
                    "aws.inspector.remediation.recommendation.text",
                )?;
            }

            let _cond = { event.has_value("json.remediation.recommendation.Url") };
            if _cond {
                uri_parts(
                    event,
                    "json.remediation.recommendation.Url",
                    "aws.inspector.remediation.recommendation.url",
                    true,
                    false,
                )?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEc2Instance.iamInstanceProfileArn") {
                        event.rename(
                            "_ingest._value.details.awsEc2Instance.iamInstanceProfileArn",
                            "_ingest._value.details.aws.ec2_instance.iam_instance_profile_arn",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEc2Instance.imageId") {
                        event.rename(
                            "_ingest._value.details.awsEc2Instance.imageId",
                            "_ingest._value.details.aws.ec2_instance.image_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEc2Instance.ipV4Addresses") {
                        event.rename(
                            "_ingest._value.details.awsEc2Instance.ipV4Addresses",
                            "_ingest._value.details.aws.ec2_instance.ipv4_addresses",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.resources").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.details.aws.ec2_instance.ipv4_addresses")
                        {
                            if let Some(Value::Array(items)) = event
                                .get("_ingest._value.details.aws.ec2_instance.ipv4_addresses")
                                .cloned()
                            {
                                // A NESTED loop borrows the same `_ingest._value` slot, so
                                // the enclosing element is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let mut out = Vec::with_capacity(items.len());
                                for item in items {
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(val) = event.get("_ingest._value") {
                                            let converted =
                                                convert_value(val, "ip").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value", converted)?;
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        event.set(
                                            "_ingest.on_failure_processor_tag",
                                            "convert_details_aws_ec2_instance_ipv4_addresses_to_ip",
                                        )?;
                                        if event.remove("_ingest._value").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value".into(),
                                            });
                                        }
                                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                event.set(
                                    "_ingest._value.details.aws.ec2_instance.ipv4_addresses",
                                    Value::Array(out),
                                )?;
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.resources", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has_value("_ingest._value.details.aws.ec2_instance.ipv4_addresses") {
                        foreach_array(
                            event,
                            "_ingest._value.details.aws.ec2_instance.ipv4_addresses",
                            |event| {
                                event.append_unique(
                                    "related.ip",
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
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEc2Instance.ipV6Addresses") {
                        event.rename(
                            "_ingest._value.details.awsEc2Instance.ipV6Addresses",
                            "_ingest._value.details.aws.ec2_instance.ipv6_addresses",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.resources").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.details.aws.ec2_instance.ipv6_addresses")
                        {
                            if let Some(Value::Array(items)) = event
                                .get("_ingest._value.details.aws.ec2_instance.ipv6_addresses")
                                .cloned()
                            {
                                // A NESTED loop borrows the same `_ingest._value` slot, so
                                // the enclosing element is saved and put back afterwards.
                                let enclosing = event.get("_ingest._value").cloned();
                                let mut out = Vec::with_capacity(items.len());
                                for item in items {
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 2 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(val) = event.get("_ingest._value") {
                                            let converted =
                                                convert_value(val, "ip").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value", converted)?;
                                        }
                                        Ok(())
                                    })() {
                                        event.set("_ingest.on_failure_message", err.to_string())?;
                                        event
                                            .set("_ingest.on_failure_processor_type", "convert")?;
                                        event.set(
                                            "_ingest.on_failure_processor_tag",
                                            "convert_details_aws_ec2_instance_ipv6_addresses_to_ip",
                                        )?;
                                        if event.remove("_ingest._value").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value".into(),
                                            });
                                        }
                                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                        event.remove("_ingest.on_failure_message");
                                        event.remove("_ingest.on_failure_processor_type");
                                        event.remove("_ingest.on_failure_processor_tag");
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
                                        {
                                            event.remove("_ingest");
                                        }
                                    }
                                    out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                                }
                                match enclosing {
                                    Some(previous) => {
                                        event.set("_ingest._value", previous)?;
                                    }
                                    None => {
                                        event.remove("_ingest");
                                    }
                                }
                                event.set(
                                    "_ingest._value.details.aws.ec2_instance.ipv6_addresses",
                                    Value::Array(out),
                                )?;
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.resources", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has_value("_ingest._value.details.aws.ec2_instance.ipv6_addresses") {
                        foreach_array(
                            event,
                            "_ingest._value.details.aws.ec2_instance.ipv6_addresses",
                            |event| {
                                event.append_unique(
                                    "related.ip",
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
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEc2Instance.keyName") {
                        event.rename(
                            "_ingest._value.details.awsEc2Instance.keyName",
                            "_ingest._value.details.aws.ec2_instance.key_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("_ingest._value.details.awsEc2Instance.launchedAt")
                        {
                            if let Some(parsed) = parse_date_out(
                                &date_str,
                                &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                                None,
                                None,
                            ) {
                                event.set(
                                    "_ingest._value.details.aws.ec2_instance.launched_at",
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
                            "date_resources_details_awsEc2Instance_launchedAt",
                        )?;
                        event.remove("_ingest._value.details.awsEc2Instance.launchedAt");
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

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEc2Instance.platform") {
                        event.rename(
                            "_ingest._value.details.awsEc2Instance.platform",
                            "_ingest._value.details.aws.ec2_instance.platform",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEc2Instance.subnetId") {
                        event.rename(
                            "_ingest._value.details.awsEc2Instance.subnetId",
                            "_ingest._value.details.aws.ec2_instance.subnet_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEc2Instance.type") {
                        event.rename(
                            "_ingest._value.details.awsEc2Instance.type",
                            "_ingest._value.details.aws.ec2_instance.type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEc2Instance.vpcId") {
                        event.rename(
                            "_ingest._value.details.awsEc2Instance.vpcId",
                            "_ingest._value.details.aws.ec2_instance.vpc_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEcrContainerImage.architecture") {
                        event.rename(
                            "_ingest._value.details.awsEcrContainerImage.architecture",
                            "_ingest._value.details.aws.ecr_container_image.architecture",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEcrContainerImage.author") {
                        event.rename(
                            "_ingest._value.details.awsEcrContainerImage.author",
                            "_ingest._value.details.aws.ecr_container_image.author",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEcrContainerImage.imageHash") {
                        event.rename(
                            "_ingest._value.details.awsEcrContainerImage.imageHash",
                            "_ingest._value.details.aws.ecr_container_image.image.hash",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.details.aws.ecr_container_image.image.hash")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEcrContainerImage.imageTags") {
                        event.rename(
                            "_ingest._value.details.awsEcrContainerImage.imageTags",
                            "_ingest._value.details.aws.ecr_container_image.image.tags",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.details.awsEcrContainerImage.inUseCount")
                        {
                            if let Some(val) =
                                event.get("_ingest._value.details.awsEcrContainerImage.inUseCount")
                            {
                                let converted =
                                    convert_value(val, "long").map_err(|message| {
                                        TransformError::ParseError {
                    path: "_ingest._value.details.awsEcrContainerImage.inUseCount".into(),
                    message,
                    }
                                    })?;
                                event.set(
                                    "_ingest._value.details.aws.ecr_container_image.in_use_count",
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
                            "convert_resources_details_awsEcrContainerImage_inUseCount_to_long",
                        )?;
                        event.remove("_ingest._value.details.awsEcrContainerImage.inUseCount");
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

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "_ingest._value.details.awsEcrContainerImage.lastInUseAt",
                        ) {
                            if let Some(parsed) = parse_date_out(
                                &date_str,
                                &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                                None,
                                None,
                            ) {
                                event.set(
                                    "_ingest._value.details.aws.ecr_container_image.last_in_use_at",
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
                            "date_resources_details_awsEcrContainerImage_lastInUseAt",
                        )?;
                        event.remove("_ingest._value.details.awsEcrContainerImage.lastInUseAt");
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

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEcrContainerImage.platform") {
                        event.rename(
                            "_ingest._value.details.awsEcrContainerImage.platform",
                            "_ingest._value.details.aws.ecr_container_image.platform",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event
                            .get_as_string("_ingest._value.details.awsEcrContainerImage.pushedAt")
                        {
                            if let Some(parsed) = parse_date_out(
                                &date_str,
                                &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                                None,
                                None,
                            ) {
                                event.set(
                                    "_ingest._value.details.aws.ecr_container_image.pushed_at",
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
                            "date_resources_details_awsEcrContainerImage_pushedAt",
                        )?;
                        event.remove("_ingest._value.details.awsEcrContainerImage.pushedAt");
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

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEcrContainerImage.registry") {
                        event.rename(
                            "_ingest._value.details.awsEcrContainerImage.registry",
                            "_ingest._value.details.aws.ecr_container_image.registry",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsEcrContainerImage.repositoryName") {
                        event.rename(
                            "_ingest._value.details.awsEcrContainerImage.repositoryName",
                            "_ingest._value.details.aws.ecr_container_image.repository_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsLambdaFunction.codeSha256") {
                        event.rename(
                            "_ingest._value.details.awsLambdaFunction.codeSha256",
                            "_ingest._value.details.awsLambdaFunction.code_sha256",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.details.awsLambdaFunction.code_sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsLambdaFunction.executionRoleArn") {
                        event.rename(
                            "_ingest._value.details.awsLambdaFunction.executionRoleArn",
                            "_ingest._value.details.awsLambdaFunction.execution_role_arn",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsLambdaFunction.functionName") {
                        event.rename(
                            "_ingest._value.details.awsLambdaFunction.functionName",
                            "_ingest._value.details.awsLambdaFunction.function_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "_ingest._value.details.awsLambdaFunction.lastModifiedAt",
                        ) {
                            if let Some(parsed) = parse_date_out(
                                &date_str,
                                &["ISO8601", "UNIX", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"],
                                None,
                                None,
                            ) {
                                event.set(
                                    "_ingest._value.details.awsLambdaFunction.last_modified_at",
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
                            "date_resources_details_awsLambdaFunction_lastModifiedAt",
                        )?;
                        event.remove("_ingest._value.details.awsLambdaFunction.lastModifiedAt");
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

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsLambdaFunction.packageType") {
                        event.rename(
                            "_ingest._value.details.awsLambdaFunction.packageType",
                            "_ingest._value.details.awsLambdaFunction.package_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event
                        .has("_ingest._value.details.awsLambdaFunction.vpcConfig.securityGroupIds")
                    {
                        event.rename("_ingest._value.details.awsLambdaFunction.vpcConfig.securityGroupIds", "_ingest._value.details.awsLambdaFunction.vpc_config.security_group_ids")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsLambdaFunction.vpcConfig.subnetIds") {
                        event.rename(
                            "_ingest._value.details.awsLambdaFunction.vpcConfig.subnetIds",
                            "_ingest._value.details.awsLambdaFunction.vpc_config.subnet_ids",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsLambdaFunction.vpcConfig.vpcId") {
                        event.rename(
                            "_ingest._value.details.awsLambdaFunction.vpcConfig.vpcId",
                            "_ingest._value.details.awsLambdaFunction.vpc_config.vpc_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    event.remove("_ingest._value.details.awsEc2Instance.launchedAt");
                    event.remove("_ingest._value.details.awsEcrContainerImage.inUseCount");
                    event.remove("_ingest._value.details.awsEcrContainerImage.lastInUseAt");
                    event.remove("_ingest._value.details.awsEcrContainerImage.pushedAt");
                    event.remove("_ingest._value.details.awsLambdaFunction.lastModifiedAt");
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.awsLambdaFunction") {
                        event.rename(
                            "_ingest._value.details.awsLambdaFunction",
                            "_ingest._value.details.aws.lambda_function",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.codeRepository.integrationArn") {
                        event.rename(
                            "_ingest._value.details.codeRepository.integrationArn",
                            "_ingest._value.details.code_repository.integration_arn",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.codeRepository.projectName") {
                        event.rename(
                            "_ingest._value.details.codeRepository.projectName",
                            "_ingest._value.details.code_repository.project_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.resources").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.resources", |event| {
                    if event.has("_ingest._value.details.codeRepository.providerType") {
                        event.rename(
                            "_ingest._value.details.codeRepository.providerType",
                            "_ingest._value.details.code_repository.provider_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has("json.resources") {
                event.rename("json.resources", "aws.inspector.resources")?;
            }

            let _cond = {
                event.get("aws.inspector.resources").is_some_and(|v| v.is_array()) && event.get("aws.inspector.resources").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 1)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: // Arrays won't work in general in current UI of Cloud Security Posture workflow. In Amazon Inspector, a finding may contain multiple resources, but rarely.\n// When a finding has single-resource, we extract fields as single-value so that the Vulnerability Findings UI behaves as expected for almost all cases.\n// But in the rare multi-resource case, we extract fields into an array to not miss any affected resources for a finding. \n// This trade-off is okay as not many findings will be affected. When our UI natively supports multi-resources, the single-value resource extraction must be removed.\n\ndef resources = ctx.aws.inspector.resources;\n\n// Define fields to be extracted.\nctx.resource = ctx.resource ?: [:];\nctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nctx.host.ip = ctx.host.ip ?: [];\nctx.cloud = ctx.cloud ?: [:];\nctx.cloud.instance = ctx.cloud.instance ?: [:];\nctx.cloud.machine = ctx.cloud.machine ?: [:];\n\n// This extraction logic is only for single resource case. Multiple resources are extracted inside script - script_extract_fields_from_multiple_resources.\nif (resources.size() == 1){\n  def res = resources[0];\n\n  ctx.resource.id = res.id;\n  ctx.resource.name = res.tags?.Name;\n  ctx.resource.type = res.type;\n  ctx.cloud.region = res.region;\n\n  if (res.type == 'AWS_EC2_INSTANCE') {\n    ctx.cloud.instance.id = res.id;\n    ctx.cloud.machine.type = res.details?.aws?.ec2_instance?.type;\n    ctx.host.id = res.id;\n    ctx.host.name = res.tags?.Name;\n    ctx.host.type = res.details?.aws?.ec2_instance?.type;\n    ctx.host.os.platform = res.details?.aws?.ec2_instance?.platform;\n    if (res.details?.aws?.ec2_instance?.ipv4_addresses instanceof List) {\n      for (ipv4 in res.details?.aws?.ec2_instance?.ipv4_addresses) {\n        ctx.host.ip.add(ipv4);\n      }\n    }\n    if (res.details?.aws?.ec2_instance?.ipv6_addresses instanceof List) {\n      for (ipv6 in res.details?.aws?.ec2_instance?.ipv6_addresses) {\n        ctx.host.ip.add(ipv6);\n      }\n    }\n    def platform = res.details?.aws?.ec2_instance?.platform?.toLowerCase();\n    if (platform?.contains('windows') == true) {\n      ctx.host.os.type = 'windows';\n    } else if (platform?.contains('linux') == true) {\n      ctx.host.os.type = 'linux';\n    } else if (platform?.contains('macos') == true) {\n      ctx.host.os.type = 'macos';\n    }\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"// Arrays won't work in general in current UI of Cloud Security Posture workflow. In Amazon Inspector, a finding may contain multiple resources, but rarely.\n// When a finding has single-resource, we extract fields as single-value so that the Vulnerability Findings UI behaves as expected for almost all cases.\n// But in the rare multi-resource case, we extract fields into an array to not miss any affected resources for a finding. \n// This trade-off is okay as not many findings will be affected. When our UI natively supports multi-resources, the single-value resource extraction must be removed.\n\ndef resources = ctx.aws.inspector.resources;\n\n// Define fields to be extracted.\nctx.resource = ctx.resource ?: [:];\nctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nctx.host.ip = ctx.host.ip ?: [];\nctx.cloud = ctx.cloud ?: [:];\nctx.cloud.instance = ctx.cloud.instance ?: [:];\nctx.cloud.machine = ctx.cloud.machine ?: [:];\n\n// This extraction logic is only for single resource case. Multiple resources are extracted inside script - script_extract_fields_from_multiple_resources.\nif (resources.size() == 1){\n  def res = resources[0];\n\n  ctx.resource.id = res.id;\n  ctx.resource.name = res.tags?.Name;\n  ctx.resource.type = res.type;\n  ctx.cloud.region = res.region;\n\n  if (res.type == 'AWS_EC2_INSTANCE') {\n    ctx.cloud.instance.id = res.id;\n    ctx.cloud.machine.type = res.details?.aws?.ec2_instance?.type;\n    ctx.host.id = res.id;\n    ctx.host.name = res.tags?.Name;\n    ctx.host.type = res.details?.aws?.ec2_instance?.type;\n    ctx.host.os.platform = res.details?.aws?.ec2_instance?.platform;\n    if (res.details?.aws?.ec2_instance?.ipv4_addresses instanceof List) {\n      for (ipv4 in res.details?.aws?.ec2_instance?.ipv4_addresses) {\n        ctx.host.ip.add(ipv4);\n      }\n    }\n    if (res.details?.aws?.ec2_instance?.ipv6_addresses instanceof List) {\n      for (ipv6 in res.details?.aws?.ec2_instance?.ipv6_addresses) {\n        ctx.host.ip.add(ipv6);\n      }\n    }\n    def platform = res.details?.aws?.ec2_instance?.platform?.toLowerCase();\n    if (platform?.contains('windows') == true) {\n      ctx.host.os.type = 'windows';\n    } else if (platform?.contains('linux') == true) {\n      ctx.host.os.type = 'linux';\n    } else if (platform?.contains('macos') == true) {\n      ctx.host.os.type = 'macos';\n    }\n  }\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_extract_fields_from_single_resource",
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

            let _cond = {
                event.get("aws.inspector.resources").is_some_and(|v| v.is_array()) && event.get("aws.inspector.resources").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 1)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def resources = ctx.aws.inspector.resources;\n\n// Define fields to be extracted.\nctx.resource = ctx.resource ?: [:];\nctx.resource.id = ctx.resource.id ?: [];\nctx.resource.name = ctx.resource.name ?: [];\n\nctx.host = ctx.host ?: [:];\nctx.host.id = ctx.host.id ?: [];\nctx.host.name = ctx.host.name ?: [];\nctx.host.ip = ctx.host.ip ?: [];\nctx.host.type = ctx.host.type ?: [];\nctx.host.os = ctx.host.os ?: [:];\nctx.host.os.platform = ctx.host.os.platform ?: [];\nctx.host.os.type = ctx.host.os.type ?: [];\n\nctx.cloud = ctx.cloud ?: [:];\nctx.cloud.instance = ctx.cloud.instance ?: [:];\nctx.cloud.instance.id = ctx.cloud.instance.id ?: [];\nctx.cloud.machine = ctx.cloud.machine ?: [:];\nctx.cloud.machine.type = ctx.cloud.machine.type ?: [];\nctx.cloud.region = ctx.cloud.region ?: [];\n\nfor (res in resources) {\n  ctx.resource.id.add(res.id);\n  ctx.resource.name.add(res.tags?.Name);\n  ctx.resource.type.add(res.type);\n  ctx.cloud.region.add(res.region);\n\n  if (res.type == 'AWS_EC2_INSTANCE') {\n    ctx.cloud.instance.id.add(res.id);\n    ctx.cloud.machine.type.add(res.details?.aws?.ec2_instance?.type);\n    ctx.host.id.add(res.id);\n    ctx.host.name.add(res.tags?.Name);\n    ctx.host.type.add(res.details?.aws?.ec2_instance?.type);\n    ctx.host.os.platform.add(res.details?.aws?.ec2_instance?.platform);\n    if (res.details?.aws?.ec2_instance?.ipv4_addresses instanceof List) {\n      for (ipv4 in res.details?.aws?.ec2_instance?.ipv4_addresses) {\n        ctx.host.ip.add(ipv4);\n      }\n    }\n    if (res.details?.aws?.ec2_instance?.ipv6_addresses instanceof List) {\n      for (ipv6 in res.details?.aws?.ec2_instance?.ipv6_addresses) {\n        ctx.host.ip.add(ipv6);\n      }\n    }\n    def platform = res.details?.aws?.ec2_instance?.platform?.toLowerCase();\n    if (platform?.contains('windows') == true) {\n      ctx.host.os.type = 'windows';\n    } else if (platform?.contains('linux') == true) {\n      ctx.host.os.type = 'linux';\n    } else if (platform?.contains('macos') == true) {\n      ctx.host.os.type = 'macos';\n    }\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def resources = ctx.aws.inspector.resources;\n\n// Define fields to be extracted.\nctx.resource = ctx.resource ?: [:];\nctx.resource.id = ctx.resource.id ?: [];\nctx.resource.name = ctx.resource.name ?: [];\n\nctx.host = ctx.host ?: [:];\nctx.host.id = ctx.host.id ?: [];\nctx.host.name = ctx.host.name ?: [];\nctx.host.ip = ctx.host.ip ?: [];\nctx.host.type = ctx.host.type ?: [];\nctx.host.os = ctx.host.os ?: [:];\nctx.host.os.platform = ctx.host.os.platform ?: [];\nctx.host.os.type = ctx.host.os.type ?: [];\n\nctx.cloud = ctx.cloud ?: [:];\nctx.cloud.instance = ctx.cloud.instance ?: [:];\nctx.cloud.instance.id = ctx.cloud.instance.id ?: [];\nctx.cloud.machine = ctx.cloud.machine ?: [:];\nctx.cloud.machine.type = ctx.cloud.machine.type ?: [];\nctx.cloud.region = ctx.cloud.region ?: [];\n\nfor (res in resources) {\n  ctx.resource.id.add(res.id);\n  ctx.resource.name.add(res.tags?.Name);\n  ctx.resource.type.add(res.type);\n  ctx.cloud.region.add(res.region);\n\n  if (res.type == 'AWS_EC2_INSTANCE') {\n    ctx.cloud.instance.id.add(res.id);\n    ctx.cloud.machine.type.add(res.details?.aws?.ec2_instance?.type);\n    ctx.host.id.add(res.id);\n    ctx.host.name.add(res.tags?.Name);\n    ctx.host.type.add(res.details?.aws?.ec2_instance?.type);\n    ctx.host.os.platform.add(res.details?.aws?.ec2_instance?.platform);\n    if (res.details?.aws?.ec2_instance?.ipv4_addresses instanceof List) {\n      for (ipv4 in res.details?.aws?.ec2_instance?.ipv4_addresses) {\n        ctx.host.ip.add(ipv4);\n      }\n    }\n    if (res.details?.aws?.ec2_instance?.ipv6_addresses instanceof List) {\n      for (ipv6 in res.details?.aws?.ec2_instance?.ipv6_addresses) {\n        ctx.host.ip.add(ipv6);\n      }\n    }\n    def platform = res.details?.aws?.ec2_instance?.platform?.toLowerCase();\n    if (platform?.contains('windows') == true) {\n      ctx.host.os.type = 'windows';\n    } else if (platform?.contains('linux') == true) {\n      ctx.host.os.type = 'linux';\n    } else if (platform?.contains('macos') == true) {\n      ctx.host.os.type = 'macos';\n    }\n  }\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_extract_fields_from_multiple_resources",
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

            if event.has("json.status") {
                event.rename("json.status", "aws.inspector.status")?;
            }

            let _cond = {
                event.get_str("aws.inspector.status") == Some("ACTIVE")
                    || event.get_str("aws.inspector.status") == Some("SUPPRESSED")
            };
            if _cond {
                event.set("vulnerability.status", json!("open"))?;
            }

            let _cond = { event.get_str("aws.inspector.status") == Some("CLOSED") };
            if _cond {
                event.set("vulnerability.status", json!("fixed"))?;
            }

            if event.has("json.title") {
                event.rename("json.title", "aws.inspector.title")?;
            }

            if let Some(v) = event
                .get("aws.inspector.title")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.title", v)?;
            }

            if event.has("json.type") {
                event.rename("json.type", "aws.inspector.type")?;
            }

            let _cond = { event.get_str("aws.inspector.type") == Some("PACKAGE_VULNERABILITY") };
            if _cond {
                event.set(
                    "event.id",
                    json!(format!(
                        "{}|{}|{}|{}|{}",
                        event
                            .get("vulnerability.id")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("resource.id")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("package.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("package.version")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("@timestamp")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.get_str("aws.inspector.type") == Some("PACKAGE_VULNERABILITY") };
            if _cond {
                event.set(
                    "aws.inspector.transform_unique_id",
                    json!(format!(
                        "{}|{}|{}|{}",
                        event
                            .get("vulnerability.id")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("resource.id")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("package.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("package.version")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            event.remove("json");

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
                event.remove("aws.inspector.description");
                event.remove("aws.inspector.updated_at");
                event.remove("aws.inspector.aws_account_id");
                event.remove("aws.inspector.network_reachability_details.protocol");
                event.remove("aws.inspector.package_vulnerability_details.reference_urls");
                event.remove("aws.inspector.package_vulnerability_details.vulnerability_id");
                event.remove("aws.inspector.package_vulnerability_details.vendor.created_at");
                event.remove("aws.inspector.inspector_score_details.adjusted_cvss.score.value");
                event.remove("aws.inspector.inspector_score_details.adjusted_cvss.version");
                event.remove("aws.inspector.title");
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
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
                event.append("error.message", json!(format!("Processor '{}'\n{}with tag '{}'\n{}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
