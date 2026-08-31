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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if event.has_value("event.original") {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.site_name}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NOTSPACE:destination.domain}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.site_name}) (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|HTTP/%{NUMBER:http.version}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.site_name}) (?:-|%{NOTSPACE:iis.access.server_name}) (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|HTTP/%{NUMBER:http.version}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NOTSPACE:destination.domain}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} \\[%{IPORHOST:destination.address}\\]\\(http://%{IPORHOST:destination.address}\\) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) \\[%{IPORHOST:source.address}\\]\\(http://%{IPORHOST:source.address}\\) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.server_name}) (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    // Grok pattern: %{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.site_name}) (?:-|%{NOTSPACE:iis.access.server_name}) (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:destination.domain}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.site_name}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NOTSPACE:destination.domain}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.site_name}) (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|HTTP/%{NUMBER:http.version}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.site_name}) (?:-|%{NOTSPACE:iis.access.server_name}) (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|HTTP/%{NUMBER:http.version}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NOTSPACE:destination.domain}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:http.response.body.bytes:long}) (?:-|%{NUMBER:http.request.body.bytes:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} \\[%{IPORHOST:destination.address}\\]\\(http://%{IPORHOST:destination.address}\\) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) \\[%{IPORHOST:source.address}\\]\\(http://%{IPORHOST:source.address}\\) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.server_name}) (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:http.request.referrer}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                            cached_grok!(
                                "%{TIMESTAMP_ISO8601:iis.access.time} (?:-|%{NOTSPACE:iis.access.site_name}) (?:-|%{NOTSPACE:iis.access.server_name}) (?:-|%{IPORHOST:destination.address}) (?:-|%{WORD:http.request.method}) (?:-|%{NOTSPACE:_temp_.url_path}) (?:-|%{NOTSPACE:_temp_.url_query}) (?:-|%{NUMBER:destination.port:long}) (?:-|%{NOTSPACE:user.name}) (?:-|%{IPORHOST:source.address}) (?:-|%{NOTSPACE:user_agent.original}) (?:-|%{NOTSPACE:iis.access.cookie}) (?:-|%{NOTSPACE:destination.domain}) (?:-|%{NUMBER:http.response.status_code:long}) (?:-|%{NUMBER:iis.access.sub_status:long}) (?:-|%{NUMBER:iis.access.win32_status:long}) (?:-|%{NUMBER:_temp_.duration:long})( (?:-|%{IP:network.forwarded_ip}))?"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "_temp_.url_path", "url", true, false)?;
                Ok(())
            })();

            let _cond = { event.has_value("_temp_.url_path") && !event.has_value("url.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "url.original",
                        json!(
                            event
                                .get("_temp_.url_path")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("url.query") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_temp_.url_query") {
                        if let Some(s) = event.get_string("_temp_.url_query") {
                            match url_decode(&s) {
                                Some(decoded) => event.set("url.query", json!(decoded))?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_temp_.url_query".into(),
                                        message: format!("cannot url-decode '{s}'"),
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("url.domain") && event.has_value("destination.domain") };
            if _cond {
                event.set(
                    "url.domain",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("@timestamp") {
                event.rename("@timestamp", "event.created")?;
            }

            if let Some(date_str) = event.get_as_string("iis.access.time") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "iis.access.time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("iis.access.time").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "iis.access.time".into(),
                });
            }

            let _cond = { event.has_value("_temp_.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = Math.round(ctx._temp_.duration * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.event.duration = Math.round(ctx._temp_.duration * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1000000}"),
                )?;
            }

            event.remove("_temp_");

            if event.has_value("user_agent.original") {
                if let Some(s) = event.get_string("user_agent.original") {
                    match url_decode(&s) {
                        Some(decoded) => event.set("user_agent.original", json!(decoded))?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "user_agent.original".into(),
                                message: format!("cannot url-decode '{s}'"),
                            });
                        }
                    }
                }
            }

            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
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

            let _cond = { event.has_value("source") };
            if _cond {
                // Painless script
                // Source: for (field in params.fields) {\n    if (ctx.source[field] instanceof List) {\n        def vals = ctx.source[field];\n        vals = vals.stream().distinct().collect(Collectors.toList());\n        if (vals.size() == 1) {\n            ctx.source[field] = vals[0]\n        } else {\n            ctx.source[field] = vals\n        }\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"for (field in params.fields) {\n    if (ctx.source[field] instanceof List) {\n        def vals = ctx.source[field];\n        vals = vals.stream().distinct().collect(Collectors.toList());\n        if (vals.size() == 1) {\n            ctx.source[field] = vals[0]\n        } else {\n            ctx.source[field] = vals\n        }\n    }\n}\n"#
                    ),
                    cached_params!("{\"fields\":[\"address\"]}"),
                )?;
            }

            let _cond = { event.has_value("destination") };
            if _cond {
                // Painless script
                // Source: for (field in params.fields) {\n    if (ctx.destination[field] instanceof List) {\n        def vals = ctx.destination[field];\n        vals = vals.stream().distinct().collect(Collectors.toList());\n        if (vals.size() == 1) {\n            ctx.destination[field] = vals[0]\n        } else {\n            ctx.destination[field] = vals\n        }\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"for (field in params.fields) {\n    if (ctx.destination[field] instanceof List) {\n        def vals = ctx.destination[field];\n        vals = vals.stream().distinct().collect(Collectors.toList());\n        if (vals.size() == 1) {\n            ctx.destination[field] = vals[0]\n        } else {\n            ctx.destination[field] = vals\n        }\n    }\n}\n"#
                    ),
                    cached_params!("{\"fields\":[\"address\"]}"),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("destination.address") {
                    // Grok pattern: (?P<destination_ip>(?:[^%]*))
                    if !cached_grok_mapped!(
                        "(?P<destination_ip>(?:[^%]*))",
                        [("destination_ip", "destination.ip")]
                    )
                    .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: (?P<source_ip>(?:[^%]*))
                    if !cached_grok_mapped!(
                        "(?P<source_ip>(?:[^%]*))",
                        [("source_ip", "source.ip")]
                    )
                    .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
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

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("web"))?;

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                event.append("event.category", json!("network"))?;
            }

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
            if _cond {
                if event.has_value("user.name") {
                    if let Some(input) = event.get_string("user.name") {
                        // Grok pattern: %{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.name}@%{GREEDYDATA:user.domain}
                        // Grok pattern: %{DATA:user.domain}\\\\%{GREEDYDATA:user.name}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "%{DATA:_tmp.user_leading_domain}\\\\%{DATA:user.name}@%{GREEDYDATA:user.domain}"
                                ),
                                cached_grok!("%{DATA:user.name}@%{GREEDYDATA:user.domain}"),
                                cached_grok!("%{DATA:user.domain}\\\\%{GREEDYDATA:user.name}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
            }

            let _cond = { event.has_value("iis.access.server_name") };
            if _cond {
                map_strings(
                    event,
                    "iis.access.server_name",
                    "host.name",
                    str::to_lowercase,
                )?;
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

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
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

            let _cond = {
                event.has_value("http.response.status_code")
                    && event
                        .get_i64("http.response.status_code")
                        .is_some_and(|n| n < 400)
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("http.response.status_code")
                    && event
                        .get_i64("http.response.status_code")
                        .is_some_and(|n| n >= 400)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // Painless script
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_tmp");
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
