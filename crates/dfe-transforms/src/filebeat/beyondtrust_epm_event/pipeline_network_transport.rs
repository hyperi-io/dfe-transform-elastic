// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_network_transport` pipeline.
pub struct PipelineNetworkTransport;

impl Transform for PipelineNetworkTransport {
    fn name(&self) -> &str {
        "pipeline_network_transport"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if let Some(v) = event.get("beyondtrust_epm.event.client.address").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.address", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.as.number") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.as.number") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.as.number".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.as.number", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_as_number_to_long")?;
                    event.remove("beyondtrust_epm.event.client.as.number");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.as.number").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.as.number", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.as.organization.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.as.organization.name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.bytes") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.bytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.bytes".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.bytes", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_bytes_to_long")?;
                    event.remove("beyondtrust_epm.event.client.bytes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.bytes", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.geo.TimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.geo.TimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.geo.TimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.geo.TimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_geo_TimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.client.geo.TimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.city_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.city_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.continent_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.continent_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.continent_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.continent_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.country_iso_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.country_iso_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.country_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.country_name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.geo.location.lat") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.geo.location.lat") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.geo.location.lat".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.geo.location.lat", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_geo_location_lat_to_double")?;
                    event.remove("beyondtrust_epm.event.client.geo.location.lat");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.geo.location.lon") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.geo.location.lon") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.geo.location.lon".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.geo.location.lon", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_geo_location_lon_to_double")?;
                    event.remove("beyondtrust_epm.event.client.geo.location.lon");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.geo.location.lat") && event.has_value("beyondtrust_epm.event.client.geo.location.lon") };
        if _cond {
            // Painless script
            // Source: def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.client.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.client.geo.location.lon);\nctx.beyondtrust_epm.event.client.geo.location = location;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.client.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.client.geo.location.lon);\nctx.beyondtrust_epm.event.client.geo.location = location;"#))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.location").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.location", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.postal_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.postal_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.region_iso_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.region_iso_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.region_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.region_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.geo.timezone").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo.timezone", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.ip") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.ip") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.ip".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_ip_to_ip")?;
                    event.remove("beyondtrust_epm.event.client.ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("beyondtrust_epm.event.client.ip").map_or_else(String::new, template_to_string)))?;
        }

        if event.has_value("beyondtrust_epm.event.client.mac") {
            gsub_field(event, "beyondtrust_epm.event.client.mac", "beyondtrust_epm.event.client.mac", cached_regex!("[:.]"), "-")?;
        }

        if event.has_value("beyondtrust_epm.event.client.mac") {
            map_strings(event, "beyondtrust_epm.event.client.mac", "beyondtrust_epm.event.client.mac", str::to_uppercase)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.mac").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.mac", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.nat.ip") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.nat.ip") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.nat.ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.nat.ip".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.nat.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_nat_ip_to_ip")?;
                    event.remove("beyondtrust_epm.event.client.nat.ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.nat.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.nat.ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.nat.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("beyondtrust_epm.event.client.nat.ip").map_or_else(String::new, template_to_string)))?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.nat.port") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.nat.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.nat.port".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.nat.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_nat_port_to_long")?;
                    event.remove("beyondtrust_epm.event.client.nat.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.nat.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.nat.port", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.packets") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.packets") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.packets".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.packets", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_packets_to_long")?;
                    event.remove("beyondtrust_epm.event.client.packets");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.packets").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.packets", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.port") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.port".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_port_to_long")?;
                    event.remove("beyondtrust_epm.event.client.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.port", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.registered_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.registered_domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.subdomain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.subdomain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.top_level_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.top_level_domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.user.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.user.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.user.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.user.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_user_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.client.user.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.user.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.user.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.user.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.user.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_user_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.client.user.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.user.changes.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.user.changes.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.user.changes.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.user.changes.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_user_changes_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.client.user.changes.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.user.changes.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.user.changes.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.user.changes.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.user.changes.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_user_changes_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.client.user.changes.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.changes.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.changes.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.changes.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.changes.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.changes.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.client.user.changes.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.changes.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.changes.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.user.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.user.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.user.effective.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.user.effective.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.user.effective.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.user.effective.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_user_effective_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.client.user.effective.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.user.effective.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.user.effective.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.user.effective.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.user.effective.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_user_effective_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.client.user.effective.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.effective.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.effective.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.effective.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.effective.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.effective.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.client.user.effective.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.effective.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.effective.id").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.effective.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.effective.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.user.email").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.user.email", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.email").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.user.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.user.full_name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.full_name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.user.group.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.user.group.domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.user.group.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.user.group.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.user.group.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.user.group.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.user.hash").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.user.hash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.client.user.hash").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.user.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.user.id", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.id").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.client.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.user.name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get("beyondtrust_epm.event.client.user.roles").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.client.user.roles", |event| {
                event.append_unique("client.user.roles", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.user.target.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.user.target.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.user.target.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.user.target.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_user_target_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.client.user.target.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.client.user.target.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.client.user.target.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.client.user.target.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.client.user.target.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_user_target_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.client.user.target.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.target.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.target.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.target.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.target.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.target.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.client.user.target.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.target.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.target.id").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.target.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.target.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.address").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.address", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.as.number") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.as.number") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.as.number".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.as.number", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_as_number_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.as.number");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.as.number").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.as.number", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.as.organization.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.as.organization.name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.bytes") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.bytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.bytes".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.bytes", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_bytes_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.bytes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.bytes", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.geo.TimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.geo.TimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.geo.TimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.geo.TimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_geo_TimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.geo.TimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.city_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.city_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.continent_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.continent_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.continent_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.continent_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.country_iso_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.country_iso_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.country_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.country_name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.geo.location.lat") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.geo.location.lat") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.geo.location.lat".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.geo.location.lat", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_geo_location_lat_to_double")?;
                    event.remove("beyondtrust_epm.event.destination.geo.location.lat");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.geo.location.lon") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.geo.location.lon") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.geo.location.lon".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.geo.location.lon", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_geo_location_lon_to_double")?;
                    event.remove("beyondtrust_epm.event.destination.geo.location.lon");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.geo.location.lat") && event.has_value("beyondtrust_epm.event.destination.geo.location.lon") };
        if _cond {
            // Painless script
            // Source: def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.destination.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.destination.geo.location.lon);\nctx.beyondtrust_epm.event.destination.geo.location = location;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.destination.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.destination.geo.location.lon);\nctx.beyondtrust_epm.event.destination.geo.location = location;"#))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.location").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.location", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.postal_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.postal_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.region_iso_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.region_iso_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.region_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.region_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.geo.timezone").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.geo.timezone", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.ip") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.ip") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.ip".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_ip_to_ip")?;
                    event.remove("beyondtrust_epm.event.destination.ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("beyondtrust_epm.event.destination.ip").map_or_else(String::new, template_to_string)))?;
        }

        if event.has_value("beyondtrust_epm.event.destination.mac") {
            gsub_field(event, "beyondtrust_epm.event.destination.mac", "beyondtrust_epm.event.destination.mac", cached_regex!("[:.]"), "-")?;
        }

        if event.has_value("beyondtrust_epm.event.destination.mac") {
            map_strings(event, "beyondtrust_epm.event.destination.mac", "beyondtrust_epm.event.destination.mac", str::to_uppercase)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.mac").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.mac", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.nat.ip") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.nat.ip") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.nat.ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.nat.ip".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.nat.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_nat_ip_to_ip")?;
                    event.remove("beyondtrust_epm.event.destination.nat.ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.nat.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.nat.ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.nat.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("beyondtrust_epm.event.destination.nat.ip").map_or_else(String::new, template_to_string)))?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.nat.port") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.nat.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.nat.port".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.nat.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_nat_port_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.nat.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.nat.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.nat.port", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.packets") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.packets") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.packets".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.packets", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_packets_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.packets");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.packets").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.packets", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.port") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.port".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_port_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.port", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.registered_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.registered_domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.subdomain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.subdomain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.top_level_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.top_level_domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.user.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.user.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.user.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.user.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_user_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.user.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.user.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.user.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.user.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.user.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_user_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.user.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.user.changes.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.user.changes.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.user.changes.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.user.changes.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_user_changes_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.user.changes.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.user.changes.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.user.changes.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.user.changes.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.user.changes.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_user_changes_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.user.changes.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.changes.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.changes.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.changes.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.changes.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.changes.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.destination.user.changes.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.changes.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.changes.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.user.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.user.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.user.effective.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.user.effective.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.user.effective.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.user.effective.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_user_effective_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.user.effective.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.user.effective.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.user.effective.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.user.effective.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.user.effective.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_user_effective_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.user.effective.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.effective.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.effective.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.effective.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.effective.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.effective.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.destination.user.effective.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.effective.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.effective.id").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.user.email").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.user.email", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.email").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.user.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.user.full_name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.full_name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.user.group.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.user.group.domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.user.group.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.user.group.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.user.group.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.user.group.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.user.hash").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.user.hash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.destination.user.hash").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.user.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.user.id", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.id").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.destination.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.user.name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get("beyondtrust_epm.event.destination.user.roles").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.destination.user.roles", |event| {
                event.append_unique("destination.user.roles", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.user.target.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.user.target.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.user.target.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.user.target.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_user_target_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.user.target.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.destination.user.target.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.destination.user.target.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.destination.user.target.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.destination.user.target.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_destination_user_target_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.destination.user.target.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.target.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.target.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.target.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.target.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.target.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.destination.user.target.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.target.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.target.id").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.destination.user.target.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.destination.user.target.name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get("beyondtrust_epm.event.dns.header_flags").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.dns.header_flags", |event| {
                event.append_unique("dns.header_flags", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.op_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.op_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.question.class").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.question.class", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.question.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.question.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.question.registered_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.question.registered_domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.question.subdomain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.question.subdomain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.question.top_level_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.question.top_level_domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.question.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.question.type", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.dns.resolved_ip").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.dns.resolved_ip", |event| {
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
                event.set("_ingest.on_failure_processor_tag", "convert_dns_resolved_ip_to_ip")?;
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
        }

        let _cond = { event.get("beyondtrust_epm.event.dns.resolved_ip").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.dns.resolved_ip", |event| {
                event.append_unique("dns.resolved_ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.dns.resolved_ip").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.dns.resolved_ip", |event| {
                event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.response_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.response_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.dns.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("dns.type", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.http.request.body.bytes") {
            if let Some(val) = event.get("beyondtrust_epm.event.http.request.body.bytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.http.request.body.bytes".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.http.request.body.bytes", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_http_request_body_bytes_to_long")?;
                    event.remove("beyondtrust_epm.event.http.request.body.bytes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.request.body.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.body.bytes", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.request.body.content").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.body.content", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.http.request.bytes") {
            if let Some(val) = event.get("beyondtrust_epm.event.http.request.bytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.http.request.bytes".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.http.request.bytes", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_http_request_bytes_to_long")?;
                    event.remove("beyondtrust_epm.event.http.request.bytes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.request.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.bytes", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.request.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.request.method").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.method", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.request.mime_type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.mime_type", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.request.referrer").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.referrer", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.http.response.body.bytes") {
            if let Some(val) = event.get("beyondtrust_epm.event.http.response.body.bytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.http.response.body.bytes".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.http.response.body.bytes", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_http_response_body_bytes_to_long")?;
                    event.remove("beyondtrust_epm.event.http.response.body.bytes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.response.body.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.response.body.bytes", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.response.body.content").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.response.body.content", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.http.response.bytes") {
            if let Some(val) = event.get("beyondtrust_epm.event.http.response.bytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.http.response.bytes".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.http.response.bytes", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_http_response_bytes_to_long")?;
                    event.remove("beyondtrust_epm.event.http.response.bytes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.response.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.response.bytes", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.response.mime_type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.response.mime_type", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.http.response.status_code") {
            if let Some(val) = event.get("beyondtrust_epm.event.http.response.status_code") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.http.response.status_code".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.http.response.status_code", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_http_response_status_code_to_long")?;
                    event.remove("beyondtrust_epm.event.http.response.status_code");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.response.status_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.response.status_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.http.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.version", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.application").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.application", v)?;
        }

        if event.has_value("network.application") {
            map_strings(event, "network.application", "network.application", str::to_lowercase)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.network.bytes") {
            if let Some(val) = event.get("beyondtrust_epm.event.network.bytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.network.bytes".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.network.bytes", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_network_bytes_to_long")?;
                    event.remove("beyondtrust_epm.event.network.bytes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.bytes", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.community_id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.community_id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.direction").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.direction", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.network.forwarded_ip") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.network.forwarded_ip") {
            if let Some(val) = event.get("beyondtrust_epm.event.network.forwarded_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.network.forwarded_ip".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.network.forwarded_ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_network_forwarded_ip_to_ip")?;
                    event.remove("beyondtrust_epm.event.network.forwarded_ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.forwarded_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.forwarded_ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.network.forwarded_ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("beyondtrust_epm.event.network.forwarded_ip").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.iana_number").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.iana_number", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.network.packets") {
            if let Some(val) = event.get("beyondtrust_epm.event.network.packets") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.network.packets".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.network.packets", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_network_packets_to_long")?;
                    event.remove("beyondtrust_epm.event.network.packets");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.packets").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.packets", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.protocol", v)?;
        }

        if event.has_value("network.protocol") {
            map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.transport").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.transport", v)?;
        }

        if event.has_value("network.transport") {
            map_strings(event, "network.transport", "network.transport", str::to_lowercase)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.type", v)?;
        }

        if event.has_value("network.type") {
            map_strings(event, "network.type", "network.type", str::to_lowercase)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.vlan.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.vlan.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.network.vlan.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.vlan.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.address").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.address", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.as.number") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.as.number") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.as.number".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.as.number", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_as_number_to_long")?;
                    event.remove("beyondtrust_epm.event.server.as.number");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.as.number").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.as.number", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.as.organization.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.as.organization.name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.bytes") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.bytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.bytes".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.bytes", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_bytes_to_long")?;
                    event.remove("beyondtrust_epm.event.server.bytes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.bytes", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.geo.TimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.geo.TimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.geo.TimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.geo.TimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_geo_TimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.server.geo.TimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.city_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.city_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.continent_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.continent_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.continent_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.continent_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.country_iso_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.country_iso_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.country_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.country_name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.geo.location.lat") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.geo.location.lat") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.geo.location.lat".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.geo.location.lat", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_geo_location_lat_to_double")?;
                    event.remove("beyondtrust_epm.event.server.geo.location.lat");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.geo.location.lon") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.geo.location.lon") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.geo.location.lon".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.geo.location.lon", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_geo_location_lon_to_double")?;
                    event.remove("beyondtrust_epm.event.server.geo.location.lon");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.geo.location.lat") && event.has_value("beyondtrust_epm.event.server.geo.location.lon") };
        if _cond {
            // Painless script
            // Source: def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.server.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.server.geo.location.lon);\nctx.beyondtrust_epm.event.server.geo.location = location;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.server.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.server.geo.location.lon);\nctx.beyondtrust_epm.event.server.geo.location = location;"#))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.location").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.location", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.postal_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.postal_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.region_iso_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.region_iso_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.region_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.region_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.geo.timezone").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo.timezone", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.ip") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.ip") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.ip".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_ip_to_ip")?;
                    event.remove("beyondtrust_epm.event.server.ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("beyondtrust_epm.event.server.ip").map_or_else(String::new, template_to_string)))?;
        }

        if event.has_value("beyondtrust_epm.event.server.mac") {
            gsub_field(event, "beyondtrust_epm.event.server.mac", "beyondtrust_epm.event.server.mac", cached_regex!("[:.]"), "-")?;
        }

        if event.has_value("beyondtrust_epm.event.server.mac") {
            map_strings(event, "beyondtrust_epm.event.server.mac", "beyondtrust_epm.event.server.mac", str::to_uppercase)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.mac").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.mac", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.nat.ip") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.nat.ip") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.nat.ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.nat.ip".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.nat.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_nat_ip_to_ip")?;
                    event.remove("beyondtrust_epm.event.server.nat.ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.nat.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.nat.ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.nat.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("beyondtrust_epm.event.server.nat.ip").map_or_else(String::new, template_to_string)))?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.nat.port") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.nat.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.nat.port".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.nat.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_nat_port_to_long")?;
                    event.remove("beyondtrust_epm.event.server.nat.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.nat.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.nat.port", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.packets") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.packets") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.packets".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.packets", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_packets_to_long")?;
                    event.remove("beyondtrust_epm.event.server.packets");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.packets").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.packets", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.port") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.port".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_port_to_long")?;
                    event.remove("beyondtrust_epm.event.server.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.port", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.registered_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.registered_domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.subdomain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.subdomain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.top_level_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.top_level_domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.user.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.user.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.user.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.user.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_user_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.server.user.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.user.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.user.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.user.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.user.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_user_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.server.user.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.user.changes.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.user.changes.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.user.changes.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.user.changes.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_user_changes_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.server.user.changes.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.user.changes.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.user.changes.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.user.changes.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.user.changes.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_user_changes_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.server.user.changes.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.changes.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.changes.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.changes.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.changes.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.changes.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.server.user.changes.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.changes.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.changes.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.user.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.user.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.user.effective.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.user.effective.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.user.effective.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.user.effective.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_user_effective_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.server.user.effective.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.user.effective.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.user.effective.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.user.effective.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.user.effective.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_user_effective_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.server.user.effective.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.effective.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.effective.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.effective.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.effective.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.effective.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.server.user.effective.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.effective.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.effective.id").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.user.email").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.user.email", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.email").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.user.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.user.full_name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.full_name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.user.group.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.user.group.domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.user.group.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.user.group.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.user.group.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.user.group.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.user.hash").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.user.hash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.server.user.hash").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.user.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.user.id", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.id").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.server.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.user.name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get("beyondtrust_epm.event.server.user.roles").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.server.user.roles", |event| {
                event.append_unique("server.user.roles", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.user.target.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.user.target.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.user.target.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.user.target.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_user_target_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.server.user.target.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.server.user.target.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.server.user.target.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.server.user.target.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.server.user.target.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_user_target_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.server.user.target.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.target.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.target.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.target.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.target.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.target.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.server.user.target.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.target.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.target.id").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.server.user.target.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.server.user.target.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.address").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.address", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.as.number") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.as.number") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.as.number".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.as.number", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_as_number_to_long")?;
                    event.remove("beyondtrust_epm.event.source.as.number");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.as.number").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.as.number", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.as.organization.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.as.organization.name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.bytes") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.bytes") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.bytes".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.bytes", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_bytes_to_long")?;
                    event.remove("beyondtrust_epm.event.source.bytes");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.bytes").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.bytes", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.geo.TimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.geo.TimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.geo.TimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.geo.TimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_geo_TimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.source.geo.TimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.city_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.city_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.continent_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.continent_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.continent_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.continent_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.country_iso_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.country_iso_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.country_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.country_name", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.geo.location.lat") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.geo.location.lat") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.geo.location.lat".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.geo.location.lat", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_geo_location_lat_to_double")?;
                    event.remove("beyondtrust_epm.event.source.geo.location.lat");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.geo.location.lon") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.geo.location.lon") {
                let converted = convert_value(val, "double")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.geo.location.lon".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.geo.location.lon", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_geo_location_lon_to_double")?;
                    event.remove("beyondtrust_epm.event.source.geo.location.lon");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.geo.location.lat") && event.has_value("beyondtrust_epm.event.source.geo.location.lon") };
        if _cond {
            // Painless script
            // Source: def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.source.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.source.geo.location.lon);\nctx.beyondtrust_epm.event.source.geo.location = location;
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def location = new HashMap();\nlocation.put('lat', ctx.beyondtrust_epm.event.source.geo.location.lat);\nlocation.put('lon', ctx.beyondtrust_epm.event.source.geo.location.lon);\nctx.beyondtrust_epm.event.source.geo.location = location;"#))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.location").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.location", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.postal_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.postal_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.region_iso_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.region_iso_code", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.region_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.region_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.geo.timezone").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.geo.timezone", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.ip") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.ip") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.ip".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_ip_to_ip")?;
                    event.remove("beyondtrust_epm.event.source.ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("beyondtrust_epm.event.source.ip").map_or_else(String::new, template_to_string)))?;
        }

        if event.has_value("beyondtrust_epm.event.source.mac") {
            gsub_field(event, "beyondtrust_epm.event.source.mac", "beyondtrust_epm.event.source.mac", cached_regex!("[:.]"), "-")?;
        }

        if event.has_value("beyondtrust_epm.event.source.mac") {
            map_strings(event, "beyondtrust_epm.event.source.mac", "beyondtrust_epm.event.source.mac", str::to_uppercase)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.mac").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.mac", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.nat.ip") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.nat.ip") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.nat.ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.nat.ip".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.nat.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_nat_ip_to_ip")?;
                    event.remove("beyondtrust_epm.event.source.nat.ip");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.nat.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.nat.ip", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.nat.ip") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("beyondtrust_epm.event.source.nat.ip").map_or_else(String::new, template_to_string)))?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.nat.port") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.nat.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.nat.port".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.nat.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_nat_port_to_long")?;
                    event.remove("beyondtrust_epm.event.source.nat.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.nat.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.nat.port", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.packets") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.packets") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.packets".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.packets", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_packets_to_long")?;
                    event.remove("beyondtrust_epm.event.source.packets");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.packets").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.packets", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.port") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.port".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_port_to_long")?;
                    event.remove("beyondtrust_epm.event.source.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.port", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.registered_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.registered_domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.subdomain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.subdomain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.top_level_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.top_level_domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.user.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.user.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.user.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.user.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_user_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.source.user.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.user.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.user.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.user.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.user.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_user_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.source.user.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.user.changes.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.user.changes.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.user.changes.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.user.changes.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_user_changes_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.source.user.changes.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.user.changes.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.user.changes.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.user.changes.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.user.changes.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_user_changes_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.source.user.changes.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.changes.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.changes.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.changes.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.changes.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.changes.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.source.user.changes.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.changes.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.changes.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.user.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.domain", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.user.effective.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.user.effective.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.user.effective.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.user.effective.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_user_effective_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.source.user.effective.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.user.effective.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.user.effective.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.user.effective.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.user.effective.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_user_effective_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.source.user.effective.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.effective.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.effective.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.effective.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.effective.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.effective.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.source.user.effective.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.effective.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.effective.id").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.user.email").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.email", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.email").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.user.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.full_name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.full_name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.user.group.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.group.domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.user.group.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.group.id", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.user.group.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.group.name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.user.hash").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.hash", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.source.user.hash").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.user.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.id", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.id").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.source.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.name", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get("beyondtrust_epm.event.source.user.roles").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.source.user.roles", |event| {
                event.append_unique("source.user.roles", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.user.target.DefaultTimezoneOffset") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.user.target.DefaultTimezoneOffset") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.user.target.DefaultTimezoneOffset".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.user.target.DefaultTimezoneOffset", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_user_target_DefaultTimezoneOffset_to_long")?;
                    event.remove("beyondtrust_epm.event.source.user.target.DefaultTimezoneOffset");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.source.user.target.LocalIdentifier") {
            if let Some(val) = event.get("beyondtrust_epm.event.source.user.target.LocalIdentifier") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.source.user.target.LocalIdentifier".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.source.user.target.LocalIdentifier", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_user_target_LocalIdentifier_to_long")?;
                    event.remove("beyondtrust_epm.event.source.user.target.LocalIdentifier");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.target.email") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.target.email").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.target.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.target.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.target.hash") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.source.user.target.hash").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.target.id") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.target.id").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.source.user.target.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.source.user.target.name").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.cipher").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.cipher", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.certificate").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.certificate", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.certificate_chain").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.certificate_chain", |event| {
                event.append_unique("tls.client.certificate_chain", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.hash.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.hash.md5", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.client.hash.md5") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.tls.client.hash.md5").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.hash.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.hash.sha1", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.client.hash.sha1") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.tls.client.hash.sha1").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.hash.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.hash.sha256", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.client.hash.sha256") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.tls.client.hash.sha256").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.issuer").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.issuer", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.ja3").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.ja3", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.client.not_after") && event.get_str("beyondtrust_epm.event.tls.client.not_after") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.tls.client.not_after") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.tls.client.not_after", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.tls.client.not_after".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_tls_client_not_after")?;
                    event.remove("beyondtrust_epm.event.tls.client.not_after");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.not_after").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.not_after", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.client.not_before") && event.get_str("beyondtrust_epm.event.tls.client.not_before") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.tls.client.not_before") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.tls.client.not_before", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.tls.client.not_before".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_tls_client_not_before")?;
                    event.remove("beyondtrust_epm.event.tls.client.not_before");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.not_before").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.not_before", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.server_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.server_name", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.subject", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.supported_ciphers").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.supported_ciphers", |event| {
                event.append_unique("tls.client.supported_ciphers", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.alternative_names").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.alternative_names", |event| {
                event.append_unique("tls.client.x509.alternative_names", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.issuer.common_name").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.issuer.common_name", |event| {
                event.append_unique("tls.client.x509.issuer.common_name", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.issuer.country").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.issuer.country", |event| {
                event.append_unique("tls.client.x509.issuer.country", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.issuer.distinguished_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.issuer.distinguished_name", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.issuer.locality").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.issuer.locality", |event| {
                event.append_unique("tls.client.x509.issuer.locality", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.issuer.organization").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.issuer.organization", |event| {
                event.append_unique("tls.client.x509.issuer.organization", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.issuer.organizational_unit").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.issuer.organizational_unit", |event| {
                event.append_unique("tls.client.x509.issuer.organizational_unit", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.issuer.state_or_province").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.issuer.state_or_province", |event| {
                event.append_unique("tls.client.x509.issuer.state_or_province", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.client.x509.not_after") && event.get_str("beyondtrust_epm.event.tls.client.x509.not_after") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.tls.client.x509.not_after") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.tls.client.x509.not_after", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.tls.client.x509.not_after".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_tls_client_x509_not_after")?;
                    event.remove("beyondtrust_epm.event.tls.client.x509.not_after");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.not_after").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.not_after", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.client.x509.not_before") && event.get_str("beyondtrust_epm.event.tls.client.x509.not_before") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.tls.client.x509.not_before") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.tls.client.x509.not_before", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.tls.client.x509.not_before".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_tls_client_x509_not_before")?;
                    event.remove("beyondtrust_epm.event.tls.client.x509.not_before");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.not_before").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.not_before", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.public_key_algorithm").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.public_key_algorithm", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.public_key_curve").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.public_key_curve", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.tls.client.x509.public_key_exponent") {
            if let Some(val) = event.get("beyondtrust_epm.event.tls.client.x509.public_key_exponent") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.tls.client.x509.public_key_exponent".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.tls.client.x509.public_key_exponent", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_tls_client_x509_public_key_exponent_to_long")?;
                    event.remove("beyondtrust_epm.event.tls.client.x509.public_key_exponent");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.public_key_exponent").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.public_key_exponent", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.tls.client.x509.public_key_size") {
            if let Some(val) = event.get("beyondtrust_epm.event.tls.client.x509.public_key_size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.tls.client.x509.public_key_size".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.tls.client.x509.public_key_size", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_tls_client_x509_public_key_size_to_long")?;
                    event.remove("beyondtrust_epm.event.tls.client.x509.public_key_size");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.public_key_size").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.public_key_size", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.serial_number", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.signature_algorithm").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.signature_algorithm", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.subject.common_name").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.subject.common_name", |event| {
                event.append_unique("tls.client.x509.subject.common_name", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.subject.country").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.subject.country", |event| {
                event.append_unique("tls.client.x509.subject.country", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.subject.distinguished_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.subject.distinguished_name", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.subject.locality").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.subject.locality", |event| {
                event.append_unique("tls.client.x509.subject.locality", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.subject.organization").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.subject.organization", |event| {
                event.append_unique("tls.client.x509.subject.organization", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.subject.organizational_unit").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.subject.organizational_unit", |event| {
                event.append_unique("tls.client.x509.subject.organizational_unit", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.client.x509.subject.state_or_province").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.client.x509.subject.state_or_province", |event| {
                event.append_unique("tls.client.x509.subject.state_or_province", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.client.x509.version_number").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.client.x509.version_number", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.curve").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.curve", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.tls.established") {
            if let Some(val) = event.get("beyondtrust_epm.event.tls.established") {
                let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.tls.established".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.tls.established", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_tls_established_to_boolean")?;
                    event.remove("beyondtrust_epm.event.tls.established");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.established").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.established", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.next_protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.next_protocol", v)?;
        }

        if event.has_value("tls.next_protocol") {
            map_strings(event, "tls.next_protocol", "tls.next_protocol", str::to_lowercase)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.tls.resumed") {
            if let Some(val) = event.get("beyondtrust_epm.event.tls.resumed") {
                let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.tls.resumed".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.tls.resumed", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_tls_resumed_to_boolean")?;
                    event.remove("beyondtrust_epm.event.tls.resumed");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.resumed").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.resumed", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.certificate").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.certificate", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.certificate_chain").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.certificate_chain", |event| {
                event.append_unique("tls.server.certificate_chain", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.hash.md5").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.hash.md5", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.server.hash.md5") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.tls.server.hash.md5").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.hash.sha1").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.hash.sha1", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.server.hash.sha1") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.tls.server.hash.sha1").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.hash.sha256").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.hash.sha256", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.server.hash.sha256") };
        if _cond {
            event.append_unique("related.hash", json!(event.get("beyondtrust_epm.event.tls.server.hash.sha256").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.issuer").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.issuer", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.ja3s").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.ja3s", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.server.not_after") && event.get_str("beyondtrust_epm.event.tls.server.not_after") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.tls.server.not_after") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.tls.server.not_after", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.tls.server.not_after".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_tls_server_not_after")?;
                    event.remove("beyondtrust_epm.event.tls.server.not_after");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.not_after").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.not_after", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.server.not_before") && event.get_str("beyondtrust_epm.event.tls.server.not_before") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.tls.server.not_before") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.tls.server.not_before", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.tls.server.not_before".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_tls_server_not_before")?;
                    event.remove("beyondtrust_epm.event.tls.server.not_before");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.not_before").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.not_before", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.subject", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.alternative_names").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.alternative_names", |event| {
                event.append_unique("tls.server.x509.alternative_names", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.issuer.common_name").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.issuer.common_name", |event| {
                event.append_unique("tls.server.x509.issuer.common_name", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.issuer.country").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.issuer.country", |event| {
                event.append_unique("tls.server.x509.issuer.country", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.issuer.distinguished_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.issuer.distinguished_name", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.issuer.locality").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.issuer.locality", |event| {
                event.append_unique("tls.server.x509.issuer.locality", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.issuer.organization").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.issuer.organization", |event| {
                event.append_unique("tls.server.x509.issuer.organization", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.issuer.organizational_unit").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.issuer.organizational_unit", |event| {
                event.append_unique("tls.server.x509.issuer.organizational_unit", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.issuer.state_or_province").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.issuer.state_or_province", |event| {
                event.append_unique("tls.server.x509.issuer.state_or_province", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.server.x509.not_after") && event.get_str("beyondtrust_epm.event.tls.server.x509.not_after") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.tls.server.x509.not_after") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.tls.server.x509.not_after", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.tls.server.x509.not_after".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_tls_server_x509_not_after")?;
                    event.remove("beyondtrust_epm.event.tls.server.x509.not_after");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.not_after").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.not_after", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.tls.server.x509.not_before") && event.get_str("beyondtrust_epm.event.tls.server.x509.not_before") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("beyondtrust_epm.event.tls.server.x509.not_before") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("beyondtrust_epm.event.tls.server.x509.not_before", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "beyondtrust_epm.event.tls.server.x509.not_before".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_tls_server_x509_not_before")?;
                    event.remove("beyondtrust_epm.event.tls.server.x509.not_before");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.not_before").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.not_before", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.public_key_algorithm").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.public_key_algorithm", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.public_key_curve").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.public_key_curve", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.tls.server.x509.public_key_exponent") {
            if let Some(val) = event.get("beyondtrust_epm.event.tls.server.x509.public_key_exponent") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.tls.server.x509.public_key_exponent".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.tls.server.x509.public_key_exponent", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_tls_server_x509_public_key_exponent_to_long")?;
                    event.remove("beyondtrust_epm.event.tls.server.x509.public_key_exponent");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.public_key_exponent").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.public_key_exponent", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.tls.server.x509.public_key_size") {
            if let Some(val) = event.get("beyondtrust_epm.event.tls.server.x509.public_key_size") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.tls.server.x509.public_key_size".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.tls.server.x509.public_key_size", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_tls_server_x509_public_key_size_to_long")?;
                    event.remove("beyondtrust_epm.event.tls.server.x509.public_key_size");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.public_key_size").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.public_key_size", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.serial_number").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.serial_number", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.signature_algorithm").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.signature_algorithm", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.subject.common_name").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.subject.common_name", |event| {
                event.append_unique("tls.server.x509.subject.common_name", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.subject.country").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.subject.country", |event| {
                event.append_unique("tls.server.x509.subject.country", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.subject.distinguished_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.subject.distinguished_name", v)?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.subject.locality").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.subject.locality", |event| {
                event.append_unique("tls.server.x509.subject.locality", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.subject.organization").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.subject.organization", |event| {
                event.append_unique("tls.server.x509.subject.organization", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.subject.organizational_unit").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.subject.organizational_unit", |event| {
                event.append_unique("tls.server.x509.subject.organizational_unit", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.get("beyondtrust_epm.event.tls.server.x509.subject.state_or_province").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "beyondtrust_epm.event.tls.server.x509.subject.state_or_province", |event| {
                event.append_unique("tls.server.x509.subject.state_or_province", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.server.x509.version_number").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.server.x509.version_number", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.version", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.tls.version_protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("tls.version_protocol", v)?;
        }

        if event.has_value("tls.version_protocol") {
            map_strings(event, "tls.version_protocol", "tls.version_protocol", str::to_lowercase)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.extension").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.extension", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.fragment").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.fragment", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.full").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.full", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.original").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.original", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.password").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.password", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.path").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.path", v)?;
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("beyondtrust_epm.event.url.port") {
            if let Some(val) = event.get("beyondtrust_epm.event.url.port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "beyondtrust_epm.event.url.port".into(),
                        message,
                    })?;
                event.set("beyondtrust_epm.event.url.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_url_port_to_long")?;
                    event.remove("beyondtrust_epm.event.url.port");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.port", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.query").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.query", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.registered_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.registered_domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.scheme").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.scheme", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.subdomain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.subdomain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.top_level_domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.top_level_domain", v)?;
        }

        if let Some(v) = event.get("beyondtrust_epm.event.url.username").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.username", v)?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.url.username") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.url.username").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.Name") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("beyondtrust_epm.event.client.Name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("beyondtrust_epm.event.client.user.DomainIdentifier") };
        if _cond {
            event.append_unique("related.user", json!(event.get("beyondtrust_epm.event.client.user.DomainIdentifier").map_or_else(String::new, template_to_string)))?;
        }

            event.remove("beyondtrust_epm.event.client.address");
            event.remove("beyondtrust_epm.event.client.as.number");
            event.remove("beyondtrust_epm.event.client.as.organization.name");
            event.remove("beyondtrust_epm.event.client.bytes");
            event.remove("beyondtrust_epm.event.client.domain");
            event.remove("beyondtrust_epm.event.client.geo.city_name");
            event.remove("beyondtrust_epm.event.client.geo.continent_code");
            event.remove("beyondtrust_epm.event.client.geo.continent_name");
            event.remove("beyondtrust_epm.event.client.geo.country_iso_code");
            event.remove("beyondtrust_epm.event.client.geo.country_name");
            event.remove("beyondtrust_epm.event.client.geo.name");
            event.remove("beyondtrust_epm.event.client.geo.postal_code");
            event.remove("beyondtrust_epm.event.client.geo.region_iso_code");
            event.remove("beyondtrust_epm.event.client.geo.region_name");
            event.remove("beyondtrust_epm.event.client.geo.timezone");
            event.remove("beyondtrust_epm.event.client.ip");
            event.remove("beyondtrust_epm.event.client.mac");
            event.remove("beyondtrust_epm.event.client.nat.ip");
            event.remove("beyondtrust_epm.event.client.nat.port");
            event.remove("beyondtrust_epm.event.client.packets");
            event.remove("beyondtrust_epm.event.client.port");
            event.remove("beyondtrust_epm.event.client.registered_domain");
            event.remove("beyondtrust_epm.event.client.subdomain");
            event.remove("beyondtrust_epm.event.client.top_level_domain");
            event.remove("beyondtrust_epm.event.client.user.domain");
            event.remove("beyondtrust_epm.event.client.user.email");
            event.remove("beyondtrust_epm.event.client.user.full_name");
            event.remove("beyondtrust_epm.event.client.user.group.domain");
            event.remove("beyondtrust_epm.event.client.user.group.id");
            event.remove("beyondtrust_epm.event.client.user.group.name");
            event.remove("beyondtrust_epm.event.client.user.hash");
            event.remove("beyondtrust_epm.event.client.user.id");
            event.remove("beyondtrust_epm.event.client.user.name");
            event.remove("beyondtrust_epm.event.client.user.roles");
            event.remove("beyondtrust_epm.event.destination.address");
            event.remove("beyondtrust_epm.event.destination.as.number");
            event.remove("beyondtrust_epm.event.destination.as.organization.name");
            event.remove("beyondtrust_epm.event.destination.bytes");
            event.remove("beyondtrust_epm.event.destination.domain");
            event.remove("beyondtrust_epm.event.destination.geo.city_name");
            event.remove("beyondtrust_epm.event.destination.geo.continent_code");
            event.remove("beyondtrust_epm.event.destination.geo.continent_name");
            event.remove("beyondtrust_epm.event.destination.geo.country_iso_code");
            event.remove("beyondtrust_epm.event.destination.geo.country_name");
            event.remove("beyondtrust_epm.event.destination.geo.name");
            event.remove("beyondtrust_epm.event.destination.geo.postal_code");
            event.remove("beyondtrust_epm.event.destination.geo.region_iso_code");
            event.remove("beyondtrust_epm.event.destination.geo.region_name");
            event.remove("beyondtrust_epm.event.destination.geo.timezone");
            event.remove("beyondtrust_epm.event.destination.ip");
            event.remove("beyondtrust_epm.event.destination.mac");
            event.remove("beyondtrust_epm.event.destination.nat.ip");
            event.remove("beyondtrust_epm.event.destination.nat.port");
            event.remove("beyondtrust_epm.event.destination.packets");
            event.remove("beyondtrust_epm.event.destination.port");
            event.remove("beyondtrust_epm.event.destination.registered_domain");
            event.remove("beyondtrust_epm.event.destination.subdomain");
            event.remove("beyondtrust_epm.event.destination.top_level_domain");
            event.remove("beyondtrust_epm.event.destination.user.domain");
            event.remove("beyondtrust_epm.event.destination.user.email");
            event.remove("beyondtrust_epm.event.destination.user.full_name");
            event.remove("beyondtrust_epm.event.destination.user.group.domain");
            event.remove("beyondtrust_epm.event.destination.user.group.id");
            event.remove("beyondtrust_epm.event.destination.user.group.name");
            event.remove("beyondtrust_epm.event.destination.user.hash");
            event.remove("beyondtrust_epm.event.destination.user.id");
            event.remove("beyondtrust_epm.event.destination.user.name");
            event.remove("beyondtrust_epm.event.destination.user.roles");
            event.remove("beyondtrust_epm.event.dns.header_flags");
            event.remove("beyondtrust_epm.event.dns.id");
            event.remove("beyondtrust_epm.event.dns.op_code");
            event.remove("beyondtrust_epm.event.dns.question.class");
            event.remove("beyondtrust_epm.event.dns.question.name");
            event.remove("beyondtrust_epm.event.dns.question.registered_domain");
            event.remove("beyondtrust_epm.event.dns.question.subdomain");
            event.remove("beyondtrust_epm.event.dns.question.top_level_domain");
            event.remove("beyondtrust_epm.event.dns.question.type");
            event.remove("beyondtrust_epm.event.dns.resolved_ip");
            event.remove("beyondtrust_epm.event.dns.response_code");
            event.remove("beyondtrust_epm.event.dns.type");
            event.remove("beyondtrust_epm.event.http.request.body.bytes");
            event.remove("beyondtrust_epm.event.http.request.body.content");
            event.remove("beyondtrust_epm.event.http.request.bytes");
            event.remove("beyondtrust_epm.event.http.request.id");
            event.remove("beyondtrust_epm.event.http.request.method");
            event.remove("beyondtrust_epm.event.http.request.mime_type");
            event.remove("beyondtrust_epm.event.http.request.referrer");
            event.remove("beyondtrust_epm.event.http.response.body.bytes");
            event.remove("beyondtrust_epm.event.http.response.body.content");
            event.remove("beyondtrust_epm.event.http.response.bytes");
            event.remove("beyondtrust_epm.event.http.response.mime_type");
            event.remove("beyondtrust_epm.event.http.response.status_code");
            event.remove("beyondtrust_epm.event.http.version");
            event.remove("beyondtrust_epm.event.network.application");
            event.remove("beyondtrust_epm.event.network.bytes");
            event.remove("beyondtrust_epm.event.network.community_id");
            event.remove("beyondtrust_epm.event.network.direction");
            event.remove("beyondtrust_epm.event.network.forwarded_ip");
            event.remove("beyondtrust_epm.event.network.iana_number");
            event.remove("beyondtrust_epm.event.network.name");
            event.remove("beyondtrust_epm.event.network.packets");
            event.remove("beyondtrust_epm.event.network.protocol");
            event.remove("beyondtrust_epm.event.network.transport");
            event.remove("beyondtrust_epm.event.network.type");
            event.remove("beyondtrust_epm.event.network.vlan.id");
            event.remove("beyondtrust_epm.event.network.vlan.name");
            event.remove("beyondtrust_epm.event.server.address");
            event.remove("beyondtrust_epm.event.server.as.number");
            event.remove("beyondtrust_epm.event.server.as.organization.name");
            event.remove("beyondtrust_epm.event.server.bytes");
            event.remove("beyondtrust_epm.event.server.domain");
            event.remove("beyondtrust_epm.event.server.geo.city_name");
            event.remove("beyondtrust_epm.event.server.geo.continent_code");
            event.remove("beyondtrust_epm.event.server.geo.continent_name");
            event.remove("beyondtrust_epm.event.server.geo.country_iso_code");
            event.remove("beyondtrust_epm.event.server.geo.country_name");
            event.remove("beyondtrust_epm.event.server.geo.name");
            event.remove("beyondtrust_epm.event.server.geo.postal_code");
            event.remove("beyondtrust_epm.event.server.geo.region_iso_code");
            event.remove("beyondtrust_epm.event.server.geo.region_name");
            event.remove("beyondtrust_epm.event.server.geo.timezone");
            event.remove("beyondtrust_epm.event.server.ip");
            event.remove("beyondtrust_epm.event.server.mac");
            event.remove("beyondtrust_epm.event.server.nat.ip");
            event.remove("beyondtrust_epm.event.server.nat.port");
            event.remove("beyondtrust_epm.event.server.packets");
            event.remove("beyondtrust_epm.event.server.port");
            event.remove("beyondtrust_epm.event.server.registered_domain");
            event.remove("beyondtrust_epm.event.server.subdomain");
            event.remove("beyondtrust_epm.event.server.top_level_domain");
            event.remove("beyondtrust_epm.event.server.user.domain");
            event.remove("beyondtrust_epm.event.server.user.email");
            event.remove("beyondtrust_epm.event.server.user.full_name");
            event.remove("beyondtrust_epm.event.server.user.group.domain");
            event.remove("beyondtrust_epm.event.server.user.group.id");
            event.remove("beyondtrust_epm.event.server.user.group.name");
            event.remove("beyondtrust_epm.event.server.user.hash");
            event.remove("beyondtrust_epm.event.server.user.id");
            event.remove("beyondtrust_epm.event.server.user.name");
            event.remove("beyondtrust_epm.event.server.user.roles");
            event.remove("beyondtrust_epm.event.source.address");
            event.remove("beyondtrust_epm.event.source.as.number");
            event.remove("beyondtrust_epm.event.source.as.organization.name");
            event.remove("beyondtrust_epm.event.source.bytes");
            event.remove("beyondtrust_epm.event.source.domain");
            event.remove("beyondtrust_epm.event.source.geo.city_name");
            event.remove("beyondtrust_epm.event.source.geo.continent_code");
            event.remove("beyondtrust_epm.event.source.geo.continent_name");
            event.remove("beyondtrust_epm.event.source.geo.country_iso_code");
            event.remove("beyondtrust_epm.event.source.geo.country_name");
            event.remove("beyondtrust_epm.event.source.geo.name");
            event.remove("beyondtrust_epm.event.source.geo.postal_code");
            event.remove("beyondtrust_epm.event.source.geo.region_iso_code");
            event.remove("beyondtrust_epm.event.source.geo.region_name");
            event.remove("beyondtrust_epm.event.source.geo.timezone");
            event.remove("beyondtrust_epm.event.source.ip");
            event.remove("beyondtrust_epm.event.source.mac");
            event.remove("beyondtrust_epm.event.source.nat.ip");
            event.remove("beyondtrust_epm.event.source.nat.port");
            event.remove("beyondtrust_epm.event.source.packets");
            event.remove("beyondtrust_epm.event.source.port");
            event.remove("beyondtrust_epm.event.source.registered_domain");
            event.remove("beyondtrust_epm.event.source.subdomain");
            event.remove("beyondtrust_epm.event.source.top_level_domain");
            event.remove("beyondtrust_epm.event.source.user.domain");
            event.remove("beyondtrust_epm.event.source.user.email");
            event.remove("beyondtrust_epm.event.source.user.full_name");
            event.remove("beyondtrust_epm.event.source.user.group.domain");
            event.remove("beyondtrust_epm.event.source.user.group.id");
            event.remove("beyondtrust_epm.event.source.user.group.name");
            event.remove("beyondtrust_epm.event.source.user.hash");
            event.remove("beyondtrust_epm.event.source.user.id");
            event.remove("beyondtrust_epm.event.source.user.name");
            event.remove("beyondtrust_epm.event.source.user.roles");
            event.remove("beyondtrust_epm.event.tls.cipher");
            event.remove("beyondtrust_epm.event.tls.client.certificate");
            event.remove("beyondtrust_epm.event.tls.client.certificate_chain");
            event.remove("beyondtrust_epm.event.tls.client.hash.md5");
            event.remove("beyondtrust_epm.event.tls.client.hash.sha1");
            event.remove("beyondtrust_epm.event.tls.client.hash.sha256");
            event.remove("beyondtrust_epm.event.tls.client.issuer");
            event.remove("beyondtrust_epm.event.tls.client.ja3");
            event.remove("beyondtrust_epm.event.tls.client.not_after");
            event.remove("beyondtrust_epm.event.tls.client.not_before");
            event.remove("beyondtrust_epm.event.tls.client.server_name");
            event.remove("beyondtrust_epm.event.tls.client.subject");
            event.remove("beyondtrust_epm.event.tls.client.supported_ciphers");
            event.remove("beyondtrust_epm.event.tls.client.x509.alternative_names");
            event.remove("beyondtrust_epm.event.tls.client.x509.issuer.common_name");
            event.remove("beyondtrust_epm.event.tls.client.x509.issuer.country");
            event.remove("beyondtrust_epm.event.tls.client.x509.issuer.distinguished_name");
            event.remove("beyondtrust_epm.event.tls.client.x509.issuer.locality");
            event.remove("beyondtrust_epm.event.tls.client.x509.issuer.organization");
            event.remove("beyondtrust_epm.event.tls.client.x509.issuer.organizational_unit");
            event.remove("beyondtrust_epm.event.tls.client.x509.issuer.state_or_province");
            event.remove("beyondtrust_epm.event.tls.client.x509.not_after");
            event.remove("beyondtrust_epm.event.tls.client.x509.not_before");
            event.remove("beyondtrust_epm.event.tls.client.x509.public_key_algorithm");
            event.remove("beyondtrust_epm.event.tls.client.x509.public_key_curve");
            event.remove("beyondtrust_epm.event.tls.client.x509.public_key_exponent");
            event.remove("beyondtrust_epm.event.tls.client.x509.public_key_size");
            event.remove("beyondtrust_epm.event.tls.client.x509.serial_number");
            event.remove("beyondtrust_epm.event.tls.client.x509.signature_algorithm");
            event.remove("beyondtrust_epm.event.tls.client.x509.subject.common_name");
            event.remove("beyondtrust_epm.event.tls.client.x509.subject.country");
            event.remove("beyondtrust_epm.event.tls.client.x509.subject.distinguished_name");
            event.remove("beyondtrust_epm.event.tls.client.x509.subject.locality");
            event.remove("beyondtrust_epm.event.tls.client.x509.subject.organization");
            event.remove("beyondtrust_epm.event.tls.client.x509.subject.organizational_unit");
            event.remove("beyondtrust_epm.event.tls.client.x509.subject.state_or_province");
            event.remove("beyondtrust_epm.event.tls.client.x509.version_number");
            event.remove("beyondtrust_epm.event.tls.curve");
            event.remove("beyondtrust_epm.event.tls.established");
            event.remove("beyondtrust_epm.event.tls.next_protocol");
            event.remove("beyondtrust_epm.event.tls.resumed");
            event.remove("beyondtrust_epm.event.tls.server.certificate");
            event.remove("beyondtrust_epm.event.tls.server.certificate_chain");
            event.remove("beyondtrust_epm.event.tls.server.hash.md5");
            event.remove("beyondtrust_epm.event.tls.server.hash.sha1");
            event.remove("beyondtrust_epm.event.tls.server.hash.sha256");
            event.remove("beyondtrust_epm.event.tls.server.issuer");
            event.remove("beyondtrust_epm.event.tls.server.ja3s");
            event.remove("beyondtrust_epm.event.tls.server.not_after");
            event.remove("beyondtrust_epm.event.tls.server.not_before");
            event.remove("beyondtrust_epm.event.tls.server.subject");
            event.remove("beyondtrust_epm.event.tls.server.x509.alternative_names");
            event.remove("beyondtrust_epm.event.tls.server.x509.issuer.common_name");
            event.remove("beyondtrust_epm.event.tls.server.x509.issuer.country");
            event.remove("beyondtrust_epm.event.tls.server.x509.issuer.distinguished_name");
            event.remove("beyondtrust_epm.event.tls.server.x509.issuer.locality");
            event.remove("beyondtrust_epm.event.tls.server.x509.issuer.organization");
            event.remove("beyondtrust_epm.event.tls.server.x509.issuer.organizational_unit");
            event.remove("beyondtrust_epm.event.tls.server.x509.issuer.state_or_province");
            event.remove("beyondtrust_epm.event.tls.server.x509.not_after");
            event.remove("beyondtrust_epm.event.tls.server.x509.not_before");
            event.remove("beyondtrust_epm.event.tls.server.x509.public_key_algorithm");
            event.remove("beyondtrust_epm.event.tls.server.x509.public_key_curve");
            event.remove("beyondtrust_epm.event.tls.server.x509.public_key_exponent");
            event.remove("beyondtrust_epm.event.tls.server.x509.public_key_size");
            event.remove("beyondtrust_epm.event.tls.server.x509.serial_number");
            event.remove("beyondtrust_epm.event.tls.server.x509.signature_algorithm");
            event.remove("beyondtrust_epm.event.tls.server.x509.subject.common_name");
            event.remove("beyondtrust_epm.event.tls.server.x509.subject.country");
            event.remove("beyondtrust_epm.event.tls.server.x509.subject.distinguished_name");
            event.remove("beyondtrust_epm.event.tls.server.x509.subject.locality");
            event.remove("beyondtrust_epm.event.tls.server.x509.subject.organization");
            event.remove("beyondtrust_epm.event.tls.server.x509.subject.organizational_unit");
            event.remove("beyondtrust_epm.event.tls.server.x509.subject.state_or_province");
            event.remove("beyondtrust_epm.event.tls.server.x509.version_number");
            event.remove("beyondtrust_epm.event.tls.version");
            event.remove("beyondtrust_epm.event.tls.version_protocol");
            event.remove("beyondtrust_epm.event.url.domain");
            event.remove("beyondtrust_epm.event.url.extension");
            event.remove("beyondtrust_epm.event.url.fragment");
            event.remove("beyondtrust_epm.event.url.full");
            event.remove("beyondtrust_epm.event.url.original");
            event.remove("beyondtrust_epm.event.url.password");
            event.remove("beyondtrust_epm.event.url.path");
            event.remove("beyondtrust_epm.event.url.port");
            event.remove("beyondtrust_epm.event.url.query");
            event.remove("beyondtrust_epm.event.url.registered_domain");
            event.remove("beyondtrust_epm.event.url.scheme");
            event.remove("beyondtrust_epm.event.url.subdomain");
            event.remove("beyondtrust_epm.event.url.top_level_domain");
            event.remove("beyondtrust_epm.event.url.username");

        Ok(TransformResult::Continue)
    }
}
