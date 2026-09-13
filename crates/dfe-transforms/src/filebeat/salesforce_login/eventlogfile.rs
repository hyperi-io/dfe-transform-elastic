// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `eventlogfile` pipeline.
pub struct Eventlogfile;

impl Transform for Eventlogfile {
    fn name(&self) -> &str {
        "eventlogfile"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.TIMESTAMP_DERIVED") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.TIMESTAMP_DERIVED".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append("error.message", json!(format!("Failed to parse TIMESTAMP_DERIVED field: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.API_TYPE") {
                    event.rename("json.API_TYPE", "salesforce.login.api.type")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.salesforce.login.api.type = params.api_type_map.getOrDefault(ctx.salesforce?.login?.api?.type, ctx.salesforce.login.api.type);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"ctx.salesforce.login.api.type = params.api_type_map.getOrDefault(ctx.salesforce?.login?.api?.type, ctx.salesforce.login.api.type);\n"#), cached_params!("{\"api_type_map\":{\"D\":\"Apex Class\",\"E\":\"SOAP Enterprise\",\"I\":\"SOAP Cross Instance\",\"M\":\"SOAP Metadata\",\"O\":\"Old SOAP\",\"P\":\"SOAP Partner\",\"S\":\"SOAP Apex\",\"T\":\"SOAP Tooling\",\"X\":\"XmlRPC\",\"f\":\"Feed\",\"l\":\"Live Agent\",\"p\":\"SOAP ClientSync\"}}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Failed to set salesforce.login.api.type: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.AUTHENTICATION_METHOD_REFERENCE") {
                    event.rename("json.AUTHENTICATION_METHOD_REFERENCE", "salesforce.login.auth.service_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.REQUEST_STATUS") {
                    event.rename("json.REQUEST_STATUS", "salesforce.login.request.status")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: ctx.salesforce.login.request.status = params.request_status_map.getOrDefault(ctx.salesforce?.login?.request.status, ctx.salesforce.login.request.status);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"ctx.salesforce.login.request.status = params.request_status_map.getOrDefault(ctx.salesforce?.login?.request.status, ctx.salesforce.login.request.status);\n"#), cached_params!("{\"request_status_map\":{\"S\":\"Success\",\"F\":\"Failure\",\"U\":\"Undefined\",\"A\":\"Authorization Error\",\"R\":\"Redirect\",\"N\":\"Not Found\"}}"))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Failed to set salesforce.login.request.status: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.API_VERSION") {
                    event.rename("json.API_VERSION", "salesforce.login.api.version")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.USER_ID") {
                    event.rename("json.USER_ID", "salesforce.login.user_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.LOGIN_KEY") {
                    event.rename("json.LOGIN_KEY", "salesforce.login.key")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.EVENT_TYPE") {
                    event.rename("json.EVENT_TYPE", "salesforce.login.event_type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.REQUEST_ID") {
                    event.rename("json.REQUEST_ID", "salesforce.login.request.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.ORGANIZATION_ID") {
                    event.rename("json.ORGANIZATION_ID", "salesforce.login.organization_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.RUN_TIME") {
                if let Some(val) = event.get("json.RUN_TIME") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.RUN_TIME".into(),
                            message,
                        })?;
                    event.set("salesforce.login.run_time", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.CPU_TIME") {
                if let Some(val) = event.get("json.CPU_TIME") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.CPU_TIME".into(),
                            message,
                        })?;
                    event.set("salesforce.login.cpu_time", converted)?;
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.DB_TOTAL_TIME") {
                if let Some(val) = event.get("json.DB_TOTAL_TIME") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.DB_TOTAL_TIME".into(),
                            message,
                        })?;
                    event.set("salesforce.login.db_total_time", converted)?;
                }
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.salesforce?.login?.db_time?.total != null) {\n    ctx.salesforce.login.db_total_time = ctx.salesforce.login.db_total_time / 1000000;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"if (ctx.salesforce?.login?.db_time?.total != null) {\n    ctx.salesforce.login.db_total_time = ctx.salesforce.login.db_total_time / 1000000;\n}\n"#))?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append("error.message", json!(format!("Failed to set salesforce.login.db_total_time: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.CLIENT_IP") {
                    event.rename("json.CLIENT_IP", "salesforce.login.client.ip")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.URI_ID_DERIVED") {
                    event.rename("json.URI_ID_DERIVED", "salesforce.login.uri.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.URI") {
                    event.rename("json.URI", "event.url")?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.LOGIN_STATUS") == Some("LOGIN_NO_ERROR") && event.has_value("json.LOGIN_STATUS") };
            if _cond {
            let v = json!("success");
            if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
            }
            }

            let _cond = { event.get_str("json.LOGIN_STATUS") != Some("LOGIN_NO_ERROR") && event.has_value("json.LOGIN_STATUS") };
            if _cond {
            let v = json!("failure");
            if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
            }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.USER_NAME") {
                    event.rename("json.USER_NAME", "user.email")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.USER_ID_DERIVED") {
                    event.rename("json.USER_ID_DERIVED", "user.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            let v = Value::Array(vec![json!(event.get("json.USER_TYPE").map_or_else(String::new, template_to_string))]);
            if !painless_is_empty_value(&v) {
                    event.set("user.roles", v)?;
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("json.USER_TYPE");
                Ok(())
            })();

            let _cond = { event.get_str("json.SOURCE_IP") != Some("Salesforce.com IP") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.SOURCE_IP") {
                    event.rename("json.SOURCE_IP", "source.ip")?;
                }
                Ok(())
            })();
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.BROWSER_TYPE") {
                if let Some(ua_str) = event.get_string("json.BROWSER_TYPE") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
                        if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                            }
                        }
                        if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                    }
                }
            }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.CIPHER_SUITE") {
                    event.rename("json.CIPHER_SUITE", "tls.cipher")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.TLS_PROTOCOL") {
                if let Some(input) = event.get_string("json.TLS_PROTOCOL") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("v") else { break 'dissect false };
                        captured.push(("tls.version_protocol", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("v") else { break 'dissect false };
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
