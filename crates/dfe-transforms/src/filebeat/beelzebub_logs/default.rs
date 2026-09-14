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

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

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
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("redact_passwords"))
                        }
                        serde_json::Value::String(s) => s.contains("redact_passwords"),
                        _ => false,
                    })
                    && !(event
                        .get_str("event.original")
                        .is_some_and(|s| cached_regex!(r#"\"Password\": *\"\""#).is_match(s)))
            };
            if _cond {
                if event.has_value("event.original") {
                    gsub_field(
                        event,
                        "event.original",
                        "event.original",
                        cached_regex!("(.*\"Password\": *\").*?(\".*)"),
                        "$1<REDACTED>$2",
                    )?;
                }
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("redact_passwords"))
                        }
                        serde_json::Value::String(s) => s.contains("redact_passwords"),
                        _ => false,
                    })
                    && !(event
                        .get_str("event.original")
                        .is_some_and(|s| cached_regex!(r#"\"Password\": *\"\""#).is_match(s)))
            };
            if _cond {
                event.remove("json");
            }

            let _cond = { event.has_value("event.original") && !event.has_value("json") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            let _cond = { event.has_value("json.event.DateTime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.event.DateTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event.DateTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("json.event.DateTime") && event.has_value("json.time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("json.event.ID")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("json.event.Msg")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if let Some(v) = event
                .get("json.msg")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            let _cond = {
                event.has_value("json.event.SourceIp")
                    && event.get_str("json.event.SourceIp") != Some("")
            };
            if _cond {
                if event.has_value("json.event.SourceIp") {
                    if let Some(val) = event.get("json.event.SourceIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.event.SourceIp".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
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

            let _cond = { event.get_str("json.event.SourcePort") != Some("") };
            if _cond {
                if event.has_value("json.event.SourcePort") {
                    if let Some(val) = event.get("json.event.SourcePort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.event.SourcePort".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if let Some(v) = event
                .get("json.event.HTTPMethod")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            if let Some(v) = event
                .get("json.event.Body")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.body.content", v)?;
            }

            let _cond = {
                event.has_value("http.request.body.content")
                    && (event
                        .get("http.request.body.content")
                        .is_some_and(|v| v.is_string()))
            };
            if _cond {
                // Painless script
                // Source: ctx.http.request.body.bytes = ctx.http.request.body.content.length();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.http.request.body.bytes = ctx.http.request.body.content.length();"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.event.RequestURI")
                    && event.get_str("json.event.RequestURI") != Some("")
            };
            if _cond {
                if event.has_value("json.event.RequestURI") {
                    uri_parts(event, "json.event.RequestURI", "url", true, false)?;
                }
            }

            let _cond = {
                event.has_value("json.event.UserAgent")
                    && event.get_str("json.event.UserAgent") != Some("")
            };
            if _cond {
                if event.has_value("json.event.UserAgent") {
                    if let Some(ua_str) = event.get_string("json.event.UserAgent") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.remove("user_agent");
                            let device_kind = ua.device_type();
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
                            if let Some(kind) = device_kind {
                                event.set("user_agent.device.type", json!(kind))?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.get_str("json.event.Headers") == Some("") };
            if _cond {
                if event.remove("json.event.Headers").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.event.Headers".into(),
                    });
                }
            }

            let _cond = {
                event
                    .get("json.event.Headers")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("json.event.Headers") {
                    event.rename("json.event.Headers", "json.event.HeadersText")?;
                }
            }

            let _cond = {
                !event.has_value("json.event.Headers")
                    && event
                        .get("json.event.HeadersMap")
                        .is_some_and(|v| v.is_object())
            };
            if _cond {
                if event.has_value("json.event.HeadersMap") {
                    event.rename_over("json.event.HeadersMap", "json.event.Headers")?;
                }
            }

            let _cond = { !event.has_value("json.event.Headers") };
            if _cond {
                if event.has_value("json.event.HeadersText") {
                    gsub_field(
                        event,
                        "json.event.HeadersText",
                        "_tmp.HeadersText",
                        cached_regex!("(^\\[Key: |],$)"),
                        "",
                    )?;
                }
            }

            let _cond = { !event.has_value("json.event.Headers") };
            if _cond {
                if event.has_value("_tmp.HeadersText") {
                    gsub_field(
                        event,
                        "_tmp.HeadersText",
                        "_tmp.HeadersText",
                        cached_regex!("],\\[Key: "),
                        ",##",
                    )?;
                }
            }

            let _cond = { !event.has_value("json.event.Headers") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.HeadersText") {
                        if let Some(kv_str) = event.get_string("_tmp.HeadersText") {
                            let mut kv_gap = false;
                            for pair in kv_str.split(",##") {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) =
                                    pair.split_once(", values: ").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "_tmp.HeadersText".into(),
                                        split: ", values: ".into(),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
                                    let value = value
                                        .strip_prefix(['(', '[', '<', '"', '\''])
                                        .unwrap_or(value);
                                    let value = value
                                        .strip_suffix([']', ')', '>', '"', '\''])
                                        .unwrap_or(value);
                                    let value = value.trim_matches(|c: char| matches!(c, ' '));
                                    if !key.is_empty() {
                                        kv_put(
                                            event,
                                            &format!("json.event.Headers.{}", key),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.event.Headers")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: def lowercaseMap = [:];\nfor(def entry : ctx.json.event.Headers.entrySet()){\n  lowercaseMap.put(entry.getKey().toLowerCase(), entry.getValue());\n}\nctx.json.event.Headers = lowercaseMap;\n
                rewrite_keys(
                    event,
                    &RewriteKeys::new(
                        "json.event.Headers".into(),
                        "json.event.Headers".into(),
                        vec![KeyRewriteStep::Lowercase],
                    ),
                );
            }

            let _cond = { !event.has_value("http.request.referrer") };
            if _cond {
                if let Some(v) = event
                    .get("json.event.Headers.referer")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.request.referrer", v)?;
                }
            }

            let _cond = { !event.has_value("url.domain") };
            if _cond {
                if let Some(v) = event
                    .get("json.event.Headers.host")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.domain", v)?;
                }
            }

            let _cond =
                { event.has_value("json.event.HostHTTPRequest") && !event.has_value("url.domain") };
            if _cond {
                if event.has_value("json.event.HostHTTPRequest") {
                    gsub_field(
                        event,
                        "json.event.HostHTTPRequest",
                        "url.domain",
                        cached_regex!("(:[0-9]+)*$"),
                        "",
                    )?;
                }
            }

            let _cond = {
                event.has_value("json.event.HostHTTPRequest")
                    && event
                        .get("json.event.HostHTTPRequest")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(":"))
                            }
                            serde_json::Value::String(s) => s.contains(":"),
                            _ => false,
                        })
                    && !event.has_value("url.port")
            };
            if _cond {
                if event.has_value("json.event.HostHTTPRequest") {
                    gsub_field(
                        event,
                        "json.event.HostHTTPRequest",
                        "_tmp.port",
                        cached_regex!("^.*:"),
                        "",
                    )?;
                }
            }

            if event.has_value("_tmp.port") {
                if let Some(val) = event.get("_tmp.port") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "_tmp.port".into(),
                            message,
                        }
                    })?;
                    event.set("url.port", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("url.domain") {
                    if let Some(val) = event.get("url.domain") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "url.domain".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("url.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            if let Some(v) = event
                .get("json.event.User")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
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

            event.rename("json", "beelzebub")?;

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            event.remove("_tmp");
            event.remove("_config");

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
                event.remove("beelzebub.event.ID");
                event.remove("beelzebub.event.Msg");
                event.remove("beelzebub.event.SourceIp");
                event.remove("beelzebub.event.SourcePort");
                event.remove("beelzebub.event.HeadersMap");
                event.remove("beelzebub.event.HeadersText");
                event.remove("beelzebub.event.HTTPMethod");
                event.remove("beelzebub.event.Body");
                event.remove("beelzebub.event.RequestURI");
                event.remove("beelzebub.event.UserAgent");
                event.remove("beelzebub.event.HostHTTPRequest");
                event.remove("beelzebub.event.User");
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
