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

            event.append("event.category", json!("package"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.DefRouteGW") {
                    if let Some(val) = event.get("json.DefRouteGW") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.DefRouteGW".into(),
                                message,
                            }
                        })?;
                        event.set("client.nat.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.DefRouteGW").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.DefRouteGW".into(),
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

            let _cond = { event.has_value("client.nat.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.nat.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.CPUUtilization") {
                event.rename("json.CPUUtilization", "host.cpu.usage")?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.TotalBytesTx") {
                    if let Some(val) = event.get("json.TotalBytesTx") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.TotalBytesTx".into(),
                                message,
                            }
                        })?;
                        event.set("host.network.egress.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.TotalBytesTx").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.TotalBytesTx".into(),
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
                if event.has_value("json.TotalBytesRx") {
                    if let Some(val) = event.get("json.TotalBytesRx") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.TotalBytesRx".into(),
                                message,
                            }
                        })?;
                        event.set("host.network.ingress.bytes", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.TotalBytesRx").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.TotalBytesRx".into(),
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

            if event.has_value("json.CountryCode") {
                event.rename("json.CountryCode", "observer.geo.country_iso_code")?;
            }

            if event.has_value("json.Latitude") {
                event.rename("json.Latitude", "observer.geo.location.lat")?;
            }

            if event.has_value("json.Longitude") {
                event.rename("json.Longitude", "observer.geo.location.lon")?;
            }

            if event.has_value("json.InterfaceDefRoute") {
                event.rename(
                    "json.InterfaceDefRoute",
                    "zscaler_zpa.app_connector_status.interface.name",
                )?;
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
                            event.set("json.PublicIP", converted)?;
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

            let _cond = { event.has_value("json.PublicIP") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "observer.ip",
                        json!(
                            event
                                .get("json.PublicIP")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.PublicIP") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("json.PublicIP")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.Platform") {
                event.rename("json.Platform", "observer.os.platform")?;
            }

            if event.has_value("json.Version") {
                event.rename("json.Version", "observer.version")?;
            }

            event.set("observer.type", json!("forwarder"))?;

            if event.has_value("json.Customer") {
                event.rename("json.Customer", "organization.name")?;
            }

            if event.has_value("json.Customer") {
                event.rename("json.Customer", "organization.name")?;
            }

            if event.has_value("json.SessionID") {
                event.rename(
                    "json.SessionID",
                    "zscaler_zpa.app_connector_status.session.id",
                )?;
            }

            if event.has_value("json.SessionType") {
                event.rename(
                    "json.SessionType",
                    "zscaler_zpa.app_connector_status.session.type",
                )?;
            }

            if event.has_value("json.SessionStatus") {
                event.rename(
                    "json.SessionStatus",
                    "zscaler_zpa.app_connector_status.session.status",
                )?;
            }

            if event.has_value("json.ZEN") {
                event.rename("json.ZEN", "zscaler_zpa.app_connector_status.zen")?;
            }

            if event.has_value("json.Connector") {
                event.rename(
                    "json.Connector",
                    "zscaler_zpa.app_connector_status.connector.name",
                )?;
            }

            if event.has_value("json.ConnectorGroup") {
                event.rename(
                    "json.ConnectorGroup",
                    "zscaler_zpa.app_connector_status.connector.group",
                )?;
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
                        event.set("zscaler_zpa.app_connector_status.private_ip", converted)?;
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

            let _cond = { event.has_value("zscaler_zpa.app_connector_status.private_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("zscaler_zpa.app_connector_status.private_ip")
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
                            Some(parsed) => event.set(
                                "zscaler_zpa.app_connector_status.timestamp.authentication",
                                parsed,
                            )?,
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
                event.has_value("json.TimestampUnAuthentication")
                    && event.get_str("json.TimestampUnAuthentication") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimestampUnAuthentication") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.app_connector_status.timestamp.unauthentication",
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

            if event.has_value("json.MemUtilization") {
                event.rename(
                    "json.MemUtilization",
                    "zscaler_zpa.app_connector_status.memory.utilization",
                )?;
            }

            if event.has_value("json.ServiceCount") {
                event.rename(
                    "json.ServiceCount",
                    "zscaler_zpa.app_connector_status.service.count",
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.PrimaryDNSResolver") {
                    if let Some(val) = event.get("json.PrimaryDNSResolver") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.PrimaryDNSResolver".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "zscaler_zpa.app_connector_status.primary_dns_resolver",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("json.PrimaryDNSResolver").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.PrimaryDNSResolver".into(),
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

            let _cond =
                { event.has_value("zscaler_zpa.app_connector_status.primary_dns_resolver") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("zscaler_zpa.app_connector_status.primary_dns_resolver")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("json.HostStartTime") != Some("0")
                    && event.has_value("json.HostStartTime")
                    && event.get_str("json.HostStartTime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.HostStartTime") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("zscaler_zpa.app_connector_status.host_start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.HostStartTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.HostStartTime").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.HostStartTime".into(),
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
                event.get_str("json.ConnectorStartTime") != Some("0")
                    && event.has_value("json.ConnectorStartTime")
                    && event.get_str("json.ConnectorStartTime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ConnectorStartTime") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set(
                                "zscaler_zpa.app_connector_status.connector_start_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.ConnectorStartTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event.remove("json.ConnectorStartTime").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "json.ConnectorStartTime".into(),
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

            if event.has_value("json.NumOfInterfaces") {
                event.rename(
                    "json.NumOfInterfaces",
                    "zscaler_zpa.app_connector_status.num_of_interfaces",
                )?;
            }

            if event.has_value("json.BytesRxInterface") {
                event.rename(
                    "json.BytesRxInterface",
                    "zscaler_zpa.app_connector_status.interface.received.bytes",
                )?;
            }

            if event.has_value("json.PacketsRxInterface") {
                event.rename(
                    "json.PacketsRxInterface",
                    "zscaler_zpa.app_connector_status.interface.received.packets",
                )?;
            }

            if event.has_value("json.ErrorsRxInterface") {
                event.rename(
                    "json.ErrorsRxInterface",
                    "zscaler_zpa.app_connector_status.interface.received.errors",
                )?;
            }

            if event.has_value("json.DiscardsRxInterface") {
                event.rename(
                    "json.DiscardsRxInterface",
                    "zscaler_zpa.app_connector_status.interface.received.discards",
                )?;
            }

            if event.has_value("json.BytesTxInterface") {
                event.rename(
                    "json.BytesTxInterface",
                    "zscaler_zpa.app_connector_status.interface.transmitted.bytes",
                )?;
            }

            if event.has_value("json.PacketsTxInterface") {
                event.rename(
                    "json.PacketsTxInterface",
                    "zscaler_zpa.app_connector_status.interface.transmitted.packets",
                )?;
            }

            if event.has_value("json.ErrorsTxInterface") {
                event.rename(
                    "json.ErrorsTxInterface",
                    "zscaler_zpa.app_connector_status.interface.transmitted.errors",
                )?;
            }

            if event.has_value("json.DiscardsTxInterface") {
                event.rename(
                    "json.DiscardsTxInterface",
                    "zscaler_zpa.app_connector_status.interface.transmitted.discards",
                )?;
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
            event.remove("json.DefRouteGW");
            event.remove("json.TotalBytesTx");
            event.remove("json.TotalBytesRx");
            event.remove("json.PublicIP");
            event.remove("json.PrivateIP");
            event.remove("json.TimestampAuthentication");
            event.remove("json.TimestampUnAuthentication");
            event.remove("json.PrimaryDNSResolver");
            event.remove("json.HostStartTime");
            event.remove("json.ConnectorStartTime");

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa.app_connector_status[m.getKey()] = m.getValue();\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"for (Map.Entry m : ctx.json.entrySet()) {\n  ctx.zscaler_zpa.app_connector_status[m.getKey()] = m.getValue();\n}\n"#
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
