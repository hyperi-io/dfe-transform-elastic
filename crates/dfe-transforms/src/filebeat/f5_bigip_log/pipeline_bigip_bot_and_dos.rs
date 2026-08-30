// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_bigip_bot_and_dos` pipeline.
pub struct PipelineBigipBotAndDos;

impl Transform for PipelineBigipBotAndDos {
    fn name(&self) -> &str {
        "pipeline_bigip_bot_and_dos"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // SKIPPED: condition not transpiled: ctx.event.original.contains('device_product="Application Security Module"')
        #[allow(unreachable_code, unused_variables)]
        if false {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(kv_str) = event.get_string("event.original") {
                for pair in kv_str.split(",") {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=") else {
                        return Err(TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        let value = match (value.chars().next(), value.chars().last()) {
                            (Some('('), Some(')'))
                            | (Some('['), Some(']'))
                            | (Some('<'), Some('>'))
                            | (Some('"'), Some('"'))
                            | (Some('\''), Some('\'')) if value.chars().count() > 1 => {
                                &value[1..value.len() - 1]
                            }
                            _ => value,
                        };
                        if !key.is_empty() {
                            kv_put(event, &format!("kv.{}", key), value)?;
                        }
                    }
                }
            }
            Ok(())
        })();
        }

        // SKIPPED: condition not transpiled: ctx.event.original.contains('device_product=ASM') || ctx.event.original.contains('device_product="ASM"')
        #[allow(unreachable_code, unused_variables)]
        if false {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(kv_str) = event.get_string("event.original") {
                for pair in kv_str.split(";") {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=") else {
                        return Err(TransformError::ParseError {
                            path: "event.original".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        let value = match (value.chars().next(), value.chars().last()) {
                            (Some('('), Some(')'))
                            | (Some('['), Some(']'))
                            | (Some('<'), Some('>'))
                            | (Some('"'), Some('"'))
                            | (Some('\''), Some('\'')) if value.chars().count() > 1 => {
                                &value[1..value.len() - 1]
                            }
                            _ => value,
                        };
                        if !key.is_empty() {
                            kv_put(event, &format!("kv.{}", key), value)?;
                        }
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "kv")?;
            event.set("_ingest.on_failure_processor_tag", "kv_event_original_for_dos")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == 'N/A' || object == 'NA') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"boolean dropEmptyFields(Object object) {\n  if (object == 'N/A' || object == 'NA') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#))?;

            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

        event.set("event.kind", json!("alert"))?;

            if event.has_value("kv.action") {
                event.rename("kv.action", "f5_bigip.log.action")?;
            }

        if let Some(v) = event.get("f5_bigip.log.action").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.action", v)?;
        }

        if event.has_value("event.action") {
            map_strings(event, "event.action", "event.action", str::to_lowercase)?;
        }

        let _cond = { event.get_str("event.action") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("event.action") {
            gsub_field(event, "event.action", "event.action", cached_regex!(" "), "-")?;
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "gsub")?;
            event.set("_ingest.on_failure_processor_tag", "gsub_event_action")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("kv.actual_mitigation_action_reason") {
                event.rename("kv.actual_mitigation_action_reason", "f5_bigip.log.actual_mitigation_action.reason")?;
            }

            if event.has_value("kv.actual_mitigation_action") {
                event.rename("kv.actual_mitigation_action", "f5_bigip.log.actual_mitigation_action.value")?;
            }

            if event.has_value("kv.additional_bot_signatures") {
                event.rename("kv.additional_bot_signatures", "f5_bigip.log.additional_bot_signatures")?;
            }

            if event.has_value("kv.anomalies") {
                event.rename("kv.anomalies", "f5_bigip.log.anomalies")?;
            }

            if event.has_value("kv.anomaly_categories") {
                event.rename("kv.anomaly_categories", "f5_bigip.log.anomaly_categories")?;
            }

            if event.has_value("kv.application_display_name") {
                event.rename("kv.application_display_name", "f5_bigip.log.application.display_name")?;
            }

            if event.has_value("kv.application_version") {
                event.rename("kv.application_version", "f5_bigip.log.application.version")?;
            }

        let _cond = { event.get_str("kv.bigip_mgmt_ip") != Some("null") && event.get_str("kv.bigip_mgmt_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.bigip_mgmt_ip") {
            if let Some(val) = event.get("kv.bigip_mgmt_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.bigip_mgmt_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bigip_management.ip", converted)?;
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

        let _cond = { event.has_value("f5_bigip.log.bigip_management.ip") && event.get_str("f5_bigip.log.bigip_management.ip") != Some("null") };
        if _cond {
            event.append_unique("host.ip", json!(event.get("f5_bigip.log.bigip_management.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("f5_bigip.log.bigip_management.ip") && event.get_str("f5_bigip.log.bigip_management.ip") != Some("null") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.bigip_management.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get_str("kv.bigip_mgmt_ip2") != Some("null") && event.get_str("kv.bigip_mgmt_ip2") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.bigip_mgmt_ip2") {
            if let Some(val) = event.get("kv.bigip_mgmt_ip2") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.bigip_mgmt_ip2".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bigip_management.ip2", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_bigip_mgmt_ip2_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("f5_bigip.log.bigip_management.ip2") && event.get_str("f5_bigip.log.bigip_management.ip2") != Some("null") };
        if _cond {
            event.append_unique("host.ip", json!(event.get("f5_bigip.log.bigip_management.ip2").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("f5_bigip.log.bigip_management.ip2") && event.get_str("f5_bigip.log.bigip_management.ip2") != Some("null") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.bigip_management.ip2").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get_str("kv.bigip_mgmt_ip_2") != Some("null") && event.get_str("kv.bigip_mgmt_ip_2") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.bigip_mgmt_ip_2") {
            if let Some(val) = event.get("kv.bigip_mgmt_ip_2") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.bigip_mgmt_ip_2".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bigip_management.ip_2", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_bigip_mgmt_ip_2_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("f5_bigip.log.bigip_management.ip_2") && event.get_str("f5_bigip.log.bigip_management.ip_2") != Some("null") };
        if _cond {
            event.append_unique("host.ip", json!(event.get("f5_bigip.log.bigip_management.ip_2").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("f5_bigip.log.bigip_management.ip_2") && event.get_str("f5_bigip.log.bigip_management.ip_2") != Some("null") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.bigip_management.ip_2").map_or_else(String::new, template_to_string)))?;
        }

            if event.has_value("kv.bot_name") {
                event.rename("kv.bot_name", "f5_bigip.log.bot_name")?;
            }

            if event.has_value("kv.bot_signature_category") {
                event.rename("kv.bot_signature_category", "f5_bigip.log.bot_signature.category")?;
            }

            if event.has_value("kv.bot_signature") {
                event.rename("kv.bot_signature", "f5_bigip.log.bot_signature.value")?;
            }

            if event.has_value("kv.browser_actual_verification_action_reason") {
                event.rename("kv.browser_actual_verification_action_reason", "f5_bigip.log.browser_actual_verification_action.reason")?;
            }

            if event.has_value("kv.browser_actual_verification_action") {
                event.rename("kv.browser_actual_verification_action", "f5_bigip.log.browser_actual_verification_action.value")?;
            }

            if event.has_value("kv.browser_configured_verification_action") {
                event.rename("kv.browser_configured_verification_action", "f5_bigip.log.browser_configured_verification_action")?;
            }

            if event.has_value("kv.browser_verification_status") {
                event.rename("kv.browser_verification_status", "f5_bigip.log.browser_verification_status")?;
            }

            if event.has_value("kv.captcha_status") {
                event.rename("kv.captcha_status", "f5_bigip.log.captcha_status")?;
            }

            if event.has_value("kv.class") {
                event.rename("kv.class", "f5_bigip.log.class")?;
            }

            if event.has_value("kv.classification_reason") {
                event.rename("kv.classification_reason", "f5_bigip.log.classification_reason")?;
            }

            if event.has_value("kv.client_ip_geo_location") {
                event.rename("kv.client_ip_geo_location", "f5_bigip.log.client.ip_geo_location")?;
            }

        let _cond = { event.get_str("kv.client_ip") != Some("null") && event.get_str("kv.client_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.client_ip") {
            if let Some(val) = event.get("kv.client_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.client_ip".into(),
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

        let _cond = { event.has_value("f5_bigip.log.client.ip") && event.get_str("f5_bigip.log.client.ip") != Some("null") };
        if _cond {
        if let Some(v) = event.get("f5_bigip.log.client.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.ip", v)?;
        }
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

        if let Some(v) = event.get("source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.ip", v)?;
        }

        let _cond = { event.has_value("f5_bigip.log.client.ip") && event.get_str("f5_bigip.log.client.ip") != Some("null") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.client.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get_str("kv.client_port") != Some("null") && event.get_str("kv.client_port") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.client_port") {
            if let Some(val) = event.get("kv.client_port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.client_port".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client.port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_port_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.client.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.port", v)?;
        }

        if let Some(v) = event.get("source.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.port", v)?;
        }

            if event.has_value("kv.client_request_uri") {
                event.rename("kv.client_request_uri", "f5_bigip.log.client.request_uri")?;
            }

        if let Some(v) = event.get("f5_bigip.log.client.request_uri").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("url.path", v)?;
        }

            if event.has_value("kv.client_type") {
                event.rename("kv.client_type", "f5_bigip.log.client.type")?;
            }

        let _cond = { event.has_value("kv.configuration_date_time") && event.get_str("kv.configuration_date_time") != Some("null") && event.get_str("kv.configuration_date_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("kv.configuration_date_time") {
                match parse_date_out(&date_str, &["MMM dd yyyy HH:mm:ss", "ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.configuration_date_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "kv.configuration_date_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_configuration_date_time")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.configuration_date_time").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.created", v)?;
        }

            if event.has_value("kv.configured_mitigation_action_reason") {
                event.rename("kv.configured_mitigation_action_reason", "f5_bigip.log.configured_mitigation_action.reason")?;
            }

            if event.has_value("kv.configured_mitigation_action") {
                event.rename("kv.configured_mitigation_action", "f5_bigip.log.configured_mitigation_action.value")?;
            }

            if event.has_value("kv.context_name") {
                event.rename("kv.context_name", "f5_bigip.log.context.name")?;
            }

            if event.has_value("kv.context_type") {
                event.rename("kv.context_type", "f5_bigip.log.context.type")?;
            }

        let _cond = { event.has_value("kv.date_time") && event.get_str("kv.date_time") != Some("null") && event.get_str("kv.date_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("kv.date_time") {
                match parse_date_out(&date_str, &["MMM dd yyyy HH:mm:ss", "ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.date_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "kv.date_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_date_time")?;
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
            event.set("event.start", v)?;
        }

        let _cond = { event.get_str("kv.dest_ip") != Some("null") && event.get_str("kv.dest_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.dest_ip") {
            if let Some(val) = event.get("kv.dest_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.dest_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.destination.ip", converted)?;
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

        if let Some(v) = event.get("f5_bigip.log.destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.ip", v)?;
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

        if let Some(v) = event.get("destination.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.ip", v)?;
        }

        let _cond = { event.has_value("f5_bigip.log.destination.ip") && event.get_str("f5_bigip.log.destination.ip") != Some("null") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.destination.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get_str("kv.dest_port") != Some("null") && event.get_str("kv.dest_port") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.dest_port") {
            if let Some(val) = event.get("kv.dest_port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.dest_port".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.destination.port", converted)?;
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

        if let Some(v) = event.get("f5_bigip.log.destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.port", v)?;
        }

        if let Some(v) = event.get("destination.port").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.port", v)?;
        }

            if event.has_value("kv.device_blade") {
                event.rename("kv.device_blade", "f5_bigip.log.device.blade")?;
            }

            if event.has_value("kv.device_id") {
                event.rename("kv.device_id", "f5_bigip.log.device.id")?;
            }

        if let Some(v) = event.get("f5_bigip.log.device.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("device.id", v)?;
        }

            if event.has_value("kv.device_product") {
                event.rename("kv.device_product", "f5_bigip.log.device.product")?;
            }

        if let Some(v) = event.get("f5_bigip.log.device.product").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("observer.product", v)?;
        }

            if event.has_value("kv.device_vendor") {
                event.rename("kv.device_vendor", "f5_bigip.log.device.vendor")?;
            }

        if let Some(v) = event.get("f5_bigip.log.device.vendor").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("observer.vendor", v)?;
        }

            if event.has_value("kv.device_version") {
                event.rename("kv.device_version", "f5_bigip.log.device.version")?;
            }

        if let Some(v) = event.get("f5_bigip.log.device.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("observer.version", v)?;
        }

            if event.has_value("kv.device_id_action") {
                event.rename("kv.device_id_action", "f5_bigip.log.device_id.action")?;
            }

            if event.has_value("kv.device_id_status") {
                event.rename("kv.device_id_status", "f5_bigip.log.device_id.status")?;
            }

            if event.has_value("kv.dos_attack_detection_mode") {
                event.rename("kv.dos_attack_detection_mode", "f5_bigip.log.dos.attack.detection_mode")?;
            }

            if event.has_value("kv.dos_attack_event") {
                event.rename("kv.dos_attack_event", "f5_bigip.log.dos.attack.event")?;
            }

            if event.has_value("kv.dos_attack_id") {
                event.rename("kv.dos_attack_id", "f5_bigip.log.dos.attack.id")?;
            }

            if event.has_value("kv.dos_attack_latency") {
                event.rename("kv.dos_attack_latency", "f5_bigip.log.dos.attack.latency")?;
            }

            if event.has_value("kv.dos_attack_name") {
                event.rename("kv.dos_attack_name", "f5_bigip.log.dos.attack.name")?;
            }

            if event.has_value("kv.dos_attack_tps") {
                event.rename("kv.dos_attack_tps", "f5_bigip.log.dos.attack.tps")?;
            }

            if event.has_value("kv.dos_baseline_latency") {
                event.rename("kv.dos_baseline_latency", "f5_bigip.log.dos.baseline.latency")?;
            }

            if event.has_value("kv.dos_baseline_tps") {
                event.rename("kv.dos_baseline_tps", "f5_bigip.log.dos.baseline.tps")?;
            }

            if event.has_value("kv.dos_baseline_traffic_percent") {
                event.rename("kv.dos_baseline_traffic_percent", "f5_bigip.log.dos.baseline.traffic_percent")?;
            }

            if event.has_value("kv.dos_current_traffic_percent") {
                event.rename("kv.dos_current_traffic_percent", "f5_bigip.log.dos.current_traffic_percent")?;
            }

        let _cond = { event.get_str("kv.dos_dropped_requests_count") != Some("null") && event.get_str("kv.dos_dropped_requests_count") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.dos_dropped_requests_count") {
            if let Some(val) = event.get("kv.dos_dropped_requests_count") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.dos_dropped_requests_count".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.dos.dropped_requests_count", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_dos_dropped_requests_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("kv.dos_incoming_requests_count") != Some("null") && event.get_str("kv.dos_incoming_requests_count") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.dos_incoming_requests_count") {
            if let Some(val) = event.get("kv.dos_incoming_requests_count") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.dos_incoming_requests_count".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.dos.incoming_requests_count", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_dos_incoming_requests_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("kv.dos_detection_condition") {
                event.rename("kv.dos_detection_condition", "f5_bigip.log.dos_detection.condition")?;
            }

            if event.has_value("kv.dos_detection_threshold") {
                event.rename("kv.dos_detection_threshold", "f5_bigip.log.dos_detection.threshold")?;
            }

            if event.has_value("kv.dos_mitigate_to_threshold") {
                event.rename("kv.dos_mitigate_to_threshold", "f5_bigip.log.dos_mitigate_to_threshold")?;
            }

            if event.has_value("kv.dos_mitigation_action") {
                event.rename("kv.dos_mitigation_action", "f5_bigip.log.dos_mitigation.action")?;
            }

            if event.has_value("kv.dos_mitigation_reason") {
                event.rename("kv.dos_mitigation_reason", "f5_bigip.log.dos_mitigation.reason")?;
            }

            if event.has_value("kv.enforced_by") {
                event.rename("kv.enforced_by", "f5_bigip.log.enforced_by")?;
            }

            if event.has_value("kv.errdefs_msg_name") {
                event.rename("kv.errdefs_msg_name", "f5_bigip.log.errdefs.msg_name")?;
            }

            if event.has_value("kv.errdefs_msgno") {
                event.rename("kv.errdefs_msgno", "f5_bigip.log.errdefs.msgno")?;
            }

        if let Some(v) = event.get("f5_bigip.log.errdefs.msgno").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("error.id", v)?;
        }

            if event.has_value("kv.event_id") {
                event.rename("kv.event_id", "f5_bigip.log.event.id")?;
            }

        if let Some(v) = event.get("f5_bigip.log.event.id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.id", v)?;
        }

            if event.has_value("kv.hostname") {
                event.rename("kv.hostname", "f5_bigip.log.hostname")?;
            }

        if let Some(v) = event.get("f5_bigip.log.hostname").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.hostname", v)?;
        }

        let _cond = { event.has_value("f5_bigip.log.hostname") && event.get_str("f5_bigip.log.hostname") != Some("null") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.hostname").map_or_else(String::new, template_to_string)))?;
        }

            if event.has_value("kv.http_method") {
                event.rename("kv.http_method", "f5_bigip.log.http.method")?;
            }

        if let Some(v) = event.get("f5_bigip.log.http.method").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.method", v)?;
        }

            if event.has_value("kv.http_protocol_indication") {
                event.rename("kv.http_protocol_indication", "f5_bigip.log.http.protocol_indication")?;
            }

        if let Some(v) = event.get("f5_bigip.log.http.protocol_indication").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("network.protocol", v)?;
        }

        if event.has_value("network.protocol") {
            map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
        }

            // Painless script
            // Source: String message = ctx.event.original;\nint startIndex = message.indexOf('http_request=\"') + 'http_request=\"'.length();\nint endIndex = message.indexOf('\"', startIndex);\nif (startIndex >= 0 && endIndex >= 0) {\n  ctx.kv.http_request = message.substring(startIndex, endIndex);\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"String message = ctx.event.original;\nint startIndex = message.indexOf('http_request=\"') + 'http_request=\"'.length();\nint endIndex = message.indexOf('\"', startIndex);\nif (startIndex >= 0 && endIndex >= 0) {\n  ctx.kv.http_request = message.substring(startIndex, endIndex);\n}\n"#))?;

            if event.has_value("kv.http_request") {
                event.rename("kv.http_request", "f5_bigip.log.http.request")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            if let Some(input) = event.get_string("f5_bigip.log.http.request") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(pos) = remaining.find(" ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.http.method", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find(" ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.http.path", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\nHost: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.http.version", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\nHost: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\nConnection: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.http.host", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\nConnection: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\nPragma: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.http.connection", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\nPragma: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\nCache-Control: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.http.pragma", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\nCache-Control: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\nUser-Agent: ") else { break 'dissect false };
                    captured.push(("f5_bigip.log.http.cache_control", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\nUser-Agent: ") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\n") else { break 'dissect false };
                    captured.push(("f5_bigip.log.http.user_agent", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\n") else { break 'dissect false };
                    remaining = rest;
                    let Some(pos) = remaining.find("\\r\\n") else { break 'dissect false };
                    captured.push(("f5_bigip.log.http.other_headers", &remaining[..pos]));
                    remaining = &remaining[pos..];
                    let Some(rest) = remaining.strip_prefix("\\r\\n") else { break 'dissect false };
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

        if event.has_value("f5_bigip.log.http.user_agent") {
            gsub_field(event, "f5_bigip.log.http.user_agent", "f5_bigip.log.http.user_agent", cached_regex!("(\\([^)]*)\\+(https?://)"), "$1%2b$2")?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.http.user_agent") {
            if let Some(s) = event.get_string("f5_bigip.log.http.user_agent") {
                match url_decode(&s) {
                    Some(decoded) => event.set("f5_bigip.log.http.user_agent", json!(decoded))?,
                    None => return Err(TransformError::ParseError {
                        path: "f5_bigip.log.http.user_agent".into(),
                        message: format!("cannot url-decode '{s}'"),
                    }),
                }
            }
        }
            Ok(())
        })();

        if event.has_value("f5_bigip.log.http.user_agent") {
            if let Some(ua_str) = event.get_string("f5_bigip.log.http.user_agent") {
                let ua_str = ua_str.to_string();
                // User agent parsing
                if let Ok(ua) = parse_user_agent(&ua_str) {
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
            if let Some(input) = event.get_string("f5_bigip.log.http.version") {
                let mut remaining: &str = &input;
                let mut captured: Vec<(&str, &str)> = Vec::new();
                let matched = 'dissect: {
                    let Some(rest) = remaining.strip_prefix("HTTP/") else { break 'dissect false };
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

        let _cond = { event.get_str("f5_bigip.log.http.host") != Some("null") && event.get_str("f5_bigip.log.http.host") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.http.host") {
            if let Some(val) = event.get("f5_bigip.log.http.host") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "f5_bigip.log.http.host".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.http.request_host", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_f5_bigip_log_http_host_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("f5_bigip.log.http.request_host") && event.get_str("f5_bigip.log.http.request_host") != Some("null") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.http.request_host").map_or_else(String::new, template_to_string)))?;
        }

            if event.has_value("kv.human_behaviour") {
                event.rename("kv.human_behaviour", "f5_bigip.log.human_behaviour")?;
            }

            if event.has_value("kv.imei") {
                event.rename("kv.imei", "f5_bigip.log.imei")?;
            }

            if event.has_value("kv.jailbroken_or_rooted_device") {
                event.rename("kv.jailbroken_or_rooted_device", "f5_bigip.log.jailbroken_or_rooted_device")?;
            }

            if event.has_value("kv.micro_service_hostname") {
                event.rename("kv.micro_service_hostname", "f5_bigip.log.micro_service.hostname")?;
            }

            if event.has_value("kv.micro_service_matched_wildcard_url") {
                event.rename("kv.micro_service_matched_wildcard_url", "f5_bigip.log.micro_service.matched_wildcard_url")?;
            }

            if event.has_value("kv.micro_service_name") {
                event.rename("kv.micro_service_name", "f5_bigip.log.micro_service.name")?;
            }

        if let Some(v) = event.get("f5_bigip.log.micro_service.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.name", v)?;
        }

            if event.has_value("kv.micro_service_type") {
                event.rename("kv.micro_service_type", "f5_bigip.log.micro_service.type")?;
            }

        if let Some(v) = event.get("f5_bigip.log.micro_service.type").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("service.type", v)?;
        }

            if event.has_value("kv.mobile_in_emulation_mode") {
                event.rename("kv.mobile_in_emulation_mode", "f5_bigip.log.mobile_in_emulation_mode")?;
            }

            if event.has_value("kv.mobile_is_app") {
                event.rename("kv.mobile_is_app", "f5_bigip.log.mobile_is_app")?;
            }

            if event.has_value("kv.new_request_status") {
                event.rename("kv.new_request_status", "f5_bigip.log.new_request_status")?;
            }

            if event.has_value("kv.os_name") {
                event.rename("kv.os_name", "f5_bigip.log.osname")?;
            }

        if let Some(v) = event.get("f5_bigip.log.osname").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.os.name", v)?;
        }

            if event.has_value("kv.partition_name") {
                event.rename("kv.partition_name", "f5_bigip.log.partition_name")?;
            }

            if event.has_value("kv.previous_action") {
                event.rename("kv.previous_action", "f5_bigip.log.previous.action")?;
            }

            if event.has_value("kv.previous_initiated_action_status") {
                event.rename("kv.previous_initiated_action_status", "f5_bigip.log.previous.initiated_action.status")?;
            }

            if event.has_value("kv.previous_initiated_action") {
                event.rename("kv.previous_initiated_action", "f5_bigip.log.previous.initiated_action.value")?;
            }

        let _cond = { event.has_value("kv.previous_request_date_time") && event.get_str("kv.previous_request_date_time") != Some("null") && event.get_str("kv.previous_request_date_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("kv.previous_request_date_time") {
                match parse_date_out(&date_str, &["MMM dd yyyy HH:mm:ss", "ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.previous.request_date_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "kv.previous_request_date_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_previous_request_date_time")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("kv.previous_support_id") {
                event.rename("kv.previous_support_id", "f5_bigip.log.previous.support_id")?;
            }

            if event.has_value("kv.profile_name") {
                event.rename("kv.profile_name", "f5_bigip.log.profile_name")?;
            }

            if event.has_value("kv.reason") {
                event.rename("kv.reason", "f5_bigip.log.reason")?;
            }

        if let Some(v) = event.get("f5_bigip.log.reason").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.reason", v)?;
        }

            if event.has_value("kv.reported_entity_type") {
                event.rename("kv.reported_entity_type", "f5_bigip.log.reported_entity_type")?;
            }

        let _cond = { event.has_value("kv.request_date_time") && event.get_str("kv.request_date_time") != Some("null") && event.get_str("kv.request_date_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("kv.request_date_time") {
                match parse_date_out(&date_str, &["MMM dd yyyy HH:mm:ss", "ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.request.date_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "kv.request_date_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_request_date_time")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.request.date_time").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.start", v)?;
        }

            if event.has_value("kv.request_status") {
                event.rename("kv.request_status", "f5_bigip.log.request.status")?;
            }

            if event.has_value("kv.route_domain") {
                event.rename("kv.route_domain", "f5_bigip.log.route_domain")?;
            }

            if event.has_value("kv.session_id") {
                event.rename("kv.session_id", "f5_bigip.log.session.id")?;
            }

        let _cond = { event.get_str("kv.severity") != Some("null") && event.get_str("kv.severity") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.severity") {
            if let Some(val) = event.get("kv.severity") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.severity".into(),
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

        if let Some(v) = event.get("f5_bigip.log.severity.code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("event.severity", v)?;
        }

        let _cond = { event.get_str("kv.source_ip") != Some("null") && event.get_str("kv.source_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("kv.source_ip") {
            if let Some(val) = event.get("kv.source_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "kv.source_ip".into(),
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

        let _cond = { event.has_value("f5_bigip.log.source.ip") && event.get_str("f5_bigip.log.source.ip") != Some("null") };
        if _cond {
            event.append_unique("source.ip", json!(event.get("f5_bigip.log.source.ip").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("f5_bigip.log.source.ip") && event.get_str("f5_bigip.log.source.ip") != Some("null") };
        if _cond {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.source.ip").map_or_else(String::new, template_to_string)))?;
        }

            if event.has_value("kv.support_id") {
                event.rename("kv.support_id", "f5_bigip.log.support.id")?;
            }

        let _cond = { event.has_value("kv.timestamp") && event.get_str("kv.timestamp") != Some("null") && event.get_str("kv.timestamp") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("kv.timestamp") {
                match parse_date_out(&date_str, &["MMM dd yyyy HH:mm:ss", "ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "kv.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.timestamp").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("@timestamp", v)?;
        }

            if event.has_value("kv.virtual_server_name") {
                event.rename("kv.virtual_server_name", "f5_bigip.log.virtual_server_name")?;
            }

        let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
        if _cond {
            event.remove("f5_bigip.log.action");
            event.remove("f5_bigip.log.bigip_management.ip");
            event.remove("f5_bigip.log.bigip_management.ip_2");
            event.remove("f5_bigip.log.bigip_management.ip2");
            event.remove("f5_bigip.log.client.ip");
            event.remove("f5_bigip.log.client.port");
            event.remove("f5_bigip.log.client.request_uri");
            event.remove("f5_bigip.log.configuration_date_time");
            event.remove("f5_bigip.log.date_time");
            event.remove("f5_bigip.log.destination.ip");
            event.remove("f5_bigip.log.destination.port");
            event.remove("f5_bigip.log.device.id");
            event.remove("f5_bigip.log.device.product");
            event.remove("f5_bigip.log.device.vendor");
            event.remove("f5_bigip.log.device.version");
            event.remove("f5_bigip.log.errdefs.msgno");
            event.remove("f5_bigip.log.event.id");
            event.remove("f5_bigip.log.hostname");
            event.remove("f5_bigip.log.http.method");
            event.remove("f5_bigip.log.http.protocol_indication");
            event.remove("f5_bigip.log.micro_service.name");
            event.remove("f5_bigip.log.micro_service.type");
            event.remove("f5_bigip.log.osname");
            event.remove("f5_bigip.log.reason");
            event.remove("f5_bigip.log.request.date_time");
            event.remove("f5_bigip.log.severity");
            event.remove("f5_bigip.log.source.ip");
            event.remove("f5_bigip.log.timestamp");
        }

            event.remove("kv.bigip_mgmt_ip");
            event.remove("kv.bigip_mgmt_ip_2");
            event.remove("kv.bigip_mgmt_ip2");
            event.remove("kv.client_ip");
            event.remove("kv.client_port");
            event.remove("kv.configuration_date_time");
            event.remove("kv.date_time");
            event.remove("kv.date_time");
            event.remove("kv.dest_ip");
            event.remove("kv.dest_port");
            event.remove("kv.dos_dropped_requests_count");
            event.remove("kv.dos_incoming_requests_count");
            event.remove("kv.previous_request_date_time");
            event.remove("kv.request_date_time");
            event.remove("kv.severity");
            event.remove("kv.source_ip");
            event.remove("kv.timestamp");

            if event.has_value("kv") {
                event.rename("kv", "f5_bigip.log.extra_fields")?;
            }

        Ok(TransformResult::Continue)
    }
}
