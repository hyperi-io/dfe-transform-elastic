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
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.occurTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.uuid") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("alert"))?;

            let _cond = {
                event.has_value("json.category")
                    && [
                        "DETECTION_CATEGORY_CORRELATION_RULE",
                        "DETECTION_CATEGORY_FIREWALL_RULE",
                        "DETECTION_CATEGORY_HIPS",
                        "DETECTION_CATEGORY_NETWORK_INTRUSION",
                        "DETECTION_CATEGORY_HIPS_RULE",
                    ]
                    .contains(&event.get_str("json.category").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            let _cond = {
                event.has_value("json.category")
                    && ["DETECTION_CATEGORY_ANTIVIRUS"]
                        .contains(&event.get_str("json.category").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("malware"))?;
            }

            let _cond = {
                event.has_value("json.category")
                    && ["DETECTION_CATEGORY_WEB_ACCESS"]
                        .contains(&event.get_str("json.category").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("web"))?;
            }

            let _cond = {
                event.has_value("json.category")
                    && ["DETECTION_CATEGORY_VULNERABILITY"]
                        .contains(&event.get_str("json.category").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("vulnerability"))?;
            }

            let _cond = {
                event.has_value("json.category")
                    && ["DETECTION_CATEGORY_APPLICATION_PATCH"]
                        .contains(&event.get_str("json.category").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("package"))?;
            }

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("json.uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("json.context.circumstances")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            event.set("observer.vendor", json!("ESET"))?;

            event.set("observer.product", json!("ESET PROTECT"))?;

            event.set("observer.type", json!("ids"))?;

            if event.has_value("json.category") {
                event.rename("json.category", "eset_protect.detection.category")?;
            }

            if let Some(v) = event
                .get("eset_protect.detection.category")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.category", v)?;
            }

            if event.has_value("json.context.circumstances") {
                event.rename(
                    "json.context.circumstances",
                    "eset_protect.detection.context.circumstances",
                )?;
            }

            if let Some(v) = event
                .get("eset_protect.detection.context.circumstances")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("json.context.deviceUuid") {
                event.rename(
                    "json.context.deviceUuid",
                    "eset_protect.detection.context.device_uuid",
                )?;
            }

            if let Some(v) = event
                .get("eset_protect.detection.context.device_uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("json.deviceDisplayName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("eset_protect.detection.context.device_uuid") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("eset_protect.detection.context.device_uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.deviceInfo.primaryIp") {
                    if let Some(val) = event.get("json.deviceInfo.primaryIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.deviceInfo.primaryIp".into(),
                                message,
                            }
                        })?;
                        event.set("json.deviceInfo.primaryIp", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_device_primary_ip",
                )?;
                event.remove("json.deviceInfo.primaryIp");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.deviceInfo.primaryIp") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("json.deviceInfo.primaryIp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.deviceInfo.primaryIp") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.deviceInfo.primaryIp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.deviceInfo.publicIp") {
                    if let Some(val) = event.get("json.deviceInfo.publicIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.deviceInfo.publicIp".into(),
                                message,
                            }
                        })?;
                        event.set("json.deviceInfo.publicIp", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_device_public_ip",
                )?;
                event.remove("json.deviceInfo.publicIp");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.deviceInfo.publicIp") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("json.deviceInfo.publicIp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.deviceInfo.publicIp") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("json.deviceInfo.publicIp")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("json.deviceInfo.osFull")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            if let Some(v) = event
                .get("json.deviceInfo.osVersion")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            let _cond = {
                event.has_value("host.os.full")
                    && event
                        .get_str("host.os.full")
                        .is_some_and(|s| s.to_lowercase().contains("windows"))
            };
            if _cond {
                event.set("host.os.type", json!("windows"))?;
            }

            let _cond = {
                event.has_value("host.os.full")
                    && event
                        .get_str("host.os.full")
                        .is_some_and(|s| s.to_lowercase().contains("linux"))
            };
            if _cond {
                event.set("host.os.type", json!("linux"))?;
            }

            let _cond = {
                event.has_value("host.os.full")
                    && (event
                        .get_str("host.os.full")
                        .is_some_and(|s| s.to_lowercase().contains("mac os"))
                        || event
                            .get_str("host.os.full")
                            .is_some_and(|s| s.to_lowercase().contains("macos")))
            };
            if _cond {
                event.set("host.os.type", json!("macos"))?;
            }

            if let Some(v) = event
                .get("host.os.full")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            let _cond = { event.get_str("host.os.type") == Some("windows") };
            if _cond {
                event.set("host.os.platform", json!("windows"))?;
            }

            let _cond = { event.get_str("host.os.type") == Some("linux") };
            if _cond {
                event.set("host.os.platform", json!("linux"))?;
            }

            let _cond = { event.get_str("host.os.type") == Some("macos") };
            if _cond {
                event.set("host.os.platform", json!("darwin"))?;
            }

            let _cond = { event.get_str("host.os.type") == Some("windows") };
            if _cond {
                event.set("host.os.family", json!("windows"))?;
            }

            let _cond = { event.get_str("host.os.type") == Some("linux") };
            if _cond {
                event.set("host.os.family", json!("linux"))?;
            }

            let _cond = { event.get_str("host.os.type") == Some("macos") };
            if _cond {
                event.set("host.os.family", json!("darwin"))?;
            }

            if event.has_value("json.context.process.path") {
                event.rename(
                    "json.context.process.path",
                    "eset_protect.detection.context.process.path",
                )?;
            }

            if let Some(v) = event
                .get("eset_protect.detection.context.process.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            let _cond =
                { event.get_str("eset_protect.detection.context.process.path") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("eset_protect.detection.context.process.path") {
                        if let Some(input) =
                            event.get_string("eset_protect.detection.context.process.path")
                        {
                            // Grok pattern: ^%{GREEDYDATA:json._temp}\\\\%{DATA:process.name}$
                            // Grok pattern: ^%{GREEDYDATA:json._temp}/%{DATA:process.name}$
                            // Grok pattern: ^%{DATA:process.name}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{GREEDYDATA:json._temp}\\\\%{DATA:process.name}$"
                                    ),
                                    cached_grok!("^%{GREEDYDATA:json._temp}/%{DATA:process.name}$"),
                                    cached_grok!("^%{DATA:process.name}$"),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_context_process_path",
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

            if event.has_value("json.context.userName") {
                event.rename(
                    "json.context.userName",
                    "eset_protect.detection.context.user_name",
                )?;
            }

            let _cond = { event.get_str("eset_protect.detection.context.user_name") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("eset_protect.detection.context.user_name") {
                        if let Some(input) =
                            event.get_string("eset_protect.detection.context.user_name")
                        {
                            // Grok pattern: ^%{HOSTNAME:user.domain}\\\\%{USERNAME:user.name}$
                            // Grok pattern: ^%{HOSTNAME:user.domain}\\\\\\\\%{USERNAME:user.name}$
                            // Grok pattern: ^%{USERNAME:user.name}@%{HOSTNAME:user.domain}$
                            // Grok pattern: ^%{GREEDYDATA:user.name}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{HOSTNAME:user.domain}\\\\%{USERNAME:user.name}$"
                                    ),
                                    cached_grok!(
                                        "^%{HOSTNAME:user.domain}\\\\\\\\%{USERNAME:user.name}$"
                                    ),
                                    cached_grok!("^%{USERNAME:user.name}@%{HOSTNAME:user.domain}$"),
                                    cached_grok!("^%{GREEDYDATA:user.name}$"),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_user_name")?;
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

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.displayName") {
                event.rename("json.displayName", "eset_protect.detection.display_name")?;
            }

            if event.has_value("json.networkCommunication.direction") {
                event.rename(
                    "json.networkCommunication.direction",
                    "eset_protect.detection.network_communication.direction",
                )?;
            }

            let _cond = {
                event.get_str("eset_protect.detection.network_communication.direction")
                    == Some("NETWORK_COMMUNICATION_DIRECTION_INBOUND")
            };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond = {
                event.get_str("eset_protect.detection.network_communication.direction")
                    == Some("NETWORK_COMMUNICATION_DIRECTION_OUTBOUND")
            };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            let _cond = {
                event.has_value("json.networkCommunication.localIpAddress")
                    && event.get_str("json.networkCommunication.localIpAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.networkCommunication.localIpAddress") {
                        if let Some(val) = event.get("json.networkCommunication.localIpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.networkCommunication.localIpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "eset_protect.detection.network_communication.local.ip_address",
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
                        "convert_networkCommunication_localIpAddress_to_ip",
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
                .get("eset_protect.detection.network_communication.local.ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
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

            let _cond = {
                event.has_value("eset_protect.detection.network_communication.local.ip_address")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("eset_protect.detection.network_communication.local.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.networkCommunication.localPort") {
                    if let Some(val) = event.get("json.networkCommunication.localPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.networkCommunication.localPort".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "eset_protect.detection.network_communication.local.port",
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
                    "convert_networkCommunication_localPort_to_long",
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

            if let Some(v) = event
                .get("eset_protect.detection.network_communication.local.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.networkCommunication.protocolName") {
                event.rename(
                    "json.networkCommunication.protocolName",
                    "eset_protect.detection.network_communication.protocol_name",
                )?;
            }

            let _cond = {
                event.get_str("eset_protect.detection.network_communication.protocol_name")
                    != Some("0")
            };
            if _cond {
                if let Some(v) = event
                    .get("eset_protect.detection.network_communication.protocol_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.transport", v)?;
                }
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = {
                event.has_value("json.networkCommunication.remoteIpAddress")
                    && event.get_str("json.networkCommunication.remoteIpAddress") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.networkCommunication.remoteIpAddress") {
                        if let Some(val) = event.get("json.networkCommunication.remoteIpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.networkCommunication.remoteIpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "eset_protect.detection.network_communication.remote.ip_address",
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
                        "convert_networkCommunication_remoteIpAddress_to_ip",
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
                .get("eset_protect.detection.network_communication.remote.ip_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
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
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = {
                event.has_value("eset_protect.detection.network_communication.remote.ip_address")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("eset_protect.detection.network_communication.remote.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.networkCommunication.remotePort") {
                    if let Some(val) = event.get("json.networkCommunication.remotePort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.networkCommunication.remotePort".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "eset_protect.detection.network_communication.remote.port",
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
                    "convert_networkCommunication_remotePort_to_long",
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

            if let Some(v) = event
                .get("eset_protect.detection.network_communication.remote.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.objectTypeName") {
                event.rename(
                    "json.objectTypeName",
                    "eset_protect.detection.object_type_name",
                )?;
            }

            if event.has_value("json.objectHashSha1") {
                event.rename(
                    "json.objectHashSha1",
                    "eset_protect.detection.object_hash_sha1",
                )?;
            }

            let _cond =
                { event.get_str("eset_protect.detection.object_type_name") == Some("File") };
            if _cond {
                if let Some(v) = event
                    .get("eset_protect.detection.object_hash_sha1")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.sha1", v)?;
                }
            }

            if event.has_value("file.hash.sha1") {
                map_strings(event, "file.hash.sha1", "file.hash.sha1", str::to_lowercase)?;
            }

            let _cond = { event.has_value("eset_protect.detection.object_hash_sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("eset_protect.detection.object_hash_sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("related.hash") {
                map_strings(event, "related.hash", "related.hash", str::to_lowercase)?;
            }

            if event.has_value("json.objectName") {
                event.rename("json.objectName", "eset_protect.detection.object_name")?;
            }

            let _cond =
                { event.get_str("eset_protect.detection.object_type_name") == Some("File") };
            if _cond {
                if let Some(v) = event
                    .get("eset_protect.detection.object_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.name", v)?;
                }
            }

            if event.has_value("json.objectUrl") {
                event.rename("json.objectUrl", "eset_protect.detection.object_url")?;
            }

            let _cond = {
                event.has_value("json.occurTime") && event.get_str("json.occurTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.occurTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("eset_protect.detection.occur_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.occurTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_occurTime")?;
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
                .get("eset_protect.detection.occur_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { event.get("json.responses").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.responses", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.deviceRestartRequired") {
                            if let Some(val) = event.get("_ingest._value.deviceRestartRequired") {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.deviceRestartRequired".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.device_restart_required", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_responses_deviceRestartRequired_to_boolean",
                        )?;
                        event.remove("_ingest._value.deviceRestartRequired");
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

            let _cond = { event.get("json.responses").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.responses", |event| {
                    if event.has_value("_ingest._value.displayName") {
                        event
                            .rename("_ingest._value.displayName", "_ingest._value.display_name")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.responses").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.responses", |event| {
                    if event.has_value("_ingest._value.protectionName") {
                        event.rename(
                            "_ingest._value.protectionName",
                            "_ingest._value.protection_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.responses").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.responses", |event| {
                    event.remove("_ingest._value.deviceRestartRequired");
                    Ok(())
                })?;
            }

            if event.has_value("json.responses") {
                event.rename("json.responses", "eset_protect.detection.responses")?;
            }

            if event.has_value("json.severityLevel") {
                event.rename(
                    "json.severityLevel",
                    "eset_protect.detection.severity_level",
                )?;
            }

            if event.has_value("json.typeName") {
                event.rename("json.typeName", "eset_protect.detection.type_name")?;
            }

            let _cond = { event.has_value("eset_protect.detection.type_name") };
            if _cond {
                event.append_unique(
                    "threat.technique.name",
                    json!(
                        event
                            .get("eset_protect.detection.type_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.uuid") {
                event.rename("json.uuid", "eset_protect.detection.uuid")?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("eset_protect.detection.category");
                event.remove("eset_protect.detection.context.device_uuid");
                event.remove("eset_protect.detection.context.process.path");
                event.remove("eset_protect.detection.network_communication.local.ip_address");
                event.remove("eset_protect.detection.network_communication.local.port");
                event.remove("eset_protect.detection.network_communication.protocol_name");
                event.remove("eset_protect.detection.network_communication.remote.ip_address");
                event.remove("eset_protect.detection.network_communication.remote.port");
                event.remove("eset_protect.detection.occur_time");
                event.remove("eset_protect.detection.type_name");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
