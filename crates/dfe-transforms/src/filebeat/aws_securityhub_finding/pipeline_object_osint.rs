// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_osint` pipeline.
pub struct PipelineObjectOsint;

impl Transform for PipelineObjectOsint {
    fn name(&self) -> &str {
        "pipeline_object_osint"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.answers") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.answers").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.ttl") {
                        if let Some(val) = event.get("_ingest._value.ttl") {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.ttl".into(),
                        message,
                        })?;
                        event.set("_ingest._value.ttl", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_answers_ttl_to_long")?;
                        if event.remove("_ingest._value.ttl").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.ttl".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.answers", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.answers") {
                    foreach_array(event, "_ingest._value.answers", |event| {
                    if event.has_value("_ingest._value.flag_ids") {
                    if let Some(val) = event.get("_ingest._value.flag_ids") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.flag_ids".into(),
                    message,
                    })?;
                    event.set("_ingest._value.flag_ids", converted)?;
                    }
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.answers") {
                    foreach_array(event, "_ingest._value.answers", |event| {
                    if event.has_value("_ingest._value.packet_uid") {
                    if let Some(val) = event.get("_ingest._value.packet_uid") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.packet_uid".into(),
                    message,
                    })?;
                    event.set("_ingest._value.packet_uid", converted)?;
                    }
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.autonomous_system.number") {
                        if let Some(val) = event.get("_ingest._value.autonomous_system.number") {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.autonomous_system.number".into(),
                        message,
                        })?;
                        event.set("_ingest._value.autonomous_system.number", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_autonomous_system_number_to_long")?;
                        if event.remove("_ingest._value.autonomous_system.number").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.autonomous_system.number".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.confidence_id") {
                    if let Some(val) = event.get("_ingest._value.confidence_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.confidence_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.confidence_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.created_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.created_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.created_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_created_time_dt")?;
                        event.remove("_ingest._value.created_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.created_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.created_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.created_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_created_time")?;
                        event.remove("_ingest._value.created_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.creator.has_mfa") {
                        if let Some(val) = event.get("_ingest._value.creator.has_mfa") {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.creator.has_mfa".into(),
                        message,
                        })?;
                        event.set("_ingest._value.creator.has_mfa", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_creator_has_mfa_to_boolean")?;
                        if event.remove("_ingest._value.creator.has_mfa").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.creator.has_mfa".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.creator.risk_level_id") {
                    if let Some(val) = event.get("_ingest._value.creator.risk_level_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.creator.risk_level_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.creator.risk_level_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.creator.risk_score") {
                        if let Some(val) = event.get("_ingest._value.creator.risk_score") {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.creator.risk_score".into(),
                        message,
                        })?;
                        event.set("_ingest._value.creator.risk_score", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_creator_risk_score_to_long")?;
                        if event.remove("_ingest._value.creator.risk_score").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.creator.risk_score".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.creator.type_id") {
                    if let Some(val) = event.get("_ingest._value.creator.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.creator.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.creator.type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.detection_pattern_type_id") {
                    if let Some(val) = event.get("_ingest._value.detection_pattern_type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.detection_pattern_type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.detection_pattern_type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.email.is_read") {
                        if let Some(val) = event.get("_ingest._value.email.is_read") {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.email.is_read".into(),
                        message,
                        })?;
                        event.set("_ingest._value.email.is_read", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_email_is_read_to_boolean")?;
                        if event.remove("_ingest._value.email.is_read").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.email.is_read".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.email.size") {
                        if let Some(val) = event.get("_ingest._value.email.size") {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.email.size".into(),
                        message,
                        })?;
                        event.set("_ingest._value.email.size", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_email_size_to_long")?;
                        if event.remove("_ingest._value.email.size").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.email.size".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.x_originating_ip") {
                    foreach_array(event, "_ingest._value.x_originating_ip", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_osint_x_originating_ip_to_ip")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.expiration_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.expiration_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.expiration_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_expiration_time_dt")?;
                        event.remove("_ingest._value.expiration_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.expiration_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.expiration_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.expiration_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_expiration_time")?;
                        event.remove("_ingest._value.expiration_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.file.accessed_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.file.accessed_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.file.accessed_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_file_accessed_time_dt")?;
                        event.remove("_ingest._value.file.accessed_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.file.accessed_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.file.accessed_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.file.accessed_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_file_accessed_time")?;
                        event.remove("_ingest._value.file.accessed_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.file.attributes") {
                        if let Some(val) = event.get("_ingest._value.file.attributes") {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.file.attributes".into(),
                        message,
                        })?;
                        event.set("_ingest._value.file.attributes", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_file_attributes_to_long")?;
                        if event.remove("_ingest._value.file.attributes").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.file.attributes".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.file.confidentiality_id") {
                    if let Some(val) = event.get("_ingest._value.file.confidentiality_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.file.confidentiality_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.file.confidentiality_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.file.created_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.file.created_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.file.created_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_file_created_time_dt")?;
                        event.remove("_ingest._value.file.created_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.file.created_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.file.created_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.file.created_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_file_created_time")?;
                        event.remove("_ingest._value.file.created_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.file.drive_type_id") {
                    if let Some(val) = event.get("_ingest._value.file.drive_type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.file.drive_type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.file.drive_type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.file.is_deleted") {
                        if let Some(val) = event.get("_ingest._value.file.is_deleted") {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.file.is_deleted".into(),
                        message,
                        })?;
                        event.set("_ingest._value.file.is_deleted", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_file_is_deleted_to_boolean")?;
                        if event.remove("_ingest._value.file.is_deleted").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.file.is_deleted".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.file.is_encrypted") {
                        if let Some(val) = event.get("_ingest._value.file.is_encrypted") {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.file.is_encrypted".into(),
                        message,
                        })?;
                        event.set("_ingest._value.file.is_encrypted", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_file_is_encrypted_to_boolean")?;
                        if event.remove("_ingest._value.file.is_encrypted").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.file.is_encrypted".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.file.is_system") {
                        if let Some(val) = event.get("_ingest._value.file.is_system") {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.file.is_system".into(),
                        message,
                        })?;
                        event.set("_ingest._value.file.is_system", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_file_is_system_to_boolean")?;
                        if event.remove("_ingest._value.file.is_system").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.file.is_system".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.file.modified_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.file.modified_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.file.modified_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_file_modified_time_dt")?;
                        event.remove("_ingest._value.file.modified_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.file.modified_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.file.modified_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.file.modified_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_file_modified_time")?;
                        event.remove("_ingest._value.file.modified_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.file.type_id") {
                    if let Some(val) = event.get("_ingest._value.file.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.file.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.file.type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.file.size") {
                        if let Some(val) = event.get("_ingest._value.file.size") {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.file.size".into(),
                        message,
                        })?;
                        event.set("_ingest._value.file.size", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_file_size_to_long")?;
                        if event.remove("_ingest._value.file.size").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.file.size".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.kill_chain") {
                    foreach_array(event, "_ingest._value.kill_chain", |event| {
                    if event.has_value("_ingest._value.phase_id") {
                    if let Some(val) = event.get("_ingest._value.phase_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.phase_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.phase_id", converted)?;
                    }
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.location.is_on_premises") {
                        if let Some(val) = event.get("_ingest._value.location.is_on_premises") {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.location.is_on_premises".into(),
                        message,
                        })?;
                        event.set("_ingest._value.location.is_on_premises", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_location_is_on_premises_to_boolean")?;
                        if event.remove("_ingest._value.location.is_on_premises").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.location.is_on_premises".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.location.lat") {
                        if let Some(val) = event.get("_ingest._value.location.lat") {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.location.lat".into(),
                        message,
                        })?;
                        event.set("_ingest._value.location.lat", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_location_lat_to_double")?;
                        if event.remove("_ingest._value.location.lat").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.location.lat".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.location.long") {
                        if let Some(val) = event.get("_ingest._value.location.long") {
                        let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.location.long".into(),
                        message,
                        })?;
                        event.set("_ingest._value.location.long", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_location_long_to_double")?;
                        if event.remove("_ingest._value.location.long").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.location.long".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.malware") {
                    foreach_array(event, "_ingest._value.malware", |event| {
                    if event.has_value("_ingest._value.classification_ids") {
                    if let Some(val) = event.get("_ingest._value.classification_ids") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.classification_ids".into(),
                    message,
                    })?;
                    event.set("_ingest._value.classification_ids", converted)?;
                    }
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.malware") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.malware").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.num_infected") {
                        if let Some(val) = event.get("_ingest._value.num_infected") {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.num_infected".into(),
                        message,
                        })?;
                        event.set("_ingest._value.num_infected", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_malware_num_infected_to_long")?;
                        if event.remove("_ingest._value.num_infected").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.num_infected".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.malware", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.malware") {
                    foreach_array(event, "_ingest._value.malware", |event| {
                    if event.has_value("_ingest._value.severity_id") {
                    if let Some(val) = event.get("_ingest._value.severity_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.severity_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.severity_id", converted)?;
                    }
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.modified_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.modified_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.modified_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_modified_time_dt")?;
                        event.remove("_ingest._value.modified_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.modified_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.modified_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.modified_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_modified_time")?;
                        event.remove("_ingest._value.modified_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.related_analytics") {
                    foreach_array(event, "_ingest._value.related_analytics", |event| {
                    if event.has_value("_ingest._value.type_id") {
                    if let Some(val) = event.get("_ingest._value.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.type_id".into(),
                    message,
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

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.reputation.base_score") {
                        if let Some(val) = event.get("_ingest._value.reputation.base_score") {
                        let converted = convert_value(val, "float")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.reputation.base_score".into(),
                        message,
                        })?;
                        event.set("_ingest._value.reputation.base_score", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_reputation_base_score_to_float")?;
                        if event.remove("_ingest._value.reputation.base_score").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.reputation.base_score".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.reputation.score_id") {
                    if let Some(val) = event.get("_ingest._value.reputation.score_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.reputation.score_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.reputation.score_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.risk_score") {
                        if let Some(val) = event.get("_ingest._value.risk_score") {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.risk_score".into(),
                        message,
                        })?;
                        event.set("_ingest._value.risk_score", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_risk_score_to_long")?;
                        if event.remove("_ingest._value.risk_score").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.risk_score".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.script.type_id") {
                    if let Some(val) = event.get("_ingest._value.script.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.script.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.script.type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.severity_id") {
                    if let Some(val) = event.get("_ingest._value.severity_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.severity_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.severity_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.signatures") {
                    foreach_array(event, "_ingest._value.signatures", |event| {
                    if event.has_value("_ingest._value.algorithm_id") {
                    if let Some(val) = event.get("_ingest._value.algorithm_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.algorithm_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.algorithm_id", converted)?;
                    }
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.signatures") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.signatures").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.created_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.created_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.created_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_signatures_created_time_dt")?;
                        event.remove("_ingest._value.created_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.signatures", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.signatures") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.signatures").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.created_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.created_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.created_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_signatures_created_time")?;
                        event.remove("_ingest._value.created_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.signatures", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.signatures") {
                    foreach_array(event, "_ingest._value.signatures", |event| {
                    if event.has_value("_ingest._value.state_id") {
                    if let Some(val) = event.get("_ingest._value.state_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.state_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.state_id", converted)?;
                    }
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.threat_actor.type_id") {
                    if let Some(val) = event.get("_ingest._value.threat_actor.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.threat_actor.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.threat_actor.type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.type_id") {
                    if let Some(val) = event.get("_ingest._value.type_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.type_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.type_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.uploaded_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.uploaded_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.uploaded_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_uploaded_time_dt")?;
                        event.remove("_ingest._value.uploaded_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.uploaded_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.uploaded_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.uploaded_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_uploaded_time")?;
                        event.remove("_ingest._value.uploaded_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.vulnerabilities") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.vulnerabilities").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.exploit_last_seen_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.exploit_last_seen_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.exploit_last_seen_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_vulnerabilities_exploit_last_seen_time_dt")?;
                        event.remove("_ingest._value.exploit_last_seen_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.vulnerabilities", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.vulnerabilities") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.vulnerabilities").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.exploit_last_seen_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.exploit_last_seen_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.exploit_last_seen_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_vulnerabilities_exploit_last_seen_time")?;
                        event.remove("_ingest._value.exploit_last_seen_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.vulnerabilities", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.vulnerabilities") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.vulnerabilities").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.first_seen_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.first_seen_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.first_seen_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_vulnerabilities_first_seen_time_dt")?;
                        event.remove("_ingest._value.first_seen_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.vulnerabilities", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.vulnerabilities") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.vulnerabilities").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.first_seen_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.first_seen_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.first_seen_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_vulnerabilities_first_seen_time")?;
                        event.remove("_ingest._value.first_seen_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.vulnerabilities", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.vulnerabilities") {
                    foreach_array(event, "_ingest._value.vulnerabilities", |event| {
                    if event.has_value("_ingest._value.fix_coverage_id") {
                    if let Some(val) = event.get("_ingest._value.fix_coverage_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.fix_coverage_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.fix_coverage_id", converted)?;
                    }
                    }
                    Ok(())
                    })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.vulnerabilities") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.vulnerabilities").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_exploit_available") {
                        if let Some(val) = event.get("_ingest._value.is_exploit_available") {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.is_exploit_available".into(),
                        message,
                        })?;
                        event.set("_ingest._value.is_exploit_available", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_vulnerabilities_is_exploit_available_to_boolean")?;
                        if event.remove("_ingest._value.is_exploit_available").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.is_exploit_available".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.vulnerabilities", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.vulnerabilities") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.vulnerabilities").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.is_fix_available") {
                        if let Some(val) = event.get("_ingest._value.is_fix_available") {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                        path: "_ingest._value.is_fix_available".into(),
                        message,
                        })?;
                        event.set("_ingest._value.is_fix_available", converted)?;
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_osint_vulnerabilities_is_fix_available_to_boolean")?;
                        if event.remove("_ingest._value.is_fix_available").is_none() {
                        return Err(TransformError::FieldNotFound { path: "_ingest._value.is_fix_available".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.vulnerabilities", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.vulnerabilities") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.vulnerabilities").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.last_seen_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.last_seen_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.last_seen_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_vulnerabilities_last_seen_time_dt")?;
                        event.remove("_ingest._value.last_seen_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.vulnerabilities", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        if event.has_value("_ingest._value.vulnerabilities") {
                        if let Some(Value::Array(items)) = event.get("_ingest._value.vulnerabilities").cloned() {
                        // A NESTED loop borrows the same `_ingest._value` slot, so
                        // the enclosing element is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.last_seen_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.last_seen_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.last_seen_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_vulnerabilities_last_seen_time")?;
                        event.remove("_ingest._value.last_seen_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                        }
                        event.set("_ingest._value.vulnerabilities", Value::Array(out))?;
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.whois.created_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.whois.created_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.whois.created_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_whois_created_time_dt")?;
                        event.remove("_ingest._value.whois.created_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.whois.created_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.whois.created_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.whois.created_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_whois_created_time")?;
                        event.remove("_ingest._value.whois.created_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "aws_securityhub.finding.osint", |event| {
                    if event.has_value("_ingest._value.whois.dnssec_status_id") {
                    if let Some(val) = event.get("_ingest._value.whois.dnssec_status_id") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.whois.dnssec_status_id".into(),
                    message,
                    })?;
                    event.set("_ingest._value.whois.dnssec_status_id", converted)?;
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.whois.last_seen_time_dt") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                        Some(parsed) => event.set("_ingest._value.whois.last_seen_time_dt", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.whois.last_seen_time_dt".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_whois_last_seen_time_dt")?;
                        event.remove("_ingest._value.whois.last_seen_time_dt");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            let _cond = { event.get("aws_securityhub.finding.osint").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("aws_securityhub.finding.osint").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 1 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.whois.last_seen_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.whois.last_seen_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.whois.last_seen_time".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_osint_whois_last_seen_time")?;
                        event.remove("_ingest._value.whois.last_seen_time");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                        }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    event.set("aws_securityhub.finding.osint", Value::Array(out))?;
                }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}'\n{}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}'\n", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
