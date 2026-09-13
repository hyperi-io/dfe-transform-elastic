// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_hardware_event` pipeline.
pub struct PipelineHardwareEvent;

impl Transform for PipelineHardwareEvent {
    fn name(&self) -> &str {
        "pipeline_hardware_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.hardware_event_info.device_attributes.IOCFPlugInTypes") {
                    event.rename("json.hardware_event_info.device_attributes.IOCFPlugInTypes", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.cf_plugin_types")?;
                }

                if event.has_value("json.hardware_event_info.device_attributes.IOClassNameOverride") {
                    event.rename("json.hardware_event_info.device_attributes.IOClassNameOverride", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.class_name_override")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.CapabilityFlags") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.CapabilityFlags") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.hardware_event_info.device_attributes.IOPowerManagement.CapabilityFlags".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.capability_flags", converted)?;
                }
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.CurrentPowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.CurrentPowerState") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.hardware_event_info.device_attributes.IOPowerManagement.CurrentPowerState".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.current_power_state", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.DevicePowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.DevicePowerState") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.hardware_event_info.device_attributes.IOPowerManagement.DevicePowerState".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.device_power_state", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.DriverPowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.DriverPowerState") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.hardware_event_info.device_attributes.IOPowerManagement.DriverPowerState".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.driver_power_state", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.hardware_event_info.device_attributes.IOPowerManagement.MaxPowerState") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.IOPowerManagement.MaxPowerState") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.hardware_event_info.device_attributes.IOPowerManagement.MaxPowerState".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.io.power_management.max_power_state", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.hardware_event_info.device_attributes.Removable") {
                    event.rename("json.hardware_event_info.device_attributes.Removable", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.removable")?;
                }

                if event.has_value("json.hardware_event_info.device_attributes.USB Product Name") {
                    event.rename("json.hardware_event_info.device_attributes.USB Product Name", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.usb.product_name")?;
                }

                if event.has_value("json.hardware_event_info.device_attributes.USB Vendor Name") {
                    event.rename("json.hardware_event_info.device_attributes.USB Vendor Name", "jamf_compliance_reporter.log.hardware_event_info.device_attributes.usb.vendor_name")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.hardware_event_info.device_attributes.iSerialNumber") {
                if let Some(val) = event.get("json.hardware_event_info.device_attributes.iSerialNumber") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.hardware_event_info.device_attributes.iSerialNumber".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.hardware_event_info.device_attributes.iserial_number", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.hardware_event_info.device_class") {
                    event.rename("json.hardware_event_info.device_class", "jamf_compliance_reporter.log.hardware_event_info.device.class")?;
                }

                if event.has_value("json.hardware_event_info.device_name") {
                    event.rename("json.hardware_event_info.device_name", "jamf_compliance_reporter.log.hardware_event_info.device.name")?;
                }

                if event.has_value("json.hardware_event_info.device_status") {
                    event.rename("json.hardware_event_info.device_status", "jamf_compliance_reporter.log.hardware_event_info.device.status")?;
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
