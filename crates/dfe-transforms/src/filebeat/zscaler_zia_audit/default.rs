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
                if event.has_value("message") {
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

            if event.has_value("resp.event") {
                event.rename("resp.event", "json")?;
            }

            let _cond =
                { event.has_value("json") && event.get_bool("_conf.strict_fields") == Some(true) };
            if _cond {
                // Painless script
                // Source: if (ctx.resp?.version == null) {\n  def fields = [];\n  for (e in ctx.json.entrySet()) {\n    fields.add(e.getKey());\n  }\n  Collections.sort(fields);\n  String signature = String.join(\"|\", fields);\n  if (signature != params.expect.fields) {\n    ctx.error = ctx.error ?: [:];\n    ctx.error.message = ctx.error.message ?: [];\n    ctx.error.message.add(\"field set mismatch: \"+signature+\" is not expected set of templated fields (see \"+params.data_stream+\" in https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n  }\n} else if (ctx.resp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add(\"template version mismatch: \"+ctx.resp.version.toString()+\" is not expected version (see \"+params.data_stream+\" in https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.resp?.version == null) {\n  def fields = [];\n  for (e in ctx.json.entrySet()) {\n    fields.add(e.getKey());\n  }\n  Collections.sort(fields);\n  String signature = String.join(\"|\", fields);\n  if (signature != params.expect.fields) {\n    ctx.error = ctx.error ?: [:];\n    ctx.error.message = ctx.error.message ?: [];\n    ctx.error.message.add(\"field set mismatch: \"+signature+\" is not expected set of templated fields (see \"+params.data_stream+\" in https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n  }\n} else if (ctx.resp.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add(\"template version mismatch: \"+ctx.resp.version.toString()+\" is not expected version (see \"+params.data_stream+\" in https://epr.elastic.co/package/zscaler_zia/\"+params.pkg_version+\"/docs/README.md)\");\n}"#
                    ),
                    cached_params!(
                        "{\"data_stream\":\"audit-log\",\"expect\":{\"fields\":\"action|adminid|auditlogtype|category|clientip|errorcode|interface|postaction|preaction|recordid|resource|result|subcategory|time|timezone\",\"version\":\"v1\"},\"pkg_version\":\"3.15.1\"}"
                    ),
                )?;
            }

            event.remove("resp");

            event.set("event.kind", json!("event"))?;

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None' || object == 'Unknown') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '' || object == 'NA' || object == 'None' || object == 'Unknown') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);"#
                ),
            )?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.json?.category == null || params.get(ctx.json.category) == null) {\n  return;\n}\nparams.get(ctx.json.category).forEach((k, v) -> {\n  if (v instanceof List) {\n    ctx.event[k] = new ArrayList(v);\n  } else {\n    ctx.event[k] = v;\n  }\n});
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (ctx.json?.category == null || params.get(ctx.json.category) == null) {\n  return;\n}\nparams.get(ctx.json.category).forEach((k, v) -> {\n  if (v instanceof List) {\n    ctx.event[k] = new ArrayList(v);\n  } else {\n    ctx.event[k] = v;\n  }\n});"#
                    ),
                    cached_params!(
                        "{\"ACCESS_CONTROL_RESOURCE\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"ACTIVATION\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"ADMINISTRATOR_MANAGEMENT\":{\"category\":[\"iam\"],\"type\":[\"admin\"]},\"ADVANCED_SETTINGS\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"ALERT\":{\"category\":[\"intrusion_detection\"],\"type\":[\"info\"]},\"AUDIT_LOGS\":{\"category\":[\"file\"],\"type\":[\"access\"]},\"AUTHENTICATION_SETTINGS\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"BACKUP_&_RESTORE\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"CASB\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"CLOUD_APPLICATION_STATUS\":{\"category\":[\"api\"],\"type\":[\"info\"]},\"CLOUD_SERVICE_API_KEY\":{\"category\":[\"api\"],\"type\":[\"info\"]},\"COMPANY_PROFILE\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"DATA_LOSS_PREVENTION_RESOURCE\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"DC_EXCLUSION\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"DLP_INCIDENT_RECEIVER\":{\"category\":[\"intrusion_detection\"],\"type\":[\"info\"]},\"FIREWALL_ACCESS_CONTROL\":{\"category\":[\"network\"],\"type\":[\"access\"]},\"FIREWALL_RESOURCE\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"FORWARDING_METHOD\":{\"category\":[\"network\"],\"type\":[\"change\"]},\"HELP\":{\"category\":[\"web\"],\"type\":[\"info\"]},\"IDENTITY_PROXY_SETTINGS\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"IPV6_CONFIGURATION\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"ISOLATION_MANAGEMENT\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"LOGIN\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"MOBILE_ACCESS_CONTROL\":{\"category\":[\"network\"],\"type\":[\"access\"]},\"MOBILE_SECURITY\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"NSS\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"ORGANIZATION_INFO\":{\"category\":[\"web\"],\"type\":[\"info\"]},\"PARTNER_INTEGRATION\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"REPORT\":{\"category\":[\"web\"],\"type\":[\"info\"]},\"ROLE_MANAGEMENT\":{\"category\":[\"iam\"],\"type\":[\"admin\"]},\"ROOT_CERTIFICATE_MANAGEMENT\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"RULE_LABEL_MANAGEMENT\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"SAAS_ASSETS_REPORT\":{\"category\":[\"web\"],\"type\":[\"info\"]},\"SAAS_SECURITY_API\":{\"category\":[\"api\"],\"type\":[\"info\"]},\"TRAFFIC_CAPTURE\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"TRAFFIC_FORWARDING_RESOURCE\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"USER_MANAGEMENT\":{\"category\":[\"iam\"],\"type\":[\"user\"]},\"VIRTUAL_SERVICE_EDGE\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"VIRTUAL_ZEN\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"WEB_ACCESS_CONTROL\":{\"category\":[\"web\"],\"type\":[\"access\"]},\"WEB_DATA_LOSS_PREVENTION\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"WEB_SECURITY\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"WORKLOAD_GROUP_MANAGEMENT\":{\"category\":[\"configuration\"],\"type\":[\"change\"]}}"
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "Set ECS categorization fields",
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

            if event.has_value("json.action") {
                event.rename("json.action", "zscaler_zia.audit.action")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.audit.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
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
                        gsub_field(
                            event,
                            "event.action",
                            "event.action",
                            cached_regex!(" "),
                            "-",
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set("_ingest.on_failure_processor_tag", "gsub_event_action")?;
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

            if event.has_value("json.adminid") {
                event.rename("json.adminid", "zscaler_zia.audit.admin_id")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.audit.admin_id")
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

            if event.has_value("json.auditlogtype") {
                event.rename("json.auditlogtype", "zscaler_zia.audit.audit_log_type")?;
            }

            if event.has_value("json.category") {
                event.rename("json.category", "zscaler_zia.audit.category")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.audit.category")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.ruleset", v)?;
            }

            let _cond = { event.get_str("json.clientip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.clientip") {
                        if let Some(val) = event.get("json.clientip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.clientip".into(),
                                    message,
                                }
                            })?;
                            event.set("zscaler_zia.audit.client_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_clientip_to_ip")?;
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
                .get("zscaler_zia.audit.client_ip")
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

            if event.has_value("json.errorcode") {
                event.rename("json.errorcode", "zscaler_zia.audit.error_code")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.audit.error_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("error.code", v)?;
            }

            if event.has_value("json.interface") {
                event.rename("json.interface", "zscaler_zia.audit.interface")?;
            }

            if event.has_value("json.postaction") {
                event.rename("json.postaction", "zscaler_zia.audit.post_action")?;
            }

            if event.has_value("json.preaction") {
                event.rename("json.preaction", "zscaler_zia.audit.pre_action")?;
            }

            if event.has_value("json.recordid") {
                event.rename("json.recordid", "zscaler_zia.audit.record.id")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.audit.record.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.resource") {
                event.rename("json.resource", "zscaler_zia.audit.resource")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.audit.resource")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.result") {
                event.rename("json.result", "zscaler_zia.audit.result")?;
            }

            let _cond = { event.get_str("zscaler_zia.audit.result") == Some("SUCCESS") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("success"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("zscaler_zia.audit.result") == Some("FAILURE") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("failure"))?;
                    Ok(())
                })();
            }

            if event.has_value("json.subcategory") {
                event.rename("json.subcategory", "zscaler_zia.audit.sub_category")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.audit.sub_category")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.category", v)?;
            }

            if event.has_value("json.timezone") {
                event.rename("json.timezone", "zscaler_zia.audit.timezone")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.audit.timezone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                event.set("event.timezone", json!("UTC"))?;
            }

            let _cond = { event.has_value("json.time") && event.get_str("json.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(
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
                            Some(parsed) => event.set("zscaler_zia.audit.time", parsed)?,
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
                    event.set("_ingest.on_failure_processor_tag", "date_time_if_timezone")?;
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
                .get("zscaler_zia.audit.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
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
                event.remove("zscaler_zia.audit.action");
                event.remove("zscaler_zia.audit.admin_id");
                event.remove("zscaler_zia.audit.client_ip");
                event.remove("zscaler_zia.audit.record.id");
                event.remove("zscaler_zia.audit.time");
                event.remove("zscaler_zia.audit.category");
                event.remove("zscaler_zia.audit.error_code");
                event.remove("zscaler_zia.audit.resource");
                event.remove("zscaler_zia.audit.sub_category");
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
