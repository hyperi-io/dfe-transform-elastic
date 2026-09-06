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

            parse_json_field(event, "event.original", "json")?;

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

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            event.append("event.type", json!("user"))?;

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

            if event.has_value("json.ClientCountryCode") {
                event.rename("json.ClientCountryCode", "client.geo.country_iso_code")?;
            }

            if event.has_value("json.ClientLatitude") {
                event.rename("json.ClientLatitude", "client.geo.location.lat")?;
            }

            if event.has_value("json.ClientLongitude") {
                event.rename("json.ClientLongitude", "client.geo.location.lon")?;
            }

            let _cond = { event.get_str("json.ClientPublicIP") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ClientPublicIP") {
                        if let Some(val) = event.get("json.ClientPublicIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ClientPublicIP".into(),
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
                    if event.remove("json.ClientPublicIP").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.ClientPublicIP".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.Host") {
                    if let Some(val) = event.get("json.Host") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Host".into(),
                                message,
                            }
                        })?;
                        event.set("json.Host", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.rename("json.Host", "host.domain")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.Host") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("json.Host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.Host") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("json.Host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("host.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.IPProtocol") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.network = ctx.network ?: [:];\nctx.network.transport = params.iana_numbers[ctx.json.IPProtocol.toString()];
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.network = ctx.network ?: [:];\nctx.network.transport = params.iana_numbers[ctx.json.IPProtocol.toString()];"#
                        ),
                        cached_params!(
                            "{\"iana_numbers\":{\"0\":\"unknown_ip_ipprotocol\",\"1\":\"icmp\",\"2\":\"igmp\",\"6\":\"tcp\",\"17\":\"udp\",\"41\":\"ip6in4\",\"47\":\"gre\",\"50\":\"esp\",\"58\":\"icmp6\",\"88\":\"eigrp\",\"97\":\"etherip\",\"103\":\"pim\",\"112\":\"vrrp\",\"132\":\"sctp\"}}"
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.Customer") {
                event.rename("json.Customer", "organization.name")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ServerIP") {
                    if let Some(val) = event.get("json.ServerIP") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ServerIP".into(),
                                message,
                            }
                        })?;
                        event.set("server.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ServerIP").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ServerIP".into(),
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

            let _cond = { event.has_value("server.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("server.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ServerPort") {
                    if let Some(val) = event.get("json.ServerPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ServerPort".into(),
                                message,
                            }
                        })?;
                        event.set("server.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ServerPort").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ServerPort".into(),
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

            if event.has_value("json.ClientZEN") {
                event.rename(
                    "json.ClientZEN",
                    "zscaler_zpa.user_activity.zen.client.domain",
                )?;
            }

            let _cond = { event.has_value("zscaler_zpa.user_activity.zen.client.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("zscaler_zpa.user_activity.zen.client.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.ConnectorZEN") {
                event.rename(
                    "json.ConnectorZEN",
                    "zscaler_zpa.user_activity.zen.connector.domain",
                )?;
            }

            let _cond = { event.has_value("zscaler_zpa.user_activity.zen.connector.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("zscaler_zpa.user_activity.zen.connector.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.SessionID") {
                event.rename("json.SessionID", "zscaler_zpa.user_activity.session_id")?;
            }

            if event.has_value("json.ConnectionID") {
                event.rename(
                    "json.ConnectionID",
                    "zscaler_zpa.user_activity.connection.id",
                )?;
            }

            if event.has_value("json.InternalReason") {
                event.rename(
                    "json.InternalReason",
                    "zscaler_zpa.user_activity.internal_reason",
                )?;
            }

            if event.has_value("json.ConnectionStatus") {
                event.rename(
                    "json.ConnectionStatus",
                    "zscaler_zpa.user_activity.connection.status",
                )?;
            }

            if event.has_value("json.DoubleEncryption") {
                event.rename(
                    "json.DoubleEncryption",
                    "zscaler_zpa.user_activity.double_encryption",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ServicePort") {
                    if let Some(val) = event.get("json.ServicePort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ServicePort".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zpa.user_activity.service_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ServicePort").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ServicePort".into(),
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ClientPrivateIP") {
                    if let Some(val) = event.get("json.ClientPrivateIP") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ClientPrivateIP".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zpa.user_activity.client_private_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ClientPrivateIP").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ClientPrivateIP".into(),
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

            let _cond = { event.has_value("zscaler_zpa.user_activity.client_private_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("zscaler_zpa.user_activity.client_private_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.Policy") {
                event.rename("json.Policy", "zscaler_zpa.user_activity.policy.name")?;
            }

            if event.has_value("json.Connector") {
                event.rename("json.Connector", "zscaler_zpa.user_activity.connector.name")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ConnectorIP") {
                    if let Some(val) = event.get("json.ConnectorIP") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ConnectorIP".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zpa.user_activity.connector.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ConnectorIP").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ConnectorIP".into(),
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

            let _cond = { event.has_value("zscaler_zpa.user_activity.connector.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("zscaler_zpa.user_activity.connector.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ConnectorPort") {
                    if let Some(val) = event.get("json.ConnectorPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ConnectorPort".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zpa.user_activity.connector.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.ConnectorPort").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.ConnectorPort".into(),
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

            if event.has_value("json.Application") {
                event.rename("json.Application", "zscaler_zpa.user_activity.application")?;
            }

            if event.has_value("json.AppGroup") {
                event.rename("json.AppGroup", "zscaler_zpa.user_activity.app_group")?;
            }

            if event.has_value("json.Server") {
                event.rename("json.Server", "zscaler_zpa.user_activity.server")?;
            }

            if event.has_value("json.PolicyProcessingTime") {
                event.rename(
                    "json.PolicyProcessingTime",
                    "zscaler_zpa.user_activity.policy.processing_time",
                )?;
            }

            if event.has_value("json.CAProcessingTime") {
                event.rename(
                    "json.CAProcessingTime",
                    "zscaler_zpa.user_activity.ca_processing_time",
                )?;
            }

            if event.has_value("json.ConnectorZENSetupTime") {
                event.rename(
                    "json.ConnectorZENSetupTime",
                    "zscaler_zpa.user_activity.connector_zen_setup_time",
                )?;
            }

            if event.has_value("json.ConnectionSetupTime") {
                event.rename(
                    "json.ConnectionSetupTime",
                    "zscaler_zpa.user_activity.connection.setup_time",
                )?;
            }

            if event.has_value("json.ServerSetupTime") {
                event.rename(
                    "json.ServerSetupTime",
                    "zscaler_zpa.user_activity.server_setup_time",
                )?;
            }

            if event.has_value("json.AppLearnTime") {
                event.rename(
                    "json.AppLearnTime",
                    "zscaler_zpa.user_activity.app_learn_time",
                )?;
            }

            let _cond = {
                event.has_value("json.TimestampConnectionStart")
                    && event.get_str("json.TimestampConnectionStart") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampConnectionStart") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.connection.start",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampConnectionStart".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampConnectionStart").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampConnectionStart".into(),
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
                event.has_value("json.TimestampConnectionEnd")
                    && event.get_str("json.TimestampConnectionEnd") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampConnectionEnd") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.connection.end",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampConnectionEnd".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampConnectionEnd").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampConnectionEnd".into(),
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
                event.has_value("json.TimestampCATx")
                    && event.get_str("json.TimestampCATx") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampCATx") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("zscaler_zpa.user_activity.timestamp.ca.tx", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampCATx".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampCATx").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampCATx".into(),
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
                event.has_value("json.TimestampCARx")
                    && event.get_str("json.TimestampCARx") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampCARx") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("zscaler_zpa.user_activity.timestamp.ca.rx", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampCARx".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampCARx").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampCARx".into(),
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
                event.has_value("json.TimestampAppLearnStart")
                    && event.get_str("json.TimestampAppLearnStart") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampAppLearnStart") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.app_learn_start",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampAppLearnStart".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampAppLearnStart").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampAppLearnStart".into(),
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
                event.has_value("json.TimestampZENFirstRxClient")
                    && event.get_str("json.TimestampZENFirstRxClient") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampZENFirstRxClient") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.zen.client.rx.first",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampZENFirstRxClient".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampZENFirstRxClient").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampZENFirstRxClient".into(),
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
                event.has_value("json.TimestampZENFirstTxClient")
                    && event.get_str("json.TimestampZENFirstTxClient") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampZENFirstTxClient") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.zen.client.tx.first",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampZENFirstTxClient".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampZENFirstTxClient").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampZENFirstTxClient".into(),
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
                event.has_value("json.TimestampZENLastRxClient")
                    && event.get_str("json.TimestampZENLastRxClient") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampZENLastRxClient") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.zen.client.rx.last",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampZENLastRxClient".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampZENLastRxClient").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampZENLastRxClient".into(),
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
                event.has_value("json.TimestampZENLastTxClient")
                    && event.get_str("json.TimestampZENLastTxClient") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampZENLastTxClient") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.zen.client.tx.last",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampZENLastTxClient".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampZENLastTxClient").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampZENLastTxClient".into(),
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
                event.has_value("json.TimestampConnectorZENSetupComplete")
                    && event.get_str("json.TimestampConnectorZENSetupComplete") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.TimestampConnectorZENSetupComplete")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.connector_zen.setup_complete",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampConnectorZENSetupComplete".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("json.TimestampConnectorZENSetupComplete")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampConnectorZENSetupComplete".into(),
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
                event.has_value("json.TimestampZENFirstRxConnector")
                    && event.get_str("json.TimestampZENFirstRxConnector") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampZENFirstRxConnector")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.zen.connector.rx.first",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampZENFirstRxConnector".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampZENFirstRxConnector").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampZENFirstRxConnector".into(),
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
                event.has_value("json.TimestampZENFirstTxConnector")
                    && event.get_str("json.TimestampZENFirstTxConnector") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampZENFirstTxConnector")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.zen.connector.tx.first",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampZENFirstTxConnector".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampZENFirstTxConnector").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampZENFirstTxConnector".into(),
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
                event.has_value("json.TimestampZENLastRxConnector")
                    && event.get_str("json.TimestampZENLastRxConnector") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampZENLastRxConnector")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.zen.connector.rx.last",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampZENLastRxConnector".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampZENLastRxConnector").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampZENLastRxConnector".into(),
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
                event.has_value("json.TimestampZENLastTxConnector")
                    && event.get_str("json.TimestampZENLastTxConnector") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampZENLastTxConnector")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.user_activity.timestamp.zen.connector.tx.last",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimestampZENLastTxConnector".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.TimestampZENLastTxConnector").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.TimestampZENLastTxConnector".into(),
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

            if event.has_value("json.ZENTotalBytesRxClient") {
                event.rename(
                    "json.ZENTotalBytesRxClient",
                    "zscaler_zpa.user_activity.zen.client.total.bytes_rx",
                )?;
            }

            if event.has_value("json.ZENBytesRxClient") {
                event.rename(
                    "json.ZENBytesRxClient",
                    "zscaler_zpa.user_activity.zen.client.bytes_rx",
                )?;
            }

            if event.has_value("json.ZENTotalBytesTxClient") {
                event.rename(
                    "json.ZENTotalBytesTxClient",
                    "zscaler_zpa.user_activity.zen.client.total.bytes_tx",
                )?;
            }

            if event.has_value("json.ZENBytesTxClient") {
                event.rename(
                    "json.ZENBytesTxClient",
                    "zscaler_zpa.user_activity.zen.client.bytes_tx",
                )?;
            }

            if event.has_value("json.ZENTotalBytesRxConnector") {
                event.rename(
                    "json.ZENTotalBytesRxConnector",
                    "zscaler_zpa.user_activity.zen.connector.total.bytes_rx",
                )?;
            }

            if event.has_value("json.ZENBytesRxConnector") {
                event.rename(
                    "json.ZENBytesRxConnector",
                    "zscaler_zpa.user_activity.zen.connector.bytes_rx",
                )?;
            }

            if event.has_value("json.ZENTotalBytesTxConnector") {
                event.rename(
                    "json.ZENTotalBytesTxConnector",
                    "zscaler_zpa.user_activity.zen.connector.total.bytes_tx",
                )?;
            }

            if event.has_value("json.ZENBytesTxConnector") {
                event.rename(
                    "json.ZENBytesTxConnector",
                    "zscaler_zpa.user_activity.zen.connector.bytes_tx",
                )?;
            }

            if event.has_value("json.ClientToClient") {
                event.rename(
                    "json.ClientToClient",
                    "zscaler_zpa.user_activity.client_to_client",
                )?;
            }

            if event.has_value("json.Idp") {
                event.rename("json.Idp", "zscaler_zpa.user_activity.idp")?;
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
            event.remove("json.ClientPublicIP");
            event.remove("json.Host");
            event.remove("json.IPProtocol");
            event.remove("json.ServerIP");
            event.remove("json.ServerPort");
            event.remove("json.ServicePort");
            event.remove("json.ClientPrivateIP");
            event.remove("json.ConnectorIP");
            event.remove("json.ConnectorPort");
            event.remove("json.TimestampConnectionStart");
            event.remove("json.TimestampConnectionEnd");
            event.remove("json.TimestampCATx");
            event.remove("json.TimestampCARx");
            event.remove("json.TimestampAppLearnStart");
            event.remove("json.TimestampZENFirstRxClient");
            event.remove("json.TimestampZENFirstTxClient");
            event.remove("json.TimestampZENLastRxClient");
            event.remove("json.TimestampZENLastTxClient");
            event.remove("json.TimestampConnectorZENSetupComplete");
            event.remove("json.TimestampZENFirstRxConnector");
            event.remove("json.TimestampZENFirstTxConnector");
            event.remove("json.TimestampZENLastRxConnector");
            event.remove("json.TimestampZENLastTxConnector");

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa.user_activity[m.getKey()] = m.getValue();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa.user_activity[m.getKey()] = m.getValue();\n}\n"#
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
