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
                // Painless script
                // Source: if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }"#
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

            let _cond = { !event.has_value("@timestamp") && event.has_value("json.created_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.created_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.created_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("@timestamp") && event.has_value("json.requested_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.requested_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.requested_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("@timestamp") };
            if _cond {
                if let Some(v) = event.get("_ingest.timestamp").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
                // Begin nested pipeline: "webhook"
                let _cond = {
                    !event.has_value("kolide.request.type")
                        && (event.get_str("json.event") == Some("requests.issue_exemption")
                            || event.get_str("json.event") == Some("request.issue_exemption"))
                };
                if _cond {
                    event.set("kolide.request.type", json!("exemption"))?;
                }
                let _cond = {
                    !event.has_value("kolide.request.type")
                        && (event.get_str("json.event") == Some("requests.registration")
                            || event.get_str("json.event") == Some("request.registration"))
                };
                if _cond {
                    event.set("kolide.request.type", json!("registration"))?;
                }
                let _cond = { !event.has_value("event.action") };
                if _cond {
                    if event.has_value("json.event") {
                        event.rename("json.event", "event.action")?;
                    }
                }
                let _cond = {
                    !event.has_value("message")
                        && event
                            .get("json.data.message")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.data.message")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("message", v)?;
                    }
                }
                let _cond =
                    { !event.has_value("kolide.request.state") && event.has_value("event.action") };
                if _cond {
                    event.set("kolide.request.state", json!("pending"))?;
                }
                let _cond = { !event.has_value("host.id") };
                if _cond {
                    if event.has_value("json.data.device_id") {
                        if let Some(val) = event.get("json.data.device_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.data.device_id".into(),
                                    message,
                                }
                            })?;
                            event.set("host.id", converted)?;
                        }
                    }
                }
                let _cond = { !event.has_value("host.id") };
                if _cond {
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
                }
                let _cond = {
                    !event.has_value("host.name")
                        && event
                            .get("json.data.device_name")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.data.device_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("host.name", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("host.name")
                        && event
                            .get("json.data.device.name")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.data.device.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("host.name", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.device.url")
                        && event
                            .get("json.data.device.url")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.data.device.url")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.device.url", v)?;
                    }
                }
                let _cond = { !event.has_value("user.id") };
                if _cond {
                    if event.has_value("json.data.requester.id") {
                        if let Some(val) = event.get("json.data.requester.id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.data.requester.id".into(),
                                    message,
                                }
                            })?;
                            event.set("user.id", converted)?;
                        }
                    }
                }
                let _cond = {
                    !event.has_value("user.email")
                        && event
                            .get("json.data.person_email")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.data.person_email")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("user.email")
                        && event
                            .get("json.data.requester.email")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.data.requester.email")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.requester.url")
                        && event
                            .get("json.data.requester.url")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.data.requester.url")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.requester.url", v)?;
                    }
                }
                let _cond = { !event.has_value("kolide.request.issues") };
                if _cond {
                    if event.has_value("json.data.issues") {
                        event.rename("json.data.issues", "kolide.request.issues")?;
                    }
                }
                // End nested pipeline: "webhook"
            }

            let _cond = { event.has_value("json._kolide_request_type") };
            if _cond {
                if let Some(v) = event.get("json._kolide_request_type").cloned() {
                    event.set("kolide.request.type", v)?;
                }
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                event.set("event.action", json!("request"))?;
            }

            let _cond = { event.has_value("json.id") };
            if _cond {
                if let Some(v) = event
                    .get("json.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.request.id", v)?;
                }
            }

            let _cond = { !event.has_value("event.id") };
            if _cond {
                if event.has_value("json.id") {
                    event.rename("json.id", "event.id")?;
                }
            }

            let _cond = { event.get_str("json._kolide_request_type") == Some("exemption") };
            if _cond {
                // Begin nested pipeline: "exemption"
                let _cond = {
                    !event.has_value("message")
                        && event
                            .get("json.requester_message")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.requester_message")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("message", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("message")
                        && event.get("json.message").is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.message")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("message", v)?;
                    }
                }
                let _cond = { event.get("json.status").is_some_and(|v| v.is_string()) };
                if _cond {
                    if let Some(v) = event
                        .get("json.status")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.state", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.state")
                        && event.get("json.state").is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.state")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.state", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.internal_message")
                        && event
                            .get("json.internal_explanation")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.internal_explanation")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.internal_message", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("event.reason")
                        && event
                            .get("json.denial_explanation")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.denial_explanation")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.reason", v)?;
                    }
                }
                let _cond = { !event.has_value("host.id") };
                if _cond {
                    if event.has_value("json.device_information.identifier") {
                        if let Some(val) = event.get("json.device_information.identifier") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.device_information.identifier".into(),
                                    message,
                                }
                            })?;
                            event.set("host.id", converted)?;
                        }
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.device.url")
                        && event
                            .get("json.device_information.link")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.device_information.link")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.device.url", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.device.url")
                        && event
                            .get("json.device_information.location")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.device_information.location")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.device.url", v)?;
                    }
                }
                let _cond = { !event.has_value("user.id") };
                if _cond {
                    if event.has_value("json.requester_information.identifier") {
                        if let Some(val) = event.get("json.requester_information.identifier") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.requester_information.identifier".into(),
                                    message,
                                }
                            })?;
                            event.set("user.id", converted)?;
                        }
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.requester.url")
                        && event
                            .get("json.requester_information.link")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.requester_information.link")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.requester.url", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.requester.url")
                        && event
                            .get("json.requester_information.location")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.requester_information.location")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.requester.url", v)?;
                    }
                }
                if event.has_value("json.issues") {
                    event.rename("json.issues", "kolide.request.issues")?;
                }
                let _cond = { event.has_value("json.requested_at") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.requested_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("kolide.request.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.requested_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.created_at")
                        && event.has_value("json.created_at")
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("kolide.request.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                // End nested pipeline: "exemption"
            }

            let _cond = { event.get_str("json._kolide_request_type") == Some("registration") };
            if _cond {
                // Begin nested pipeline: "registration"
                let _cond = {
                    !event.has_value("message")
                        && event
                            .get("json.requester_message")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.requester_message")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("message", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("message")
                        && event.get("json.message").is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.message")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("message", v)?;
                    }
                }
                let _cond = { event.get("json.status").is_some_and(|v| v.is_string()) };
                if _cond {
                    if let Some(v) = event
                        .get("json.status")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.state", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.state")
                        && event.get("json.state").is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.state")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.state", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.internal_message")
                        && event
                            .get("json.internal_denial_note")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.internal_denial_note")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.internal_message", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.internal_message")
                        && event
                            .get("json.internal_approval_note")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.internal_approval_note")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.internal_message", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("event.reason")
                        && event
                            .get("json.end_user_denial_note")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.end_user_denial_note")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.reason", v)?;
                    }
                }
                let _cond = { !event.has_value("host.id") };
                if _cond {
                    if event.has_value("json.device_information.identifier") {
                        if let Some(val) = event.get("json.device_information.identifier") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.device_information.identifier".into(),
                                    message,
                                }
                            })?;
                            event.set("host.id", converted)?;
                        }
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.device.url")
                        && event
                            .get("json.device_information.link")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.device_information.link")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.device.url", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.device.url")
                        && event
                            .get("json.device_information.location")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.device_information.location")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.device.url", v)?;
                    }
                }
                let _cond = { !event.has_value("user.id") };
                if _cond {
                    if event.has_value("json.requester_information.identifier") {
                        if let Some(val) = event.get("json.requester_information.identifier") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.requester_information.identifier".into(),
                                    message,
                                }
                            })?;
                            event.set("user.id", converted)?;
                        }
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.requester.url")
                        && event
                            .get("json.requester_information.link")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.requester_information.link")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.requester.url", v)?;
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.requester.url")
                        && event
                            .get("json.requester_information.location")
                            .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.requester_information.location")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("kolide.request.requester.url", v)?;
                    }
                }
                let _cond = { event.has_value("json.requested_at") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.requested_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("kolide.request.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.requested_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = {
                    !event.has_value("kolide.request.created_at")
                        && event.has_value("json.created_at")
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("kolide.request.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                // End nested pipeline: "registration"
            }

            let _cond = {
                !event.has_value("kolide.request.message")
                    && event.get("message").is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("message")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.request.message", v)?;
                }
            }

            let _cond = {
                !event.has_value("user.name")
                    && event.get("user.email").is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("user.email")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                if let Some(v) = event.get("host.name").cloned() {
                    event.set("host.hostname", v)?;
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

            let _cond = { event.has_value("host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("kolide.request.state") {
                map_strings(
                    event,
                    "kolide.request.state",
                    "kolide.request.state",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.get_str("event.action") == Some("request")
                    && event.has_value("kolide.request.type")
                    && event.has_value("kolide.request.state")
            };
            if _cond {
                event.set(
                    "event.action",
                    json!(format!(
                        "{}_request_{}",
                        event
                            .get("kolide.request.type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("kolide.request.state")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.get_str("kolide.request.state") == Some("approved") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("kolide.request.state") == Some("denied")
                    || event.get_str("kolide.request.state") == Some("rejected")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("kolide.request.state") == Some("pending")
                    || event.get_str("kolide.request.state") == Some("open")
                    || event.get_str("kolide.request.state") == Some("withdrawn")
            };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            // Begin nested pipeline: "categorize"
            let _cond = { event.has_value("event.action") };
            if _cond {
                // Painless script
                // Source: def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n} else {\n  ctx.event.kind = 'event';\n  ctx.event.category = ['configuration', 'iam'];\n  ctx.event.type = ['creation'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n} else {\n  ctx.event.kind = 'event';\n  ctx.event.category = ['configuration', 'iam'];\n  ctx.event.type = ['creation'];\n}"#
                    ),
                    cached_params!(
                        "{\"exact\":{\"request\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"creation\"]},\"requests.issue_exemption\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"creation\"]},\"requests.registration\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"creation\"]},\"exemption_request_open\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"creation\"]},\"exemption_request_withdrawn\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"change\"]},\"exemption_request_approved\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"change\"]},\"exemption_request_denied\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"change\"]},\"registration_request_pending\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"creation\"]},\"registration_request_approved\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"change\"]},\"registration_request_denied\":{\"kind\":\"event\",\"category\":[\"configuration\",\"iam\"],\"type\":[\"change\"]}}}"
                    ),
                )?;
            }
            // End nested pipeline: "categorize"

            let _cond = {
                event.has_value("kolide.request.id")
                    && event.get_str("kolide.request.id") != Some("")
            };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.reason") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.request.id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.request.internal_message") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.request.state") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.request.type") {
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

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
