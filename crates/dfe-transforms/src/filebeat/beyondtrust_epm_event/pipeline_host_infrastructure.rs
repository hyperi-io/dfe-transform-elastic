// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_host_infrastructure` pipeline.
pub struct PipelineHostInfrastructure;

impl Transform for PipelineHostInfrastructure {
    fn name(&self) -> &str {
        "pipeline_host_infrastructure"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if let Some(v) = event
            .get("beyondtrust_epm.event.EPMWinMac.TenantId")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.account.id", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.cloud.account.id") };
        if _cond {
            event.append_unique(
                "cloud.account.id",
                json!(
                    event
                        .get("beyondtrust_epm.event.cloud.account.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.account.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.account.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.availability_zone")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.availability_zone", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.instance.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.instance.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.instance.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.instance.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.machine.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.machine.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.account.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.account.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.account.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.account.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.availability_zone")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.availability_zone", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.instance.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.instance.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.instance.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.instance.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.machine.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.machine.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.project.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.project.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.project.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.project.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.provider")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.provider", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.region")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.region", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.origin.service.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.origin.service.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.project.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.project.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.project.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.project.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.provider")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.provider", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.region")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.region", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.service.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.service.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.account.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.account.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.account.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.account.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.availability_zone")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.availability_zone", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.instance.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.instance.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.instance.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.instance.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.machine.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.machine.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.project.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.project.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.project.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.project.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.provider")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.provider", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.region")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.region", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.cloud.target.service.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("cloud.target.service.name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.container.cpu.usage") {
                if let Some(val) = event.get("beyondtrust_epm.event.container.cpu.usage") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.container.cpu.usage".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.container.cpu.usage", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_container_cpu_usage_to_long",
            )?;
            event.remove("beyondtrust_epm.event.container.cpu.usage");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.cpu.usage")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.cpu.usage", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.container.disk.read.bytes") {
                if let Some(val) = event.get("beyondtrust_epm.event.container.disk.read.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.container.disk.read.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.container.disk.read.bytes", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_container_disk_read_bytes_to_long",
            )?;
            event.remove("beyondtrust_epm.event.container.disk.read.bytes");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.disk.read.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.disk.read.bytes", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.container.disk.write.bytes") {
                if let Some(val) = event.get("beyondtrust_epm.event.container.disk.write.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.container.disk.write.bytes".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.container.disk.write.bytes",
                        converted,
                    )?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_container_disk_write_bytes_to_long",
            )?;
            event.remove("beyondtrust_epm.event.container.disk.write.bytes");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.disk.write.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.disk.write.bytes", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.id", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.container.image.hash.all")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.container.image.hash.all",
                |event| {
                    event.append_unique(
                        "container.image.hash.all",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.container.image.hash.all")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.related == null) {\n  ctx.related = new HashMap();\n}\nif (ctx.related.hash == null) {\n  ctx.related.hash = new ArrayList();\n}\nfor (def entry : ctx.beyondtrust_epm.event.container.image.hash.all) {\n  if (entry == null) {\n    continue;\n  }\n  def digest = entry.toString().replace('[', '').replace(']', '');\n  int sep = digest.indexOf(':');\n  if (sep >= 0) {\n    digest = digest.substring(sep + 1);\n  }\n  if (digest.length() > 0 && !ctx.related.hash.contains(digest)) {\n    ctx.related.hash.add(digest);\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.related == null) {\n  ctx.related = new HashMap();\n}\nif (ctx.related.hash == null) {\n  ctx.related.hash = new ArrayList();\n}\nfor (def entry : ctx.beyondtrust_epm.event.container.image.hash.all) {\n  if (entry == null) {\n    continue;\n  }\n  def digest = entry.toString().replace('[', '').replace(']', '');\n  int sep = digest.indexOf(':');\n  if (sep >= 0) {\n    digest = digest.substring(sep + 1);\n  }\n  if (digest.length() > 0 && !ctx.related.hash.contains(digest)) {\n    ctx.related.hash.add(digest);\n  }\n}"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_container_image_hash_all_into_related_hash",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.image.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.image.name", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.container.image.tag")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.container.image.tag",
                |event| {
                    event.append_unique(
                        "container.image.tag",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.container.memory.usage") {
                if let Some(val) = event.get("beyondtrust_epm.event.container.memory.usage") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.container.memory.usage".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.container.memory.usage", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_container_memory_usage_to_long",
            )?;
            event.remove("beyondtrust_epm.event.container.memory.usage");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.memory.usage")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.memory.usage", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.container.network.egress.bytes") {
                if let Some(val) = event.get("beyondtrust_epm.event.container.network.egress.bytes")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.container.network.egress.bytes".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.container.network.egress.bytes",
                        converted,
                    )?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_container_network_egress_bytes_to_long",
            )?;
            event.remove("beyondtrust_epm.event.container.network.egress.bytes");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.network.egress.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.network.egress.bytes", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.container.network.ingress.bytes") {
                if let Some(val) =
                    event.get("beyondtrust_epm.event.container.network.ingress.bytes")
                {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.container.network.ingress.bytes".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.container.network.ingress.bytes",
                        converted,
                    )?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_container_network_ingress_bytes_to_long",
            )?;
            event.remove("beyondtrust_epm.event.container.network.ingress.bytes");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.network.ingress.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.network.ingress.bytes", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.container.runtime")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("container.runtime", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.faas.coldstart") {
                if let Some(val) = event.get("beyondtrust_epm.event.faas.coldstart") {
                    let converted = convert_value(val, "boolean").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.faas.coldstart".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.faas.coldstart", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_faas_coldstart_to_boolean",
            )?;
            event.remove("beyondtrust_epm.event.faas.coldstart");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.faas.coldstart")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("faas.coldstart", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.faas.execution")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("faas.execution", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.faas.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("faas.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.faas.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("faas.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.faas.trigger.request_id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("faas.trigger.request_id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.faas.trigger.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("faas.trigger.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.faas.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("faas.version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.architecture")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.architecture", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.boot.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.boot.id", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.cpu.usage") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.cpu.usage") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.cpu.usage".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.host.cpu.usage", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_cpu_usage_to_long",
            )?;
            event.remove("beyondtrust_epm.event.host.cpu.usage");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.cpu.usage")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.cpu.usage", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.disk.read.bytes") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.disk.read.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.disk.read.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.host.disk.read.bytes", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_disk_read_bytes_to_long",
            )?;
            event.remove("beyondtrust_epm.event.host.disk.read.bytes");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.disk.read.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.disk.read.bytes", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.disk.write.bytes") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.disk.write.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.disk.write.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.host.disk.write.bytes", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_disk_write_bytes_to_long",
            )?;
            event.remove("beyondtrust_epm.event.host.disk.write.bytes");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.disk.write.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.disk.write.bytes", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.domain")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.geo.TimezoneOffset") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.geo.TimezoneOffset") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.geo.TimezoneOffset".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.host.geo.TimezoneOffset", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_geo_TimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.host.geo.TimezoneOffset");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.city_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.city_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.continent_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.continent_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.continent_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.continent_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.country_iso_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.country_iso_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.country_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.country_name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.geo.location.lat") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.geo.location.lat") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.geo.location.lat".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.host.geo.location.lat", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_geo_location_lat_to_double",
            )?;
            event.remove("beyondtrust_epm.event.host.geo.location.lat");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.geo.location.lon") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.geo.location.lon") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.geo.location.lon".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.host.geo.location.lon", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_geo_location_lon_to_double",
            )?;
            event.remove("beyondtrust_epm.event.host.geo.location.lon");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.host.geo.location.lat")
                && event.has_value("beyondtrust_epm.event.host.geo.location.lon")
        };
        if _cond {
            // Painless script
            // Source: def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.host.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.host.geo.location.lon);\nctx.beyondtrust_epm.event.host.geo.location = location;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.host.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.host.geo.location.lon);\nctx.beyondtrust_epm.event.host.geo.location = location;"#
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.location")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.location", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.postal_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.postal_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.region_iso_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.region_iso_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.region_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.region_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.geo.timezone")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.geo.timezone", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.hostname")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.hostname", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.id", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.host.ip")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.host.ip", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_host_ip_to_ip")?;
                    event.remove("_ingest._value");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.host.ip")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.host.ip", |event| {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.host.ip")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.host.ip", |event| {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if event.has_value("beyondtrust_epm.event.host.mac") {
            gsub_field(
                event,
                "beyondtrust_epm.event.host.mac",
                "beyondtrust_epm.event.host.mac",
                cached_regex!("[:.]"),
                "-",
            )?;
        }

        if event.has_value("beyondtrust_epm.event.host.mac") {
            map_strings(
                event,
                "beyondtrust_epm.event.host.mac",
                "beyondtrust_epm.event.host.mac",
                str::to_uppercase,
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.host.mac")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.host.mac", |event| {
                event.append_unique(
                    "host.mac",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.network.egress.bytes") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.network.egress.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.network.egress.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.host.network.egress.bytes", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_network_egress_bytes_to_long",
            )?;
            event.remove("beyondtrust_epm.event.host.network.egress.bytes");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.network.egress.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.network.egress.bytes", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.network.egress.packets") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.network.egress.packets") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.network.egress.packets".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.host.network.egress.packets",
                        converted,
                    )?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_network_egress_packets_to_long",
            )?;
            event.remove("beyondtrust_epm.event.host.network.egress.packets");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.network.egress.packets")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.network.egress.packets", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.network.ingress.bytes") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.network.ingress.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.network.ingress.bytes".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.host.network.ingress.bytes",
                        converted,
                    )?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_network_ingress_bytes_to_long",
            )?;
            event.remove("beyondtrust_epm.event.host.network.ingress.bytes");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.network.ingress.bytes")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.network.ingress.bytes", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.network.ingress.packets") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.network.ingress.packets") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.network.ingress.packets".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.host.network.ingress.packets",
                        converted,
                    )?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_network_ingress_packets_to_long",
            )?;
            event.remove("beyondtrust_epm.event.host.network.ingress.packets");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.network.ingress.packets")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.network.ingress.packets", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.os.family")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.os.family", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.os.full")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.os.full", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.os.kernel")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.os.kernel", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.os.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.os.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.os.platform")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.os.platform", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.os.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.os.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.os.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.os.version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.pid_ns_ino")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.pid_ns_ino", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.type", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.host.uptime") {
                if let Some(val) = event.get("beyondtrust_epm.event.host.uptime") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.host.uptime".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.host.uptime", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_host_uptime_to_long",
            )?;
            event.remove("beyondtrust_epm.event.host.uptime");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.uptime")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.uptime", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.observer.geo.TimezoneOffset") {
                if let Some(val) = event.get("beyondtrust_epm.event.observer.geo.TimezoneOffset") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.observer.geo.TimezoneOffset".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "beyondtrust_epm.event.observer.geo.TimezoneOffset",
                        converted,
                    )?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_observer_geo_TimezoneOffset_to_long",
            )?;
            event.remove("beyondtrust_epm.event.observer.geo.TimezoneOffset");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.city_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.city_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.continent_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.continent_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.continent_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.continent_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.country_iso_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.country_iso_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.country_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.country_name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.observer.geo.location.lat") {
                if let Some(val) = event.get("beyondtrust_epm.event.observer.geo.location.lat") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.observer.geo.location.lat".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.observer.geo.location.lat", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_observer_geo_location_lat_to_double",
            )?;
            event.remove("beyondtrust_epm.event.observer.geo.location.lat");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if event.has_value("beyondtrust_epm.event.observer.geo.location.lon") {
                if let Some(val) = event.get("beyondtrust_epm.event.observer.geo.location.lon") {
                    let converted = convert_value(val, "double").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.event.observer.geo.location.lon".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.event.observer.geo.location.lon", converted)?;
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set(
                "_ingest.on_failure_processor_tag",
                "convert_observer_geo_location_lon_to_double",
            )?;
            event.remove("beyondtrust_epm.event.observer.geo.location.lon");
            event.append(
                "error.message",
                json!(format!(
                    "Processor {} with tag {} in pipeline {} failed with message: {}",
                    event
                        .get("_ingest.on_failure_processor_type")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_processor_tag")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_pipeline")
                        .map_or_else(String::new, template_to_string),
                    event
                        .get("_ingest.on_failure_message")
                        .map_or_else(String::new, template_to_string)
                )),
            )?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = {
            event.has_value("beyondtrust_epm.event.observer.geo.location.lat")
                && event.has_value("beyondtrust_epm.event.observer.geo.location.lon")
        };
        if _cond {
            // Painless script
            // Source: def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.observer.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.observer.geo.location.lon);\nctx.beyondtrust_epm.event.observer.geo.location = location;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.observer.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.observer.geo.location.lon);\nctx.beyondtrust_epm.event.observer.geo.location = location;"#
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.location")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.location", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.postal_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.postal_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.region_iso_code")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.region_iso_code", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.region_name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.region_name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.geo.timezone")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.geo.timezone", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.hostname")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.hostname", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.observer.ip")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.observer.ip", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                        if let Some(val) = event.get("_ingest._value") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_observer_ip_to_ip",
                    )?;
                    event.remove("_ingest._value");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.observer.ip")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.observer.ip", |event| {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.observer.ip")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.observer.ip", |event| {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if event.has_value("beyondtrust_epm.event.observer.mac") {
            gsub_field(
                event,
                "beyondtrust_epm.event.observer.mac",
                "beyondtrust_epm.event.observer.mac",
                cached_regex!("[:.]"),
                "-",
            )?;
        }

        if event.has_value("beyondtrust_epm.event.host.mac") {
            map_strings(
                event,
                "beyondtrust_epm.event.host.mac",
                "beyondtrust_epm.event.host.mac",
                str::to_uppercase,
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.observer.mac")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.observer.mac", |event| {
                event.append_unique(
                    "observer.mac",
                    json!(
                        event
                            .get("_ingest._value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.os.family")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.os.family", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.os.full")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.os.full", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.os.kernel")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.os.kernel", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.os.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.os.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.os.platform")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.os.platform", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.os.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.os.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.os.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.os.version", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.observer.product") };
        if _cond {
            event.append_unique(
                "observer.product",
                json!(
                    event
                        .get("beyondtrust_epm.event.observer.product")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.serial_number")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.serial_number", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.type", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.observer.vendor") };
        if _cond {
            event.append_unique(
                "observer.vendor",
                json!(
                    event
                        .get("beyondtrust_epm.event.observer.vendor")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.observer.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("observer.version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.api_version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.api_version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.cluster.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.cluster.id", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.cluster.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.cluster.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.cluster.url")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.cluster.url", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.cluster.version")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.cluster.version", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.namespace")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.namespace", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.organization")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.organization", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.resource.id")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.resource.id", v)?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.orchestrator.resource.ip")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.orchestrator.resource.ip",
                |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_orchestrator_resource_ip_to_ip",
                        )?;
                        event.remove("_ingest._value");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} with tag {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_pipeline")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.orchestrator.resource.ip")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.orchestrator.resource.ip",
                |event| {
                    event.append_unique(
                        "orchestrator.resource.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        let _cond = {
            event
                .get("beyondtrust_epm.event.orchestrator.resource.ip")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            foreach_array(
                event,
                "beyondtrust_epm.event.orchestrator.resource.ip",
                |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                },
            )?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.resource.name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.resource.name", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.resource.parent.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.resource.parent.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.resource.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.resource.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.orchestrator.type")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("orchestrator.type", v)?;
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.client.Name")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("host.name") {
                event.set("host.name", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.ChassisType")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("host.type") {
                event.set("host.type", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.DomainNetBIOSName")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("host.domain") {
                event.set("host.domain", v)?;
            }
        }

        if let Some(v) = event
            .get("beyondtrust_epm.event.host.os.ProductType")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            if !event.has("host.os.type") {
                event.set("host.os.type", v)?;
            }
        }

        let _cond = {
            event.has_value("host")
                && (!event.has_value("event.category")
                    || !(event.get("event.category").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("host")),
                        serde_json::Value::String(s) => s.contains("host"),
                        _ => false,
                    })))
        };
        if _cond {
            event.append_unique("event.category", json!("host"))?;
        }

        event.remove("beyondtrust_epm.event.EPMWinMac.TenantId");
        event.remove("beyondtrust_epm.event.cloud.account.id");
        event.remove("beyondtrust_epm.event.cloud.account.name");
        event.remove("beyondtrust_epm.event.cloud.availability_zone");
        event.remove("beyondtrust_epm.event.cloud.instance.id");
        event.remove("beyondtrust_epm.event.cloud.instance.name");
        event.remove("beyondtrust_epm.event.cloud.machine.type");
        event.remove("beyondtrust_epm.event.cloud.origin.account.id");
        event.remove("beyondtrust_epm.event.cloud.origin.account.name");
        event.remove("beyondtrust_epm.event.cloud.origin.availability_zone");
        event.remove("beyondtrust_epm.event.cloud.origin.instance.id");
        event.remove("beyondtrust_epm.event.cloud.origin.instance.name");
        event.remove("beyondtrust_epm.event.cloud.origin.machine.type");
        event.remove("beyondtrust_epm.event.cloud.origin.project.id");
        event.remove("beyondtrust_epm.event.cloud.origin.project.name");
        event.remove("beyondtrust_epm.event.cloud.origin.provider");
        event.remove("beyondtrust_epm.event.cloud.origin.region");
        event.remove("beyondtrust_epm.event.cloud.origin.service.name");
        event.remove("beyondtrust_epm.event.cloud.project.id");
        event.remove("beyondtrust_epm.event.cloud.project.name");
        event.remove("beyondtrust_epm.event.cloud.provider");
        event.remove("beyondtrust_epm.event.cloud.region");
        event.remove("beyondtrust_epm.event.cloud.service.name");
        event.remove("beyondtrust_epm.event.cloud.target.account.id");
        event.remove("beyondtrust_epm.event.cloud.target.account.name");
        event.remove("beyondtrust_epm.event.cloud.target.availability_zone");
        event.remove("beyondtrust_epm.event.cloud.target.instance.id");
        event.remove("beyondtrust_epm.event.cloud.target.instance.name");
        event.remove("beyondtrust_epm.event.cloud.target.machine.type");
        event.remove("beyondtrust_epm.event.cloud.target.project.id");
        event.remove("beyondtrust_epm.event.cloud.target.project.name");
        event.remove("beyondtrust_epm.event.cloud.target.provider");
        event.remove("beyondtrust_epm.event.cloud.target.region");
        event.remove("beyondtrust_epm.event.cloud.target.service.name");
        event.remove("beyondtrust_epm.event.container.cpu.usage");
        event.remove("beyondtrust_epm.event.container.disk.read.bytes");
        event.remove("beyondtrust_epm.event.container.disk.write.bytes");
        event.remove("beyondtrust_epm.event.container.id");
        event.remove("beyondtrust_epm.event.container.image.hash.all");
        event.remove("beyondtrust_epm.event.container.image.name");
        event.remove("beyondtrust_epm.event.container.image.tag");
        event.remove("beyondtrust_epm.event.container.memory.usage");
        event.remove("beyondtrust_epm.event.container.name");
        event.remove("beyondtrust_epm.event.container.network.egress.bytes");
        event.remove("beyondtrust_epm.event.container.network.ingress.bytes");
        event.remove("beyondtrust_epm.event.container.runtime");
        event.remove("beyondtrust_epm.event.faas.coldstart");
        event.remove("beyondtrust_epm.event.faas.execution");
        event.remove("beyondtrust_epm.event.faas.id");
        event.remove("beyondtrust_epm.event.faas.name");
        event.remove("beyondtrust_epm.event.faas.trigger.request_id");
        event.remove("beyondtrust_epm.event.faas.trigger.type");
        event.remove("beyondtrust_epm.event.faas.version");
        event.remove("beyondtrust_epm.event.host.architecture");
        event.remove("beyondtrust_epm.event.host.boot.id");
        event.remove("beyondtrust_epm.event.host.cpu.usage");
        event.remove("beyondtrust_epm.event.host.disk.read.bytes");
        event.remove("beyondtrust_epm.event.host.disk.write.bytes");
        event.remove("beyondtrust_epm.event.host.domain");
        event.remove("beyondtrust_epm.event.host.geo.city_name");
        event.remove("beyondtrust_epm.event.host.geo.continent_code");
        event.remove("beyondtrust_epm.event.host.geo.continent_name");
        event.remove("beyondtrust_epm.event.host.geo.country_iso_code");
        event.remove("beyondtrust_epm.event.host.geo.country_name");
        event.remove("beyondtrust_epm.event.host.geo.name");
        event.remove("beyondtrust_epm.event.host.geo.postal_code");
        event.remove("beyondtrust_epm.event.host.geo.region_iso_code");
        event.remove("beyondtrust_epm.event.host.geo.region_name");
        event.remove("beyondtrust_epm.event.host.geo.timezone");
        event.remove("beyondtrust_epm.event.host.hostname");
        event.remove("beyondtrust_epm.event.host.id");
        event.remove("beyondtrust_epm.event.host.ip");
        event.remove("beyondtrust_epm.event.host.mac");
        event.remove("beyondtrust_epm.event.host.name");
        event.remove("beyondtrust_epm.event.host.network.egress.bytes");
        event.remove("beyondtrust_epm.event.host.network.egress.packets");
        event.remove("beyondtrust_epm.event.host.network.ingress.bytes");
        event.remove("beyondtrust_epm.event.host.network.ingress.packets");
        event.remove("beyondtrust_epm.event.host.os.family");
        event.remove("beyondtrust_epm.event.host.os.full");
        event.remove("beyondtrust_epm.event.host.os.kernel");
        event.remove("beyondtrust_epm.event.host.os.name");
        event.remove("beyondtrust_epm.event.host.os.platform");
        event.remove("beyondtrust_epm.event.host.os.type");
        event.remove("beyondtrust_epm.event.host.os.version");
        event.remove("beyondtrust_epm.event.host.pid_ns_ino");
        event.remove("beyondtrust_epm.event.host.type");
        event.remove("beyondtrust_epm.event.host.uptime");
        event.remove("beyondtrust_epm.event.observer.geo.city_name");
        event.remove("beyondtrust_epm.event.observer.geo.continent_code");
        event.remove("beyondtrust_epm.event.observer.geo.continent_name");
        event.remove("beyondtrust_epm.event.observer.geo.country_iso_code");
        event.remove("beyondtrust_epm.event.observer.geo.country_name");
        event.remove("beyondtrust_epm.event.observer.geo.name");
        event.remove("beyondtrust_epm.event.observer.geo.postal_code");
        event.remove("beyondtrust_epm.event.observer.geo.region_iso_code");
        event.remove("beyondtrust_epm.event.observer.geo.region_name");
        event.remove("beyondtrust_epm.event.observer.geo.timezone");
        event.remove("beyondtrust_epm.event.observer.hostname");
        event.remove("beyondtrust_epm.event.observer.ip");
        event.remove("beyondtrust_epm.event.observer.mac");
        event.remove("beyondtrust_epm.event.observer.name");
        event.remove("beyondtrust_epm.event.observer.os.family");
        event.remove("beyondtrust_epm.event.observer.os.full");
        event.remove("beyondtrust_epm.event.observer.os.kernel");
        event.remove("beyondtrust_epm.event.observer.os.name");
        event.remove("beyondtrust_epm.event.observer.os.platform");
        event.remove("beyondtrust_epm.event.observer.os.type");
        event.remove("beyondtrust_epm.event.observer.os.version");
        event.remove("beyondtrust_epm.event.observer.product");
        event.remove("beyondtrust_epm.event.observer.serial_number");
        event.remove("beyondtrust_epm.event.observer.type");
        event.remove("beyondtrust_epm.event.observer.vendor");
        event.remove("beyondtrust_epm.event.observer.version");
        event.remove("beyondtrust_epm.event.orchestrator.api_version");
        event.remove("beyondtrust_epm.event.orchestrator.cluster.id");
        event.remove("beyondtrust_epm.event.orchestrator.cluster.name");
        event.remove("beyondtrust_epm.event.orchestrator.cluster.url");
        event.remove("beyondtrust_epm.event.orchestrator.cluster.version");
        event.remove("beyondtrust_epm.event.orchestrator.namespace");
        event.remove("beyondtrust_epm.event.orchestrator.organization");
        event.remove("beyondtrust_epm.event.orchestrator.resource.id");
        event.remove("beyondtrust_epm.event.orchestrator.resource.ip");
        event.remove("beyondtrust_epm.event.orchestrator.resource.name");
        event.remove("beyondtrust_epm.event.orchestrator.resource.parent.type");
        event.remove("beyondtrust_epm.event.orchestrator.resource.type");
        event.remove("beyondtrust_epm.event.orchestrator.type");

        Ok(TransformResult::Continue)
    }
}
