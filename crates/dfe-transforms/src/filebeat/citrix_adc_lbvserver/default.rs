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
                if event.has_value("json.primaryport") {
                    event.rename("json.primaryport", "server.port")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.cltresponsetimeapdex") {
                    event.rename(
                        "json.cltresponsetimeapdex",
                        "citrix_adc.lbvserver.client.response_time.application_performance_index",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.deferredreqrate") {
                    event.rename(
                        "json.deferredreqrate",
                        "citrix_adc.lbvserver.request.deferred.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.name") {
                    event.rename("json.name", "citrix_adc.lbvserver.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.state") {
                    event.rename("json.state", "citrix_adc.lbvserver.state")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.pktssentrate") {
                    event.rename(
                        "json.pktssentrate",
                        "citrix_adc.lbvserver.packets.sent.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.requestbytesrate") {
                    event.rename(
                        "json.requestbytesrate",
                        "citrix_adc.lbvserver.request.received.bytes.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.requestsrate") {
                    event.rename(
                        "json.requestsrate",
                        "citrix_adc.lbvserver.request.received.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.responsebytesrate") {
                    event.rename(
                        "json.responsebytesrate",
                        "citrix_adc.lbvserver.response.received.bytes.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.responsesrate") {
                    event.rename(
                        "json.responsesrate",
                        "citrix_adc.lbvserver.response.received.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.hitsrate") {
                    event.rename("json.hitsrate", "citrix_adc.lbvserver.hit.rate")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.type") {
                    event.rename("json.type", "citrix_adc.lbvserver.protocol")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.primaryipaddress") {
                    if let Some(val) = event.get("json.primaryipaddress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.primaryipaddress".into(),
                                message,
                            }
                        })?;
                        event.set("server.ip", converted)?;
                    }
                }
                Ok(())
            })();

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.actsvcs") {
                    if let Some(val) = event.get("json.actsvcs") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.actsvcs".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.service.active.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.avgcltttlb") {
                    if let Some(val) = event.get("json.avgcltttlb") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.avgcltttlb".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.time_to_last_byte.avg", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.curclntconnections") {
                    if let Some(val) = event.get("json.curclntconnections") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.curclntconnections".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.lbvserver.client.connections.current.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.cursrvrconnections") {
                    if let Some(val) = event.get("json.cursrvrconnections") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.cursrvrconnections".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.connections.actual.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.deferredreq") {
                    if let Some(val) = event.get("json.deferredreq") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.deferredreq".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.request.deferred.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.establishedconn") {
                    if let Some(val) = event.get("json.establishedconn") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.establishedconn".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.lbvserver.client.connections.established.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.frustratingttlbtransactions") {
                    if let Some(val) = event.get("json.frustratingttlbtransactions") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.frustratingttlbtransactions".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.lbvserver.transaction.frustrating.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.inactsvcs") {
                    if let Some(val) = event.get("json.inactsvcs") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.inactsvcs".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.service.inactive.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.invalidrequestresponse") {
                    if let Some(val) = event.get("json.invalidrequestresponse") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.invalidrequestresponse".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.lbvserver.requests_responses.invalid.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.invalidrequestresponsedropped") {
                    if let Some(val) = event.get("json.invalidrequestresponsedropped") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.invalidrequestresponsedropped".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.lbvserver.requests_responses.dropped.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.sothreshold") {
                    if let Some(val) = event.get("json.sothreshold") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.sothreshold".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.threshold.spillover", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.surgecount") {
                    if let Some(val) = event.get("json.surgecount") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.surgecount".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.request.surge_queue.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.toleratingttlbtransactions") {
                    if let Some(val) = event.get("json.toleratingttlbtransactions") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.toleratingttlbtransactions".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.lbvserver.transaction.tolerable.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totalpktsrecvd") {
                    if let Some(val) = event.get("json.totalpktsrecvd") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totalpktsrecvd".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.packets.received.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totalpktssent") {
                    if let Some(val) = event.get("json.totalpktssent") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totalpktssent".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.packets.sent.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totalrequestbytes") {
                    if let Some(val) = event.get("json.totalrequestbytes") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totalrequestbytes".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.lbvserver.request.received.bytes.value",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totalrequests") {
                    if let Some(val) = event.get("json.totalrequests") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totalrequests".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.request.received.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totalresponsebytes") {
                    if let Some(val) = event.get("json.totalresponsebytes") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totalresponsebytes".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.lbvserver.response.received.bytes.value",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totalresponses") {
                    if let Some(val) = event.get("json.totalresponses") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totalresponses".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.response.received.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.tothits") {
                    if let Some(val) = event.get("json.tothits") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.tothits".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.hit.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totspillovers") {
                    if let Some(val) = event.get("json.totspillovers") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totspillovers".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.spillover.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totvserverdownbackuphits") {
                    if let Some(val) = event.get("json.totvserverdownbackuphits") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totvserverdownbackuphits".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.down.backup.hits", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vslbhealth") {
                    if let Some(val) = event.get("json.vslbhealth") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vslbhealth".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.health", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.vsvrsurgecount") {
                    if let Some(val) = event.get("json.vsvrsurgecount") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.vsvrsurgecount".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.lbvserver.request.waiting.count", converted)?;
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
