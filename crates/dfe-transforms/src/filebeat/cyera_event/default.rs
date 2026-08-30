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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "json_event_original_a68ecd77",
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

            event.set("event.kind", json!("alert"))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.date") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.date".into(),
                    });
                }
                if let Some(v) = event.get("json.uid") {
                    values.push(v.clone());
                } else {
                    return Err(TransformError::FieldNotFound {
                        path: "json.uid".into(),
                    });
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.account.name") {
                event.rename("json.account.name", "cyera.event.account.name")?;
            }

            if let Some(v) = event
                .get("cyera.event.account.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.name", v)?;
            }

            if event.has_value("json.account.platform") {
                event.rename("json.account.platform", "cyera.event.account.platform")?;
            }

            if event.has_value("json.account.uid") {
                event.rename("json.account.uid", "cyera.event.account.uid")?;
            }

            if let Some(v) = event
                .get("cyera.event.account.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            let _cond = {
                event
                    .get("json.affectedDataClassAppearances")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.affectedDataClassAppearances", |event| {
                    if event.has_value("_ingest._value.recordsCount.dataClass.classificationName") {
                        event.rename(
                            "_ingest._value.recordsCount.dataClass.classificationName",
                            "_ingest._value.records_count.data_class.classification_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.affectedDataClassAppearances")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.affectedDataClassAppearances", |event| {
                    if event.has_value("_ingest._value.recordsCount.dataClass.uid") {
                        event.rename(
                            "_ingest._value.recordsCount.dataClass.uid",
                            "_ingest._value.records_count.data_class.uid",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.affectedDataClassAppearances")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.affectedDataClassAppearances", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.recordsCount.recordsCount") {
                            if let Some(val) = event.get("_ingest._value.recordsCount.recordsCount")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.recordsCount.recordsCount".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.records_count.value", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_affectedDataClassAppearances_recordsCount_recordsCount_to_long_71a34cb8")?;
                        event.remove("_ingest._value.recordsCount.recordsCount");
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
                    .get("json.affectedDataClassAppearances")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.affectedDataClassAppearances", |event| {
                    event.remove("_ingest._value.recordsCount.recordsCount");
                    Ok(())
                })?;
            }

            if event.has_value("json.affectedDataClassAppearances") {
                event.rename(
                    "json.affectedDataClassAppearances",
                    "cyera.event.affected.data_class_appearances",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.affectedObjectsCount") {
                    if let Some(val) = event.get("json.affectedObjectsCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.affectedObjectsCount".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.event.affected.objects_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_affectedObjectsCount_to_long_538200a2",
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
                if event.has_value("json.affectedObjectsDiff") {
                    if let Some(val) = event.get("json.affectedObjectsDiff") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.affectedObjectsDiff".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.event.affected.objects_diff", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_affectedObjectsDiff_to_long_554e0912",
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
                if event.has_value("json.affectedRecordsDiff") {
                    if let Some(val) = event.get("json.affectedRecordsDiff") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.affectedRecordsDiff".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.event.affected.records_diff", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_affectedRecordsDiff_to_long_5a5ca13b",
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

            if event.has_value("json.automaticScanning") {
                event.rename("json.automaticScanning", "cyera.event.automatic_scanning")?;
            }

            if event.has_value("json.changedClassificationUid") {
                event.rename(
                    "json.changedClassificationUid",
                    "cyera.event.changed_classification_uid",
                )?;
            }

            if event.has_value("json.classificationName") {
                event.rename("json.classificationName", "cyera.event.classification_name")?;
            }

            let _cond = {
                event
                    .get("json.classifications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.classifications", |event| {
                    if event.has_value("_ingest._value.classificationName") {
                        event.rename("_ingest._value.classificationName", "_ingest._value.name")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.classifications") {
                event.rename("json.classifications", "cyera.event.classifications")?;
            }

            if event.has_value("json.cloudProvider") {
                event.rename("json.cloudProvider", "cyera.event.cloud_provider")?;
            }

            if let Some(v) = event
                .get("cyera.event.cloud_provider")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if event.has_value("cloud.provider") {
                map_strings(event, "cloud.provider", "cloud.provider", str::to_lowercase)?;
            }

            if event.has_value("json.datastore.dataOwner") {
                event.rename(
                    "json.datastore.dataOwner",
                    "cyera.event.datastore.data_owner",
                )?;
            }

            if event.has_value("json.datastore.infrastructure") {
                event.rename(
                    "json.datastore.infrastructure",
                    "cyera.event.datastore.infrastructure",
                )?;
            }

            if let Some(v) = event
                .get("cyera.event.datastore.infrastructure")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.service.name", v)?;
            }

            if event.has_value("json.datastore.name") {
                event.rename("json.datastore.name", "cyera.event.datastore.name")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.datastore.recordsAtHighRisk") {
                    if let Some(val) = event.get("json.datastore.recordsAtHighRisk") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.datastore.recordsAtHighRisk".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.event.datastore.records_at_high_risk", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_datastore_recordsAtHighRisk_to_long_394730d0",
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

            if event.has_value("json.datastore.uid") {
                event.rename("json.datastore.uid", "cyera.event.datastore.uid")?;
            }

            if event.has_value("json.datastore.userTags") {
                event.rename(
                    "json.datastore.userTags",
                    "cyera.event.datastore.tags.user_tags",
                )?;
            }

            if event.has_value("json.datastoreUserTags") {
                event.rename("json.datastoreUserTags", "cyera.event.datastore.user_tags")?;
            }

            if event.has_value("json.datastore.vpcId") {
                event.rename("json.datastore.vpcId", "cyera.event.datastore.vpc_id")?;
            }

            let _cond = { event.has_value("json.date") && event.get_str("json.date") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("cyera.event.date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_date_fdcfaee1")?;
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
                .get("cyera.event.date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.deploymentName") {
                event.rename("json.deploymentName", "cyera.event.deployment_name")?;
            }

            if event.has_value("json.domainName") {
                event.rename("json.domainName", "cyera.event.domain_name")?;
            }

            let _cond = { event.has_value("cyera.event.domain_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cyera.event.domain_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.expectedM365SensitivityLabelAssignmentsCount") {
                    if let Some(val) =
                        event.get("json.expectedM365SensitivityLabelAssignmentsCount")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.expectedM365SensitivityLabelAssignmentsCount".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cyera.event.expected_m365_sensitivity_label_assignments_count",
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
                    "convert_expectedM365SensitivityLabelAssignmentsCount_to_long_e0a52d80",
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
                if event.has_value("json.failedM365SensitivityLabelAssignmentsCount") {
                    if let Some(val) = event.get("json.failedM365SensitivityLabelAssignmentsCount")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.failedM365SensitivityLabelAssignmentsCount".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cyera.event.failed_m365_sensitivity_label_assignments_count",
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
                    "convert_failedM365SensitivityLabelAssignmentsCount_to_long_0d6becc9",
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

            if event.has_value("json.frequency") {
                event.rename("json.frequency", "cyera.event.frequency")?;
            }

            if event.has_value("json.inPlatformIdentifier") {
                event.rename(
                    "json.inPlatformIdentifier",
                    "cyera.event.in_platform_identifier",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.isDomainTrusted") {
                    if let Some(val) = event.get("json.isDomainTrusted") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.isDomainTrusted".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.event.is_domain_trusted", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isDomainTrusted_to_boolean_fd93830c",
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
                if event.has_value("json.isReportDeleted") {
                    if let Some(val) = event.get("json.isReportDeleted") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.isReportDeleted".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.event.is_report.deleted", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isReportDeleted_to_boolean_f65f50d1",
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
                if event.has_value("json.isReportExpired") {
                    if let Some(val) = event.get("json.isReportExpired") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.isReportExpired".into(),
                                message,
                            }
                        })?;
                        event.set("cyera.event.is_report.expired", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_isReportExpired_to_boolean_47ba1c2f",
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

            if event.has_value("json.issueName") {
                event.rename("json.issueName", "cyera.event.issue_name")?;
            }

            if event.has_value("json.issue.policy.name") {
                event.rename("json.issue.policy.name", "cyera.event.issue.policy.name")?;
            }

            if event.has_value("json.issue.policy.uid") {
                event.rename("json.issue.policy.uid", "cyera.event.issue.policy.uid")?;
            }

            if event.has_value("json.issue.policy.useCases") {
                event.rename(
                    "json.issue.policy.useCases",
                    "cyera.event.issue.policy.use_cases",
                )?;
            }

            if event.has_value("json.issue.resolutionNote") {
                event.rename(
                    "json.issue.resolutionNote",
                    "cyera.event.issue.resolution.note",
                )?;
            }

            if event.has_value("json.issue.resolution") {
                event.rename(
                    "json.issue.resolution",
                    "cyera.event.issue.resolution.value",
                )?;
            }

            if event.has_value("json.issue.riskStatus") {
                event.rename("json.issue.riskStatus", "cyera.event.issue.risk_status")?;
            }

            if event.has_value("json.issueUid") {
                event.rename("json.issueUid", "cyera.event.issue_uid")?;
            }

            if event.has_value("json.issue.uid") {
                event.rename("json.issue.uid", "cyera.event.issue.uid")?;
            }

            let _cond = { event.get("json.issues").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.issues", |event| {
                    if event.has_value("_ingest._value.policy.useCases") {
                        event.rename(
                            "_ingest._value.policy.useCases",
                            "_ingest._value.policy.use_cases",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.issues").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.issues", |event| {
                    if event.has_value("_ingest._value.resolutionNote") {
                        event.rename(
                            "_ingest._value.resolutionNote",
                            "_ingest._value.resolution_note",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.issues").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.issues", |event| {
                    if event.has_value("_ingest._value.resolution") {
                        event.rename(
                            "_ingest._value.resolution",
                            "_ingest._value.resolution_value",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.issues").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.issues", |event| {
                    if event.has_value("_ingest._value.riskStatus") {
                        event.rename("_ingest._value.riskStatus", "_ingest._value.risk_status")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.issues") {
                event.rename("json.issues", "cyera.event.issues")?;
            }

            if event.has_value("json.m365AssignedSensitivityLabelName") {
                event.rename(
                    "json.m365AssignedSensitivityLabelName",
                    "cyera.event.m365_assigned_sensitivity_label_name",
                )?;
            }

            if event.has_value("json.originalSensitivityDisplayName") {
                event.rename(
                    "json.originalSensitivityDisplayName",
                    "cyera.event.original_sensitivity.display_name",
                )?;
            }

            if event.has_value("json.originalSensitivity") {
                event.rename(
                    "json.originalSensitivity",
                    "cyera.event.original_sensitivity.value",
                )?;
            }

            if event.has_value("json.policy.name") {
                event.rename("json.policy.name", "cyera.event.policy.name")?;
            }

            if event.has_value("json.policy.uid") {
                event.rename("json.policy.uid", "cyera.event.policy.uid")?;
            }

            if event.has_value("json.policy.useCases") {
                event.rename("json.policy.useCases", "cyera.event.policy.use_cases")?;
            }

            if event.has_value("json.projectName") {
                event.rename("json.projectName", "cyera.event.project.name")?;
            }

            if event.has_value("json.projectUid") {
                event.rename("json.projectUid", "cyera.event.project.uid")?;
            }

            if event.has_value("json.recipients") {
                event.rename("json.recipients", "cyera.event.recipients")?;
            }

            if event.has_value("json.reportFileName") {
                event.rename("json.reportFileName", "cyera.event.report.file_name")?;
            }

            if event.has_value("json.reportInstanceUid") {
                event.rename("json.reportInstanceUid", "cyera.event.report.instance_uid")?;
            }

            if event.has_value("json.reportJobUid") {
                event.rename("json.reportJobUid", "cyera.event.report.job_uid")?;
            }

            if event.has_value("json.reportName") {
                event.rename("json.reportName", "cyera.event.report.name")?;
            }

            if event.has_value("json.reportType") {
                event.rename("json.reportType", "cyera.event.report.type")?;
            }

            if event.has_value("json.sku") {
                event.rename("json.sku", "cyera.event.sku")?;
            }

            let _cond = {
                event
                    .get("json.sourceClassifications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.sourceClassifications", |event| {
                    if event.has_value("_ingest._value.classificationName") {
                        event.rename("_ingest._value.classificationName", "_ingest._value.name")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.sourceClassifications") {
                event.rename(
                    "json.sourceClassifications",
                    "cyera.event.source_classifications",
                )?;
            }

            if event.has_value("json.subscriptionUid") {
                event.rename("json.subscriptionUid", "cyera.event.subscription_uid")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.successfulM365SensitivityLabelAssignmentsCount") {
                    if let Some(val) =
                        event.get("json.successfulM365SensitivityLabelAssignmentsCount")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.successfulM365SensitivityLabelAssignmentsCount".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cyera.event.successful_m365_sensitivity_label_assignments_count",
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
                    "convert_successfulM365SensitivityLabelAssignmentsCount_to_long_22d7bf88",
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
                    .get("json.targetClassifications")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.targetClassifications", |event| {
                    if event.has_value("_ingest._value.classificationName") {
                        event.rename("_ingest._value.classificationName", "_ingest._value.name")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.targetClassifications") {
                event.rename(
                    "json.targetClassifications",
                    "cyera.event.target_classifications",
                )?;
            }

            if event.has_value("json.targetSensitivityDisplayName") {
                event.rename(
                    "json.targetSensitivityDisplayName",
                    "cyera.event.target_sensitivity.display_name",
                )?;
            }

            if event.has_value("json.targetSensitivity") {
                event.rename(
                    "json.targetSensitivity",
                    "cyera.event.target_sensitivity.value",
                )?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "cyera.event.type")?;
            }

            if event.has_value("json.uid") {
                event.rename("json.uid", "cyera.event.uid")?;
            }

            if let Some(v) = event
                .get("cyera.event.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.user") {
                event.rename("json.user", "cyera.event.user")?;
            }

            if let Some(v) = event
                .get("cyera.event.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("cyera.event.user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cyera.event.user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.vendorLink") {
                event.rename("json.vendorLink", "cyera.event.vendor.link")?;
            }

            if event.has_value("json.vendorStatus") {
                event.rename("json.vendorStatus", "cyera.event.vendor.status")?;
            }

            if event.has_value("json.vendorTicketId") {
                event.rename("json.vendorTicketId", "cyera.event.vendor.ticket_id")?;
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
                event.remove("cyera.event.account.name");
                event.remove("cyera.event.account.uid");
                event.remove("cyera.event.cloud_provider");
                event.remove("cyera.event.datastore.infrastructure");
                event.remove("cyera.event.date");
                event.remove("cyera.event.uid");
                event.remove("cyera.event.user");
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
