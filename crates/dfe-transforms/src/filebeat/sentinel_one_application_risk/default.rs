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

            parse_json_field(event, "event.original", "json")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.status") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("state"))?;

            event.append("event.type", json!("info"))?;

            event.append("event.category", json!("vulnerability"))?;

            if event.has("json.application") {
                event.rename(
                    "json.application",
                    "sentinel_one.application_risk.application",
                )?;
            }

            if event.has("json.applicationName") {
                event.rename(
                    "json.applicationName",
                    "sentinel_one.application_risk.application_name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.application_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.name", v)?;
            }

            if event.has("json.applicationVendor") {
                event.rename(
                    "json.applicationVendor",
                    "sentinel_one.application_risk.application_vendor",
                )?;
            }

            if event.has("json.applicationVersion") {
                event.rename(
                    "json.applicationVersion",
                    "sentinel_one.application_risk.application_version",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.application_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("package.version", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.baseScore") {
                    if let Some(val) = event.get("json.baseScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.baseScore".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.application_risk.base_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_baseScore_to_double",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                .get("sentinel_one.application_risk.base_score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.base", v)?;
            }

            if event.has("json.cveId") {
                event.rename("json.cveId", "sentinel_one.application_risk.cve_id")?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.cve_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.id", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.cve_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.cve", v)?;
            }

            if event.has("json.cvssVersion") {
                event.rename(
                    "json.cvssVersion",
                    "sentinel_one.application_risk.cvss_version",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.cvss_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.version", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.daysDetected") {
                    if let Some(val) = event.get("json.daysDetected") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.daysDetected".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.application_risk.days_detected", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_daysDetected_to_long",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
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
                event.has_value("json.detectionDate")
                    && event.get_str("json.detectionDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.detectionDate") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            event.set("sentinel_one.application_risk.detection_date", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_detectionDate")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has_value("json.endpointId") {
                if let Some(val) = event.get("json.endpointId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.endpointId".into(),
                            message,
                        }
                    })?;
                    event.set("sentinel_one.application_risk.endpoint_id", converted)?;
                }
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.endpoint_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.endpoint_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.id", v)?;
            }

            if event.has("json.endpointName") {
                event.rename(
                    "json.endpointName",
                    "sentinel_one.application_risk.endpoint_name",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.endpoint_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.name", v)?;
            }

            if event.has("json.endpointType") {
                event.rename(
                    "json.endpointType",
                    "sentinel_one.application_risk.endpoint_type",
                )?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.endpoint_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }

            if event.has("json.exploitCodeMaturity") {
                event.rename(
                    "json.exploitCodeMaturity",
                    "sentinel_one.application_risk.exploit_code_maturity",
                )?;
            }

            if event.has_value("json.id") {
                if let Some(val) = event.get("json.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.id".into(),
                            message,
                        }
                    })?;
                    event.set("sentinel_one.application_risk.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("json.lastScanDate")
                    && event.get_str("json.lastScanDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastScanDate") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["date_optional_time"], None, None)
                        {
                            event.set("sentinel_one.application_risk.last_scan_date", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastScanDate")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                .get("sentinel_one.application_risk.last_scan_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has("json.lastScanResult") {
                event.rename(
                    "json.lastScanResult",
                    "sentinel_one.application_risk.last_scan_result",
                )?;
            }

            let _cond = {
                event
                    .get_str("sentinel_one.application_risk.last_scan_result")
                    .is_some_and(|s| s.eq_ignore_ascii_case("Succeeded"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event
                    .get_str("sentinel_one.application_risk.last_scan_result")
                    .is_some_and(|s| s.eq_ignore_ascii_case("Failed"))
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if !event.has("event.outcome") {
                event.set("event.outcome", json!("unknown"))?;
            }

            if event.has("json.markType") {
                event.rename("json.markType", "sentinel_one.application_risk.mark_type")?;
            }

            if event.has("json.markedBy") {
                event.rename("json.markedBy", "sentinel_one.application_risk.marked_by")?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.marked_by")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = {
                event.has_value("json.markedDate") && event.get_str("json.markedDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.markedDate") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["date_optional_time"], None, None)
                        {
                            event.set("sentinel_one.application_risk.marked_date", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_markedDate")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.mitigationStatus") {
                event.rename(
                    "json.mitigationStatus",
                    "sentinel_one.application_risk.mitigation_status",
                )?;
            }

            let _cond = {
                event.has_value("json.mitigationStatusChangeTime")
                    && event.get_str("json.mitigationStatusChangeTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.mitigationStatusChangeTime") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &["ISO8601", "date_optional_time"],
                            None,
                            None,
                        ) {
                            event.set(
                                "sentinel_one.application_risk.mitigation_status_change_time",
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
                        "date_mitigationStatusChangeTime",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.mitigationStatusChangedBy") {
                event.rename(
                    "json.mitigationStatusChangedBy",
                    "sentinel_one.application_risk.mitigation_status_changed_by",
                )?;
            }

            if event.has("json.mitigationStatusReason") {
                event.rename(
                    "json.mitigationStatusReason",
                    "sentinel_one.application_risk.mitigation_status_reason",
                )?;
            }

            let _cond = { event.get_str("json.nvdBaseScore") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.nvdBaseScore") {
                        if let Some(val) = event.get("json.nvdBaseScore") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.nvdBaseScore".into(),
                                    message,
                                }
                            })?;
                            event.set("sentinel_one.application_risk.nvd_base_score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_nvdBaseScore_to_float",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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

            if event.has("json.nvdCvssVersion") {
                event.rename(
                    "json.nvdCvssVersion",
                    "sentinel_one.application_risk.nvd_cvss_version",
                )?;
            }

            if event.has("json.osType") {
                event.rename("json.osType", "sentinel_one.application_risk.os_type")?;
            }

            let _cond = {
                event.get_str("sentinel_one.application_risk.os_type") == Some("windows")
                    || event.get_str("sentinel_one.application_risk.os_type") == Some("linux")
                    || event.get_str("sentinel_one.application_risk.os_type") == Some("macos")
            };
            if _cond {
                if let Some(v) = event
                    .get("sentinel_one.application_risk.os_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.type", v)?;
                }
            }

            let _cond = {
                event.has_value("json.publishedDate")
                    && event.get_str("json.publishedDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.publishedDate") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["date_optional_time"], None, None)
                        {
                            event.set("sentinel_one.application_risk.published_date", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_publishedDate")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                .get("sentinel_one.application_risk.published_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.package.published_date", v)?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.detection_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has("json.reason") {
                event.rename("json.reason", "sentinel_one.application_risk.reason")?;
            }

            if let Some(v) = event
                .get("sentinel_one.application_risk.reason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if event.has("json.remediationLevel") {
                event.rename(
                    "json.remediationLevel",
                    "sentinel_one.application_risk.remediation_level",
                )?;
            }

            if event.has("json.reportConfidence") {
                event.rename(
                    "json.reportConfidence",
                    "sentinel_one.application_risk.report_confidence",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.riskScore") {
                    if let Some(val) = event.get("json.riskScore") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.riskScore".into(),
                                message,
                            }
                        })?;
                        event.set("sentinel_one.application_risk.risk_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_riskScore_to_double",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.severity") {
                event.rename("json.severity", "sentinel_one.application_risk.severity")?;
            }

            let _cond = { event.has_value("sentinel_one.application_risk.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nctx.event.severity = params.get(ctx.sentinel_one.application_risk.severity.toLowerCase());
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nctx.event.severity = params.get(ctx.sentinel_one.application_risk.severity.toLowerCase());"#
                        ),
                        cached_params!("{\"critical\":99,\"high\":73,\"low\":21,\"medium\":47}"),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_e4bd0b3b")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
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
                event.rename("json.status", "sentinel_one.application_risk.status")?;
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
                event.remove("sentinel_one.application_risk.application_name");
                event.remove("sentinel_one.application_risk.application_version");
                event.remove("sentinel_one.application_risk.base_score");
                event.remove("sentinel_one.application_risk.cve_id");
                event.remove("sentinel_one.application_risk.cvss_version");
                event.remove("sentinel_one.application_risk.endpoint_id");
                event.remove("sentinel_one.application_risk.endpoint_name");
                event.remove("sentinel_one.application_risk.endpoint_type");
                event.remove("sentinel_one.application_risk.id");
                event.remove("sentinel_one.application_risk.os_type");
                event.remove("sentinel_one.application_risk.published_date");
                event.remove("sentinel_one.application_risk.reason");
            }

            event.remove("json");

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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}'\n{}with tag '{}'\n{}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("#_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("/_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
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
