// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `gpusbevent` pipeline.
pub struct Gpusbevent;

impl Transform for Gpusbevent {
    fn name(&self) -> &str {
        "gpusbevent"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.bsdName") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.bsdName", "volume.nt_name")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.content") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.content", "volume.file_system_type")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.busName") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.busName", "volume.bus_type")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.productName") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.productName", "volume.product_name")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.productId") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.productId", "volume.product_id")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.isRemovable") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.isRemovable", "volume.removable")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.serialNumber") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.serialNumber", "volume.serial_number")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.size") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.size", "volume.size")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.vendorId") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.vendorId", "volume.vendor_id")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.vendorName") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.vendorName", "volume.vendor_name")?;
            }

            let _cond = { event.get_str("jamf_protect.alerts.input.eventType") == Some("GPUSBEvent") && event.has_value("jamf_protect.alerts.input.match.event.device.isWritable") };
            if _cond {
                event.rename("jamf_protect.alerts.input.match.event.device.isWritable", "volume.writable")?;
            }

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
