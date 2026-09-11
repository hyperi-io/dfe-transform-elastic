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
            event.set("ecs.version", json!("8.16.0"))?;

            event.set("observer.vendor", json!("Google Workspace"))?;

            event.set("observer.product", json!("Calendar"))?;

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

            let _cond = { event.has_value("event.original") };
            if _cond {
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
            }

            let _cond = {
                event
                    .get("json.events.parameters")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script, resolved to its runners at generation time
                    // Source: ctx.google_workspace = ctx.google_workspace ?: [:]; ctx.google_workspace.calendar = ctx.google_workspace.calendar ?: [:]; for (def param : ctx.json.events.parameters) {\n  if (param.name == null) {\n    continue;\n  }\n  def lw_case_name = param.name.toLowerCase();\n  if (param.value != null) {\n    ctx.google_workspace.calendar[lw_case_name] = param.value;\n  } else if (param.boolValue != null) {\n    ctx.google_workspace.calendar[lw_case_name] = param.boolValue;\n  } else if (param.intValue != null) {\n    ctx.google_workspace.calendar[lw_case_name] = param.intValue;\n  } else if (param.multiValue != null) {\n    ctx.google_workspace.calendar[lw_case_name] = param.multiValue;\n  } else if (param.multiIntValue != null) {\n    ctx.google_workspace.calendar[lw_case_name] = param.multiIntValue;\n  } else if (param.multiBoolValue != null) {\n    ctx.google_workspace.calendar[lw_case_name] = param.multiBoolValue;\n  }\n}\n
                    parameters_into_map(
                        event,
                        &ParametersIntoMap::new(
                            "json.events.parameters".into(),
                            "google_workspace.calendar".into(),
                            "name".into(),
                            true,
                            vec![
                                "value".into(),
                                "boolValue".into(),
                                "intValue".into(),
                                "multiValue".into(),
                                "multiIntValue".into(),
                                "multiBoolValue".into(),
                            ],
                        ),
                    );
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_flatten_event_parameters",
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

            event.remove("json.events.parameters");

            if event.has_value("json.events.name") {
                event.rename("json.events.name", "google_workspace.calendar.name")?;
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("configuration"))?;

            let _cond = { event.has_value("google_workspace.calendar.name") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\ndef category = params.get(ctx.google_workspace.calendar.name);\nif (category == null) {\n  ctx.event.remove('category');\n} else {\n  ctx.event.category = [category];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\ndef category = params.get(ctx.google_workspace.calendar.name);\nif (category == null) {\n  ctx.event.remove('category');\n} else {\n  ctx.event.category = [category];\n}"#
                        ),
                        cached_params!(
                            "{\"change_calendar_acls\":\"iam\",\"change_calendar_country\":\"configuration\",\"create_calendar\":\"configuration\",\"delete_calendar\":\"configuration\",\"change_calendar_description\":\"configuration\",\"change_calendar_location\":\"configuration\",\"change_calendar_timezone\":\"configuration\",\"change_calendar_title\":\"configuration\",\"add_subscription\":\"configuration\",\"delete_subscription\":\"configuration\",\"change_appointment_schedule\":\"configuration\",\"create_appointment_schedule\":\"configuration\",\"delete_appointment_schedule\":\"configuration\",\"create_event\":\"configuration\",\"delete_event\":\"configuration\",\"add_event_guest\":\"configuration\",\"change_event_guest_response_auto\":\"configuration\",\"remove_event_guest\":\"configuration\",\"change_event_guest_response\":\"configuration\",\"change_event\":\"configuration\",\"remove_event_from_trash\":\"configuration\",\"restore_event\":\"configuration\",\"change_event_start_time\":\"configuration\",\"change_event_title\":\"configuration\",\"transfer_event_completed\":\"configuration\",\"transfer_event_requested\":\"configuration\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_event_category_from_calendar_name",
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

            let _cond = { event.has_value("google_workspace.calendar.name") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\ndef type = params.get(ctx.google_workspace.calendar.name);\nif (type == null) {\n  ctx.event.remove('type');\n} else {\n  ctx.event.type = [type];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\ndef type = params.get(ctx.google_workspace.calendar.name);\nif (type == null) {\n  ctx.event.remove('type');\n} else {\n  ctx.event.type = [type];\n}"#
                        ),
                        cached_params!(
                            "{\"change_calendar_acls\":\"change\",\"change_calendar_country\":\"change\",\"create_calendar\":\"creation\",\"delete_calendar\":\"deletion\",\"change_calendar_description\":\"change\",\"export_calendar\":\"info\",\"change_calendar_location\":\"change\",\"print_preview_calendar\":\"info\",\"change_calendar_timezone\":\"change\",\"change_calendar_title\":\"change\",\"notification_triggered\":\"info\",\"add_subscription\":\"change\",\"delete_subscription\":\"change\",\"change_appointment_schedule\":\"change\",\"create_appointment_schedule\":\"creation\",\"delete_appointment_schedule\":\"deletion\",\"create_event\":\"creation\",\"delete_event\":\"deletion\",\"add_event_guest\":\"change\",\"change_event_guest_response_auto\":\"change\",\"remove_event_guest\":\"change\",\"change_event_guest_response\":\"change\",\"change_event\":\"change\",\"print_preview_event\":\"info\",\"remove_event_from_trash\":\"change\",\"restore_event\":\"change\",\"change_event_start_time\":\"change\",\"change_event_title\":\"change\",\"transfer_event_completed\":\"change\",\"transfer_event_requested\":\"info\",\"interop_freebusy_lookup_outbound_successful\":\"info\",\"interop_freebusy_lookup_inbound_successful\":\"info\",\"interop_exchange_resource_availability_lookup_successful\":\"info\",\"interop_exchange_resource_list_lookup_successful\":\"info\",\"interop_freebusy_lookup_outbound_unsuccessful\":\"info\",\"interop_freebusy_lookup_inbound_unsuccessful\":\"info\",\"interop_exchange_resource_availability_lookup_unsuccessful\":\"info\",\"interop_exchange_resource_list_lookup_unsuccessful\":\"info\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_event_type_from_calendar_name",
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

            if event.has_value("json.kind") {
                event.rename("json.kind", "google_workspace.kind")?;
            }

            if event.has_value("json.ownerDomain") {
                event.rename("json.ownerDomain", "google_workspace.organization.domain")?;
            }

            if event.has_value("json.etag") {
                event.rename("json.etag", "google_workspace.etag")?;
            }

            if event.has_value("json.actor.callerType") {
                event.rename(
                    "json.actor.callerType",
                    "google_workspace.actor.caller_type",
                )?;
            }

            if event.has_value("json.actor.email") {
                event.rename("json.actor.email", "google_workspace.actor.email")?;
            }

            if let Some(v) = event
                .get("google_workspace.actor.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.email", v)?;
            }

            if let Some(v) = event
                .get("google_workspace.actor.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            let _cond = {
                event.has_value("source.user.email")
                    && event.get("source.user.email").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("source.user.email") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("source.user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "source.user.email".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set("_ingest.on_failure_processor_tag", "dissect_user_email")?;
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

            if event.has_value("json.actor.key") {
                event.rename("json.actor.key", "google_workspace.actor.key")?;
            }

            if event.has_value("json.actor.profileId") {
                event.rename("json.actor.profileId", "google_workspace.actor.profile_id")?;
            }

            if let Some(v) = event
                .get("google_workspace.actor.profile_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
            }

            if let Some(v) = event
                .get("source.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user", v)?;
            }

            if event.has_value("json.id.applicationName") {
                event.rename(
                    "json.id.applicationName",
                    "google_workspace.id.application_name",
                )?;
            }

            if let Some(v) = event
                .get("google_workspace.id.application_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if event.has_value("json.id.customerId") {
                event.rename("json.id.customerId", "google_workspace.id.customer_id")?;
            }

            if let Some(v) = event
                .get("google_workspace.id.customer_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            let _cond =
                { event.has_value("json.id.time") && event.get_str("json.id.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.id.time") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => event.set("google_workspace.id.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.id.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_id_time")?;
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
                .get("google_workspace.id.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.id.uniqueQualifier") {
                event.rename(
                    "json.id.uniqueQualifier",
                    "google_workspace.id.unique_qualifier",
                )?;
            }

            if let Some(v) = event
                .get("google_workspace.id.unique_qualifier")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = { event.get_str("json.ipAddress") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ipAddress") {
                        if let Some(val) = event.get("json.ipAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ipAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("google_workspace.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ipAddress_to_ip",
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
                .get("google_workspace.ip_address")
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

            if event.has_value("json.events.type") {
                event.rename("json.events.type", "google_workspace.calendar.type")?;
            }

            if let Some(v) = event
                .get("google_workspace.calendar.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
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
                            cached_regex!("_"),
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

            if event.has_value("google_workspace.calendar.appointment_schedule_title") {
                event.rename(
                    "google_workspace.calendar.appointment_schedule_title",
                    "google_workspace.calendar.event.appointment_schedule_title",
                )?;
            }

            if event.has_value("google_workspace.calendar.calendar_country") {
                event.rename(
                    "google_workspace.calendar.calendar_country",
                    "google_workspace.calendar.country",
                )?;
            }

            if event.has_value("google_workspace.calendar.calendar_description") {
                event.rename(
                    "google_workspace.calendar.calendar_description",
                    "google_workspace.calendar.description",
                )?;
            }

            if event.has_value("google_workspace.calendar.client_side_encrypted") {
                event.rename(
                    "google_workspace.calendar.client_side_encrypted",
                    "google_workspace.calendar.event.client_side_encrypted",
                )?;
            }

            let _cond = { event.has_value("google_workspace.calendar.end_time") };
            if _cond {
                // Painless script
                // Source: long gregorianOffset = 62135683200L;\nlong startTimeInSeconds = Long.parseLong(ctx.google_workspace.calendar.end_time);\nctx.google_workspace.calendar.end_time = (startTimeInSeconds - gregorianOffset) * 1000L;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"long gregorianOffset = 62135683200L;\nlong startTimeInSeconds = Long.parseLong(ctx.google_workspace.calendar.end_time);\nctx.google_workspace.calendar.end_time = (startTimeInSeconds - gregorianOffset) * 1000L;"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("google_workspace.calendar.end_time")
                    && event.get_str("google_workspace.calendar.end_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_workspace.calendar.end_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("google_workspace.calendar.event.end_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_workspace.calendar.end_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_end_time")?;
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

            if event.has_value("google_workspace.calendar.grantee_email") {
                event.rename(
                    "google_workspace.calendar.grantee_email",
                    "google_workspace.calendar.event.grantee_email",
                )?;
            }

            if let Some(v) = event
                .get("google_workspace.calendar.event.grantee_email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.email", v)?;
            }

            if event.has_value("google_workspace.calendar.event_guest") {
                event.rename(
                    "google_workspace.calendar.event_guest",
                    "google_workspace.calendar.event.guest",
                )?;
            }

            if event.has_value("google_workspace.calendar.event_id") {
                event.rename(
                    "google_workspace.calendar.event_id",
                    "google_workspace.calendar.event.id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.calendar.is_recurring") {
                    if let Some(val) = event.get("google_workspace.calendar.is_recurring") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.calendar.is_recurring".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.calendar.event.is_recurring", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_recurring_to_boolean",
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

            if event.has_value("google_workspace.calendar.old_event_title") {
                event.rename(
                    "google_workspace.calendar.old_event_title",
                    "google_workspace.calendar.event.old_title",
                )?;
            }

            if event.has_value("google_workspace.calendar.organizer_calendar_id") {
                event.rename(
                    "google_workspace.calendar.organizer_calendar_id",
                    "google_workspace.calendar.event.organizer_calendar_id",
                )?;
            }

            if event.has_value("google_workspace.calendar.recurring") {
                event.rename(
                    "google_workspace.calendar.recurring",
                    "google_workspace.calendar.event.recurring",
                )?;
            }

            if event.has_value("google_workspace.calendar.event_response_status") {
                event.rename(
                    "google_workspace.calendar.event_response_status",
                    "google_workspace.calendar.event.response_status",
                )?;
            }

            let _cond = { event.has_value("google_workspace.calendar.start_time") };
            if _cond {
                // Painless script
                // Source: long gregorianOffset = 62135683200L;\nlong startTimeInSeconds = Long.parseLong(ctx.google_workspace.calendar.start_time);\nctx.google_workspace.calendar.start_time = (startTimeInSeconds - gregorianOffset) * 1000L;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"long gregorianOffset = 62135683200L;\nlong startTimeInSeconds = Long.parseLong(ctx.google_workspace.calendar.start_time);\nctx.google_workspace.calendar.start_time = (startTimeInSeconds - gregorianOffset) * 1000L;"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("google_workspace.calendar.start_time")
                    && event.get_str("google_workspace.calendar.start_time") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_workspace.calendar.start_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("google_workspace.calendar.event.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_workspace.calendar.start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_start_time")?;
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.calendar.secs_in_advance") {
                    if let Some(val) = event.get("google_workspace.calendar.secs_in_advance") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_workspace.calendar.secs_in_advance".into(),
                                message,
                            }
                        })?;
                        event.set("google_workspace.calendar.secs_in_advance", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_secs_in_advance_to_long",
                )?;
                if event
                    .remove("google_workspace.calendar.secs_in_advance")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "google_workspace.calendar.secs_in_advance".into(),
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

            if event.has_value("google_workspace.calendar.event_title") {
                event.rename(
                    "google_workspace.calendar.event_title",
                    "google_workspace.calendar.event.title",
                )?;
            }

            if event.has_value("google_workspace.calendar.calendar_id") {
                event.rename(
                    "google_workspace.calendar.calendar_id",
                    "google_workspace.calendar.id",
                )?;
            }

            if event.has_value("google_workspace.calendar.interop_error_code") {
                event.rename(
                    "google_workspace.calendar.interop_error_code",
                    "google_workspace.calendar.interop.error_code",
                )?;
            }

            if let Some(v) = event
                .get("google_workspace.calendar.interop.error_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("error.code", v)?;
            }

            if event.has_value("google_workspace.calendar.remote_ews_url") {
                event.rename(
                    "google_workspace.calendar.remote_ews_url",
                    "google_workspace.calendar.interop.remote_ews_url",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_workspace.calendar.interop.remote_ews_url") {
                    if !uri_parts(
                        event,
                        "google_workspace.calendar.interop.remote_ews_url",
                        "url",
                        true,
                        false,
                    )? && event
                        .get_str("google_workspace.calendar.interop.remote_ews_url")
                        .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "google_workspace.calendar.interop.remote_ews_url".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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

            if event.has_value("google_workspace.calendar.calendar_location") {
                event.rename(
                    "google_workspace.calendar.calendar_location",
                    "google_workspace.calendar.location",
                )?;
            }

            if event.has_value("google_workspace.calendar.notification_message_id") {
                event.rename(
                    "google_workspace.calendar.notification_message_id",
                    "google_workspace.calendar.notification.message_id",
                )?;
            }

            if event.has_value("google_workspace.calendar.notification_method") {
                event.rename(
                    "google_workspace.calendar.notification_method",
                    "google_workspace.calendar.notification.method",
                )?;
            }

            if event.has_value("google_workspace.calendar.recipient_email") {
                event.rename(
                    "google_workspace.calendar.recipient_email",
                    "google_workspace.calendar.notification.recipient_email",
                )?;
            }

            if let Some(v) = event
                .get("google_workspace.calendar.notification.recipient_email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.email", v)?;
            }

            if event.has_value("google_workspace.calendar.notification_type") {
                event.rename(
                    "google_workspace.calendar.notification_type",
                    "google_workspace.calendar.notification.type",
                )?;
            }

            let _cond = {
                event.has_value("google_workspace.calendar.requested_period_end")
                    && event.get_str("google_workspace.calendar.requested_period_end") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_workspace.calendar.requested_period_end")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => event
                                .set("google_workspace.calendar.requested_period_end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_workspace.calendar.requested_period_end".into(),
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
                        "date_requested_period_end",
                    )?;
                    if event
                        .remove("google_workspace.calendar.requested_period_end")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_workspace.calendar.requested_period_end".into(),
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

            let _cond = {
                event.has_value("google_workspace.calendar.requested_period_start")
                    && event.get_str("google_workspace.calendar.requested_period_start") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_workspace.calendar.requested_period_start")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => event
                                .set("google_workspace.calendar.requested_period_start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_workspace.calendar.requested_period_start".into(),
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
                        "date_requested_period_start",
                    )?;
                    if event
                        .remove("google_workspace.calendar.requested_period_start")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "google_workspace.calendar.requested_period_start".into(),
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

            if event.has_value("google_workspace.calendar.calendar_timezone") {
                event.rename(
                    "google_workspace.calendar.calendar_timezone",
                    "google_workspace.calendar.timezone",
                )?;
            }

            if event.has_value("google_workspace.calendar.calendar_title") {
                event.rename(
                    "google_workspace.calendar.calendar_title",
                    "google_workspace.calendar.title",
                )?;
            }

            if event.has_value("google_workspace.calendar.user_agent") {
                if let Some(ua_str) = event.get_string("google_workspace.calendar.user_agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
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

            let _cond = { event.has_value("google_workspace.actor.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_workspace.actor.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_workspace.calendar.event.guest") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_workspace.calendar.event.guest")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.append_unique(
                "related.user",
                json!(
                    event
                        .get("google_workspace.calendar.event.grantee_email")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond =
                { event.has_value("google_workspace.calendar.event.organizer_calendar_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_workspace.calendar.event.organizer_calendar_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("google_workspace.calendar.notification.recipient_email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_workspace.calendar.notification.recipient_email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_workspace.calendar.subscriber_calendar_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_workspace.calendar.subscriber_calendar_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_workspace.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("google_workspace.ip_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
                event.remove("google_workspace.actor.email");
                event.remove("google_workspace.actor.profile_id");
                event.remove("google_workspace.id.application_name");
                event.remove("google_workspace.id.customer_id");
                event.remove("google_workspace.id.time");
                event.remove("google_workspace.id.unique_qualifier");
                event.remove("google_workspace.ip_address");
                event.remove("google_workspace.calendar.id");
                event.remove("google_workspace.calendar.user_agent");
                event.remove("google_workspace.calendar.interop.error_code");
            }

            event.remove("google_workspace.calendar.start_time");
            event.remove("google_workspace.calendar.end_time");
            event.remove("google_workspace.calendar.is_recurring");

            event.remove("json");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script, resolved to its runners at generation time
                // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_drop_null_values",
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
                        "Processor '{}' {}failed with message '{}'",
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
                                "with tag '{}' ",
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
