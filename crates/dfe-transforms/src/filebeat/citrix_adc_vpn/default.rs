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
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("8.11.0");
                if !painless_is_empty_value(&v) {
                    event.set("ecs.version", v)?;
                }
                Ok(())
            })();

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
                let v = Value::Array(vec![json!("web")]);
                if !painless_is_empty_value(&v) {
                    event.set("event.category", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!("citrix_adc");
                if !painless_is_empty_value(&v) {
                    event.set("event.module", v)?;
                }
                Ok(())
            })();

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.cfghtmlservedrate") {
                    event.rename(
                        "json.vpn.cfghtmlservedrate",
                        "citrix_adc.vpn.configuration_request_served.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.cpsconnfailurerate") {
                    event.rename(
                        "json.vpn.cpsconnfailurerate",
                        "citrix_adc.vpn.cps.failure.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.cpsconnsuccessrate") {
                    event.rename(
                        "json.vpn.cpsconnsuccessrate",
                        "citrix_adc.vpn.cps.success.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.csrequesthitrate") {
                    event.rename(
                        "json.vpn.csrequesthitrate",
                        "citrix_adc.vpn.client_server.request.hit.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.fsrequestrate") {
                    event.rename(
                        "json.vpn.fsrequestrate",
                        "citrix_adc.vpn.file_system.request.received.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.icalicensefailurerate") {
                    event.rename(
                        "json.vpn.icalicensefailurerate",
                        "citrix_adc.vpn.ica.license_failure.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksclienterrorrate") {
                    event.rename(
                        "json.vpn.socksclienterrorrate",
                        "citrix_adc.vpn.socks.client_error.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksconnreqrcvdrate") {
                    event.rename(
                        "json.vpn.socksconnreqrcvdrate",
                        "citrix_adc.vpn.socks.connection.request.received.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksconnreqsentrate") {
                    event.rename(
                        "json.vpn.socksconnreqsentrate",
                        "citrix_adc.vpn.socks.connection.request.sent.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksconnresprcvdrate") {
                    event.rename(
                        "json.vpn.socksconnresprcvdrate",
                        "citrix_adc.vpn.socks.connection.response.received.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksconnrespsentrate") {
                    event.rename(
                        "json.vpn.socksconnrespsentrate",
                        "citrix_adc.vpn.socks.connection.response.sent.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksmethreqrcvdrate") {
                    event.rename(
                        "json.vpn.socksmethreqrcvdrate",
                        "citrix_adc.vpn.socks.method.request.received.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksmethreqsentrate") {
                    event.rename(
                        "json.vpn.socksmethreqsentrate",
                        "citrix_adc.vpn.socks.method.request.sent.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksmethresprcvdrate") {
                    event.rename(
                        "json.vpn.socksmethresprcvdrate",
                        "citrix_adc.vpn.socks.method.response.received.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksmethrespsentrate") {
                    event.rename(
                        "json.vpn.socksmethrespsentrate",
                        "citrix_adc.vpn.socks.method.response.sent.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksservererrorrate") {
                    event.rename(
                        "json.vpn.socksservererrorrate",
                        "citrix_adc.vpn.socks.server_error.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.staconnfailurerate") {
                    event.rename(
                        "json.vpn.staconnfailurerate",
                        "citrix_adc.vpn.sta.connection.failure.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.staconnsuccessrate") {
                    event.rename(
                        "json.vpn.staconnsuccessrate",
                        "citrix_adc.vpn.sta.connection.success.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.starequestsentrate") {
                    event.rename(
                        "json.vpn.starequestsentrate",
                        "citrix_adc.vpn.sta.request.sent.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.staresponserecvdrate") {
                    event.rename(
                        "json.vpn.staresponserecvdrate",
                        "citrix_adc.vpn.sta.response.received.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.cfghtmlserved") {
                    if let Some(val) = event.get("json.vpn.cfghtmlserved") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.cfghtmlserved".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.vpn.configuration_request_served.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.cpsconnfailure") {
                    if let Some(val) = event.get("json.vpn.cpsconnfailure") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.cpsconnfailure".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.cps.failure.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.cpsconnsuccess") {
                    if let Some(val) = event.get("json.vpn.cpsconnsuccess") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.cpsconnsuccess".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.cps.success.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.csrequesthit") {
                    if let Some(val) = event.get("json.vpn.csrequesthit") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.csrequesthit".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.client_server.request.hit.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.totalfsrequest") {
                    if let Some(val) = event.get("json.vpn.totalfsrequest") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.totalfsrequest".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.vpn.file_system.request.received.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.icalicensefailure") {
                    if let Some(val) = event.get("json.vpn.icalicensefailure") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.icalicensefailure".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.ica.license_failure.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.indexhtmlhit") {
                    if let Some(val) = event.get("json.vpn.indexhtmlhit") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.indexhtmlhit".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.login_page.hits", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.vpnlicensefail") {
                    if let Some(val) = event.get("json.vpn.vpnlicensefail") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.vpnlicensefail".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.vpn.login_failed.license_unavailable.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksclienterror") {
                    if let Some(val) = event.get("json.vpn.socksclienterror") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksclienterror".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.socks.client_error.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksconnreqrcvd") {
                    if let Some(val) = event.get("json.vpn.socksconnreqrcvd") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksconnreqrcvd".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.vpn.socks.connection.request.received.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksconnreqsent") {
                    if let Some(val) = event.get("json.vpn.socksconnreqsent") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksconnreqsent".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.vpn.socks.connection.request.sent.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksconnresprcvd") {
                    if let Some(val) = event.get("json.vpn.socksconnresprcvd") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksconnresprcvd".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.vpn.socks.connection.response.received.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksconnrespsent") {
                    if let Some(val) = event.get("json.vpn.socksconnrespsent") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksconnrespsent".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.vpn.socks.connection.response.sent.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksmethreqrcvd") {
                    if let Some(val) = event.get("json.vpn.socksmethreqrcvd") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksmethreqrcvd".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.vpn.socks.method.request.received.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksmethreqsent") {
                    if let Some(val) = event.get("json.vpn.socksmethreqsent") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksmethreqsent".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.socks.method.request.sent.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksmethresprcvd") {
                    if let Some(val) = event.get("json.vpn.socksmethresprcvd") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksmethresprcvd".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.vpn.socks.method.response.received.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksmethrespsent") {
                    if let Some(val) = event.get("json.vpn.socksmethrespsent") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksmethrespsent".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.socks.method.response.sent.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.socksservererror") {
                    if let Some(val) = event.get("json.vpn.socksservererror") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.socksservererror".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.socks.server_error.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.staconnfailure") {
                    if let Some(val) = event.get("json.vpn.staconnfailure") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.staconnfailure".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.sta.connection.failure.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.staconnsuccess") {
                    if let Some(val) = event.get("json.vpn.staconnsuccess") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.staconnsuccess".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.sta.connection.success.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.starequestsent") {
                    if let Some(val) = event.get("json.vpn.starequestsent") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.starequestsent".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.sta.request.sent.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vpn.staresponserecvd") {
                    if let Some(val) = event.get("json.vpn.staresponserecvd") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vpn.staresponserecvd".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.vpn.sta.response.received.count", converted)?;
                    }
                }
                Ok(())
            })();

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("json");
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
