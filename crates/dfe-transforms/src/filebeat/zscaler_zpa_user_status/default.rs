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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.LogTimestamp")
                    && event.get_str("json.LogTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.LogTimestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LogTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.LogTimestamp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.LogTimestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.append("event.category", json!("iam"))?;

            event.set("event.kind", json!("state"))?;

            event.append("event.type", json!("info"))?;

            event.append("event.type", json!("user"))?;

            if event.has_value("json.CountryCode") {
                event.rename("json.CountryCode", "client.geo.country_iso_code")?;
            }

            if event.has_value("json.Latitude") {
                event.rename("json.Latitude", "client.geo.location.lat")?;
            }

            if event.has_value("json.Longitude") {
                event.rename("json.Longitude", "client.geo.location.lon")?;
            }

            let _cond = { event.get_str("json.PublicIP") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.PublicIP") {
                        if let Some(val) = event.get("json.PublicIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.PublicIP".into(),
                                    message,
                                }
                            })?;
                            event.set("client.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    if event.remove("json.PublicIP").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.PublicIP".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.Hostname") {
                event.rename("json.Hostname", "host.hostname")?;
            }

            if event.has_value("json.Platform") {
                event.rename("json.Platform", "host.os.platform")?;
            }

            if event.has_value("json.Customer") {
                event.rename("json.Customer", "organization.name")?;
            }

            let _cond = { event.has_value("server.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("server.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.ZENCountryCode") {
                event.rename("json.ZENCountryCode", "server.geo.country_iso_code")?;
            }

            if event.has_value("json.ZENLatitude") {
                event.rename("json.ZENLatitude", "server.geo.location.lat")?;
            }

            if event.has_value("json.ZENLongitude") {
                event.rename("json.ZENLongitude", "server.geo.location.lon")?;
            }

            if event.has_value("json.Username") {
                event.rename("json.Username", "user.name")?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
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
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.CertificateCN") };
            if _cond {
                event.append(
                    "x509.issuer.common_name",
                    json!(
                        event
                            .get("json.CertificateCN")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json.CertificateCN");

            if event.has_value("json.SessionID") {
                event.rename("json.SessionID", "zscaler_zpa.user_status.session.id")?;
            }

            if event.has_value("json.SessionStatus") {
                event.rename(
                    "json.SessionStatus",
                    "zscaler_zpa.user_status.session.status",
                )?;
            }

            if event.has_value("json.Version") {
                event.rename("json.Version", "zscaler_zpa.user_status.version")?;
            }

            if event.has_value("json.ZEN") {
                event.rename("json.ZEN", "zscaler_zpa.user_status.zen.domain")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.PrivateIP") {
                    if let Some(val) = event.get("json.PrivateIP") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.PrivateIP".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zpa.user_status.private_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.PrivateIP").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.PrivateIP".into(),
                    });
                }
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("zscaler_zpa.user_status.private_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("zscaler_zpa.user_status.private_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.TimestampAuthentication")
                    && event.get_str("json.TimestampAuthentication") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampAuthentication") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("zscaler_zpa.user_status.timestamp.authentication", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampAuthentication".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampAuthentication").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampAuthentication".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
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
                event.has_value("json.TimestampAuthentication")
                    && event.get_str("json.TimestampUnAuthentication") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampUnAuthentication") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_status.timestamp.unauthentication",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampUnAuthentication".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampUnAuthentication").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampUnAuthentication".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("json.TotalBytesRx") {
                event.rename(
                    "json.TotalBytesRx",
                    "zscaler_zpa.user_status.total.bytes_rx",
                )?;
            }

            if event.has_value("json.TotalBytesTx") {
                event.rename(
                    "json.TotalBytesTx",
                    "zscaler_zpa.user_status.total.bytes_tx",
                )?;
            }

            if event.has_value("json.Idp") {
                event.rename("json.Idp", "zscaler_zpa.user_status.idp")?;
            }

            if event.has_value("json.ClientType") {
                event.rename("json.ClientType", "zscaler_zpa.user_status.client.type")?;
            }

            if event.has_value("json.TrustedNetworks") {
                event.rename(
                    "json.TrustedNetworks",
                    "zscaler_zpa.user_status.trusted_networks",
                )?;
            }

            if event.has_value("json.TrustedNetworksNames") {
                event.rename(
                    "json.TrustedNetworksNames",
                    "zscaler_zpa.user_status.trusted_networks_names",
                )?;
            }

            let _cond = { event.has_value("json.FQDNRegistered") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.zscaler_zpa.user_status.fqdn = new HashMap();\nctx.zscaler_zpa.user_status.fqdn.registered = (ctx.json.FQDNRegistered != '0');\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.zscaler_zpa.user_status.fqdn = new HashMap();\nctx.zscaler_zpa.user_status.fqdn.registered = (ctx.json.FQDNRegistered != '0');\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.FQDNRegisteredError") {
                event.rename(
                    "json.FQDNRegisteredError",
                    "zscaler_zpa.user_status.fqdn.registered_error",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("json.SAMLAttributes") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set(
                        "zscaler_zpa.user_status.saml_attributes",
                        Value::Array(parts),
                    )?;
                }
                Ok(())
            })();

            let _cond = { event.get("json.PosturesHit").is_some_and(|v| v.is_string()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("json.PosturesHit") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("zscaler_zpa.user_status.postures.hit", Value::Array(parts))?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("json.PosturesMiss")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(s) = event.get_string("json.PosturesMiss") {
                        let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("zscaler_zpa.user_status.postures.miss", Value::Array(parts))?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get("json.PosturesHit").is_some_and(|v| v.is_array()) };
            if _cond {
                event.rename("json.PosturesHit", "zscaler_zpa.user_status.postures.hit")?;
            }

            let _cond = { event.get("json.PosturesMiss").is_some_and(|v| v.is_array()) };
            if _cond {
                event.rename("json.PosturesMiss", "zscaler_zpa.user_status.postures.miss")?;
            }

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

            event.remove("json.LogTimestamp");
            event.remove("json.PublicIP");
            event.remove("json.PrivateIP");
            event.remove("json.TimestampAuthentication");
            event.remove("json.TimestampUnAuthentication");
            event.remove("json.FQDNRegistered");
            event.remove("json.SAMLAttributes");
            event.remove("json.PosturesHit");
            event.remove("json.PosturesMiss");

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa.user_status[m.getKey()] = m.getValue();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa.user_status[m.getKey()] = m.getValue();\n}\n"#
                    ),
                )?;
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
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
