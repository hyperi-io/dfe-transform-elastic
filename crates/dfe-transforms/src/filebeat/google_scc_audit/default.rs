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

            event.set("event.kind", json!("event"))?;

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

            let _cond = {
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("google_scc.audit.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_audit_timestamp")?;
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
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
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

            if event.has_value("json.protoPayload.serviceName") {
                event.rename(
                    "json.protoPayload.serviceName",
                    "google_scc.audit.proto_payload.service_name",
                )?;
            }

            if let Some(v) = event
                .get("google_scc.audit.proto_payload.service_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.service.name", v)?;
            }

            let _cond = {
                event.get_str("json.protoPayload.requestMetadata.destinationAttributes.ip")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.protoPayload.requestMetadata.destinationAttributes.ip")
                    {
                        if let Some(val) =
                            event.get("json.protoPayload.requestMetadata.destinationAttributes.ip")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "json.protoPayload.requestMetadata.destinationAttributes.ip"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.proto_payload.request_metadata.destination_attributes.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_destination_attributes_ip",
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
                event.has_value(
                    "google_scc.audit.proto_payload.request_metadata.destination_attributes.ip",
                )
            };
            if _cond {
                event.append_unique("related.ip", json!(event.get("google_scc.audit.proto_payload.request_metadata.destination_attributes.ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event
                .get("google_scc.audit.proto_payload.request_metadata.destination_attributes.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = {
                event.get_str("json.protoPayload.requestMetadata.destinationAttributes.port")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("json.protoPayload.requestMetadata.destinationAttributes.port")
                    {
                        if let Some(val) = event
                            .get("json.protoPayload.requestMetadata.destinationAttributes.port")
                        {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.protoPayload.requestMetadata.destinationAttributes.port".into(),
                            message,
                        })?;
                            event.set("google_scc.audit.proto_payload.request_metadata.destination_attributes.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_destination_attributes_port",
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

            if let Some(v) = event
                .get("google_scc.audit.proto_payload.request_metadata.destination_attributes.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.protoPayload.methodName") {
                event.rename(
                    "json.protoPayload.methodName",
                    "google_scc.audit.proto_payload.method_name",
                )?;
            }

            if let Some(v) = event
                .get("google_scc.audit.proto_payload.method_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("json.insertId") {
                event.rename("json.insertId", "google_scc.audit.insert_id")?;
            }

            if let Some(v) = event
                .get("google_scc.audit.insert_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.get_str("json.protoPayload.requestMetadata.requestAttributes.size")
                    != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.protoPayload.requestMetadata.requestAttributes.size") {
                        if let Some(val) =
                            event.get("json.protoPayload.requestMetadata.requestAttributes.size")
                        {
                            let converted =
                                convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                            path: "json.protoPayload.requestMetadata.requestAttributes.size".into(),
                            message,
                        }
                                })?;
                            event.set("google_scc.audit.proto_payload.request_metadata.request_attributes.size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_request_attributes_size",
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

            if let Some(v) = event
                .get("google_scc.audit.proto_payload.request_metadata.request_attributes.size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.bytes", v)?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.id") {
                event.rename(
                    "json.protoPayload.requestMetadata.requestAttributes.id",
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.id",
                )?;
            }

            if let Some(v) = event
                .get("google_scc.audit.proto_payload.request_metadata.request_attributes.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.id", v)?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.method") {
                event.rename(
                    "json.protoPayload.requestMetadata.requestAttributes.method",
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.method",
                )?;
            }

            if let Some(v) = event
                .get("google_scc.audit.proto_payload.request_metadata.request_attributes.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            if event.has_value("json.labels") {
                event.rename("json.labels", "google_scc.audit.labels")?;
            }

            if let Some(v) = event
                .get("google_scc.audit.labels")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("labels", v)?;
            }

            if event.has_value("json.severity") {
                event.rename("json.severity", "google_scc.audit.severity.value")?;
            }

            if let Some(v) = event
                .get("google_scc.audit.severity.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.level", v)?;
            }

            let _cond = { event.has_value("google_scc.audit.severity.value") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.google_scc.audit.severity.put('code',params.get(ctx.google_scc.audit.severity.value));
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.google_scc.audit.severity.put('code',params.get(ctx.google_scc.audit.severity.value));"#
                        ),
                        cached_params!(
                            "{\"DEFAULT\":0,\"DEBUG\":100,\"INFO\":200,\"NOTICE\":300,\"WARNING\":400,\"ERROR\":500,\"CRITICAL\":600,\"ALERT\":700,\"EMERGENCY\":800}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "painless_audit_severity_value",
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

            if let Some(v) = event
                .get("google_scc.audit.severity.code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            let _cond = { event.get_str("json.sourceLocation.line") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.sourceLocation.line") {
                        if let Some(val) = event.get("json.sourceLocation.line") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.sourceLocation.line".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.source_location.line", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_location_line",
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

            if let Some(v) = event
                .get("google_scc.audit.source_location.line")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.origin.file.line", v)?;
            }

            if event.has_value("json.sourceLocation.file") {
                event.rename(
                    "json.sourceLocation.file",
                    "google_scc.audit.source_location.file",
                )?;
            }

            if let Some(v) = event
                .get("google_scc.audit.source_location.file")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.origin.file.name", v)?;
            }

            if event.has_value("json.sourceLocation.function") {
                event.rename(
                    "json.sourceLocation.function",
                    "google_scc.audit.source_location.function",
                )?;
            }

            if let Some(v) = event
                .get("google_scc.audit.source_location.function")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.origin.function", v)?;
            }

            let _cond = { event.get_str("json.protoPayload.requestMetadata.callerIp") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.protoPayload.requestMetadata.callerIp") {
                        if let Some(val) = event.get("json.protoPayload.requestMetadata.callerIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.protoPayload.requestMetadata.callerIp".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_scc.audit.proto_payload.request_metadata.caller.ip",
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
                        "convert_request_metadata_caller_ip",
                    )?;
                    event.rename(
                        "json.protoPayload.requestMetadata.callerIp",
                        "google_scc.audit.proto_payload.request_metadata.caller.ip_value",
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
                { event.has_value("google_scc.audit.proto_payload.request_metadata.caller.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("google_scc.audit.proto_payload.request_metadata.caller.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_scc.audit.proto_payload.request_metadata.caller.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if event.has_value("json.protoPayload.authenticationInfo.principalEmail") {
                event.rename(
                    "json.protoPayload.authenticationInfo.principalEmail",
                    "google_scc.audit.proto_payload.authentication_info.principal_email",
                )?;
            }

            if let Some(v) = event
                .get("google_scc.audit.proto_payload.authentication_info.principal_email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.email", v)?;
            }

            let _cond = {
                event
                    .has_value("google_scc.audit.proto_payload.authentication_info.principal_email")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get(
                                "google_scc.audit.proto_payload.authentication_info.principal_email"
                            )
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.callerSuppliedUserAgent") {
                event.rename(
                    "json.protoPayload.requestMetadata.callerSuppliedUserAgent",
                    "google_scc.audit.proto_payload.request_metadata.caller.supplied_user_agent",
                )?;
            }

            if event.has_value(
                "google_scc.audit.proto_payload.request_metadata.caller.supplied_user_agent",
            ) {
                if let Some(ua_str) = event.get_string(
                    "google_scc.audit.proto_payload.request_metadata.caller.supplied_user_agent",
                ) {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            let _cond = { event.get_str("json.httpRequest.cacheFillBytes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.httpRequest.cacheFillBytes") {
                        if let Some(val) = event.get("json.httpRequest.cacheFillBytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.httpRequest.cacheFillBytes".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("google_scc.audit.http_request.cache.fill_bytes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_http_request_cache_fill_bytes",
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

            let _cond = { event.get_str("json.httpRequest.cacheHit") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.httpRequest.cacheHit") {
                        if let Some(val) = event.get("json.httpRequest.cacheHit") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.httpRequest.cacheHit".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.http_request.cache.hit", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_http_request_cache_hit",
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

            let _cond = { event.get_str("json.httpRequest.cacheLookup") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.httpRequest.cacheLookup") {
                        if let Some(val) = event.get("json.httpRequest.cacheLookup") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.httpRequest.cacheLookup".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.http_request.cache.look_up", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_http_request_cache_look_up",
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
                { event.get_str("json.httpRequest.cacheValidatedWithOriginServer") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.httpRequest.cacheValidatedWithOriginServer") {
                        if let Some(val) =
                            event.get("json.httpRequest.cacheValidatedWithOriginServer")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.httpRequest.cacheValidatedWithOriginServer".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_scc.audit.http_request.cache.validated_with_origin_server",
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
                        "convert_http_request_cache_validated_with_origin_server",
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

            if event.has_value("json.httpRequest.latency") {
                event.rename(
                    "json.httpRequest.latency",
                    "google_scc.audit.http_request.latency",
                )?;
            }

            if event.has_value("json.httpRequest.protocol") {
                event.rename(
                    "json.httpRequest.protocol",
                    "google_scc.audit.http_request.protocol",
                )?;
            }

            if event.has_value("json.httpRequest.referer") {
                event.rename(
                    "json.httpRequest.referer",
                    "google_scc.audit.http_request.referer",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.httpRequest.remoteIp") {
                    if let Some(input) = event.get_string("json.httpRequest.remoteIp") {
                        // Grok pattern: ^%{IP:google_scc.audit.http_request.remote.ip}:%{NUMBER:google_scc.audit.http_request.remote.port:long}$
                        // Grok pattern: ^%{IP:google_scc.audit.http_request.remote.ip}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{IP:google_scc.audit.http_request.remote.ip}:%{NUMBER:google_scc.audit.http_request.remote.port:long}$"
                                ),
                                cached_grok!("^%{IP:google_scc.audit.http_request.remote.ip}$"),
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
                event.set("_ingest.on_failure_processor_tag", "grok_extract_remote_ip")?;
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

            let _cond = { event.has_value("google_scc.audit.http_request.remote.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("google_scc.audit.http_request.remote.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.httpRequest.requestMethod") {
                event.rename(
                    "json.httpRequest.requestMethod",
                    "google_scc.audit.http_request.request_method",
                )?;
            }

            let _cond = { event.get_str("json.httpRequest.requestSize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.httpRequest.requestSize") {
                        if let Some(val) = event.get("json.httpRequest.requestSize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.httpRequest.requestSize".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.http_request.request_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_http_request_size",
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

            if event.has_value("json.httpRequest.requestUrl") {
                event.rename(
                    "json.httpRequest.requestUrl",
                    "google_scc.audit.http_request.request_url",
                )?;
            }

            let _cond = { event.get_str("json.httpRequest.responseSize") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.httpRequest.responseSize") {
                        if let Some(val) = event.get("json.httpRequest.responseSize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.httpRequest.responseSize".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.http_request.response_size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_http_response_size",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.httpRequest.serverIp") {
                    if let Some(input) = event.get_string("json.httpRequest.serverIp") {
                        // Grok pattern: ^%{IP:google_scc.audit.http_request.server.ip}:%{NUMBER:google_scc.audit.http_request.server.port:long}$
                        // Grok pattern: ^%{IP:google_scc.audit.http_request.server.ip}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{IP:google_scc.audit.http_request.server.ip}:%{NUMBER:google_scc.audit.http_request.server.port:long}$"
                                ),
                                cached_grok!("^%{IP:google_scc.audit.http_request.server.ip}$"),
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
                event.set("_ingest.on_failure_processor_tag", "grok_extract_server_ip")?;
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

            let _cond = { event.has_value("google_scc.audit.http_request.server.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("google_scc.audit.http_request.server.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.httpRequest.status") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.httpRequest.status") {
                        if let Some(val) = event.get("json.httpRequest.status") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.httpRequest.status".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.http_request.status", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_http_request_status",
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

            if event.has_value("json.httpRequest.userAgent") {
                event.rename(
                    "json.httpRequest.userAgent",
                    "google_scc.audit.http_request.user_agent",
                )?;
            }

            if event.has_value("json.logName") {
                event.rename("json.logName", "google_scc.audit.log_name")?;
            }

            let _cond = { event.get_str("json.operation.first") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.operation.first") {
                        if let Some(val) = event.get("json.operation.first") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.operation.first".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.operation.first", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_operation_first",
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

            if event.has_value("json.operation.id") {
                event.rename("json.operation.id", "google_scc.audit.operation.id")?;
            }

            let _cond = { event.get_str("json.operation.last") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.operation.last") {
                        if let Some(val) = event.get("json.operation.last") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.operation.last".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.operation.last", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_operation_last")?;
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

            if event.has_value("json.operation.producer") {
                event.rename(
                    "json.operation.producer",
                    "google_scc.audit.operation.producer",
                )?;
            }

            if event.has_value("json.protoPayload.authenticationInfo.authoritySelector") {
                event.rename(
                    "json.protoPayload.authenticationInfo.authoritySelector",
                    "google_scc.audit.proto_payload.authentication_info.authority_selector",
                )?;
            }

            if event.has_value("json.protoPayload.authenticationInfo.principalSubject") {
                event.rename(
                    "json.protoPayload.authenticationInfo.principalSubject",
                    "google_scc.audit.proto_payload.authentication_info.principal_subject",
                )?;
            }

            let _cond = {
                event
                    .get("json.protoPayload.authenticationInfo.serviceAccountDelegationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.protoPayload.authenticationInfo.serviceAccountDelegationInfo",
                        |event| {
                            if event.has_value("_ingest._value.firstPartyPrincipal.principalEmail")
                            {
                                event.rename(
                                    "_ingest._value.firstPartyPrincipal.principalEmail",
                                    "_ingest._value.first_party_principal.email",
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
                    .get("json.protoPayload.authenticationInfo.serviceAccountDelegationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.protoPayload.authenticationInfo.serviceAccountDelegationInfo",
                        |event| {
                            if event.has_value("_ingest._value.firstPartyPrincipal.serviceMetadata")
                            {
                                event.rename(
                                    "_ingest._value.firstPartyPrincipal.serviceMetadata",
                                    "_ingest._value.first_party_principal.service_metadata",
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
                    .get("json.protoPayload.authenticationInfo.serviceAccountDelegationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.protoPayload.authenticationInfo.serviceAccountDelegationInfo",
                        |event| {
                            if event.has_value("_ingest._value.principalSubject") {
                                event.rename(
                                    "_ingest._value.principalSubject",
                                    "_ingest._value.principal_subject",
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
                    .get("json.protoPayload.authenticationInfo.serviceAccountDelegationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "json.protoPayload.authenticationInfo.serviceAccountDelegationInfo",
                        |event| {
                            if event
                                .has_value("_ingest._value.thirdPartyPrincipal.thirdPartyClaims")
                            {
                                event.rename(
                                    "_ingest._value.thirdPartyPrincipal.thirdPartyClaims",
                                    "_ingest._value.third_party_principal.claims",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.protoPayload.authenticationInfo.serviceAccountDelegationInfo")
            {
                event.rename("json.protoPayload.authenticationInfo.serviceAccountDelegationInfo", "google_scc.audit.proto_payload.authentication_info.service_account_delegation_info")?;
            }

            if event.has_value("json.protoPayload.authenticationInfo.serviceAccountKeyName") {
                event.rename(
                    "json.protoPayload.authenticationInfo.serviceAccountKeyName",
                    "google_scc.audit.proto_payload.authentication_info.service_account_key_name",
                )?;
            }

            if event.has_value("json.protoPayload.authenticationInfo.thirdPartyPrincipal") {
                event.rename(
                    "json.protoPayload.authenticationInfo.thirdPartyPrincipal",
                    "google_scc.audit.proto_payload.authentication_info.third_party_principal",
                )?;
            }

            let _cond = {
                event
                    .get("json.protoPayload.authorizationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.protoPayload.authorizationInfo", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.granted") {
                                if let Some(val) = event.get("_ingest._value.granted") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.granted".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.granted", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_authorization_info_granted",
                            )?;
                            event.remove("_ingest._value.granted");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.protoPayload.authorizationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.protoPayload.authorizationInfo").cloned();
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
                                    if let Some(date_str) = event.get_as_string(
                                        "_ingest._value.resourceAttributes.createTime",
                                    ) {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.resourceAttributes.create_time",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                            path: "_ingest._value.resourceAttributes.createTime".into(),
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
                                        "date_authorization_info_resource_attributes_create_time",
                                    )?;
                                    event.remove("_ingest._value.resourceAttributes.createTime");
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
                                "json.protoPayload.authorizationInfo",
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
                    .get("json.protoPayload.authorizationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.protoPayload.authorizationInfo").cloned();
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
                                    if let Some(date_str) = event.get_as_string(
                                        "_ingest._value.resourceAttributes.deleteTime",
                                    ) {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.resourceAttributes.delete_time",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                            path: "_ingest._value.resourceAttributes.deleteTime".into(),
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
                                        "date_authorization_info_resource_attributes_delete_time",
                                    )?;
                                    event.remove("_ingest._value.resourceAttributes.deleteTime");
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
                                "json.protoPayload.authorizationInfo",
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
                    .get("json.protoPayload.authorizationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("json.protoPayload.authorizationInfo").cloned();
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
                                    if let Some(date_str) = event.get_as_string(
                                        "_ingest._value.resourceAttributes.updateTime",
                                    ) {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.resourceAttributes.update_time",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                            path: "_ingest._value.resourceAttributes.updateTime".into(),
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
                                        "date_authorization_info_resource_attributes_update_time",
                                    )?;
                                    event.remove("_ingest._value.resourceAttributes.updateTime");
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
                                "json.protoPayload.authorizationInfo",
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
                    .get("json.protoPayload.authorizationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.protoPayload.authorizationInfo", |event| {
                        if event.has_value("_ingest._value.resourceAttributes.displayName") {
                            event.rename(
                                "_ingest._value.resourceAttributes.displayName",
                                "_ingest._value.resourceAttributes.display_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.protoPayload.authorizationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.protoPayload.authorizationInfo", |event| {
                        event.remove("_ingest._value.resourceAttributes.updateTime");
                        event.remove("_ingest._value.resourceAttributes.deleteTime");
                        event.remove("_ingest._value.resourceAttributes.createTime");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.protoPayload.authorizationInfo")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "json.protoPayload.authorizationInfo", |event| {
                        if event.has_value("_ingest._value.resourceAttributes") {
                            event.rename(
                                "_ingest._value.resourceAttributes",
                                "_ingest._value.resource_attributes",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.protoPayload.authorizationInfo") {
                event.rename(
                    "json.protoPayload.authorizationInfo",
                    "google_scc.audit.proto_payload.authorization_info",
                )?;
            }

            if event.has_value("json.protoPayload.metadata") {
                event.rename(
                    "json.protoPayload.metadata",
                    "google_scc.audit.proto_payload.metadata",
                )?;
            }

            let _cond = { event.get_str("json.protoPayload.numResponseItems") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.protoPayload.numResponseItems") {
                        if let Some(val) = event.get("json.protoPayload.numResponseItems") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.protoPayload.numResponseItems".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_scc.audit.proto_payload.num_response_items",
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
                        "convert_num_response_items",
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

            if event
                .has_value("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.payload")
            {
                event.rename("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.payload", "google_scc.audit.proto_payload.policy_violation_info.org_policy_violation_info.payload")?;
            }

            if event.has_value(
                "json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.resourceTags",
            ) {
                event.rename("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.resourceTags", "google_scc.audit.proto_payload.policy_violation_info.org_policy_violation_info.resource.tags")?;
            }

            if event.has_value(
                "json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.resourceType",
            ) {
                event.rename("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.resourceType", "google_scc.audit.proto_payload.policy_violation_info.org_policy_violation_info.resource.type")?;
            }

            if event.has_value("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo.checkedValue") {
                    event.rename("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo.checkedValue", "google_scc.audit.proto_payload.policy_violation_info.org_policy_violation_info.violation_info.checked_value")?;
                }

            if event.has_value("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo.constraint") {
                    event.rename("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo.constraint", "google_scc.audit.proto_payload.policy_violation_info.org_policy_violation_info.violation_info.constraint")?;
                }

            if event.has_value("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo.errorMessage") {
                    event.rename("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo.errorMessage", "google_scc.audit.proto_payload.policy_violation_info.org_policy_violation_info.violation_info.error_message")?;
                }

            if event.has_value("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo.policyType") {
                    event.rename("json.protoPayload.policyViolationInfo.orgPolicyViolationInfo.violationInfo.policyType", "google_scc.audit.proto_payload.policy_violation_info.org_policy_violation_info.violation_info.policy_type")?;
                }

            if event.has_value("json.protoPayload.request") {
                event.rename(
                    "json.protoPayload.request",
                    "google_scc.audit.proto_payload.request",
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.callerNetwork") {
                event.rename(
                    "json.protoPayload.requestMetadata.callerNetwork",
                    "google_scc.audit.proto_payload.request_metadata.caller.network",
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.destinationAttributes.labels") {
                event.rename(
                    "json.protoPayload.requestMetadata.destinationAttributes.labels",
                    "google_scc.audit.proto_payload.request_metadata.destination_attributes.labels",
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.destinationAttributes.principal")
            {
                event.rename("json.protoPayload.requestMetadata.destinationAttributes.principal", "google_scc.audit.proto_payload.request_metadata.destination_attributes.principal")?;
            }

            if event.has_value("json.protoPayload.requestMetadata.destinationAttributes.regionCode")
            {
                event.rename("json.protoPayload.requestMetadata.destinationAttributes.regionCode", "google_scc.audit.proto_payload.request_metadata.destination_attributes.region_code")?;
            }

            if event
                .has_value("json.protoPayload.requestMetadata.requestAttributes.auth.accessLevels")
            {
                event.rename("json.protoPayload.requestMetadata.requestAttributes.auth.accessLevels", "google_scc.audit.proto_payload.request_metadata.request_attributes.auth.access_levels")?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.auth.audiences")
            {
                event.rename("json.protoPayload.requestMetadata.requestAttributes.auth.audiences", "google_scc.audit.proto_payload.request_metadata.request_attributes.auth.audiences")?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.auth.claims") {
                event.rename("json.protoPayload.requestMetadata.requestAttributes.auth.claims", "google_scc.audit.proto_payload.request_metadata.request_attributes.auth.claims")?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.auth.presenter")
            {
                event.rename("json.protoPayload.requestMetadata.requestAttributes.auth.presenter", "google_scc.audit.proto_payload.request_metadata.request_attributes.auth.presenter")?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.auth.principal")
            {
                event.rename("json.protoPayload.requestMetadata.requestAttributes.auth.principal", "google_scc.audit.proto_payload.request_metadata.request_attributes.auth.principal")?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.headers") {
                event.rename(
                    "json.protoPayload.requestMetadata.requestAttributes.headers",
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.headers",
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.host") {
                event.rename(
                    "json.protoPayload.requestMetadata.requestAttributes.host",
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.host",
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.path") {
                event.rename(
                    "json.protoPayload.requestMetadata.requestAttributes.path",
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.path",
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.protocol") {
                event.rename(
                    "json.protoPayload.requestMetadata.requestAttributes.protocol",
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.protocol",
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.query") {
                event.rename(
                    "json.protoPayload.requestMetadata.requestAttributes.query",
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.query",
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.reason") {
                event.rename(
                    "json.protoPayload.requestMetadata.requestAttributes.reason",
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.reason",
                )?;
            }

            if event.has_value("json.protoPayload.requestMetadata.requestAttributes.scheme") {
                event.rename(
                    "json.protoPayload.requestMetadata.requestAttributes.scheme",
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.schema",
                )?;
            }

            let _cond = {
                event.has_value("json.protoPayload.requestMetadata.requestAttributes.time")
                    && event.get_str("json.protoPayload.requestMetadata.requestAttributes.time")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("json.protoPayload.requestMetadata.requestAttributes.time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("google_scc.audit.proto_payload.request_metadata.request_attributes.time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.protoPayload.requestMetadata.requestAttributes.time".into(),
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
                        "date_request_metadata_destination_attributes_time",
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

            if event.has_value("json.protoPayload.resourceLocation.currentLocations") {
                event.rename(
                    "json.protoPayload.resourceLocation.currentLocations",
                    "google_scc.audit.proto_payload.resource_location.current_locations",
                )?;
            }

            if event.has_value("json.protoPayload.resourceLocation.originalLocations") {
                event.rename(
                    "json.protoPayload.resourceLocation.originalLocations",
                    "google_scc.audit.proto_payload.resource_location.original_locations",
                )?;
            }

            if event.has_value("json.protoPayload.resourceName") {
                event.rename(
                    "json.protoPayload.resourceName",
                    "google_scc.audit.proto_payload.resource_name",
                )?;
            }

            if event.has_value("json.protoPayload.resourceOriginalState") {
                event.rename(
                    "json.protoPayload.resourceOriginalState",
                    "google_scc.audit.proto_payload.resource_original_state",
                )?;
            }

            if event.has_value("json.protoPayload.response") {
                event.rename(
                    "json.protoPayload.response",
                    "google_scc.audit.proto_payload.response",
                )?;
            }

            if event.has_value("json.protoPayload.serviceData") {
                event.rename(
                    "json.protoPayload.serviceData",
                    "google_scc.audit.proto_payload.service_data",
                )?;
            }

            let _cond = { event.get_str("json.protoPayload.status.code") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.protoPayload.status.code") {
                        if let Some(val) = event.get("json.protoPayload.status.code") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.protoPayload.status.code".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.proto_payload.status.code", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_status_code")?;
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

            if event.has_value("json.protoPayload.status.details") {
                event.rename(
                    "json.protoPayload.status.details",
                    "google_scc.audit.proto_payload.status.details",
                )?;
            }

            if event.has_value("json.protoPayload.status.message") {
                event.rename(
                    "json.protoPayload.status.message",
                    "google_scc.audit.proto_payload.status.message",
                )?;
            }

            if event.has_value("json.protoPayload.@type") {
                event.rename(
                    "json.protoPayload.@type",
                    "google_scc.audit.proto_payload.type",
                )?;
            }

            let _cond = {
                event.has_value("json.receiveTimestamp")
                    && event.get_str("json.receiveTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.receiveTimestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("google_scc.audit.receive_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.receiveTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_receive_timestamp")?;
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

            if event.has_value("json.resource.labels") {
                event.rename("json.resource.labels", "google_scc.audit.resource.labels")?;
            }

            if event.has_value("json.resource.type") {
                event.rename("json.resource.type", "google_scc.audit.resource.type")?;
            }

            if event.has_value("json.spanId") {
                event.rename("json.spanId", "google_scc.audit.span_id")?;
            }

            let _cond = { event.get_str("json.split.index") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.split.index") {
                        if let Some(val) = event.get("json.split.index") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.split.index".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.split.index", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_split_index")?;
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

            let _cond = { event.get_str("json.split.totalSplits") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.split.totalSplits") {
                        if let Some(val) = event.get("json.split.totalSplits") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.split.totalSplits".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.split.total_splits", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_split_total")?;
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

            if event.has_value("json.split.uid") {
                event.rename("json.split.uid", "google_scc.audit.split.uid")?;
            }

            if event.has_value("json.trace") {
                event.rename("json.trace", "google_scc.audit.trace")?;
            }

            let _cond = { event.get_str("json.traceSampled") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.traceSampled") {
                        if let Some(val) = event.get("json.traceSampled") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.traceSampled".into(),
                                    message,
                                }
                            })?;
                            event.set("google_scc.audit.trace_sampled", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_trace_sampled")?;
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

            event.remove("json");

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
                event.remove("google_scc.audit.timestamp");
                event.remove("google_scc.audit.proto_payload.service_name");
                event.remove(
                    "google_scc.audit.proto_payload.request_metadata.destination_attributes.ip",
                );
                event.remove(
                    "google_scc.audit.proto_payload.request_metadata.destination_attributes.port",
                );
                event.remove("google_scc.audit.proto_payload.method_name");
                event.remove("google_scc.audit.insert_id");
                event.remove("google_scc.audit.severity.code");
                event.remove(
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.size",
                );
                event.remove(
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.id",
                );
                event.remove(
                    "google_scc.audit.proto_payload.request_metadata.request_attributes.method",
                );
                event.remove("google_scc.audit.labels");
                event.remove("google_scc.audit.severity.value");
                event.remove("google_scc.audit.source_location.line");
                event.remove("google_scc.audit.source_location.file");
                event.remove("google_scc.audit.source_location.function");
                event.remove("google_scc.audit.proto_payload.request_metadata.caller.ip");
                event.remove("google_scc.audit.proto_payload.authentication_info.principal_email");
                event.remove(
                    "google_scc.audit.proto_payload.request_metadata.caller.supplied_user_agent",
                );
            }

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
