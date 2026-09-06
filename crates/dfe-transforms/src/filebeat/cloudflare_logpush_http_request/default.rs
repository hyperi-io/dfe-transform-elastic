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
            event.set("ecs.version", json!("9.3.0"))?;

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

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.kind", json!("event"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = {
                event.get_bool("_conf.enable_deduplication") == Some(false)
                    && event.get_str("input.type") == Some("aws-s3")
            };
            if _cond {
                event.remove("_id");
            }

            let _cond = {
                event.has_value("json.EdgeStartTimestamp")
                    && event.get_str("json.EdgeStartTimestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.EdgeStartTimestamp") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.EdgeStartTimestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.EdgeStartTimestamp", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.EdgeEndTimestamp")
                    && event.get_str("json.EdgeEndTimestamp") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.EdgeEndTimestamp") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.EdgeEndTimestamp".into(),
                                message,
                            }
                        })?;
                        event.set("json.EdgeEndTimestamp", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("json.Datetime") && event.get_str("json.Datetime") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.Datetime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Datetime".into(),
                                message,
                            }
                        })?;
                        event.set("json.Datetime", converted)?;
                    }
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def convertToMillis(long timestamp) {\n  if (timestamp > (long)(1e18)) {\n    return timestamp/(long)(1e6)\n  } else if (timestamp < (long)(1e10))  {\n    return timestamp*(long)(1e3)\n  }\n  return timestamp\n}\nif (ctx.json?.EdgeStartTimestamp != null && ctx.json.EdgeStartTimestamp instanceof Number) {\n  ctx.json.EdgeStartTimestamp = convertToMillis(ctx.json.EdgeStartTimestamp);\n}\nif (ctx.json?.EdgeEndTimestamp != null && ctx.json.EdgeEndTimestamp instanceof Number) {\n  ctx.json.EdgeEndTimestamp = convertToMillis(ctx.json.EdgeEndTimestamp);\n}\nif (ctx.json?.Datetime != null && ctx.json.Datetime instanceof Number) {\n  ctx.json.Datetime = convertToMillis(ctx.json.Datetime);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def convertToMillis(long timestamp) {\n  if (timestamp > (long)(1e18)) {\n    return timestamp/(long)(1e6)\n  } else if (timestamp < (long)(1e10))  {\n    return timestamp*(long)(1e3)\n  }\n  return timestamp\n}\nif (ctx.json?.EdgeStartTimestamp != null && ctx.json.EdgeStartTimestamp instanceof Number) {\n  ctx.json.EdgeStartTimestamp = convertToMillis(ctx.json.EdgeStartTimestamp);\n}\nif (ctx.json?.EdgeEndTimestamp != null && ctx.json.EdgeEndTimestamp instanceof Number) {\n  ctx.json.EdgeEndTimestamp = convertToMillis(ctx.json.EdgeEndTimestamp);\n}\nif (ctx.json?.Datetime != null && ctx.json.Datetime instanceof Number) {\n  ctx.json.Datetime = convertToMillis(ctx.json.Datetime);\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "painless_edge_start_timestamp_to_milli",
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
                event.has_value("json.EdgeStartTimestamp")
                    && event.get_str("json.EdgeStartTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.EdgeStartTimestamp") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601", "yyyy-MM-dd'T'HH:mm:ssZ"],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("cloudflare_logpush.http_request.edge.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.EdgeStartTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_EdgeStartTimestamp_to_cloudflare_logpush_http_request_edge_start_time_6e9e517a")?;
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
                event.has_value("json.EdgeEndTimestamp")
                    && event.get_str("json.EdgeEndTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.EdgeEndTimestamp") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601", "yyyy-MM-dd'T'HH:mm:ssZ"],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("cloudflare_logpush.http_request.edge.end_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.EdgeEndTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_EdgeEndTimestamp_to_cloudflare_logpush_http_request_edge_end_time_b977f372")?;
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

            let _cond =
                { event.has_value("json.Datetime") && event.get_str("json.Datetime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Datetime") {
                        match parse_date_out(
                            &date_str,
                            &["UNIX_MS", "ISO8601", "yyyy-MM-dd'T'HH:mm:ssZ"],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("cloudflare_logpush.http_request.datetime", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.Datetime".into(),
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
                        "date_json_Datetime_to_cloudflare_logpush_http_request_datetime_9a7be6f2",
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
                .get("cloudflare_logpush.http_request.datetime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.edge.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("json.OriginResponseHTTPExpires")
                    && event.get_str("json.OriginResponseHTTPExpires") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.OriginResponseHTTPExpires") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "EEE, dd MMM yyyy HH:mm:ss zzz",
                                "uuuu-MM-dd'T'HH:mm:ssX",
                                "uuuu-MM-dd'T'HH:mm:ss.SSSX",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "UNIX_MS",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set(
                                "cloudflare_logpush.http_request.origin.response.http.expires",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.OriginResponseHTTPExpires".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_OriginResponseHTTPExpires_to_cloudflare_logpush_http_request_origin_response_http_expires_5fcc3d5b")?;
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
                event.has_value("json.OriginResponseHTTPLastModified")
                    && event.get_str("json.OriginResponseHTTPLastModified") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.OriginResponseHTTPLastModified")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "EEE, dd MMM yyyy HH:mm:ss zzz", "uuuu-MM-dd'T'HH:mm:ssX", "uuuu-MM-dd'T'HH:mm:ss.SSSX", "yyyy-MM-dd'T'HH:mm:ssZ", "yyyy-MM-dd'T'HH:mm:ss.SSSZ", "UNIX_MS"], Some("UTC"), None) {
                        Some(parsed) => event.set("cloudflare_logpush.http_request.origin.response.http.last_modified", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.OriginResponseHTTPLastModified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_OriginResponseHTTPLastModified_to_cloudflare_logpush_http_request_origin_response_http_last_modified_bd80d19e")?;
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

            let _cond =
                { event.has_value("json.OriginIP") && event.get_str("json.OriginIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginIP") {
                        if let Some(val) = event.get("json.OriginIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginIP".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.http_request.origin.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginIP_to_cloudflare_logpush_http_request_origin_ip_191f66a5")?;
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
                .get("cloudflare_logpush.http_request.origin.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if event.has_value("json.ClientRequestMethod") {
                event.rename(
                    "json.ClientRequestMethod",
                    "cloudflare_logpush.http_request.client.request.method",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.client.request.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            if event.has_value("json.EdgeResponseContentType") {
                event.rename(
                    "json.EdgeResponseContentType",
                    "cloudflare_logpush.http_request.edge.response.content_type",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.edge.response.content_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.mime_type", v)?;
            }

            let _cond = { event.get_str("json.EdgeResponseStatus") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.EdgeResponseStatus") {
                        if let Some(val) = event.get("json.EdgeResponseStatus") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.EdgeResponseStatus".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.edge.response.status",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_EdgeResponseStatus_to_cloudflare_logpush_http_request_edge_response_status_168bfabf")?;
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
                .get("cloudflare_logpush.http_request.edge.response.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            let _cond = { event.get_str("json.ClientASN") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientASN") {
                        if let Some(val) = event.get("json.ClientASN") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientASN".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.http_request.client.asn", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientASN_to_cloudflare_logpush_http_request_client_asn_82a23ec8")?;
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
                .get("cloudflare_logpush.http_request.client.asn")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.as.number", v)?;
            }

            if event.has_value("json.ClientCountry") {
                event.rename(
                    "json.ClientCountry",
                    "cloudflare_logpush.http_request.client.country",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.client.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_iso_code", v)?;
            }

            let _cond =
                { event.has_value("json.ClientIP") && event.get_str("json.ClientIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientIP") {
                        if let Some(val) = event.get("json.ClientIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientIP".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.http_request.client.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientIP_to_cloudflare_logpush_http_request_client_ip_3d9346d4")?;
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
                .get("cloudflare_logpush.http_request.client.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if event.has_value("json.BotDetectionIDs") {
                if let Some(val) = event.get("json.BotDetectionIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.BotDetectionIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.http_request.bot.detection_ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.BotDetectionTags") {
                event.rename(
                    "json.BotDetectionTags",
                    "cloudflare_logpush.http_request.bot.detection_tags",
                )?;
            }

            let _cond = { event.get_str("json.BotScore") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.BotScore") {
                        if let Some(val) = event.get("json.BotScore") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.BotScore".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.bot.score.value",
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
                        "convert_bot_score_value",
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

            if event.has_value("json.BotScoreSrc") {
                event.rename(
                    "json.BotScoreSrc",
                    "cloudflare_logpush.http_request.bot.score.src",
                )?;
            }

            if event.has_value("json.BotTags") {
                event.rename("json.BotTags", "cloudflare_logpush.http_request.bot.tag")?;
            }

            if event.has_value("json.CacheCacheStatus") {
                event.rename(
                    "json.CacheCacheStatus",
                    "cloudflare_logpush.http_request.cache.status",
                )?;
            }

            let _cond = {
                event.has_value("json.CacheReserveUsed")
                    && event.get_str("json.CacheReserveUsed") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.CacheReserveUsed") {
                        if let Some(val) = event.get("json.CacheReserveUsed") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.CacheReserveUsed".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.cache.reserve_used",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_CacheReserveUsed_to_cloudflare_logpush_http_request_cache_reserve_used_f36316b4")?;
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
                event.has_value("json.CacheResponseBytes")
                    && event.get_str("json.CacheResponseBytes") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.CacheResponseBytes") {
                        if let Some(val) = event.get("json.CacheResponseBytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.CacheResponseBytes".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.cache.response.bytes",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_CacheResponseBytes_to_cloudflare_logpush_http_request_cache_response_bytes_af455bfb")?;
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

            let _cond = { event.get_str("json.CacheResponseStatus") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.CacheResponseStatus") {
                        if let Some(val) = event.get("json.CacheResponseStatus") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.CacheResponseStatus".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.cache.response.status",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_CacheResponseStatus_to_cloudflare_logpush_http_request_cache_response_status_18a55e44")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.CacheTieredFill") {
                    if let Some(val) = event.get("json.CacheTieredFill") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.CacheTieredFill".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.http_request.cache.tiered_fill",
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
                    "convert_cachetieredfill_to_boolean",
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

            if event.has_value("json.ClientCity") {
                event.rename(
                    "json.ClientCity",
                    "cloudflare_logpush.http_request.client.city",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.client.city")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.city_name", v)?;
            }

            if event.has_value("json.ClientDeviceType") {
                event.rename(
                    "json.ClientDeviceType",
                    "cloudflare_logpush.http_request.client.device.type",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.client.device.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.type", v)?;
            }

            if event.has_value("json.ClientIPClass") {
                event.rename(
                    "json.ClientIPClass",
                    "cloudflare_logpush.http_request.client.ip_class",
                )?;
            }

            if event.has_value("json.ClientLatitude") {
                event.rename(
                    "json.ClientLatitude",
                    "cloudflare_logpush.http_request.client.latitude",
                )?;
            }

            if event.has_value("json.ClientLongitude") {
                event.rename(
                    "json.ClientLongitude",
                    "cloudflare_logpush.http_request.client.longitude",
                )?;
            }

            if event.has_value("json.ClientMTLSAuthCertFingerprint") {
                event.rename(
                    "json.ClientMTLSAuthCertFingerprint",
                    "cloudflare_logpush.http_request.client.mtls.auth.fingerprint",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.client.mtls.auth.fingerprint")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.hash.sha256", v)?;
            }

            if event.has_value("tls.client.hash.sha256") {
                map_strings(
                    event,
                    "tls.client.hash.sha256",
                    "tls.client.hash.sha256",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("json.ClientMTLSAuthStatus") {
                event.rename(
                    "json.ClientMTLSAuthStatus",
                    "cloudflare_logpush.http_request.client.mtls.auth.status",
                )?;
            }

            if event.has_value("json.ClientRegionCode") {
                event.rename(
                    "json.ClientRegionCode",
                    "cloudflare_logpush.http_request.client.region_code",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.client.region_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.region_iso_code", v)?;
            }

            let _cond = { event.get_str("json.ClientRequestBytes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientRequestBytes") {
                        if let Some(val) = event.get("json.ClientRequestBytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientRequestBytes".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.client.request.bytes",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientRequestBytes_to_cloudflare_logpush_http_request_client_request_bytes_7aa1b76a")?;
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
                .get("cloudflare_logpush.http_request.client.request.bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.bytes", v)?;
            }

            if event.has_value("json.ContentScanObjResults") {
                event.rename(
                    "json.ContentScanObjResults",
                    "cloudflare_logpush.http_request.content_scan.results",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ContentScanObjSizes") {
                    if let Some(val) = event.get("json.ContentScanObjSizes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ContentScanObjSizes".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.http_request.content_scan.sizes",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_ContentScanObjSizes_to_cloudflare_logpush_http_request_content_scan_sizes_8c19d2c4")?;
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

            if event.has_value("json.ContentScanObjTypes") {
                event.rename(
                    "json.ContentScanObjTypes",
                    "cloudflare_logpush.http_request.content_scan.types",
                )?;
            }

            if event.has_value("json.LeakedCredentialCheckResult") {
                event.rename(
                    "json.LeakedCredentialCheckResult",
                    "cloudflare_logpush.http_request.leaked_credential_check",
                )?;
            }

            let _cond = { !event.has_value("url.scheme") };
            if _cond {
                if let Some(v) = event
                    .get("json.ClientRequestScheme")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.scheme", v)?;
                }
            }

            let _cond =
                { !event.has_value("url.scheme") && event.has_value("json.ClientSSLProtocol") };
            if _cond {
                let v = json!("https");
                if !painless_is_empty_value(&v) {
                    event.set("url.scheme", v)?;
                }
            }

            let _cond = { !event.has_value("url.scheme") };
            if _cond {
                let v = json!("http");
                if !painless_is_empty_value(&v) {
                    event.set("url.scheme", v)?;
                }
            }

            let _cond = { !event.has_value("url.domain") };
            if _cond {
                if let Some(v) = event
                    .get("json.ClientRequestHost")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.domain", v)?;
                }
            }

            let _cond = { !event.has_value("url.path") };
            if _cond {
                if let Some(v) = event
                    .get("json.ClientRequestPath")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.path", v)?;
                }
            }

            let _cond = { !event.has_value("url.query") };
            if _cond {
                if let Some(v) = event
                    .get("json.ClientRequestQuery")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.query", v)?;
                }
            }

            let _cond = {
                event.has_value("json.ClientRequestURI")
                    && event.has_value("url.scheme")
                    && event.has_value("url.domain")
                    && event
                        .get_str("json.ClientRequestURI")
                        .is_some_and(|s| s.starts_with("/"))
            };
            if _cond {
                let v = json!(format!(
                    "{}://{}{}",
                    event
                        .get("url.scheme")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("url.domain")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("json.ClientRequestURI")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("tmp.constructed_uri", v)?;
                }
            }

            let _cond = {
                event.has_value("json.ClientRequestURI")
                    && !(event
                        .get_str("json.ClientRequestURI")
                        .is_some_and(|s| s.starts_with("/")))
            };
            if _cond {
                if let Some(v) = event
                    .get("json.ClientRequestURI")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("tmp.constructed_uri", v)?;
                }
            }

            let _cond = { event.has_value("tmp.constructed_uri") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "tmp.constructed_uri", "url", true, false)?;
                    Ok(())
                })();
            }

            event.remove("tmp.constructed_uri");

            if event.has_value("json.ClientRequestHost") {
                event.rename(
                    "json.ClientRequestHost",
                    "cloudflare_logpush.http_request.client.request.host",
                )?;
            }

            if event.has_value("json.ClientRequestPath") {
                event.rename(
                    "json.ClientRequestPath",
                    "cloudflare_logpush.http_request.client.request.path",
                )?;
            }

            if event.has_value("json.ClientRequestProtocol") {
                event.rename(
                    "json.ClientRequestProtocol",
                    "cloudflare_logpush.http_request.client.request.protocol",
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.http_request.client.request.protocol")
                    && event.get_str("cloudflare_logpush.http_request.client.request.protocol")
                        != Some("none")
                    && event.get_str("cloudflare_logpush.http_request.client.request.protocol")
                        != Some("unknown")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.http_request.client.request.protocol") {
                        if let Some(input) = event
                            .get_string("cloudflare_logpush.http_request.client.request.protocol")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("/") else {
                                    break 'dissect false;
                                };
                                captured.push(("network.protocol", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("/") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("http.version", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "cloudflare_logpush.http_request.client.request.protocol"
                                        .into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_client_request_protocol",
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

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.ClientRequestReferer") {
                event.rename(
                    "json.ClientRequestReferer",
                    "cloudflare_logpush.http_request.client.request.referer",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.client.request.referer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.referrer", v)?;
            }

            if event.has_value("json.ClientRequestScheme") {
                event.rename(
                    "json.ClientRequestScheme",
                    "cloudflare_logpush.http_request.client.request.scheme",
                )?;
            }

            if event.has_value("json.ClientRequestSource") {
                event.rename(
                    "json.ClientRequestSource",
                    "cloudflare_logpush.http_request.client.request.source",
                )?;
            }

            if event.has_value("json.ClientRequestURI") {
                event.rename(
                    "json.ClientRequestURI",
                    "cloudflare_logpush.http_request.client.request.uri",
                )?;
            }

            let _cond = { event.get_str("json.ClientRequestUserAgent") != Some("") };
            if _cond {
                if event.has_value("json.ClientRequestUserAgent") {
                    if let Some(ua_str) = event.get_string("json.ClientRequestUserAgent") {
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
            }

            if event.has_value("json.ClientRequestUserAgent") {
                event.rename(
                    "json.ClientRequestUserAgent",
                    "cloudflare_logpush.http_request.client.request.user.agent",
                )?;
            }

            if event.has_value("json.ClientSSLCipher") {
                event.rename(
                    "json.ClientSSLCipher",
                    "cloudflare_logpush.http_request.client.ssl.cipher",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.client.ssl.cipher")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.cipher", v)?;
            }

            if event.has_value("json.ClientSSLProtocol") {
                event.rename(
                    "json.ClientSSLProtocol",
                    "cloudflare_logpush.http_request.client.ssl.protocol",
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.http_request.client.ssl.protocol")
                    && event.get_str("cloudflare_logpush.http_request.client.ssl.protocol")
                        != Some("none")
                    && event.get_str("cloudflare_logpush.http_request.client.ssl.protocol")
                        != Some("unknown")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.http_request.client.ssl.protocol") {
                        if let Some(input) =
                            event.get_string("cloudflare_logpush.http_request.client.ssl.protocol")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("v") else {
                                    break 'dissect false;
                                };
                                captured.push(("tls.version_protocol", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("v") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("tls.version", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "cloudflare_logpush.http_request.client.ssl.protocol"
                                        .into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_client_ssl_protocol",
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

            if event.has_value("tls.version_protocol") {
                map_strings(
                    event,
                    "tls.version_protocol",
                    "tls.version_protocol",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("json.ClientSrcPort") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientSrcPort") {
                        if let Some(val) = event.get("json.ClientSrcPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientSrcPort".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.client.src.port",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientSrcPort_to_cloudflare_logpush_http_request_client_src_port_1a19cb99")?;
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
                .get("cloudflare_logpush.http_request.client.src.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            let _cond = { event.get_str("json.ClientTCPRTTMs") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientTCPRTTMs") {
                        if let Some(val) = event.get("json.ClientTCPRTTMs") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientTCPRTTMs".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.client.tcp_rtt.ms",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientTCPRTTMs_to_cloudflare_logpush_http_request_client_tcp_rtt_ms_d65b4e12")?;
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

            if event.has_value("json.ClientXRequestedWith") {
                event.rename(
                    "json.ClientXRequestedWith",
                    "cloudflare_logpush.http_request.client.xrequested_with",
                )?;
            }

            if event.has_value("json.Cookies") {
                event.rename("json.Cookies", "cloudflare_logpush.http_request.cookies")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.EdgeCFConnectingO2O") {
                    if let Some(val) = event.get("json.EdgeCFConnectingO2O") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.EdgeCFConnectingO2O".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.http_request.edge.cf_connecting_o2o",
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
                    "convert_edgecfconnectingo2o_to_boolean",
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

            if event.has_value("json.EdgeColoCode") {
                event.rename(
                    "json.EdgeColoCode",
                    "cloudflare_logpush.http_request.edge.colo.code",
                )?;
            }

            let _cond = { event.get_str("json.EdgeColoID") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.EdgeColoID") {
                        if let Some(val) = event.get("json.EdgeColoID") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.EdgeColoID".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.http_request.edge.colo.id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_EdgeColoID_to_cloudflare_logpush_http_request_edge_colo_id_329b38d6")?;
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

            if event.has_value("json.EdgePathingOp") {
                event.rename(
                    "json.EdgePathingOp",
                    "cloudflare_logpush.http_request.edge.pathing.op",
                )?;
            }

            if event.has_value("json.EdgePathingSrc") {
                event.rename(
                    "json.EdgePathingSrc",
                    "cloudflare_logpush.http_request.edge.pathing.src",
                )?;
            }

            if event.has_value("json.EdgePathingStatus") {
                event.rename(
                    "json.EdgePathingStatus",
                    "cloudflare_logpush.http_request.edge.pathing.status",
                )?;
            }

            if event.has_value("json.EdgeRateLimitAction") {
                event.rename(
                    "json.EdgeRateLimitAction",
                    "cloudflare_logpush.http_request.edge.rate.limit.action",
                )?;
            }

            if event.has_value("json.EdgeRateLimitID") {
                if let Some(val) = event.get("json.EdgeRateLimitID") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.EdgeRateLimitID".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.http_request.edge.rate.limit.id",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.EdgeRequestHost") {
                event.rename(
                    "json.EdgeRequestHost",
                    "cloudflare_logpush.http_request.edge.request.host",
                )?;
            }

            let _cond = { event.get_str("json.EdgeResponseBodyBytes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.EdgeResponseBodyBytes") {
                        if let Some(val) = event.get("json.EdgeResponseBodyBytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.EdgeResponseBodyBytes".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.edge.response.body_bytes",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_EdgeResponseBodyBytes_to_cloudflare_logpush_http_request_edge_response_body_bytes_a3f683b9")?;
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
                .get("cloudflare_logpush.http_request.edge.response.body_bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.body.bytes", v)?;
            }

            let _cond = { event.get_str("json.EdgeResponseBytes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.EdgeResponseBytes") {
                        if let Some(val) = event.get("json.EdgeResponseBytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.EdgeResponseBytes".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.edge.response.bytes",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_EdgeResponseBytes_to_cloudflare_logpush_http_request_edge_response_bytes_ae183152")?;
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
                .get("cloudflare_logpush.http_request.edge.response.bytes")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.bytes", v)?;
            }

            let _cond = { event.get_str("json.EdgeResponseCompressionRatio") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.EdgeResponseCompressionRatio") {
                        if let Some(val) = event.get("json.EdgeResponseCompressionRatio") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.EdgeResponseCompressionRatio".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.edge.response.compression_ratio",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_EdgeResponseCompressionRatio_to_cloudflare_logpush_http_request_edge_response_compression_ratio_14ec9748")?;
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
                event.has_value("json.EdgeServerIP")
                    && event.get_str("json.EdgeServerIP") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.EdgeServerIP") {
                        if let Some(val) = event.get("json.EdgeServerIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.EdgeServerIP".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.http_request.edge.server.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_EdgeServerIP_to_cloudflare_logpush_http_request_edge_server_ip_4aaf36d3")?;
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

            let _cond = { event.get_str("json.EdgeTimeToFirstByteMs") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.EdgeTimeToFirstByteMs") {
                        if let Some(val) = event.get("json.EdgeTimeToFirstByteMs") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.EdgeTimeToFirstByteMs".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.edge.time_to_first_byte.ms",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_EdgeTimeToFirstByteMs_to_cloudflare_logpush_http_request_edge_time_to_first_byte_ms_c11ce527")?;
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

            if event.has_value("json.SecurityActions") {
                event.rename(
                    "json.SecurityActions",
                    "cloudflare_logpush.http_request.firewall.matches.action",
                )?;
            }

            let _cond =
                { !event.has_value("cloudflare_logpush.http_request.firewall.matches.action") };
            if _cond {
                if event.has_value("json.FirewallMatchesActions") {
                    event.rename(
                        "json.FirewallMatchesActions",
                        "cloudflare_logpush.http_request.firewall.matches.action",
                    )?;
                }
            }

            if event.has_value("json.SecurityRuleIDs") {
                event.rename(
                    "json.SecurityRuleIDs",
                    "cloudflare_logpush.http_request.firewall.matches.rule_id",
                )?;
            }

            let _cond =
                { !event.has_value("cloudflare_logpush.http_request.firewall.matches.rule_id") };
            if _cond {
                if event.has_value("json.FirewallMatchesRuleIDs") {
                    event.rename(
                        "json.FirewallMatchesRuleIDs",
                        "cloudflare_logpush.http_request.firewall.matches.rule_id",
                    )?;
                }
            }

            if event.has_value("json.SecuritySources") {
                event.rename(
                    "json.SecuritySources",
                    "cloudflare_logpush.http_request.firewall.matches.sources",
                )?;
            }

            let _cond =
                { !event.has_value("cloudflare_logpush.http_request.firewall.matches.sources") };
            if _cond {
                if event.has_value("json.FirewallMatchesSources") {
                    event.rename(
                        "json.FirewallMatchesSources",
                        "cloudflare_logpush.http_request.firewall.matches.sources",
                    )?;
                }
            }

            if event.has_value("json.JA3Hash") {
                event.rename("json.JA3Hash", "cloudflare_logpush.http_request.ja3_hash")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.ja3_hash")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.client.ja3", v)?;
            }

            if event.has_value("json.JA4") {
                event.rename("json.JA4", "cloudflare_logpush.http_request.ja4")?;
            }

            if event.has_value("json.JA4Signals") {
                event.rename(
                    "json.JA4Signals",
                    "cloudflare_logpush.http_request.ja4_signals",
                )?;
            }

            let _cond = { event.get_str("json.OriginDNSResponseTimeMs") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginDNSResponseTimeMs") {
                        if let Some(val) = event.get("json.OriginDNSResponseTimeMs") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginDNSResponseTimeMs".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.origin.dns_response_time.ms",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginDNSResponseTimeMs_to_cloudflare_logpush_http_request_origin_dns_response_time_ms_d0100f64")?;
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

            let _cond = { event.get_str("json.OriginRequestHeaderSendDurationMs") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginRequestHeaderSendDurationMs") {
                        if let Some(val) = event.get("json.OriginRequestHeaderSendDurationMs") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginRequestHeaderSendDurationMs".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.http_request.origin.request_header_send_duration.ms", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginRequestHeaderSendDurationMs_to_cloudflare_logpush_http_request_origin_request_header_send_duration_ms_b5c4844e")?;
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

            let _cond = { event.get_str("json.OriginResponseBytes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginResponseBytes") {
                        if let Some(val) = event.get("json.OriginResponseBytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginResponseBytes".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.origin.response.bytes",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginResponseBytes_to_cloudflare_logpush_http_request_origin_response_bytes_4252af5f")?;
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

            let _cond = { event.get_str("json.OriginResponseDurationMs") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginResponseDurationMs") {
                        if let Some(val) = event.get("json.OriginResponseDurationMs") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginResponseDurationMs".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.origin.response.duration.ms",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginResponseDurationMs_to_cloudflare_logpush_http_request_origin_response_duration_ms_50a340a2")?;
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

            let _cond = { event.get_str("json.OriginResponseHeaderReceiveDurationMs") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginResponseHeaderReceiveDurationMs") {
                        if let Some(val) = event.get("json.OriginResponseHeaderReceiveDurationMs") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginResponseHeaderReceiveDurationMs".into(),
                                    message,
                                }
                            })?;
                            event.set("cloudflare_logpush.http_request.origin.response.header_receive_duration.ms", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginResponseHeaderReceiveDurationMs_to_cloudflare_logpush_http_request_origin_response_header_receive_duration_ms_10d61fbe")?;
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

            let _cond = { event.get_str("json.OriginResponseStatus") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginResponseStatus") {
                        if let Some(val) = event.get("json.OriginResponseStatus") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginResponseStatus".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.origin.response.status",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginResponseStatus_to_cloudflare_logpush_http_request_origin_response_status_ef757c92")?;
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

            let _cond = { event.get_str("json.OriginResponseTime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginResponseTime") {
                        if let Some(val) = event.get("json.OriginResponseTime") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginResponseTime".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.origin.response.time",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginResponseTime_to_cloudflare_logpush_http_request_origin_response_time_e009296b")?;
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

            if event.has_value("json.OriginSSLProtocol") {
                event.rename(
                    "json.OriginSSLProtocol",
                    "cloudflare_logpush.http_request.origin.ssl_protocol",
                )?;
            }

            let _cond = { event.get_str("json.OriginTCPHandshakeDurationMs") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginTCPHandshakeDurationMs") {
                        if let Some(val) = event.get("json.OriginTCPHandshakeDurationMs") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginTCPHandshakeDurationMs".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.origin.tcp_handshake_duration.ms",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginTCPHandshakeDurationMs_to_cloudflare_logpush_http_request_origin_tcp_handshake_duration_ms_03155549")?;
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

            let _cond = { event.get_str("json.OriginTLSHandshakeDurationMs") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.OriginTLSHandshakeDurationMs") {
                        if let Some(val) = event.get("json.OriginTLSHandshakeDurationMs") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.OriginTLSHandshakeDurationMs".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.origin.tls_handshake_duration.ms",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginTLSHandshakeDurationMs_to_cloudflare_logpush_http_request_origin_tls_handshake_duration_ms_8519a42f")?;
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

            if event.has_value("json.ParentRayID") {
                event.rename(
                    "json.ParentRayID",
                    "cloudflare_logpush.http_request.parent_ray.id",
                )?;
            }

            if event.has_value("json.RayID") {
                event.rename("json.RayID", "cloudflare_logpush.http_request.ray.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.ray.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.http_request.ray.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.id", v)?;
            }

            if event.has_value("json.RequestHeaders") {
                event.rename(
                    "json.RequestHeaders",
                    "cloudflare_logpush.http_request.request.headers",
                )?;
            }

            if event.has_value("json.ResponseHeaders") {
                event.rename(
                    "json.ResponseHeaders",
                    "cloudflare_logpush.http_request.response.headers",
                )?;
            }

            if event.has_value("json.SecurityLevel") {
                event.rename(
                    "json.SecurityLevel",
                    "cloudflare_logpush.http_request.security_level",
                )?;
            }

            let _cond = { event.get_str("json.SmartRouteColoID") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.SmartRouteColoID") {
                        if let Some(val) = event.get("json.SmartRouteColoID") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.SmartRouteColoID".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.smart_route.colo.id",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_SmartRouteColoID_to_cloudflare_logpush_http_request_smart_route_colo_id_9738fe1a")?;
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

            let _cond = { event.get_str("json.UpperTierColoID") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.UpperTierColoID") {
                        if let Some(val) = event.get("json.UpperTierColoID") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.UpperTierColoID".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.upper_tier.colo.id",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_UpperTierColoID_to_cloudflare_logpush_http_request_upper_tier_colo_id_72c67384")?;
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

            if event.has_value("json.SecurityAction") {
                event.rename(
                    "json.SecurityAction",
                    "cloudflare_logpush.http_request.waf.action",
                )?;
            }

            let _cond = { !event.has_value("cloudflare_logpush.http_request.waf.action") };
            if _cond {
                if event.has_value("json.WAFAction") {
                    event.rename(
                        "json.WAFAction",
                        "cloudflare_logpush.http_request.waf.action",
                    )?;
                }
            }

            if event.has_value("json.WAFFlags") {
                event.rename("json.WAFFlags", "cloudflare_logpush.http_request.waf.flag")?;
            }

            if event.has_value("json.WAFMatchedVar") {
                event.rename(
                    "json.WAFMatchedVar",
                    "cloudflare_logpush.http_request.waf.matched_var",
                )?;
            }

            if event.has_value("json.WAFProfile") {
                event.rename(
                    "json.WAFProfile",
                    "cloudflare_logpush.http_request.waf.profile",
                )?;
            }

            if event.has_value("json.SecurityRuleID") {
                event.rename(
                    "json.SecurityRuleID",
                    "cloudflare_logpush.http_request.waf.rule.id",
                )?;
            }

            let _cond = { !event.has_value("cloudflare_logpush.http_request.waf.rule.id") };
            if _cond {
                if event.has_value("json.WAFRuleID") {
                    event.rename(
                        "json.WAFRuleID",
                        "cloudflare_logpush.http_request.waf.rule.id",
                    )?;
                }
            }

            if event.has_value("json.SecurityRuleDescription") {
                event.rename(
                    "json.SecurityRuleDescription",
                    "cloudflare_logpush.http_request.waf.rule.message",
                )?;
            }

            let _cond = { !event.has_value("cloudflare_logpush.http_request.waf.rule.message") };
            if _cond {
                if event.has_value("json.WAFRuleMessage") {
                    event.rename(
                        "json.WAFRuleMessage",
                        "cloudflare_logpush.http_request.waf.rule.message",
                    )?;
                }
            }

            let _cond = { event.get_str("json.WAFAttackScore") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.WAFAttackScore") {
                        if let Some(val) = event.get("json.WAFAttackScore") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.WAFAttackScore".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.waf.score.global",
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
                        "convert_waf_score_global",
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

            let _cond = { event.get_str("json.WAFRCEAttackScore") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.WAFRCEAttackScore") {
                        if let Some(val) = event.get("json.WAFRCEAttackScore") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.WAFRCEAttackScore".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.http_request.waf.score.rce", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_waf_score_rce")?;
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

            let _cond = { event.get_str("json.WAFSQLiAttackScore") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.WAFSQLiAttackScore") {
                        if let Some(val) = event.get("json.WAFSQLiAttackScore") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.WAFSQLiAttackScore".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.http_request.waf.score.sqli", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_waf_score_sqli")?;
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

            let _cond = { event.get_str("json.WAFXSSAttackScore") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.WAFXSSAttackScore") {
                        if let Some(val) = event.get("json.WAFXSSAttackScore") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.WAFXSSAttackScore".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.http_request.waf.score.xss", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_waf_score_xss")?;
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

            let _cond = { event.get_str("json.WorkerCPUTime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.WorkerCPUTime") {
                        if let Some(val) = event.get("json.WorkerCPUTime") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.WorkerCPUTime".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.worker.cpu_time",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_WorkerCPUTime_to_cloudflare_logpush_http_request_worker_cpu_time_8e836d9d")?;
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

            if event.has_value("json.WorkerStatus") {
                event.rename(
                    "json.WorkerStatus",
                    "cloudflare_logpush.http_request.worker.status",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.WorkerSubrequest") {
                    if let Some(val) = event.get("json.WorkerSubrequest") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.WorkerSubrequest".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "cloudflare_logpush.http_request.worker.subrequest.value",
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
                    "convert_workersubrequest_to_boolean",
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

            let _cond = { event.get_str("json.WorkerSubrequestCount") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.WorkerSubrequestCount") {
                        if let Some(val) = event.get("json.WorkerSubrequestCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.WorkerSubrequestCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.worker.subrequest.count",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_WorkerSubrequestCount_to_cloudflare_logpush_http_request_worker_subrequest_count_108a2873")?;
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

            let _cond = { event.get_str("json.WorkerWallTimeUs") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.WorkerWallTimeUs") {
                        if let Some(val) = event.get("json.WorkerWallTimeUs") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.WorkerWallTimeUs".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cloudflare_logpush.http_request.worker.wall_time_us",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_WorkerWallTimeUs_to_cloudflare_logpush_http_request_worker_wall_time_us_ecfb12d0")?;
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

            if event.has_value("json.ZoneID") {
                if let Some(val) = event.get("json.ZoneID") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.ZoneID".into(),
                            message,
                        }
                    })?;
                    event.set("cloudflare_logpush.http_request.zone.id", converted)?;
                }
            }

            if event.has_value("json.ZoneName") {
                event.rename("json.ZoneName", "cloudflare_logpush.http_request.zone.name")?;
            }

            if event.has_value("json.FraudAttack") {
                event.rename(
                    "json.FraudAttack",
                    "cloudflare_logpush.http_request.fraud.attack",
                )?;
            }

            if event.has_value("json.FraudDetectionIDs") {
                if let Some(val) = event.get("json.FraudDetectionIDs") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.FraudDetectionIDs".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "cloudflare_logpush.http_request.fraud.detection_ids",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.FraudDetectionTags") {
                event.rename(
                    "json.FraudDetectionTags",
                    "cloudflare_logpush.http_request.fraud.detection_tags",
                )?;
            }

            if event.has_value("json.FraudEmailRisk") {
                event.rename(
                    "json.FraudEmailRisk",
                    "cloudflare_logpush.http_request.fraud.email_risk",
                )?;
            }

            if event.has_value("json.FraudUserID") {
                event.rename(
                    "json.FraudUserID",
                    "cloudflare_logpush.http_request.fraud.user_id",
                )?;
            }

            if event.has_value("json.JSDetectionPassed") {
                event.rename(
                    "json.JSDetectionPassed",
                    "cloudflare_logpush.http_request.js_detection_passed",
                )?;
            }

            if event.has_value("json.PayPerCrawlStatus") {
                event.rename(
                    "json.PayPerCrawlStatus",
                    "cloudflare_logpush.http_request.pay_per_crawl_status",
                )?;
            }

            if event.has_value("json.VerifiedBotCategory") {
                event.rename(
                    "json.VerifiedBotCategory",
                    "cloudflare_logpush.http_request.verified_bot_category",
                )?;
            }

            if event.has_value("json.WebAssetsLabelsManaged") {
                event.rename(
                    "json.WebAssetsLabelsManaged",
                    "cloudflare_logpush.http_request.web_assets.labels_managed",
                )?;
            }

            if event.has_value("json.WebAssetsOperationID") {
                event.rename(
                    "json.WebAssetsOperationID",
                    "cloudflare_logpush.http_request.web_assets.operation_id",
                )?;
            }

            if event.has_value("json.WorkerScriptName") {
                event.rename(
                    "json.WorkerScriptName",
                    "cloudflare_logpush.http_request.worker.script_name",
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.http_request.fraud.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.http_request.fraud.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.http_request.ja3_hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cloudflare_logpush.http_request.ja3_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cloudflare_logpush.http_request.client.mtls.auth.fingerprint") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cloudflare_logpush.http_request.client.mtls.auth.fingerprint")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.http_request.ja4") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cloudflare_logpush.http_request.ja4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.http_request.client.request.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.http_request.client.request.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.http_request.edge.request.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.http_request.edge.request.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.http_request.edge.server.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.http_request.edge.server.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.http_request.client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.http_request.client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.http_request.origin.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.http_request.origin.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");
            event.remove("_conf");

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
                event.remove("cloudflare_logpush.http_request.origin.ip");
                event.remove("cloudflare_logpush.http_request.client.request.method");
                event.remove("cloudflare_logpush.http_request.edge.response.content_type");
                event.remove("cloudflare_logpush.http_request.edge.response.status");
                event.remove("cloudflare_logpush.http_request.client.asn");
                event.remove("cloudflare_logpush.http_request.client.country");
                event.remove("cloudflare_logpush.http_request.client.ip");
                event.remove("cloudflare_logpush.http_request.client.request.user.agent");
                event.remove("cloudflare_logpush.http_request.client.ssl.cipher");
                event.remove("cloudflare_logpush.http_request.client.device.type");
                event.remove("cloudflare_logpush.http_request.client.request.bytes");
                event.remove("cloudflare_logpush.http_request.edge.response.bytes");
                event.remove("cloudflare_logpush.http_request.client.src.port");
                event.remove("cloudflare_logpush.http_request.ja3_hash");
                event.remove("cloudflare_logpush.http_request.client.request.referer");
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["N/A".into()],
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
                            .get("_ingest.on_failure_pipeline")
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
