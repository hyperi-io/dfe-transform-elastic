// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_bigipafm` pipeline.
pub struct PipelineBigipafm;

impl Transform for PipelineBigipafm {
    fn name(&self) -> &str {
        "pipeline_bigipafm"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

        event.set("observer.product", json!("Advanced Firewall Module"))?;

        let _cond = { event.has_value("json.date_time") && event.get_str("json.date_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.date_time") {
                match parse_date_out(&date_str, &["MMM dd yyyy HH:mm:ss", "ISO8601"], None, None) {
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

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.date_time").cloned() {
            event.set("@timestamp", v)?;
        }
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

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.dest.ip").cloned() {
            event.set("destination.ip", v)?;
        }
            Ok(())
        })();

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

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.dest.port").cloned() {
            event.set("destination.port", v)?;
        }
            Ok(())
        })();

        if let Some(v) = event.get("destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.port", v)?;
        }

            if event.has_value("json.action") {
                event.rename("json.action", "f5_bigip.log.action")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.action").cloned() {
            event.set("event.action", v)?;
        }
            Ok(())
        })();

        let _cond = { event.get_str("json.severity") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.severity") {
            if let Some(val) = event.get("json.severity") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.severity".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.severity.code", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_severity_to_long")?;
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
        if let Some(v) = event.get("f5_bigip.log.severity.code").cloned() {
            event.set("event.severity", v)?;
        }
            Ok(())
        })();

            if event.has_value("json.ip_protocol") {
                event.rename("json.ip_protocol", "f5_bigip.log.ip_protocol")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if let Some(v) = event.get("f5_bigip.log.ip_protocol").cloned() {
            event.set("network.transport", v)?;
        }
            Ok(())
        })();

        if event.has_value("network.transport") {
            map_strings(event, "network.transport", "network.transport", str::to_lowercase)?;
        }

        let _cond = { event.get_str("json.source_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.source_ip") {
            if let Some(val) = event.get("json.source_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.source_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.source.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_ip_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.ip", v)?;
        }

        if let Some(v) = event.get("source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.ip", v)?;
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
            event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.source_port") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.source_port") {
            if let Some(val) = event.get("json.source_port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.source_port".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.source.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_source_port_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.source.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.port", v)?;
        }

        if let Some(v) = event.get("source.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.port", v)?;
        }

            if event.has_value("json.source_user_group") {
                event.rename("json.source_user_group", "f5_bigip.log.source.user_group")?;
            }

        if let Some(v) = event.get("f5_bigip.log.source.user_group").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.group.name", v)?;
        }

            if event.has_value("json.source_user") {
                event.rename("json.source_user", "f5_bigip.log.source.user")?;
            }

        if let Some(v) = event.get("f5_bigip.log.source.user").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.user.name", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.user", json!(event.get("source.user.name").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.acl_policy_name") {
                event.rename("json.acl_policy_name", "f5_bigip.log.acl.policy.name")?;
            }

            if event.has_value("json.acl_policy_type") {
                event.rename("json.acl_policy_type", "f5_bigip.log.acl.policy.type")?;
            }

            if event.has_value("json.acl_rule_name") {
                event.rename("json.acl_rule_name", "f5_bigip.log.acl.rule.name")?;
            }

        let _cond = { event.get_str("json.bigip_mgmt_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.bigip_mgmt_ip") {
            if let Some(val) = event.get("json.bigip_mgmt_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.bigip_mgmt_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.mgmt_ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_bigip_mgmt_ip_to_ip")?;
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
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.mgmt_ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.context_name") {
                event.rename("json.context_name", "f5_bigip.log.context.name")?;
            }

            if event.has_value("json.context_type") {
                event.rename("json.context_type", "f5_bigip.log.context.type")?;
            }

            if event.has_value("json.dest_fqdn") {
                event.rename("json.dest_fqdn", "f5_bigip.log.dest.fqdn")?;
            }

        if let Some(v) = event.get("f5_bigip.log.dest.fqdn").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.domain", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.dest.fqdn").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.device_product") {
                event.rename("json.device_product", "f5_bigip.log.device.product")?;
            }

            if event.has_value("json.device_vendor") {
                event.rename("json.device_vendor", "f5_bigip.log.device.vendor")?;
            }

            if event.has_value("json.device_version") {
                event.rename("json.device_version", "f5_bigip.log.device.version")?;
            }

            if event.has_value("json.drop_reason") {
                event.rename("json.drop_reason", "f5_bigip.log.drop_reason")?;
            }

            if event.has_value("json.dst_geo") {
                event.rename("json.dst_geo", "f5_bigip.log.dst.geo")?;
            }

            if event.has_value("json.errdefs_msg_name") {
                event.rename("json.errdefs_msg_name", "f5_bigip.log.errdefs.msg_name")?;
            }

            if event.has_value("json.errdefs_msgno") {
                event.rename("json.errdefs_msgno", "f5_bigip.log.errdefs.msgno")?;
            }

            if event.has_value("json.flow_id") {
                event.rename("json.flow_id", "f5_bigip.log.flow.id")?;
            }

            if event.has_value("json.partition_name") {
                event.rename("json.partition_name", "f5_bigip.log.partition_name")?;
            }

            if event.has_value("json.route_domain") {
                event.rename("json.route_domain", "f5_bigip.log.route_domain")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.route_domain").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.sa_translation_pool") {
                event.rename("json.sa_translation_pool", "f5_bigip.log.sa_translation.pool")?;
            }

            if event.has_value("json.sa_translation_type") {
                event.rename("json.sa_translation_type", "f5_bigip.log.sa_translation.type")?;
            }

            if event.has_value("json.send_to_vs") {
                event.rename("json.send_to_vs", "f5_bigip.log.send_to_vs")?;
            }

            if event.has_value("json.source_fqdn") {
                event.rename("json.source_fqdn", "f5_bigip.log.source.fqdn")?;
            }

        if let Some(v) = event.get("f5_bigip.log.source.fqdn").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.domain", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.source.fqdn").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.src_geo") {
                event.rename("json.src_geo", "f5_bigip.log.src.geo")?;
            }

            if event.has_value("json.telemetryEventCategory") {
                event.rename("json.telemetryEventCategory", "f5_bigip.log.telemetry.event.category")?;
            }

            if event.has_value("json.tenant") {
                event.rename("json.tenant", "f5_bigip.log.tenant")?;
            }

        let _cond = { event.get_str("json.translated_dest_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.translated_dest_ip") {
            if let Some(val) = event.get("json.translated_dest_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.translated_dest_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.translated.dest.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_translated_dest_ip_to_ip")?;
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
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.translated.dest.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.translated_dest_port") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.translated_dest_port") {
            if let Some(val) = event.get("json.translated_dest_port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.translated_dest_port".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.translated.dest.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_translated_dest_port_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.translated_ip_protocol") {
                event.rename("json.translated_ip_protocol", "f5_bigip.log.translated.ip_protocol")?;
            }

            if event.has_value("json.translated_route_domain") {
                event.rename("json.translated_route_domain", "f5_bigip.log.translated.route_domain")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.translated.route_domain").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.translated_source_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.translated_source_ip") {
            if let Some(val) = event.get("json.translated_source_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.translated_source_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.translated.source.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_translated_source_ip_to_ip")?;
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
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.translated.source.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.translated_source_port") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.translated_source_port") {
            if let Some(val) = event.get("json.translated_source_port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.translated_source_port".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.translated.source.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_translated_source_port_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.translated_vlan") {
                event.rename("json.translated_vlan", "f5_bigip.log.translated.vlan")?;
            }

            if event.has_value("json.acl_rule_uuid") {
                event.rename("json.acl_rule_uuid", "f5_bigip.log.acl.rule.uuid")?;
            }

            if event.has_value("json.src_zone") {
                event.rename("json.src_zone", "f5_bigip.log.src.zone")?;
            }

            if event.has_value("json.dest_ipint_categories") {
                event.rename("json.dest_ipint_categories", "f5_bigip.log.dest.ipint_categories")?;
            }

            if event.has_value("json.dest_vlan") {
                event.rename("json.dest_vlan", "f5_bigip.log.dest.vlan")?;
            }

            if event.has_value("json.dest_zone") {
                event.rename("json.dest_zone", "f5_bigip.log.dest.zone")?;
            }

            if event.has_value("json.source_ipint_categories") {
                event.rename("json.source_ipint_categories", "f5_bigip.log.source.ipint_categories")?;
            }

            if event.has_value("json.vlan") {
                event.rename("json.vlan", "f5_bigip.log.vlan")?;
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
            event.remove("f5_bigip.log.dest.ip");
            event.remove("f5_bigip.log.dest.port");
            event.remove("f5_bigip.log.action");
            event.remove("f5_bigip.log.hostname");
            event.remove("f5_bigip.log.severity.code");
            event.remove("f5_bigip.log.dest.fqdn");
            event.remove("f5_bigip.log.source.fqdn");
            event.remove("f5_bigip.log.application.name");
            event.remove("f5_bigip.log.ip_protocol");
            event.remove("f5_bigip.log.source.ip");
            event.remove("f5_bigip.log.source.port");
            event.remove("f5_bigip.log.source.user_group");
            event.remove("f5_bigip.log.source.user");
            Ok(())
        })();
        }

        Ok(TransformResult::Continue)
    }
}
