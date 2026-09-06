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
            event.set("ecs.version", json!("9.3.0"))?;

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
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

            parse_json_field(event, "event.original", "json")?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("json.event_id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("json.time") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "fingerprint")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "fingerprint_event_id_time",
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

            event.set("event.kind", json!("event"))?;

            event.append_unique("event.category", json!("network"))?;

            if event.has_value("json.account_id") {
                event.rename("json.account_id", "cato_networks.event.account_id")?;
            }

            if event.has_value("json.account_name") {
                event.rename("json.account_name", "cato_networks.event.account_name")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.account_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            if let Some(v) = event
                .get("cato_networks.event.account_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if event.has_value("json.action") {
                event.rename("json.action", "cato_networks.event.action")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") == Some("allow") };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            let _cond =
                { ["block", "drop"].contains(&event.get_str("event.action").unwrap_or("")) };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = {
                !(["allow", "block", "drop"].contains(&event.get_str("event.action").unwrap_or("")))
            };
            if _cond {
                event.append_unique("event.type", json!("info"))?;
            }

            let _cond = { event.get_str("event.action") == Some("succeeded") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("event.action") == Some("failed") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has_value("json.ad_name") {
                event.rename("json.ad_name", "cato_networks.event.ad_name")?;
            }

            let _cond = { event.has_value("cato_networks.event.ad_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cato_networks.event.ad_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.api_name") {
                event.rename("json.api_name", "cato_networks.event.api_name")?;
            }

            if event.has_value("json.api_type") {
                event.rename("json.api_type", "cato_networks.event.api_type")?;
            }

            if event.has_value("json.app_stack") {
                event.rename("json.app_stack", "cato_networks.event.app_stack")?;
            }

            if event.has_value("json.application_id") {
                event.rename("json.application_id", "cato_networks.event.application_id")?;
            }

            if event.has_value("json.application_name") {
                event.rename(
                    "json.application_name",
                    "cato_networks.event.application_name",
                )?;
            }

            if let Some(v) = event
                .get("cato_networks.event.application_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.application_risk") {
                    if let Some(val) = event.get("json.application_risk") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.application_risk".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.application_risk", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_application_risk_to_long",
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

            if event.has_value("json.application_type") {
                event.rename(
                    "json.application_type",
                    "cato_networks.event.application_type",
                )?;
            }

            if event.has_value("json.authentication_type") {
                event.rename(
                    "json.authentication_type",
                    "cato_networks.event.authentication_type",
                )?;
            }

            if event.has_value("json.categories") {
                event.rename("json.categories", "cato_networks.event.categories")?;
            }

            if event.has_value("json.cato_app") {
                event.rename("json.cato_app", "cato_networks.event.cato_app")?;
            }

            if event.has_value("json.client_class") {
                event.rename("json.client_class", "cato_networks.event.client_class")?;
            }

            let _cond = { event.get_str("json.client_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.client_ip") {
                        if let Some(val) = event.get("json.client_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.client_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("cato_networks.event.client_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_client_ip_to_ip",
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
                .get("cato_networks.event.client_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.ip", v)?;
            }

            let _cond = { event.has_value("cato_networks.event.client_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cato_networks.event.client_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.client_version") {
                event.rename("json.client_version", "cato_networks.event.client_version")?;
            }

            if event.has_value("json.configured_host_name") {
                event.rename(
                    "json.configured_host_name",
                    "cato_networks.event.configured_host_name",
                )?;
            }

            if let Some(v) = event
                .get("cato_networks.event.configured_host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("cato_networks.event.configured_host_name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("cato_networks.event.configured_host_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("host.name") {
                map_strings(event, "host.name", "host.name", str::to_lowercase)?;
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

            if event.has_value("json.congestion_algorithm") {
                event.rename(
                    "json.congestion_algorithm",
                    "cato_networks.event.congestion_algorithm",
                )?;
            }

            if event.has_value("json.connection_origin") {
                event.rename(
                    "json.connection_origin",
                    "cato_networks.event.connection_origin",
                )?;
            }

            if event.has_value("json.custom_category_id") {
                event.rename(
                    "json.custom_category_id",
                    "cato_networks.event.custom_category_id",
                )?;
            }

            if event.has_value("json.custom_category_name") {
                event.rename(
                    "json.custom_category_name",
                    "cato_networks.event.custom_category_name",
                )?;
            }

            if event.has_value("json.dest_country") {
                event.rename("json.dest_country", "cato_networks.event.dest_country")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.dest_country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.geo.country_name", v)?;
            }

            if event.has_value("json.dest_country_code") {
                event.rename(
                    "json.dest_country_code",
                    "cato_networks.event.dest_country_code",
                )?;
            }

            if let Some(v) = event
                .get("cato_networks.event.dest_country_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.geo.country_iso_code", v)?;
            }

            let _cond = { event.get_str("json.dest_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.dest_ip") {
                        if let Some(val) = event.get("json.dest_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dest_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("cato_networks.event.dest_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dest_ip_to_ip")?;
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
                .get("cato_networks.event.dest_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event
                .get("cato_networks.event.dest_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.address", v)?;
            }

            let _cond = { event.has_value("cato_networks.event.dest_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cato_networks.event.dest_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.dest_port") {
                    if let Some(val) = event.get("json.dest_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.dest_port".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.dest_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_dest_port_to_long",
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
                .get("cato_networks.event.dest_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.device_name") {
                event.rename("json.device_name", "cato_networks.event.device_name")?;
            }

            if event.has_value("json.device_categories") {
                event.rename(
                    "json.device_categories",
                    "cato_networks.event.device_categories",
                )?;
            }

            if event.has_value("json.device_id") {
                event.rename("json.device_id", "cato_networks.event.device_id")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.device_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if event.has_value("json.device_os_type") {
                event.rename("json.device_os_type", "cato_networks.event.device_os_type")?;
            }

            if event.has_value("json.device_posture_profile") {
                event.rename(
                    "json.device_posture_profile",
                    "cato_networks.event.device_posture_profile",
                )?;
            }

            if event.has_value("json.device_type") {
                event.rename("json.device_type", "cato_networks.event.device_type")?;
            }

            if event.has_value("json.dns_protection_category") {
                event.rename(
                    "json.dns_protection_category",
                    "cato_networks.event.dns_protection_category",
                )?;
            }

            if event.has_value("json.dns_query") {
                event.rename("json.dns_query", "cato_networks.event.dns_query")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.dns_query")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.name", v)?;
            }

            let _cond = { event.has_value("dns.question.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("dns.question.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.dns_record_type") {
                event.rename(
                    "json.dns_record_type",
                    "cato_networks.event.dns_record_type",
                )?;
            }

            if let Some(v) = event
                .get("cato_networks.event.dns_record_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.type", v)?;
            }

            if event.has_value("json.domain_name") {
                event.rename("json.domain_name", "cato_networks.event.domain_name")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.domain_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.domain", v)?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.egress_pop_name") {
                event.rename(
                    "json.egress_pop_name",
                    "cato_networks.event.egress_pop_name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.event_count") {
                    if let Some(val) = event.get("json.event_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.event_count".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.event_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_count_to_long",
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

            if event.has_value("json.event_id") {
                event.rename("json.event_id", "cato_networks.event.event_id")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.event_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.event_message") {
                event.rename("json.event_message", "cato_networks.event.event_message")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.event_message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("json.event_sub_type") {
                event.rename("json.event_sub_type", "cato_networks.event.event_sub_type")?;
            }

            if event.has_value("json.event_type") {
                event.rename("json.event_type", "cato_networks.event.event_type")?;
            }

            if event.has_value("json.full_path_url") {
                event.rename("json.full_path_url", "cato_networks.event.full_path_url")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.full_path_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            if event.has_value("json.flow_id") {
                event.rename("json.flow_id", "cato_networks.event.flow_id")?;
            }

            let _cond = { event.get_str("json.host_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.host_ip") {
                        if let Some(val) = event.get("json.host_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.host_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("cato_networks.event.host_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_host_ip_to_ip")?;
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

            let _cond = { event.has_value("cato_networks.event.host_ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("cato_networks.event.host_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cato_networks.event.host_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cato_networks.event.host_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.host_mac") {
                event.rename("json.host_mac", "cato_networks.event.host_mac")?;
            }

            let _cond = { event.has_value("cato_networks.event.host_mac") };
            if _cond {
                event.append_unique(
                    "host.mac",
                    json!(
                        event
                            .get("cato_networks.event.host_mac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("host.mac") {
                gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
            }

            if event.has_value("host.mac") {
                map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
            }

            if event.has_value("json.http_request_method") {
                event.rename(
                    "json.http_request_method",
                    "cato_networks.event.http_request_method",
                )?;
            }

            if let Some(v) = event
                .get("cato_networks.event.http_request_method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.http_response_code") {
                    if let Some(val) = event.get("json.http_response_code") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.http_response_code".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.http_response_code", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_http_response_code_to_long",
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
                .get("cato_networks.event.http_response_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            if event.has_value("json.internalId") {
                event.rename("json.internalId", "cato_networks.event.internalid")?;
            }

            if event.has_value("json.internal_id") {
                event.rename("json.internal_id", "cato_networks.event.internal_id")?;
            }

            if event.has_value("json.ip_protocol") {
                event.rename("json.ip_protocol", "cato_networks.event.ip_protocol")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.ip_protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.transport", v)?;
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.is_sanctioned_app") {
                    if let Some(val) = event.get("json.is_sanctioned_app") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.is_sanctioned_app".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.is_sanctioned_app", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_sanctioned_app_to_boolean",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.is_sinkhole") {
                    if let Some(val) = event.get("json.is_sinkhole") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.is_sinkhole".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.is_sinkhole", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_sinkhole_to_boolean",
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

            if event.has_value("json.ISP_name") {
                event.rename("json.ISP_name", "cato_networks.event.isp_name")?;
            }

            if event.has_value("json.key_name") {
                event.rename("json.key_name", "cato_networks.event.key_name")?;
            }

            if event.has_value("json.login_type") {
                event.rename("json.login_type", "cato_networks.event.login_type")?;
            }

            if event.has_value("json.mitre_attack_subtechniques") {
                event.rename(
                    "json.mitre_attack_subtechniques",
                    "cato_networks.event.mitre_attack_subtechniques",
                )?;
            }

            let _cond = { event.has_value("cato_networks.event.mitre_attack_subtechniques") };
            if _cond {
                event.append_unique(
                    "threat.technique.subtechnique.name",
                    json!(
                        event
                            .get("cato_networks.event.mitre_attack_subtechniques")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.mitre_attack_tactics") {
                event.rename(
                    "json.mitre_attack_tactics",
                    "cato_networks.event.mitre_attack_tactics",
                )?;
            }

            let _cond = { event.has_value("cato_networks.event.mitre_attack_tactics") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) =
                        event.get_string("cato_networks.event.mitre_attack_tactics")
                    {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" (") else {
                                break 'dissect false;
                            };
                            captured.push(("_tmp_tactic_name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" (") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(")") else {
                                break 'dissect false;
                            };
                            captured.push(("_tmp_tactic_id", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(")") else {
                                break 'dissect false;
                            };
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
            }

            let _cond = { event.has_value("_tmp_tactic_name") };
            if _cond {
                event.append_unique(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("_tmp_tactic_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_tmp_tactic_id") };
            if _cond {
                event.append_unique(
                    "threat.tactic.id",
                    json!(
                        event
                            .get("_tmp_tactic_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("_tmp_tactic_name");
            event.remove("_tmp_tactic_id");

            if event.has_value("json.mitre_attack_techniques") {
                event.rename(
                    "json.mitre_attack_techniques",
                    "cato_networks.event.mitre_attack_techniques",
                )?;
            }

            let _cond = { event.has_value("cato_networks.event.mitre_attack_techniques") };
            if _cond {
                event.append_unique(
                    "threat.technique.name",
                    json!(
                        event
                            .get("cato_networks.event.mitre_attack_techniques")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.network_rule") {
                event.rename("json.network_rule", "cato_networks.event.network_rule")?;
            }

            if event.has_value("json.os_type") {
                event.rename("json.os_type", "cato_networks.event.os_type")?;
            }

            let _cond = { event.has_value("cato_networks.event.os_type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String os_type = ctx.cato_networks.event.os_type.toLowerCase();\nctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nfor (String os: params.os_type) {\n  if (os_type.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\nif (os_type.contains('centos') || os_type.contains('ubuntu')) {\n  ctx.host.os.put('type', 'linux');\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"String os_type = ctx.cato_networks.event.os_type.toLowerCase();\nctx.host = ctx.host ?: [:];\nctx.host.os = ctx.host.os ?: [:];\nfor (String os: params.os_type) {\n  if (os_type.contains(os)) {\n    ctx.host.os.put('type', os);\n    return;\n  }\n}\nif (os_type.contains('centos') || os_type.contains('ubuntu')) {\n  ctx.host.os.put('type', 'linux');\n}\n"#
                        ),
                        cached_params!(
                            "{\"os_type\":[\"linux\",\"macos\",\"unix\",\"windows\",\"ios\",\"android\"]}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_map_host_os_type",
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

            if event.has_value("json.os_version") {
                event.rename("json.os_version", "cato_networks.event.os_version")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.os_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has_value("json.pop_name") {
                event.rename("json.pop_name", "cato_networks.event.pop_name")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.pop_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if event.has_value("json.prompt_action") {
                event.rename("json.prompt_action", "cato_networks.event.prompt_action")?;
            }

            let _cond = { event.get_str("json.public_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.public_ip") {
                        if let Some(val) = event.get("json.public_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.public_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("cato_networks.event.public_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_public_ip_to_ip",
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

            let _cond = { event.has_value("cato_networks.event.public_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cato_networks.event.public_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.qos_priority") {
                    if let Some(val) = event.get("json.qos_priority") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.qos_priority".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.qos_priority", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_qos_priority_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.request_size") {
                    if let Some(val) = event.get("json.request_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.request_size".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.request_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_request_size_to_long",
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
                .get("cato_networks.event.request_size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.bytes", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.response_size") {
                    if let Some(val) = event.get("json.response_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.response_size".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.response_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_response_size_to_long",
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
                .get("cato_networks.event.response_size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.bytes", v)?;
            }

            if event.has_value("json.risk_level") {
                event.rename("json.risk_level", "cato_networks.event.risk_level")?;
            }

            let _cond = { event.has_value("cato_networks.event.risk_level") };
            if _cond {
                // Painless script
                // Source: def sev = params.levels.get(ctx.cato_networks.event.risk_level);\nif (sev != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.severity = sev;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def sev = params.levels.get(ctx.cato_networks.event.risk_level);\nif (sev != null) {\n  if (ctx.event == null) {\n    ctx.event = new HashMap();\n  }\n  ctx.event.severity = sev;\n}"#
                    ),
                    cached_params!(
                        "{\"levels\":{\"Low\":21,\"Medium\":47,\"High\":73,\"Critical\":99}}"
                    ),
                )?;
            }

            if event.has_value("json.rule_id") {
                event.rename("json.rule_id", "cato_networks.event.rule_id")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.rule_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has_value("json.rule_name") {
                event.rename("json.rule_name", "cato_networks.event.rule_name")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            let _cond = { event.get_str("json.server_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.server_ip") {
                        if let Some(val) = event.get("json.server_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.server_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("cato_networks.event.server_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_server_ip_to_ip",
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
                .get("cato_networks.event.server_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server.ip", v)?;
            }

            let _cond = { event.has_value("cato_networks.event.server_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cato_networks.event.server_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.signature_id") {
                event.rename("json.signature_id", "cato_networks.event.signature_id")?;
            }

            if event.has_value("json.socket_version") {
                event.rename("json.socket_version", "cato_networks.event.socket_version")?;
            }

            if event.has_value("json.src_country") {
                event.rename("json.src_country", "cato_networks.event.src_country")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.src_country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_name", v)?;
            }

            if event.has_value("json.src_country_code") {
                event.rename(
                    "json.src_country_code",
                    "cato_networks.event.src_country_code",
                )?;
            }

            if let Some(v) = event
                .get("cato_networks.event.src_country_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_iso_code", v)?;
            }

            let _cond = { event.get_str("json.src_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src_ip") {
                        if let Some(val) = event.get("json.src_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("cato_networks.event.src_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_src_ip_to_ip")?;
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
                .get("cato_networks.event.src_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event
                .get("cato_networks.event.src_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.address", v)?;
            }

            let _cond = { event.has_value("cato_networks.event.src_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cato_networks.event.src_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.src_is_site_or_vpn") {
                event.rename(
                    "json.src_is_site_or_vpn",
                    "cato_networks.event.src_is_site_or_vpn",
                )?;
            }

            let _cond = { event.get_str("json.src_isp_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.src_isp_ip") {
                        if let Some(val) = event.get("json.src_isp_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.src_isp_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("cato_networks.event.src_isp_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_src_isp_ip_to_ip",
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

            let _cond = { event.has_value("cato_networks.event.src_isp_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cato_networks.event.src_isp_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.src_port") {
                    if let Some(val) = event.get("json.src_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.src_port".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.src_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_src_port_to_long",
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
                .get("cato_networks.event.src_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.src_site_id") {
                event.rename("json.src_site_id", "cato_networks.event.src_site_id")?;
            }

            if event.has_value("json.src_site_name") {
                event.rename("json.src_site_name", "cato_networks.event.src_site_name")?;
            }

            let _cond = {
                event.get_str("cato_networks.event.src_is_site_or_vpn") == Some("SDP User")
                    && event.has_value("cato_networks.event.src_site_name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cato_networks.event.src_site_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.static_host") {
                    if let Some(val) = event.get("json.static_host") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.static_host".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.static_host", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_static_host_to_boolean",
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

            if event.has_value("json.subnet_name") {
                event.rename("json.subnet_name", "cato_networks.event.subnet_name")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.tcp_acceleration") {
                    if let Some(val) = event.get("json.tcp_acceleration") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.tcp_acceleration".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.tcp_acceleration", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_tcp_acceleration_to_boolean",
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

            if event.has_value("json.threat_name") {
                event.rename("json.threat_name", "cato_networks.event.threat_name")?;
            }

            let _cond = {
                event.has_value("cato_networks.event.threat_name")
                    || ["IPS", "DNS Protection"].contains(
                        &event
                            .get_str("cato_networks.event.event_sub_type")
                            .unwrap_or(""),
                    )
            };
            if _cond {
                event.append_unique("event.category", json!("intrusion_detection"))?;
            }

            let _cond = { event.has_value("cato_networks.event.threat_name") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has_value("json.threat_reference") {
                event.rename(
                    "json.threat_reference",
                    "cato_networks.event.threat_reference",
                )?;
            }

            if let Some(v) = event
                .get("cato_networks.event.threat_reference")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            if event.has_value("json.threat_type") {
                event.rename("json.threat_type", "cato_networks.event.threat_type")?;
            }

            let _cond = { event.has_value("json.time") && event.get_str("json.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("cato_networks.event.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_time")?;
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
                .get("cato_networks.event.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond =
                { event.has_value("json.time_str") && event.get_str("json.time_str") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time_str") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("cato_networks.event.time_str", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time_str".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_time_str")?;
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

            let _cond = { !event.has_value("cato_networks.event.time") };
            if _cond {
                if let Some(v) = event
                    .get("cato_networks.event.time_str")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.tls_inspection") {
                    if let Some(val) = event.get("json.tls_inspection") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.tls_inspection".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.tls_inspection", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_tls_inspection_to_boolean",
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

            if event.has_value("json.traffic_direction") {
                event.rename(
                    "json.traffic_direction",
                    "cato_networks.event.traffic_direction",
                )?;
            }

            let _cond =
                { event.get_str("cato_networks.event.traffic_direction") == Some("OUTBOUND") };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            let _cond =
                { event.get_str("cato_networks.event.traffic_direction") == Some("INBOUND") };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.transaction_size") {
                    if let Some(val) = event.get("json.transaction_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.transaction_size".into(),
                                message,
                            }
                        })?;
                        event.set("cato_networks.event.transaction_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_transaction_size_to_long",
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

            if event.has_value("json.url") {
                event.rename("json.url", "cato_networks.event.url")?;
            }

            if event.has_value("json.user_agent") {
                event.rename("json.user_agent", "cato_networks.event.user_agent")?;
            }

            let _cond = { event.has_value("cato_networks.event.user_agent") };
            if _cond {
                if event.has_value("cato_networks.event.user_agent") {
                    if let Some(ua_str) = event.get_string("cato_networks.event.user_agent") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.set("user_agent.original", json!(ua_str))?;
                            if let Some(name) = ua.name {
                                event.set("user_agent.name", json!(name))?;
                            }
                            if let Some(version) = ua.version {
                                event.set("user_agent.version", json!(version))?;
                            }
                            if let Some(os_name) = ua.os_name {
                                event.set("user_agent.os.name", json!(os_name))?;
                                if let Some(os_version) = ua.os_version {
                                    event.set("user_agent.os.version", json!(os_version))?;
                                    event.set(
                                        "user_agent.os.full",
                                        json!(format!("{} {}", os_name, os_version)),
                                    )?;
                                }
                            }
                            if let Some(device) = ua.device {
                                event.set("user_agent.device.name", json!(device))?;
                            }
                        }
                    }
                }
            }

            if event.has_value("json.user_reference_id") {
                event.rename(
                    "json.user_reference_id",
                    "cato_networks.event.user_reference_id",
                )?;
            }

            if event.has_value("json.user_id") {
                event.rename("json.user_id", "cato_networks.event.user_id")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("cato_networks.event.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cato_networks.event.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.user_name") {
                event.rename("json.user_name", "cato_networks.event.user_name")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("cato_networks.event.user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cato_networks.event.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.visible_device_id") {
                event.rename(
                    "json.visible_device_id",
                    "cato_networks.event.visible_device_id",
                )?;
            }

            if event.has_value("json.vpn_user_email") {
                event.rename("json.vpn_user_email", "cato_networks.event.vpn_user_email")?;
            }

            if let Some(v) = event
                .get("cato_networks.event.vpn_user_email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("cato_networks.event.vpn_user_email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("cato_networks.event.vpn_user_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("cato_networks.event.account_id");
            event.remove("cato_networks.event.account_name");
            event.remove("cato_networks.event.action");
            event.remove("cato_networks.event.application_name");
            event.remove("cato_networks.event.client_ip");
            event.remove("cato_networks.event.configured_host_name");
            event.remove("cato_networks.event.dest_country");
            event.remove("cato_networks.event.dest_country_code");
            event.remove("cato_networks.event.dest_ip");
            event.remove("cato_networks.event.dest_port");
            event.remove("cato_networks.event.device_id");
            event.remove("cato_networks.event.dns_query");
            event.remove("cato_networks.event.dns_record_type");
            event.remove("cato_networks.event.domain_name");
            event.remove("cato_networks.event.event_id");
            event.remove("cato_networks.event.event_message");
            event.remove("cato_networks.event.full_path_url");
            event.remove("cato_networks.event.host_ip");
            event.remove("cato_networks.event.host_mac");
            event.remove("cato_networks.event.http_request_method");
            event.remove("cato_networks.event.http_response_code");
            event.remove("cato_networks.event.ip_protocol");
            event.remove("cato_networks.event.mitre_attack_subtechniques");
            event.remove("cato_networks.event.mitre_attack_tactics");
            event.remove("cato_networks.event.mitre_attack_techniques");
            event.remove("cato_networks.event.os_version");
            event.remove("cato_networks.event.pop_name");
            event.remove("cato_networks.event.request_size");
            event.remove("cato_networks.event.response_size");
            event.remove("cato_networks.event.rule_id");
            event.remove("cato_networks.event.rule_name");
            event.remove("cato_networks.event.server_ip");
            event.remove("cato_networks.event.src_country");
            event.remove("cato_networks.event.src_country_code");
            event.remove("cato_networks.event.src_ip");
            event.remove("cato_networks.event.src_port");
            event.remove("cato_networks.event.threat_reference");
            event.remove("cato_networks.event.time_str");
            event.remove("cato_networks.event.user_agent");
            event.remove("cato_networks.event.user_id");
            event.remove("cato_networks.event.user_name");
            event.remove("cato_networks.event.vpn_user_email");

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}'\n{}failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}'\n",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
