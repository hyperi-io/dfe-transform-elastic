// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `match_details` pipeline.
pub struct MatchDetails;

impl Transform for MatchDetails {
    fn name(&self) -> &str {
        "match_details"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                // Painless script, resolved to its runners at generation time
                // Source: boolean dropEmptyMapsAndLists(Object object) {\n  if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyMapsAndLists(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyMapsAndLists(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyMapsAndLists(ctx.tanium.threat_response.match_details);\n
                drop_empty(event, &DropPolicy { empty_collections: true, prune_lists: true, ..DropPolicy::none() }, Some("tanium.threat_response.match_details"));

                if event.has_value("tanium.threat_response.match_details.match.properties.file.fullpath") {
                    event.rename("tanium.threat_response.match_details.match.properties.file.fullpath", "tanium.threat_response.match_details.match.properties.file.full_path")?;
                }

                if event.has_value("tanium.threat_response.match_details.match.properties.parent.file.fullpath") {
                    event.rename("tanium.threat_response.match_details.match.properties.parent.file.fullpath", "tanium.threat_response.match_details.match.properties.parent.file.full_path")?;
                }

                if event.has_value("tanium.threat_response.match_details.finding.finding_id") {
                    event.rename("tanium.threat_response.match_details.finding.finding_id", "tanium.threat_response.match_details.finding.id")?;
                }

                if event.has_value("tanium.threat_response.match_details.match.properties.parent.parent.file.fullpath") {
                    event.rename("tanium.threat_response.match_details.match.properties.parent.parent.file.fullpath", "tanium.threat_response.match_details.match.properties.parent.parent.file.full_path")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("tanium.threat_response.match_details.finding.system_info.os") {
                if let Some(input) = event.get_string("tanium.threat_response.match_details.finding.system_info.os") {
                    // Grok pattern: ^%{DATA:tanium.threat_response.match_details.finding.system_info.os_platform} %{GREEDYDATA:tanium.threat_response.match_details.finding.system_info.os_version}$
                    // Grok pattern: ^%{GREEDYDATA:tanium.threat_response.match_details.system_info.os}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^%{DATA:tanium.threat_response.match_details.finding.system_info.os_platform} %{GREEDYDATA:tanium.threat_response.match_details.finding.system_info.os_version}$"),
                            cached_grok!("^%{GREEDYDATA:tanium.threat_response.match_details.system_info.os}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_tanium_threat_response_match_details_finding_system_info_os")?;
                        event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("tanium.threat_response.match_details.finding.system_info.os") {
                    event.rename("tanium.threat_response.match_details.finding.system_info.os", "tanium.threat_response.match_details.finding.system_info.os.value")?;
                }

                if event.has_value("tanium.threat_response.match_details.finding.system_info.os_platform") {
                    event.rename("tanium.threat_response.match_details.finding.system_info.os_platform", "tanium.threat_response.match_details.finding.system_info.os.platform")?;
                }

            if let Some(v) = event.get("tanium.threat_response.match_details.finding.system_info.os.platform").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("os.platform", v)?;
            }

                if event.has_value("tanium.threat_response.match_details.finding.system_info.os_version") {
                    event.rename("tanium.threat_response.match_details.finding.system_info.os_version", "tanium.threat_response.match_details.finding.system_info.os.version")?;
                }

            if let Some(v) = event.get("tanium.threat_response.match_details.finding.system_info.os.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("os.version", v)?;
            }

            if event.has_value("tanium.threat_response.match_details.finding.system_info.platform") {
                map_strings(event, "tanium.threat_response.match_details.finding.system_info.platform", "tanium.threat_response.match_details.finding.system_info.platform", str::to_lowercase)?;
            }

            if let Some(v) = event.get("tanium.threat_response.match_details.finding.system_info.platform").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("os.type", v)?;
            }

            let _cond = { event.get_str("tanium.threat_response.match_details.finding.system_info.bits") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("tanium.threat_response.match_details.finding.system_info.bits") {
                if let Some(val) = event.get("tanium.threat_response.match_details.finding.system_info.bits") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "tanium.threat_response.match_details.finding.system_info.bits".into(),
                            message,
                        })?;
                    event.set("tanium.threat_response.match_details.finding.system_info.bits", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_tanium_threat_response_match_details_finding_system_info_bits")?;
                        event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("tanium.threat_response.match_details.match.contexts").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "tanium.threat_response.match_details.match.contexts", |event| {
                    if event.has_value("_ingest._value.file.uniqueEventId") {
                    event.rename("_ingest._value.file.uniqueEventId", "_ingest._value.file.unique_event_id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("tanium.threat_response.match_details.match.contexts").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "tanium.threat_response.match_details.match.contexts", |event| {
                    if event.has_value("_ingest._value.event.fileCreate.path") {
                    event.rename("_ingest._value.event.fileCreate.path", "_ingest._value.event.file_create.path")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("tanium.threat_response.match_details.finding.first_seen") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("tanium.threat_response.match_details.finding.first_seen") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("tanium.threat_response.match_details.finding.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tanium.threat_response.match_details.finding.first_seen".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tanium_threat_response_match_details_finding_first_seen")?;
                        event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("tanium.threat_response.match_details.finding.last_seen") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("tanium.threat_response.match_details.finding.last_seen") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("tanium.threat_response.match_details.finding.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tanium.threat_response.match_details.finding.last_seen".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tanium_threat_response_match_details_finding_last_seen")?;
                        event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("tanium.threat_response.match_details.match.contexts").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("tanium.threat_response.match_details.match.contexts").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            if let Some(date_str) = event.get_as_string("_ingest._value.event.timestampMs") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("_ingest._value.event.timestampMs", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.event.timestampMs".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_ingest_value_event_timestampms")?;
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
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
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("tanium.threat_response.match_details.match.contexts", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.has_value("tanium.threat_response.match_details.match.properties.parent.parent.start_time") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("tanium.threat_response.match_details.match.properties.parent.parent.start_time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("tanium.threat_response.match_details.match.properties.parent.parent.start_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tanium.threat_response.match_details.match.properties.parent.parent.start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tanium_threat_response_match_details_match_properties_parent_parent_start_time")?;
                        event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("tanium.threat_response.match_details.match.properties.parent.start_time") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("tanium.threat_response.match_details.match.properties.parent.start_time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("tanium.threat_response.match_details.match.properties.parent.start_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tanium.threat_response.match_details.match.properties.parent.start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tanium_threat_response_match_details_match_properties_parent_start_time")?;
                        event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("tanium.threat_response.match_details.match.properties.start_time") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("tanium.threat_response.match_details.match.properties.start_time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("tanium.threat_response.match_details.match.properties.start_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tanium.threat_response.match_details.match.properties.start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tanium_threat_response_match_details_match_properties_start_time")?;
                        event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if event.has_value("tanium.threat_response.match_details.finding.whats") {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("tanium.threat_response.match_details.finding.whats").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // Begin nested pipeline: "match_details_whats"
                            event.rename("_ingest._value", "_what")?;
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.domain") {
                            event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.domain", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.domain")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.user.user.domain") {
                            event.rename("_what.artifact_activity.acting_artifact.process.parent.process.user.user.domain", "_what.artifact_activity.acting_artifact.process.parent.process.user.domain")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.user.user.domain") {
                            event.rename("_what.artifact_activity.acting_artifact.process.user.user.domain", "_what.artifact_activity.acting_artifact.process.user.domain")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.name") {
                            event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.name", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.name")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.user.user.name") {
                            event.rename("_what.artifact_activity.acting_artifact.process.parent.process.user.user.name", "_what.artifact_activity.acting_artifact.process.parent.process.user.name")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.user.user.name") {
                            event.rename("_what.artifact_activity.acting_artifact.process.user.user.name", "_what.artifact_activity.acting_artifact.process.user.name")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.user_id") {
                            event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.user.user_id", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.user.id")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.user.user.user_id") {
                            event.rename("_what.artifact_activity.acting_artifact.process.parent.process.user.user.user_id", "_what.artifact_activity.acting_artifact.process.parent.process.user.id")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.user.user.user_id") {
                            event.rename("_what.artifact_activity.acting_artifact.process.user.user.user_id", "_what.artifact_activity.acting_artifact.process.user.id")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.user.user.group_id") {
                            event.rename("_what.artifact_activity.acting_artifact.process.user.user.group_id", "_what.artifact_activity.acting_artifact.process.user.group_id")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.file.path") {
                            event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.file.path", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.path")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.file.file.path") {
                            event.rename("_what.artifact_activity.acting_artifact.process.parent.process.file.file.path", "_what.artifact_activity.acting_artifact.process.parent.process.file.path")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.file.file.path") {
                            event.rename("_what.artifact_activity.acting_artifact.process.file.file.path", "_what.artifact_activity.acting_artifact.process.file.path")?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.file.hash.md5") {
                            event.rename("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.file.hash.md5", "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.file.hash.md5")?;
                            }
                            let _cond = { event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.start_time") };
                            if _cond {
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.start_time") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.start_time", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.start_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_what_artifact_activity_acting_artifact_process_parent_process_parent_process_start_time")?;
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            }
                            let _cond = { event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.start_time") };
                            if _cond {
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_what.artifact_activity.acting_artifact.process.parent.process.start_time") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_what.artifact_activity.acting_artifact.process.parent.process.start_time", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_what.artifact_activity.acting_artifact.process.parent.process.start_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_what_artifact_activity_acting_artifact_process_parent_process_start_time")?;
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            }
                            let _cond = { event.has_value("_what.artifact_activity.acting_artifact.process.start_time") };
                            if _cond {
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_what.artifact_activity.acting_artifact.process.start_time") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_what.artifact_activity.acting_artifact.process.start_time", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_what.artifact_activity.acting_artifact.process.start_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_what_artifact_activity_acting_artifact_process_start_time")?;
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.parent.process.handles") {
                            foreach_array(event, "_what.artifact_activity.acting_artifact.process.parent.process.parent.process.handles", |event| {
                            if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "string")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value".into(),
                            message,
                            })?;
                            event.set("_ingest._value", converted)?;
                            }
                            }
                            Ok(())
                            })?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.parent.process.handles") {
                            foreach_array(event, "_what.artifact_activity.acting_artifact.process.parent.process.handles", |event| {
                            if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "string")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value".into(),
                            message,
                            })?;
                            event.set("_ingest._value", converted)?;
                            }
                            }
                            Ok(())
                            })?;
                            }
                            if event.has_value("_what.artifact_activity.acting_artifact.process.handles") {
                            foreach_array(event, "_what.artifact_activity.acting_artifact.process.handles", |event| {
                            if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "string")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value".into(),
                            message,
                            })?;
                            event.set("_ingest._value", converted)?;
                            }
                            }
                            Ok(())
                            })?;
                            }
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_what.artifact_activity.acting_artifact.is_intel_target") {
                            if let Some(val) = event.get("_what.artifact_activity.acting_artifact.is_intel_target") {
                            let converted = convert_value(val, "boolean")
                            .map_err(|message| TransformError::ParseError {
                            path: "_what.artifact_activity.acting_artifact.is_intel_target".into(),
                            message,
                            })?;
                            event.set("_what.artifact_activity.acting_artifact.is_intel_target", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_what_artifact_activity_acting_artifact_is_intel_target")?;
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            if event.has_value("_what.artifact_activity.relevant_actions") {
                            {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("_what.artifact_activity.relevant_actions").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                            Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // Begin nested pipeline: "match_details_whats_actions"
                            event.rename("_ingest._value", "_action")?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_action.verb") {
                            if let Some(val) = event.get("_action.verb") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_action.verb".into(),
                            message,
                            })?;
                            event.set("_action.verb", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_action_verb")?;
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let _cond = { event.has_value("_action.tanium_recorder_context.event.timestamp_ms") };
                            if _cond {
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_action.tanium_recorder_context.event.timestamp_ms") {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("_action.tanium_recorder_context.event.timestamp_ms", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_action.tanium_recorder_context.event.timestamp_ms".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_action_tanium_recorder_context_event_timestamp_ms")?;
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            }
                            let _cond = { event.has_value("_action.target.file.modification_time") };
                            if _cond {
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_action.target.file.modification_time") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_action.target.file.modification_time", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_action.target.file.modification_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_action_target_file_modification_time")?;
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            }
                            let _cond = { event.has_value("_action.timestamp") };
                            if _cond {
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_action.timestamp") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_action.timestamp", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_action.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_action_timestamp")?;
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            }
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                            event.append_unique("related.hash", json!(event.get("_action.target.file.hash.md5").map_or_else(String::new, template_to_string)))?;
                            Ok(())
                            })();
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                            event.append_unique("related.hash", json!(event.get("_action.target.file.hash.sha1").map_or_else(String::new, template_to_string)))?;
                            Ok(())
                            })();
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                            event.append_unique("related.hash", json!(event.get("_action.target.file.hash.sha256").map_or_else(String::new, template_to_string)))?;
                            Ok(())
                            })();
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_action.target.file.size_bytes") {
                            if let Some(val) = event.get("_action.target.file.size_bytes") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_action.target.file.size_bytes".into(),
                            message,
                            })?;
                            event.set("_action.target.file.size_bytes", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_action_target_file_size_bytes")?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                            event.append_unique("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                            Ok(())
                            })();
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            if let Some(v) = event.get("_action").cloned() {
                            event.set("_ingest._value", v)?;
                            }
                            if event.remove("_action").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_action".into() });
                            }
                            // End nested pipeline: "match_details_whats_actions"
                            let left = event.remove("_ingest._value");
                            match key {
                            // An entry the body renamed AWAY is gone from the
                            // object, which is how a foreach lifts fields up.
                            Some(key) => {
                            if let Some(value) = left { fields.insert(key, value); }
                            }
                            None => list.push(left.unwrap_or(Value::Null)),
                            }
                            }
                            match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                            }
                            if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                            }
                            event.set("_what.artifact_activity.relevant_actions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                            }
                            }
                            }
                            if let Some(v) = event.get("_what").cloned() {
                            event.set("_ingest._value", v)?;
                            }
                            if event.remove("_what").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_what".into() });
                            }
                            // End nested pipeline: "match_details_whats"
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("tanium.threat_response.match_details.finding.whats", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            if let Some(v) = event.get("tanium.threat_response.match_details.finding.whats.artifact_activity.relevant_actions.target.file.size_bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("file.size", v)?;
            }

            if let Some(v) = event.get("tanium.threat_response.match_details.finding.whats.artifact_activity.acting_artifact.process.pid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pid", v)?;
            }

            let _cond = { event.has_value("error.message") && event.get_str("error.message") != Some("") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
            if _cond {
                event.remove("tanium.threat_response.match_details.finding.whats.artifact_activity.relevant_actions.target.file.hash.md5");
                event.remove("tanium.threat_response.match_details.finding.whats.artifact_activity.relevant_actions.target.file.hash.sha1");
                event.remove("tanium.threat_response.match_details.finding.whats.artifact_activity.relevant_actions.target.file.hash.sha256");
                event.remove("tanium.threat_response.match_details.finding.whats.artifact_activity.acting_artifact.process.pid");
                event.remove("tanium.threat_response.match_details.finding.system_info.platform");
                event.remove("tanium.threat_response.match_details.finding.system_info.os.platform");
                event.remove("tanium.threat_response.match_details.finding.system_info.os.version");
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
