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
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "message", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Failed to parse JSON: {}",
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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("message")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("event.original", v)?;
                    }
                    Ok(())
                })();
            }

            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

            let _cond = { event.get_str("event.provider") == Some("Object") };
            if _cond {
                // Begin nested pipeline: "object"
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.EventDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.EventDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to parse EventDate field: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.AuthServiceId") {
                        event.rename("json.AuthServiceId", "salesforce.login.auth.service_id")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.EvaluationTime") {
                        if let Some(val) = event.get("json.EvaluationTime") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.EvaluationTime".into(),
                                    message,
                                }
                            })?;
                            event.set("salesforce.login.evaluation_time", converted)?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.ClientVersion") {
                        event.rename("json.ClientVersion", "salesforce.login.client_version")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.LoginGeoId") {
                        event.rename("json.LoginGeoId", "salesforce.login.geo_id")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.LoginHistoryId") {
                        event.rename("json.LoginHistoryId", "salesforce.login.history_id")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.ApiType") {
                        event.rename("json.ApiType", "salesforce.login.api.type")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.AuthMethodReference") {
                        event.rename(
                            "json.AuthMethodReference",
                            "salesforce.login.auth.method_reference",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.LoginType") {
                        event.rename("json.LoginType", "salesforce.login.type")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.PolicyOutcome") {
                        event.rename(
                            "json.PolicyOutcome",
                            "salesforce.login.transaction_security.policy.outcome",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.ApiVersion") {
                        event.rename("json.ApiVersion", "salesforce.login.api.version")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.EventIdentifier") {
                        event.rename("json.EventIdentifier", "event.id")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.RelatedEventIdentifier") {
                        event.rename(
                            "json.RelatedEventIdentifier",
                            "salesforce.login.related_event_identifier",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.LoginKey") {
                        event.rename("json.LoginKey", "salesforce.login.key")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.Application") {
                        event.rename("json.Application", "salesforce.login.application")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.PolicyId") {
                        event.rename(
                            "json.PolicyId",
                            "salesforce.login.transaction_security.policy.id",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.get_str("json.Status") == Some("Success") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        let v = json!("success");
                        if !painless_is_empty_value(&v) {
                            event.set("event.outcome", v)?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("json.Status") != Some("Success")
                        && event.has_value("json.Status")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        let v = json!("failure");
                        if !painless_is_empty_value(&v) {
                            event.set("event.outcome", v)?;
                        }
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.CreatedDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.CreatedDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to parse CreatedDate field: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.LoginUrl") {
                        event.rename("json.LoginUrl", "event.url")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.Username") {
                        event.rename("json.Username", "user.email")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.UserId") {
                        event.rename("json.UserId", "user.id")?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("json.UserType") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.roles",
                            json!(
                                event
                                    .get("json.UserType")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("json.UserType");
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("json.SourceIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.SourceIp".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.LoginLatitude") {
                        event.rename("json.LoginLatitude", "source.geo.location.lat")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.LoginLongitude") {
                        event.rename("json.LoginLongitude", "source.geo.location.lon")?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("source.ip")
                        && !event.has_value("source.geo.location.lat")
                        && !event.has_value("source.geo.location.lon")
                };
                if _cond {
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
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.CountryIso") {
                        event.rename("json.CountryIso", "source.geo.country_iso_code")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.PostalCode") {
                        event.rename("json.PostalCode", "source.geo.postal_code")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.City") {
                        event.rename("json.City", "source.geo.city_name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.Subdivision") {
                        event.rename("json.Subdivision", "source.geo.region_name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.Country") {
                        event.rename("json.Country", "source.geo.country_name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.Browser") {
                        event.rename("json.Browser", "user_agent.name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.Platform") {
                        event.rename("json.Platform", "user_agent.os.name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.HttpMethod") {
                        event.rename("json.HttpMethod", "http.request.method")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.AdditionalInfo") {
                        event.rename("json.AdditionalInfo", "salesforce.login.additional_info")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.CipherSuite") {
                        event.rename("json.CipherSuite", "tls.cipher")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.TlsProtocol") {
                        if let Some(input) = event.get_string("json.TlsProtocol") {
                            // Grok pattern: ^TLS %{NUMBER:tls.version}$
                            // Grok pattern: ^%{WORD:tls.version}$
                            if !extract_first_match(
                                &[
                                    cached_grok!("^TLS %{NUMBER:tls.version}$"),
                                    cached_grok!("^%{WORD:tls.version}$"),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("tls.version")
                        && event.get_str("tls.version") != Some("Unknown")
                };
                if _cond {
                    let v = json!("tls");
                    if !painless_is_empty_value(&v) {
                        event.set("tls.version_protocol", v)?;
                    }
                }
                event.remove("json");
                // End nested pipeline: "object"
            }

            let _cond = { event.get_str("event.provider") == Some("EventLogFile") };
            if _cond {
                // Begin nested pipeline: "eventlogfile"
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
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to parse TIMESTAMP_DERIVED field: {}",
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
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.salesforce.login.api.type = params.api_type_map.getOrDefault(ctx.salesforce?.login?.api?.type, ctx.salesforce.login.api.type);\n"#
                        ),
                        cached_params!(
                            "{\"api_type_map\":{\"D\":\"Apex Class\",\"E\":\"SOAP Enterprise\",\"I\":\"SOAP Cross Instance\",\"M\":\"SOAP Metadata\",\"O\":\"Old SOAP\",\"P\":\"SOAP Partner\",\"S\":\"SOAP Apex\",\"T\":\"SOAP Tooling\",\"X\":\"XmlRPC\",\"f\":\"Feed\",\"l\":\"Live Agent\",\"p\":\"SOAP ClientSync\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.login.api.type: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("json.AUTHENTICATION_METHOD_REFERENCE") {
                        event.rename(
                            "json.AUTHENTICATION_METHOD_REFERENCE",
                            "salesforce.login.auth.service_id",
                        )?;
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
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.salesforce.login.request.status = params.request_status_map.getOrDefault(ctx.salesforce?.login?.request.status, ctx.salesforce.login.request.status);\n"#
                        ),
                        cached_params!(
                            "{\"request_status_map\":{\"S\":\"Success\",\"F\":\"Failure\",\"U\":\"Undefined\",\"A\":\"Authorization Error\",\"R\":\"Redirect\",\"N\":\"Not Found\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.login.request.status: {}",
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
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.RUN_TIME".into(),
                                    message,
                                }
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
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.CPU_TIME".into(),
                                    message,
                                }
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
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.DB_TOTAL_TIME".into(),
                                    message,
                                }
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
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.salesforce?.login?.db_time?.total != null) {\n    ctx.salesforce.login.db_total_time = ctx.salesforce.login.db_total_time / 1000000;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Failed to set salesforce.login.db_total_time: {}",
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
                let _cond = {
                    event.get_str("json.LOGIN_STATUS") == Some("LOGIN_NO_ERROR")
                        && event.has_value("json.LOGIN_STATUS")
                };
                if _cond {
                    let v = json!("success");
                    if !painless_is_empty_value(&v) {
                        event.set("event.outcome", v)?;
                    }
                }
                let _cond = {
                    event.get_str("json.LOGIN_STATUS") != Some("LOGIN_NO_ERROR")
                        && event.has_value("json.LOGIN_STATUS")
                };
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
                    let v = Value::Array(vec![json!(
                        event
                            .get("json.USER_TYPE")
                            .map_or_else(String::new, template_to_string)
                    )]);
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
                                if let Some(name) = ua.name {
                                    event.set("user_agent.name", json!(name))?;
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
                // End nested pipeline: "eventlogfile"
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("info")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("event");
                if !painless_is_empty_value(&v) {
                    event.set("event.kind", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("login-attempt");
                if !painless_is_empty_value(&v) {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = Value::Array(vec![json!("authentication")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.category", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("salesforce.login");
                if !painless_is_empty_value(&v) {
                    event.set("event.dataset", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("salesforce");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("source.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("salesforce.login.client.ip")
                    && event.get_str("salesforce.login.client.ip") != Some("Salesforce.com IP")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("salesforce.login.client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return ((Map) object).isEmpty();\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return ((List) object).isEmpty();\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("json");
                event.remove("message");
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
                event.set(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set(
                    "error.type",
                    json!(
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
