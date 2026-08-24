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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set(
                "event.category",
                Value::Array(vec![json!("web"), json!("network")]),
            )?;

            event.append("event.type", json!("access"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            parse_json_field(event, "event.original", "json")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();

            if event.has("json.httpRequest.clientIp") {
                event.rename("json.httpRequest.clientIp", "source.ip")?;
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { !event.has_value("source.geo.country_iso_code") };
            if _cond {
                if event.has("json.httpRequest.country") {
                    event.rename("json.httpRequest.country", "source.geo.country_iso_code")?;
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            let _cond = { !event.has_value("source.as.number") };
            if _cond {
                if event.has("json.ClientASN") {
                    event.rename("json.ClientASN", "source.as.number")?;
                }
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.captchaResponse.responseCode") {
                    if let Some(val) = event.get("json.captchaResponse.responseCode") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.captchaResponse.responseCode".into(),
                                message,
                            }
                        })?;
                        event.set("aws.waf.captcha_response.response_code", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_captchaResponse_responseCode_to_long",
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
                event.has_value("json.captchaResponse.solveTimestamp")
                    && event.get_str("json.captchaResponse.solveTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.captchaResponse.solveTimestamp")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            event.set("aws.waf.captcha_response.solve_timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_captchaResponse_solveTimestamp",
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

            if event.has("json.captchaResponse.failureReason") {
                event.rename(
                    "json.captchaResponse.failureReason",
                    "aws.waf.captcha_response.failure_reason",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.challengeResponse.responseCode") {
                    if let Some(val) = event.get("json.challengeResponse.responseCode") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.challengeResponse.responseCode".into(),
                                message,
                            }
                        })?;
                        event.set("aws.waf.challenge_response.response_code", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_challengeResponse_responseCode_to_long",
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
                event.has_value("json.challengeResponse.solveTimestamp")
                    && event.get_str("json.challengeResponse.solveTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.challengeResponse.solveTimestamp")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            event.set("aws.waf.challenge_response.solve_timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_challengeResponse_solveTimestamp",
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

            if event.has("json.challengeResponse.failureReason") {
                event.rename(
                    "json.challengeResponse.failureReason",
                    "aws.waf.challenge_response.failure_reason",
                )?;
            }

            if event.has_value("json.formatVersion") {
                if let Some(val) = event.get("json.formatVersion") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.formatVersion".into(),
                            message,
                        }
                    })?;
                    event.set("aws.waf.format_version", converted)?;
                }
            }

            if event.has("json.httpRequest.fragment") {
                event.rename("json.httpRequest.fragment", "url.fragment")?;
            }

            if event.has("json.httpRequest.host") {
                event.rename("json.httpRequest.host", "url.registered_domain")?;
            }

            if event.has("json.httpRequest.requestId") {
                event.rename("json.httpRequest.requestId", "http.request.id")?;
            }

            if event.has("json.httpRequest.scheme") {
                event.rename("json.httpRequest.scheme", "url.scheme")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.requestBodySize") {
                    if let Some(val) = event.get("json.requestBodySize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.requestBodySize".into(),
                                message,
                            }
                        })?;
                        event.set("aws.waf.request_body_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_requestBodySize_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.requestBodySizeInspectedByWAF") {
                    if let Some(val) = event.get("json.requestBodySizeInspectedByWAF") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.requestBodySizeInspectedByWAF".into(),
                                message,
                            }
                        })?;
                        event.set("aws.waf.request_body_size_inspected_by_waf", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_requestBodySizeInspectedByWAF_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.responseCodeSent") {
                    if let Some(val) = event.get("json.responseCodeSent") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.responseCodeSent".into(),
                                message,
                            }
                        })?;
                        event.set("aws.waf.response_code_sent", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_responseCodeSent_to_long",
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

            if event.has("json.httpRequest.httpMethod") {
                event.rename("json.httpRequest.httpMethod", "http.request.method")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.httpRequest.httpVersion") {
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
                    }
                }
                Ok(())
            })();

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("network.protocol")
                    && event.get_str("network.protocol") == Some("http")
            };
            if _cond {
                event.set("network.transport", json!("tcp"))?;
            }

            if event.has("json.httpRequest.args") {
                event.rename("json.httpRequest.args", "url.query")?;
            }

            if event.has("json.httpRequest.uri") {
                event.rename("json.httpRequest.uri", "url.path")?;
            }

            if event.has("json.ja3Fingerprint") {
                event.rename("json.ja3Fingerprint", "tls.client.ja3")?;
            }

            if event.has("json.ja4Fingerprint") {
                event.rename("json.ja4Fingerprint", "aws.waf.ja4_fingerprint")?;
            }

            if event.has("json.labels") {
                event.rename("json.labels", "aws.waf.labels")?;
            }

            if event.has("json.oversizeFields") {
                event.rename("json.oversizeFields", "aws.waf.oversize_fields")?;
            }

            if event.has("json.terminatingRuleMatchDetails") {
                event.rename(
                    "json.terminatingRuleMatchDetails",
                    "aws.waf.terminating_rule_match_details",
                )?;
            }

            if event.has("json.ruleGroupList") {
                event.rename("json.ruleGroupList", "aws.waf.rule_group_list")?;
            }

            if event.has("json.rateBasedRuleList") {
                event.rename("json.rateBasedRuleList", "aws.waf.rate_based_rule_list")?;
            }

            if event.has("json.nonTerminatingMatchingRules") {
                event.rename(
                    "json.nonTerminatingMatchingRules",
                    "aws.waf.non_terminating_matching_rules",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.json.httpRequest.headers != null) {\n  ctx.aws.waf.request = new HashMap();\n  ctx.aws.waf.request.headers = new HashMap();\n  for (def i = 0; i < ctx.json.httpRequest.headers.length; i++) {\n    ctx.aws.waf.request.headers[ctx.json.httpRequest.headers[i].name] = ctx.json.httpRequest.headers[i].value;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.json.httpRequest.headers != null) {\n  ctx.aws.waf.request = new HashMap();\n  ctx.aws.waf.request.headers = new HashMap();\n  for (def i = 0; i < ctx.json.httpRequest.headers.length; i++) {\n    ctx.aws.waf.request.headers[ctx.json.httpRequest.headers[i].name] = ctx.json.httpRequest.headers[i].value;\n  }\n}"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.requestHeadersInserted")
                    && event.has_value("aws.waf")
                    && event
                        .get("json.requestHeadersInserted")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.aws.waf.request_headers_inserted = new HashMap(); for (def i = 0; i < ctx.json.requestHeadersInserted.length; i++) {\n  ctx.aws.waf.request_headers_inserted[ctx.json.requestHeadersInserted[i].name] = ctx.json.requestHeadersInserted[i].value;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.aws.waf.request_headers_inserted = new HashMap(); for (def i = 0; i < ctx.json.requestHeadersInserted.length; i++) {\n  ctx.aws.waf.request_headers_inserted[ctx.json.requestHeadersInserted[i].name] = ctx.json.requestHeadersInserted[i].value;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_request_headers_inserted",
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

            if event.has("json.action") {
                event.rename("json.action", "event.action")?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.set("cloud.provider", json!("aws"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { event.get_str("event.action") == Some("ALLOW") };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { event.get_str("event.action") == Some("BLOCK") };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            if event.has("json.webaclId") {
                event.rename("json.webaclId", "aws.waf.arn")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("aws.waf.arn") {
                    if let Some(input) = event.get_string("aws.waf.arn") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("arn:") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(":") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(":") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(":") else {
                                break 'dissect false;
                            };
                            captured.push(("cloud.service.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(":") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(":") else {
                                break 'dissect false;
                            };
                            captured.push(("cloud.region", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(":") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(":") else {
                                break 'dissect false;
                            };
                            captured.push(("cloud.account.id", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(":") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("aws.waf.id", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            if event.has("json.terminatingRuleId") {
                event.rename("json.terminatingRuleId", "rule.id")?;
            }

            if event.has("json.terminatingRuleType") {
                event.rename("json.terminatingRuleType", "rule.ruleset")?;
            }

            if event.has("json.httpSourceName") {
                event.rename("json.httpSourceName", "aws.waf.source.name")?;
            }

            if event.has("json.httpSourceId") {
                event.rename("json.httpSourceId", "aws.waf.source.id")?;
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n    for (def x : map.values()) {\n        if (x instanceof Map) {\n            handleMap(x);\n        } else if (x instanceof List) {\n            handleList(x);\n        }\n    }\n    map.values().removeIf(v -> v == null || v == \"\" || v == \"-\" || ((v instanceof List || v instanceof Map) && v.isEmpty()));\n}\nvoid handleList(List list) {\n    for (def x : list) {\n        if (x instanceof Map) {\n            handleMap(x);\n        } else if (x instanceof List) {\n            handleList(x);\n        }\n    }\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n    for (def x : map.values()) {\n        if (x instanceof Map) {\n            handleMap(x);\n        } else if (x instanceof List) {\n            handleList(x);\n        }\n    }\n    map.values().removeIf(v -> v == null || v == \"\" || v == \"-\" || ((v instanceof List || v instanceof Map) && v.isEmpty()));\n}\nvoid handleList(List list) {\n    for (def x : list) {\n        if (x instanceof Map) {\n            handleMap(x);\n        } else if (x instanceof List) {\n            handleList(x);\n        }\n    }\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
