// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_bigipasm` pipeline.
pub struct PipelineBigipasm;

impl Transform for PipelineBigipasm {
    fn name(&self) -> &str {
        "pipeline_bigipasm"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

        let _cond = { event.has_value("json.severity") && event.get_str("json.severity").is_some_and(|s| ["emergency", "critical", "alert", "warning", "error"].contains(&s.to_lowercase().as_str())) };
        if _cond {
        event.set("event.kind", json!("alert"))?;
        }

        event.set("observer.product", json!("Application Security Module"))?;

        let _cond = { event.has_value("json.date_time") && event.get_str("json.date_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.date_time") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss", "ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.date_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.date_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_time_conversion")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.date_time").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("@timestamp", v)?;
        }

        let _cond = { event.get_str("json.ip_client") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.ip_client") {
            if let Some(val) = event.get("json.ip_client") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.ip_client".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_ip_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.client.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.ip", v)?;
        }

        if let Some(v) = event.get("client.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.ip", v)?;
        }

        if event.has_value("source.ip") {
            if let Some(ip_str) = event.get_string("source.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-City.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                    if let Some(v) = geo.get("country_iso_code") {
                        event.set("source.geo.country_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("country_name") {
                        event.set("source.geo.country_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("continent_name") {
                        event.set("source.geo.continent_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_iso_code") {
                        event.set("source.geo.region_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_name") {
                        event.set("source.geo.region_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("city_name") {
                        event.set("source.geo.city_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("timezone") {
                        event.set("source.geo.timezone", v.clone())?;
                    }
                    if let Some(v) = geo.get("location") {
                        event.set("source.geo.location", v.clone())?;
                    }
                }
            }
        }

        if event.has_value("source.ip") {
            if let Some(ip_str) = event.get_string("source.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-ASN.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                    if let Some(v) = geo.get("asn") {
                        event.set("source.as.asn", v.clone())?;
                    }
                    if let Some(v) = geo.get("organization_name") {
                        event.set("source.as.organization_name", v.clone())?;
                    }
                }
            }
        }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

        if let Some(v) = event.get("source.geo").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo", v)?;
        }

        if let Some(v) = event.get("source.as").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.as", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.dest_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.dest_ip") {
            if let Some(val) = event.get("json.dest_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.dest_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.dest.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_dest_ip_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.dest.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.ip", v)?;
        }

        if let Some(v) = event.get("destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.ip", v)?;
        }

        if event.has_value("destination.ip") {
            if let Some(ip_str) = event.get_string("destination.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-City.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                    if let Some(v) = geo.get("country_iso_code") {
                        event.set("destination.geo.country_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("country_name") {
                        event.set("destination.geo.country_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("continent_name") {
                        event.set("destination.geo.continent_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_iso_code") {
                        event.set("destination.geo.region_iso_code", v.clone())?;
                    }
                    if let Some(v) = geo.get("region_name") {
                        event.set("destination.geo.region_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("city_name") {
                        event.set("destination.geo.city_name", v.clone())?;
                    }
                    if let Some(v) = geo.get("timezone") {
                        event.set("destination.geo.timezone", v.clone())?;
                    }
                    if let Some(v) = geo.get("location") {
                        event.set("destination.geo.location", v.clone())?;
                    }
                }
            }
        }

        if event.has_value("destination.ip") {
            if let Some(ip_str) = event.get_string("destination.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-ASN.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                    if let Some(v) = geo.get("asn") {
                        event.set("destination.as.asn", v.clone())?;
                    }
                    if let Some(v) = geo.get("organization_name") {
                        event.set("destination.as.organization_name", v.clone())?;
                    }
                }
            }
        }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename("destination.as.organization_name", "destination.as.organization.name")?;
            }

        if let Some(v) = event.get("destination.geo").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo", v)?;
        }

        if let Some(v) = event.get("destination.as").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.as", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.dest_port") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.dest_port") {
            if let Some(val) = event.get("json.dest_port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.dest_port".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.dest.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_dest_port_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.dest.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.port", v)?;
        }

        if let Some(v) = event.get("destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.port", v)?;
        }

            if event.has_value("json.geo_location") {
                event.rename("json.geo_location", "f5_bigip.log.geo.location")?;
            }

        if let Some(v) = event.get("f5_bigip.log.geo.location").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.geo.country_iso_code", v)?;
        }

            if event.has_value("json.device_id") {
                event.rename("json.device_id", "f5_bigip.log.device.id")?;
            }

        if let Some(v) = event.get("f5_bigip.log.device.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.id", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("host.id").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.method") {
                event.rename("json.method", "f5_bigip.log.method")?;
            }

        if let Some(v) = event.get("f5_bigip.log.method").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.method", v)?;
        }

            if event.has_value("json.severity") {
                event.rename("json.severity", "f5_bigip.log.severity.name")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.severity.name").cloned() {
            event.set("log.level", v)?;
        }
            Ok(())
        })();

        if event.has_value("log.level") {
            map_strings(event, "log.level", "log.level", str::to_lowercase)?;
        }

            if event.has_value("json.protocol") {
                event.rename("json.protocol", "f5_bigip.log.protocol")?;
            }

        if let Some(v) = event.get("f5_bigip.log.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.protocol", v)?;
        }

        let _cond = { event.get_str("json.src_port") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.src_port") {
            if let Some(val) = event.get("json.src_port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.src_port".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.src.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_src_port_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.src.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.port", v)?;
        }

        if let Some(v) = event.get("source.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.port", v)?;
        }

            if event.has_value("json.username") {
                event.rename("json.username", "f5_bigip.log.username")?;
            }

        if let Some(v) = event.get("f5_bigip.log.username").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("user.name", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.attack_type") {
                event.rename("json.attack_type", "f5_bigip.log.attack.type")?;
            }

            if event.has_value("json.blocking_exception_reason") {
                event.rename("json.blocking_exception_reason", "f5_bigip.log.blocking_exception_reason")?;
            }

            if event.has_value("json.captcha_result") {
                event.rename("json.captcha_result", "f5_bigip.log.captcha_result")?;
            }

            if event.has_value("json.fragment") {
                event.rename("json.fragment", "f5_bigip.log.fragment")?;
            }

            if event.has_value("json.http_class_name") {
                event.rename("json.http_class_name", "f5_bigip.log.http.class_name")?;
            }

            if event.has_value("json.ip_address_intelligence") {
                event.rename("json.ip_address_intelligence", "f5_bigip.log.ip_address_intelligence")?;
            }

        let _cond = { event.get_str("json.management_ip_address") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.management_ip_address") {
            if let Some(val) = event.get("json.management_ip_address") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.management_ip_address".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.management.ip_address", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_management_ip_address_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.management.ip_address").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.management_ip_address_2") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.management_ip_address_2") {
            if let Some(val) = event.get("json.management_ip_address_2") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.management_ip_address_2".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.management.ip_address_2", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_management_ip_address_2_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.management.ip_address_2").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.has_value("json.policy_apply_date") && event.get_str("json.policy_apply_date") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.policy_apply_date") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.policy.apply_date", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.policy_apply_date".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_policy_apply_date")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.policy_name") {
                event.rename("json.policy_name", "f5_bigip.log.policy.name")?;
            }

            // Painless script
            // Source: def log; if (ctx.event?.original != null) {\n  log = ctx.event.original;\n} else if (ctx.json?.originalRawData != null) {\n  log = ctx.json.originalRawData;\n} if (log != null) {\n  def sAMAccountNameMatch = /sAMAccountName=([^\\\\)]+)/.matcher(log);\n  if (sAMAccountNameMatch.find()) {\n    ctx.f5_bigip.log.sam_account_name = sAMAccountNameMatch.group(1);\n  }\n\n  def userPrincipleNameMatch = /UserPrincipleName=([^\\\\)]+)/.matcher(log);\n  if (userPrincipleNameMatch.find()) {\n    ctx.f5_bigip.log.user_principle_name = userPrincipleNameMatch.group(1);\n  }\n\n  def userNameMatch = /User_Name=([^\\\\)]+)/.matcher(log);\n  if (userNameMatch.find()) {\n    ctx.f5_bigip.log.user_name = userNameMatch.group(1);\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def log; if (ctx.event?.original != null) {\n  log = ctx.event.original;\n} else if (ctx.json?.originalRawData != null) {\n  log = ctx.json.originalRawData;\n} if (log != null) {\n  def sAMAccountNameMatch = /sAMAccountName=([^\\\\)]+)/.matcher(log);\n  if (sAMAccountNameMatch.find()) {\n    ctx.f5_bigip.log.sam_account_name = sAMAccountNameMatch.group(1);\n  }\n\n  def userPrincipleNameMatch = /UserPrincipleName=([^\\\\)]+)/.matcher(log);\n  if (userPrincipleNameMatch.find()) {\n    ctx.f5_bigip.log.user_principle_name = userPrincipleNameMatch.group(1);\n  }\n\n  def userNameMatch = /User_Name=([^\\\\)]+)/.matcher(log);\n  if (userNameMatch.find()) {\n    ctx.f5_bigip.log.user_name = userNameMatch.group(1);\n  }\n}\n"#))?;

            if event.has_value("json.query_string") {
                event.rename("json.query_string", "f5_bigip.log.query.string")?;
            }

            if event.has_value("f5_bigip.log.sam_account_name") {
                event.rename("f5_bigip.log.sam_account_name", "f5_bigip.log.query.sam_account_name")?;
            }

        let _cond = { event.has_value("f5_bigip.log.query.sam_account_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("f5_bigip.log.query.sam_account_name").map_or_else(String::new, template_to_string)))?;
        }

            if event.has_value("f5_bigip.log.user_principle_name") {
                event.rename("f5_bigip.log.user_principle_name", "f5_bigip.log.query.user_principle_name")?;
            }

            if event.has_value("f5_bigip.log.user_name") {
                event.rename("f5_bigip.log.user_name", "f5_bigip.log.query.user_name")?;
            }

            if event.has_value("json.request") {
                event.rename("json.request", "f5_bigip.log.request.detail")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("f5_bigip.log.request.detail") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.method", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.path", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\nHost: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.protocol", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\nHost: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\nConnection: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.host", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\nConnection: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\nCache-Control: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.connection", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\nCache-Control: ") else { break 'dissect false };
                    remaining = rest;
                    captured.push(("f5_bigip.log.request.cache_control", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                }
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("f5_bigip.log.request.detail") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.method", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.path", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\r\nHost: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.protocol", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\r\nHost: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\r\nUser-Agent: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.host", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\r\nUser-Agent: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\r\nAccept: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.user_agent", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\r\nAccept: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\r\nX-Forwarded-For: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.accept", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\r\nX-Forwarded-For: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\r\n\r\n") else { break 'dissect false };
                    captured.push(("f5_bigip.log.request.x_forwarded_for", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\r\n\r\n") else { break 'dissect false };
                    remaining = rest;
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                }
            }
            Ok(())
        })();

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.request.host").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        if event.has_value("related.hosts") {
            map_strings(event, "related.hosts", "related.hosts", str::to_lowercase)?;
        }

        if event.has_value("f5_bigip.log.request.user_agent") {
            gsub_field(event, "f5_bigip.log.request.user_agent", "f5_bigip.log.request.user_agent", cached_regex!("(\\([^)]*)\\+(https?://)"), "$1%2b$2")?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.request.user_agent") {
            if let Some(s) = event.get_string("f5_bigip.log.request.user_agent") {
                match url_decode(&s) {
                    Some(decoded) => event.set("f5_bigip.log.request.user_agent", json!(decoded))?,
                    None => return Err(TransformError::ParseError {
                        path: "f5_bigip.log.request.user_agent".into(),
                        message: format!("cannot url-decode '{s}'"),
                    }),
                }
            }
        }
            Ok(())
        })();

        if event.has_value("f5_bigip.log.request.user_agent") {
            if let Some(ua_str) = event.get_string("f5_bigip.log.request.user_agent") {
                let ua_str = ua_str.to_string();
                // User agent parsing
                if let Ok(ua) = parse_user_agent(&ua_str) {
                    event.remove("user_agent");
                    event.set("user_agent.original", json!(ua_str))?;
                    if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                    if let Some(version) = ua.version { event.set("user_agent.version", json!(version))?; }
                    if let Some(os_name) = ua.os_name {
                        event.set("user_agent.os.name", json!(os_name))?;
                        if let Some(os_version) = ua.os_version {
                            event.set("user_agent.os.version", json!(os_version))?;
                            event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                        }
                    }
                    if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                }
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("f5_bigip.log.request.protocol") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find("/") else { break 'dissect false };
                    captured.push(("network.protocol", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("/") else { break 'dissect false };
                    remaining = rest;
                    captured.push(("http.version", remaining));
                    true
                };
                if matched {
                    for (path, value) in captured {
                        event.set(path, value)?;
                    }
                }
            }
            Ok(())
        })();

        if event.has_value("network.protocol") {
            map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
        }

        let _cond = { event.has_value("network.protocol") && event.has_value("f5_bigip.log.request.host") && event.has_value("f5_bigip.log.request.protocol") };
        if _cond {
            // Painless script
            // Source: if (ctx.url == null) {\n    ctx.url = new HashMap();\n}\nctx.url.original = ctx.network.protocol + '://' +ctx.f5_bigip.log.request.host + ctx.f5_bigip.log.request.path\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"if (ctx.url == null) {\n    ctx.url = new HashMap();\n}\nctx.url.original = ctx.network.protocol + '://' +ctx.f5_bigip.log.request.host + ctx.f5_bigip.log.request.path\n"#))?;
        }

        let _cond = { event.has_value("url.original") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if !uri_parts(event, "url.original", "url", true, false)?
                && event.get_str("url.original").is_some_and(|value| !value.is_empty())
            {
                return Err(TransformError::ParseError {
                    path: "url.original".into(),
                    message: "uri_parts: not a parseable URI".into(),
                });
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("f5_bigip.log.request.x_forwarded_for") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.request.x_forwarded_for") {
            if let Some(val) = event.get("f5_bigip.log.request.x_forwarded_for") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "f5_bigip.log.request.x_forwarded_for".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.request.x_forwarded_for", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_x_forwarded_for_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.request.x_forwarded_for").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.request_status") {
                event.rename("json.request_status", "f5_bigip.log.request.status")?;
            }

        let _cond = { event.get_str("json.response_code") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.response_code") {
            if let Some(val) = event.get("json.response_code") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.response_code".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.response.code", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_response_code_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.route_domain") {
                event.rename("json.route_domain", "f5_bigip.log.route_domain")?;
            }

            if event.has_value("json.session_id") {
                event.rename("json.session_id", "f5_bigip.log.session.id")?;
            }

            if event.has_value("json.sig_ids") {
                event.rename("json.sig_ids", "f5_bigip.log.sig.ids")?;
            }

            if event.has_value("json.sig_names") {
                event.rename("json.sig_names", "f5_bigip.log.sig.names")?;
            }

            if event.has_value("json.staged_sig_ids") {
                event.rename("json.staged_sig_ids", "f5_bigip.log.staged.sig.ids")?;
            }

            if event.has_value("json.staged_sig_names") {
                event.rename("json.staged_sig_names", "f5_bigip.log.staged.sig.names")?;
            }

            if event.has_value("json.staged_threat_campaign_names") {
                event.rename("json.staged_threat_campaign_names", "f5_bigip.log.staged.threat_campaign_names")?;
            }

            if event.has_value("json.sub_violations") {
                event.rename("json.sub_violations", "f5_bigip.log.sub_violations")?;
            }

            if event.has_value("json.support_id") {
                event.rename("json.support_id", "f5_bigip.log.support.id")?;
            }

            if event.has_value("json.telemetryEventCategory") {
                event.rename("json.telemetryEventCategory", "f5_bigip.log.telemetry.event.category")?;
            }

            if event.has_value("json.tenant") {
                event.rename("json.tenant", "f5_bigip.log.tenant")?;
            }

            if event.has_value("json.threat_campaign_names") {
                event.rename("json.threat_campaign_names", "f5_bigip.log.threat_campaign_names")?;
            }

            if event.has_value("json.uri") {
                event.rename("json.uri", "f5_bigip.log.uri")?;
            }

        let _cond = { event.get_str("json.violation_rating") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.violation_rating") {
            if let Some(val) = event.get("json.violation_rating") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.violation_rating".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.violation.rating", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_violation_rating_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.violations") {
                event.rename("json.violations", "f5_bigip.log.violations")?;
            }

            if event.has_value("json.virus_name") {
                event.rename("json.virus_name", "f5_bigip.log.virus_name")?;
            }

            if event.has_value("json.web_application_name") {
                event.rename("json.web_application_name", "f5_bigip.log.web_application_name")?;
            }

            if event.has_value("json.websocket_direction") {
                event.rename("json.websocket_direction", "f5_bigip.log.websocket.direction")?;
            }

            if event.has_value("json.websocket_message_type") {
                event.rename("json.websocket_message_type", "f5_bigip.log.websocket.message_type")?;
            }

        if event.has_value("json.x_forwarded_for_header_value") {
            if let Some(s) = event.get_string("json.x_forwarded_for_header_value") {
                let mut parts: Vec<Value> = cached_regex!(",\\s*")
                    .split(&s)
                    .into_iter()
                    .map(|p| json!(p))
                    .collect();
                while parts.last().and_then(Value::as_str) == Some("") {
                    parts.pop();
                }
                event.set("json.x_forwarded_for_header_value", Value::Array(parts))?;
            }
        }

        let _cond = { event.get_str("json.x_forwarded_for_header_value") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.x_forwarded_for_header_value") {
            if let Some(val) = event.get("json.x_forwarded_for_header_value") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.x_forwarded_for_header_value".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.x_forwarded_for_header_value", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_x_forwarded_for_header_value_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get("f5_bigip.log.x_forwarded_for_header_value").is_some_and(|v| v.is_array()) };
        if _cond {
            foreach_array(event, "f5_bigip.log.x_forwarded_for_header_value", |event| {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
                })();
                Ok(())
            })?;
        }

            if event.has_value("json.geo_info") {
                event.rename("json.geo_info", "f5_bigip.log.geo.info")?;
            }

            if event.has_value("json.headers") {
                event.rename("json.headers", "f5_bigip.log.headers")?;
            }

            if event.has_value("json.ip_route_domain") {
                event.rename("json.ip_route_domain", "f5_bigip.log.ip_route_domain")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.ip_route_domain").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.is_trunct") {
                event.rename("json.is_trunct", "f5_bigip.log.is_trunct")?;
            }

            if event.has_value("json.resp") {
                event.rename("json.resp", "f5_bigip.log.resp")?;
            }

            if event.has_value("json.unit_host") {
                event.rename("json.unit_host", "f5_bigip.log.unit_host")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.unit_host").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.violate_details") {
                event.rename("json.violate_details", "f5_bigip.log.violate_details")?;
            }

            if event.has_value("json.microservice") {
                event.rename("json.microservice", "f5_bigip.log.microservice")?;
            }

            if event.has_value("json.response") {
                event.rename("json.response", "f5_bigip.log.response.value")?;
            }

            if event.has_value("json.sig_cves") {
                event.rename("json.sig_cves", "f5_bigip.log.sig.cves")?;
            }

            if event.has_value("json.staged_sig_cves") {
                event.rename("json.staged_sig_cves", "f5_bigip.log.staged.sig.cves")?;
            }

            if event.has_value("json.tap_event_id") {
                event.rename("json.tap_event_id", "f5_bigip.log.tap.event_id")?;
            }

            if event.has_value("json.tap_vid") {
                event.rename("json.tap_vid", "f5_bigip.log.tap.vid")?;
            }

            if event.has_value("json.vs_name") {
                event.rename("json.vs_name", "f5_bigip.log.vs_name")?;
            }

            if event.has_value("json.compression_method") {
                event.rename("json.compression_method", "f5_bigip.log.compression_method")?;
            }

            if event.has_value("json.client_type") {
                event.rename("json.client_type", "f5_bigip.log.client.type")?;
            }

            if event.has_value("json.conviction_traps") {
                event.rename("json.conviction_traps", "f5_bigip.log.conviction_traps")?;
            }

            if event.has_value("json.credential_stuffing_lookup_result") {
                event.rename("json.credential_stuffing_lookup_result", "f5_bigip.log.credential_stuffing_lookup_result")?;
            }

            if event.has_value("json.enforced_by") {
                event.rename("json.enforced_by", "f5_bigip.log.enforced_by")?;
            }

            if event.has_value("json.enforcement_action") {
                event.rename("json.enforcement_action", "f5_bigip.log.enforcement_action")?;
            }

        let _cond = { event.has_value("json.epoch_time") && event.get_str("json.epoch_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.epoch_time") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.epoch_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.epoch_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_epoch_time")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.ip_with_route_domain") {
                event.rename("json.ip_with_route_domain", "f5_bigip.log.ip_with_route_domain")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.ip_with_route_domain").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.is_truncated") {
                event.rename("json.is_truncated", "f5_bigip.log.is_truncated")?;
            }

            if event.has_value("json.likely_false_positive_sig_ids") {
                event.rename("json.likely_false_positive_sig_ids", "f5_bigip.log.likely_false_positive_sig_ids")?;
            }

            if event.has_value("json.login_result") {
                event.rename("json.login_result", "f5_bigip.log.login_result")?;
            }

            if event.has_value("json.mobile_application_name") {
                event.rename("json.mobile_application_name", "f5_bigip.log.mobile_application.name")?;
            }

            if event.has_value("json.mobile_application_version") {
                event.rename("json.mobile_application_version", "f5_bigip.log.mobile_application.version")?;
            }

            if event.has_value("json.operation_id") {
                event.rename("json.operation_id", "f5_bigip.log.operation.id")?;
            }

            if event.has_value("json.password_hash_prefix") {
                event.rename("json.password_hash_prefix", "f5_bigip.log.password_hash_prefix")?;
            }

            if event.has_value("json.protocol_info") {
                event.rename("json.protocol_info", "f5_bigip.log.protocol_info")?;
            }

            if event.has_value("json.sig_set_names") {
                event.rename("json.sig_set_names", "f5_bigip.log.sig.set_names")?;
            }

        let _cond = { event.get_str("json.slot_number") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.slot_number") {
            if let Some(val) = event.get("json.slot_number") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.slot_number".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.slot.number", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_slot_number")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.staged_sig_set_names") {
                event.rename("json.staged_sig_set_names", "f5_bigip.log.staged.sig.set_names")?;
            }

            if event.has_value("json.tap_requested_actions") {
                event.rename("json.tap_requested_actions", "f5_bigip.log.tap.requested_actions")?;
            }

        let _cond = { event.get_str("json.tap_sent_token") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.tap_sent_token") {
            if let Some(val) = event.get("json.tap_sent_token") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.tap_sent_token".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.tap.sent_token", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_tap_sent_token_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.tap_transaction_id") {
                event.rename("json.tap_transaction_id", "f5_bigip.log.tap.transaction_id")?;
            }

            if event.has_value("json.unit_hostname") {
                event.rename("json.unit_hostname", "f5_bigip.log.unit_hostname")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.unit_hostname").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.violation_details") {
                event.rename("json.violation_details", "f5_bigip.log.violation.details")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.remove("json");
            Ok(())
        })();

        let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.remove("f5_bigip.log.date_time");
            event.remove("f5_bigip.log.client.ip");
            event.remove("f5_bigip.log.dest.ip");
            event.remove("f5_bigip.log.dest.port");
            event.remove("f5_bigip.log.geo.location");
            event.remove("f5_bigip.log.device.id");
            event.remove("f5_bigip.log.hostname");
            event.remove("f5_bigip.log.method");
            event.remove("f5_bigip.log.protocol");
            event.remove("f5_bigip.log.src.port");
            event.remove("f5_bigip.log.severity.name");
            event.remove("f5_bigip.log.username");
            event.remove("f5_bigip.log.application.name");
            Ok(())
        })();
        }

        Ok(TransformResult::Continue)
    }
}
