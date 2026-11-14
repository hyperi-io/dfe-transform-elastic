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
            event.set("ecs.version", json!("9.4.0"))?;

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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { event.get("event").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("event") {
                    event.rename("event", "json.event")?;
                }
            }

            let _cond = { event.has_value("json.event") && event.has_value("data") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "30f5dfb289e7b9cf4888a0118ad8346585c9fddfd5207cc54f6638b2cbca4ec0",
                        r#"{"params":null,"known":[{"MoveKeys":[["id","json.id"],["timestamp","json.timestamp"],["data","json.data"]]}]}"#
                    ),
                )?;
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

            let _cond = { !event.has_value("json") && event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond =
                { !event.has_value("json.timestamp") && event.has_value("json.detected_at") };
            if _cond {
                if let Some(v) = event.get("_ingest.timestamp").cloned() {
                    event.set("_tmp.ingest_timestamp", v)?;
                }
            }

            let _cond =
                { !event.has_value("json.timestamp") && event.has_value("json.detected_at") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "46874178d40278e7d579b84db17760e7fb181710715f86f6a6815e21c09c738c",
                        r#"{"params":null,"known":[]}"#
                    ),
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "event.id")?;
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
                event.set("_tmp.is_webhook", json!(true))?;
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
                // Begin nested pipeline: "webhook"
                if event.has_value("json.event") {
                    event.rename("json.event", "event.action")?;
                }
                if event.has_value("json.data.device.name") {
                    event.rename("json.data.device.name", "host.name")?;
                }
                if event.has_value("json.data.device.id") {
                    if let Some(val) = event.get("json.data.device.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.device.id".into(),
                                message,
                            }
                        })?;
                        event.set("host.id", converted)?;
                    }
                }
                if event.has_value("json.data.check.name") {
                    event.rename("json.data.check.name", "rule.name")?;
                }
                if event.has_value("json.data.check.id") {
                    if let Some(val) = event.get("json.data.check.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.check.id".into(),
                                message,
                            }
                        })?;
                        event.set("rule.id", converted)?;
                    }
                }
                let _cond = { event.get("json.data.title").is_some_and(|v| v.is_string()) };
                if _cond {
                    if event.has_value("json.data.title") {
                        event.rename("json.data.title", "kolide.issues.title")?;
                    }
                }
                if event.has_value("json.data.issue_id") {
                    if let Some(val) = event.get("json.data.issue_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.issue_id".into(),
                                message,
                            }
                        })?;
                        event.set("kolide.issues.id", converted)?;
                    }
                }
                if event.has_value("json.data.check_id") {
                    if let Some(val) = event.get("json.data.check_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.check_id".into(),
                                message,
                            }
                        })?;
                        event.set("kolide.issues.check.id", converted)?;
                    }
                }
                if event.has_value("json.data.check.tags") {
                    event.rename("json.data.check.tags", "kolide.issues.check.tags")?;
                }
                let _cond = {
                    event.has_value("json.timestamp")
                        && event.get_str("event.action") == Some("issues.resolved")
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("kolide.issues.resolved_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.timestamp")
                        && event.get_str("event.action") == Some("issues.new")
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("kolide.issues.detected_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                // End nested pipeline: "webhook"
            }

            let _cond = { !event.has_value("_tmp.is_webhook") && !event.has_value("event.action") };
            if _cond {
                event.set("event.action", json!("issue"))?;
            }

            let _cond = {
                !event.has_value("_tmp.is_webhook")
                    && !event.has_value("kolide.issues.id")
                    && event.has_value("event.id")
            };
            if _cond {
                if let Some(v) = event.get("event.id").cloned() {
                    event.set("kolide.issues.id", v)?;
                }
            }

            let _cond = { event.has_value("json.detected_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.detected_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.detected_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.resolved_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.resolved_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.resolved_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("host.id") };
            if _cond {
                if event.has_value("json.device_information.identifier") {
                    event.rename("json.device_information.identifier", "host.id")?;
                }
            }

            let _cond = { !event.has_value("rule.id") };
            if _cond {
                if event.has_value("json.check_information.identifier") {
                    event.rename("json.check_information.identifier", "rule.id")?;
                }
            }

            let _cond = {
                !event.has_value("kolide.issues.title")
                    && event.get("json.title").is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("json.title") {
                    event.rename("json.title", "kolide.issues.title")?;
                }
            }

            let _cond = { event.has_value("kolide.issues.title") && !event.has_value("rule.name") };
            if _cond {
                event.set(
                    "rule.name",
                    json!(
                        event
                            .get("kolide.issues.title")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("rule.reference") && event.has_value("json.check_information.link")
            };
            if _cond {
                event.set(
                    "rule.reference",
                    json!(
                        event
                            .get("json.check_information.link")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("rule.reference")
                    && !event.has_value("json.check_information.link")
                    && event.has_value("rule.id")
            };
            if _cond {
                event.set(
                    "rule.reference",
                    json!(format!(
                        "https://api.kolide.com/checks/{}",
                        event
                            .get("rule.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if event.has_value("json.issue_key") {
                event.rename("json.issue_key", "kolide.issues.issue_key")?;
            }

            if event.has_value("json.issue_value") {
                event.rename("json.issue_value", "kolide.issues.issue_value")?;
            }

            if event.has_value("json.value") {
                event.rename("json.value", "kolide.issues.value")?;
            }

            let _cond = { event.has_value("json.exempted") };
            if _cond {
                if let Some(v) = event.get("json.exempted").cloned() {
                    event.set("kolide.issues.exempted", v)?;
                }
            }

            event.remove("json.exempted");

            if event.has_value("json.device_information.link") {
                event.rename(
                    "json.device_information.link",
                    "kolide.issues.device_information.link",
                )?;
            }

            if event.has_value("json.check_information.link") {
                event.rename(
                    "json.check_information.link",
                    "kolide.issues.check_information.link",
                )?;
            }

            let _cond = { event.has_value("json.detected_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.detected_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.detected_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.detected_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.resolved_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.resolved_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.resolved_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.resolved_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.blocks_device_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.blocks_device_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.blocks_device_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.blocks_device_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.last_rechecked_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.last_rechecked_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.last_rechecked_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_rechecked_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("kolide.issues.value.username")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.username")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event
                    .get("kolide.issues.value.path")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.path")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.path", v)?;
                }
            }

            let _cond = {
                event
                    .get("kolide.issues.value.md5")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.md5")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.md5", v)?;
                }
            }

            let _cond = {
                event
                    .get("kolide.issues.value.fingerprint_sha256")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.fingerprint_sha256")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.sha256", v)?;
                }
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                if let Some(v) = event.get("host.name").cloned() {
                    event.set("host.hostname", v)?;
                }
            }

            let _cond = { event.has_value("kolide.issues.title") };
            if _cond {
                if let Some(v) = event.get("kolide.issues.title").cloned() {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                event.has_value("kolide.issues.resolved_at")
                    || event.get_str("event.action") == Some("issues.resolved")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event
                    .get("kolide.issues.check.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.check.tags")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.category", v)?;
                }
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            // Begin nested pipeline: "extended-mappings"
            let _cond = {
                event
                    .get("kolide.issues.value.version")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.issues.detected_version", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.detected_version")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.detected_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("package.version", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.value.newest_version")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.newest_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.issues.expected_version", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.value.key_type")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.key_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.issues.ssh_key_type", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.value.device")
                    .is_some_and(|v| v.is_string())
                    && !event.has_value("file.device")
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.device")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.device", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.value.username")
                    .is_some_and(|v| v.is_string())
                    && !event.has_value("user.name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("kolide.issues.value.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            // End nested pipeline: "extended-mappings"

            // Begin nested pipeline: "categorize"
            let _cond = { event.has_value("event.action") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "788b3b02269ed8c4cd70d891723600ccd9e84c44031cedbc9aec4a9196529fc7",
                        r#"{"params":{"IssueLifecycle":{"subject":"event.action","table":"exact","columns":[["event.kind","kind"],["event.category","category"]],"defaults":[["event.kind","event"],["event.category",["configuration"]]],"domain":{"table":"check_category","key":"rule.id","target":"event.category"},"deadline":"kolide.issues.blocks_device_at","clock":"@timestamp","resolved":"kolide.issues.resolved_at","resolved_action":"issues.resolved","type_target":"event.type","changed":["change"],"created":["creation"],"flag_target":"_tmp.blocked","rewrite_when":"issue","rewrite_target":"event.action","on_resolved":"resolved","on_blocked":"blocked","on_pending":"will_be_blocked"}},"known":[{"GuardedCopy":{"statements":[{"If":{"test":[["Never"]],"then":[],"alt":[{"Literal":{"Set":{"path":"event.kind","value":{"Literal":"event"}}}},{"Literal":{"Set":{"path":"event.category","value":{"Literal":["configuration"]}}}}]}},{"If":{"test":[[{"Compare":{"path":"rule.id","wanted":"Null","negated":true,"lowered":false}}]],"then":[{"If":{"test":[["Never",{"Not":"Never"}]],"then":[],"alt":[]}}],"alt":[]}},{"If":{"test":[["Never",{"Compare":{"path":"@timestamp","wanted":"Null","negated":true,"lowered":false}}]],"then":[],"alt":[]}},{"If":{"test":[["Never"]],"then":[{"If":{"test":[["Never"]],"then":[{"Literal":{"Set":{"path":"event.action","value":{"Literal":"resolved"}}}}],"alt":[{"If":{"test":[["Never"]],"then":[{"Literal":{"Set":{"path":"event.action","value":{"Literal":"blocked"}}}}],"alt":[{"If":{"test":[["Never"]],"then":[{"Literal":{"Set":{"path":"event.action","value":{"Literal":"will_be_blocked"}}}}],"alt":[]}}]}}]}}],"alt":[]}}],"whole":false}}]}"#
                    ),
                    cached_params!(
                        "{\"exact\":{\"issue\":{\"kind\":\"event\",\"category\":[\"configuration\"]},\"issues.new\":{\"kind\":\"event\",\"category\":[\"configuration\"]},\"issues.resolved\":{\"kind\":\"event\",\"category\":[\"configuration\"]}},\"check_category\":{\"17\":\"malware\",\"18\":\"malware\",\"20\":\"malware\",\"75639\":\"malware\",\"41\":\"vulnerability\"}}"
                    ),
                )?;
            }
            // End nested pipeline: "categorize"

            let _cond = { event.has_value("event.id") && event.get_str("event.id") != Some("") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("_tmp.blocked") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("event.id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.issues.blocks_device_at") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.issues.detected_at") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.issues.resolved_at") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set(
                            "_id",
                            json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?),
                        )?;
                    }
                }
            }

            event.remove("json");
            event.remove("data");
            event.remove("_tmp");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            // The pipeline's script, emitted as the runtime's equivalent
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
