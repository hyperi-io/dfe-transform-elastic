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
            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "e4dbbe11b258255ff325ac23b4688e56bcd65a2ec19c6e309110508bf94f3fc6",
                    r#"{"params":null,"known":[{"RescaleFields":{"targets":["meraki.uplink.loss.pct","meraki.device.channel_utilization.2_4.utilization_80211","meraki.device.channel_utilization.2_4.utilization_non_80211","meraki.device.channel_utilization.2_4.utilization_total","meraki.device.channel_utilization.5.utilization_80211","meraki.device.channel_utilization.5.utilization_non_80211","meraki.device.channel_utilization.5.utilization_total"],"divisor":100,"round_unit":10000}}]}"#
                ),
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("meraki.uplink.rsrp") {
                    if let Some(val) = event.get("meraki.uplink.rsrp") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "meraki.uplink.rsrp".into(),
                                message,
                            }
                        })?;
                        event.set("meraki.uplink.rsrp", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("meraki.uplink.rsrp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "meraki.uplink.rsrp".into(),
                        });
                    }
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("meraki.uplink.rsrq") {
                    if let Some(val) = event.get("meraki.uplink.rsrq") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "meraki.uplink.rsrq".into(),
                                message,
                            }
                        })?;
                        event.set("meraki.uplink.rsrq", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("meraki.uplink.rsrq").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "meraki.uplink.rsrq".into(),
                        });
                    }
                    Ok(())
                })();
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("meraki.device.channel_utilization.2_4") {
                event.rename(
                    "meraki.device.channel_utilization.2_4",
                    "meraki.device.channel_utilization.wifi0",
                )?;
            }

            if event.has_value("meraki.device.channel_utilization.5") {
                event.rename(
                    "meraki.device.channel_utilization.5",
                    "meraki.device.channel_utilization.wifi1",
                )?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
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
