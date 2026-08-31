// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `object` pipeline.
pub struct Object;

impl Transform for Object {
    fn name(&self) -> &str {
        "object"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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
                        event.append("error.message", json!(format!("Failed to parse EventDate field: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.EvaluationTime".into(),
                            message,
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
                    event.rename("json.AuthMethodReference", "salesforce.login.auth.method_reference")?;
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
                    event.rename("json.PolicyOutcome", "salesforce.login.transaction_security.policy.outcome")?;
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
                    event.rename("json.RelatedEventIdentifier", "salesforce.login.related_event_identifier")?;
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
                    event.rename("json.PolicyId", "salesforce.login.transaction_security.policy.id")?;
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

            let _cond = { event.get_str("json.Status") != Some("Success") && event.has_value("json.Status") };
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
                        event.append("error.message", json!(format!("Failed to parse CreatedDate field: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                event.append_unique("user.roles", json!(event.get("json.UserType").map_or_else(String::new, template_to_string)))?;
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
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.SourceIp".into(),
                            message,
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

            let _cond = { event.has_value("source.ip") && !event.has_value("source.geo.location.lat") && !event.has_value("source.geo.location.lon") };
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

            let _cond = { event.has_value("tls.version") && event.get_str("tls.version") != Some("Unknown") };
            if _cond {
            let v = json!("tls");
            if !painless_is_empty_value(&v) {
                    event.set("tls.version_protocol", v)?;
            }
            }

                event.remove("json");

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
