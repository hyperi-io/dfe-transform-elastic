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
                if event.has_value("json.curintfstate") {
                    event.rename("json.curintfstate", "citrix_adc.interface.state")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.curlinkdowntime") {
                    event.rename(
                        "json.curlinkdowntime",
                        "citrix_adc.interface.link.down_time",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.curlinkuptime") {
                    event.rename("json.curlinkuptime", "citrix_adc.interface.link.up_time")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errdroppedrxpktsrate") {
                    event.rename(
                        "json.errdroppedrxpktsrate",
                        "citrix_adc.interface.packets.inbound.dropped.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errdroppedtxpktsrate") {
                    event.rename(
                        "json.errdroppedtxpktsrate",
                        "citrix_adc.interface.packets.transmission.dropped.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errifindiscardsrate") {
                    event.rename(
                        "json.errifindiscardsrate",
                        "citrix_adc.interface.packets.inbound.error_free.discarded.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errpktrxrate") {
                    event.rename(
                        "json.errpktrxrate",
                        "citrix_adc.interface.packets.inbound.dropped_by_hardware.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errpkttxrate") {
                    event.rename(
                        "json.errpkttxrate",
                        "citrix_adc.interface.packets.outbound.dropped_by_hardware.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.id") {
                    event.rename("json.id", "interface.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.jumbopktsreceivedrate") {
                    event.rename(
                        "json.jumbopktsreceivedrate",
                        "citrix_adc.interface.packets.received.jumbo.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.jumbopktstransmittedrate") {
                    event.rename(
                        "json.jumbopktstransmittedrate",
                        "citrix_adc.interface.packets.transmitted.jumbo.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.nicerrifoutdiscardsrate") {
                    event.rename(
                        "json.nicerrifoutdiscardsrate",
                        "citrix_adc.interface.packets.outbound.error_free.discarded.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.nicmulticastpktsrate") {
                    event.rename(
                        "json.nicmulticastpktsrate",
                        "citrix_adc.interface.packets.received.multicast.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.macmovedrate") {
                    event.rename("json.macmovedrate", "citrix_adc.interface.mac.moved.rate")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.txpktsrate") {
                    event.rename(
                        "json.txpktsrate",
                        "citrix_adc.interface.packets.transmitted.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.rxbytesrate") {
                    event.rename(
                        "json.rxbytesrate",
                        "citrix_adc.interface.received.bytes.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.rxpktsrate") {
                    event.rename(
                        "json.rxpktsrate",
                        "citrix_adc.interface.packets.received.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.txbytesrate") {
                    event.rename(
                        "json.txbytesrate",
                        "citrix_adc.interface.transmitted.bytes.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.trunkpktsreceivedrate") {
                    event.rename(
                        "json.trunkpktsreceivedrate",
                        "citrix_adc.interface.packets.received.tagged.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.trunkpktstransmittedrate") {
                    event.rename(
                        "json.trunkpktstransmittedrate",
                        "citrix_adc.interface.packets.transmitted.tagged.rate",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.nicerrdisables") {
                    if let Some(val) = event.get("json.nicerrdisables") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.nicerrdisables".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.interface.disabled.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.nicrxstalls") {
                    if let Some(val) = event.get("json.nicrxstalls") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.nicrxstalls".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.interface.stalled.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totrxbytes") {
                    if let Some(val) = event.get("json.totrxbytes") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totrxbytes".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.interface.received.bytes.value", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.tottxbytes") {
                    if let Some(val) = event.get("json.tottxbytes") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.tottxbytes".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.interface.transmitted.bytes.value", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totrxpkts") {
                    if let Some(val) = event.get("json.totrxpkts") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totrxpkts".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.interface.packets.received.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.jumbopktsreceived") {
                    if let Some(val) = event.get("json.jumbopktsreceived") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.jumbopktsreceived".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.received.jumbo.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.trunkpktsreceived") {
                    if let Some(val) = event.get("json.trunkpktsreceived") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.trunkpktsreceived".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.received.tagged.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.nictotmulticastpkts") {
                    if let Some(val) = event.get("json.nictotmulticastpkts") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.nictotmulticastpkts".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.received.multicast.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errdroppedtxpkts") {
                    if let Some(val) = event.get("json.errdroppedtxpkts") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.errdroppedtxpkts".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.transmission.dropped.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.tottxpkts") {
                    if let Some(val) = event.get("json.tottxpkts") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.tottxpkts".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.interface.packets.transmitted.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.jumbopktstransmitted") {
                    if let Some(val) = event.get("json.jumbopktstransmitted") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.jumbopktstransmitted".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.transmitted.jumbo.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.trunkpktstransmitted") {
                    if let Some(val) = event.get("json.trunkpktstransmitted") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.trunkpktstransmitted".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.transmitted.tagged.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errdroppedrxpkts") {
                    if let Some(val) = event.get("json.errdroppedrxpkts") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.errdroppedrxpkts".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.inbound.dropped.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errifindiscards") {
                    if let Some(val) = event.get("json.errifindiscards") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.errifindiscards".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.inbound.error_free.discarded.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errpktrx") {
                    if let Some(val) = event.get("json.errpktrx") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.errpktrx".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.inbound.dropped_by_hardware.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.errpkttx") {
                    if let Some(val) = event.get("json.errpkttx") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.errpkttx".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.outbound.dropped_by_hardware.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.nicerrifoutdiscards") {
                    if let Some(val) = event.get("json.nicerrifoutdiscards") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.nicerrifoutdiscards".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "citrix_adc.interface.packets.outbound.error_free.discarded.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.totmacmoved") {
                    if let Some(val) = event.get("json.totmacmoved") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.totmacmoved".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.interface.mac.moved.count", converted)?;
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
