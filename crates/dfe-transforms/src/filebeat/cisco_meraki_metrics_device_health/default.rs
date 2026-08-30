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
            // Painless script
            // Source: // some values have unit 'percent' in the mappings; we need to scale them down from 0->100 to 0->1. // we round to 4 decimal places to avoid floating point errors.\nif (ctx.meraki != null) {\n    if (ctx.meraki.uplink != null && ctx.meraki.uplink.loss != null && ctx.meraki.uplink.loss.pct != null) {\n        ctx.meraki.uplink.loss.pct = Math.round((ctx.meraki.uplink.loss.pct / 100) * 10000) / 10000.0;\n    }\n\n    if (ctx.meraki.device != null && ctx.meraki.device.channel_utilization != null) {\n        def wifi0 = ctx.meraki.device.channel_utilization[\"2_4\"];\n        def wifi1 = ctx.meraki.device.channel_utilization[\"5\"];\n\n        if (wifi0 != null) {\n            if (wifi0.utilization_80211 != null) {\n                wifi0.utilization_80211 = Math.round((wifi0.utilization_80211 / 100) * 10000) / 10000.0;\n            }\n            if (wifi0.utilization_non_80211 != null) {\n                wifi0.utilization_non_80211 = Math.round((wifi0.utilization_non_80211 / 100) * 10000) / 10000.0;\n            }\n            if (wifi0.utilization_total != null) {\n                wifi0.utilization_total = Math.round((wifi0.utilization_total / 100) * 10000) / 10000.0;\n            }\n        }\n\n        if (wifi1 != null) {\n            if (wifi1.utilization_80211 != null) {\n                wifi1.utilization_80211 = Math.round((wifi1.utilization_80211 / 100) * 10000) / 10000.0;\n            }\n            if (wifi1.utilization_non_80211 != null) {\n                wifi1.utilization_non_80211 = Math.round((wifi1.utilization_non_80211 / 100) * 10000) / 10000.0;\n            }\n            if (wifi1.utilization_total != null) {\n                wifi1.utilization_total = Math.round((wifi1.utilization_total / 100) * 10000) / 10000.0;\n            }\n        }\n    }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"// some values have unit 'percent' in the mappings; we need to scale them down from 0->100 to 0->1. // we round to 4 decimal places to avoid floating point errors.\nif (ctx.meraki != null) {\n    if (ctx.meraki.uplink != null && ctx.meraki.uplink.loss != null && ctx.meraki.uplink.loss.pct != null) {\n        ctx.meraki.uplink.loss.pct = Math.round((ctx.meraki.uplink.loss.pct / 100) * 10000) / 10000.0;\n    }\n\n    if (ctx.meraki.device != null && ctx.meraki.device.channel_utilization != null) {\n        def wifi0 = ctx.meraki.device.channel_utilization[\"2_4\"];\n        def wifi1 = ctx.meraki.device.channel_utilization[\"5\"];\n\n        if (wifi0 != null) {\n            if (wifi0.utilization_80211 != null) {\n                wifi0.utilization_80211 = Math.round((wifi0.utilization_80211 / 100) * 10000) / 10000.0;\n            }\n            if (wifi0.utilization_non_80211 != null) {\n                wifi0.utilization_non_80211 = Math.round((wifi0.utilization_non_80211 / 100) * 10000) / 10000.0;\n            }\n            if (wifi0.utilization_total != null) {\n                wifi0.utilization_total = Math.round((wifi0.utilization_total / 100) * 10000) / 10000.0;\n            }\n        }\n\n        if (wifi1 != null) {\n            if (wifi1.utilization_80211 != null) {\n                wifi1.utilization_80211 = Math.round((wifi1.utilization_80211 / 100) * 10000) / 10000.0;\n            }\n            if (wifi1.utilization_non_80211 != null) {\n                wifi1.utilization_non_80211 = Math.round((wifi1.utilization_non_80211 / 100) * 10000) / 10000.0;\n            }\n            if (wifi1.utilization_total != null) {\n                wifi1.utilization_total = Math.round((wifi1.utilization_total / 100) * 10000) / 10000.0;\n            }\n        }\n    }\n}\n"#
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
