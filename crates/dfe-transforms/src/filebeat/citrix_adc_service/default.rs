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
                    event.rename("json.primaryport", "citrix_adc.service.primary.port")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.name") {
                    event.rename("json.name", "service.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.requestbytesrate") {
                    event.rename(
                        "json.requestbytesrate",
                        "citrix_adc.service.request.bytes.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.requestsrate") {
                    event.rename("json.requestsrate", "citrix_adc.service.request.rate")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.responsebytesrate") {
                    event.rename(
                        "json.responsebytesrate",
                        "citrix_adc.service.response.bytes.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.responsesrate") {
                    event.rename("json.responsesrate", "citrix_adc.service.response.rate")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.state") {
                    event.rename("json.state", "service.state")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.throughputrate") {
                    event.rename("json.throughputrate", "citrix_adc.service.throughput.rate")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.servicetype") {
                    event.rename("json.servicetype", "service.type")?;
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
                        event.set("citrix_adc.service.client_connection.count", converted)?;
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
                        event.set("citrix_adc.service.request.bytes.value", converted)?;
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
                        event.set("citrix_adc.service.request.count", converted)?;
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
                        event.set("citrix_adc.service.response.bytes.value", converted)?;
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
                        event.set("citrix_adc.service.response.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.curreusepool") {
                    if let Some(val) = event.get("json.curreusepool") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.curreusepool".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.service.reuse_pool", converted)?;
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
                        event.set("citrix_adc.service.server.connection.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.svrestablishedconn") {
                    if let Some(val) = event.get("json.svrestablishedconn") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.svrestablishedconn".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.service.server.connection.established.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.avgsvrttfb") {
                    if let Some(val) = event.get("json.avgsvrttfb") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.avgsvrttfb".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.service.server.time_to_first_byte.avg",
                            converted,
                        )?;
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
                        event.set("citrix_adc.service.surge_queue.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.throughput") {
                    if let Some(val) = event.get("json.throughput") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.throughput".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.service.throughput.value", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.activetransactions") {
                    if let Some(val) = event.get("json.activetransactions") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.activetransactions".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.service.transaction.active.count", converted)?;
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
                            "citrix_adc.service.transaction.frustrating.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totsvrttlbtransactions") {
                    if let Some(val) = event.get("json.totsvrttlbtransactions") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totsvrttlbtransactions".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.service.transaction.time_to_last_byte.count",
                            converted,
                        )?;
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
                        event.set("citrix_adc.service.transaction.tolerable.count", converted)?;
                    }
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
                        event.set("citrix_adc.service.primary.ip_address", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("citrix_adc.service.primary.ip_address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("citrix_adc.service.primary.ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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
