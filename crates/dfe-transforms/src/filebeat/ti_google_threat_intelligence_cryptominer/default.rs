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
            let _cond = { event.get_str("message") == Some("retry") };
            if _cond {
                return Ok(TransformResult::Drop);
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
                if let Some(v) = event.get("json.data.attributes.last_modification_date") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.data.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("enrichment"))?;

            event.append("event.category", json!("threat"))?;

            event.append("event.type", json!("indicator"))?;

            event.set("observer.vendor", json!("Google"))?;

            event.set("observer.product", json!("Threat Intelligence"))?;

            event.set("threat.feed.name", json!("GTI Cryptominer"))?;

            event.set(
                "threat.feed.dashboard_id",
                Value::Array(vec![
                    json!("ti_google_threat_intelligence-0b0fb6b4-d250-4e31-a56a-bb872e4c7c4a"),
                    json!("ti_google_threat_intelligence-9e8de699-a623-4a1b-9f63-7d641116f531"),
                    json!("ti_google_threat_intelligence-95187e5c-b4a2-45ad-b6a4-d6ce68e1f43e"),
                ]),
            )?;

            if event.has_value("json.data.attributes.as_owner") {
                event.rename(
                    "json.data.attributes.as_owner",
                    "gti.cryptominer.attributes.as_owner",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.attributes.asn") {
                    if let Some(val) = event.get("json.data.attributes.asn") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.asn".into(),
                                message,
                            }
                        })?;
                        event.set("gti.cryptominer.attributes.as_number", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_data_attributes_asn_to_long",
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
                .get("gti.cryptominer.attributes.as_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.as.number", v)?;
            }

            if event.has_value("json.data.attributes.categories") {
                event.rename(
                    "json.data.attributes.categories",
                    "gti.cryptominer.attributes.categories",
                )?;
            }

            if event.has_value("json.data.attributes.continent") {
                event.rename(
                    "json.data.attributes.continent",
                    "gti.cryptominer.attributes.continent",
                )?;
            }

            if let Some(v) = event
                .get("gti.cryptominer.attributes.continent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.geo.continent_code", v)?;
            }

            if event.has_value("json.data.attributes.country") {
                event.rename(
                    "json.data.attributes.country",
                    "gti.cryptominer.attributes.country",
                )?;
            }

            if let Some(v) = event
                .get("gti.cryptominer.attributes.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.geo.country_iso_code", v)?;
            }

            let _cond = { event.has_value("json.data.attributes.creation_date") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.data.attributes.creation_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("gti.cryptominer.attributes.creation_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.attributes.creation_date".into(),
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
                        "date_data_attributes_creation_date",
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
                .get("gti.cryptominer.attributes.creation_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("event.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.first_seen", v)?;
            }

            let _cond = {
                event.has_value("json.data.attributes.first_submission_date")
                    && event.get_str("json.data.attributes.first_submission_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.data.attributes.first_submission_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("gti.cryptominer.attributes.first_submission_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.attributes.first_submission_date".into(),
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
                        "date_data_attributes_first_submission_date",
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

            if event.has_value("json.data.attributes.gti_assessment.severity.value") {
                event.rename(
                    "json.data.attributes.gti_assessment.severity.value",
                    "gti.cryptominer.attributes.gti_assessment.severity",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.attributes.gti_assessment.threat_score.value") {
                    if let Some(val) =
                        event.get("json.data.attributes.gti_assessment.threat_score.value")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.gti_assessment.threat_score.value"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.cryptominer.attributes.gti_assessment.threat_score",
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
                    "convert_data_attributes_gti_assessment_threat_score_to_long",
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

            if event.has_value("json.data.attributes.gti_assessment.verdict.value") {
                event.rename(
                    "json.data.attributes.gti_assessment.verdict.value",
                    "gti.cryptominer.attributes.gti_assessment.verdict",
                )?;
            }

            if event.has_value("json.data.attributes.jarm") {
                event.rename(
                    "json.data.attributes.jarm",
                    "gti.cryptominer.attributes.jarm",
                )?;
            }

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.cryptominer.attributes.jarm")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.has_value("json.data.attributes.last_analysis_date") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.data.attributes.last_analysis_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("gti.cryptominer.attributes.last_analysis_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.attributes.last_analysis_date".into(),
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
                        "date_data_attributes_last_analysis_date",
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
                .get("gti.cryptominer.attributes.last_analysis_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.last_seen", v)?;
            }

            if event.has_value("json.data.attributes.url") {
                event.rename("json.data.attributes.url", "gti.cryptominer.attributes.url")?;
            }

            if let Some(v) = event
                .get("gti.cryptominer.attributes.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.url.original", v)?;
            }

            if event.has_value("json.data.attributes.last_final_url") {
                event.rename(
                    "json.data.attributes.last_final_url",
                    "gti.cryptominer.attributes.last_final_url",
                )?;
            }

            if let Some(v) = event
                .get("threat.indicator.url.original")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.url.full", v)?;
            }

            if let Some(v) = event
                .get("gti.cryptominer.attributes.last_final_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.url.full", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.attributes.last_http_response_code") {
                    if let Some(val) = event.get("json.data.attributes.last_http_response_code") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.last_http_response_code".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.cryptominer.attributes.last_http_response_code",
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
                    "convert_data_attributes_last_http_response_code_to_long",
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
                .get("gti.cryptominer.attributes.last_http_response_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            let _cond = { event.has_value("json.data.attributes.last_modification_date") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.data.attributes.last_modification_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("gti.cryptominer.attributes.last_modification_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.attributes.last_modification_date".into(),
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
                        "date_data_attributes_last_modification_date",
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
                .get("gti.cryptominer.attributes.last_modification_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("gti.cryptominer.attributes.last_modification_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.modified_at", v)?;
            }

            let _cond = { event.has_value("json.data.attributes.last_submission_date") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.data.attributes.last_submission_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("gti.cryptominer.attributes.last_submission_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.attributes.last_submission_date".into(),
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
                        "date_data_attributes_last_submission_date",
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

            if event.has_value("json.data.attributes.md5") {
                event.rename("json.data.attributes.md5", "gti.cryptominer.attributes.md5")?;
            }

            if let Some(v) = event
                .get("gti.cryptominer.attributes.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.file.hash.md5", v)?;
            }

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.cryptominer.attributes.md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.data.attributes.meaningful_name") {
                event.rename(
                    "json.data.attributes.meaningful_name",
                    "gti.cryptominer.attributes.meaningful_name",
                )?;
            }

            if event.has_value("json.data.attributes.names") {
                event.rename(
                    "json.data.attributes.names",
                    "gti.cryptominer.attributes.names",
                )?;
            }

            if let Some(v) = event
                .get("gti.cryptominer.attributes.names")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.file.name", v)?;
            }

            if event.has_value("json.data.attributes.network") {
                event.rename(
                    "json.data.attributes.network",
                    "gti.cryptominer.attributes.network",
                )?;
            }

            if event.has_value("json.data.attributes.outgoing_links") {
                event.rename(
                    "json.data.attributes.outgoing_links",
                    "gti.cryptominer.attributes.outgoing_links",
                )?;
            }

            if event.has_value("json.data.attributes.regional_internet_registry") {
                event.rename(
                    "json.data.attributes.regional_internet_registry",
                    "gti.cryptominer.attributes.regional_internet_registry",
                )?;
            }

            if event.has_value("json.data.attributes.tags") {
                event.rename(
                    "json.data.attributes.tags",
                    "gti.cryptominer.attributes.tags",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.attributes.positives") {
                    if let Some(val) = event.get("json.data.attributes.positives") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.positives".into(),
                                message,
                            }
                        })?;
                        event.set("gti.cryptominer.attributes.positives", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_data_attributes_positives_to_long",
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
                if event.has_value("json.data.attributes.times_submitted") {
                    if let Some(val) = event.get("json.data.attributes.times_submitted") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.times_submitted".into(),
                                message,
                            }
                        })?;
                        event.set("gti.cryptominer.attributes.times_submitted", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_data_attributes_times_submitted_to_long",
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

            if event.has_value("json.data.attributes.title") {
                event.rename(
                    "json.data.attributes.title",
                    "gti.cryptominer.attributes.title",
                )?;
            }

            if event.has_value("json.data.attributes.type_tags") {
                event.rename(
                    "json.data.attributes.type_tags",
                    "gti.cryptominer.attributes.type_tags",
                )?;
            }

            if event.has_value("json.data.attributes.vhash") {
                event.rename(
                    "json.data.attributes.vhash",
                    "gti.cryptominer.attributes.vhash",
                )?;
            }

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.cryptominer.attributes.vhash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = {
                event
                    .get("json.data.attributes.tld")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.append(
                    "gti.cryptominer.attributes.top_level_domain",
                    json!(
                        event
                            .get("json.data.attributes.tld")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("json.data.attributes.tld")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("json.data.attributes.tld") {
                    event.rename(
                        "json.data.attributes.tld",
                        "gti.cryptominer.attributes.top_level_domain",
                    )?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.attributes.last_analysis_stats.harmless") {
                    if let Some(val) =
                        event.get("json.data.attributes.last_analysis_stats.harmless")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.last_analysis_stats.harmless".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.cryptominer.attributes.last_analysis_stats.harmless",
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
                    "convert_data_attributes_last_analysis_stats_harmless_to_long",
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
                if event.has_value("json.data.attributes.last_analysis_stats.malicious") {
                    if let Some(val) =
                        event.get("json.data.attributes.last_analysis_stats.malicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.last_analysis_stats.malicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.cryptominer.attributes.last_analysis_stats.malicious",
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
                    "convert_data_attributes_last_analysis_stats_malicious_to_long",
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
                if event.has_value("json.data.attributes.last_analysis_stats.suspicious") {
                    if let Some(val) =
                        event.get("json.data.attributes.last_analysis_stats.suspicious")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.last_analysis_stats.suspicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.cryptominer.attributes.last_analysis_stats.suspicious",
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
                    "convert_data_attributes_last_analysis_stats_suspicious_to_long",
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
                if event.has_value("json.data.attributes.last_analysis_stats.timeout") {
                    if let Some(val) = event.get("json.data.attributes.last_analysis_stats.timeout")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.last_analysis_stats.timeout".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.cryptominer.attributes.last_analysis_stats.timeout",
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
                    "convert_data_attributes_last_analysis_stats_timeout_to_long",
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
                if event.has_value("json.data.attributes.last_analysis_stats.undetected") {
                    if let Some(val) =
                        event.get("json.data.attributes.last_analysis_stats.undetected")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.attributes.last_analysis_stats.undetected".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.cryptominer.attributes.last_analysis_stats.undetected",
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
                    "convert_data_attributes_last_analysis_stats_undetected_to_long",
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

            if event.has_value("json.data.type") {
                event.rename("json.data.type", "gti.cryptominer.type")?;
            }

            let _cond = { event.has_value("gti.cryptominer.type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.threat == null) {\n  ctx.threat = new HashMap();\n}\nif (ctx.threat.indicator == null) {\n  ctx.threat.indicator = new HashMap();\n}\nctx.threat.indicator.type = params[ctx.gti.cryptominer.type];
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.threat == null) {\n  ctx.threat = new HashMap();\n}\nif (ctx.threat.indicator == null) {\n  ctx.threat.indicator = new HashMap();\n}\nctx.threat.indicator.type = params[ctx.gti.cryptominer.type];"#
                        ),
                        cached_params!(
                            "{\"domain\":\"domain-name\",\"file\":\"file\",\"ip_address\":\"ipv4-addr\",\"url\":\"url\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_threat_indicator_type_from_gti_cryptominer_type",
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

            event.append_unique(
                "threat.indicator.id",
                json!(
                    event
                        .get("json.data.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(v) = event
                .get("json.data.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            if event.has_value("json.data.id") {
                event.rename("json.data.id", "gti.cryptominer.id")?;
            }

            let _cond = { event.get_str("gti.cryptominer.type") == Some("domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("gti.cryptominer.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("gti.cryptominer.type") == Some("file") };
            if _cond {
                if let Some(v) = event
                    .get("gti.cryptominer.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.hash.sha256", v)?;
                }
            }

            let _cond = { event.get_str("gti.cryptominer.type") == Some("file") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("gti.cryptominer.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("gti.cryptominer.type") == Some("url") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("gti.cryptominer.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("gti.cryptominer.type") == Some("ip_address") };
            if _cond {
                if let Some(v) = event
                    .get("gti.cryptominer.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.ip", v)?;
                }
            }

            let _cond = { event.get_str("gti.cryptominer.type") == Some("ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("gti.cryptominer.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("threat.indicator.url.original") {
                uri_parts(event, "threat.indicator.url.original", "url", true, false)?;
            }

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("url.domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.data.attributes.last_analysis_results") {
                event.rename(
                    "json.data.attributes.last_analysis_results",
                    "gti.cryptominer.attributes.last_analysis_results",
                )?;
            }

            if event.has_value("json.data.relationships.campaigns.data") {
                event.rename(
                    "json.data.relationships.campaigns.data",
                    "gti.cryptominer.relationships.campaigns",
                )?;
            }

            if event.has_value("json.data.relationships.collections.data") {
                event.rename(
                    "json.data.relationships.collections.data",
                    "gti.cryptominer.relationships.collections",
                )?;
            }

            if event.has_value("json.data.relationships.malware_families.data") {
                event.rename(
                    "json.data.relationships.malware_families.data",
                    "gti.cryptominer.relationships.malware_families",
                )?;
            }

            if event.has_value("json.data.relationships.reports.data") {
                event.rename(
                    "json.data.relationships.reports.data",
                    "gti.cryptominer.relationships.reports",
                )?;
            }

            if event.has_value("json.data.relationships.software_toolkits.data") {
                event.rename(
                    "json.data.relationships.software_toolkits.data",
                    "gti.cryptominer.relationships.software_toolkits",
                )?;
            }

            if event.has_value("json.data.relationships.threat_actors.data") {
                event.rename(
                    "json.data.relationships.threat_actors.data",
                    "gti.cryptominer.relationships.threat_actors",
                )?;
            }

            if event.has_value("json.data.relationships.vulnerabilities.data") {
                event.rename(
                    "json.data.relationships.vulnerabilities.data",
                    "gti.cryptominer.relationships.vulnerabilities",
                )?;
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
