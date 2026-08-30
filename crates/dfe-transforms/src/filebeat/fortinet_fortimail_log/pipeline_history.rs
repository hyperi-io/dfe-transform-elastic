// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_history` pipeline.
pub struct PipelineHistory;

impl Transform for PipelineHistory {
    fn name(&self) -> &str {
        "pipeline_history"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { event.get_str("temp.client_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("temp.client_ip") {
                if let Some(val) = event.get("temp.client_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "temp.client_ip".into(),
                            message,
                        })?;
                    event.set("fortinet_fortimail.log.client.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_client_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("fortinet_fortimail.log.client.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("fortinet_fortimail.log.client.ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("fortinet_fortimail.log.client.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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

                if event.has_value("temp.client_name") {
                    event.rename("temp.client_name", "fortinet_fortimail.log.client.name")?;
                }

            let _cond = { event.has_value("fortinet_fortimail.log.client.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("fortinet_fortimail.log.client.name").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("fortinet_fortimail.log.client.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.user.name", v)?;
            }

            let _cond = { event.get_str("temp.dst_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("temp.dst_ip") {
                if let Some(val) = event.get("temp.dst_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "temp.dst_ip".into(),
                            message,
                        })?;
                    event.set("fortinet_fortimail.log.destination_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dst_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("fortinet_fortimail.log.destination_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("fortinet_fortimail.log.destination_ip").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("fortinet_fortimail.log.destination_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

                if event.has_value("temp.direction") {
                    event.rename("temp.direction", "fortinet_fortimail.log.direction")?;
                }

            if let Some(v) = event.get("fortinet_fortimail.log.direction").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.direction", v)?;
            }

                if event.has_value("temp.from") {
                    event.rename("temp.from", "fortinet_fortimail.log.from")?;
                }

            let _cond = { event.has_value("fortinet_fortimail.log.from") };
            if _cond {
                event.append_unique("related.user", json!(event.get("fortinet_fortimail.log.from").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("fortinet_fortimail.log.from") };
            if _cond {
                event.append_unique("email.from.address", json!(event.get("fortinet_fortimail.log.from").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("temp.subject") {
                    event.rename("temp.subject", "fortinet_fortimail.log.subject")?;
                }

            if let Some(v) = event.get("fortinet_fortimail.log.subject").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.subject", v)?;
            }

                if event.has_value("temp.to") {
                    event.rename("temp.to", "fortinet_fortimail.log.to")?;
                }

            let _cond = { event.has_value("fortinet_fortimail.log.to") };
            if _cond {
                event.append_unique("related.user", json!(event.get("fortinet_fortimail.log.to").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("fortinet_fortimail.log.to") };
            if _cond {
                event.append_unique("email.to.address", json!(event.get("fortinet_fortimail.log.to").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("temp.mailer") {
                    event.rename("temp.mailer", "fortinet_fortimail.log.mailer")?;
                }

            if let Some(v) = event.get("fortinet_fortimail.log.mailer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("email.x_mailer", v)?;
            }

                if event.has_value("temp.session_id") {
                    event.rename("temp.session_id", "fortinet_fortimail.log.session_id")?;
                }

                if event.has_value("temp.endpoint") {
                    event.rename("temp.endpoint", "fortinet_fortimail.log.endpoint")?;
                }

                if event.has_value("temp.polid") {
                    event.rename("temp.polid", "fortinet_fortimail.log.policy_id")?;
                }

                if event.has_value("temp.domain") {
                    event.rename("temp.domain", "fortinet_fortimail.log.domain")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("fortinet_fortimail.log.domain") {
                if let Some(domain_str) = event.get_string("fortinet_fortimail.log.domain") {
                    let domain = domain_str.to_string();
                    event.set("server.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        if let Some(registered) = rd.registered_domain {
                            event.set("server.registered_domain", json!(registered))?;
                        }
                        event.set("server.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("server.subdomain", json!(sub))?;
                        }
                    }
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "registered_domain")?;
                event.set("_ingest.on_failure_processor_tag", "registered_domain_for_domain")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("temp.resolved") {
                    event.rename("temp.resolved", "fortinet_fortimail.log.resolved")?;
                }

            let _cond = { event.get_str("fortinet_fortimail.log.resolved").is_some_and(|s| s.to_lowercase() == "ok") };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.resolved").is_some_and(|s| s.to_lowercase() == "fail") };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
            event.set("event.outcome", json!("unknown"))?;
            }

                if event.has_value("temp.virus") {
                    event.rename("temp.virus", "fortinet_fortimail.log.virus")?;
                }

                if event.has_value("temp.disposition") {
                    event.rename("temp.disposition", "fortinet_fortimail.log.disposition")?;
                }

                if event.has_value("temp.classifier") {
                    event.rename("temp.classifier", "fortinet_fortimail.log.classifier")?;
                }

                if event.has_value("temp.hfrom") {
                    event.rename("temp.hfrom", "fortinet_fortimail.log.hfrom")?;
                }

                if event.has_value("temp.src_type") {
                    event.rename("temp.src_type", "fortinet_fortimail.log.source.type")?;
                }

            let _cond = { event.get_str("temp.message_length") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("temp.message_length") {
                if let Some(val) = event.get("temp.message_length") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "temp.message_length".into(),
                            message,
                        })?;
                    event.set("fortinet_fortimail.log.message_length", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_message_length_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("temp.client_cc") {
                    event.rename("temp.client_cc", "fortinet_fortimail.log.client.cc")?;
                }

                if event.has_value("temp.detail") {
                    event.rename("temp.detail", "fortinet_fortimail.log.detail")?;
                }

                if event.has_value("temp.message_id") {
                    event.rename("temp.message_id", "fortinet_fortimail.log.message_id")?;
                }

                if event.has_value("temp.recv_time") {
                    event.rename("temp.recv_time", "fortinet_fortimail.log.recv_time")?;
                }

                if event.has_value("temp.notif_delay") {
                    event.rename("temp.notif_delay", "fortinet_fortimail.log.notif_delay")?;
                }

            let _cond = { event.get_str("temp.scan_time") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("temp.scan_time") {
                if let Some(val) = event.get("temp.scan_time") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "temp.scan_time".into(),
                            message,
                        })?;
                    event.set("fortinet_fortimail.log.scan_time", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_scan_time_to_double")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("temp.xfer_time") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("temp.xfer_time") {
                if let Some(val) = event.get("temp.xfer_time") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "temp.xfer_time".into(),
                            message,
                        })?;
                    event.set("fortinet_fortimail.log.xfer_time", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_xfer_time_to_double")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("temp.srcfolder") {
                    event.rename("temp.srcfolder", "fortinet_fortimail.log.source.folder")?;
                }

                if event.has_value("temp.read_status") {
                    event.rename("temp.read_status", "fortinet_fortimail.log.read_status")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
