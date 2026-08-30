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
        event.set("ecs.version", json!("8.17.0"))?;

        if event.has_value("prometheus.labels.DCGM_FI_DRIVER_VERSION") {
            event.rename(
                "prometheus.labels.DCGM_FI_DRIVER_VERSION",
                "gpu.labels.driver_version",
            )?;
        }

        if event.has_value("prometheus.labels.Hostname") {
            event.rename("prometheus.labels.Hostname", "gpu.labels.hostname")?;
        }

        if event.has_value("prometheus.labels.UUID") {
            event.rename("prometheus.labels.UUID", "gpu.labels.uuid")?;
        }

        if event.has_value("prometheus.labels.device") {
            event.rename("prometheus.labels.device", "gpu.labels.device")?;
        }

        if event.has_value("prometheus.labels.gpu") {
            event.rename("prometheus.labels.gpu", "gpu.labels.gpu")?;
        }

        if event.has_value("prometheus.labels.instance") {
            event.rename("prometheus.labels.instance", "server.address")?;
        }

        if event.has_value("prometheus.labels.job") {
            event.rename("prometheus.labels.job", "gpu.labels.job")?;
        }

        if event.has_value("prometheus.labels.modelName") {
            event.rename("prometheus.labels.modelName", "gpu.labels.model_name")?;
        }

        if event.has_value("prometheus.labels.pci_bus_id") {
            event.rename("prometheus.labels.pci_bus_id", "gpu.labels.pci_bus_id")?;
        }

        if event.has_value("prometheus.labels.err_code") {
            event.rename("prometheus.labels.err_code", "gpu.labels.err_code")?;
        }

        if event.has_value("prometheus.labels.err_msg") {
            event.rename("prometheus.labels.err_msg", "gpu.labels.err_msg")?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_SM_CLOCK.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_SM_CLOCK.value",
                "gpu.clock.streaming_multiprocessor_frequency",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_MEM_CLOCK.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_MEM_CLOCK.value",
                "gpu.clock.mem_frequency",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_GPU_TEMP.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_GPU_TEMP.value",
                "gpu.temperature.gpu",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_MEMORY_TEMP.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_MEMORY_TEMP.value",
                "gpu.temperature.memory",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_POWER_USAGE.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_POWER_USAGE.value",
                "gpu.power.usage",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_TOTAL_ENERGY_CONSUMPTION.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_TOTAL_ENERGY_CONSUMPTION.counter",
                "gpu.power.energy_consumption_total",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_PCIE_TX_BYTES.counter") {
            event.rename(
                "prometheus.DCGM_FI_PROF_PCIE_TX_BYTES.counter",
                "gpu.pcie.tx_bytes",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_PCIE_RX_BYTES.counter") {
            event.rename(
                "prometheus.DCGM_FI_PROF_PCIE_RX_BYTES.counter",
                "gpu.pcie.rx_bytes",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_PCIE_REPLAY_COUNTER.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_PCIE_REPLAY_COUNTER.counter",
                "gpu.pcie.replay",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_GPU_UTIL.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_GPU_UTIL.value",
                "gpu.utilization.gpu.pct",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_ENC_UTIL.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_ENC_UTIL.value",
                "gpu.utilization.encoder.pct",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_DEC_UTIL.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_DEC_UTIL.value",
                "gpu.utilization.decoder.pct",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_MEM_COPY_UTIL.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_MEM_COPY_UTIL.value",
                "gpu.utilization.memory_copy.pct",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_XID_ERRORS.value") {
            event.rename("prometheus.DCGM_FI_DEV_XID_ERRORS.value", "gpu.error.xid")?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_FB_USED.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_FB_USED.value",
                "gpu.memory.framebuffer.used_size",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_FB_FREE.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_FB_FREE.value",
                "gpu.memory.framebuffer.free_size",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_ECC_DBE_VOL_TOTAL.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_ECC_DBE_VOL_TOTAL.counter",
                "gpu.ecc.double_bit_volatile.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_ECC_DBE_AGG_TOTAL.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_ECC_DBE_AGG_TOTAL.counter",
                "gpu.ecc.double_bit_persistent.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_ECC_SBE_VOL_TOTAL.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_ECC_SBE_VOL_TOTAL.counter",
                "gpu.ecc.single_bit_volatile.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_ECC_SBE_AGG_TOTAL.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_ECC_SBE_AGG_TOTAL.counter",
                "gpu.ecc.single_bit_persistent.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_RETIRED_SBE.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_RETIRED_SBE.counter",
                "gpu.retired.single_bit_errors.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_RETIRED_DBE.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_RETIRED_DBE.counter",
                "gpu.retired.double_bit_errors.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_RETIRED_PENDING.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_RETIRED_PENDING.counter",
                "gpu.retired.pending.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT.counter",
                "gpu.nvlink.flowcontrol_crc_errors.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT.counter",
                "gpu.nvlink.data_crc_errors.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_NVLINK_REPLAY_ERROR_COUNT.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_NVLINK_REPLAY_ERROR_COUNT.counter",
                "gpu.nvlink.replay_errors.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT.counter",
                "gpu.nvlink.recovery_errors.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_NVLINK_BANDWIDTH_TOTAL.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_NVLINK_BANDWIDTH_TOTAL.counter",
                "gpu.nvlink.bandwidth_total",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_NVLINK_BANDWIDTH_L0.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_NVLINK_BANDWIDTH_L0.counter",
                "gpu.nvlink.bandwidth_l0_total",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_VGPU_LICENSE_STATUS.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_VGPU_LICENSE_STATUS.value",
                "gpu.license_vgpu_status",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_UNCORRECTABLE_REMAPPED_ROWS.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_UNCORRECTABLE_REMAPPED_ROWS.counter",
                "gpu.remapped.uncorrectable_remapped_rows.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_CORRECTABLE_REMAPPED_ROWS.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_CORRECTABLE_REMAPPED_ROWS.counter",
                "gpu.remapped.correctable_remapped_rows.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_ROW_REMAP_FAILURE.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_ROW_REMAP_FAILURE.value",
                "gpu.remapped.failed_remapped_rows.count",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_BRAND.value") {
            event.rename("prometheus.DCGM_FI_DEV_BRAND.value", "gpu.device.brand")?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_SERIAL.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_SERIAL.value",
                "gpu.device.serial_number",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_POWER_INFOROM_VER.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_POWER_INFOROM_VER.value",
                "gpu.device.power_info_rom_version",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_ECC_INFOROM_VER.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_ECC_INFOROM_VER.value",
                "gpu.device.ecc_info_rom_version",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_VBIOS_VERSION.value") {
            event.rename(
                "prometheus.DCGM_FI_DEV_VBIOS_VERSION.value",
                "gpu.device.vbios_version",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_SYNC_BOOST_VIOLATION.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_SYNC_BOOST_VIOLATION.counter",
                "gpu.throttling.sync_boost.us",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_THERMAL_VIOLATION.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_THERMAL_VIOLATION.counter",
                "gpu.throttling.thermal.us",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_LOW_UTIL_VIOLATION.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_LOW_UTIL_VIOLATION.counter",
                "gpu.throttling.low_utilization.us",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_BOARD_LIMIT_VIOLATION.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_BOARD_LIMIT_VIOLATION.counter",
                "gpu.throttling.board_limit.us",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_POWER_VIOLATION.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_POWER_VIOLATION.counter",
                "gpu.throttling.power.us",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_DEV_RELIABILITY_VIOLATION.counter") {
            event.rename(
                "prometheus.DCGM_FI_DEV_RELIABILITY_VIOLATION.counter",
                "gpu.throttling.reliability.us",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_GR_ENGINE_ACTIVE.value") {
            event.rename(
                "prometheus.DCGM_FI_PROF_GR_ENGINE_ACTIVE.value",
                "gpu.dcp.graphics_engine.active",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_SM_ACTIVE.value") {
            event.rename(
                "prometheus.DCGM_FI_PROF_SM_ACTIVE.value",
                "gpu.dcp.sm.active",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_SM_OCCUPANCY.value") {
            event.rename(
                "prometheus.DCGM_FI_PROF_SM_OCCUPANCY.value",
                "gpu.dcp.sm.occupancy",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_PIPE_TENSOR_ACTIVE.value") {
            event.rename(
                "prometheus.DCGM_FI_PROF_PIPE_TENSOR_ACTIVE.value",
                "gpu.dcp.tensor_pipe.active",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_DRAM_ACTIVE.value") {
            event.rename(
                "prometheus.DCGM_FI_PROF_DRAM_ACTIVE.value",
                "gpu.dcp.dram.active",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_PIPE_FP64_ACTIVE.value") {
            event.rename(
                "prometheus.DCGM_FI_PROF_PIPE_FP64_ACTIVE.value",
                "gpu.dcp.fp64_pipe.active",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_PIPE_FP32_ACTIVE.value") {
            event.rename(
                "prometheus.DCGM_FI_PROF_PIPE_FP32_ACTIVE.value",
                "gpu.dcp.fp32_pipe.active",
            )?;
        }

        if event.has_value("prometheus.DCGM_FI_PROF_PIPE_FP16_ACTIVE.value") {
            event.rename(
                "prometheus.DCGM_FI_PROF_PIPE_FP16_ACTIVE.value",
                "gpu.dcp.fp16_pipe.active",
            )?;
        }

        if event.has_value("prometheus.up.value") {
            event.rename("prometheus.up.value", "gpu.up")?;
        }

        if event.has_value("prometheus.labels.namespace") {
            event.rename("prometheus.labels.namespace", "kubernetes.namespace")?;
        }

        if event.has_value("prometheus.labels.container") {
            event.rename("prometheus.labels.container", "kubernetes.container.name")?;
        }

        if event.has_value("prometheus.labels.pod") {
            event.rename("prometheus.labels.pod", "kubernetes.pod.name")?;
        }

        Ok(TransformResult::Continue)
    }
}
