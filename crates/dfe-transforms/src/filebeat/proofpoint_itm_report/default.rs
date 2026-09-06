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

            event.set("event.kind", json!("alert"))?;

            event.append("event.category", json!("session"))?;

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("Proofpoint"))?;

            event.set("observer.product", json!("ObserveIT"))?;

            let _cond =
                { event.has_value("json._time") && event.get_str("json._time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json._time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("proofpoint_itm.report._time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json._time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date__time")?;
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

            if event.has_value("json.accessedSiteName") {
                event.rename(
                    "json.accessedSiteName",
                    "proofpoint_itm.report.accessed.site_name",
                )?;
            }

            if event.has_value("json.accessedUrl") {
                event.rename("json.accessedUrl", "proofpoint_itm.report.accessed.url")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("proofpoint_itm.report.accessed.url") {
                    if !uri_parts(
                        event,
                        "proofpoint_itm.report.accessed.url",
                        "url",
                        true,
                        false,
                    )? && event
                        .get_str("proofpoint_itm.report.accessed.url")
                        .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "proofpoint_itm.report.accessed.url".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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
                .get("proofpoint_itm.report.accessed.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            if event.has_value("json.applicationName") {
                event.rename(
                    "json.applicationName",
                    "proofpoint_itm.report.application_name",
                )?;
            }

            if event.has_value("json.collectorId") {
                event.rename("json.collectorId", "proofpoint_itm.report.collector.id")?;
            }

            if event.has_value("json.collectorUrl") {
                event.rename("json.collectorUrl", "proofpoint_itm.report.collector.url")?;
            }

            if event.has_value("json.commandParams") {
                event.rename("json.commandParams", "proofpoint_itm.report.command.params")?;
            }

            let _cond = { event.has_value("proofpoint_itm.report.command.params") };
            if _cond {
                event.append_unique(
                    "process.args",
                    json!(
                        event
                            .get("proofpoint_itm.report.command.params")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.command") {
                event.rename("json.command", "proofpoint_itm.report.command.value")?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.command.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            let _cond = {
                event.has_value("json.createdAt") && event.get_str("json.createdAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("proofpoint_itm.report.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_createdAt")?;
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
                .get("proofpoint_itm.report.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has_value("json.databaseName") {
                event.rename("json.databaseName", "proofpoint_itm.report.database_name")?;
            }

            if event.has_value("json.details") {
                event.rename("json.details", "proofpoint_itm.report.details.name")?;
            }

            if event.has_value("json.detailsUrl") {
                event.rename("json.detailsUrl", "proofpoint_itm.report.details.url")?;
            }

            if event.has_value("json.domainName") {
                event.rename("json.domainName", "proofpoint_itm.report.domain_name")?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.domain_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            if event.has_value("json.endpointId") {
                event.rename("json.endpointId", "proofpoint_itm.report.endpoint.id")?;
            }

            if event.has_value("json.endpointName") {
                event.rename("json.endpointName", "proofpoint_itm.report.endpoint.name")?;
            }

            if event.has_value("json.eventPlaybackUrl") {
                event.rename(
                    "json.eventPlaybackUrl",
                    "proofpoint_itm.report.event_playback_url",
                )?;
            }

            if event.has_value("json.host") {
                event.rename("json.host", "proofpoint_itm.report.host")?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "proofpoint_itm.report.id")?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.loginName") {
                event.rename("json.loginName", "proofpoint_itm.report.login_name")?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.login_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("proofpoint_itm.report.login_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_itm.report.login_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.observedAt") && event.get_str("json.observedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.observedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("proofpoint_itm.report.observed_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.observedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_observedAt")?;
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

            if event.has_value("json.operationKind") {
                event.rename("json.operationKind", "proofpoint_itm.report.operation_kind")?;
            }

            if event.has_value("json.originFileName") {
                event.rename(
                    "json.originFileName",
                    "proofpoint_itm.report.origin.file_name",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.origin.file_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if event.has_value("json.originSiteName") {
                event.rename(
                    "json.originSiteName",
                    "proofpoint_itm.report.origin.site_name",
                )?;
            }

            if event.has_value("json.os") {
                event.rename("json.os", "proofpoint_itm.report.os")?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.type", v)?;
            }

            if event.has_value("host.os.type") {
                map_strings(event, "host.os.type", "host.os.type", str::to_lowercase)?;
            }

            if event.has_value("json.playbackUrl") {
                event.rename("json.playbackUrl", "proofpoint_itm.report.playback_url")?;
            }

            if event.has_value("json.processExecutable") {
                event.rename(
                    "json.processExecutable",
                    "proofpoint_itm.report.process_executable",
                )?;
            }

            let _cond = { event.get_str("json.remoteAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.remoteAddress") {
                        if let Some(val) = event.get("json.remoteAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.remoteAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("proofpoint_itm.report.remote.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_remoteAddress_to_ip",
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

            let _cond = { event.has_value("proofpoint_itm.report.remote.address") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("proofpoint_itm.report.remote.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("proofpoint_itm.report.remote.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proofpoint_itm.report.remote.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.remoteHostName") {
                event.rename(
                    "json.remoteHostName",
                    "proofpoint_itm.report.remote.host_name",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.remote.host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("proofpoint_itm.report.remote.host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("proofpoint_itm.report.remote.host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.risingValue") && event.get_str("json.risingValue") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.risingValue") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("proofpoint_itm.report.rising_value", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.risingValue".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_risingValue")?;
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
                .get("proofpoint_itm.report.rising_value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.ruleCategoryName") {
                event.rename(
                    "json.ruleCategoryName",
                    "proofpoint_itm.report.rule.category_name",
                )?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.rule.category_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.category", v)?;
            }

            if event.has_value("json.ruleDesc") {
                event.rename("json.ruleDesc", "proofpoint_itm.report.rule.desc")?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.rule.desc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
            }

            if event.has_value("json.ruleName") {
                event.rename("json.ruleName", "proofpoint_itm.report.rule.name")?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.rule.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("proofpoint_itm.report.friendly_name", v)?;
            }

            if let Some(v) = event
                .get("proofpoint_itm.report.rule.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.secondaryDomainName") {
                event.rename(
                    "json.secondaryDomainName",
                    "proofpoint_itm.report.secondary.domain_name",
                )?;
            }

            if event.has_value("json.secondaryLoginName") {
                event.rename(
                    "json.secondaryLoginName",
                    "proofpoint_itm.report.secondary.login_name",
                )?;
            }

            let _cond = { event.has_value("proofpoint_itm.report.secondary.login_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_itm.report.secondary.login_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.sessionId") {
                event.rename("json.sessionId", "proofpoint_itm.report.session.id")?;
            }

            if event.has_value("json.sessionUrl") {
                event.rename("json.sessionUrl", "proofpoint_itm.report.session.url")?;
            }

            if event.has_value("json.severity") {
                event.rename("json.severity", "proofpoint_itm.report.severity")?;
            }

            if event.has_value("json.sqlCommand") {
                event.rename("json.sqlCommand", "proofpoint_itm.report.sql.command")?;
            }

            if event.has_value("json.sqlUserName") {
                event.rename("json.sqlUserName", "proofpoint_itm.report.sql.user_name")?;
            }

            let _cond = { event.has_value("proofpoint_itm.report.sql.user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("proofpoint_itm.report.sql.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.targetFileName") {
                event.rename(
                    "json.targetFileName",
                    "proofpoint_itm.report.target.file_name",
                )?;
            }

            if event.has_value("json.targetSiteName") {
                event.rename(
                    "json.targetSiteName",
                    "proofpoint_itm.report.target.site_name",
                )?;
            }

            let _cond = { event.get_str("json.timezoneOffset") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.timezoneOffset") {
                        if let Some(val) = event.get("json.timezoneOffset") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.timezoneOffset".into(),
                                    message,
                                }
                            })?;
                            event.set("proofpoint_itm.report.timezone_offset", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_timezoneOffset_to_long",
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

            if event.has_value("json.userActivityEventId") {
                if let Some(val) = event.get("json.userActivityEventId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.userActivityEventId".into(),
                            message,
                        }
                    })?;
                    event.set("proofpoint_itm.report.user_activity.event_id", converted)?;
                }
            }

            let _cond = {
                event.has_value("json.userActivityObservedAt")
                    && event.get_str("json.userActivityObservedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.userActivityObservedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("proofpoint_itm.report.user_activity.observed_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.userActivityObservedAt".into(),
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
                        "date_userActivityObservedAt",
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

            if event.has_value("json.windowTitle") {
                event.rename("json.windowTitle", "proofpoint_itm.report.window_title")?;
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
                event.remove("proofpoint_itm.report.accessed.url");
                event.remove("proofpoint_itm.report.command.params");
                event.remove("proofpoint_itm.report.command.value");
                event.remove("proofpoint_itm.report.created_at");
                event.remove("proofpoint_itm.report.domain_name");
                event.remove("proofpoint_itm.report.id");
                event.remove("proofpoint_itm.report.login_name");
                event.remove("proofpoint_itm.report.origin.file_name");
                event.remove("proofpoint_itm.report.os");
                event.remove("proofpoint_itm.report.remote.address");
                event.remove("proofpoint_itm.report.remote.host_name");
                event.remove("proofpoint_itm.report.rising_value");
                event.remove("proofpoint_itm.report.rule.category_name");
                event.remove("proofpoint_itm.report.rule.desc");
                event.remove("proofpoint_itm.report.rule.name");
            }

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
