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

            event.append("event.category", json!("threat"))?;

            event.append("event.type", json!("indicator"))?;

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

            let _cond = {
                event.has_value("event.original")
                    && event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
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
                            .get("_ingest.pipeline")
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
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastUpdated") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.alertAdditionalInfo") {
                event.rename(
                    "json.alertAdditionalInfo",
                    "prisma_cloud.alert.additional_info",
                )?;
            }

            if event.has_value("prisma_cloud.alert.additional_info.scannerVersion") {
                event.rename(
                    "prisma_cloud.alert.additional_info.scannerVersion",
                    "prisma_cloud.alert.additional_info.scanner_version",
                )?;
            }

            if event.has_value("json.alertAttribution") {
                event.rename("json.alertAttribution", "prisma_cloud.alert.attribution")?;
            }

            if event.has_value("prisma_cloud.alert.attribution.attributionEventList") {
                event.rename(
                    "prisma_cloud.alert.attribution.attributionEventList",
                    "prisma_cloud.alert.attribution.event_list",
                )?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.attribution.event_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.attribution.event_list",
                        |event| {
                            if event.has_value("_ingest._value.event") {
                                event.rename("_ingest._value.event", "_ingest._value.value")?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.attribution.event_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("prisma_cloud.alert.attribution.event_list")
                            .cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.event_ts")
                                    {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.ts", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.event_ts".into(),
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
                                    event.remove("_ingest._value.event_ts");
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
                                "prisma_cloud.alert.attribution.event_list",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.attribution.event_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.attribution.event_list",
                        |event| {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.username")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.attribution.event_list")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.attribution.event_list",
                        |event| {
                            event.remove("_ingest._value.event_ts");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("prisma_cloud.alert.attribution.resourceCreatedBy") {
                event.rename(
                    "prisma_cloud.alert.attribution.resourceCreatedBy",
                    "prisma_cloud.alert.attribution.resource.created_by",
                )?;
            }

            let _cond = {
                event.has_value("prisma_cloud.alert.attribution.resourceCreatedOn")
                    && event.get_str("prisma_cloud.alert.attribution.resourceCreatedOn") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("prisma_cloud.alert.attribution.resourceCreatedOn")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set(
                                "prisma_cloud.alert.attribution.resource.created_on",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "prisma_cloud.alert.attribution.resourceCreatedOn".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_resourceCreatedOn")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.alertCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.alertCount") {
                        if let Some(val) = event.get("json.alertCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.alertCount".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.alert.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_alertCount")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("json.alertTime") && event.get_str("json.alertTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.alertTime") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("prisma_cloud.alert.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.alertTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_alertTime")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.dismissalDuration") {
                event.rename(
                    "json.dismissalDuration",
                    "prisma_cloud.alert.dismissal.duration",
                )?;
            }

            if event.has_value("json.dismissalNote") {
                event.rename("json.dismissalNote", "prisma_cloud.alert.dismissal.note")?;
            }

            let _cond = {
                event.has_value("json.dismissalUntilTs")
                    && event.get_str("json.dismissalUntilTs") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.dismissalUntilTs") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("prisma_cloud.alert.dismissal.until_ts", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.dismissalUntilTs".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_dismissalUnitlTs")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.dismissedBy") {
                event.rename("json.dismissedBy", "prisma_cloud.alert.dismissed_by")?;
            }

            let _cond = {
                event.has_value("json.eventOccurred")
                    && event.get_str("json.eventOccurred") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.eventOccurred") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("prisma_cloud.alert.event_occurred", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.eventOccurred".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_eventOccurred")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("json.firstSeen") && event.get_str("json.firstSeen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.firstSeen") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("prisma_cloud.alert.first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.firstSeen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_firstSeen")?;
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
                                .get("_ingest.pipeline")
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
                .get("prisma_cloud.alert.first_seen")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if event.has_value("json.history") {
                event.rename("json.history", "prisma_cloud.alert.history")?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.history")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.alert.history", |event| {
                        if event.has_value("_ingest._value.modifiedBy") {
                            event.rename(
                                "_ingest._value.modifiedBy",
                                "_ingest._value.modified_by",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.history")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("prisma_cloud.alert.history").cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.modifiedOn")
                                    {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.modified_on", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.modifiedOn".into(),
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
                                    event.remove("_ingest._value.modifiedOn");
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
                                "prisma_cloud.alert.history",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.history")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.alert.history", |event| {
                        event.remove("_ingest._value.modifiedOn");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.id") {
                event.rename("json.id", "prisma_cloud.alert.id")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.alert.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond =
                { event.has_value("json.lastSeen") && event.get_str("json.lastSeen") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastSeen") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("prisma_cloud.alert.last.seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.lastSeen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastSeen")?;
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
                                .get("_ingest.pipeline")
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
                .get("prisma_cloud.alert.last.seen")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.has_value("json.lastUpdated") && event.get_str("json.lastUpdated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastUpdated") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.lastUpdated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastUpdated")?;
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
                                .get("_ingest.pipeline")
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
                event.has_value("json.lastUpdated") && event.get_str("json.lastUpdated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastUpdated") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("prisma_cloud.alert.last.updated", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.lastUpdated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastUpdated")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.metadata.saveSearchId") {
                event.rename(
                    "json.metadata.saveSearchId",
                    "prisma_cloud.alert.metadata.save_search_id",
                )?;
            }

            if event.has_value("json.policy.cloudType") {
                event.rename(
                    "json.policy.cloudType",
                    "prisma_cloud.alert.policy.cloud_type",
                )?;
            }

            if event.has_value("json.policy.complianceMetadata") {
                event.rename(
                    "json.policy.complianceMetadata",
                    "prisma_cloud.alert.policy.compliance_metadata",
                )?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.complianceId") {
                                event.rename(
                                    "_ingest._value.complianceId",
                                    "_ingest._value.compliance_id",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.customAssigned") {
                                    if let Some(val) = event.get("_ingest._value.customAssigned") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.customAssigned".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.custom_assigned", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_alert_policy_compliance_metadata_customAssigned")?;
                                event.remove("_ingest._value.customAssigned");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.policyId") {
                                event.rename(
                                    "_ingest._value.policyId",
                                    "_ingest._value.policy_id",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.requirementDescription") {
                                event.rename(
                                    "_ingest._value.requirementDescription",
                                    "_ingest._value.requirement.description",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.requirementId") {
                                event.rename(
                                    "_ingest._value.requirementId",
                                    "_ingest._value.requirement.id",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.requirementName") {
                                event.rename(
                                    "_ingest._value.requirementName",
                                    "_ingest._value.requirement.name",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.sectionDescription") {
                                event.rename(
                                    "_ingest._value.sectionDescription",
                                    "_ingest._value.section.description",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.sectionId") {
                                event.rename(
                                    "_ingest._value.sectionId",
                                    "_ingest._value.section.id",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.sectionLabel") {
                                event.rename(
                                    "_ingest._value.sectionLabel",
                                    "_ingest._value.section.label",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.sectionViewOrder") {
                                    if let Some(val) = event.get("_ingest._value.sectionViewOrder")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.sectionViewOrder".into(),
                                                    message,
                                                }
                                            })?;
                                        event
                                            .set("_ingest._value.section.view_order", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_alert_policy_compliance_metadata_sectionViewOrder")?;
                                event.remove("_ingest._value.sectionViewOrder");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.requirementViewOrder") {
                                    if let Some(val) =
                                        event.get("_ingest._value.requirementViewOrder")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.requirementViewOrder"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.requirement.view_order",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_alert_policy_compliance_metadata_requirementViewOrder")?;
                                event.remove("_ingest._value.requirementViewOrder");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.systemDefault") {
                                    if let Some(val) = event.get("_ingest._value.systemDefault") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.systemDefault".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.system_default", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_alert_policy_compliance_metadata_systemDefault")?;
                                event.remove("_ingest._value.systemDefault");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.standardDescription") {
                                event.rename(
                                    "_ingest._value.standardDescription",
                                    "_ingest._value.standard.description",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.standardId") {
                                event.rename(
                                    "_ingest._value.standardId",
                                    "_ingest._value.standard.id",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            if event.has_value("_ingest._value.standardName") {
                                event.rename(
                                    "_ingest._value.standardName",
                                    "_ingest._value.standard.name",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.policy.compliance_metadata")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.policy.compliance_metadata",
                        |event| {
                            event.remove("_ingest._value.customAssigned");
                            event.remove("_ingest._value.sectionViewOrder");
                            event.remove("_ingest._value.requirementViewOrder");
                            event.remove("_ingest._value.systemDefault");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.policy.createdBy") {
                event.rename(
                    "json.policy.createdBy",
                    "prisma_cloud.alert.policy.created_by",
                )?;
            }

            let _cond = {
                event.has_value("json.policy.createdOn")
                    && event.get_str("json.policy.createdOn") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.policy.createdOn") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("prisma_cloud.alert.policy.created_on", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.policy.createdOn".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_createdOn")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.policy.deleted") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.policy.deleted") {
                        if let Some(val) = event.get("json.policy.deleted") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.policy.deleted".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.alert.policy.deleted", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_deleted")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.policy.description") {
                event.rename(
                    "json.policy.description",
                    "prisma_cloud.alert.policy.description",
                )?;
            }

            let _cond = { event.get_str("json.policy.enabled") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.policy.enabled") {
                        if let Some(val) = event.get("json.policy.enabled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.policy.enabled".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.alert.policy.enabled", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_enabled")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.policy.findingTypes") {
                event.rename(
                    "json.policy.findingTypes",
                    "prisma_cloud.alert.policy.finding_types",
                )?;
            }

            if event.has_value("json.policy.labels") {
                event.rename("json.policy.labels", "prisma_cloud.alert.policy.labels")?;
            }

            if event.has_value("json.policy.lastModifiedBy") {
                event.rename(
                    "json.policy.lastModifiedBy",
                    "prisma_cloud.alert.policy.last_modified_by",
                )?;
            }

            let _cond = {
                event.has_value("json.policy.lastModifiedOn")
                    && event.get_str("json.policy.lastModifiedOn") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.policy.lastModifiedOn") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("prisma_cloud.alert.policy.last_modified_on", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.policy.lastModifiedOn".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_lastModifiedOn")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.policy.name") {
                event.rename("json.policy.name", "prisma_cloud.alert.policy.name")?;
            }

            if event.has_value("json.policy.policyId") {
                event.rename("json.policy.policyId", "prisma_cloud.alert.policy.id")?;
            }

            if event.has_value("json.policy.policyType") {
                event.rename("json.policy.policyType", "prisma_cloud.alert.policy.type")?;
            }

            if event.has_value("json.policy.policyUpi") {
                event.rename("json.policy.policyUpi", "prisma_cloud.alert.policy.upi")?;
            }

            if event.has_value("json.policy.recommendation") {
                event.rename(
                    "json.policy.recommendation",
                    "prisma_cloud.alert.policy.recommendation",
                )?;
            }

            let _cond = { event.get_str("json.policy.remediable") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.policy.remediable") {
                        if let Some(val) = event.get("json.policy.remediable") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.policy.remediable".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.alert.policy.remediable", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_remediable")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.policy.remediation") {
                event.rename(
                    "json.policy.remediation",
                    "prisma_cloud.alert.policy.remediation",
                )?;
            }

            if event.has_value("prisma_cloud.alert.policy.remediation.cliScriptTemplate") {
                event.rename(
                    "prisma_cloud.alert.policy.remediation.cliScriptTemplate",
                    "prisma_cloud.alert.policy.remediation.cli_script_template",
                )?;
            }

            let _cond = { event.get_str("json.policy.restrictAlertDismissal") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.policy.restrictAlertDismissal") {
                        if let Some(val) = event.get("json.policy.restrictAlertDismissal") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.policy.restrictAlertDismissal".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "prisma_cloud.alert.policy.restrict_alert_dismissal",
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
                        "rename_restrictAlertDismissal",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.policy.rule.apiName") {
                event.rename(
                    "json.policy.rule.apiName",
                    "prisma_cloud.alert.policy.rule.api_name",
                )?;
            }

            if event.has_value("json.policy.rule.cloudAccount") {
                event.rename(
                    "json.policy.rule.cloudAccount",
                    "prisma_cloud.alert.policy.rule.cloud.account",
                )?;
            }

            if event.has_value("json.policy.rule.cloudType") {
                event.rename(
                    "json.policy.rule.cloudType",
                    "prisma_cloud.alert.policy.rule.cloud.type",
                )?;
            }

            if event.has_value("json.policy.rule.criteria") {
                event.rename(
                    "json.policy.rule.criteria",
                    "prisma_cloud.alert.policy.rule.criteria",
                )?;
            }

            if event.has_value("json.policy.rule.dataCriteria.classificationResult") {
                event.rename(
                    "json.policy.rule.dataCriteria.classificationResult",
                    "prisma_cloud.alert.policy.rule.data_criteria.classification_result",
                )?;
            }

            if event.has_value("json.policy.rule.dataCriteria.exposure") {
                event.rename(
                    "json.policy.rule.dataCriteria.exposure",
                    "prisma_cloud.alert.policy.rule.data_criteria.exposure",
                )?;
            }

            if event.has_value("json.policy.rule.dataCriteria.extension") {
                event.rename(
                    "json.policy.rule.dataCriteria.extension",
                    "prisma_cloud.alert.policy.rule.data_criteria.extension",
                )?;
            }

            if event.has_value("json.policy.rule.name") {
                event.rename(
                    "json.policy.rule.name",
                    "prisma_cloud.alert.policy.rule.name",
                )?;
            }

            if event.has_value("json.policy.rule.parameters") {
                event.rename(
                    "json.policy.rule.parameters",
                    "prisma_cloud.alert.policy.rule.parameters",
                )?;
            }

            if event.has_value("json.policy.rule.resourceIdPath") {
                event.rename(
                    "json.policy.rule.resourceIdPath",
                    "prisma_cloud.alert.policy.rule.resource.id_path",
                )?;
            }

            if event.has_value("json.policy.rule.resourceType") {
                event.rename(
                    "json.policy.rule.resourceType",
                    "prisma_cloud.alert.policy.rule.resource.type",
                )?;
            }

            if event.has_value("json.policy.rule.type") {
                event.rename(
                    "json.policy.rule.type",
                    "prisma_cloud.alert.policy.rule.type",
                )?;
            }

            let _cond = {
                event.has_value("json.policy.ruleLastModifiedOn")
                    && event.get_str("json.policy.ruleLastModifiedOn") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.policy.ruleLastModifiedOn") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event
                                .set("prisma_cloud.alert.policy.rule.last_modified_on", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.policy.ruleLastModifiedOn".into(),
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
                        "rename_ruleLastModifiedOn",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.policy.severity") {
                event.rename("json.policy.severity", "prisma_cloud.alert.policy.severity")?;
            }

            let _cond = { event.get_str("json.policy.systemDefault") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.policy.systemDefault") {
                        if let Some(val) = event.get("json.policy.systemDefault") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.policy.systemDefault".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.alert.policy.system_default", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "rename_systemdDefault")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.policyId") {
                event.rename("json.policyId", "prisma_cloud.alert.policy_id")?;
            }

            if event.has_value("json.reason") {
                event.rename("json.reason", "prisma_cloud.alert.reason")?;
            }

            if event.has_value("json.resource") {
                event.rename("json.resource", "prisma_cloud.alert.resource")?;
            }

            if event.has_value("prisma_cloud.alert.resource.account") {
                event.rename(
                    "prisma_cloud.alert.resource.account",
                    "prisma_cloud.alert.resource.account.value",
                )?;
            }

            if event.has_value("prisma_cloud.alert.resource.accountId") {
                event.rename(
                    "prisma_cloud.alert.resource.accountId",
                    "prisma_cloud.alert.resource.account.id",
                )?;
            }

            if let Some(v) = event
                .get("prisma_cloud.alert.resource.account.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if event.has_value("prisma_cloud.alert.resource.additionalInfo") {
                event.rename(
                    "prisma_cloud.alert.resource.additionalInfo",
                    "prisma_cloud.alert.resource.additional_info",
                )?;
            }

            if event.has_value("prisma_cloud.alert.resource.cloudAccountAncestors") {
                event.rename(
                    "prisma_cloud.alert.resource.cloudAccountAncestors",
                    "prisma_cloud.alert.resource.cloud.account.ancestors",
                )?;
            }

            if event.has_value("prisma_cloud.alert.resource.cloudAccountGroups") {
                event.rename(
                    "prisma_cloud.alert.resource.cloudAccountGroups",
                    "prisma_cloud.alert.resource.cloud.account.groups",
                )?;
            }

            if event.has_value("prisma_cloud.alert.resource.cloudAccountOwners") {
                event.rename(
                    "prisma_cloud.alert.resource.cloudAccountOwners",
                    "prisma_cloud.alert.resource.cloud.account.owners",
                )?;
            }

            if event.has_value("prisma_cloud.alert.resource.cloudServiceName") {
                event.rename(
                    "prisma_cloud.alert.resource.cloudServiceName",
                    "prisma_cloud.alert.resource.cloud.service_name",
                )?;
            }

            if let Some(v) = event
                .get("prisma_cloud.alert.resource.cloud.service_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.service.name", v)?;
            }

            if event.has_value("prisma_cloud.alert.resource.cloudType") {
                event.rename(
                    "prisma_cloud.alert.resource.cloudType",
                    "prisma_cloud.alert.resource.cloud.type",
                )?;
            }

            if let Some(v) = event
                .get("prisma_cloud.alert.resource.cloud.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if event.has_value("prisma_cloud.alert.resource.region") {
                event.rename(
                    "prisma_cloud.alert.resource.region",
                    "prisma_cloud.alert.resource.region.value",
                )?;
            }

            if event.has_value("prisma_cloud.alert.resource.regionId") {
                event.rename(
                    "prisma_cloud.alert.resource.regionId",
                    "prisma_cloud.alert.resource.region.id",
                )?;
            }

            if event.has_value("prisma_cloud.alert.resource.resourceApiName") {
                event.rename(
                    "prisma_cloud.alert.resource.resourceApiName",
                    "prisma_cloud.alert.resource.api_name",
                )?;
            }

            let _cond = {
                event.get_str("prisma_cloud.alert.resource.resourceConfigJsonAvailable") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("prisma_cloud.alert.resource.resourceConfigJsonAvailable") {
                        if let Some(val) =
                            event.get("prisma_cloud.alert.resource.resourceConfigJsonAvailable")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "prisma_cloud.alert.resource.resourceConfigJsonAvailable"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "prisma_cloud.alert.resource.config_json_available",
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
                        "convert_resourceConfigJsonAvailable",
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
                                .get("_ingest.pipeline")
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
                event.get_str("prisma_cloud.alert.resource.resourceDetailsAvailable") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("prisma_cloud.alert.resource.resourceDetailsAvailable") {
                        if let Some(val) =
                            event.get("prisma_cloud.alert.resource.resourceDetailsAvailable")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "prisma_cloud.alert.resource.resourceDetailsAvailable"
                                        .into(),
                                    message,
                                }
                            })?;
                            event
                                .set("prisma_cloud.alert.resource.details_available", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "rename_resourceDetailsAvailable",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("prisma_cloud.alert.resource.resourceTags") {
                event.rename(
                    "prisma_cloud.alert.resource.resourceTags",
                    "prisma_cloud.alert.resource.tags",
                )?;
            }

            let _cond = {
                event.has_value("prisma_cloud.alert.resource.resourceTs")
                    && event.get_str("prisma_cloud.alert.resource.resourceTs") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("prisma_cloud.alert.resource.resourceTs")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("prisma_cloud.alert.resource.ts", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "prisma_cloud.alert.resource.resourceTs".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "rename_resourceTs")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("prisma_cloud.alert.resource.resourceType") {
                event.rename(
                    "prisma_cloud.alert.resource.resourceType",
                    "prisma_cloud.alert.resource.type",
                )?;
            }

            if event.has_value("prisma_cloud.alert.resource.unifiedAssetId") {
                event.rename(
                    "prisma_cloud.alert.resource.unifiedAssetId",
                    "prisma_cloud.alert.resource.unified_asset_id",
                )?;
            }

            let _cond = { event.has_value("prisma_cloud.alert.resource.url") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if !uri_parts(event, "prisma_cloud.alert.resource.url", "url", true, false)?
                        && event
                            .get_str("prisma_cloud.alert.resource.url")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "prisma_cloud.alert.resource.url".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.pipeline")
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

            if event.has_value("json.riskDetail") {
                event.rename("json.riskDetail", "prisma_cloud.alert.risk_detail")?;
            }

            if event.has_value("prisma_cloud.alert.risk_detail.policyScores") {
                event.rename(
                    "prisma_cloud.alert.risk_detail.policyScores",
                    "prisma_cloud.alert.risk_detail.policy_scores",
                )?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.cloudType") {
                                event.rename(
                                    "_ingest._value.cloudType",
                                    "_ingest._value.cloud_type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.complianceMetadata") {
                                event.rename(
                                    "_ingest._value.complianceMetadata",
                                    "_ingest._value.compliance_metadata",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.complianceId") {
                                            event.rename(
                                                "_ingest._value.complianceId",
                                                "_ingest._value.compliance_id",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        // on_failure: 2 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if event.has_value("_ingest._value.customAssigned") {
                                                if let Some(val) =
                                                    event.get("_ingest._value.customAssigned")
                                                {
                                                    let converted = convert_value(val, "boolean")
                                                        .map_err(|message| {
                                                        TransformError::ParseError {
                                                            path: "_ingest._value.customAssigned"
                                                                .into(),
                                                            message,
                                                        }
                                                    })?;
                                                    event.set(
                                                        "_ingest._value.custom_assigned",
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
                                            event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_alert_risk_detail_policy_scores_compliance_metadata_customAssigned")?;
                                            event.remove("_ingest._value.customAssigned");
                                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.policyId") {
                                            event.rename(
                                                "_ingest._value.policyId",
                                                "_ingest._value.policy.id",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.requirementDescription")
                                        {
                                            event.rename(
                                                "_ingest._value.requirementDescription",
                                                "_ingest._value.requirement.description",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.requirementId") {
                                            event.rename(
                                                "_ingest._value.requirementId",
                                                "_ingest._value.requirement.id",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.requirementName") {
                                            event.rename(
                                                "_ingest._value.requirementName",
                                                "_ingest._value.requirement.name",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.sectionDescription") {
                                            event.rename(
                                                "_ingest._value.sectionDescription",
                                                "_ingest._value.section.description",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.sectionId") {
                                            event.rename(
                                                "_ingest._value.sectionId",
                                                "_ingest._value.section.id",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.sectionLabel") {
                                            event.rename(
                                                "_ingest._value.sectionLabel",
                                                "_ingest._value.section.label",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.standardDescription") {
                                            event.rename(
                                                "_ingest._value.standardDescription",
                                                "_ingest._value.standard.description",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.standardId") {
                                            event.rename(
                                                "_ingest._value.standardId",
                                                "_ingest._value.standard.id",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        if event.has_value("_ingest._value.standardName") {
                                            event.rename(
                                                "_ingest._value.standardName",
                                                "_ingest._value.standard.name",
                                            )?;
                                        }
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(
                                    event,
                                    "_ingest._value.compliance_metadata",
                                    |event| {
                                        event.remove("_ingest._value.customAssigned");
                                        Ok(())
                                    },
                                )?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.createdBy") {
                                event.rename(
                                    "_ingest._value.createdBy",
                                    "_ingest._value.created.by",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("prisma_cloud.alert.risk_detail.policy_scores")
                            .cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.createdOn")
                                    {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.created.on", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.createdOn".into(),
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
                                    event.remove("_ingest._value.createdOn");
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
                                "prisma_cloud.alert.risk_detail.policy_scores",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.deleted") {
                                    if let Some(val) = event.get("_ingest._value.deleted") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.deleted".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.deleted", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_prisma_cloud_alert_risk_detail_policy_scores_deleted",
                                )?;
                                event.remove("_ingest._value.deleted");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.enabled") {
                                    if let Some(val) = event.get("_ingest._value.enabled") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.enabled".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.enabled", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_prisma_cloud_alert_risk_detail_policy_scores_enabled",
                                )?;
                                event.remove("_ingest._value.enabled");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.findingTypes") {
                                event.rename(
                                    "_ingest._value.findingTypes",
                                    "_ingest._value.finding_types",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.lastModifiedBy") {
                                event.rename(
                                    "_ingest._value.lastModifiedBy",
                                    "_ingest._value.last_modified.by",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("prisma_cloud.alert.risk_detail.policy_scores")
                            .cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.lastModifiedOn")
                                    {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => event
                                                .set("_ingest._value.last_modified.on", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.lastModifiedOn".into(),
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
                                    event.remove("_ingest._value.lastModifiedOn");
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
                                "prisma_cloud.alert.risk_detail.policy_scores",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.overridden") {
                                    if let Some(val) = event.get("_ingest._value.overridden") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.overridden".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.overridden", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "rename_prisma_cloud_alert_risk_detail_policy_scores_overridden")?;
                                event.remove("_ingest._value.overridden");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.policyId") {
                                event.rename(
                                    "_ingest._value.policyId",
                                    "_ingest._value.policy.id",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.policySubTypes") {
                                event.rename(
                                    "_ingest._value.policySubTypes",
                                    "_ingest._value.policy.subtypes",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.policyType") {
                                event.rename(
                                    "_ingest._value.policyType",
                                    "_ingest._value.policy.type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.policyUpi") {
                                event.rename(
                                    "_ingest._value.policyUpi",
                                    "_ingest._value.policy.upi",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.remediable") {
                                    if let Some(val) = event.get("_ingest._value.remediable") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.remediable".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.remediable", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_alert_risk_detail_policy_scores_remediable")?;
                                event.remove("_ingest._value.remediable");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.remediation.cliScriptTemplate") {
                                event.rename(
                                    "_ingest._value.remediation.cliScriptTemplate",
                                    "_ingest._value.remediation.cli_script_template",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.restrictAlertDismissal") {
                                    if let Some(val) =
                                        event.get("_ingest._value.restrictAlertDismissal")
                                    {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.restrictAlertDismissal"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set(
                                            "_ingest._value.restrict_alert_dismissal",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "rename_prisma_cloud_alert_risk_detail_policy_scores_restrictAlertDismissal")?;
                                event.remove("_ingest._value.restrictAlertDismissal");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.riskScore.maxScore") {
                                    if let Some(val) =
                                        event.get("_ingest._value.riskScore.maxScore")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.riskScore.maxScore"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.risk_score.max", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_prisma_cloud_alert_risk_detail_policy_scores_maxScore",
                                )?;
                                event.remove("_ingest._value.riskScore.maxScore");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.riskScore.score") {
                                    if let Some(val) = event.get("_ingest._value.riskScore.score") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.riskScore.score".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.risk_score.value", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_prisma_cloud_alert_risk_detail_policy_scores_score",
                                )?;
                                event.remove("_ingest._value.riskScore.score");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.rule.apiName") {
                                event.rename(
                                    "_ingest._value.rule.apiName",
                                    "_ingest._value.rule.api_name",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.rule.cloudAccount") {
                                event.rename(
                                    "_ingest._value.rule.cloudAccount",
                                    "_ingest._value.rule.cloud.account",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.rule.cloudType") {
                                event.rename(
                                    "_ingest._value.rule.cloudType",
                                    "_ingest._value.rule.cloud.type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.rule.dataCriteria") {
                                event.rename(
                                    "_ingest._value.rule.dataCriteria",
                                    "_ingest._value.rule.data_criteria",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event
                                .has_value("_ingest._value.rule.data_criteria.classificationResult")
                            {
                                event.rename(
                                    "_ingest._value.rule.data_criteria.classificationResult",
                                    "_ingest._value.rule.data_criteria.classification_result",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.rule.resourceIdPath") {
                                event.rename(
                                    "_ingest._value.rule.resourceIdPath",
                                    "_ingest._value.rule.resource.id_path",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            if event.has_value("_ingest._value.rule.resourceType") {
                                event.rename(
                                    "_ingest._value.rule.resourceType",
                                    "_ingest._value.rule.resource.type",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("prisma_cloud.alert.risk_detail.policy_scores")
                            .cloned();
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
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.ruleLastModifiedOn")
                                    {
                                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.rule.last_modified_on",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.ruleLastModifiedOn"
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
                                    event.remove("_ingest._value.ruleLastModifiedOn");
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
                                "prisma_cloud.alert.risk_detail.policy_scores",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.systemDefault") {
                                    if let Some(val) = event.get("_ingest._value.systemDefault") {
                                        let converted =
                                            convert_value(val, "boolean").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.systemDefault".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.system_default", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_alert_risk_detail_policy_scores_systemDefault")?;
                                event.remove("_ingest._value.systemDefault");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.alert.risk_detail.policy_scores")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.alert.risk_detail.policy_scores",
                        |event| {
                            event.remove("_ingest._value.systemDefault");
                            event.remove("_ingest._value.ruleLastModifiedOn");
                            event.remove("_ingest._value.restrictAlertDismissal");
                            event.remove("_ingest._value.lastModifiedOn");
                            event.remove("_ingest._value.createdOn");
                            event.remove("_ingest._value.riskScore");
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond =
                { event.get_str("prisma_cloud.alert.risk_detail.riskScore.maxScore") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("prisma_cloud.alert.risk_detail.riskScore.maxScore") {
                        if let Some(val) =
                            event.get("prisma_cloud.alert.risk_detail.riskScore.maxScore")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "prisma_cloud.alert.risk_detail.riskScore.maxScore"
                                        .into(),
                                    message,
                                }
                            })?;
                            event
                                .set("prisma_cloud.alert.risk_detail.risk_score.max", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "foreach_convert_maxScore",
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
                                .get("_ingest.pipeline")
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
                { event.get_str("prisma_cloud.alert.risk_detail.riskScore.score") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("prisma_cloud.alert.risk_detail.riskScore.score") {
                        if let Some(val) =
                            event.get("prisma_cloud.alert.risk_detail.riskScore.score")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "prisma_cloud.alert.risk_detail.riskScore.score".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "prisma_cloud.alert.risk_detail.risk_score.value",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "foreach_convert_score")?;
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.saveSearchId") {
                event.rename("json.saveSearchId", "prisma_cloud.alert.save_search_id")?;
            }

            if event.has_value("json.status") {
                event.rename("json.status", "prisma_cloud.alert.status")?;
            }

            if event.has_value("json.triggeredBy") {
                event.rename("json.triggeredBy", "prisma_cloud.alert.triggered_by")?;
            }

            event.remove("json");
            event.remove("prisma_cloud.alert.attribution.resourceCreatedOn");
            event.remove("prisma_cloud.alert.risk_detail.riskScore");
            event.remove("prisma_cloud.alert.resource.resourceConfigJsonAvailable");
            event.remove("prisma_cloud.alert.resource.resourceDetailsAvailable");
            event.remove("prisma_cloud.alert.resource.resourceTs");

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
                event.remove("prisma_cloud.alert.first_seen");
                event.remove("prisma_cloud.alert.id");
                event.remove("prisma_cloud.alert.last.seen");
                event.remove("prisma_cloud.alert.last.updated");
                event.remove("prisma_cloud.alert.resource.account.id");
                event.remove("prisma_cloud.alert.resource.cloud.service_name");
                event.remove("prisma_cloud.alert.resource.cloud.type");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
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
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "script_painless")?;
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
                            .get("_ingest.pipeline")
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
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
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
