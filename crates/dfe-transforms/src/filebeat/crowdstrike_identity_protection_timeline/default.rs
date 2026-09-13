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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            event.set("ecs.version", json!("8.17.0"))?;

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

            parse_json_field(event, "event.original", "crowdstrike.idp.timeline")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.original") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("crowdstrike.idp.timeline") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.crowdstrike.idp.timeline = convertToSnakeCase(ctx.crowdstrike.idp.timeline);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.crowdstrike.idp.timeline = convertToSnakeCase(ctx.crowdstrike.idp.timeline);\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_convert_camelcase_to_snake_case",
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

            let _cond = { event.has_value("crowdstrike.idp") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = { event.has_value("crowdstrike.idp.timeline.event_type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def m = params.get(ctx.crowdstrike.idp.timeline.event_type);\nif (m != null) {\n  m.forEach((k, v) -> {\n    if (v instanceof List) {\n      ctx.event[k] = new ArrayList(v);\n    } else {\n      ctx.event[k] = v;\n    }\n  });\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def m = params.get(ctx.crowdstrike.idp.timeline.event_type);\nif (m != null) {\n  m.forEach((k, v) -> {\n    if (v instanceof List) {\n      ctx.event[k] = new ArrayList(v);\n    } else {\n      ctx.event[k] = v;\n    }\n  });\n}"#
                        ),
                        cached_params!(
                            "{\"SUCCESSFUL_AUTHENTICATION\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"FAILED_AUTHENTICATION\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"SERVICE_ACCESS\":{\"category\":[\"network\"],\"type\":[\"access\"]},\"LDAP_SEARCH\":{\"category\":[\"database\"],\"type\":[\"info\"]},\"ACCOUNT_CREATED\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"PASSWORD_CHANGE\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"ACCOUNT_ENABLED\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"ACCOUNT_DISABLED\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"ACCOUNT_LOCKED\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"ACCOUNT_UNLOCKED\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"PRIVILEGE_ESCALATION\":{\"category\":[\"iam\"],\"type\":[\"change\"]}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_event_category_and_type_from_timeline_event_type",
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

            let _cond = {
                event
                    .get("crowdstrike.idp.timeline.event_severity")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("crowdstrike.idp.timeline.event_severity") != Some("")
            };
            if _cond {
                // Painless script
                // Source: Integer severity = params[ctx.crowdstrike.idp.timeline.event_severity.toUpperCase()];\nif (severity != null) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.severity = severity;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"Integer severity = params[ctx.crowdstrike.idp.timeline.event_severity.toUpperCase()];\nif (severity != null) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.severity = severity;\n}\n"#
                    ),
                    cached_params!("{\"NEUTRAL\":21,\"MODERATE\":47,\"IMPORTANT\":73}"),
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.event_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.event_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.user_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.user_entity.primary_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = { event.has_value("crowdstrike.idp.timeline.entity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def entity = ctx.crowdstrike.idp.timeline.entity;\ndef entityType = entity.type;\ndef secondary = entity.secondary_display_name;\nif (secondary != null && secondary instanceof String && secondary != '') {\n  int slash = secondary.indexOf('\\\\');\n  if (slash >= 0) {\n    def domain = secondary.substring(0, slash);\n    if (domain != null && domain != '' && entityType == 'USER') {\n      ctx.user = ctx.user ?: [:];\n      ctx.user.domain = domain;\n    }\n  } else if (entityType == 'ENDPOINT') {\n    ctx.host = ctx.host ?: [:];\n    ctx.host.hostname = secondary;\n  }\n}\nif (entityType == 'ENDPOINT') {\n  if (entity.primary_display_name != null) {\n    ctx.host = ctx.host ?: [:];\n    ctx.host.name = entity.primary_display_name;\n  }\n} else if (entityType == 'USER') {\n  if (entity.primary_display_name != null) {\n    ctx.user = ctx.user ?: [:];\n    ctx.user.full_name = entity.primary_display_name;\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def entity = ctx.crowdstrike.idp.timeline.entity;\ndef entityType = entity.type;\ndef secondary = entity.secondary_display_name;\nif (secondary != null && secondary instanceof String && secondary != '') {\n  int slash = secondary.indexOf('\\\\');\n  if (slash >= 0) {\n    def domain = secondary.substring(0, slash);\n    if (domain != null && domain != '' && entityType == 'USER') {\n      ctx.user = ctx.user ?: [:];\n      ctx.user.domain = domain;\n    }\n  } else if (entityType == 'ENDPOINT') {\n    ctx.host = ctx.host ?: [:];\n    ctx.host.hostname = secondary;\n  }\n}\nif (entityType == 'ENDPOINT') {\n  if (entity.primary_display_name != null) {\n    ctx.host = ctx.host ?: [:];\n    ctx.host.name = entity.primary_display_name;\n  }\n} else if (entityType == 'USER') {\n  if (entity.primary_display_name != null) {\n    ctx.user = ctx.user ?: [:];\n    ctx.user.full_name = entity.primary_display_name;\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "map_entity_identity_fields_by_type",
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
                .get("crowdstrike.idp.timeline.endpoint_entity.primary_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { !event.has_value("host.name") };
            if _cond {
                if let Some(v) = event
                    .get("crowdstrike.idp.timeline.endpoint_display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.host_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.target_endpoint_entity.primary_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.protocol_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.tls_version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("tls.version", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.target_service_display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.name", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.target_service_identifier")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.id", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.operating_system_info.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.full", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.operating_system_info.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            let _cond =
                { event.has_value("crowdstrike.idp.timeline.operating_system_info.family") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def os = params[ctx.crowdstrike.idp.timeline.operating_system_info.family.toUpperCase()];\nif (os != null) {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.os = ctx.host.os ?: [:];\n  ctx.host.os.type = os;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def os = params[ctx.crowdstrike.idp.timeline.operating_system_info.family.toUpperCase()];\nif (os != null) {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.os = ctx.host.os ?: [:];\n  ctx.host.os.type = os;\n}\n"#
                        ),
                        cached_params!(
                            "{\"WINDOWS\":\"windows\",\"OSX\":\"macos\",\"UNIX\":\"unix\",\"LINUX\":\"linux\",\"IOS\":\"ios\",\"ANDROID\":\"android\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_set_host_os_type_from_operating_system_info_family",
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
                .get("crowdstrike.idp.timeline.operating_system_info.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.version", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.geo_location.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_name", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.geo_location.city")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.city_name", v)?;
            }

            let _cond = {
                event
                    .get("crowdstrike.idp.timeline.user_entity.email_addresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def emails = ctx.crowdstrike.idp.timeline.user_entity.email_addresses;\nif (emails.size() > 0 && emails[0] != null) {\n  ctx.user = ctx.user ?: [:];\n  ctx.user.email = emails[0];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def emails = ctx.crowdstrike.idp.timeline.user_entity.email_addresses;\nif (emails.size() > 0 && emails[0] != null) {\n  ctx.user = ctx.user ?: [:];\n  ctx.user.email = emails[0];\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.idp.timeline.entity.accounts")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def entity = ctx.crowdstrike.idp.timeline.entity;\ndef entityType = entity.type;\ndef accounts = entity.accounts;\nif (accounts.size() == 0 || !(accounts[0] instanceof Map)) {\n  return;\n}\ndef acc = accounts[0];\nif (entityType == 'ENDPOINT') {\n  ctx.host = ctx.host ?: [:];\n  if (acc.sam_account_name != null) {\n    def sam = acc.sam_account_name;\n    if (sam.endsWith('$')) {\n      sam = sam.substring(0, sam.length() - 1);\n    }\n    if (ctx.host.name == null) {\n      ctx.host.name = sam;\n    }\n  }\n  if (acc.object_sid != null) {\n    ctx.host.id = acc.object_sid;\n  }\n} else if (entityType == 'USER') {\n  ctx.user = ctx.user ?: [:];\n  if (acc.sam_account_name != null) {\n    ctx.user.name = acc.sam_account_name;\n    ctx.user.target = ctx.user.target ?: [:];\n    ctx.user.target.name = acc.sam_account_name;\n  }\n  if (acc.object_sid != null) {\n    ctx.user.id = acc.object_sid;\n  }\n} else {\n  ctx.entity = ctx.entity ?: [:];\n  if (acc.sam_account_name != null) {\n    ctx.entity.target = ctx.entity.target ?: [:];\n    ctx.entity.target.name = acc.sam_account_name;\n  }\n  if (acc.object_sid != null) {\n    ctx.entity.id = acc.object_sid;\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def entity = ctx.crowdstrike.idp.timeline.entity;\ndef entityType = entity.type;\ndef accounts = entity.accounts;\nif (accounts.size() == 0 || !(accounts[0] instanceof Map)) {\n  return;\n}\ndef acc = accounts[0];\nif (entityType == 'ENDPOINT') {\n  ctx.host = ctx.host ?: [:];\n  if (acc.sam_account_name != null) {\n    def sam = acc.sam_account_name;\n    if (sam.endsWith('$')) {\n      sam = sam.substring(0, sam.length() - 1);\n    }\n    if (ctx.host.name == null) {\n      ctx.host.name = sam;\n    }\n  }\n  if (acc.object_sid != null) {\n    ctx.host.id = acc.object_sid;\n  }\n} else if (entityType == 'USER') {\n  ctx.user = ctx.user ?: [:];\n  if (acc.sam_account_name != null) {\n    ctx.user.name = acc.sam_account_name;\n    ctx.user.target = ctx.user.target ?: [:];\n    ctx.user.target.name = acc.sam_account_name;\n  }\n  if (acc.object_sid != null) {\n    ctx.user.id = acc.object_sid;\n  }\n} else {\n  ctx.entity = ctx.entity ?: [:];\n  if (acc.sam_account_name != null) {\n    ctx.entity.target = ctx.entity.target ?: [:];\n    ctx.entity.target.name = acc.sam_account_name;\n  }\n  if (acc.object_sid != null) {\n    ctx.entity.id = acc.object_sid;\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "map_entity_accounts_by_type",
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

            let _cond = {
                event.has_value("crowdstrike.idp.timeline.ip_address")
                    && event.get_str("crowdstrike.idp.timeline.ip_address") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.idp.timeline.ip_address") {
                        if let Some(val) = event.get("crowdstrike.idp.timeline.ip_address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.idp.timeline.ip_address".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_timeline_ip_address_to_source_ip",
                    )?;
                    event.remove("crowdstrike.idp.timeline.ip_address");
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = {
                event.has_value("crowdstrike.idp.timeline.timestamp")
                    && event.get_str("crowdstrike.idp.timeline.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.idp.timeline.timestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.idp.timeline.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.idp.timeline.timestamp".into(),
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
                        "date_crowdstrike_idp_timestamp",
                    )?;
                    event.remove("crowdstrike.idp.timeline.timestamp");
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
                event.has_value("crowdstrike.idp.timeline.start_time")
                    && event.get_str("crowdstrike.idp.timeline.start_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.idp.timeline.start_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.idp.timeline.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.idp.timeline.start_time".into(),
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
                        "date_crowdstrike_idp_start_time",
                    )?;
                    event.remove("crowdstrike.idp.timeline.start_time");
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
                event.has_value("crowdstrike.idp.timeline.end_time")
                    && event.get_str("crowdstrike.idp.timeline.end_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("crowdstrike.idp.timeline.end_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("crowdstrike.idp.timeline.end_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "crowdstrike.idp.timeline.end_time".into(),
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
                        "date_crowdstrike_idp_end_time",
                    )?;
                    event.remove("crowdstrike.idp.timeline.end_time");
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
                .get("crowdstrike.idp.timeline.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if let Some(v) = event
                .get("crowdstrike.idp.timeline.end_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
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

            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.full_name")
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

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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
                event.remove("crowdstrike.idp.timeline.event_id");
                event.remove("crowdstrike.idp.timeline.event_type");
                event.remove("crowdstrike.idp.timeline.timestamp");
                event.remove("crowdstrike.idp.timeline.start_time");
                event.remove("crowdstrike.idp.timeline.end_time");
                event.remove("crowdstrike.idp.timeline.endpoint_entity");
                event.remove("crowdstrike.idp.timeline.ip_address");
                event.remove("crowdstrike.idp.timeline.protocol_type");
                event.remove("crowdstrike.idp.timeline.tls_version");
                event.remove("crowdstrike.idp.timeline.target_service_display_name");
                event.remove("crowdstrike.idp.timeline.target_service_identifier");
                event.remove("crowdstrike.idp.timeline.user_display_name");
                event.remove("crowdstrike.idp.timeline.user_entity.primary_display_name");
                event.remove("crowdstrike.idp.timeline.entity.primary_display_name");
                event.remove("crowdstrike.idp.timeline.operating_system_info");
                event.remove("crowdstrike.idp.timeline.geo_location");
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
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
