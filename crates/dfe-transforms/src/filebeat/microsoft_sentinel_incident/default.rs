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

            event.set("event.kind", json!("alert"))?;

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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.properties.lastModifiedTimeUtc") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.etag") {
                event.rename("json.etag", "microsoft_sentinel.incident.etag")?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "microsoft_sentinel.incident.id")?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.incident.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "microsoft_sentinel.incident.name")?;
            }

            if event.has_value("json.properties.additionalData.alertProductNames") {
                event.rename(
                    "json.properties.additionalData.alertProductNames",
                    "microsoft_sentinel.incident.properties.additional_data.alert.product_names",
                )?;
            }

            let _cond = { event.get_str("json.properties.additionalData.alertsCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.additionalData.alertsCount") {
                        if let Some(val) = event.get("json.properties.additionalData.alertsCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.additionalData.alertsCount".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_sentinel.incident.properties.additional_data.alerts.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_additionalData_alertsCount_to_long",
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

            let _cond =
                { event.get_str("json.properties.additionalData.bookmarksCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.additionalData.bookmarksCount") {
                        if let Some(val) =
                            event.get("json.properties.additionalData.bookmarksCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.additionalData.bookmarksCount".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_sentinel.incident.properties.additional_data.bookmarks_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_additionalData_bookmarksCount_to_long",
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

            let _cond =
                { event.get_str("json.properties.additionalData.commentsCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.additionalData.commentsCount") {
                        if let Some(val) = event.get("json.properties.additionalData.commentsCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.additionalData.commentsCount".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_sentinel.incident.properties.additional_data.comments_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_properties_additionalData_commentsCount_to_long",
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

            if event.has_value("json.properties.additionalData.providerIncidentUrl") {
                event.rename(
                    "json.properties.additionalData.providerIncidentUrl",
                    "microsoft_sentinel.incident.properties.additional_data.provider_incident_url",
                )?;
            }

            if event.has_value("json.properties.additionalData.tactics") {
                event.rename(
                    "json.properties.additionalData.tactics",
                    "microsoft_sentinel.incident.properties.additional_data.tactics",
                )?;
            }

            let _cond = {
                event
                    .get("microsoft_sentinel.incident.properties.additional_data.tactics")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "microsoft_sentinel.incident.properties.additional_data.tactics",
                    |event| {
                        event.append_unique(
                            "threat.tactic.name",
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

            if event.has_value("json.properties.classification") {
                event.rename(
                    "json.properties.classification",
                    "microsoft_sentinel.incident.properties.classification",
                )?;
            }

            if event.has_value("json.properties.classificationComment") {
                event.rename(
                    "json.properties.classificationComment",
                    "microsoft_sentinel.incident.properties.classification_comment",
                )?;
            }

            if event.has_value("json.properties.classificationReason") {
                event.rename(
                    "json.properties.classificationReason",
                    "microsoft_sentinel.incident.properties.classification_reason",
                )?;
            }

            let _cond = {
                event.has_value("json.properties.createdTimeUtc")
                    && event.get_str("json.properties.createdTimeUtc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.createdTimeUtc") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.incident.properties.created_time_utc",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.createdTimeUtc".into(),
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
                        "date_properties_createdTimeUtc",
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
                .get("microsoft_sentinel.incident.properties.created_time_utc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has_value("json.properties.description") {
                event.rename(
                    "json.properties.description",
                    "microsoft_sentinel.incident.properties.description",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.incident.properties.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event.has_value("json.properties.firstActivityTimeUtc")
                    && event.get_str("json.properties.firstActivityTimeUtc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.properties.firstActivityTimeUtc")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.incident.properties.first_activity_time_utc",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.firstActivityTimeUtc".into(),
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
                        "date_properties_firstActivityTimeUtc",
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

            let _cond = { event.get_str("json.properties.incidentNumber") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.incidentNumber") {
                        if let Some(val) = event.get("json.properties.incidentNumber") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.incidentNumber".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_sentinel.incident.properties.incident.number",
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
                        "convert_properties_incidentNumber_to_long",
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

            if event.has_value("json.properties.incidentUrl") {
                event.rename(
                    "json.properties.incidentUrl",
                    "microsoft_sentinel.incident.properties.incident.url",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.incident.properties.incident.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            if event.has_value("json.properties.labels.labelName") {
                event.rename(
                    "json.properties.labels.labelName",
                    "microsoft_sentinel.incident.properties.labels.name",
                )?;
            }

            if event.has_value("json.properties.labels.labelType") {
                event.rename(
                    "json.properties.labels.labelType",
                    "microsoft_sentinel.incident.properties.labels.type",
                )?;
            }

            let _cond = {
                event.has_value("json.properties.lastActivityTimeUtc")
                    && event.get_str("json.properties.lastActivityTimeUtc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.properties.lastActivityTimeUtc")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.incident.properties.last_activity_time_utc",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.lastActivityTimeUtc".into(),
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
                        "date_properties_lastActivityTimeUtc",
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
                event.has_value("json.properties.lastModifiedTimeUtc")
                    && event.get_str("json.properties.lastModifiedTimeUtc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.properties.lastModifiedTimeUtc")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.incident.properties.last_modified_time_utc",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.lastModifiedTimeUtc".into(),
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
                        "date_properties_lastModifiedTimeUtc",
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
                .get("microsoft_sentinel.incident.properties.last_modified_time_utc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.properties.owner.assignedTo") {
                event.rename(
                    "json.properties.owner.assignedTo",
                    "microsoft_sentinel.incident.properties.owner.assigned_to",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.incident.properties.owner.assigned_to")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.owner.email") {
                event.rename(
                    "json.properties.owner.email",
                    "microsoft_sentinel.incident.properties.owner.email",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.incident.properties.owner.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = {
                event.has_value("user.email")
                    && event.get("user.email").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(input) = event.get_string("user.email") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp", &remaining[..pos]));
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
                            path: "user.email".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.owner.objectId") {
                event.rename(
                    "json.properties.owner.objectId",
                    "microsoft_sentinel.incident.properties.owner.object_id",
                )?;
            }

            if event.has_value("json.properties.owner.ownerType") {
                event.rename(
                    "json.properties.owner.ownerType",
                    "microsoft_sentinel.incident.properties.owner.type",
                )?;
            }

            let _cond = {
                event.has_value("microsoft_sentinel.incident.properties.owner.object_id")
                    && event
                        .get_str("microsoft_sentinel.incident.properties.owner.type")
                        .is_some_and(|s| s.to_lowercase() == "user")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_sentinel.incident.properties.owner.object_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.owner.userPrincipalName") {
                event.rename(
                    "json.properties.owner.userPrincipalName",
                    "microsoft_sentinel.incident.properties.owner.user_principal_name",
                )?;
            }

            let _cond = {
                event.has_value("microsoft_sentinel.incident.properties.owner.user_principal_name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_sentinel.incident.properties.owner.user_principal_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.providerIncidentId") {
                event.rename(
                    "json.properties.providerIncidentId",
                    "microsoft_sentinel.incident.properties.provider.incident_id",
                )?;
            }

            if event.has_value("json.properties.providerName") {
                event.rename(
                    "json.properties.providerName",
                    "microsoft_sentinel.incident.properties.provider.name",
                )?;
            }

            if event.has_value("json.properties.relatedAnalyticRuleIds") {
                event.rename(
                    "json.properties.relatedAnalyticRuleIds",
                    "microsoft_sentinel.incident.properties.related_analytic_rule_ids",
                )?;
            }

            let _cond = {
                event
                    .get("microsoft_sentinel.incident.properties.related_analytic_rule_ids")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "microsoft_sentinel.incident.properties.related_analytic_rule_ids",
                    |event| {
                        event.append_unique(
                            "rule.id",
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

            if event.has_value("json.properties.severity") {
                event.rename(
                    "json.properties.severity",
                    "microsoft_sentinel.incident.properties.severity",
                )?;
            }

            let _cond = { event.has_value("microsoft_sentinel.incident.properties.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def temp = ctx.microsoft_sentinel.incident.properties.severity.toLowerCase(); if (temp == 'informational') {\n  ctx.event.severity = 0;\n} else if (temp == 'low') {\n  ctx.event.severity = 1;\n} else if (temp == 'medium') {\n  ctx.event.severity = 2;\n} else if (temp == 'high') {\n  ctx.event.severity = 3;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def temp = ctx.microsoft_sentinel.incident.properties.severity.toLowerCase(); if (temp == 'informational') {\n  ctx.event.severity = 0;\n} else if (temp == 'low') {\n  ctx.event.severity = 1;\n} else if (temp == 'medium') {\n  ctx.event.severity = 2;\n} else if (temp == 'high') {\n  ctx.event.severity = 3;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_event_severity",
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

            if event.has_value("json.properties.status") {
                event.rename(
                    "json.properties.status",
                    "microsoft_sentinel.incident.properties.status",
                )?;
            }

            if event.has_value("json.properties.title") {
                event.rename(
                    "json.properties.title",
                    "microsoft_sentinel.incident.properties.title",
                )?;
            }

            let _cond = {
                event.has_value("json.systemData.createdAt")
                    && event.get_str("json.systemData.createdAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.systemData.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.incident.system_data.created_at",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.systemData.createdAt".into(),
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
                        "date_systemData_createdAt",
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

            if event.has_value("json.systemData.createdBy") {
                event.rename(
                    "json.systemData.createdBy",
                    "microsoft_sentinel.incident.system_data.created_by",
                )?;
            }

            if event.has_value("json.systemData.createdByType") {
                event.rename(
                    "json.systemData.createdByType",
                    "microsoft_sentinel.incident.system_data.created_by_type",
                )?;
            }

            let _cond = {
                event.has_value("microsoft_sentinel.incident.system_data.created_by")
                    && event
                        .get_str("microsoft_sentinel.incident.system_data.created_by_type")
                        .is_some_and(|s| s.to_lowercase() == "user")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_sentinel.incident.system_data.created_by")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.systemData.lastModifiedAt")
                    && event.get_str("json.systemData.lastModifiedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.systemData.lastModifiedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.incident.system_data.last_modified_at",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.systemData.lastModifiedAt".into(),
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
                        "date_systemData_lastModifiedAt",
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

            if event.has_value("json.systemData.lastModifiedBy") {
                event.rename(
                    "json.systemData.lastModifiedBy",
                    "microsoft_sentinel.incident.system_data.last_modified_by",
                )?;
            }

            if event.has_value("json.systemData.lastModifiedByType") {
                event.rename(
                    "json.systemData.lastModifiedByType",
                    "microsoft_sentinel.incident.system_data.last_modified_by_type",
                )?;
            }

            let _cond = {
                event.has_value("microsoft_sentinel.incident.system_data.last_modified_by")
                    && event
                        .get_str("microsoft_sentinel.incident.system_data.last_modified_by_type")
                        .is_some_and(|s| s.to_lowercase() == "user")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_sentinel.incident.system_data.last_modified_by")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "microsoft_sentinel.incident.type")?;
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
                event.remove("microsoft_sentinel.incident.id");
                event.remove("microsoft_sentinel.incident.properties.additional_data.tactics");
                event.remove("microsoft_sentinel.incident.properties.created_time_utc");
                event.remove("microsoft_sentinel.incident.properties.description");
                event.remove("microsoft_sentinel.incident.properties.incident.url");
                event.remove("microsoft_sentinel.incident.properties.last_modified_time_utc");
                event.remove("microsoft_sentinel.incident.properties.owner.assigned_to");
                event.remove("microsoft_sentinel.incident.properties.owner.email");
                event.remove("microsoft_sentinel.incident.properties.related_analytic_rule_ids");
            }

            event.remove("_temp");
            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
