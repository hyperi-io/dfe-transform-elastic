// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_vulnerabilities` pipeline.
pub struct PipelineObjectVulnerabilities;

impl Transform for PipelineObjectVulnerabilities {
    fn name(&self) -> &str {
        "pipeline_object_vulnerabilities"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = {
                event
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.advisory.created_time_dt")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "ISO8601",
                                            "UNIX_MS",
                                            "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.advisory.created_time_dt",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.advisory.created_time_dt"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_advisory_created_time_dt",
                                )?;
                                event.remove("_ingest._value.advisory.created_time_dt");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.advisory.created_time")
                                {
                                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                        Some(parsed) => event
                                            .set("_ingest._value.advisory.created_time", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.advisory.created_time".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_advisory_created_time",
                                )?;
                                event.remove("_ingest._value.advisory.created_time");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.vulnerabilities", |event| {
                    if event.has_value("_ingest._value.advisory.install_state_id") {
                        if let Some(val) = event.get("_ingest._value.advisory.install_state_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.advisory.install_state_id".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.advisory.install_state_id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.advisory.is_superseded") {
                                    if let Some(val) =
                                        event.get("_ingest._value.advisory.is_superseded")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.advisory.is_superseded"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.advisory.is_superseded",
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
                                    "convert_vulnerabilities_advisory_is_superseded_to_boolean",
                                )?;
                                if event
                                    .remove("_ingest._value.advisory.is_superseded")
                                    .is_none()
                                {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.advisory.is_superseded".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.advisory.modified_time_dt")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "ISO8601",
                                            "UNIX_MS",
                                            "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.advisory.modified_time_dt",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.advisory.modified_time_dt"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_advisory_modified_time_dt",
                                )?;
                                event.remove("_ingest._value.advisory.modified_time_dt");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.advisory.modified_time")
                                {
                                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                        Some(parsed) => event
                                            .set("_ingest._value.advisory.modified_time", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.advisory.modified_time"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_advisory_modified_time",
                                )?;
                                event.remove("_ingest._value.advisory.modified_time");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.advisory.size") {
                                    if let Some(val) = event.get("_ingest._value.advisory.size") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.advisory.size".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.advisory.size", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_vulnerabilities_advisory_size_to_long",
                                )?;
                                if event.remove("_ingest._value.advisory.size").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.advisory.size".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            if event.has_value("_ingest._value.affected_code") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject =
                                        event.get("_ingest._value.affected_code").cloned();
                                    let keyed = matches!(subject, Some(Value::Object(_)));
                                    let entries: Vec<(Option<String>, Value)> = match subject {
                                        Some(Value::Array(items)) => {
                                            items.into_iter().map(|v| (None, v)).collect()
                                        }
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 2 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.end_column") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.end_column")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path:
                                                                        "_ingest._value.end_column"
                                                                            .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.end_column",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_affected_code_end_column_to_long")?;
                                                if event
                                                    .remove("_ingest._value.end_column")
                                                    .is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.end_column".into(),
                                                    });
                                                }
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.affected_code",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            if event.has_value("_ingest._value.affected_code") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject =
                                        event.get("_ingest._value.affected_code").cloned();
                                    let keyed = matches!(subject, Some(Value::Object(_)));
                                    let entries: Vec<(Option<String>, Value)> = match subject {
                                        Some(Value::Array(items)) => {
                                            items.into_iter().map(|v| (None, v)).collect()
                                        }
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 2 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.end_line") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.end_line")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.end_line"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.end_line",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_affected_code_end_line_to_long")?;
                                                if event.remove("_ingest._value.end_line").is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.end_line".into(),
                                                    });
                                                }
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.affected_code",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            if event.has_value("_ingest._value.affected_code") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject =
                                        event.get("_ingest._value.affected_code").cloned();
                                    let keyed = matches!(subject, Some(Value::Object(_)));
                                    let entries: Vec<(Option<String>, Value)> = match subject {
                                        Some(Value::Array(items)) => {
                                            items.into_iter().map(|v| (None, v)).collect()
                                        }
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 2 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.start_column") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.start_column")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                            path: "_ingest._value.start_column".into(),
                            message,
                            }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.start_column",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_affected_code_start_column_to_long")?;
                                                if event
                                                    .remove("_ingest._value.start_column")
                                                    .is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.start_column".into(),
                                                    });
                                                }
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.affected_code",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            if event.has_value("_ingest._value.affected_code") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject =
                                        event.get("_ingest._value.affected_code").cloned();
                                    let keyed = matches!(subject, Some(Value::Object(_)));
                                    let entries: Vec<(Option<String>, Value)> = match subject {
                                        Some(Value::Array(items)) => {
                                            items.into_iter().map(|v| (None, v)).collect()
                                        }
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 2 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.start_line") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.start_line")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path:
                                                                        "_ingest._value.start_line"
                                                                            .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.start_line",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_affected_code_start_line_to_long")?;
                                                if event
                                                    .remove("_ingest._value.start_line")
                                                    .is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.start_line".into(),
                                                    });
                                                }
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.affected_code",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            if event.has_value("_ingest._value.affected_packages") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject =
                                        event.get("_ingest._value.affected_packages").cloned();
                                    let keyed = matches!(subject, Some(Value::Object(_)));
                                    let entries: Vec<(Option<String>, Value)> = match subject {
                                        Some(Value::Array(items)) => {
                                            items.into_iter().map(|v| (None, v)).collect()
                                        }
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 2 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.epoch") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.epoch")
                                                    {
                                                        let converted = convert_value(val, "long")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path: "_ingest._value.epoch"
                                                                        .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.epoch",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_affected_packages_epoch_to_long")?;
                                                if event.remove("_ingest._value.epoch").is_none() {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.epoch".into(),
                                                    });
                                                }
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.affected_packages",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.vulnerabilities", |event| {
                    if event.has_value("_ingest._value.affected_packages") {
                        foreach_array(event, "_ingest._value.affected_packages", |event| {
                            if event.has_value("_ingest._value.type_id") {
                                if let Some(val) = event.get("_ingest._value.type_id") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.type_id".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.type_id", converted)?;
                                }
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.cve.created_time_dt")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "ISO8601",
                                            "UNIX_MS",
                                            "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event
                                            .set("_ingest._value.cve.created_time_dt", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.cve.created_time_dt".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_cve_created_time_dt",
                                )?;
                                event.remove("_ingest._value.cve.created_time_dt");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.cve.created_time")
                                {
                                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.cve.created_time", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.cve.created_time".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_cve_created_time",
                                )?;
                                event.remove("_ingest._value.cve.created_time");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            if event.has_value("_ingest._value.cve.cvss") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.cve.cvss").cloned();
                                    let keyed = matches!(subject, Some(Value::Object(_)));
                                    let entries: Vec<(Option<String>, Value)> = match subject {
                                        Some(Value::Array(items)) => {
                                            items.into_iter().map(|v| (None, v)).collect()
                                        }
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 2 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.base_score") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.base_score")
                                                    {
                                                        let converted = convert_value(val, "float")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                                                                    path:
                                                                        "_ingest._value.base_score"
                                                                            .into(),
                                                                    message,
                                                                }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.base_score",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_cve_cvss_base_score_to_float")?;
                                                if event
                                                    .remove("_ingest._value.base_score")
                                                    .is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.base_score".into(),
                                                    });
                                                }
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.cve.cvss",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            if event.has_value("_ingest._value.cve.cvss") {
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.cve.cvss").cloned();
                                    let keyed = matches!(subject, Some(Value::Object(_)));
                                    let entries: Vec<(Option<String>, Value)> = match subject {
                                        Some(Value::Array(items)) => {
                                            items.into_iter().map(|v| (None, v)).collect()
                                        }
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // on_failure: 2 handler(s)
                                            if let Err(err) = (|| -> Result<()> {
                                                if event.has_value("_ingest._value.overall_score") {
                                                    if let Some(val) =
                                                        event.get("_ingest._value.overall_score")
                                                    {
                                                        let converted = convert_value(val, "float")
                                                            .map_err(|message| {
                                                                TransformError::ParseError {
                            path: "_ingest._value.overall_score".into(),
                            message,
                            }
                                                            })?;
                                                        event.set(
                                                            "_ingest._value.overall_score",
                                                            converted,
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            ) {
                                                event.set(
                                                    "_ingest.on_failure_message",
                                                    err.to_string(),
                                                )?;
                                                event.set(
                                                    "_ingest.on_failure_processor_type",
                                                    "convert",
                                                )?;
                                                event.set("_ingest.on_failure_processor_tag", "convert_vulnerabilities_cve_cvss_overall_score_to_float")?;
                                                if event
                                                    .remove("_ingest._value.overall_score")
                                                    .is_none()
                                                {
                                                    return Err(TransformError::FieldNotFound {
                                                        path: "_ingest._value.overall_score".into(),
                                                    });
                                                }
                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                event.remove("_ingest.on_failure_message");
                                                event.remove("_ingest.on_failure_processor_type");
                                                event.remove("_ingest.on_failure_processor_tag");
                                                if event
                                                    .get_object("_ingest")
                                                    .is_some_and(|m| m.is_empty())
                                                {
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
                                            "_ingest._value.cve.cvss",
                                            if keyed {
                                                Value::Object(fields)
                                            } else {
                                                Value::Array(list)
                                            },
                                        )?;
                                    }
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.cve.epss.created_time_dt")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "ISO8601",
                                            "UNIX_MS",
                                            "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.cve.epss.created_time_dt",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.cve.epss.created_time_dt"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_cve_epss_created_time_dt",
                                )?;
                                event.remove("_ingest._value.cve.epss.created_time_dt");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.cve.epss.created_time")
                                {
                                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                        Some(parsed) => event
                                            .set("_ingest._value.cve.epss.created_time", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.cve.epss.created_time".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_cve_epss_created_time",
                                )?;
                                event.remove("_ingest._value.cve.epss.created_time");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.cve.epss.percentile") {
                                    if let Some(val) =
                                        event.get("_ingest._value.cve.epss.percentile")
                                    {
                                        let converted =
                                            convert_value(val, "float").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.cve.epss.percentile"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event
                                            .set("_ingest._value.cve.epss.percentile", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_vulnerabilities_cve_epss_percentile_to_float",
                                )?;
                                if event.remove("_ingest._value.cve.epss.percentile").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.cve.epss.percentile".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.cve.modified_time_dt")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "ISO8601",
                                            "UNIX_MS",
                                            "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event
                                            .set("_ingest._value.cve.modified_time_dt", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.cve.modified_time_dt".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_cve_modified_time_dt",
                                )?;
                                event.remove("_ingest._value.cve.modified_time_dt");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.cve.modified_time")
                                {
                                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.cve.modified_time", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.cve.modified_time".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_cve_modified_time",
                                )?;
                                event.remove("_ingest._value.cve.modified_time");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.exploit_last_seen_time_dt")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "ISO8601",
                                            "UNIX_MS",
                                            "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set(
                                            "_ingest._value.exploit_last_seen_time_dt",
                                            parsed,
                                        )?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.exploit_last_seen_time_dt"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_exploit_last_seen_time_dt",
                                )?;
                                event.remove("_ingest._value.exploit_last_seen_time_dt");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.exploit_last_seen_time")
                                {
                                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                        Some(parsed) => event
                                            .set("_ingest._value.exploit_last_seen_time", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.exploit_last_seen_time"
                                                    .into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_exploit_last_seen_time",
                                )?;
                                event.remove("_ingest._value.exploit_last_seen_time");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.first_seen_time_dt")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "ISO8601",
                                            "UNIX_MS",
                                            "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event
                                            .set("_ingest._value.first_seen_time_dt", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.first_seen_time_dt".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_first_seen_time_dt",
                                )?;
                                event.remove("_ingest._value.first_seen_time_dt");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.first_seen_time")
                                {
                                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.first_seen_time", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.first_seen_time".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_first_seen_time",
                                )?;
                                event.remove("_ingest._value.first_seen_time");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.vulnerabilities", |event| {
                    if event.has_value("_ingest._value.fix_coverage_id") {
                        if let Some(val) = event.get("_ingest._value.fix_coverage_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.fix_coverage_id".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.fix_coverage_id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.is_exploit_available") {
                                    if let Some(val) =
                                        event.get("_ingest._value.is_exploit_available")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.is_exploit_available"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.is_exploit_available",
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
                                    "convert_vulnerabilities_is_exploit_available_to_boolean",
                                )?;
                                if event
                                    .remove("_ingest._value.is_exploit_available")
                                    .is_none()
                                {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.is_exploit_available".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.is_fix_available") {
                                    if let Some(val) = event.get("_ingest._value.is_fix_available")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.is_fix_available".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.is_fix_available", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_vulnerabilities_is_fix_available_to_boolean",
                                )?;
                                if event.remove("_ingest._value.is_fix_available").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value.is_fix_available".into(),
                                    });
                                }
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.last_seen_time_dt")
                                {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "ISO8601",
                                            "UNIX_MS",
                                            "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_seen_time_dt", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_seen_time_dt".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_last_seen_time_dt",
                                )?;
                                event.remove("_ingest._value.last_seen_time_dt");
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
                            "aws_securityhub.finding.vulnerabilities",
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
                    .get("aws_securityhub.finding.vulnerabilities")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("aws_securityhub.finding.vulnerabilities")
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
                                    event.get_as_string("_ingest._value.last_seen_time")
                                {
                                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_seen_time", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.last_seen_time".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date_vulnerabilities_last_seen_time",
                                )?;
                                event.remove("_ingest._value.last_seen_time");
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
                            "aws_securityhub.finding.vulnerabilities",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
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
