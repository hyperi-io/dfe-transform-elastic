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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "resp")?;
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
            }

            let _cond = { event.get_str("input.type") == Some("http_endpoint") };
            if _cond {
                event.remove("json");
            }

            if event.has("resp.event") {
                event.rename("resp.event", "json")?;
            }

            let _cond =
                { event.has_value("json") && event.get_bool("_conf.strict_fields") == Some(true) };
            if _cond {
                // Painless script
                // Source: if (ctx.resp?.version == null) {\n  def fields = [];\n  for (e in ctx.json.entrySet()) {\n    fields.add(e.getKey());\n  }\n  Collections.sort(fields);\n  String signature = String.join(\"|\", fields);\n  if (signature != params.expect.fields) {\n    ctx.error = ctx.error ?: [:];\n    ctx.error.message = ctx.error.message ?: [];\n    ctx.error.message.add(\"field set mismatch: \"+signature+\" is not expected set of templated fields (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n  }\n} else if (ctx.resp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add(\"template version mismatch: \"+ctx.resp.version.toString()+\" is not expected version (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.resp?.version == null) {\n  def fields = [];\n  for (e in ctx.json.entrySet()) {\n    fields.add(e.getKey());\n  }\n  Collections.sort(fields);\n  String signature = String.join(\"|\", fields);\n  if (signature != params.expect.fields) {\n    ctx.error = ctx.error ?: [:];\n    ctx.error.message = ctx.error.message ?: [];\n    ctx.error.message.add(\"field set mismatch: \"+signature+\" is not expected set of templated fields (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n  }\n} else if (ctx.resp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add(\"template version mismatch: \"+ctx.resp.version.toString()+\" is not expected version (see \"+params.data_stream+\" https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n}"#
                    ),
                    cached_params!(
                        "{\"data_stream\":\"dns-log\",\"expect\":{\"fields\":\"category|cloudname|clt_sip|company|datacenter|datacentercity|datacentercountry|datetime|day|day_of_month|department|dept|deviceappversion|devicehostname|devicemodel|devicename|deviceostype|deviceosversion|deviceowner|devicetype|dns_gateway_rule|dns_gateway_server_protocol|dns_gateway_status|dns_req|dns_reqtype|dns_resp|dnsapp|dnsappcat|durationms|ecs_prefix|ecs_slot|ednsreq|eedone|epochtime|error|hour|http_code|istcp|loc|location|login|minutes|month|month_of_year|oclientsourceip|odevicehostname|odevicename|odeviceowner|odomcat|protocol|recordid|reqaction|reqrulelabel|resaction|respipcat|respipcategory|resrulelabel|restype|second|srv_dip|srv_dport|tz|user|year\",\"version\":\"v3\"},\"pkg_version\":\"3.17.0\"}"
                    ),
                )?;
            }

            event.remove("resp");

            event.set("network.protocol", json!("dns"))?;

            event.append("event.category", json!("network"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);"#
                ),
            )?;

            let _cond = { event.get_str("json.clt_sip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.clt_sip") {
                        if let Some(val) = event.get("json.clt_sip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.clt_sip".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.client.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_clt_sip_to_ip")?;
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
                .get("zscaler_zia.dns.client.ip")
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

            if event.has("json.cloudname") {
                event.rename("json.cloudname", "zscaler_zia.dns.cloud.name")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.cloud.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if event.has("json.company") {
                event.rename("json.company", "zscaler_zia.dns.company")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.company")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.name", v)?;
            }

            if event.has("json.datacentercity") {
                event.rename("json.datacentercity", "zscaler_zia.dns.datacenter.city")?;
            }

            if event.has("json.datacentercountry") {
                event.rename(
                    "json.datacentercountry",
                    "zscaler_zia.dns.datacenter.country",
                )?;
            }

            if event.has("json.datacenter") {
                event.rename("json.datacenter", "zscaler_zia.dns.datacenter.name")?;
            }

            if event.has("json.day") {
                event.rename("json.day", "zscaler_zia.dns.day")?;
            }

            let _cond = { event.get_str("json.day_of_month") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.day_of_month") {
                        if let Some(val) = event.get("json.day_of_month") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.day_of_month".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.day_of_month", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_day_of_month_to_long",
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

            if event.has("json.dept") {
                event.rename("json.dept", "zscaler_zia.dns.dept")?;
            }

            if event.has("json.deviceappversion") {
                event.rename("json.deviceappversion", "zscaler_zia.dns.device.appversion")?;
            }

            if event.has("json.devicehostname") {
                event.rename("json.devicehostname", "zscaler_zia.dns.device.hostname")?;
            }

            let _cond = { event.get_str("zscaler_zia.dns.device.hostname") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("zscaler_zia.dns.device.hostname") {
                        map_strings(
                            event,
                            "zscaler_zia.dns.device.hostname",
                            "host.name",
                            str::to_lowercase,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "lowercase")?;
                    event.set("_ingest.on_failure_processor_tag", "lowercase_host_name")?;
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

            if event.has("json.devicemodel") {
                event.rename("json.devicemodel", "zscaler_zia.dns.device.model")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.device.model")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.model.name", v)?;
            }

            if event.has("json.devicename") {
                event.rename("json.devicename", "zscaler_zia.dns.device.name")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.device.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.deviceostype") {
                event.rename("json.deviceostype", "zscaler_zia.dns.device.os.type")?;
            }

            if event.has("json.deviceosversion") {
                event.rename("json.deviceosversion", "zscaler_zia.dns.device.os.version")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.device.os.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if event.has("json.deviceowner") {
                event.rename("json.deviceowner", "zscaler_zia.dns.device.owner")?;
            }

            let _cond = { event.has_value("zscaler_zia.dns.device.owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.dns.device.owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.devicetype") {
                event.rename("json.devicetype", "zscaler_zia.dns.device.type")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.device.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.type", v)?;
            }

            if event.has("json.dnsappcat") {
                event.rename("json.dnsappcat", "zscaler_zia.dns.dns.category")?;
            }

            if event.has("json.dns_gateway_rule") {
                event.rename("json.dns_gateway_rule", "zscaler_zia.dns.dns.gateway.rule")?;
            }

            if event.has("json.dns_gateway_server_protocol") {
                event.rename(
                    "json.dns_gateway_server_protocol",
                    "zscaler_zia.dns.dns.gateway.server_protocol",
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.dns.dns.gateway.server_protocol") };
            if _cond {
                event.append_unique(
                    "network.transport",
                    json!(
                        event
                            .get("zscaler_zia.dns.dns.gateway.server_protocol")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.dns_gateway_status") {
                event.rename(
                    "json.dns_gateway_status",
                    "zscaler_zia.dns.dns.gateway.status",
                )?;
            }

            if event.has("json.dnsapp") {
                event.rename("json.dnsapp", "zscaler_zia.dns.dns.type")?;
            }

            let _cond = { event.get_str("zscaler_zia.dns.dns.type") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("zscaler_zia.dns.dns.type") {
                        map_strings(
                            event,
                            "zscaler_zia.dns.dns.type",
                            "network.application",
                            str::to_lowercase,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "lowercase")?;
                    event.set("_ingest.on_failure_processor_tag", "lowercase_dns_type")?;
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

            if event.has("json.category") {
                event.rename("json.category", "zscaler_zia.dns.dom.category")?;
            }

            let _cond = { event.get_str("json.durationms") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.durationms") {
                        if let Some(val) = event.get("json.durationms") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.durationms".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.duration.milliseconds", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_duration_milliseconds_to_long",
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

            let _cond = { event.has_value("zscaler_zia.dns.duration.milliseconds") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.event == null) {\n  ctx.put('event', new HashMap());\n} ctx.event.duration = ctx.zscaler_zia.dns.duration.milliseconds * 1000000;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.event == null) {\n  ctx.put('event', new HashMap());\n} ctx.event.duration = ctx.zscaler_zia.dns.duration.milliseconds * 1000000;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_duration_ms_to_ns",
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

            if event.has("json.ecs_prefix") {
                event.rename("json.ecs_prefix", "zscaler_zia.dns.ecs.prefix")?;
            }

            if event.has("json.ecs_slot") {
                event.rename("json.ecs_slot", "zscaler_zia.dns.ecs.slot")?;
            }

            if event.has("json.ednsreq") {
                event.rename("json.ednsreq", "zscaler_zia.dns.ednsreq")?;
            }

            if event.has("json.eedone") {
                event.rename("json.eedone", "zscaler_zia.dns.eedone")?;
            }

            let _cond = {
                event.has_value("json.epochtime") && event.get_str("json.epochtime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.epochtime") {
                        if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], None, None) {
                            event.set("zscaler_zia.dns.epochtime", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_epochtime")?;
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

            if event.has("json.error") {
                event.rename("json.error", "zscaler_zia.dns.error")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.error")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.response_code", v)?;
            }

            let _cond = { event.get_str("json.hour") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.hour") {
                        if let Some(val) = event.get("json.hour") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.hour".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.hour", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_hour_to_long")?;
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

            if event.has("json.http_code") {
                event.rename("json.http_code", "zscaler_zia.dns.http_code")?;
            }

            if event.has("json.istcp") {
                event.rename("json.istcp", "zscaler_zia.dns.istcp")?;
            }

            if event.has("json.loc") {
                event.rename("json.loc", "zscaler_zia.dns.loc")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.user") {
                    if let Some(s) = event.get_string("json.user") {
                        match url_decode(&s) {
                            Some(decoded) => event.set("json.user", json!(decoded))?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.user".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("json.user")
                    && event.get("json.user").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                if event.has("json.user") {
                    event.rename("json.user", "zscaler_zia.dns.user")?;
                }
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "user.email".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "create_user_name_and_user_domain",
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

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.minutes") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.minutes") {
                        if let Some(val) = event.get("json.minutes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.minutes".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.minutes", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_minutes_to_long",
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

            if event.has("json.month") {
                event.rename("json.month", "zscaler_zia.dns.month")?;
            }

            let _cond = { event.get_str("json.month_of_year") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.month_of_year") {
                        if let Some(val) = event.get("json.month_of_year") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.month_of_year".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.month_of_year", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_month_of_year_to_long",
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

            if event.has("json.oclientsourceip") {
                event.rename(
                    "json.oclientsourceip",
                    "zscaler_zia.dns.obfuscated.client_source_ip",
                )?;
            }

            if event.has("json.odevicename") {
                event.rename("json.odevicename", "zscaler_zia.dns.obfuscated.device.name")?;
            }

            if event.has("json.odeviceowner") {
                event.rename(
                    "json.odeviceowner",
                    "zscaler_zia.dns.obfuscated.device.owner",
                )?;
            }

            if event.has("json.odomcat") {
                event.rename("json.odomcat", "zscaler_zia.dns.obfuscated.dom.category")?;
            }

            if event.has("json.odevicehostname") {
                event.rename(
                    "json.odevicehostname",
                    "zscaler_zia.dns.obfuscated.host_name",
                )?;
            }

            if event.has("json.protocol") {
                event.rename("json.protocol", "zscaler_zia.dns.protocol")?;
            }

            let _cond = { event.has_value("zscaler_zia.dns.protocol") };
            if _cond {
                event.append_unique(
                    "network.transport",
                    json!(
                        event
                            .get("zscaler_zia.dns.protocol")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("network.transport") };
            if _cond {
                foreach_array(event, "network.transport", |event| {
                    if event.has_value("_ingest._value") {
                        map_strings(event, "_ingest._value", "_ingest._value", str::to_lowercase)?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.recordid") {
                if let Some(val) = event.get("json.recordid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.recordid".into(),
                            message,
                        }
                    })?;
                    event.set("zscaler_zia.dns.record.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.record.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has("json.reqaction") {
                event.rename("json.reqaction", "zscaler_zia.dns.request.action")?;
            }

            if event.has("json.dns_req") {
                event.rename("json.dns_req", "zscaler_zia.dns.request.name")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.request.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.name", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.reqrulelabel") {
                    if let Some(s) = event.get_string("json.reqrulelabel") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.dns.request.rule.label", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.reqrulelabel".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("zscaler_zia.dns.request.rule.label") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("zscaler_zia.dns.request.rule.label")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.dns_reqtype") {
                event.rename("json.dns_reqtype", "zscaler_zia.dns.request.type")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.request.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("dns.question.type", v)?;
            }

            if event.has("json.resaction") {
                event.rename("json.resaction", "zscaler_zia.dns.response.action")?;
            }

            if event.has("json.respipcategory") {
                event.rename("json.respipcategory", "zscaler_zia.dns.response.category")?;
            }

            let _cond =
                { event.has_value("json.dns_resp") && event.get_str("json.dns_resp") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.dns_resp") {
                        if let Some(val) = event.get("json.dns_resp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.dns_resp".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.response.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dns_resp_to_ip")?;
                    if event.has("json.dns_resp") {
                        event.rename("json.dns_resp", "zscaler_zia.dns.response.name")?;
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("zscaler_zia.dns.response.ip") };
            if _cond {
                event.append_unique(
                    "dns.answers",
                    json!(
                        event
                            .get("zscaler_zia.dns.response.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.dns.response.name") };
            if _cond {
                event.append_unique(
                    "dns.answers",
                    json!(
                        event
                            .get("zscaler_zia.dns.response.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get("dns.answers").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "dns.answers", |event| {
                    if event.has("_ingest._value") {
                        event.rename("_ingest._value", "_ingest._value.data")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("zscaler_zia.dns.response.ip") };
            if _cond {
                event.append_unique(
                    "dns.resolved_ip",
                    json!(
                        event
                            .get("zscaler_zia.dns.response.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.dns.response.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("zscaler_zia.dns.response.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.resrulelabel") {
                    if let Some(s) = event.get_string("json.resrulelabel") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.dns.response.rule.label", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.resrulelabel".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("zscaler_zia.dns.response.rule.label") };
            if _cond {
                event.append_unique(
                    "rule.name",
                    json!(
                        event
                            .get("zscaler_zia.dns.response.rule.label")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("json.restype") {
                event.rename("json.restype", "zscaler_zia.dns.response.type")?;
            }

            let _cond = { event.get("dns.answers").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "dns.answers", |event| {
                    if let Some(v) = event
                        .get("zscaler_zia.dns.response.type")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("_ingest._value.type", v)?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get_str("json.second") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.second") {
                        if let Some(val) = event.get("json.second") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.second".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.second", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_second_to_long")?;
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

            let _cond = { event.get_str("json.srv_dip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.srv_dip") {
                        if let Some(val) = event.get("json.srv_dip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.srv_dip".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.server.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_srv_dip_to_ip")?;
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
                .get("zscaler_zia.dns.server.ip")
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.srv_dport") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.srv_dport") {
                        if let Some(val) = event.get("json.srv_dport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.srv_dport".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.server.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_srv_dport_to_long",
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
                .get("zscaler_zia.dns.server.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has("json.tz") {
                event.rename("json.tz", "zscaler_zia.dns.timezone")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.dns.timezone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                event.set("event.timezone", json!("UTC"))?;
            }

            let _cond =
                { event.has_value("json.datetime") && event.get_str("json.datetime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.datetime") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-mm-dd HH:mm:ss",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            event.set("zscaler_zia.dns.time", parsed)?;
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
                .get("zscaler_zia.dns.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { event.get_str("json.year") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.year") {
                        if let Some(val) = event.get("json.year") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.year".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.dns.year", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_year_to_long")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.login") {
                    if let Some(s) = event.get_string("json.login") {
                        match url_decode(&s) {
                            Some(decoded) => event.set("json.login", json!(decoded))?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.login".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("json.login")
                    && event.get("json.login").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                if event.has("json.login") {
                    event.rename("json.login", "zscaler_zia.dns.login")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.department") {
                    if let Some(s) = event.get_string("json.department") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.dns.department", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.department".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.location") {
                    if let Some(s) = event.get_string("json.location") {
                        match url_decode(&s) {
                            Some(decoded) => {
                                event.set("zscaler_zia.dns.location", json!(decoded))?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.location".into(),
                                    message: format!("cannot url-decode '{s}'"),
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

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
                event.remove("zscaler_zia.dns.client.ip");
                event.remove("zscaler_zia.dns.cloud.name");
                event.remove("zscaler_zia.dns.company");
                event.remove("zscaler_zia.dns.device.hostname");
                event.remove("zscaler_zia.dns.device.model");
                event.remove("zscaler_zia.dns.device.name");
                event.remove("zscaler_zia.dns.device.os.version");
                event.remove("zscaler_zia.dns.device.type");
                event.remove("zscaler_zia.dns.dns.gateway.server_protocol");
                event.remove("zscaler_zia.dns.dns.type");
                event.remove("zscaler_zia.dns.error");
                event.remove("zscaler_zia.dns.user");
                event.remove("zscaler_zia.dns.protocol");
                event.remove("zscaler_zia.dns.response.name");
                event.remove("zscaler_zia.dns.response.ip");
                event.remove("zscaler_zia.dns.response.type");
                event.remove("zscaler_zia.dns.record.id");
                event.remove("zscaler_zia.dns.request.name");
                event.remove("zscaler_zia.dns.request.rule.label");
                event.remove("zscaler_zia.dns.request.type");
                event.remove("zscaler_zia.dns.response.rule.label");
                event.remove("zscaler_zia.dns.duration.milliseconds");
                event.remove("zscaler_zia.dns.server.ip");
                event.remove("zscaler_zia.dns.server.port");
                event.remove("zscaler_zia.dns.time");
            }

            event.remove("json");
            event.remove("_conf");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
