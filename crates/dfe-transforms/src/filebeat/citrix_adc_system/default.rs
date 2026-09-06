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
                if event.has_value("json.system.addimgmtcpuusagepcnt") {
                    event.rename(
                        "json.system.addimgmtcpuusagepcnt",
                        "citrix_adc.system.cpu.utilization.additional_management.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.mgmtcpuusagepcnt") {
                    event.rename(
                        "json.system.mgmtcpuusagepcnt",
                        "citrix_adc.system.cpu.utilization.management.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.pktcpuusagepcnt") {
                    event.rename(
                        "json.system.pktcpuusagepcnt",
                        "citrix_adc.system.cpu.utilization.packets.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.cpuusagepcnt") {
                    event.rename(
                        "json.system.cpuusagepcnt",
                        "citrix_adc.system.cpu.utilization.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.disk0perusage") {
                    event.rename(
                        "json.system.disk0perusage",
                        "citrix_adc.system.disk.usage.flash_partition.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.disk1perusage") {
                    event.rename(
                        "json.system.disk1perusage",
                        "citrix_adc.system.disk.usage.var_partition.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.memusagepcnt") {
                    event.rename(
                        "json.system.memusagepcnt",
                        "citrix_adc.system.memory.utilization.pct",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.numcpus") {
                    if let Some(val) = event.get("json.system.numcpus") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.system.numcpus".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.system.cpu.count", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.rescpuusage") {
                    if let Some(val) = event.get("json.system.rescpuusage") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.system.rescpuusage".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.system.cpu.utilization.avg.pct", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.mastercpuusage") {
                    if let Some(val) = event.get("json.system.mastercpuusage") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.system.mastercpuusage".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.system.cpu.utilization.master.pct", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.slavecpuusage") {
                    if let Some(val) = event.get("json.system.slavecpuusage") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.system.slavecpuusage".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.system.cpu.utilization.slave.pct", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.memsizemb") {
                    if let Some(val) = event.get("json.system.memsizemb") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.system.memsizemb".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.system.memory.size.value", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.system.memuseinmb") {
                    if let Some(val) = event.get("json.system.memuseinmb") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.system.memuseinmb".into(),
                                message,
                            }
                        })?;
                        event.set("citrix_adc.system.memory.usage.value", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.system.starttime") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "EEE MMM dd HH:mm:ss yyyy",
                            "EEE MMM dd HH:mm:ss.SSSSSS yyyy",
                            "EEE MMM  d HH:mm:ss yyyy",
                            "EEE MMM  d HH:mm:ss.SSSSSS yyyy",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("citrix_adc.system.start.time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.system.starttime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // Painless script
            // Source: def convert(def megabytes){\n    def bytes = (megabytes*1024*1024);\n    return bytes;\n}\nif(ctx.citrix_adc?.system?.memory?.size?.value!=null && ctx.citrix_adc?.system?.memory?.size?.value!=\"\"){\n    ctx.citrix_adc.system.memory.size.value = convert(ctx.citrix_adc.system.memory.size.value);\n}\nif(ctx.citrix_adc?.system?.memory?.usage?.value!=null && ctx.citrix_adc?.system?.memory?.usage?.value!=\"\"){\n    ctx.citrix_adc.system.memory.usage.value = convert(ctx.citrix_adc.system.memory.usage.value);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def convert(def megabytes){\n    def bytes = (megabytes*1024*1024);\n    return bytes;\n}\nif(ctx.citrix_adc?.system?.memory?.size?.value!=null && ctx.citrix_adc?.system?.memory?.size?.value!=\"\"){\n    ctx.citrix_adc.system.memory.size.value = convert(ctx.citrix_adc.system.memory.size.value);\n}\nif(ctx.citrix_adc?.system?.memory?.usage?.value!=null && ctx.citrix_adc?.system?.memory?.usage?.value!=\"\"){\n    ctx.citrix_adc.system.memory.usage.value = convert(ctx.citrix_adc.system.memory.usage.value);\n}\n"#
                ),
            )?;

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\nif (o == null || o == \"\") {\n    return true;\n} else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n} else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n}\nreturn false;\n}\ndrop(ctx);\n
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
