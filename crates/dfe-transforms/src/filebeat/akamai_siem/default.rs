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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
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

            parse_json_field(event, "event.original", "json")?;

            let _cond = { event.has_value("json.offset") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("observer.vendor", json!("akamai"))?;

            event.set("observer.type", json!("proxy"))?;

            let _cond = { event.has_value("json.httpMessage.start") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.httpMessage.start") {
                    match parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.httpMessage.start".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if event.has_value("json.httpMessage.status") {
                event.rename("json.httpMessage.status", "http.response.status_code")?;
            }

            if event.has_value("http.response.status_code") {
                if let Some(val) = event.get("http.response.status_code") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "http.response.status_code".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.status_code", converted)?;
                }
            }

            if event.has_value("json.httpMessage.bytes") {
                event.rename("json.httpMessage.bytes", "http.response.bytes")?;
            }

            if event.has_value("http.response.bytes") {
                if let Some(val) = event.get("http.response.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "http.response.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.bytes", converted)?;
                }
            }

            if event.has_value("json.httpMessage.requestId") {
                event.rename("json.httpMessage.requestId", "http.request.id")?;
            }

            if let Some(v) = event
                .get("http.request.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.original") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "event.original".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            let _cond = { event.has_value("json.httpMessage.start") && event.has_value("_id") };
            if _cond {
                event.set(
                    "_id",
                    json!(format!(
                        "{}-{}",
                        event
                            .get("json.httpMessage.start")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if event.has_value("json.httpMessage.method") {
                event.rename("json.httpMessage.method", "http.request.method")?;
            }

            if event.has_value("json.httpMessage.host") {
                event.rename("json.httpMessage.host", "url.domain")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.httpMessage.path") {
                    if let Some(s) = event.get_string("json.httpMessage.path") {
                        match url_decode(&s) {
                            Some(decoded) => event.set("url.path", json!(decoded))?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.httpMessage.path".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "urldecode_httpMessage_path",
                )?;
                if let Some(v) = event.get("json.httpMessage.path").cloned() {
                    event.set("url.path", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.httpMessage.query") {
                    if let Some(s) = event.get_string("json.httpMessage.query") {
                        match url_decode(&s) {
                            Some(decoded) => event.set("url.query", json!(decoded))?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.httpMessage.query".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "urldecode")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "urldecode_httpMessage_query",
                )?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("json.httpMessage.query", "url.query")?;
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.httpMessage.port") {
                event.rename("json.httpMessage.port", "url.port")?;
            }

            if event.has_value("url.port") {
                if let Some(val) = event.get("url.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "url.port".into(),
                            message,
                        }
                    })?;
                    event.set("url.port", converted)?;
                }
            }

            if event.has_value("json.httpMessage.responseHeaders") {
                if let Some(s) = event.get_string("json.httpMessage.responseHeaders") {
                    match url_decode(&s) {
                        Some(decoded) => {
                            event.set("json.httpMessage.responseHeaders", json!(decoded))?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.httpMessage.responseHeaders".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("json.httpMessage.responseHeaders")
                    .is_some_and(|v| v.is_string())
                    && !(event
                        .get("json.httpMessage.responseHeaders")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(":"))
                            }
                            serde_json::Value::String(s) => s.contains(":"),
                            _ => false,
                        }))
            };
            if _cond {
                if event.remove("json.httpMessage.responseHeaders").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.httpMessage.responseHeaders".into(),
                    });
                }
            }

            if event.has_value("json.httpMessage.responseHeaders") {
                gsub_field(
                    event,
                    "json.httpMessage.responseHeaders",
                    "json.httpMessage.responseHeaders",
                    cached_regex!("\\r\\n[\\t ]+"),
                    " ",
                )?;
            }

            let _cond = {
                event.has_value("json.httpMessage.responseHeaders")
                    && event.get_str("json.httpMessage.responseHeaders") != Some("")
            };
            if _cond {
                if event.has_value("json.httpMessage.responseHeaders") {
                    if let Some(kv_str) = event.get_string("json.httpMessage.responseHeaders") {
                        for pair in cached_regex!("\\r\\n").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = ({
                                let parts = cached_regex!(":\\s*").splitn(&pair, 2);
                                match (parts.first(), parts.get(1)) {
                                    (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                    _ => None,
                                }
                            }) else {
                                return Err(TransformError::ParseError {
                                    path: "json.httpMessage.responseHeaders".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                let key = &key[..];
                                let key = key.trim_matches(|c| " ".contains(c));
                                let value = value.trim_matches(|c| " ".contains(c));
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("akamai.siem.response.headers.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
            }

            if event.has_value("json.httpMessage.requestHeaders") {
                if let Some(s) = event.get_string("json.httpMessage.requestHeaders") {
                    match url_decode(&s) {
                        Some(decoded) => {
                            event.set("json.httpMessage.requestHeaders", json!(decoded))?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.httpMessage.requestHeaders".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("json.httpMessage.requestHeaders")
                    .is_some_and(|v| v.is_string())
                    && !(event
                        .get("json.httpMessage.requestHeaders")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(":"))
                            }
                            serde_json::Value::String(s) => s.contains(":"),
                            _ => false,
                        }))
            };
            if _cond {
                if event.remove("json.httpMessage.requestHeaders").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.httpMessage.requestHeaders".into(),
                    });
                }
            }

            if event.has_value("json.httpMessage.requestHeaders") {
                gsub_field(
                    event,
                    "json.httpMessage.requestHeaders",
                    "json.httpMessage.requestHeaders",
                    cached_regex!("\\r\\n[\\t ]+"),
                    " ",
                )?;
            }

            let _cond = {
                event.has_value("json.httpMessage.requestHeaders")
                    && event.get_str("json.httpMessage.requestHeaders") != Some("")
            };
            if _cond {
                if event.has_value("json.httpMessage.requestHeaders") {
                    if let Some(kv_str) = event.get_string("json.httpMessage.requestHeaders") {
                        for pair in cached_regex!("\\r\\n").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = ({
                                let parts = cached_regex!(":\\s*").splitn(&pair, 2);
                                match (parts.first(), parts.get(1)) {
                                    (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                    _ => None,
                                }
                            }) else {
                                return Err(TransformError::ParseError {
                                    path: "json.httpMessage.requestHeaders".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                let key = &key[..];
                                let key = key.trim_matches(|c| " ".contains(c));
                                let value = value.trim_matches(|c| " ".contains(c));
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("akamai.siem.request.headers.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
            }

            // Painless script
            // Source: String full = '';\nif (ctx.url?.scheme != null && ctx.url.scheme != \"\") {\n  full += ctx.url.scheme+\"://\";\n}\nif (ctx.url?.domain != null && ctx.url.domain != \"\") {\n  full += ctx.url.domain;\n}\nif (ctx.json.httpMessage?.path != null && ctx.json.httpMessage.path != \"\") {\n  full += ctx.json.httpMessage.path;\n}\nif (ctx.json.httpMessage?.query != null && ctx.json.httpMessage.query != \"\") {\n  full += \"?\"+ctx.json.httpMessage.query;\n}\nif (full != \"\") {\n  if (ctx.url == null) {\n    ctx.url = [:];\n  }\n  ctx.url.full = full\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"String full = '';\nif (ctx.url?.scheme != null && ctx.url.scheme != \"\") {\n  full += ctx.url.scheme+\"://\";\n}\nif (ctx.url?.domain != null && ctx.url.domain != \"\") {\n  full += ctx.url.domain;\n}\nif (ctx.json.httpMessage?.path != null && ctx.json.httpMessage.path != \"\") {\n  full += ctx.json.httpMessage.path;\n}\nif (ctx.json.httpMessage?.query != null && ctx.json.httpMessage.query != \"\") {\n  full += \"?\"+ctx.json.httpMessage.query;\n}\nif (full != \"\") {\n  if (ctx.url == null) {\n    ctx.url = [:];\n  }\n  ctx.url.full = full\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("json.httpMessage.protocol") {
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.httpMessage.tls") {
                    if let Some(input) = event.get_string("json.httpMessage.tls") {
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
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("tls.version_protocol") {
                map_strings(
                    event,
                    "tls.version_protocol",
                    "tls.version_protocol",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.attackData.clientIP") {
                event.rename("json.attackData.clientIP", "source.address")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })();

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
                if event.has_value("json.geo.country") {
                    event.rename("json.geo.country", "source.geo.country_iso_code")?;
                }
            }

            let _cond = { !event.has_value("source.geo.region_iso_code") };
            if _cond {
                let v = json!(format!(
                    "{}-{}",
                    event
                        .get("json.geo.country")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("json.geo.regionCode")
                        .map_or_else(String::new, template_to_string)
                ));
                if !painless_is_empty_value(&v) {
                    event.set("source.geo.region_iso_code", v)?;
                }
            }

            let _cond = { !event.has_value("source.geo.city_name") };
            if _cond {
                if event.has_value("json.geo.city") {
                    event.rename("json.geo.city", "source.geo.city_name")?;
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

            let _cond = {
                event.get_str("json.geo.asn") != Some("") && !event.has_value("source.as.number")
            };
            if _cond {
                if event.has_value("json.geo.asn") {
                    if let Some(val) = event.get("json.geo.asn") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.geo.asn".into(),
                                message,
                            }
                        })?;
                        event.set("source.as.number", converted)?;
                    }
                }
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("json.attackData.ruleActions") {
                if let Some(s) = event.get_string("json.attackData.ruleActions") {
                    match url_decode(&s) {
                        Some(decoded) => {
                            event.set("json.attackData.ruleActions", json!(decoded))?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attackData.ruleActions".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.attackData.ruleActions") {
                if let Some(s) = event.get_string("json.attackData.ruleActions") {
                    let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    event.set("json.attackData.ruleActions", Value::Array(parts))?;
                }
            }

            if event.has_value("json.attackData.ruleData") {
                if let Some(s) = event.get_string("json.attackData.ruleData") {
                    match url_decode(&s) {
                        Some(decoded) => event.set("json.attackData.ruleData", json!(decoded))?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attackData.ruleData".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.attackData.ruleData") {
                if let Some(s) = event.get_string("json.attackData.ruleData") {
                    let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    event.set("json.attackData.ruleData", Value::Array(parts))?;
                }
            }

            if event.has_value("json.attackData.ruleMessages") {
                if let Some(s) = event.get_string("json.attackData.ruleMessages") {
                    match url_decode(&s) {
                        Some(decoded) => {
                            event.set("json.attackData.ruleMessages", json!(decoded))?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attackData.ruleMessages".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.attackData.ruleMessages") {
                if let Some(s) = event.get_string("json.attackData.ruleMessages") {
                    let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    event.set("json.attackData.ruleMessages", Value::Array(parts))?;
                }
            }

            if event.has_value("json.attackData.ruleSelectors") {
                if let Some(s) = event.get_string("json.attackData.ruleSelectors") {
                    match url_decode(&s) {
                        Some(decoded) => {
                            event.set("json.attackData.ruleSelectors", json!(decoded))?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attackData.ruleSelectors".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.attackData.ruleSelectors") {
                if let Some(s) = event.get_string("json.attackData.ruleSelectors") {
                    let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    event.set("json.attackData.ruleSelectors", Value::Array(parts))?;
                }
            }

            if event.has_value("json.attackData.ruleTags") {
                if let Some(s) = event.get_string("json.attackData.ruleTags") {
                    match url_decode(&s) {
                        Some(decoded) => event.set("json.attackData.ruleTags", json!(decoded))?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attackData.ruleTags".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.attackData.ruleTags") {
                if let Some(s) = event.get_string("json.attackData.ruleTags") {
                    let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    event.set("json.attackData.ruleTags", Value::Array(parts))?;
                }
            }

            if event.has_value("json.attackData.ruleVersions") {
                if let Some(s) = event.get_string("json.attackData.ruleVersions") {
                    match url_decode(&s) {
                        Some(decoded) => {
                            event.set("json.attackData.ruleVersions", json!(decoded))?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attackData.ruleVersions".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.attackData.ruleVersions") {
                if let Some(s) = event.get_string("json.attackData.ruleVersions") {
                    let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    event.set("json.attackData.ruleVersions", Value::Array(parts))?;
                }
            }

            if event.has_value("json.attackData.rules") {
                if let Some(s) = event.get_string("json.attackData.rules") {
                    match url_decode(&s) {
                        Some(decoded) => event.set("json.attackData.rules", json!(decoded))?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attackData.rules".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.attackData.rules") {
                if let Some(s) = event.get_string("json.attackData.rules") {
                    let parts: Vec<Value> = s.split(";").map(|p| json!(p)).collect();
                    event.set("json.attackData.rules", Value::Array(parts))?;
                }
            }

            let _cond = {
                event
                    .get("json.attackData.rules")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: ArrayList rules_array = new ArrayList();\nArrayList rule_actions = new ArrayList();\nArrayList rule_tags = new ArrayList();\nfor (def i = 0; i < ctx.json.attackData.rules.length; i++) {\n  HashMap map = new HashMap();\n  for (String key: params.items) {\n    if (i < ctx.json.attackData[key].length) {\n      String data = ctx.json.attackData[key][i].replace(\" \", \"\");\n      try {\n        String value = data.decodeBase64();\n        map.put(key, value);\n        if (key == \"ruleTags\") {\n          rule_tags.add(value.toLowerCase());\n        } else if (key == \"ruleActions\") {\n          rule_actions.add(value.toLowerCase());\n        }\n      }\n      catch (Exception e) {\n        if (data.length() > 10) {\n          data = data.substring(0,10)+\"...\"\n        }\n        String error = e.toString();\n        if (error.startsWith(\"java.lang.IllegalArgumentException: \")) {\n          error = error.substring(\"java.lang.IllegalArgumentException: \".length());\n        }\n        String warning = \"failed to decode base64 data: \" + error + \": \" + data;\n        map.put(key, warning);\n      }\n    }\n  }\n  rules_array.add(map);\n}\nctx.akamai.siem.rules = rules_array;\nctx._rule_actions = rule_actions;\nctx._rule_tags = rule_tags;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ArrayList rules_array = new ArrayList();\nArrayList rule_actions = new ArrayList();\nArrayList rule_tags = new ArrayList();\nfor (def i = 0; i < ctx.json.attackData.rules.length; i++) {\n  HashMap map = new HashMap();\n  for (String key: params.items) {\n    if (i < ctx.json.attackData[key].length) {\n      String data = ctx.json.attackData[key][i].replace(\" \", \"\");\n      try {\n        String value = data.decodeBase64();\n        map.put(key, value);\n        if (key == \"ruleTags\") {\n          rule_tags.add(value.toLowerCase());\n        } else if (key == \"ruleActions\") {\n          rule_actions.add(value.toLowerCase());\n        }\n      }\n      catch (Exception e) {\n        if (data.length() > 10) {\n          data = data.substring(0,10)+\"...\"\n        }\n        String error = e.toString();\n        if (error.startsWith(\"java.lang.IllegalArgumentException: \")) {\n          error = error.substring(\"java.lang.IllegalArgumentException: \".length());\n        }\n        String warning = \"failed to decode base64 data: \" + error + \": \" + data;\n        map.put(key, warning);\n      }\n    }\n  }\n  rules_array.add(map);\n}\nctx.akamai.siem.rules = rules_array;\nctx._rule_actions = rule_actions;\nctx._rule_tags = rule_tags;\n"#
                    ),
                    cached_params!(
                        "{\"items\":[\"rules\",\"ruleActions\",\"ruleData\",\"ruleMessages\",\"ruleTags\",\"ruleSelectors\",\"ruleVersions\"]}"
                    ),
                )?;
            }

            if event.has_value("_rule_actions") {
                foreach_array(event, "_rule_actions", |event| {
                    event.append_unique(
                        "akamai.siem.rule_actions",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_rule_actions");
                Ok(())
            })();

            if event.has_value("_rule_tags") {
                foreach_array(event, "_rule_tags", |event| {
                    event.append_unique(
                        "akamai.siem.rule_tags",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_rule_tags");
                Ok(())
            })();

            if event.has_value("json.attackData.configId") {
                event.rename("json.attackData.configId", "akamai.siem.config_id")?;
            }

            if event.has_value("json.attackData.policyId") {
                event.rename("json.attackData.policyId", "akamai.siem.policy_id")?;
            }

            if event.has_value("json.attackData.policyId") {
                event.rename("json.attackData.policyId", "akamai.siem.policy_id")?;
            }

            if event.has_value("json.attackData.slowPostAction") {
                event.rename(
                    "json.attackData.slowPostAction",
                    "akamai.siem.slow_post_action",
                )?;
            }

            let _cond = { event.get_str("json.attackData.slowPostRate") != Some("") };
            if _cond {
                if event.has_value("json.attackData.slowPostRate") {
                    if let Some(val) = event.get("json.attackData.slowPostRate") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attackData.slowPostRate".into(),
                                message,
                            }
                        })?;
                        event.set("akamai.siem.slow_post_rate", converted)?;
                    }
                }
            }

            if event.has_value("json.attackData.clientReputation") {
                event.rename(
                    "json.attackData.clientReputation",
                    "akamai.siem.client_reputation",
                )?;
            }

            if event.has_value("json.attackData.clientReputation") {
                event.rename(
                    "json.attackData.clientReputation",
                    "akamai.siem.client_reputation",
                )?;
            }

            let _cond = { event.get_str("json.botData.botScore") != Some("") };
            if _cond {
                if event.has_value("json.botData.botScore") {
                    if let Some(val) = event.get("json.botData.botScore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.botData.botScore".into(),
                                message,
                            }
                        })?;
                        event.set("akamai.siem.bot.score", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("json.botData.responseSegment") != Some("") };
            if _cond {
                if event.has_value("json.botData.responseSegment") {
                    if let Some(val) = event.get("json.botData.responseSegment") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.botData.responseSegment".into(),
                                message,
                            }
                        })?;
                        event.set("akamai.siem.bot.response_segment", converted)?;
                    }
                }
            }

            if event.has_value("json.clientData.appBundleId") {
                event.rename(
                    "json.clientData.appBundleId",
                    "akamai.siem.client_data.app_bundle_id",
                )?;
            }

            if event.has_value("json.clientData.appVersion") {
                event.rename(
                    "json.clientData.appVersion",
                    "akamai.siem.client_data.app_version",
                )?;
            }

            let _cond = { event.get_str("json.clientData.telemetryType") != Some("") };
            if _cond {
                if event.has_value("json.clientData.telemetryType") {
                    if let Some(val) = event.get("json.clientData.telemetryType") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.clientData.telemetryType".into(),
                                message,
                            }
                        })?;
                        event.set("akamai.siem.client_data.telemetry_type", converted)?;
                    }
                }
            }

            if event.has_value("json.clientData.sdkVersion") {
                event.rename(
                    "json.clientData.sdkVersion",
                    "akamai.siem.client_data.sdk_version",
                )?;
            }

            if event.has_value("json.identity.ja4") {
                event.rename("json.identity.ja4", "akamai.siem.identity.ja4")?;
            }

            if event.has_value("json.identity.tlsFingerprintV2") {
                event.rename(
                    "json.identity.tlsFingerprintV2",
                    "akamai.siem.identity.tls_fingerprint_v2",
                )?;
            }

            if event.has_value("json.identity.tlsFingerprintV3") {
                event.rename(
                    "json.identity.tlsFingerprintV3",
                    "akamai.siem.identity.tls_fingerprint_v3",
                )?;
            }

            if event.has_value("json.userRiskData.uuid") {
                event.rename("json.userRiskData.uuid", "akamai.siem.user_risk.uuid")?;
            }

            let _cond = { event.get_str("json.userRiskData.status") != Some("") };
            if _cond {
                if event.has_value("json.userRiskData.status") {
                    if let Some(val) = event.get("json.userRiskData.status") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.userRiskData.status".into(),
                                message,
                            }
                        })?;
                        event.set("akamai.siem.user_risk.status", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("json.userRiskData.score") != Some("") };
            if _cond {
                if event.has_value("json.userRiskData.score") {
                    if let Some(val) = event.get("json.userRiskData.score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.userRiskData.score".into(),
                                message,
                            }
                        })?;
                        event.set("akamai.siem.user_risk.score", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("json.userRiskData.allow") != Some("") };
            if _cond {
                if event.has_value("json.userRiskData.allow") {
                    if let Some(val) = event.get("json.userRiskData.allow") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.userRiskData.allow".into(),
                                message,
                            }
                        })?;
                        event.set("akamai.siem.user_risk.allow", converted)?;
                    }
                }
            }

            let _cond = { event.get_str("json.userRiskData.risk") != Some("") };
            if _cond {
                if event.has_value("json.userRiskData.risk") {
                    if let Some(kv_str) = event.get_string("json.userRiskData.risk") {
                        for pair in cached_regex!("\\|").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once(":") else {
                                return Err(TransformError::ParseError {
                                    path: "json.userRiskData.risk".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("akamai.siem.user_risk.risk.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
            }

            let _cond = { event.get_str("json.userRiskData.trust") != Some("") };
            if _cond {
                if event.has_value("json.userRiskData.trust") {
                    if let Some(kv_str) = event.get_string("json.userRiskData.trust") {
                        for pair in cached_regex!("\\|").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once(":") else {
                                return Err(TransformError::ParseError {
                                    path: "json.userRiskData.trust".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("akamai.siem.user_risk.trust.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("json.userRiskData.general")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("json.userRiskData.general") != Some("")
            };
            if _cond {
                // Painless script
                // Source: String text = ctx.json.userRiskData.general.trim();\nif (text != \"\") {\n  def m = new HashMap();\n  for (String f: /\\|/.split(text)) {\n    if (f == \"\") {\n      continue;\n    }\n    int idx = f.indexOf(':');\n    if (idx == -1) {\n      m.put(f, \"-\"); // Include a non-empty string to prevent the field being removed.\n      continue;\n    }\n    String k = f.substring(0, idx);\n    String v = f.substring(idx+1);\n    m.put(k, v);\n  }\n  if (m.size() > 0) {\n    if (ctx.akamai == null) {\n      ctx.akamai = new HashMap();\n    }\n    if (ctx.akamai.siem == null) {\n      ctx.akamai.siem = new HashMap();\n    }\n    if (ctx.akamai.siem.user_risk == null) {\n      ctx.akamai.siem.user_risk = new HashMap();\n    }\n    ctx.akamai.siem.user_risk.general = m;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String text = ctx.json.userRiskData.general.trim();\nif (text != \"\") {\n  def m = new HashMap();\n  for (String f: /\\|/.split(text)) {\n    if (f == \"\") {\n      continue;\n    }\n    int idx = f.indexOf(':');\n    if (idx == -1) {\n      m.put(f, \"-\"); // Include a non-empty string to prevent the field being removed.\n      continue;\n    }\n    String k = f.substring(0, idx);\n    String v = f.substring(idx+1);\n    m.put(k, v);\n  }\n  if (m.size() > 0) {\n    if (ctx.akamai == null) {\n      ctx.akamai = new HashMap();\n    }\n    if (ctx.akamai.siem == null) {\n      ctx.akamai.siem = new HashMap();\n    }\n    if (ctx.akamai.siem.user_risk == null) {\n      ctx.akamai.siem.user_risk = new HashMap();\n    }\n    ctx.akamai.siem.user_risk.general = m;\n  }\n}\n"#
                    ),
                )?;
            }

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("source.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(v) = event.get("source").cloned() {
                event.set("client", v)?;
            }

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.kind", json!("event"))?;

            event.remove("json");
            event.remove("_tmp");

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n"#
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
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
