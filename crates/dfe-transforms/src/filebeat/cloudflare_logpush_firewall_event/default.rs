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

            let _cond = {
                event.has_value("json.Datetime")
                    && event.get("json.Datetime").is_some_and(|v| v.is_number())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: long t = (long)(ctx.json.Datetime);\nif (t > (long)(1e18)) {\n  ctx.json.Datetime = t/(long)(1e6)\n} else if (t < (long)(1e10))  {\n  ctx.json.Datetime = t*(long)(1e3)\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"long t = (long)(ctx.json.Datetime);\nif (t > (long)(1e18)) {\n  ctx.json.Datetime = t/(long)(1e6)\n} else if (t < (long)(1e10))  {\n  ctx.json.Datetime = t*(long)(1e3)\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "painless_datetime_to_milli",
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
                            Some(parsed) => event.set("@timestamp", parsed)?,
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
                        "date_json_Datetime_138a0e6b",
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
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloudflare_logpush.firewall_event.timestamp", v)?;
            }

            if event.has_value("json.Action") {
                event.rename("json.Action", "cloudflare_logpush.firewall_event.action")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.firewall_event.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if event.has_value("json.ClientRequestMethod") {
                event.rename(
                    "json.ClientRequestMethod",
                    "cloudflare_logpush.firewall_event.client.request.method",
                )?;
            }

            if event.has_value("json.ContentScanObjResults") {
                event.rename(
                    "json.ContentScanObjResults",
                    "cloudflare_logpush.firewall_event.content_scan.results",
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
                            "cloudflare_logpush.firewall_event.content_scan.sizes",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_json_ContentScanObjSizes_to_cloudflare_logpush_firewall_event_content_scan_sizes_97df0491")?;
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
                    "cloudflare_logpush.firewall_event.content_scan.types",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.firewall_event.client.request.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
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
                                "cloudflare_logpush.firewall_event.edge.response.status",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_EdgeResponseStatus_to_cloudflare_logpush_firewall_event_edge_response_status_346dde9e")?;
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
                .get("cloudflare_logpush.firewall_event.edge.response.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            if event.has_value("json.RuleID") {
                event.rename("json.RuleID", "cloudflare_logpush.firewall_event.rule.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.firewall_event.rule.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has_value("json.Description") {
                event.rename(
                    "json.Description",
                    "cloudflare_logpush.firewall_event.rule.description",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.firewall_event.rule.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
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
                            event.set(
                                "cloudflare_logpush.firewall_event.client.asn.value",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientASN_to_cloudflare_logpush_firewall_event_client_asn_value_90a6e434")?;
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
                .get("cloudflare_logpush.firewall_event.client.asn.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.as.number", v)?;
            }

            if event.has_value("json.ClientCountry") {
                event.rename(
                    "json.ClientCountry",
                    "cloudflare_logpush.firewall_event.client.country",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.firewall_event.client.country")
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
                            event.set("cloudflare_logpush.firewall_event.client.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_ClientIP_to_cloudflare_logpush_firewall_event_client_ip_534d6853")?;
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
                .get("cloudflare_logpush.firewall_event.client.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if event.has_value("json.ClientASNDescription") {
                event.rename(
                    "json.ClientASNDescription",
                    "cloudflare_logpush.firewall_event.client.asn.description",
                )?;
            }

            if event.has_value("json.ClientIPClass") {
                event.rename(
                    "json.ClientIPClass",
                    "cloudflare_logpush.firewall_event.client.ip_class",
                )?;
            }

            if event.has_value("json.ClientRefererHost") {
                event.rename(
                    "json.ClientRefererHost",
                    "cloudflare_logpush.firewall_event.client.referer.host",
                )?;
            }

            if event.has_value("json.ClientRefererPath") {
                event.rename(
                    "json.ClientRefererPath",
                    "cloudflare_logpush.firewall_event.client.referer.path",
                )?;
            }

            if event.has_value("json.ClientRefererQuery") {
                event.rename(
                    "json.ClientRefererQuery",
                    "cloudflare_logpush.firewall_event.client.referer.query",
                )?;
            }

            if event.has_value("json.ClientRefererScheme") {
                event.rename(
                    "json.ClientRefererScheme",
                    "cloudflare_logpush.firewall_event.client.referer.scheme",
                )?;
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

            if event.has_value("json.ClientRequestHost") {
                event.rename(
                    "json.ClientRequestHost",
                    "cloudflare_logpush.firewall_event.client.request.host",
                )?;
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

            if event.has_value("json.ClientRequestPath") {
                event.rename(
                    "json.ClientRequestPath",
                    "cloudflare_logpush.firewall_event.client.request.path",
                )?;
            }

            if event.has_value("json.ClientRequestProtocol") {
                event.rename(
                    "json.ClientRequestProtocol",
                    "cloudflare_logpush.firewall_event.client.request.protocol",
                )?;
            }

            let _cond = {
                event.has_value("cloudflare_logpush.firewall_event.client.request.protocol")
                    && event.get_str("cloudflare_logpush.firewall_event.client.request.protocol")
                        != Some("none")
                    && event.get_str("cloudflare_logpush.firewall_event.client.request.protocol")
                        != Some("unknown")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cloudflare_logpush.firewall_event.client.request.protocol")
                    {
                        if let Some(input) = event
                            .get_string("cloudflare_logpush.firewall_event.client.request.protocol")
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
                                    path:
                                        "cloudflare_logpush.firewall_event.client.request.protocol"
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
                event.get("url.query").is_some_and(|v| v.is_string())
                    && event
                        .get_str("url.query")
                        .is_some_and(|s| s.starts_with("?"))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.url.query = ctx.url.query.substring(1);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(r#"ctx.url.query = ctx.url.query.substring(1);\n"#),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_4acec607")?;
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

            if event.has_value("json.ClientRequestQuery") {
                event.rename(
                    "json.ClientRequestQuery",
                    "cloudflare_logpush.firewall_event.client.request.query",
                )?;
            }

            if event.has_value("json.ClientRequestScheme") {
                event.rename(
                    "json.ClientRequestScheme",
                    "cloudflare_logpush.firewall_event.client.request.scheme",
                )?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.firewall_event.client.request.scheme")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.scheme", v)?;
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
                    "cloudflare_logpush.firewall_event.client.request.user.agent",
                )?;
            }

            if event.has_value("json.EdgeColoCode") {
                event.rename(
                    "json.EdgeColoCode",
                    "cloudflare_logpush.firewall_event.edge.colo.code",
                )?;
            }

            if event.has_value("json.Kind") {
                event.rename("json.Kind", "cloudflare_logpush.firewall_event.kind")?;
            }

            if event.has_value("json.LeakedCredentialCheckResult") {
                event.rename(
                    "json.LeakedCredentialCheckResult",
                    "cloudflare_logpush.firewall_event.leaked_credential_check",
                )?;
            }

            let _cond = { event.get_str("json.MatchIndex") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.MatchIndex") {
                        if let Some(val) = event.get("json.MatchIndex") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.MatchIndex".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("cloudflare_logpush.firewall_event.match_index", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_MatchIndex_to_cloudflare_logpush_firewall_event_match_index_23dc3fca")?;
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

            if event.has_value("json.Metadata") {
                event.rename(
                    "json.Metadata",
                    "cloudflare_logpush.firewall_event.meta_data",
                )?;
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
                                "cloudflare_logpush.firewall_event.origin.response.status",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_OriginResponseStatus_to_cloudflare_logpush_firewall_event_origin_response_status_da198427")?;
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

            if event.has_value("json.OriginatorRayID") {
                event.rename(
                    "json.OriginatorRayID",
                    "cloudflare_logpush.firewall_event.origin.ray.id",
                )?;
            }

            if event.has_value("json.Ref") {
                event.rename("json.Ref", "cloudflare_logpush.firewall_event.ref")?;
            }

            if event.has_value("json.RayID") {
                event.rename("json.RayID", "cloudflare_logpush.firewall_event.ray.id")?;
            }

            if let Some(v) = event
                .get("cloudflare_logpush.firewall_event.ray.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.Source") {
                event.rename("json.Source", "cloudflare_logpush.firewall_event.source")?;
            }

            if event.has_value("json.ZoneName") {
                event.rename(
                    "json.ZoneName",
                    "cloudflare_logpush.firewall_event.zone.name",
                )?;
            }

            if event.has_value("json.FraudUserID") {
                event.rename(
                    "json.FraudUserID",
                    "cloudflare_logpush.firewall_event.fraud.user_id",
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.firewall_event.fraud.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cloudflare_logpush.firewall_event.fraud.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cloudflare_logpush.firewall_event.client.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cloudflare_logpush.firewall_event.client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cloudflare_logpush.firewall_event.client.referer.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.firewall_event.client.referer.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("cloudflare_logpush.firewall_event.client.request.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cloudflare_logpush.firewall_event.client.request.host")
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
                event.remove("cloudflare_logpush.firewall_event.timestamp");
                event.remove("cloudflare_logpush.firewall_event.action");
                event.remove("cloudflare_logpush.firewall_event.client.request.method");
                event.remove("cloudflare_logpush.firewall_event.edge.response.status");
                event.remove("cloudflare_logpush.firewall_event.rule.id");
                event.remove("cloudflare_logpush.firewall_event.rule.description");
                event.remove("cloudflare_logpush.firewall_event.client.asn.value");
                event.remove("cloudflare_logpush.firewall_event.client.country");
                event.remove("cloudflare_logpush.firewall_event.client.ip");
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || v == 'N/A' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
                ),
            )?;

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
