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
            event.set("ecs.version", json!("8.17.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { event.get_str("message") == Some("retry") };
            if _cond {
                return Ok(TransformResult::Drop);
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.internet_scanner_intelligence.last_seen_timestamp")
                {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.ip") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("enrichment"))?;

            event.append("event.category", json!("threat"))?;

            event.append("event.type", json!("indicator"))?;

            event.set("observer.product", json!("Threat Intelligence"))?;

            event.set("observer.vendor", json!("GreyNoise"))?;

            if event.has_value("json.ip") {
                event.rename("json.ip", "threat.indicator.ip")?;
            }

            if let Some(v) = event
                .get("threat.indicator.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("greynoise.ip.indicator.ip", v)?;
            }

            if let Some(v) = event
                .get("threat.indicator.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.business_service_intelligence.found") {
                    if let Some(val) = event.get("json.business_service_intelligence.found") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.business_service_intelligence.found".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "greynoise.ip.business_service_intelligence.found",
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
                    "convert_business_service_intelligence_found_to_boolean_e08b8a3a",
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

            if event.has_value("json.business_service_intelligence.category") {
                event.rename(
                    "json.business_service_intelligence.category",
                    "greynoise.ip.business_service_intelligence.category",
                )?;
            }

            if event.has_value("json.business_service_intelligence.description") {
                event.rename(
                    "json.business_service_intelligence.description",
                    "greynoise.ip.business_service_intelligence.description",
                )?;
            }

            if event.has_value("json.business_service_intelligence.explanation") {
                event.rename(
                    "json.business_service_intelligence.explanation",
                    "greynoise.ip.business_service_intelligence.explanation",
                )?;
            }

            let _cond = {
                event.has_value("json.business_service_intelligence.last_updated")
                    && event.get_str("json.business_service_intelligence.last_updated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.business_service_intelligence.last_updated")
                    {
                        match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss'Z'"], None, None) {
                            Some(parsed) => event.set(
                                "greynoise.ip.business_service_intelligence.last_updated",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.business_service_intelligence.last_updated".into(),
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
                        "date_business_service_intelligence_last_updated_9b8bb192",
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

            if event.has_value("json.business_service_intelligence.name") {
                event.rename(
                    "json.business_service_intelligence.name",
                    "greynoise.ip.business_service_intelligence.name",
                )?;
            }

            if event.has_value("json.business_service_intelligence.reference") {
                event.rename(
                    "json.business_service_intelligence.reference",
                    "greynoise.ip.business_service_intelligence.reference",
                )?;
            }

            if let Some(v) = event
                .get("greynoise.ip.business_service_intelligence.reference")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.url.full", v)?;
            }

            if event.has_value("json.business_service_intelligence.trust_level") {
                event.rename(
                    "json.business_service_intelligence.trust_level",
                    "greynoise.ip.business_service_intelligence.trust_level",
                )?;
            }

            if event.has_value("json.internet_scanner_intelligence.actor") {
                event.rename(
                    "json.internet_scanner_intelligence.actor",
                    "greynoise.ip.internet_scanner_intelligence.actor",
                )?;
            }

            if let Some(v) = event
                .get("greynoise.ip.internet_scanner_intelligence.actor")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.internet_scanner_intelligence.bot") {
                    if let Some(val) = event.get("json.internet_scanner_intelligence.bot") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.internet_scanner_intelligence.bot".into(),
                                message,
                            }
                        })?;
                        event.set("greynoise.ip.internet_scanner_intelligence.bot", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_internet_scanner_intelligence_bot_to_boolean_ed563d75",
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

            if event.has_value("json.internet_scanner_intelligence.classification") {
                event.rename(
                    "json.internet_scanner_intelligence.classification",
                    "greynoise.ip.internet_scanner_intelligence.classification",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.internet_scanner_intelligence.found") {
                    if let Some(val) = event.get("json.internet_scanner_intelligence.found") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.internet_scanner_intelligence.found".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "greynoise.ip.internet_scanner_intelligence.found",
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
                    "convert_internet_scanner_intelligence_found_to_boolean_bacbc3ce",
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
                event.has_value("json.internet_scanner_intelligence.last_seen")
                    && event.get_str("json.internet_scanner_intelligence.last_seen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.internet_scanner_intelligence.last_seen")
                    {
                        match parse_date_out(&date_str, &["yyyy-MM-dd"], None, None) {
                            Some(parsed) => event.set(
                                "greynoise.ip.internet_scanner_intelligence.last_seen",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.internet_scanner_intelligence.last_seen".into(),
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
                        "date_internet_scanner_intelligence_last_seen_aa6a3d49",
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
                event.has_value("json.internet_scanner_intelligence.last_seen_timestamp")
                    && event.get_str("json.internet_scanner_intelligence.last_seen_timestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("json.internet_scanner_intelligence.last_seen_timestamp")
                    {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss", "ISO8601"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "greynoise.ip.internet_scanner_intelligence.last_seen_timestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.internet_scanner_intelligence.last_seen_timestamp"
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
                        "date_internet_scanner_intelligence_last_seen_timestamp_9da4028c",
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
                .get("greynoise.ip.internet_scanner_intelligence.last_seen_timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("greynoise.ip.internet_scanner_intelligence.last_seen_timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.last_seen", v)?;
            }

            if event.has_value("json.internet_scanner_intelligence.metadata.asn") {
                event.rename(
                    "json.internet_scanner_intelligence.metadata.asn",
                    "greynoise.ip.internet_scanner_intelligence.metadata.asn",
                )?;
            }

            let _cond = {
                event.has_value("greynoise.ip.internet_scanner_intelligence.metadata.asn")
                    && event.get_str("greynoise.ip.internet_scanner_intelligence.metadata.asn")
                        != Some("")
            };
            if _cond {
                gsub_field(
                    event,
                    "greynoise.ip.internet_scanner_intelligence.metadata.asn",
                    "threat.indicator.as.number",
                    cached_regex!("AS"),
                    "",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("threat.indicator.as.number") {
                    if let Some(val) = event.get("threat.indicator.as.number") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "threat.indicator.as.number".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.as.number", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_threat_indicator_as_number_69f78850",
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
                if event.remove("threat.indicator.as.number").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "threat.indicator.as.number".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.internet_scanner_intelligence.metadata.category") {
                event.rename(
                    "json.internet_scanner_intelligence.metadata.category",
                    "greynoise.ip.internet_scanner_intelligence.metadata.category",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.internet_scanner_intelligence.metadata.mobile") {
                    if let Some(val) =
                        event.get("json.internet_scanner_intelligence.metadata.mobile")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.internet_scanner_intelligence.metadata.mobile".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "greynoise.ip.internet_scanner_intelligence.metadata.mobile",
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
                    "convert_internet_scanner_intelligence_metadata_mobile_to_boolean_b2444528",
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

            if event.has_value("json.internet_scanner_intelligence.metadata.organization") {
                event.rename(
                    "json.internet_scanner_intelligence.metadata.organization",
                    "greynoise.ip.internet_scanner_intelligence.metadata.organization",
                )?;
            }

            if let Some(v) = event
                .get("greynoise.ip.internet_scanner_intelligence.metadata.organization")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.as.organization.name", v)?;
            }

            if event.has_value("json.internet_scanner_intelligence.metadata.rdns") {
                event.rename(
                    "json.internet_scanner_intelligence.metadata.rdns",
                    "greynoise.ip.internet_scanner_intelligence.metadata.rdns",
                )?;
            }

            if event.has_value("json.internet_scanner_intelligence.metadata.region") {
                event.rename(
                    "json.internet_scanner_intelligence.metadata.region",
                    "greynoise.ip.internet_scanner_intelligence.metadata.region",
                )?;
            }

            if let Some(v) = event
                .get("greynoise.ip.internet_scanner_intelligence.metadata.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.geo.region_name", v)?;
            }

            if event.has_value("json.internet_scanner_intelligence.metadata.source_city") {
                event.rename(
                    "json.internet_scanner_intelligence.metadata.source_city",
                    "greynoise.ip.internet_scanner_intelligence.metadata.source_city",
                )?;
            }

            if let Some(v) = event
                .get("greynoise.ip.internet_scanner_intelligence.metadata.source_city")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.geo.city_name", v)?;
            }

            if event.has_value("json.internet_scanner_intelligence.metadata.source_country") {
                event.rename(
                    "json.internet_scanner_intelligence.metadata.source_country",
                    "greynoise.ip.internet_scanner_intelligence.metadata.source_country",
                )?;
            }

            if let Some(v) = event
                .get("greynoise.ip.internet_scanner_intelligence.metadata.source_country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.geo.country_name", v)?;
            }

            if event.has_value("json.internet_scanner_intelligence.metadata.source_country_code") {
                event.rename(
                    "json.internet_scanner_intelligence.metadata.source_country_code",
                    "greynoise.ip.internet_scanner_intelligence.metadata.source_country_code",
                )?;
            }

            if let Some(v) = event
                .get("greynoise.ip.internet_scanner_intelligence.metadata.source_country_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.geo.country_iso_code", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.internet_scanner_intelligence.spoofable") {
                    if let Some(val) = event.get("json.internet_scanner_intelligence.spoofable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.internet_scanner_intelligence.spoofable".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "greynoise.ip.internet_scanner_intelligence.spoofable",
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
                    "convert_internet_scanner_intelligence_spoofable_to_boolean_44fd2792",
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
                    .get("json.internet_scanner_intelligence.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.internet_scanner_intelligence.tags", |event| {
                    event.append_unique(
                        "greynoise.ip.internet_scanner_intelligence.tag.names",
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
                    .get("greynoise.ip.internet_scanner_intelligence.tag.names")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "greynoise.ip.internet_scanner_intelligence.tag.names",
                    |event| {
                        event.append_unique(
                            "tags",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.internet_scanner_intelligence.tor") {
                    if let Some(val) = event.get("json.internet_scanner_intelligence.tor") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.internet_scanner_intelligence.tor".into(),
                                message,
                            }
                        })?;
                        event.set("greynoise.ip.internet_scanner_intelligence.tor", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_internet_scanner_intelligence_tor_to_boolean_7f76a778",
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
                if event.has_value("json.internet_scanner_intelligence.vpn") {
                    if let Some(val) = event.get("json.internet_scanner_intelligence.vpn") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.internet_scanner_intelligence.vpn".into(),
                                message,
                            }
                        })?;
                        event.set("greynoise.ip.internet_scanner_intelligence.vpn", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_internet_scanner_intelligence_vpn_to_boolean_2d9e6184",
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

            if event.has_value("json.internet_scanner_intelligence.vpn_service") {
                event.rename(
                    "json.internet_scanner_intelligence.vpn_service",
                    "greynoise.ip.internet_scanner_intelligence.vpn_service",
                )?;
            }

            let _cond = {
                event.has_value("threat.indicator.ip")
                    && event.get_str("threat.indicator.ip") != Some("")
            };
            if _cond {
                event.set(
                    "threat.indicator.reference",
                    json!(format!(
                        "https://www.greynoise.io/ip/{}",
                        event
                            .get("threat.indicator.ip")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            event.set("threat.indicator.provider", json!("GreyNoise"))?;

            let _cond = {
                event.has_value("threat.indicator.ip")
                    && event.get_str("threat.indicator.ip") != Some("")
                    && event.has_value("greynoise.ip.internet_scanner_intelligence.classification")
                    && event.get_str("greynoise.ip.internet_scanner_intelligence.classification")
                        != Some("")
            };
            if _cond {
                event.set("threat.indicator.description", json!(format!("{} IP has been observed mass scanning the internet by GreyNoise with a classification of {}", event.get("threat.indicator.ip").map_or_else(String::new, template_to_string), event.get("greynoise.ip.internet_scanner_intelligence.classification").map_or_else(String::new, template_to_string))))?;
            }

            event.set("threat.indicator.type", json!("ipv4-addr"))?;

            event.set(
                "threat.feed.description",
                json!("Threat feed from the GreyNoise cybersecurity platform"),
            )?;

            event.set("threat.feed.name", json!("GreyNoise IP"))?;

            event.set(
                "threat.feed.reference",
                json!("https://docs.greynoise.io/docs/using-greynoise-as-an-indicator-feed"),
            )?;

            let _cond = {
                event.has_value("threat.indicator.ip")
                    && event.get_str("threat.indicator.ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("threat.indicator.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
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
