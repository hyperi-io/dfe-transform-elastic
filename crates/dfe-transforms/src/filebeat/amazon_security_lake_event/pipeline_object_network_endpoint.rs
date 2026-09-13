// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_network_endpoint` pipeline.
pub struct PipelineObjectNetworkEndpoint;

impl Transform for PipelineObjectNetworkEndpoint {
    fn name(&self) -> &str {
        "pipeline_object_network_endpoint"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.has_value("ocsf.src_endpoint.domain") };
            if _cond {
                event.append_unique("source.domain", json!(event.get("ocsf.src_endpoint.domain").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.src_endpoint.location.city").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.city_name", v)?;
            }

            if let Some(v) = event.get("ocsf.src_endpoint.location.continent").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.continent_name", v)?;
            }

            let _cond = { event.get("ocsf.src_endpoint.location.coordinates").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.src_endpoint.location.coordinates", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_src_endpoint_location_coordinates_to_double")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.src_endpoint.location.coordinates").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.location", v)?;
            }

            if let Some(v) = event.get("ocsf.src_endpoint.location.country").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.country_iso_code", v)?;
            }

            if let Some(v) = event.get("ocsf.src_endpoint.location.desc").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.src_endpoint.location.is_on_premises") {
                if let Some(val) = event.get("ocsf.src_endpoint.location.is_on_premises") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.src_endpoint.location.is_on_premises".into(),
                            message,
                        })?;
                    event.set("ocsf.src_endpoint.location.is_on_premises", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_endpoint_location_is_on_premises_to_boolean")?;
                        event.remove("ocsf.src_endpoint.location.is_on_premises");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.src_endpoint.location.postal_code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.postal_code", v)?;
            }

            if let Some(v) = event.get("ocsf.src_endpoint.location.region").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.geo.region_iso_code", v)?;
            }

            let _cond = { event.has_value("ocsf.src_endpoint.hostname") };
            if _cond {
                event.append_unique("source.domain", json!(event.get("ocsf.src_endpoint.hostname").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("source.domain").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "source.domain", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("ocsf.src_endpoint.ip") == Some("") || event.get_str("ocsf.src_endpoint.ip") == Some("-") };
            if _cond {
                if event.remove("ocsf.src_endpoint.ip").is_none() {
                    return Err(TransformError::FieldNotFound { path: "ocsf.src_endpoint.ip".into() });
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.src_endpoint.ip") {
                if let Some(val) = event.get("ocsf.src_endpoint.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.src_endpoint.ip".into(),
                            message,
                        })?;
                    event.set("ocsf.src_endpoint.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_endpoint_ip_to_ip")?;
                        event.remove("ocsf.src_endpoint.ip");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.src_endpoint.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("ocsf.src_endpoint.ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.src_endpoint.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            let _cond = { event.get("ocsf.src_endpoint.intermediate_ips").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: ctx.ocsf.src_endpoint.intermediate_ips.removeIf(v -> v == '-');
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.ocsf.src_endpoint.intermediate_ips.removeIf(v -> v == '-');"#))?;
            }

            let _cond = { event.get("ocsf.src_endpoint.intermediate_ips").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.src_endpoint.intermediate_ips", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_src_endpoint_intermediate_ips_to_ip")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.src_endpoint.intermediate_ips").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.src_endpoint.intermediate_ips", |event| {
                    event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.src_endpoint.mac") {
                gsub_field(event, "ocsf.src_endpoint.mac", "ocsf.src_endpoint.mac", cached_regex!("[-:.]"), "-")?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_src_endpoint_mac")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("ocsf.src_endpoint.mac") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.src_endpoint.mac") {
                map_strings(event, "ocsf.src_endpoint.mac", "ocsf.src_endpoint.mac", str::to_uppercase)?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set("_ingest.on_failure_processor_tag", "uppercase_src_endpoint_mac")?;
                        event.remove("ocsf.src_endpoint.mac");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.src_endpoint.mac").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.mac", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.src_endpoint.port") {
                if let Some(val) = event.get("ocsf.src_endpoint.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.src_endpoint.port".into(),
                            message,
                        })?;
                    event.set("ocsf.src_endpoint.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_endpoint_port_to_long")?;
                        event.remove("ocsf.src_endpoint.port");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.src_endpoint.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.port", v)?;
            }

            let _cond = { event.has_value("ocsf.src_endpoint.svc_name") };
            if _cond {
                event.append_unique("network.application", json!(event.get("ocsf.src_endpoint.svc_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("ocsf.dst_endpoint.domain") };
            if _cond {
                event.append_unique("destination.domain", json!(event.get("ocsf.dst_endpoint.domain").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.location.city").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.city_name", v)?;
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.location.continent").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.continent_name", v)?;
            }

            let _cond = { event.get("ocsf.dst_endpoint.location.coordinates").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.dst_endpoint.location.coordinates", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dst_endpoint_location_coordinates_to_double")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.location.coordinates").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.location", v)?;
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.location.country").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.country_iso_code", v)?;
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.location.desc").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.name", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.dst_endpoint.location.is_on_premises") {
                if let Some(val) = event.get("ocsf.dst_endpoint.location.is_on_premises") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.dst_endpoint.location.is_on_premises".into(),
                            message,
                        })?;
                    event.set("ocsf.dst_endpoint.location.is_on_premises", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_endpoint_location_is_on_premises_to_boolean")?;
                        event.remove("ocsf.dst_endpoint.location.is_on_premises");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.location.postal_code").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.postal_code", v)?;
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.location.region").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.geo.region_iso_code", v)?;
            }

            let _cond = { event.has_value("ocsf.dst_endpoint.hostname") };
            if _cond {
                event.append_unique("destination.domain", json!(event.get("ocsf.dst_endpoint.hostname").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("destination.domain").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "destination.domain", |event| {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("ocsf.dst_endpoint.ip") == Some("") || event.get_str("ocsf.dst_endpoint.ip") == Some("-") };
            if _cond {
                if event.remove("ocsf.dst_endpoint.ip").is_none() {
                    return Err(TransformError::FieldNotFound { path: "ocsf.dst_endpoint.ip".into() });
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.dst_endpoint.ip") {
                if let Some(val) = event.get("ocsf.dst_endpoint.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.dst_endpoint.ip".into(),
                            message,
                        })?;
                    event.set("ocsf.dst_endpoint.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_endpoint_ip_to_ip")?;
                        event.remove("ocsf.dst_endpoint.ip");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("ocsf.dst_endpoint.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("ocsf.dst_endpoint.ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.get("ocsf.dst_endpoint.intermediate_ips").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: ctx.ocsf.dst_endpoint.intermediate_ips.removeIf(v -> v == '-');
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.ocsf.dst_endpoint.intermediate_ips.removeIf(v -> v == '-');"#))?;
            }

            let _cond = { event.get("ocsf.dst_endpoint.intermediate_ips").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.dst_endpoint.intermediate_ips", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dst_endpoint_intermediate_ips_to_ip")?;
                    event.remove("_ingest._value");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.dst_endpoint.intermediate_ips").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.dst_endpoint.intermediate_ips", |event| {
                    event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.dst_endpoint.mac") {
                gsub_field(event, "ocsf.dst_endpoint.mac", "ocsf.dst_endpoint.mac", cached_regex!("[-:.]"), "-")?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_dst_endpoint_mac")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("ocsf.dst_endpoint.mac") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.dst_endpoint.mac") {
                map_strings(event, "ocsf.dst_endpoint.mac", "ocsf.dst_endpoint.mac", str::to_uppercase)?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set("_ingest.on_failure_processor_tag", "uppercase_dst_endpoint_mac")?;
                        event.remove("ocsf.dst_endpoint.mac");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.mac").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.mac", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.dst_endpoint.port") {
                if let Some(val) = event.get("ocsf.dst_endpoint.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.dst_endpoint.port".into(),
                            message,
                        })?;
                    event.set("ocsf.dst_endpoint.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_endpoint_port_to_long")?;
                        event.remove("ocsf.dst_endpoint.port");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event.get("ocsf.dst_endpoint.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            let _cond = { event.has_value("ocsf.dst_endpoint.svc_name") };
            if _cond {
                event.append_unique("network.application", json!(event.get("ocsf.dst_endpoint.svc_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("network.application") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("network.application") {
                map_strings(event, "network.application", "network.application", str::to_lowercase)?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "lowercase")?;
                event.set("_ingest.on_failure_processor_tag", "lowercase_network_application")?;
                        event.remove("network.application");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
