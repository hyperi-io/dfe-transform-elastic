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

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("both")),
                    serde_json::Value::String(s) => s.contains("both"),
                    _ => false,
                })
            };
            if _cond {
                event.append("tags", json!("elastic_cloud_data"))?;
                event.append("tags", json!("provider_cloud_data"))?;
            }

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("both")),
                    serde_json::Value::String(s) => s.contains("both"),
                    _ => false,
                })
            };
            if _cond {
                // Painless script
                // Source: ctx.tags.remove(ctx.tags.indexOf('both'));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.tags.remove(ctx.tags.indexOf('both'));"#),
                )?;
            }

            let _cond = {
                !event.has_value("cloud")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("elastic_cloud_data"))
                        }
                        serde_json::Value::String(s) => s.contains("elastic_cloud_data"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: ctx.tags.remove(ctx.tags.indexOf('elastic_cloud_data'));
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.tags.remove(ctx.tags.indexOf('elastic_cloud_data'));"#),
                )?;
            }

            let _cond = {
                event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => {
                        a.iter().any(|x| x.as_str() == Some("provider_cloud_data"))
                    }
                    serde_json::Value::String(s) => s.contains("provider_cloud_data"),
                    _ => false,
                })
            };
            if _cond {
                event.set("_conf.want_provider_cloud", json!(true))?;
            }

            let _cond = {
                event.has_value("tags")
                    && !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("elastic_cloud_data"))
                        }
                        serde_json::Value::String(s) => s.contains("elastic_cloud_data"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("cloud");
                    Ok(())
                })();
            }

            if event.has_value("interval_id") {
                event.rename(
                    "interval_id",
                    "qualys_vmdr.asset_host_detection.interval_id",
                )?;
            }

            let _cond = {
                event.has_value("interval_start") && event.get_str("interval_start") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("interval_start") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("qualys_vmdr.asset_host_detection.interval_start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "interval_start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_interval_start")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("event.kind", json!("alert"))?;

            event.set("event.category", Value::Array(vec![json!("vulnerability")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            event.set("observer.vendor", json!("Qualys VMDR"))?;

            event.set("vulnerability.scanner.vendor", json!("Qualys"))?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "message", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_message")?;
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
                            .get("_ingest.pipeline")
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
                if event.has_value("json.ASSET_ID") {
                    if let Some(val) = event.get("json.ASSET_ID") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ASSET_ID".into(),
                                message,
                            }
                        })?;
                        event.set("qualys_vmdr.asset_host_detection.asset_id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ASSET_ID_to_long",
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
                            .get("_ingest.pipeline")
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

            if event.has_value("json.DNS") {
                event.rename("json.DNS", "qualys_vmdr.asset_host_detection.dns")?;
            }

            if event.has_value("json.DNS_DATA.DOMAIN") {
                event.rename(
                    "json.DNS_DATA.DOMAIN",
                    "qualys_vmdr.asset_host_detection.dns_data.domain",
                )?;
            }

            if event.has_value("json.DNS_DATA.FQDN") {
                event.rename(
                    "json.DNS_DATA.FQDN",
                    "qualys_vmdr.asset_host_detection.dns_data.fqdn",
                )?;
            }

            if event.has_value("json.DNS_DATA.HOSTNAME") {
                event.rename(
                    "json.DNS_DATA.HOSTNAME",
                    "qualys_vmdr.asset_host_detection.dns_data.hostname",
                )?;
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.dns_data.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_vmdr.asset_host_detection.dns_data.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let v = json!(
                event
                    .get("qualys_vmdr.asset_host_detection.dns_data.fqdn")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("host.name", v)?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                let v = json!(
                    event
                        .get("qualys_vmdr.asset_host_detection.dns_data.hostname")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("host.name", v)?;
                }
            }

            let v = json!(
                event
                    .get("qualys_vmdr.asset_host_detection.dns_data.hostname")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.dns_data.fqdn") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_vmdr.asset_host_detection.dns_data.fqdn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.ID") {
                event.rename("json.ID", "qualys_vmdr.asset_host_detection.id")?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.asset_host_detection.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.id", v)?;
            }

            if let Some(v) = event
                .get("host.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.name", v)?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.asset_host_detection.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.has_value("host.id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.NETBIOS") {
                event.rename("json.NETBIOS", "qualys_vmdr.asset_host_detection.netbios")?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.asset_host_detection.netbios")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.domain", v)?;
            }

            let _cond = { event.has_value("host.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.NETWORK_ID") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.NETWORK_ID") {
                        if let Some(val) = event.get("json.NETWORK_ID") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.NETWORK_ID".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_vmdr.asset_host_detection.network_id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_NETWORK_ID_to_string",
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
                                .get("_ingest.pipeline")
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

            if event.has_value("json.EC2_INSTANCE_ID") {
                event.rename(
                    "json.EC2_INSTANCE_ID",
                    "qualys_vmdr.asset_host_detection.ec2_instance_id",
                )?;
            }

            if event.has_value("json.CLOUD_RESOURCE_ID") {
                event.rename(
                    "json.CLOUD_RESOURCE_ID",
                    "qualys_vmdr.asset_host_detection.cloud_resource_id",
                )?;
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_vmdr.asset_host_detection.cloud_resource_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            if event.has_value("json.CLOUD_SERVICE") {
                event.rename(
                    "json.CLOUD_SERVICE",
                    "qualys_vmdr.asset_host_detection.cloud_service",
                )?;
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_vmdr.asset_host_detection.cloud_service")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.service.name", v)?;
                }
            }

            if event.has_value("json.CLOUD_PROVIDER") {
                event.rename(
                    "json.CLOUD_PROVIDER",
                    "qualys_vmdr.asset_host_detection.cloud_provider",
                )?;
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.cloud_provider") {
                    map_strings(
                        event,
                        "qualys_vmdr.asset_host_detection.cloud_provider",
                        "cloud.provider",
                        str::to_lowercase,
                    )?;
                }
            }

            if event.has_value("json.QG_HOSTID") {
                event.rename(
                    "json.QG_HOSTID",
                    "qualys_vmdr.asset_host_detection.qg_hostid",
                )?;
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.qg_hostid") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_vmdr.asset_host_detection.qg_hostid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.OS_CPE") {
                event.rename("json.OS_CPE", "qualys_vmdr.asset_host_detection.os_cpe")?;
            }

            if event.has_value("json.OS") {
                event.rename("json.OS", "qualys_vmdr.asset_host_detection.os")?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.asset_host_detection.os")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            let _cond = {
                event.has_value("host.os.full")
                    && event
                        .get_str("host.os.full")
                        .is_some_and(|s| s.to_lowercase().contains("linux"))
            };
            if _cond {
                let v = json!("linux");
                if !painless_is_empty_value(&v) {
                    event.set("host.os.platform", v)?;
                }
            }

            let _cond = {
                event.has_value("host.os.full")
                    && event
                        .get_str("host.os.full")
                        .is_some_and(|s| s.to_lowercase().contains("windows"))
            };
            if _cond {
                let v = json!("windows");
                if !painless_is_empty_value(&v) {
                    event.set("host.os.platform", v)?;
                }
            }

            if let Some(v) = event
                .get("host.os.platform")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.type", v)?;
            }

            let _cond = {
                event.has_value("host.os.full")
                    && event
                        .get_str("host.os.full")
                        .is_some_and(|s| s.to_lowercase().contains("macos"))
            };
            if _cond {
                let v = json!("darwin");
                if !painless_is_empty_value(&v) {
                    event.set("host.os.platform", v)?;
                }
            }

            let _cond = {
                event.has_value("host.os.full")
                    && event
                        .get_str("host.os.full")
                        .is_some_and(|s| s.to_lowercase().contains("macos"))
            };
            if _cond {
                let v = json!("macos");
                if !painless_is_empty_value(&v) {
                    event.set("host.os.type", v)?;
                }
            }

            if event.has_value("json.TRACKING_METHOD") {
                event.rename(
                    "json.TRACKING_METHOD",
                    "qualys_vmdr.asset_host_detection.tracking_method",
                )?;
            }

            let _cond = { event.get_str("json.IP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IP") {
                        if let Some(val) = event.get("json.IP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IP".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_vmdr.asset_host_detection.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_IP_to_ip")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("qualys_vmdr.asset_host_detection.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.IPV6") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.IPV6") {
                        if let Some(val) = event.get("json.IPV6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.IPV6".into(),
                                    message,
                                }
                            })?;
                            event.set("qualys_vmdr.asset_host_detection.ipv6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_IPV6_to_ip")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.ipv6") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("qualys_vmdr.asset_host_detection.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("qualys_vmdr.asset_host_detection.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.LAST_PC_SCANNED_DATE")
                    && event.get_str("json.LAST_PC_SCANNED_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.LAST_PC_SCANNED_DATE") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_vmdr.asset_host_detection.last_pc_scanned_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LAST_PC_SCANNED_DATE".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_LAST_PC_SCANNED_DATE",
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event.has_value("json.LAST_SCAN_DATETIME")
                    && event.get_str("json.LAST_SCAN_DATETIME") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.LAST_SCAN_DATETIME") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_vmdr.asset_host_detection.last_scan_datetime",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LAST_SCAN_DATETIME".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_LAST_SCAN_DATETIME",
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event.has_value("json.LAST_VM_AUTH_SCANNED_DATE")
                    && event.get_str("json.LAST_VM_AUTH_SCANNED_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.LAST_VM_AUTH_SCANNED_DATE") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_vmdr.asset_host_detection.last_vm_auth_scanned_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LAST_VM_AUTH_SCANNED_DATE".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_LAST_VM_AUTH_SCANNED_DATE",
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event.has_value("json.LAST_VM_SCANNED_DATE")
                    && event.get_str("json.LAST_VM_SCANNED_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.LAST_VM_SCANNED_DATE") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_vmdr.asset_host_detection.last_vm_scanned_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.LAST_VM_SCANNED_DATE".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_LAST_VM_SCANNED_DATE",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.LAST_VM_AUTH_SCANNED_DURATION") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.LAST_VM_AUTH_SCANNED_DURATION") {
                        if let Some(val) = event.get("json.LAST_VM_AUTH_SCANNED_DURATION") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.LAST_VM_AUTH_SCANNED_DURATION".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "qualys_vmdr.asset_host_detection.last_vm_auth_scanned_duration",
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
                        "convert_LAST_VM_AUTH_SCANNED_DURATION_to_long",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get_str("json.LAST_VM_SCANNED_DURATION") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.LAST_VM_SCANNED_DURATION") {
                        if let Some(val) = event.get("json.LAST_VM_SCANNED_DURATION") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.LAST_VM_SCANNED_DURATION".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "qualys_vmdr.asset_host_detection.last_vm_scanned_duration",
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
                        "convert_LAST_VM_SCANNED_DURATION_to_long",
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.get("json.TAGS.TAG").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.TAGS.TAG", |event| {
                    if event.has_value("_ingest._value.TAG_ID") {
                        event.rename("_ingest._value.TAG_ID", "_ingest._value.id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.TAGS.TAG").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.TAGS.TAG", |event| {
                    if event.has_value("_ingest._value.NAME") {
                        event.rename("_ingest._value.NAME", "_ingest._value.name")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.TAGS.TAG").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.TAGS.TAG", |event| {
                    if event.has_value("_ingest._value.COLOR") {
                        event.rename("_ingest._value.COLOR", "_ingest._value.color")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.TAGS.TAG").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.TAGS.TAG", |event| {
                    if event.has_value("_ingest._value.BACKGROUND_COLOR") {
                        event.rename(
                            "_ingest._value.BACKGROUND_COLOR",
                            "_ingest._value.background_color",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.TAGS.TAG") {
                event.rename("json.TAGS.TAG", "qualys_vmdr.asset_host_detection.tags")?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.tags.NAME") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.tags.NAME",
                    "qualys_vmdr.asset_host_detection.tags.name",
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.tags.TAG_ID") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.tags.TAG_ID",
                    "qualys_vmdr.asset_host_detection.tags.id",
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.tags.COLOR") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.tags.COLOR",
                    "qualys_vmdr.asset_host_detection.tags.color",
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.tags.BACKGROUND_COLOR") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.tags.BACKGROUND_COLOR",
                    "qualys_vmdr.asset_host_detection.tags.background_color",
                )?;
            }

            if event.has_value("json.METADATA.EC2.ATTRIBUTE") {
                event.rename(
                    "json.METADATA.EC2.ATTRIBUTE",
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute",
                    |event| {
                        if event.has_value("_ingest._value.NAME") {
                            event.rename("_ingest._value.NAME", "_ingest._value.name")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.metadata.ec2.attribute.NAME") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.NAME",
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.name",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute",
                    |event| {
                        if event.has_value("_ingest._value.LAST_STATUS") {
                            event.rename(
                                "_ingest._value.LAST_STATUS",
                                "_ingest._value.last.status",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event
                .has_value("qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_STATUS")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_STATUS",
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.last.status",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute",
                    |event| {
                        if event.has_value("_ingest._value.VALUE") {
                            event.rename("_ingest._value.VALUE", "_ingest._value.value")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.metadata.ec2.attribute.VALUE") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.VALUE",
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.value",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute",
                    |event| {
                        if event.has_value("_ingest._value.LAST_ERROR") {
                            event.rename(
                                "_ingest._value.LAST_ERROR",
                                "_ingest._value.last.error.value",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_ERROR")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_ERROR",
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.last.error.value",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LAST_SUCCESS_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last.success_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.LAST_SUCCESS_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "qualys_vmdr.asset_host_detection.metadata.ec2.attribute",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                (!(event
                    .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                    .is_some_and(|v| v.is_array())))
                    && event.has_value(
                        "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_SUCCESS_DATE",
                    )
                    && event.get_str(
                        "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_SUCCESS_DATE",
                    ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_SUCCESS_DATE",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.metadata.ec2.attribute.last.success_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_SUCCESS_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_asset_host_detection_metadata_ec2_attribute_LAST_SUCCESS_DATE_2")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LAST_ERROR_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last.error.date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.LAST_ERROR_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "qualys_vmdr.asset_host_detection.metadata.ec2.attribute",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                (!(event
                    .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                    .is_some_and(|v| v.is_array())))
                    && event.has_value(
                        "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_ERROR_DATE",
                    )
                    && event.get_str(
                        "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_ERROR_DATE",
                    ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_ERROR_DATE",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.metadata.ec2.attribute.last.error.date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_ERROR_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_asset_host_detection_metadata_ec2_attribute_LAST_ERROR_DATE_2")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.ec2.attribute",
                    |event| {
                        event.remove("_ingest._value.LAST_SUCCESS_DATE");
                        event.remove("_ingest._value.LAST_ERROR_DATE");
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.METADATA.GOOGLE.ATTRIBUTE") {
                event.rename(
                    "json.METADATA.GOOGLE.ATTRIBUTE",
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute",
                    |event| {
                        if event.has_value("_ingest._value.NAME") {
                            event.rename("_ingest._value.NAME", "_ingest._value.name")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.metadata.google.attribute.NAME") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute.NAME",
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute.name",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute",
                    |event| {
                        if event.has_value("_ingest._value.LAST_STATUS") {
                            event.rename(
                                "_ingest._value.LAST_STATUS",
                                "_ingest._value.last.status",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event
                .has_value("qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_STATUS")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_STATUS",
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute.last.status",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute",
                    |event| {
                        if event.has_value("_ingest._value.VALUE") {
                            event.rename("_ingest._value.VALUE", "_ingest._value.value")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.metadata.google.attribute.VALUE") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute.VALUE",
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute.value",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute",
                    |event| {
                        if event.has_value("_ingest._value.LAST_ERROR") {
                            event.rename(
                                "_ingest._value.LAST_ERROR",
                                "_ingest._value.last.error.value",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event
                .has_value("qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_ERROR")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_ERROR",
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute.last.error.value",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LAST_SUCCESS_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last.success_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.LAST_SUCCESS_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "qualys_vmdr.asset_host_detection.metadata.google.attribute",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                (!(event.get("qualys_vmdr.asset_host_detection.metadata.google.attribute").is_some_and(|v| v.is_array()))) && event.has_value("qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_SUCCESS_DATE") && event.get_str("qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_SUCCESS_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_SUCCESS_DATE") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.metadata.google.attribute.last.success_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_SUCCESS_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_asset_host_detection_metadata_google_attribute_LAST_SUCCESS_DATE_2")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LAST_ERROR_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last.error.date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.LAST_ERROR_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "qualys_vmdr.asset_host_detection.metadata.google.attribute",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                (!(event.get("qualys_vmdr.asset_host_detection.metadata.google.attribute").is_some_and(|v| v.is_array()))) && event.has_value("qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_ERROR_DATE") && event.get_str("qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_ERROR_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_ERROR_DATE") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.metadata.google.attribute.last.error.date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_ERROR_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_asset_host_detection_metadata_google_attribute_LAST_ERROR_DATE_2")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.google.attribute",
                    |event| {
                        event.remove("_ingest._value.LAST_SUCCESS_DATE");
                        event.remove("_ingest._value.LAST_ERROR_DATE");
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.METADATA.AZURE.ATTRIBUTE") {
                event.rename(
                    "json.METADATA.AZURE.ATTRIBUTE",
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute",
                    |event| {
                        if event.has_value("_ingest._value.NAME") {
                            event.rename("_ingest._value.NAME", "_ingest._value.name")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.metadata.azure.attribute.NAME") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute.NAME",
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute.name",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute",
                    |event| {
                        if event.has_value("_ingest._value.LAST_STATUS") {
                            event.rename(
                                "_ingest._value.LAST_STATUS",
                                "_ingest._value.last.status",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event
                .has_value("qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_STATUS")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_STATUS",
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute.last.status",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute",
                    |event| {
                        if event.has_value("_ingest._value.VALUE") {
                            event.rename("_ingest._value.VALUE", "_ingest._value.value")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.metadata.azure.attribute.VALUE") {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute.VALUE",
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute.value",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute",
                    |event| {
                        if event.has_value("_ingest._value.LAST_ERROR") {
                            event.rename(
                                "_ingest._value.LAST_ERROR",
                                "_ingest._value.last.error.value",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event
                .has_value("qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_ERROR")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_ERROR",
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute.last.error.value",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LAST_SUCCESS_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last.success_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.LAST_SUCCESS_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "qualys_vmdr.asset_host_detection.metadata.azure.attribute",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                (!(event.get("qualys_vmdr.asset_host_detection.metadata.azure.attribute").is_some_and(|v| v.is_array()))) && event.has_value("qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_SUCCESS_DATE") && event.get_str("qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_SUCCESS_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_SUCCESS_DATE") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.metadata.azure.attribute.last.success_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_SUCCESS_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_asset_host_detection_metadata_azure_attribute_LAST_SUCCESS_DATE_2")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LAST_ERROR_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last.error.date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.LAST_ERROR_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "qualys_vmdr.asset_host_detection.metadata.azure.attribute",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                (!(event
                    .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                    .is_some_and(|v| v.is_array())))
                    && event.has_value(
                        "qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_ERROR_DATE",
                    )
                    && event.get_str(
                        "qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_ERROR_DATE",
                    ) != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_ERROR_DATE",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.metadata.azure.attribute.last.error.date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_ERROR_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_asset_host_detection_metadata_azure_attribute_LAST_ERROR_DATE_2")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.azure.attribute",
                    |event| {
                        event.remove("_ingest._value.LAST_SUCCESS_DATE");
                        event.remove("_ingest._value.LAST_ERROR_DATE");
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.METADATA.ALICLOUD.ATTRIBUTE") {
                event.rename(
                    "json.METADATA.ALICLOUD.ATTRIBUTE",
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute",
                    |event| {
                        if event.has_value("_ingest._value.NAME") {
                            event.rename("_ingest._value.NAME", "_ingest._value.name")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.NAME")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.NAME",
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.name",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute",
                    |event| {
                        if event.has_value("_ingest._value.LAST_STATUS") {
                            event.rename(
                                "_ingest._value.LAST_STATUS",
                                "_ingest._value.last.status",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value(
                "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_STATUS",
            ) {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_STATUS",
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.last.status",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute",
                    |event| {
                        if event.has_value("_ingest._value.VALUE") {
                            event.rename("_ingest._value.VALUE", "_ingest._value.value")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.VALUE")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.VALUE",
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.value",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute",
                    |event| {
                        if event.has_value("_ingest._value.LAST_ERROR") {
                            event.rename(
                                "_ingest._value.LAST_ERROR",
                                "_ingest._value.last.error.value",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value(
                "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_ERROR",
            ) {
                event.rename(
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_ERROR",
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.last.error.value",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LAST_SUCCESS_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last.success_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.LAST_SUCCESS_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                (!(event.get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute").is_some_and(|v| v.is_array()))) && event.has_value("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_SUCCESS_DATE") && event.get_str("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_SUCCESS_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_SUCCESS_DATE") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.last.success_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_SUCCESS_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_asset_host_detection_metadata_alicloud_attribute_LAST_SUCCESS_DATE_2")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LAST_ERROR_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last.error.date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.LAST_ERROR_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                (!(event.get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute").is_some_and(|v| v.is_array()))) && event.has_value("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_ERROR_DATE") && event.get_str("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_ERROR_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_ERROR_DATE") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.last.error.date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_ERROR_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_asset_host_detection_metadata_alicloud_attribute_LAST_ERROR_DATE_2")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.metadata.alicloud.attribute")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute",
                    |event| {
                        event.remove("_ingest._value.LAST_SUCCESS_DATE");
                        event.remove("_ingest._value.LAST_ERROR_DATE");
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.CLOUD_PROVIDER_TAGS.CLOUD_TAG") {
                event.rename(
                    "json.CLOUD_PROVIDER_TAGS.CLOUD_TAG",
                    "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag",
                    |event| {
                        if event.has_value("_ingest._value.NAME") {
                            event.rename("_ingest._value.NAME", "_ingest._value.name")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event
                .has_value("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.NAME")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.NAME",
                    "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.name",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag",
                    |event| {
                        if event.has_value("_ingest._value.VALUE") {
                            event.rename("_ingest._value.VALUE", "_ingest._value.value")?;
                        }
                        Ok(())
                    },
                )?;
            }

            if event
                .has_value("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.VALUE")
            {
                event.rename(
                    "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.VALUE",
                    "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.value",
                )?;
            }

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.LAST_SUCCESS_DATE")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.last_success_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.LAST_SUCCESS_DATE".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                (!(event.get("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag").is_some_and(|v| v.is_array()))) && event.has_value("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.LAST_SUCCESS_DATE") && event.get_str("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.LAST_SUCCESS_DATE") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.LAST_SUCCESS_DATE") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.last_success_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.LAST_SUCCESS_DATE".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_qualys_vmdr_asset_host_detection_cloud_provider_tags_cloud_tag_LAST_SUCCESS_DATE_2")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event
                    .get("qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag",
                    |event| {
                        event.remove("_ingest._value.LAST_SUCCESS_DATE");
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.get_bool("_conf.want_provider_cloud") == Some(true)
                    && (event
                        .get("qualys_vmdr.asset_host_detection.metadata.ec2.attribute")
                        .is_some_and(|v| v.is_array())
                        || event
                            .get("qualys_vmdr.asset_host_detection.metadata.google.attribute")
                            .is_some_and(|v| v.is_array())
                        || event
                            .get("qualys_vmdr.asset_host_detection.metadata.azure.attribute")
                            .is_some_and(|v| v.is_array()))
            };
            if _cond {
                // Painless script
                // Source: def accIDs = new ArrayList();\ndef projIDs = new ArrayList();\ndef names = new ArrayList();\ndef regions = new ArrayList();\ndef types = new ArrayList();\ndef zones = new ArrayList();\n// Collate attributes.\nif (ctx.qualys_vmdr?.asset_host_detection?.metadata?.ec2?.attribute instanceof List) {\n  for (def attr: ctx.qualys_vmdr.asset_host_detection.metadata.ec2.attribute) {\n    if (attr.value == null && attr.value == \"\") {\n      continue;\n    }\n    if (attr.name == \"latest/dynamic/instance-identity/document/accountId\") {\n      accIDs.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"latest/dynamic/instance-identity/document/instanceType\") {\n      types.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"latest/dynamic/instance-identity/document/region\") {\n      regions.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"latest/dynamic/instance-identity/document/availabilityZone\") {\n      zones.add(attr.value);\n    }\n  }\n}\nif (ctx.qualys_vmdr?.asset_host_detection?.metadata?.google?.attribute instanceof List) {\n  for (def attr: ctx.qualys_vmdr.asset_host_detection.metadata.google.attribute) {\n    if (attr.value == null && attr.value == \"\") {\n      continue;\n    }\n    if (attr.name == \"projectId\") {\n      names.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"projectIdNo\") {\n      projIDs.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"machineType\") {\n      types.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"location\") {\n      regions.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"zone\") {\n      zones.add(attr.value);\n    }\n  }\n}\nif (ctx.qualys_vmdr?.asset_host_detection?.metadata?.azure?.attribute instanceof List) {\n  for (def attr: ctx.qualys_vmdr.asset_host_detection.metadata.azure.attribute) {\n    if (attr.value == null && attr.value == \"\") {\n      continue;\n    }\n    if (attr.name == \"subscriptionId\") {\n      projIDs.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"location\") {\n      regions.add(attr.value);\n    }\n  }\n}\n// Apply collation.\nif (accIDs.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  if (ctx.cloud.account == null) {\n    ctx.cloud.account = new HashMap();\n  }\n  ctx.cloud.account.id = accIDs;\n}\nif (projIDs.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  if (ctx.cloud.project == null) {\n    ctx.cloud.project = new HashMap();\n  }\n  ctx.cloud.project.id = projIDs;\n  if (ctx.cloud?.account?.id == null) {\n    if (ctx.cloud.account == null) {\n      ctx.cloud.account = new HashMap();\n    }\n    ctx.cloud.account.id = projIDs;\n  }\n}\nif (names.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  if (ctx.cloud.project == null) {\n    ctx.cloud.project = new HashMap();\n  }\n  ctx.cloud.project.name = names;\n  if (ctx.cloud?.account?.name == null) {\n    if (ctx.cloud.account == null) {\n      ctx.cloud.account = new HashMap();\n    }\n    ctx.cloud.account.name = names;\n  }\n}\nif (regions.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  ctx.cloud.region = regions;\n}\nif (types.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  if (ctx.cloud.machine == null) {\n    ctx.cloud.machine = new HashMap();\n  }\n  ctx.cloud.machine.type = types;\n}\nif (zones.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  ctx.cloud.availability_zone = zones;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def accIDs = new ArrayList();\ndef projIDs = new ArrayList();\ndef names = new ArrayList();\ndef regions = new ArrayList();\ndef types = new ArrayList();\ndef zones = new ArrayList();\n// Collate attributes.\nif (ctx.qualys_vmdr?.asset_host_detection?.metadata?.ec2?.attribute instanceof List) {\n  for (def attr: ctx.qualys_vmdr.asset_host_detection.metadata.ec2.attribute) {\n    if (attr.value == null && attr.value == \"\") {\n      continue;\n    }\n    if (attr.name == \"latest/dynamic/instance-identity/document/accountId\") {\n      accIDs.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"latest/dynamic/instance-identity/document/instanceType\") {\n      types.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"latest/dynamic/instance-identity/document/region\") {\n      regions.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"latest/dynamic/instance-identity/document/availabilityZone\") {\n      zones.add(attr.value);\n    }\n  }\n}\nif (ctx.qualys_vmdr?.asset_host_detection?.metadata?.google?.attribute instanceof List) {\n  for (def attr: ctx.qualys_vmdr.asset_host_detection.metadata.google.attribute) {\n    if (attr.value == null && attr.value == \"\") {\n      continue;\n    }\n    if (attr.name == \"projectId\") {\n      names.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"projectIdNo\") {\n      projIDs.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"machineType\") {\n      types.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"location\") {\n      regions.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"zone\") {\n      zones.add(attr.value);\n    }\n  }\n}\nif (ctx.qualys_vmdr?.asset_host_detection?.metadata?.azure?.attribute instanceof List) {\n  for (def attr: ctx.qualys_vmdr.asset_host_detection.metadata.azure.attribute) {\n    if (attr.value == null && attr.value == \"\") {\n      continue;\n    }\n    if (attr.name == \"subscriptionId\") {\n      projIDs.add(attr.value);\n      continue;\n    }\n    if (attr.name == \"location\") {\n      regions.add(attr.value);\n    }\n  }\n}\n// Apply collation.\nif (accIDs.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  if (ctx.cloud.account == null) {\n    ctx.cloud.account = new HashMap();\n  }\n  ctx.cloud.account.id = accIDs;\n}\nif (projIDs.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  if (ctx.cloud.project == null) {\n    ctx.cloud.project = new HashMap();\n  }\n  ctx.cloud.project.id = projIDs;\n  if (ctx.cloud?.account?.id == null) {\n    if (ctx.cloud.account == null) {\n      ctx.cloud.account = new HashMap();\n    }\n    ctx.cloud.account.id = projIDs;\n  }\n}\nif (names.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  if (ctx.cloud.project == null) {\n    ctx.cloud.project = new HashMap();\n  }\n  ctx.cloud.project.name = names;\n  if (ctx.cloud?.account?.name == null) {\n    if (ctx.cloud.account == null) {\n      ctx.cloud.account = new HashMap();\n    }\n    ctx.cloud.account.name = names;\n  }\n}\nif (regions.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  ctx.cloud.region = regions;\n}\nif (types.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  if (ctx.cloud.machine == null) {\n    ctx.cloud.machine = new HashMap();\n  }\n  ctx.cloud.machine.type = types;\n}\nif (zones.length != 0) {\n  if (ctx.cloud == null) {\n    ctx.cloud = new HashMap();\n  }\n  ctx.cloud.availability_zone = zones;\n}"#
                    ),
                )?;
            }

            let _cond = { event.get_bool("_conf.want_provider_cloud") == Some(true) };
            if _cond {
                if let Some(v) = event
                    .get("qualys_vmdr.asset_host_detection.dns_data.hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            let _cond = { event.has_value("json.DETECTION_LIST") };
            if _cond {
                // Painless script
                // Source: def obj = ctx.json.DETECTION_LIST; if (obj.containsKey(\"IS_DISABLED\") && obj.get(\"IS_DISABLED\").equals('0')) {\n  obj.remove(\"IS_DISABLED\");\n  obj.put(\"IS_DISABLED\", false);\n} else if (obj.containsKey(\"IS_DISABLED\") && obj.get(\"IS_DISABLED\").equals('1')) {\n  obj.remove(\"IS_DISABLED\");\n  obj.put(\"IS_DISABLED\", true);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def obj = ctx.json.DETECTION_LIST; if (obj.containsKey(\"IS_DISABLED\") && obj.get(\"IS_DISABLED\").equals('0')) {\n  obj.remove(\"IS_DISABLED\");\n  obj.put(\"IS_DISABLED\", false);\n} else if (obj.containsKey(\"IS_DISABLED\") && obj.get(\"IS_DISABLED\").equals('1')) {\n  obj.remove(\"IS_DISABLED\");\n  obj.put(\"IS_DISABLED\", true);\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("json.DETECTION_LIST") };
            if _cond {
                // Painless script
                // Source: def obj = ctx.json.DETECTION_LIST; if (obj.containsKey(\"IS_IGNORED\") && obj.get(\"IS_IGNORED\").equals('0')) {\n  obj.remove(\"IS_IGNORED\");\n  obj.put(\"IS_IGNORED\", false);\n} else if (obj.containsKey(\"IS_IGNORED\") && obj.get(\"IS_IGNORED\").equals('1')) {\n  obj.remove(\"IS_IGNORED\");\n  obj.put(\"IS_IGNORED\", true);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def obj = ctx.json.DETECTION_LIST; if (obj.containsKey(\"IS_IGNORED\") && obj.get(\"IS_IGNORED\").equals('0')) {\n  obj.remove(\"IS_IGNORED\");\n  obj.put(\"IS_IGNORED\", false);\n} else if (obj.containsKey(\"IS_IGNORED\") && obj.get(\"IS_IGNORED\").equals('1')) {\n  obj.remove(\"IS_IGNORED\");\n  obj.put(\"IS_IGNORED\", true);\n}"#
                    ),
                )?;
            }

            if event.has_value("json.DETECTION_LIST") {
                event.rename(
                    "json.DETECTION_LIST",
                    "qualys_vmdr.asset_host_detection.vulnerability",
                )?;
            }

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.UNIQUE_VULN_ID")
            };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.UNIQUE_VULN_ID")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.UNIQUE_VULN_ID",
                        "qualys_vmdr.asset_host_detection.vulnerability.unique_vuln_id",
                    )?;
                }
            }

            if let Some(v) = event
                .get("qualys_vmdr.asset_host_detection.vulnerability.unique_vuln_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.PROTOCOL") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.PROTOCOL") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.PROTOCOL",
                        "qualys_vmdr.asset_host_detection.vulnerability.protocol",
                    )?;
                }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.FQDN") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.FQDN") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.FQDN",
                        "qualys_vmdr.asset_host_detection.vulnerability.fqdn",
                    )?;
                }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.fqdn") };
            if _cond {
                event.append(
                    "related.hosts",
                    json!(
                        event
                            .get("qualys_vmdr.asset_host_detection.vulnerability.fqdn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.INSTANCE") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.INSTANCE") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.INSTANCE",
                        "qualys_vmdr.asset_host_detection.vulnerability.instance",
                    )?;
                }
            }

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.SERVICE") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.SERVICE") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.SERVICE",
                        "qualys_vmdr.asset_host_detection.vulnerability.service",
                    )?;
                }
            }

            let _cond = {
                event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.AFFECT_RUNNING_KERNEL",
                )
            };
            if _cond {
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.AFFECT_RUNNING_KERNEL",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.AFFECT_RUNNING_KERNEL",
                        "qualys_vmdr.asset_host_detection.vulnerability.affect_running_kernel",
                    )?;
                }
            }

            let _cond = {
                event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.AFFECT_RUNNING_SERVICE",
                )
            };
            if _cond {
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.AFFECT_RUNNING_SERVICE",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.AFFECT_RUNNING_SERVICE",
                        "qualys_vmdr.asset_host_detection.vulnerability.affect_running_service",
                    )?;
                }
            }

            let _cond = {
                event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.AFFECT_EXPLOITABLE_CONFIG",
                )
            };
            if _cond {
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.AFFECT_EXPLOITABLE_CONFIG",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.AFFECT_EXPLOITABLE_CONFIG",
                        "qualys_vmdr.asset_host_detection.vulnerability.affect_exploitable_config",
                    )?;
                }
            }

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.ASSET_CVE") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.ASSET_CVE") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.ASSET_CVE",
                        "qualys_vmdr.asset_host_detection.vulnerability.asset_cve",
                    )?;
                }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.CVE") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.CVE") {
                    if let Some(s) =
                        event.get_string("qualys_vmdr.asset_host_detection.vulnerability.CVE")
                    {
                        let parts: Vec<Value> = cached_regex!(",\\s?")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set(
                            "qualys_vmdr.asset_host_detection.vulnerability.cve",
                            Value::Array(parts),
                        )?;
                    }
                }
            }

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.LATEST_VULNERABILITY_DETECTION_SOURCE")
            };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.LATEST_VULNERABILITY_DETECTION_SOURCE") {
                    event.rename("qualys_vmdr.asset_host_detection.vulnerability.LATEST_VULNERABILITY_DETECTION_SOURCE", "qualys_vmdr.asset_host_detection.vulnerability.latest_vulnerability_detection_source")?;
                }
            }

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TACTIC_NAME")
            };
            if _cond {
                if event
                    .has_value("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TACTIC_NAME")
                {
                    if let Some(s) = event.get_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.MITRE_TACTIC_NAME",
                    ) {
                        let parts: Vec<Value> = cached_regex!(",\\s?")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set(
                            "qualys_vmdr.asset_host_detection.vulnerability.mitre_tactic_name",
                            Value::Array(parts),
                        )?;
                    }
                }
            }

            let _cond = {
                event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.MITRE_TECHNIQUE_NAME",
                )
            };
            if _cond {
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.MITRE_TECHNIQUE_NAME",
                ) {
                    if let Some(s) = event.get_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.MITRE_TECHNIQUE_NAME",
                    ) {
                        let parts: Vec<Value> = cached_regex!(",\\s?")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set(
                            "qualys_vmdr.asset_host_detection.vulnerability.mitre_technique_name",
                            Value::Array(parts),
                        )?;
                    }
                }
            }

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TACTIC_ID")
            };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TACTIC_ID")
                {
                    if let Some(s) = event.get_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.MITRE_TACTIC_ID",
                    ) {
                        let parts: Vec<Value> = cached_regex!(",\\s?")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set(
                            "qualys_vmdr.asset_host_detection.vulnerability.mitre_tactic_id",
                            Value::Array(parts),
                        )?;
                    }
                }
            }

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TECHNIQUE_ID")
            };
            if _cond {
                if event
                    .has_value("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TECHNIQUE_ID")
                {
                    if let Some(s) = event.get_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.MITRE_TECHNIQUE_ID",
                    ) {
                        let parts: Vec<Value> = cached_regex!(",\\s?")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set(
                            "qualys_vmdr.asset_host_detection.vulnerability.mitre_technique_id",
                            Value::Array(parts),
                        )?;
                    }
                }
            }

            let _cond = {
                event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.TRURISK_ELIMINATION_STATUS",
                )
            };
            if _cond {
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.TRURISK_ELIMINATION_STATUS",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.TRURISK_ELIMINATION_STATUS",
                        "qualys_vmdr.asset_host_detection.vulnerability.trurisk_elimination_status",
                    )?;
                }
            }

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.VULNERABILITY_DETECTION_SOURCES")
            };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.VULNERABILITY_DETECTION_SOURCES") {
                if let Some(s) = event.get_string("qualys_vmdr.asset_host_detection.vulnerability.VULNERABILITY_DETECTION_SOURCES") {
                    let parts: Vec<Value> = cached_regex!(",\\s?")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("qualys_vmdr.asset_host_detection.vulnerability.vulnerability_detection_sources", Value::Array(parts))?;
                }
            }
            }

            event.remove("qualys_vmdr.asset_host_detection.vulnerability.CVE");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TACTIC_NAME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TECHNIQUE_NAME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TACTIC_ID");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.MITRE_TECHNIQUE_ID");
            event.remove(
                "qualys_vmdr.asset_host_detection.vulnerability.VULNERABILITY_DETECTION_SOURCES",
            );

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.QID") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.QID") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.QID",
                        "qualys_vmdr.asset_host_detection.vulnerability.qid",
                    )?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.qid") {
                    if let Some(val) =
                        event.get("qualys_vmdr.asset_host_detection.vulnerability.qid")
                    {
                        let converted = convert_value(val, "integer").map_err(|message| {
                            TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.qid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "qualys_vmdr.asset_host_detection.vulnerability.qid",
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
                    "convert_qualys_vmdr_asset_host_detection_vulnerability_qid_1_to_integer",
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
                            .get("_ingest.pipeline")
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

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.RESULTS") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.RESULTS") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.RESULTS",
                        "qualys_vmdr.asset_host_detection.vulnerability.results",
                    )?;
                }
            }

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.results")
                    && event
                        .get_str("qualys_vmdr.asset_host_detection.vulnerability.results")
                        .is_some_and(|s| s.starts_with("Package"))
            };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.results") {
                    gsub_field(
                        event,
                        "qualys_vmdr.asset_host_detection.vulnerability.results",
                        "qualys_vmdr.asset_host_detection.vulnerability.results",
                        cached_regex!("\\n"),
                        ";;",
                    )?;
                }
            }

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.results")
                    && event
                        .get_str("qualys_vmdr.asset_host_detection.vulnerability.results")
                        .is_some_and(|s| s.starts_with("Package"))
            };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.results") {
                    gsub_field(
                        event,
                        "qualys_vmdr.asset_host_detection.vulnerability.results",
                        "qualys_vmdr.asset_host_detection.vulnerability.results",
                        cached_regex!("\\t"),
                        "||",
                    )?;
                }
            }

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.results") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String results = ctx.qualys_vmdr.asset_host_detection.vulnerability.results;\nif (results.startsWith(\"Package||\")) {\n  if (ctx.package == null) {\n    ctx.package = new HashMap();\n    ctx.qualys_vmdr.asset_host_detection.package_nested = new ArrayList();\n  }\n  if (ctx.package.name == null) {\n    ctx.package.name = new ArrayList();\n  }\n  if (ctx.package.version == null) {\n    ctx.package.version = new ArrayList();\n  }\n  if (ctx.package.fixed_version == null) {\n    ctx.package.fixed_version = new ArrayList();\n  }\n  def res = results.splitOnToken(\";;\");\n  for (int i=1; i < res.length; i++) {\n    def pkg_nest = [:];\n    def pkg = res[i].splitOnToken(\"||\");\n    if (pkg.length < 1) {\n      continue;\n    }\n    ctx.package.name.add(pkg[0]);\n    pkg_nest.name = pkg[0];\n    if (pkg.length < 2) {\n      continue;\n    }\n    ctx.package.version.add(pkg[1]);\n    pkg_nest.version = pkg[1];\n    if (pkg.length == 3) {\n      ctx.package.fixed_version.add(pkg[2]);\n      pkg_nest.fixed_version = pkg[2];\n    }\n    ctx.qualys_vmdr.asset_host_detection.package_nested.add(pkg_nest);\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String results = ctx.qualys_vmdr.asset_host_detection.vulnerability.results;\nif (results.startsWith(\"Package||\")) {\n  if (ctx.package == null) {\n    ctx.package = new HashMap();\n    ctx.qualys_vmdr.asset_host_detection.package_nested = new ArrayList();\n  }\n  if (ctx.package.name == null) {\n    ctx.package.name = new ArrayList();\n  }\n  if (ctx.package.version == null) {\n    ctx.package.version = new ArrayList();\n  }\n  if (ctx.package.fixed_version == null) {\n    ctx.package.fixed_version = new ArrayList();\n  }\n  def res = results.splitOnToken(\";;\");\n  for (int i=1; i < res.length; i++) {\n    def pkg_nest = [:];\n    def pkg = res[i].splitOnToken(\"||\");\n    if (pkg.length < 1) {\n      continue;\n    }\n    ctx.package.name.add(pkg[0]);\n    pkg_nest.name = pkg[0];\n    if (pkg.length < 2) {\n      continue;\n    }\n    ctx.package.version.add(pkg[1]);\n    pkg_nest.version = pkg[1];\n    if (pkg.length == 3) {\n      ctx.package.fixed_version.add(pkg[2]);\n      pkg_nest.fixed_version = pkg[2];\n    }\n    ctx.qualys_vmdr.asset_host_detection.package_nested.add(pkg_nest);\n  }\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_extract_package_fields",
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
                                .get("_ingest.pipeline")
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
                .get("package.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.package.name", v)?;
            }

            if let Some(v) = event
                .get("package.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.package.version", v)?;
            }

            if let Some(v) = event
                .get("package.fixed_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.package.fixed_version", v)?;
            }

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.STATUS") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.STATUS") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.STATUS",
                        "qualys_vmdr.asset_host_detection.vulnerability.status",
                    )?;
                }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.TYPE") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.TYPE") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.TYPE",
                        "qualys_vmdr.asset_host_detection.vulnerability.type",
                    )?;
                }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.QDS") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.QDS") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.QDS",
                        "qualys_vmdr.asset_host_detection.vulnerability.qds",
                    )?;
                }
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.qds") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.qds.#text") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.qds.#text",
                        "qualys_vmdr.asset_host_detection.vulnerability.qds.score",
                    )?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.qds.score") {
                    if let Some(val) =
                        event.get("qualys_vmdr.asset_host_detection.vulnerability.qds.score")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.qds.score"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "qualys_vmdr.asset_host_detection.vulnerability.qds.score",
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
                    "convert_qualys_vmdr_asset_host_detection_vulnerability_qds_score_1_to_long",
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
                            .get("_ingest.pipeline")
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

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.SSL") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.SSL") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.SSL",
                        "qualys_vmdr.asset_host_detection.vulnerability.ssl",
                    )?;
                }
            }

            let _cond = {
                event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.FIRST_FOUND_DATETIME",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.FIRST_FOUND_DATETIME",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.vulnerability.first_found_datetime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.FIRST_FOUND_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.FIRST_REOPENED_DATETIME",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.FIRST_REOPENED_DATETIME",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.vulnerability.first_reopened_datetime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.FIRST_REOPENED_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .has_value("qualys_vmdr.asset_host_detection.vulnerability.LAST_FOUND_DATETIME")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.LAST_FOUND_DATETIME",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.vulnerability.last_found_datetime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.LAST_FOUND_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string(
                    "qualys_vmdr.asset_host_detection.vulnerability.LAST_REOPENED_DATETIME",
                ) {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "qualys_vmdr.asset_host_detection.vulnerability.last_reopened_datetime",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.LAST_REOPENED_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string(
                    "qualys_vmdr.asset_host_detection.vulnerability.LAST_PROCESSED_DATETIME",
                ) {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.vulnerability.last_processed_datetime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.LAST_PROCESSED_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.LAST_TEST_DATETIME")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.LAST_TEST_DATETIME",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "qualys_vmdr.asset_host_detection.vulnerability.last_test_datetime",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.LAST_TEST_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.LAST_UPDATE_DATETIME",
                )
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.LAST_UPDATE_DATETIME",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.vulnerability.last_update_datetime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.LAST_UPDATE_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .has_value("qualys_vmdr.asset_host_detection.vulnerability.LAST_FIXED_DATETIME")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "qualys_vmdr.asset_host_detection.vulnerability.LAST_FIXED_DATETIME",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("qualys_vmdr.asset_host_detection.vulnerability.last_fixed_datetime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "qualys_vmdr.asset_host_detection.vulnerability.LAST_FIXED_DATETIME".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                    }
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.SEVERITY") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.SEVERITY") {
                        if let Some(val) =
                            event.get("qualys_vmdr.asset_host_detection.vulnerability.SEVERITY")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_vmdr.asset_host_detection.vulnerability.SEVERITY"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "qualys_vmdr.asset_host_detection.vulnerability.severity",
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
                        "convert_qualys_vmdr_asset_host_detection_vulnerability_SEVERITY_to_long_1",
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.IS_IGNORED") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.IS_IGNORED")
                    {
                        if let Some(val) =
                            event.get("qualys_vmdr.asset_host_detection.vulnerability.IS_IGNORED")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "qualys_vmdr.asset_host_detection.vulnerability.IS_IGNORED"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "qualys_vmdr.asset_host_detection.vulnerability.is_ignored",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_qualys_vmdr_asset_host_detection_vulnerability_IS_IGNORED_to_boolean")?;
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.IS_DISABLED") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.IS_DISABLED")
                    {
                        if let Some(val) =
                            event.get("qualys_vmdr.asset_host_detection.vulnerability.IS_DISABLED")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "qualys_vmdr.asset_host_detection.vulnerability.IS_DISABLED"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "qualys_vmdr.asset_host_detection.vulnerability.is_disabled",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_qualys_vmdr_asset_host_detection_vulnerability_IS_DISABLED_to_boolean")?;
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
                                .get("_ingest.pipeline")
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

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.TIMES_FOUND") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.TIMES_FOUND")
                    {
                        if let Some(val) =
                            event.get("qualys_vmdr.asset_host_detection.vulnerability.TIMES_FOUND")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path:
                                        "qualys_vmdr.asset_host_detection.vulnerability.TIMES_FOUND"
                                            .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "qualys_vmdr.asset_host_detection.vulnerability.times_found",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_qualys_vmdr_asset_host_detection_vulnerability_TIMES_FOUND_to_long_1")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.vulnerability.TIMES_REOPENED")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("qualys_vmdr.asset_host_detection.vulnerability.TIMES_REOPENED")
                    {
                        if let Some(val) = event
                            .get("qualys_vmdr.asset_host_detection.vulnerability.TIMES_REOPENED")
                        {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "qualys_vmdr.asset_host_detection.vulnerability.TIMES_REOPENED".into(),
                            message,
                        })?;
                            event.set(
                                "qualys_vmdr.asset_host_detection.vulnerability.times_reopened",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_qualys_vmdr_asset_host_detection_vulnerability_TIMES_REOPENED_to_long_1")?;
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.PORT") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.PORT") {
                        if let Some(val) =
                            event.get("qualys_vmdr.asset_host_detection.vulnerability.PORT")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "qualys_vmdr.asset_host_detection.vulnerability.PORT"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "qualys_vmdr.asset_host_detection.vulnerability.port",
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
                        "convert_qualys_vmdr_asset_host_detection_vulnerability_PORT_to_long_1",
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
                                .get("_ingest.pipeline")
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

            let _cond = {
                event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.QDS_FACTORS.QDS_FACTOR",
                )
            };
            if _cond {
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.vulnerability.QDS_FACTORS.QDS_FACTOR",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.QDS_FACTORS.QDS_FACTOR",
                        "qualys_vmdr.asset_host_detection.vulnerability.qds_factors",
                    )?;
                }
            }

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.qds_factors") };
            if _cond {
                if event.has_value("qualys_vmdr.asset_host_detection.vulnerability.qds_factors") {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.vulnerability.qds_factors",
                        |event| {
                            if event.has_value("_ingest._value.#text") {
                                event.rename("_ingest._value.#text", "_ingest._value.text")?;
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond =
                { event.has_value("qualys_vmdr.asset_host_detection.vulnerability.qds_factors") };
            if _cond {
                if event
                    .has_value("qualys_vmdr.asset_host_detection.vulnerability.qds_factors.#text")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.vulnerability.qds_factors.#text",
                        "qualys_vmdr.asset_host_detection.vulnerability.qds_factors.text",
                    )?;
                }
            }

            let _cond = {
                event.has_value("json.KNOWLEDGE_BASE") && event.get("json.KNOWLEDGE_BASE").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                // Begin nested pipeline: "pipeline_knowledge_base"
                if event.has_value("json.KNOWLEDGE_BASE") {
                    event.rename(
                        "json.KNOWLEDGE_BASE",
                        "qualys_vmdr.asset_host_detection.knowledge_base",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.ID_RANGE") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.ID_RANGE",
                        "qualys_vmdr.asset_host_detection.knowledge_base.id_range",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.ID") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.ID",
                        "qualys_vmdr.asset_host_detection.knowledge_base.ids",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CONSEQUENCE") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CONSEQUENCE",
                        "qualys_vmdr.asset_host_detection.knowledge_base.consequence.value",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CONSEQUENCE_COMMENT",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CONSEQUENCE_COMMENT",
                        "qualys_vmdr.asset_host_detection.knowledge_base.consequence.comment",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.DETECTION_INFO")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.DETECTION_INFO",
                        "qualys_vmdr.asset_host_detection.knowledge_base.detection_info",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.TYPE") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.TYPE", "qualys_vmdr.asset_host_detection.knowledge_base.compliance_list.type")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.SECTION") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.SECTION", "qualys_vmdr.asset_host_detection.knowledge_base.compliance_list.section")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.DESCRIPTION") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE.DESCRIPTION", "qualys_vmdr.asset_host_detection.knowledge_base.compliance_list.description")?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE",
                        |event| {
                            if event.has_value("_ingest._value.TYPE") {
                                event.rename("_ingest._value.TYPE", "_ingest._value.type")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE",
                        |event| {
                            if event.has_value("_ingest._value.SECTION") {
                                event.rename("_ingest._value.SECTION", "_ingest._value.section")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE",
                        |event| {
                            if event.has_value("_ingest._value.DESCRIPTION") {
                                event.rename(
                                    "_ingest._value.DESCRIPTION",
                                    "_ingest._value.description",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.COMPLIANCE_LIST.COMPLIANCE", "qualys_vmdr.asset_host_detection.knowledge_base.compliance_list")?;
                }
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CATEGORY") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CATEGORY",
                        "qualys_vmdr.asset_host_detection.knowledge_base.category",
                    )?;
                }
                let _cond =
                    { event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.category") };
                if _cond {
                    event.append_unique(
                        "vulnerability.category",
                        json!(
                            event
                                .get("qualys_vmdr.asset_host_detection.knowledge_base.category")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.DIAGNOSIS") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.DIAGNOSIS",
                        "qualys_vmdr.asset_host_detection.knowledge_base.diagnosis.value",
                    )?;
                }
                if let Some(v) = event
                    .get("qualys_vmdr.asset_host_detection.knowledge_base.diagnosis.value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.description", v)?;
                }
                if event
                    .has_value("qualys_vmdr.asset_host_detection.knowledge_base.DIAGNOSIS_COMMENT")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.DIAGNOSIS_COMMENT",
                        "qualys_vmdr.asset_host_detection.knowledge_base.diagnosis.comment",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.PCI_REASONS.PCI_REASON",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.PCI_REASONS.PCI_REASON",
                        "qualys_vmdr.asset_host_detection.knowledge_base.pci_reasons.value",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.QID") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.QID",
                        "qualys_vmdr.asset_host_detection.knowledge_base.qid",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SOLUTION") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.SOLUTION",
                        "qualys_vmdr.asset_host_detection.knowledge_base.solution.value",
                    )?;
                }
                if event
                    .has_value("qualys_vmdr.asset_host_detection.knowledge_base.SOLUTION_COMMENT")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.SOLUTION_COMMENT",
                        "qualys_vmdr.asset_host_detection.knowledge_base.solution.comment",
                    )?;
                }
                if event
                    .has_value("qualys_vmdr.asset_host_detection.knowledge_base.SUPPORTED_MODULES")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.SUPPORTED_MODULES",
                        "qualys_vmdr.asset_host_detection.knowledge_base.supported_modules",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.TITLE") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.TITLE",
                        "qualys_vmdr.asset_host_detection.knowledge_base.title",
                    )?;
                }
                if let Some(v) = event
                    .get("qualys_vmdr.asset_host_detection.knowledge_base.title")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.title", v)?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ.ID",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ.ID",
                        "qualys_vmdr.asset_host_detection.knowledge_base.bugtraq_list.id",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ.URL",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ.URL",
                        "qualys_vmdr.asset_host_detection.knowledge_base.bugtraq_list.url",
                    )?;
                }
                let _cond = {
                    event
                        .get("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ",
                        |event| {
                            if event.has_value("_ingest._value.ID") {
                                event.rename("_ingest._value.ID", "_ingest._value.id")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event
                        .get("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ",
                        |event| {
                            if event.has_value("_ingest._value.URL") {
                                event.rename("_ingest._value.URL", "_ingest._value.url")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event
                        .get("qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if event.has_value(
                        "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ",
                    ) {
                        event.rename(
                            "qualys_vmdr.asset_host_detection.knowledge_base.BUGTRAQ_LIST.BUGTRAQ",
                            "qualys_vmdr.asset_host_detection.knowledge_base.bugtraq_list",
                        )?;
                    }
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.VULN_TYPE") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.VULN_TYPE",
                        "qualys_vmdr.asset_host_detection.knowledge_base.vuln_type",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVE_LIST") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVE_LIST",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cve_list",
                    )?;
                }
                let _cond = {
                    event
                        .get("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE")
                    {
                        event.rename(
                            "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE",
                            "qualys_vmdr.asset_host_detection.knowledge_base.cvss.base",
                        )?;
                    }
                }
                // SKIPPED: condition not transpiled: ctx.qualys_vmdr?.asset_host_detection?.knowledge_base?.CVSS?.BASE instanceof Object
                #[allow(unreachable_code, unused_variables)]
                if false {
                    if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE")
                    {
                        event.rename(
                            "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.BASE",
                            "qualys_vmdr.asset_host_detection.knowledge_base.cvss.base_obj",
                        )?;
                    }
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.TEMPORAL")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.TEMPORAL",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.temporal",
                    )?;
                }
                if event
                    .has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.VECTOR_STRING")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.VECTOR_STRING",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.vector_string",
                    )?;
                }
                if event
                    .has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.ACCESS.VECTOR")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.ACCESS.VECTOR",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.access.vector",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.ACCESS.COMPLEXITY",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.ACCESS.COMPLEXITY",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.access.complexity",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.CONFIDENTIALITY",
                ) {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.CONFIDENTIALITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss.impact.confidentiality")?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.INTEGRITY",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.INTEGRITY",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.impact.integrity",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.AVAILABILITY",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.IMPACT.AVAILABILITY",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.impact.availability",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.AUTHENTICATION",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.AUTHENTICATION",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.authentication",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.EXPLOITABILITY",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.EXPLOITABILITY",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.exploitability",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.REMEDIATION_LEVEL",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.REMEDIATION_LEVEL",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.remediation_level",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.REPORT_CONFIDENCE",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS.REPORT_CONFIDENCE",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.report_confidence",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.BASE") {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.BASE",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.base",
                    )?;
                }
                if event
                    .has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.TEMPORAL")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.TEMPORAL",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.temporal",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.VECTOR_STRING",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.VECTOR_STRING",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.vector_string",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.CVSS3_VERSION",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.CVSS3_VERSION",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.version",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.ATTACK.VECTOR",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.ATTACK.VECTOR",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.attack.vector",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.ATTACK.COMPLEXITY",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.ATTACK.COMPLEXITY",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.attack.complexity",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.CONFIDENTIALITY") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.CONFIDENTIALITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.impact.confidentiality")?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.INTEGRITY",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.INTEGRITY",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.impact.integrity",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.AVAILABILITY",
                ) {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.IMPACT.AVAILABILITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.impact.availability")?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.PRIVILEGES_REQUIRED",
                ) {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.PRIVILEGES_REQUIRED", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.privileges_required")?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.USER_INTERACTION",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.USER_INTERACTION",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.user_interaction",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.SCOPE")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.SCOPE",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.scope",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.EXPLOIT_CODE_MATURITY",
                ) {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.EXPLOIT_CODE_MATURITY", "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.exploit_code_maturity")?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.REMEDIATION_LEVEL",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.REMEDIATION_LEVEL",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.remediation_level",
                    )?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.REPORT_CONFIDENCE",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.CVSS_V3.REPORT_CONFIDENCE",
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.report_confidence",
                    )?;
                }
                if event
                    .has_value("qualys_vmdr.asset_host_detection.knowledge_base.AUTOMATIC_PCI_FAIL")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.AUTOMATIC_PCI_FAIL",
                        "qualys_vmdr.asset_host_detection.knowledge_base.automatic_pci_fail",
                    )?;
                }
                if let Some(v) = event
                    .get("qualys_vmdr.asset_host_detection.knowledge_base.cve_list")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.id", v)?;
                }
                let _cond = {
                    event.has_value("vulnerability.id")
                        && !(event.get("vulnerability.id").is_some_and(|v| v.is_array()))
                };
                if _cond {
                    let v = json!(format!(
                        "https://cve.mitre.org/cgi-bin/cvename.cgi?name={}",
                        event
                            .get("vulnerability.id")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("vulnerability.reference", v)?;
                    }
                }
                let _cond = { event.get("vulnerability.id").is_some_and(|v| v.is_array()) };
                if _cond {
                    foreach_array(event, "vulnerability.id", |event| {
                        event.append_unique(
                            "vulnerability.reference",
                            json!(format!(
                                "https://cve.mitre.org/cgi-bin/cvename.cgi?name={}",
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        Ok(())
                    })?;
                }
                let v = json!("CVE");
                if !painless_is_empty_value(&v) {
                    event.set("vulnerability.enumeration", v)?;
                }
                let _cond = {
                    event.has_value("vulnerability.reference")
                        && !(event
                            .get("vulnerability.reference")
                            .is_some_and(|v| v.is_array()))
                };
                if _cond {
                    uri_parts(event, "vulnerability.reference", "url", true, false)?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.DESC") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.DESC", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src.list.explt.desc")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.LINK") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.LINK", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src.list.explt.link")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.REF") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.EXPLT_LIST.EXPLT.REF", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src.list.explt.ref")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.SRC_NAME") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC.SRC_NAME", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src.name")?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC",
                        |event| {
                            if event.has_value("_ingest._value.SRC_NAME") {
                                event.rename("_ingest._value.SRC_NAME", "_ingest._value.name")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(event, "_ingest._value.EXPLT_LIST.EXPLT", |event| {
                                    if event.has_value("_ingest._value.DESC") {
                                        event
                                            .rename("_ingest._value.DESC", "_ingest._value.desc")?;
                                    }
                                    Ok(())
                                })?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(event, "_ingest._value.EXPLT_LIST.EXPLT", |event| {
                                    if event.has_value("_ingest._value.LINK") {
                                        event
                                            .rename("_ingest._value.LINK", "_ingest._value.link")?;
                                    }
                                    Ok(())
                                })?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC",
                        |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                foreach_array(event, "_ingest._value.EXPLT_LIST.EXPLT", |event| {
                                    if event.has_value("_ingest._value.REF") {
                                        event.rename("_ingest._value.REF", "_ingest._value.ref")?;
                                    }
                                    Ok(())
                                })?;
                                Ok(())
                            })();
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC",
                        |event| {
                            if event.has_value("_ingest._value.EXPLT_LIST.EXPLT") {
                                event.rename(
                                    "_ingest._value.EXPLT_LIST.EXPLT",
                                    "_ingest._value.list.explt",
                                )?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC").is_some_and(|v| v.is_array())
                };
                if _cond {
                    if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.EXPLOITS.EXPLT_SRC", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.exploits.explt_src")?;
                }
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ID") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ID", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.id")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.SRC_NAME") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.SRC_NAME", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.name")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_TYPE") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_TYPE", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.type")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_PLATFORM") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_PLATFORM", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.platform")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ALIAS") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_ALIAS", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.alias")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_RATING") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_RATING", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.rating")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_LINK") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CORRELATION.MALWARE.MW_SRC.MW_LIST.MW_INFO.MW_LINK", "qualys_vmdr.asset_host_detection.knowledge_base.correlation.malware.src.list.info.link")?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.ADDITIONAL_INFO",
                ) {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.ADDITIONAL_INFO",
                        "qualys_vmdr.asset_host_detection.knowledge_base.discovery.additional_info",
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.AUTH_TYPE_LIST.AUTH_TYPE") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.AUTH_TYPE_LIST.AUTH_TYPE", "qualys_vmdr.asset_host_detection.knowledge_base.discovery.auth_type_list.value")?;
                }
                let _cond = {
                    event
                        .get_str("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE")
                        != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value(
                            "qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE",
                        ) {
                            if let Some(val) = event.get(
                                "qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE",
                            ) {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                path: "qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE".into(),
                message,
                }
                                })?;
                                event.set("qualys_vmdr.asset_host_detection.knowledge_base.discovery.remote", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_DISCOVERY_REMOTE_to_long",
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
                                    .get("_ingest.pipeline")
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
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE.PRODUCT") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE.PRODUCT", "qualys_vmdr.asset_host_detection.knowledge_base.software_list.product")?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE.VENDOR",
                ) {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE.VENDOR", "qualys_vmdr.asset_host_detection.knowledge_base.software_list.vendor")?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE",
                        |event| {
                            if event.has_value("_ingest._value.PRODUCT") {
                                event.rename("_ingest._value.PRODUCT", "_ingest._value.product")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE",
                        |event| {
                            if event.has_value("_ingest._value.VENDOR") {
                                event.rename("_ingest._value.VENDOR", "_ingest._value.vendor")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    if event.has_value(
                        "qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE",
                    ) {
                        event.rename("qualys_vmdr.asset_host_detection.knowledge_base.SOFTWARE_LIST.SOFTWARE", "qualys_vmdr.asset_host_detection.knowledge_base.software_list")?;
                    }
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.ID") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.ID", "qualys_vmdr.asset_host_detection.knowledge_base.vendor_reference_list.id")?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.URL") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE.URL", "qualys_vmdr.asset_host_detection.knowledge_base.vendor_reference_list.url")?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE",
                        |event| {
                            if event.has_value("_ingest._value.ID") {
                                event.rename("_ingest._value.ID", "_ingest._value.id")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE",
                        |event| {
                            if event.has_value("_ingest._value.URL") {
                                event.rename("_ingest._value.URL", "_ingest._value.url")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE").is_some_and(|v| v.is_array())
                };
                if _cond {
                    if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.VENDOR_REFERENCE_LIST.VENDOR_REFERENCE", "qualys_vmdr.asset_host_detection.knowledge_base.vendor_reference_list")?;
                }
                }
                let _cond = {
                    event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME") && event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.last.service_modification_datetime", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_LAST_SERVICE_MODIFICATION_DATETIME",
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
                                    .get("_ingest.pipeline")
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
                let _cond = {
                    event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.DATETIME") && event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.DATETIME") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.DATETIME") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.last.customization.datetime", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.DATETIME".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_LAST_CUSTOMIZATION_DATETIME",
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
                                    .get("_ingest.pipeline")
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
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.USER_LOGIN",
                ) {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.LAST_CUSTOMIZATION.USER_LOGIN", "qualys_vmdr.asset_host_detection.knowledge_base.last.customization.user_login")?;
                }
                let _cond = {
                    event.has_value(
                        "qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME",
                    ) && event.get_str(
                        "qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME",
                    ) != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME",
                        ) {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.published_datetime", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_PUBLISHED_DATETIME",
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
                                    .get("_ingest.pipeline")
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
                let _cond = {
                    event.has_value(
                        "qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE",
                    ) && event.get_str(
                        "qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE",
                    ) != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string(
                            "qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE",
                        ) {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.patch_published_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_PATCH_PUBLISHED_DATE",
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
                                    .get("_ingest.pipeline")
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
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.CHANGE_LOG_LIST.CHANGE_LOG_INFO") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.CHANGE_LOG_LIST.CHANGE_LOG_INFO", "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info")?;
                }
                if event.has_value(
                    "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.COMMENTS",
                ) {
                    event.rename("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.COMMENTS", "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.comments")?;
                }
                let _cond = {
                    (!(event.get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info").is_some_and(|v| v.is_array()))) && event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE") && event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                Some(parsed) => event.set("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.change_date", parsed)?,
                None => {
                return Err(TransformError::ParseError {
                path: "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE".into(),
                message: format!("unable to parse date [{date_str}]"),
                });
                }
                }
                }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_qualys_vmdr_knowledge_base_changelog_list_info_CHANGE_DATE_1",
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
                                    .get("_ingest.pipeline")
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
                let _cond = {
                    event
                        .get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info",
                        |event| {
                            if event.has_value("_ingest._value.COMMENTS") {
                                event
                                    .rename("_ingest._value.COMMENTS", "_ingest._value.comments")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event
                        .get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.CHANGE_DATE")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.change_date", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.CHANGE_DATE".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                        }
                    }
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.THREAT_INTELLIGENCE.THREAT_INTEL") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.THREAT_INTELLIGENCE.THREAT_INTEL", "qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel")?;
                }
                let _cond = {
                    event.get("qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel").is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel",
                        |event| {
                            if event.has_value("_ingest._value.#text") {
                                event.rename("_ingest._value.#text", "_ingest._value.text")?;
                            }
                            Ok(())
                        },
                    )?;
                }
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel.#text") {
                event.rename("qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel.#text", "qualys_vmdr.asset_host_detection.knowledge_base.threat_intelligence.intel.text")?;
                }
                let _cond = {
                    event
                        .get("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(
                        event,
                        "qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info",
                        |event| {
                            event.remove("_ingest._value.CHANGE_DATE");
                            Ok(())
                        },
                    )?;
                }
                let _cond = {
                    event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED")
                        == Some("1")
                };
                if _cond {
                    event.set(
                        "qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED",
                        json!(true),
                    )?;
                }
                let _cond = {
                    event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED")
                        == Some("0")
                };
                if _cond {
                    event.set(
                        "qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED",
                        json!(false),
                    )?;
                }
                let _cond = {
                    event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED")
                        != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value(
                            "qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED",
                        ) {
                            if let Some(val) = event
                                .get("qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                path: "qualys_vmdr.asset_host_detection.knowledge_base.IS_DISABLED".into(),
                message,
                }
                                    })?;
                                event.set(
                                    "qualys_vmdr.asset_host_detection.knowledge_base.is_disabled",
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
                            "convert_IS_DISABLED_to_boolean",
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
                                    .get("_ingest.pipeline")
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
                let _cond = {
                    event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE")
                        == Some("1")
                };
                if _cond {
                    event.set(
                        "qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE",
                        json!(true),
                    )?;
                }
                let _cond = {
                    event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE")
                        == Some("0")
                };
                if _cond {
                    event.set(
                        "qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE",
                        json!(false),
                    )?;
                }
                let _cond = {
                    event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE")
                        != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event
                            .has_value("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE")
                        {
                            if let Some(val) = event
                                .get("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                path: "qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE".into(),
                message,
                }
                                    })?;
                                event.set(
                                    "qualys_vmdr.asset_host_detection.knowledge_base.patchable",
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
                            "convert_PATCHABLE_to_boolean",
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
                                    .get("_ingest.pipeline")
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
                let _cond = {
                    event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG")
                        == Some("1")
                };
                if _cond {
                    event.set(
                        "qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG",
                        json!(true),
                    )?;
                }
                let _cond = {
                    event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG")
                        == Some("0")
                };
                if _cond {
                    event.set(
                        "qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG",
                        json!(false),
                    )?;
                }
                let _cond = {
                    event.get_str("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG")
                        != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event
                            .has_value("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG")
                        {
                            if let Some(val) = event
                                .get("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                path: "qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG".into(),
                message,
                }
                                    })?;
                                event.set(
                                    "qualys_vmdr.asset_host_detection.knowledge_base.pci_flag",
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
                            "convert_PCI_FLAG_to_boolean",
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
                                    .get("_ingest.pipeline")
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (!(ctx.qualys_vmdr?.asset_host_detection?.knowledge_base?.SEVERITY_LEVEL instanceof String)) {\n  return;\n} def vuln_type = ctx.qualys_vmdr?.asset_host_detection?.knowledge_base?.vuln_type; if (!(vuln_type instanceof String)) {\n  return;\n} String level = ctx.qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL; if (params.vuln_types.contains(vuln_type)) {\n  ctx.qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL = params.vuln_level.getOrDefault(level, params.vuln_level[\"0\"]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (!(ctx.qualys_vmdr?.asset_host_detection?.knowledge_base?.SEVERITY_LEVEL instanceof String)) {\n  return;\n} def vuln_type = ctx.qualys_vmdr?.asset_host_detection?.knowledge_base?.vuln_type; if (!(vuln_type instanceof String)) {\n  return;\n} String level = ctx.qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL; if (params.vuln_types.contains(vuln_type)) {\n  ctx.qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL = params.vuln_level.getOrDefault(level, params.vuln_level[\"0\"]);\n}"#
                        ),
                        cached_params!(
                            "{\"vuln_level\":{\"0\":\"None\",\"1\":\"Minimal\",\"2\":\"Medium\",\"3\":\"Serious\",\"4\":\"Critical\",\"5\":\"Urgent\"},\"vuln_types\":[\"Potential Vulnerability\",\"Vulnerability\",\"Vulnerability or Potential Vulnerability\",\"Information Gathered\"]}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_SEVERITY_LEVEL",
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
                                .get("_ingest.pipeline")
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
                if event.has_value("qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL")
                {
                    event.rename(
                        "qualys_vmdr.asset_host_detection.knowledge_base.SEVERITY_LEVEL",
                        "qualys_vmdr.asset_host_detection.knowledge_base.severity_level",
                    )?;
                }
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.changelog_list.info.CHANGE_DATE");
                event.remove(
                    "qualys_vmdr.asset_host_detection.knowledge_base.CODE_MODIFIED_DATETIME",
                );
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.PCI_FLAG");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.PATCHABLE");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.LAST_SERVICE_MODIFICATION_DATETIME");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.DISCOVERY.REMOTE");
                event.remove("qualys_vmdr.asset_host_detection.knowledge_base.PUBLISHED_DATETIME");
                event
                    .remove("qualys_vmdr.asset_host_detection.knowledge_base.PATCH_PUBLISHED_DATE");
                event.remove("ID_RANGE");
                event.remove("ID");
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
                    event.remove("qualys_vmdr.asset_host_detection.knowledge_base.category");
                    event.remove("qualys_vmdr.asset_host_detection.knowledge_base.cve_list");
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_remove_null_values",
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
                                .get("_ingest.pipeline")
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
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.set("event.kind", json!("pipeline_error"))?;
                }
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.append_unique("tags", json!("preserve_original_event"))?;
                }
                // End nested pipeline: "pipeline_knowledge_base"
            }

            let _cond = { event.has_value("qualys_vmdr.asset_host_detection.knowledge_base") };
            if _cond {
                event.set(
                    "qualys_vmdr.asset_host_detection.knowledge_base.status",
                    json!("found"),
                )?;
            }

            let _cond = { !event.has_value("qualys_vmdr.asset_host_detection.knowledge_base") };
            if _cond {
                event.set(
                    "qualys_vmdr.asset_host_detection.knowledge_base.status",
                    json!("not_found"),
                )?;
            }

            if let Some(v) = event
                .get("qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.base")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.base", v)?;
            }

            let _cond = { event.has_value("vulnerability.score.base") };
            if _cond {
                if let Some(v) = event
                    .get("qualys_vmdr.asset_host_detection.knowledge_base.cvss_v3.version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.score.version", v)?;
                }
            }

            let _cond = { !event.has_value("vulnerability.score.base") };
            if _cond {
                if let Some(v) = event
                    .get("qualys_vmdr.asset_host_detection.knowledge_base.cvss.base")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.score.base", v)?;
                }
            }

            let _cond = { !event.has_value("vulnerability.score.base") };
            if _cond {
                if let Some(v) = event
                    .get("qualys_vmdr.asset_host_detection.knowledge_base.cvss.base_obj.#text")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("vulnerability.score.base", v)?;
                }
            }

            let _cond = { !event.has_value("vulnerability.score.version") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value(
                        "qualys_vmdr.asset_host_detection.knowledge_base.cvss.vector_string",
                    ) {
                        if let Some(input) = event.get_string(
                            "qualys_vmdr.asset_host_detection.knowledge_base.cvss.vector_string",
                        ) {
                            // Grok pattern: ^CVSS:%{DATA:vulnerability.score.version}/%{GREEDYDATA}$
                            // Grok pattern: ^%{GREEDYDATA}$
                            let _ = extract_first_match(
                                &[
                                    cached_grok!(
                                        "^CVSS:%{DATA:vulnerability.score.version}/%{GREEDYDATA}$"
                                    ),
                                    cached_grok!("^%{GREEDYDATA}$"),
                                ],
                                &input,
                                event,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "grok_to_extract_vulnerability_score_version",
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

            let _cond = { event.get_str("vulnerability.score.base") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("vulnerability.score.base") {
                        if let Some(val) = event.get("vulnerability.score.base") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "vulnerability.score.base".into(),
                                    message,
                                }
                            })?;
                            event.set("vulnerability.score.base", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_vulnerability_score_base_to_float",
                    )?;
                    if event.remove("vulnerability.score.base").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "vulnerability.score.base".into(),
                        });
                    }
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
                                .get("_ingest.pipeline")
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

            let _cond = { event.has_value("vulnerability.score.base") };
            if _cond {
                event.set("vulnerability.classification", json!("CVSS"))?;
            }

            let _cond = { event.has_value("vulnerability.score.base") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: // CVSS score between 9.0 and 10.0)\nif (9.0 <= ctx.vulnerability.score.base) {\n  ctx.vulnerability.severity = \"Critical\";\n}\n// CVSS score between 7.0 and 8.9\nelse if (7.0 <= ctx.vulnerability.score.base) {\n  ctx.vulnerability.severity = \"High\";\n}\n// CVSS score between 4.0 and 6.9\nelse if (4.0 <= ctx.vulnerability.score.base) {\n  ctx.vulnerability.severity = \"Medium\";\n}\n// CVSS score between 0.1 and 3.9\nelse if (0.1 <= ctx.vulnerability.score.base) {\n  ctx.vulnerability.severity = \"Low\";\n}\nelse if (ctx.vulnerability.score.base == 0) {\n  ctx.vulnerability.severity = \"None\";\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"// CVSS score between 9.0 and 10.0)\nif (9.0 <= ctx.vulnerability.score.base) {\n  ctx.vulnerability.severity = \"Critical\";\n}\n// CVSS score between 7.0 and 8.9\nelse if (7.0 <= ctx.vulnerability.score.base) {\n  ctx.vulnerability.severity = \"High\";\n}\n// CVSS score between 4.0 and 6.9\nelse if (4.0 <= ctx.vulnerability.score.base) {\n  ctx.vulnerability.severity = \"Medium\";\n}\n// CVSS score between 0.1 and 3.9\nelse if (0.1 <= ctx.vulnerability.score.base) {\n  ctx.vulnerability.severity = \"Low\";\n}\nelse if (ctx.vulnerability.score.base == 0) {\n  ctx.vulnerability.severity = \"None\";\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_assign_vulnerability_severity",
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
                                .get("_ingest.pipeline")
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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        event.rename("message", "event.original")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = {
                event.has_value("event.original")
                    && event.get_bool("_conf.enable_deduplication") == Some(true)
            };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.original") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "event.original".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            let _cond = {
                event.has_value("qualys_vmdr.asset_host_detection.last_scan_datetime")
                    && event.has_value("_id")
                    && event.get_bool("_conf.enable_deduplication") == Some(true)
            };
            if _cond {
                event.set(
                    "_id",
                    json!(format!(
                        "{}-{}",
                        event
                            .get("qualys_vmdr.asset_host_detection.last_scan_datetime")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            event.remove("json");
            event.remove("message");
            event.remove("interval_start");
            event.remove("_conf");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.FIRST_FOUND_DATETIME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.FIRST_REOPENED_DATETIME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.IS_DISABLED");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.LAST_FOUND_DATETIME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.LAST_REOPENED_DATETIME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.LAST_PROCESSED_DATETIME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.LAST_TEST_DATETIME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.LAST_UPDATE_DATETIME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.LAST_FIXED_DATETIME");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.PORT");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.QDS_FACTORS");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.SEVERITY");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.TIMES_FOUND");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.TIMES_REOPENED");
            event.remove("qualys_vmdr.asset_host_detection.vulnerability.IS_IGNORED");
            event.remove(
                "qualys_vmdr.asset_host_detection.cloud_provider_tags.cloud_tag.LAST_SUCCESS_DATE",
            );
            event.remove("qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_ERROR_DATE");
            event.remove(
                "qualys_vmdr.asset_host_detection.metadata.ec2.attribute.LAST_SUCCESS_DATE",
            );
            event.remove(
                "qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_ERROR_DATE",
            );
            event.remove(
                "qualys_vmdr.asset_host_detection.metadata.google.attribute.LAST_SUCCESS_DATE",
            );
            event.remove(
                "qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_ERROR_DATE",
            );
            event.remove(
                "qualys_vmdr.asset_host_detection.metadata.azure.attribute.LAST_SUCCESS_DATE",
            );
            event.remove(
                "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_ERROR_DATE",
            );
            event.remove(
                "qualys_vmdr.asset_host_detection.metadata.alicloud.attribute.LAST_SUCCESS_DATE",
            );

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
                event.remove("qualys_vmdr.asset_host_detection.netbios");
                event.remove("qualys_vmdr.asset_host_detection.ip");
                event.remove("qualys_vmdr.asset_host_detection.id");
                event.remove("qualys_vmdr.asset_host_detection.os");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_remove_null_values",
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
                            .get("_ingest.pipeline")
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

            // Painless script
            // Source: def filterMassive(def src) {\n  if (src instanceof Map) {\n    for (def entry: src.entrySet()) {\n      entry.setValue(filterMassive(entry.getValue()));\n    }\n    return src;\n  } else if (src instanceof List) {\n    for (int i = 0; i < src.length; i++) {\n      src[i] = filterMassive(src[i]);\n    }\n    return src;\n  } else if (src instanceof String && src.length() > 32766) {\n    return src.substring(0, 32700)+' (truncated)';\n  }\n  return src;\n}\nfilterMassive(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def filterMassive(def src) {\n  if (src instanceof Map) {\n    for (def entry: src.entrySet()) {\n      entry.setValue(filterMassive(entry.getValue()));\n    }\n    return src;\n  } else if (src instanceof List) {\n    for (int i = 0; i < src.length; i++) {\n      src[i] = filterMassive(src[i]);\n    }\n    return src;\n  } else if (src instanceof String && src.length() > 32766) {\n    return src.substring(0, 32700)+' (truncated)';\n  }\n  return src;\n}\nfilterMassive(ctx);"#
                ),
            )?;

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
                            .get("_ingest.pipeline")
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
