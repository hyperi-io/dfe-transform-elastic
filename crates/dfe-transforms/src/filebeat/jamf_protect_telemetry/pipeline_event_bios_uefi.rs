// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_event_bios_uefi` pipeline.
pub struct PipelineEventBiosUefi;

impl Transform for PipelineEventBiosUefi {
    fn name(&self) -> &str {
        "pipeline_event_bios_uefi"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("event.reason", json!("Collection of bios data"))?;

            if event.has_value("jamf_protect.telemetry.event.bios_uefi.architecture") {
                event.rename("jamf_protect.telemetry.event.bios_uefi.architecture", "host.architecture")?;
            }

            if event.has_value("jamf_protect.telemetry.event.bios_uefi.bios.firmware-version") {
                event.rename("jamf_protect.telemetry.event.bios_uefi.bios.firmware-version", "jamf_protect.telemetry.bios_firmware_version")?;
            }

            if event.has_value("jamf_protect.telemetry.event.bios_uefi.bios.system-firmware-version") {
                event.rename("jamf_protect.telemetry.event.bios_uefi.bios.system-firmware-version", "jamf_protect.telemetry.bios_system_firmware_version")?;
            }

        Ok(TransformResult::Continue)
    }
}
