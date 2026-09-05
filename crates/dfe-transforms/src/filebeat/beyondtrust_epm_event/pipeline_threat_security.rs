// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_threat_security` pipeline.
pub struct PipelineThreatSecurity;

impl Transform for PipelineThreatSecurity {
    fn name(&self) -> &str {
        "pipeline_threat_security"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.owner")
                .is_some_and(|v| v.is_string())
        };
        if _cond {
            event.rename(
                "beyondtrust_epm.event.threat.indicator.file.owner",
                "beyondtrust_epm.event.threat.indicator.file.owner_keyword",
            )?;
        }

        let v = json!(
            event
                .get("beyondtrust_epm.event.threat.indicator.file.owner_keyword")
                .map_or_else(String::new, template_to_string)
        );
        if !painless_is_empty_value(&v) {
            event.set("threat.indicator.file.owner", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.EPMWinMac.Configuration.Workstyle.Description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("rule.description", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.EPMWinMac.Configuration.Workstyle.Identifier")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("rule.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.EPMWinMac.Configuration.Workstyle.Name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("rule.name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.rule.author")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.rule.author", |event| {
                event.append_unique(
                    "rule.author",
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
            .get("beyondtrust_epm.event.rule.category")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("rule.category", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.rule.description") };
        if _cond {
            event.append_unique(
                "rule.description",
                json!(
                    event
                        .get("beyondtrust_epm.event.rule.description")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.rule.id") };
        if _cond {
            event.append_unique(
                "rule.id",
                json!(
                    event
                        .get("beyondtrust_epm.event.rule.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.rule.license")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("rule.license", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.rule.name") };
        if _cond {
            event.append_unique(
                "rule.name",
                json!(
                    event
                        .get("beyondtrust_epm.event.rule.name")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.rule.reference")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("rule.reference", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.rule.ruleset")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("rule.ruleset", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.rule.uuid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("rule.uuid", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.rule.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("rule.version", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.enrichments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.enrichments", |event| {
                event.append_unique(
                    "threat.enrichments.matched.atomic",
                    json!(
                        event
                            .get("_ingest._value.matched.atomic")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.enrichments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.enrichments", |event| {
                event.append_unique(
                    "threat.enrichments.matched.field",
                    json!(
                        event
                            .get("_ingest._value.matched.field")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.enrichments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.enrichments", |event| {
                event.append_unique(
                    "threat.enrichments.matched.id",
                    json!(
                        event
                            .get("_ingest._value.matched.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.enrichments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.enrichments", |event| {
                event.append_unique(
                    "threat.enrichments.matched.index",
                    json!(
                        event
                            .get("_ingest._value.matched.index")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.enrichments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event
                    .get("beyondtrust_epm.event.threat.enrichments")
                    .cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                    Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                    Some(Value::Object(fields)) => {
                        fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                    }
                    _ => Vec::new(),
                };
                if !entries.is_empty() {
                    // A NESTED loop borrows the same slots, so the enclosing
                    // entry is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let enclosing_key = event.get("_ingest._key").cloned();
                    let mut list = Vec::with_capacity(entries.len());
                    let mut fields = Map::new();
                    for (key, item) in entries {
                        if let Some(key) = key.as_deref() {
                            event.set("_ingest._key", Value::String(key.to_string()))?;
                        }
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) =
                                event.get_as_string("_ingest._value.matched.occurred")
                            {
                                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                    Some(parsed) => {
                                        event.set("_ingest._value.matched.occurred", parsed)?
                                    }
                                    None => {
                                        return Err(TransformError::ParseError {
                                            path: "_ingest._value.matched.occurred".into(),
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
                                "date_threat_enrichments_matched_occurred",
                            )?;
                            event.remove("_ingest._value.matched.occurred");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        let left = event.remove("_ingest._value");
                        match key {
                            // An entry the body renamed AWAY is gone from the
                            // object, which is how a foreach lifts fields up.
                            Some(key) => {
                                if let Some(value) = left {
                                    fields.insert(key, value);
                                }
                            }
                            None => list.push(left.unwrap_or(Value::Null)),
                        }
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    if let Some(previous) = enclosing_key {
                        event.set("_ingest._key", previous)?;
                    }
                    event.set(
                        "beyondtrust_epm.event.threat.enrichments",
                        if keyed {
                            Value::Object(fields)
                        } else {
                            Value::Array(list)
                        },
                    )?;
                }
            }
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.enrichments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.enrichments", |event| {
                event.append_unique(
                    "threat.enrichments.matched.occurred",
                    json!(
                        event
                            .get("_ingest._value.matched.occurred")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.enrichments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.enrichments", |event| {
                event.append_unique(
                    "threat.enrichments.matched.type",
                    json!(
                        event
                            .get("_ingest._value.matched.type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.feed.dashboard_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.feed.dashboard_id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.feed.description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.feed.description", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.feed.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.feed.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.feed.reference")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.feed.reference", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.framework")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.framework", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.group.alias")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.group.alias", |event| {
                event.append_unique(
                    "threat.group.alias",
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
            .get("beyondtrust_epm.event.threat.group.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.group.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.group.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.group.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.group.reference")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.group.reference", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.as.number") {
                if let Some(val) = event.get("beyondtrust_epm.event.threat.indicator.as.number") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.as.number".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.as.number",
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
                "convert_threat_indicator_as_number_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.as.number");
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
            .get("beyondtrust_epm.event.threat.indicator.as.number")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.as.number", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.as.organization.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.as.organization.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.confidence")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.confidence", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.description", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.email.address")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.email.address", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.file.accessed")
                && event.get_str("beyondtrust_epm.event.threat.indicator.file.accessed") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.threat.indicator.file.accessed")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.threat.indicator.file.accessed",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.file.accessed".into(),
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
                    "date_threat_indicator_file_accessed",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.file.accessed");
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
            .get("beyondtrust_epm.event.threat.indicator.file.accessed")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.accessed", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.attributes")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.attributes",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.attributes",
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
            .get("beyondtrust_epm.event.threat.indicator.file.code_signature.digest_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.code_signature.digest_algorithm", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.file.code_signature.exists")
            {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.file.code_signature.exists")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.threat.indicator.file.code_signature.exists"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.file.code_signature.exists",
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
                "convert_threat_indicator_file_code_signature_exists_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.exists");
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
            .get("beyondtrust_epm.event.threat.indicator.file.code_signature.exists")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.code_signature.exists", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.code_signature.signing_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.code_signature.signing_id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.code_signature.status")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.code_signature.status", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.code_signature.subject_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.code_signature.subject_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.code_signature.team_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.code_signature.team_id", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.file.code_signature.timestamp")
                && event
                    .get_str("beyondtrust_epm.event.threat.indicator.file.code_signature.timestamp")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string(
                    "beyondtrust_epm.event.threat.indicator.file.code_signature.timestamp",
                ) {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.threat.indicator.file.code_signature.timestamp",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.file.code_signature.timestamp".into(),
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
                    "date_threat_indicator_file_code_signature_timestamp",
                )?;
                event
                    .remove("beyondtrust_epm.event.threat.indicator.file.code_signature.timestamp");
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
            .get("beyondtrust_epm.event.threat.indicator.file.code_signature.timestamp")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.code_signature.timestamp", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.file.code_signature.trusted")
            {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.file.code_signature.trusted")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.threat.indicator.file.code_signature.trusted"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.file.code_signature.trusted",
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
                "convert_threat_indicator_file_code_signature_trusted_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.trusted");
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
            .get("beyondtrust_epm.event.threat.indicator.file.code_signature.trusted")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.code_signature.trusted", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.file.code_signature.valid") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.file.code_signature.valid")
                {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.threat.indicator.file.code_signature.valid"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.file.code_signature.valid",
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
                "convert_threat_indicator_file_code_signature_valid_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.valid");
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
            .get("beyondtrust_epm.event.threat.indicator.file.code_signature.valid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.code_signature.valid", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.file.created")
                && event.get_str("beyondtrust_epm.event.threat.indicator.file.created") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.threat.indicator.file.created")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.threat.indicator.file.created",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.file.created".into(),
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
                    "date_threat_indicator_file_created",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.file.created");
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
            .get("beyondtrust_epm.event.threat.indicator.file.created")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.created", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.file.ctime")
                && event.get_str("beyondtrust_epm.event.threat.indicator.file.ctime") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.threat.indicator.file.ctime")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event
                            .set("beyondtrust_epm.event.threat.indicator.file.ctime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.file.ctime".into(),
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
                    "date_threat_indicator_file_ctime",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.file.ctime");
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
            .get("beyondtrust_epm.event.threat.indicator.file.ctime")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.ctime", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.device")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.device", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.directory")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.directory", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.drive_letter")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.drive_letter", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.architecture")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.architecture", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.byte_order")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.byte_order", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.cpu_type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.cpu_type", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.file.elf.creation_date")
                && event.get_str("beyondtrust_epm.event.threat.indicator.file.elf.creation_date")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event
                    .get_as_string("beyondtrust_epm.event.threat.indicator.file.elf.creation_date")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.threat.indicator.file.elf.creation_date",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path:
                                    "beyondtrust_epm.event.threat.indicator.file.elf.creation_date"
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
                    "date_threat_indicator_file_elf_creation_date",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.file.elf.creation_date");
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
            .get("beyondtrust_epm.event.threat.indicator.file.elf.creation_date")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.creation_date", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.header.abi_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.header.abi_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.header.class")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.header.class", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.header.data")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.header.data", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.file.elf.header.entrypoint")
            {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.file.elf.header.entrypoint")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.threat.indicator.file.elf.header.entrypoint"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.file.elf.header.entrypoint",
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
                "convert_threat_indicator_file_elf_header_entrypoint_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.file.elf.header.entrypoint");
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
            .get("beyondtrust_epm.event.threat.indicator.file.elf.header.entrypoint")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.header.entrypoint", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.header.object_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.header.object_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.header.os_abi")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.header.os_abi", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.header.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.header.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.header.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.header.version", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
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
                            "convert_threat_indicator_file_elf_sections_chi2_to_long",
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
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
                            "convert_threat_indicator_file_elf_sections_entropy_to_long",
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.elf.sections.flags",
                        json!(
                            event
                                .get("_ingest._value.flags")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.elf.sections.name",
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
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.elf.sections.physical_offset",
                        json!(
                            event
                                .get("_ingest._value.physical_offset")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
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
                            "convert_threat_indicator_file_elf_sections_physical_size_to_long",
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.elf.sections.type",
                        json!(
                            event
                                .get("_ingest._value.type")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
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
                            "convert_threat_indicator_file_elf_sections_virtual_address_to_long",
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
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
                            "convert_threat_indicator_file_elf_sections_virtual_size_to_long",
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
                },
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.sections", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.segments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.segments",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.elf.segments.sections",
                        json!(
                            event
                                .get("_ingest._value.sections")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.segments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.segments",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.elf.segments.type",
                        json!(
                            event
                                .get("_ingest._value.type")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.shared_libraries")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.shared_libraries",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.elf.shared_libraries",
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
            .get("beyondtrust_epm.event.threat.indicator.file.elf.telfhash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.elf.telfhash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.elf.telfhash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.elf.telfhash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.extension")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.extension", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.fork_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.fork_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.gid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.gid", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.group")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.group", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.hash.md5")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.hash.md5", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.hash.md5") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.hash.md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.hash.sha1")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.hash.sha1", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.hash.sha1") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.hash.sha1")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.hash.sha256")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.hash.sha256", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.hash.sha256") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.hash.sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.hash.sha384")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.hash.sha384", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.hash.sha384") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.hash.sha384")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.hash.sha512")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.hash.sha512", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.hash.sha512") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.hash.sha512")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.hash.ssdeep")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.hash.ssdeep", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.hash.ssdeep") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.hash.ssdeep")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.hash.tlsh")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.hash.tlsh", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.hash.tlsh") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.hash.tlsh")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.inode")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.inode", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.mime_type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.mime_type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.mode")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.mode", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.file.mtime")
                && event.get_str("beyondtrust_epm.event.threat.indicator.file.mtime") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.threat.indicator.file.mtime")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event
                            .set("beyondtrust_epm.event.threat.indicator.file.mtime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.file.mtime".into(),
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
                    "date_threat_indicator_file_mtime",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.file.mtime");
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
            .get("beyondtrust_epm.event.threat.indicator.file.mtime")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.mtime", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.path", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.pe.architecture")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.pe.architecture", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.pe.company")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.pe.company", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.pe.description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.pe.description", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.pe.file_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.pe.file_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.pe.imphash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.pe.imphash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.pe.imphash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.pe.imphash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.pe.original_file_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.pe.original_file_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.pe.pehash")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.pe.pehash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.file.pe.pehash") };
        if _cond {
            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.file.pe.pehash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.pe.product")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.pe.product", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.file.size") {
                if let Some(val) = event.get("beyondtrust_epm.event.threat.indicator.file.size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.file.size".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.file.size",
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
                "convert_threat_indicator_file_size_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.file.size");
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
            .get("beyondtrust_epm.event.threat.indicator.file.size")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.size", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.target_path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.target_path", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.uid")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.uid", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.x509.alternative_names")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.alternative_names",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.alternative_names",
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
                .get("beyondtrust_epm.event.threat.indicator.file.x509.issuer.common_name")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.issuer.common_name",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.issuer.common_name",
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
                .get("beyondtrust_epm.event.threat.indicator.file.x509.issuer.country")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.issuer.country",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.issuer.country",
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
            .get("beyondtrust_epm.event.threat.indicator.file.x509.issuer.distinguished_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.issuer.distinguished_name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.x509.issuer.locality")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.issuer.locality",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.issuer.locality",
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
                .get("beyondtrust_epm.event.threat.indicator.file.x509.issuer.organization")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.issuer.organization",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.issuer.organization",
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
                .get("beyondtrust_epm.event.threat.indicator.file.x509.issuer.organizational_unit")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.issuer.organizational_unit",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.issuer.organizational_unit",
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
                .get("beyondtrust_epm.event.threat.indicator.file.x509.issuer.state_or_province")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.issuer.state_or_province",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.issuer.state_or_province",
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
            event.has_value("beyondtrust_epm.event.threat.indicator.file.x509.not_after")
                && event.get_str("beyondtrust_epm.event.threat.indicator.file.x509.not_after")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event
                    .get_as_string("beyondtrust_epm.event.threat.indicator.file.x509.not_after")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.threat.indicator.file.x509.not_after",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.file.x509.not_after"
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
                    "date_threat_indicator_file_x509_not_after",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.file.x509.not_after");
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
            .get("beyondtrust_epm.event.threat.indicator.file.x509.not_after")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.not_after", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.file.x509.not_before")
                && event.get_str("beyondtrust_epm.event.threat.indicator.file.x509.not_before")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event
                    .get_as_string("beyondtrust_epm.event.threat.indicator.file.x509.not_before")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.threat.indicator.file.x509.not_before",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.file.x509.not_before"
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
                    "date_threat_indicator_file_x509_not_before",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.file.x509.not_before");
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
            .get("beyondtrust_epm.event.threat.indicator.file.x509.not_before")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.not_before", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.x509.public_key_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.public_key_algorithm", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.x509.public_key_curve")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.public_key_curve", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event
                .has_value("beyondtrust_epm.event.threat.indicator.file.x509.public_key_exponent")
            {
                if let Some(val) = event
                    .get("beyondtrust_epm.event.threat.indicator.file.x509.public_key_exponent")
                {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.threat.indicator.file.x509.public_key_exponent".into(),
                        message,
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.file.x509.public_key_exponent",
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
                "convert_threat_indicator_file_x509_public_key_exponent_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.file.x509.public_key_exponent");
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
            .get("beyondtrust_epm.event.threat.indicator.file.x509.public_key_exponent")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.public_key_exponent", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.file.x509.public_key_size") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.file.x509.public_key_size")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path:
                                "beyondtrust_epm.event.threat.indicator.file.x509.public_key_size"
                                    .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.file.x509.public_key_size",
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
                "convert_threat_indicator_file_x509_public_key_size_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.file.x509.public_key_size");
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
            .get("beyondtrust_epm.event.threat.indicator.file.x509.public_key_size")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.public_key_size", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.x509.serial_number")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.serial_number", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.file.x509.signature_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.signature_algorithm", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.x509.subject.common_name")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.subject.common_name",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.subject.common_name",
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
                .get("beyondtrust_epm.event.threat.indicator.file.x509.subject.country")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.subject.country",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.subject.country",
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
            .get("beyondtrust_epm.event.threat.indicator.file.x509.subject.distinguished_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.subject.distinguished_name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.x509.subject.locality")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.subject.locality",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.subject.locality",
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
                .get("beyondtrust_epm.event.threat.indicator.file.x509.subject.organization")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.subject.organization",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.subject.organization",
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
                .get("beyondtrust_epm.event.threat.indicator.file.x509.subject.organizational_unit")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.subject.organizational_unit",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.subject.organizational_unit",
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
                .get("beyondtrust_epm.event.threat.indicator.file.x509.subject.state_or_province")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.x509.subject.state_or_province",
                |event| {
                    event.append_unique(
                        "threat.indicator.file.x509.subject.state_or_province",
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
            .get("beyondtrust_epm.event.threat.indicator.file.x509.version_number")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.file.x509.version_number", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.first_seen")
                && event.get_str("beyondtrust_epm.event.threat.indicator.first_seen") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.threat.indicator.first_seen")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event
                            .set("beyondtrust_epm.event.threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.first_seen".into(),
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
                    "date_threat_indicator_first_seen",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.first_seen");
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
            .get("beyondtrust_epm.event.threat.indicator.first_seen")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.first_seen", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.geo.TimezoneOffset") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.geo.TimezoneOffset")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.geo.TimezoneOffset"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.geo.TimezoneOffset",
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
                "convert_threat_indicator_geo_TimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.geo.TimezoneOffset");
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
            .get("beyondtrust_epm.event.threat.indicator.geo.city_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.city_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.continent_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.continent_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.continent_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.continent_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.country_iso_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.country_iso_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.country_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.country_name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.geo.location.lat") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.geo.location.lat")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.geo.location.lat".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.geo.location.lat",
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
                "convert_threat_indicator_geo_location_lat_to_double",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.geo.location.lat");
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
            if event.has_value("beyondtrust_epm.event.threat.indicator.geo.location.lon") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.geo.location.lon")
                {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.geo.location.lon".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.geo.location.lon",
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
                "convert_threat_indicator_geo_location_lon_to_double",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.geo.location.lon");
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
            event.has_value("beyondtrust_epm.event.threat.indicator.geo.location.lat")
                && event.has_value("beyondtrust_epm.event.threat.indicator.geo.location.lon")
        };
        if _cond {
            // Painless script
            // Source: def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.threat.indicator.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.threat.indicator.geo.location.lon);\nctx.beyondtrust_epm.event.threat.indicator.geo.location = location;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.threat.indicator.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.threat.indicator.geo.location.lon);\nctx.beyondtrust_epm.event.threat.indicator.geo.location = location;"#
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.location")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.location", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.postal_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.postal_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.region_iso_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.region_iso_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.region_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.region_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.geo.timezone")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.geo.timezone", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.ip") };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.event.threat.indicator.ip") {
                    if let Some(val) = event.get("beyondtrust_epm.event.threat.indicator.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.ip".into(),
                                message,
                            }
                        })?;
                        event.set("beyondtrust_epm.event.threat.indicator.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_threat_indicator_ip_to_ip",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.ip");
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
            .get("beyondtrust_epm.event.threat.indicator.ip")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.ip") };
        if _cond {
            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.last_seen")
                && event.get_str("beyondtrust_epm.event.threat.indicator.last_seen") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.threat.indicator.last_seen")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("beyondtrust_epm.event.threat.indicator.last_seen", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.last_seen".into(),
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
                    "date_threat_indicator_last_seen",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.last_seen");
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
            .get("beyondtrust_epm.event.threat.indicator.last_seen")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.last_seen", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.marking.tlp")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.marking.tlp", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.modified_at")
                && event.get_str("beyondtrust_epm.event.threat.indicator.modified_at") != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.threat.indicator.modified_at")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event
                            .set("beyondtrust_epm.event.threat.indicator.modified_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.modified_at".into(),
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
                    "date_threat_indicator_modified_at",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.modified_at");
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
            .get("beyondtrust_epm.event.threat.indicator.modified_at")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.modified_at", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.port") {
                if let Some(val) = event.get("beyondtrust_epm.event.threat.indicator.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.port".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.threat.indicator.port", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_threat_indicator_port_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.port");
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
            .get("beyondtrust_epm.event.threat.indicator.port")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.port", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.provider")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.provider", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.reference")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.reference", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.registry.data.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.registry.data.bytes", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.registry.data.strings")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.registry.data.strings",
                |event| {
                    event.append_unique(
                        "threat.indicator.registry.data.strings",
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
            .get("beyondtrust_epm.event.threat.indicator.registry.data.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.registry.data.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.registry.hive")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.registry.hive", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.registry.key")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.registry.key", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.registry.path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.registry.path", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.registry.value")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.registry.value", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.scanner_stats") {
                if let Some(val) = event.get("beyondtrust_epm.event.threat.indicator.scanner_stats")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.scanner_stats".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.scanner_stats",
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
                "convert_threat_indicator_scanner_stats_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.scanner_stats");
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
            .get("beyondtrust_epm.event.threat.indicator.scanner_stats")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.scanner_stats", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.sightings") {
                if let Some(val) = event.get("beyondtrust_epm.event.threat.indicator.sightings") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.sightings".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.sightings",
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
                "convert_threat_indicator_sightings_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.sightings");
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
            .get("beyondtrust_epm.event.threat.indicator.sightings")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.sightings", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.extension")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.extension", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.fragment")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.fragment", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.full")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.full", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.original")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.original", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.password")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.password", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.path")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.path", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.url.port") {
                if let Some(val) = event.get("beyondtrust_epm.event.threat.indicator.url.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.url.port".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.threat.indicator.url.port", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_threat_indicator_url_port_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.url.port");
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
            .get("beyondtrust_epm.event.threat.indicator.url.port")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.port", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.query")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.query", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.registered_domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.registered_domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.scheme")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.scheme", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.subdomain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.subdomain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.top_level_domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.top_level_domain", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.url.username")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.url.username", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.threat.indicator.url.username") };
        if _cond {
            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("beyondtrust_epm.event.threat.indicator.url.username")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.x509.alternative_names")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.alternative_names",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.alternative_names",
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
                .get("beyondtrust_epm.event.threat.indicator.x509.issuer.common_name")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.issuer.common_name",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.issuer.common_name",
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
                .get("beyondtrust_epm.event.threat.indicator.x509.issuer.country")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.issuer.country",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.issuer.country",
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
            .get("beyondtrust_epm.event.threat.indicator.x509.issuer.distinguished_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.issuer.distinguished_name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.x509.issuer.locality")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.issuer.locality",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.issuer.locality",
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
                .get("beyondtrust_epm.event.threat.indicator.x509.issuer.organization")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.issuer.organization",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.issuer.organization",
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
                .get("beyondtrust_epm.event.threat.indicator.x509.issuer.organizational_unit")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.issuer.organizational_unit",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.issuer.organizational_unit",
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
                .get("beyondtrust_epm.event.threat.indicator.x509.issuer.state_or_province")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.issuer.state_or_province",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.issuer.state_or_province",
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
            event.has_value("beyondtrust_epm.event.threat.indicator.x509.not_after")
                && event.get_str("beyondtrust_epm.event.threat.indicator.x509.not_after")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.threat.indicator.x509.not_after")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.threat.indicator.x509.not_after",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.x509.not_after"
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
                    "date_threat_indicator_x509_not_after",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.x509.not_after");
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
            .get("beyondtrust_epm.event.threat.indicator.x509.not_after")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.not_after", v)?;
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.threat.indicator.x509.not_before")
                && event.get_str("beyondtrust_epm.event.threat.indicator.x509.not_before")
                    != Some("")
        };
        if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("beyondtrust_epm.event.threat.indicator.x509.not_before")
                {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "beyondtrust_epm.event.threat.indicator.x509.not_before",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "beyondtrust_epm.event.threat.indicator.x509.not_before"
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
                    "date_threat_indicator_x509_not_before",
                )?;
                event.remove("beyondtrust_epm.event.threat.indicator.x509.not_before");
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
            .get("beyondtrust_epm.event.threat.indicator.x509.not_before")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.not_before", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.x509.public_key_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.public_key_algorithm", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.x509.public_key_curve")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.public_key_curve", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.x509.public_key_exponent") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.x509.public_key_exponent")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.x509.public_key_exponent"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.x509.public_key_exponent",
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
                "convert_threat_indicator_x509_public_key_exponent_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.x509.public_key_exponent");
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
            .get("beyondtrust_epm.event.threat.indicator.x509.public_key_exponent")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.public_key_exponent", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.threat.indicator.x509.public_key_size") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.threat.indicator.x509.public_key_size")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.threat.indicator.x509.public_key_size"
                                .into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.threat.indicator.x509.public_key_size",
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
                "convert_threat_indicator_x509_public_key_size_to_long",
            )?;
            event.remove("beyondtrust_epm.event.threat.indicator.x509.public_key_size");
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
            .get("beyondtrust_epm.event.threat.indicator.x509.public_key_size")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.public_key_size", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.x509.serial_number")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.serial_number", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.indicator.x509.signature_algorithm")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.signature_algorithm", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.x509.subject.common_name")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.subject.common_name",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.subject.common_name",
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
                .get("beyondtrust_epm.event.threat.indicator.x509.subject.country")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.subject.country",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.subject.country",
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
            .get("beyondtrust_epm.event.threat.indicator.x509.subject.distinguished_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.subject.distinguished_name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.x509.subject.locality")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.subject.locality",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.subject.locality",
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
                .get("beyondtrust_epm.event.threat.indicator.x509.subject.organization")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.subject.organization",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.subject.organization",
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
                .get("beyondtrust_epm.event.threat.indicator.x509.subject.organizational_unit")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.subject.organizational_unit",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.subject.organizational_unit",
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
                .get("beyondtrust_epm.event.threat.indicator.x509.subject.state_or_province")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.x509.subject.state_or_province",
                |event| {
                    event.append_unique(
                        "threat.indicator.x509.subject.state_or_province",
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
            .get("beyondtrust_epm.event.threat.indicator.x509.version_number")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.indicator.x509.version_number", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.software.alias")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.software.alias",
                |event| {
                    event.append_unique(
                        "threat.software.alias",
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
            .get("beyondtrust_epm.event.threat.software.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.software.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.software.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.software.name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.software.platforms")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.software.platforms",
                |event| {
                    event.append_unique(
                        "threat.software.platforms",
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
            .get("beyondtrust_epm.event.threat.software.reference")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.software.reference", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.threat.software.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("threat.software.type", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.tactic.id")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.tactic.id", |event| {
                event.append_unique(
                    "threat.tactic.id",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.tactic.name")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.tactic.name", |event| {
                event.append_unique(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.tactic.reference")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.tactic.reference",
                |event| {
                    event.append_unique(
                        "threat.tactic.reference",
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
                .get("beyondtrust_epm.event.threat.technique.id")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.technique.id",
                |event| {
                    event.append_unique(
                        "threat.technique.id",
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
                .get("beyondtrust_epm.event.threat.technique.name")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.technique.name",
                |event| {
                    event.append_unique(
                        "threat.technique.name",
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
                .get("beyondtrust_epm.event.threat.technique.reference")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.technique.reference",
                |event| {
                    event.append_unique(
                        "threat.technique.reference",
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
                .get("beyondtrust_epm.event.threat.technique.subtechnique.id")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.technique.subtechnique.id",
                |event| {
                    event.append_unique(
                        "threat.technique.subtechnique.id",
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
                .get("beyondtrust_epm.event.threat.technique.subtechnique.name")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.technique.subtechnique.name",
                |event| {
                    event.append_unique(
                        "threat.technique.subtechnique.name",
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
                .get("beyondtrust_epm.event.threat.technique.subtechnique.reference")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.technique.subtechnique.reference",
                |event| {
                    event.append_unique(
                        "threat.technique.subtechnique.reference",
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
                .get("beyondtrust_epm.event.vulnerability.category")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.vulnerability.category",
                |event| {
                    event.append_unique(
                        "vulnerability.category",
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
            .get("beyondtrust_epm.event.vulnerability.classification")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.classification", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.vulnerability.description")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.description", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.vulnerability.enumeration")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.enumeration", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.vulnerability.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.vulnerability.reference")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.reference", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.vulnerability.report_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.report_id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.vulnerability.scanner.vendor")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.scanner.vendor", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.vulnerability.score.base") {
                if let Some(val) = event.get("beyondtrust_epm.event.vulnerability.score.base") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.vulnerability.score.base".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.vulnerability.score.base", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_vulnerability_score_base_to_long",
            )?;
            event.remove("beyondtrust_epm.event.vulnerability.score.base");
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
            .get("beyondtrust_epm.event.vulnerability.score.base")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.score.base", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.vulnerability.score.environmental") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.vulnerability.score.environmental")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.vulnerability.score.environmental".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.vulnerability.score.environmental",
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
                "convert_vulnerability_score_environmental_to_long",
            )?;
            event.remove("beyondtrust_epm.event.vulnerability.score.environmental");
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
            .get("beyondtrust_epm.event.vulnerability.score.environmental")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.score.environmental", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.vulnerability.score.temporal") {
                if let Some(val) = event.get("beyondtrust_epm.event.vulnerability.score.temporal") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.vulnerability.score.temporal".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.vulnerability.score.temporal",
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
                "convert_vulnerability_score_temporal_to_long",
            )?;
            event.remove("beyondtrust_epm.event.vulnerability.score.temporal");
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
            .get("beyondtrust_epm.event.vulnerability.score.temporal")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.score.temporal", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.vulnerability.score.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.score.version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.vulnerability.severity")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("vulnerability.severity", v)?;
        }

        let _cond = {
            event.has_value("threat")
                && (!event.has_value("event.category")
                    || !(event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("threat"))
                        }
                        serde_json::Value::String(s) => s.contains("threat"),
                        _ => false,
                    })))
        };
        if _cond {
            event.append_unique("event.category", json!("threat"))?;
        }

        let _cond = {
            event.has_value("vulnerability")
                && (!event.has_value("event.category")
                    || !(event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("vulnerability"))
                        }
                        serde_json::Value::String(s) => s.contains("vulnerability"),
                        _ => false,
                    })))
        };
        if _cond {
            event.append_unique("event.category", json!("vulnerability"))?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.enrichments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.threat.enrichments", |event| {
                event.remove("_ingest._value.matched.atomic");
                event.remove("_ingest._value.matched.field");
                event.remove("_ingest._value.matched.id");
                event.remove("_ingest._value.matched.index");
                event.remove("_ingest._value.matched.occurred");
                event.remove("_ingest._value.matched.type");
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.sections")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.sections",
                |event| {
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
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.threat.indicator.file.elf.segments")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.threat.indicator.file.elf.segments",
                |event| {
                    event.remove("_ingest._value.sections");
                    event.remove("_ingest._value.type");
                    Ok(())
                },
            )?;
        }

        event.remove("beyondtrust_epm.event.EPMWinMac.Configuration.Workstyle.Description");
        event.remove("beyondtrust_epm.event.EPMWinMac.Configuration.Workstyle.Identifier");
        event.remove("beyondtrust_epm.event.EPMWinMac.Configuration.Workstyle.Name");
        event.remove("beyondtrust_epm.event.rule.author");
        event.remove("beyondtrust_epm.event.rule.category");
        event.remove("beyondtrust_epm.event.rule.description");
        event.remove("beyondtrust_epm.event.rule.id");
        event.remove("beyondtrust_epm.event.rule.license");
        event.remove("beyondtrust_epm.event.rule.name");
        event.remove("beyondtrust_epm.event.rule.reference");
        event.remove("beyondtrust_epm.event.rule.ruleset");
        event.remove("beyondtrust_epm.event.rule.uuid");
        event.remove("beyondtrust_epm.event.rule.version");
        event.remove("beyondtrust_epm.event.threat.feed.dashboard_id");
        event.remove("beyondtrust_epm.event.threat.feed.description");
        event.remove("beyondtrust_epm.event.threat.feed.name");
        event.remove("beyondtrust_epm.event.threat.feed.reference");
        event.remove("beyondtrust_epm.event.threat.framework");
        event.remove("beyondtrust_epm.event.threat.group.alias");
        event.remove("beyondtrust_epm.event.threat.group.id");
        event.remove("beyondtrust_epm.event.threat.group.name");
        event.remove("beyondtrust_epm.event.threat.group.reference");
        event.remove("beyondtrust_epm.event.threat.indicator.as.number");
        event.remove("beyondtrust_epm.event.threat.indicator.as.organization.name");
        event.remove("beyondtrust_epm.event.threat.indicator.confidence");
        event.remove("beyondtrust_epm.event.threat.indicator.description");
        event.remove("beyondtrust_epm.event.threat.indicator.email.address");
        event.remove("beyondtrust_epm.event.threat.indicator.file.accessed");
        event.remove("beyondtrust_epm.event.threat.indicator.file.attributes");
        event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.digest_algorithm");
        event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.exists");
        event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.signing_id");
        event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.status");
        event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.subject_name");
        event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.team_id");
        event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.timestamp");
        event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.trusted");
        event.remove("beyondtrust_epm.event.threat.indicator.file.code_signature.valid");
        event.remove("beyondtrust_epm.event.threat.indicator.file.created");
        event.remove("beyondtrust_epm.event.threat.indicator.file.ctime");
        event.remove("beyondtrust_epm.event.threat.indicator.file.device");
        event.remove("beyondtrust_epm.event.threat.indicator.file.directory");
        event.remove("beyondtrust_epm.event.threat.indicator.file.drive_letter");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.architecture");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.byte_order");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.cpu_type");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.creation_date");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.header.abi_version");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.header.class");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.header.data");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.header.entrypoint");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.header.object_version");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.header.os_abi");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.header.type");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.header.version");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.shared_libraries");
        event.remove("beyondtrust_epm.event.threat.indicator.file.elf.telfhash");
        event.remove("beyondtrust_epm.event.threat.indicator.file.extension");
        event.remove("beyondtrust_epm.event.threat.indicator.file.fork_name");
        event.remove("beyondtrust_epm.event.threat.indicator.file.gid");
        event.remove("beyondtrust_epm.event.threat.indicator.file.group");
        event.remove("beyondtrust_epm.event.threat.indicator.file.hash.md5");
        event.remove("beyondtrust_epm.event.threat.indicator.file.hash.sha1");
        event.remove("beyondtrust_epm.event.threat.indicator.file.hash.sha256");
        event.remove("beyondtrust_epm.event.threat.indicator.file.hash.sha384");
        event.remove("beyondtrust_epm.event.threat.indicator.file.hash.sha512");
        event.remove("beyondtrust_epm.event.threat.indicator.file.hash.ssdeep");
        event.remove("beyondtrust_epm.event.threat.indicator.file.hash.tlsh");
        event.remove("beyondtrust_epm.event.threat.indicator.file.inode");
        event.remove("beyondtrust_epm.event.threat.indicator.file.mime_type");
        event.remove("beyondtrust_epm.event.threat.indicator.file.mode");
        event.remove("beyondtrust_epm.event.threat.indicator.file.mtime");
        event.remove("beyondtrust_epm.event.threat.indicator.file.name");
        event.remove("beyondtrust_epm.event.threat.indicator.file.owner_keyword");
        event.remove("beyondtrust_epm.event.threat.indicator.file.path");
        event.remove("beyondtrust_epm.event.threat.indicator.file.pe.architecture");
        event.remove("beyondtrust_epm.event.threat.indicator.file.pe.company");
        event.remove("beyondtrust_epm.event.threat.indicator.file.pe.description");
        event.remove("beyondtrust_epm.event.threat.indicator.file.pe.file_version");
        event.remove("beyondtrust_epm.event.threat.indicator.file.pe.imphash");
        event.remove("beyondtrust_epm.event.threat.indicator.file.pe.original_file_name");
        event.remove("beyondtrust_epm.event.threat.indicator.file.pe.pehash");
        event.remove("beyondtrust_epm.event.threat.indicator.file.pe.product");
        event.remove("beyondtrust_epm.event.threat.indicator.file.size");
        event.remove("beyondtrust_epm.event.threat.indicator.file.target_path");
        event.remove("beyondtrust_epm.event.threat.indicator.file.type");
        event.remove("beyondtrust_epm.event.threat.indicator.file.uid");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.alternative_names");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.issuer.common_name");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.issuer.country");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.issuer.distinguished_name");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.issuer.locality");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.issuer.organization");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.issuer.organizational_unit");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.issuer.state_or_province");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.not_after");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.not_before");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.public_key_algorithm");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.public_key_curve");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.public_key_exponent");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.public_key_size");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.serial_number");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.signature_algorithm");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.subject.common_name");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.subject.country");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.subject.distinguished_name");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.subject.locality");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.subject.organization");
        event
            .remove("beyondtrust_epm.event.threat.indicator.file.x509.subject.organizational_unit");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.subject.state_or_province");
        event.remove("beyondtrust_epm.event.threat.indicator.file.x509.version_number");
        event.remove("beyondtrust_epm.event.threat.indicator.first_seen");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.city_name");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.continent_code");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.continent_name");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.country_iso_code");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.country_name");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.name");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.postal_code");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.region_iso_code");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.region_name");
        event.remove("beyondtrust_epm.event.threat.indicator.geo.timezone");
        event.remove("beyondtrust_epm.event.threat.indicator.ip");
        event.remove("beyondtrust_epm.event.threat.indicator.last_seen");
        event.remove("beyondtrust_epm.event.threat.indicator.marking.tlp");
        event.remove("beyondtrust_epm.event.threat.indicator.modified_at");
        event.remove("beyondtrust_epm.event.threat.indicator.port");
        event.remove("beyondtrust_epm.event.threat.indicator.provider");
        event.remove("beyondtrust_epm.event.threat.indicator.reference");
        event.remove("beyondtrust_epm.event.threat.indicator.registry.data.bytes");
        event.remove("beyondtrust_epm.event.threat.indicator.registry.data.strings");
        event.remove("beyondtrust_epm.event.threat.indicator.registry.data.type");
        event.remove("beyondtrust_epm.event.threat.indicator.registry.hive");
        event.remove("beyondtrust_epm.event.threat.indicator.registry.key");
        event.remove("beyondtrust_epm.event.threat.indicator.registry.path");
        event.remove("beyondtrust_epm.event.threat.indicator.registry.value");
        event.remove("beyondtrust_epm.event.threat.indicator.scanner_stats");
        event.remove("beyondtrust_epm.event.threat.indicator.sightings");
        event.remove("beyondtrust_epm.event.threat.indicator.type");
        event.remove("beyondtrust_epm.event.threat.indicator.url.domain");
        event.remove("beyondtrust_epm.event.threat.indicator.url.extension");
        event.remove("beyondtrust_epm.event.threat.indicator.url.fragment");
        event.remove("beyondtrust_epm.event.threat.indicator.url.full");
        event.remove("beyondtrust_epm.event.threat.indicator.url.original");
        event.remove("beyondtrust_epm.event.threat.indicator.url.password");
        event.remove("beyondtrust_epm.event.threat.indicator.url.path");
        event.remove("beyondtrust_epm.event.threat.indicator.url.port");
        event.remove("beyondtrust_epm.event.threat.indicator.url.query");
        event.remove("beyondtrust_epm.event.threat.indicator.url.registered_domain");
        event.remove("beyondtrust_epm.event.threat.indicator.url.scheme");
        event.remove("beyondtrust_epm.event.threat.indicator.url.subdomain");
        event.remove("beyondtrust_epm.event.threat.indicator.url.top_level_domain");
        event.remove("beyondtrust_epm.event.threat.indicator.url.username");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.alternative_names");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.issuer.common_name");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.issuer.country");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.issuer.distinguished_name");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.issuer.locality");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.issuer.organization");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.issuer.organizational_unit");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.issuer.state_or_province");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.not_after");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.not_before");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.public_key_algorithm");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.public_key_curve");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.public_key_exponent");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.public_key_size");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.serial_number");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.signature_algorithm");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.subject.common_name");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.subject.country");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.subject.distinguished_name");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.subject.locality");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.subject.organization");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.subject.organizational_unit");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.subject.state_or_province");
        event.remove("beyondtrust_epm.event.threat.indicator.x509.version_number");
        event.remove("beyondtrust_epm.event.threat.software.alias");
        event.remove("beyondtrust_epm.event.threat.software.id");
        event.remove("beyondtrust_epm.event.threat.software.name");
        event.remove("beyondtrust_epm.event.threat.software.platforms");
        event.remove("beyondtrust_epm.event.threat.software.reference");
        event.remove("beyondtrust_epm.event.threat.software.type");
        event.remove("beyondtrust_epm.event.threat.tactic.id");
        event.remove("beyondtrust_epm.event.threat.tactic.name");
        event.remove("beyondtrust_epm.event.threat.tactic.reference");
        event.remove("beyondtrust_epm.event.threat.technique.id");
        event.remove("beyondtrust_epm.event.threat.technique.name");
        event.remove("beyondtrust_epm.event.threat.technique.reference");
        event.remove("beyondtrust_epm.event.threat.technique.subtechnique.id");
        event.remove("beyondtrust_epm.event.threat.technique.subtechnique.name");
        event.remove("beyondtrust_epm.event.threat.technique.subtechnique.reference");
        event.remove("beyondtrust_epm.event.vulnerability.category");
        event.remove("beyondtrust_epm.event.vulnerability.classification");
        event.remove("beyondtrust_epm.event.vulnerability.description");
        event.remove("beyondtrust_epm.event.vulnerability.enumeration");
        event.remove("beyondtrust_epm.event.vulnerability.id");
        event.remove("beyondtrust_epm.event.vulnerability.reference");
        event.remove("beyondtrust_epm.event.vulnerability.report_id");
        event.remove("beyondtrust_epm.event.vulnerability.scanner.vendor");
        event.remove("beyondtrust_epm.event.vulnerability.score.base");
        event.remove("beyondtrust_epm.event.vulnerability.score.environmental");
        event.remove("beyondtrust_epm.event.vulnerability.score.temporal");
        event.remove("beyondtrust_epm.event.vulnerability.score.version");
        event.remove("beyondtrust_epm.event.vulnerability.severity");

        Ok(TransformResult::Continue)
    }
}
