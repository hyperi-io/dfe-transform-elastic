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
            event.set("ecs.version", json!("8.17.0"))?;

            event.set("event.kind", json!("event"))?;

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
                event.has_value("json.action.type")
                    && !(event.get("json.action.type").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("APP")),
                        serde_json::Value::String(s) => s.contains("APP"),
                        _ => false,
                    }) || event.get("json.action.type").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("EXPORT"))
                        }
                        serde_json::Value::String(s) => s.contains("EXPORT"),
                        _ => false,
                    }) || event.get("json.action.type").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DESIGN"))
                        }
                        serde_json::Value::String(s) => s.contains("DESIGN"),
                        _ => false,
                    }))
            };
            if _cond {
                event.append("event.category", json!("iam"))?;
            }

            let _cond = { event.has_value("json.action.type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def eventType = new ArrayList(); if (ctx.event.type != null) {\n  eventType = ctx.event.type;\n} def val = ctx.json.action.type; if (val.toLowerCase().contains('remove') || val.toLowerCase().contains('delete')) {\n  eventType.add('deletion');\n} else if (val.toLowerCase().contains('update') || val.toLowerCase().contains('change')) {\n  eventType.add('change');\n} else if (val.toLowerCase().contains('create') || val.toLowerCase().contains('add')) {\n  eventType.add('creation');\n} else if (val.toLowerCase().contains('user')) {\n  eventType.add('user');\n} else if (val.toLowerCase().contains('group')) {\n  eventType.add('group');\n} else {\n  eventType.add('info');\n} ctx.event.put('type', eventType);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def eventType = new ArrayList(); if (ctx.event.type != null) {\n  eventType = ctx.event.type;\n} def val = ctx.json.action.type; if (val.toLowerCase().contains('remove') || val.toLowerCase().contains('delete')) {\n  eventType.add('deletion');\n} else if (val.toLowerCase().contains('update') || val.toLowerCase().contains('change')) {\n  eventType.add('change');\n} else if (val.toLowerCase().contains('create') || val.toLowerCase().contains('add')) {\n  eventType.add('creation');\n} else if (val.toLowerCase().contains('user')) {\n  eventType.add('user');\n} else if (val.toLowerCase().contains('group')) {\n  eventType.add('group');\n} else {\n  eventType.add('info');\n} ctx.event.put('type', eventType);"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_append_event_type",
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

            if event.has_value("json.action.app_id") {
                event.rename("json.action.app_id", "canva.audit.action.app.id")?;
            }

            if event.has_value("json.action.app_name") {
                event.rename("json.action.app_name", "canva.audit.action.app.name")?;
            }

            if event.has_value("json.action.app_version") {
                event.rename("json.action.app_version", "canva.audit.action.app.version")?;
            }

            if event.has_value("json.action.approval_status") {
                event.rename(
                    "json.action.approval_status",
                    "canva.audit.action.approval_status",
                )?;
            }

            if event.has_value("json.action.changed_fields") {
                event.rename(
                    "json.action.changed_fields",
                    "canva.audit.action.changed_fields",
                )?;
            }

            let _cond = {
                event
                    .get("json.action.changes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def convertKeysToBoolean(Map versionMap, String[] keysToConvert) {\n    def convertedMap = [: ];\n    versionMap.entrySet().forEach(entry -> {\n        def key = entry.getKey().toString();\n        def value = entry.getValue();\n        if (Arrays.asList(keysToConvert).contains(key)) {\n            if (value instanceof String) {\n                convertedMap[key] = Boolean.parseBoolean(value);\n            } else if (value instanceof Boolean) {\n                convertedMap[key] = (Boolean) value;\n            } else {\n                throw new Exception('Unsupported type');\n            }\n        } else {\n            convertedMap[key] = value;\n        }\n    });\n    return convertedMap;\n}\ndef valuesToConvert = new String[] {\n    'read',\n    'write',\n    'owning_team_only'\n};\nArrayList keysToConvert = new ArrayList([\n    'access',\n    'new_access',\n    'new_link_role',\n    'old_access',\n    'old_link_role'\n]);\ndef changeArray = ctx.json.action.changes;\nif (changeArray instanceof List) {\n    for (int i = 0; i < changeArray.size(); i++) {\n        for (def j = 0; j < keysToConvert.length; j++) {\n            if (changeArray[i][keysToConvert[j]] != null) {\n                changeArray[i][keysToConvert[j]] = convertKeysToBoolean(changeArray[i][keysToConvert[j]], valuesToConvert);\n            }\n        }\n        if (changeArray[i]['new_link_role'] != null && changeArray[i]['new_link_role']['access'] != null) {\n            changeArray[i]['new_link_role']['access'] = convertKeysToBoolean(changeArray[i]['new_link_role']['access'], valuesToConvert);\n        }\n        if (changeArray[i]['old_link_role'] != null && changeArray[i]['old_link_role']['access'] != null) {\n            changeArray[i]['old_link_role']['access'] = convertKeysToBoolean(changeArray[i]['old_link_role']['access'], valuesToConvert);\n        }\n        changeArray[i] = convertKeysToBoolean(changeArray[i], valuesToConvert);\n    }\n}\nctx.json.action.put('changes', changeArray);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def convertKeysToBoolean(Map versionMap, String[] keysToConvert) {\n    def convertedMap = [: ];\n    versionMap.entrySet().forEach(entry -> {\n        def key = entry.getKey().toString();\n        def value = entry.getValue();\n        if (Arrays.asList(keysToConvert).contains(key)) {\n            if (value instanceof String) {\n                convertedMap[key] = Boolean.parseBoolean(value);\n            } else if (value instanceof Boolean) {\n                convertedMap[key] = (Boolean) value;\n            } else {\n                throw new Exception('Unsupported type');\n            }\n        } else {\n            convertedMap[key] = value;\n        }\n    });\n    return convertedMap;\n}\ndef valuesToConvert = new String[] {\n    'read',\n    'write',\n    'owning_team_only'\n};\nArrayList keysToConvert = new ArrayList([\n    'access',\n    'new_access',\n    'new_link_role',\n    'old_access',\n    'old_link_role'\n]);\ndef changeArray = ctx.json.action.changes;\nif (changeArray instanceof List) {\n    for (int i = 0; i < changeArray.size(); i++) {\n        for (def j = 0; j < keysToConvert.length; j++) {\n            if (changeArray[i][keysToConvert[j]] != null) {\n                changeArray[i][keysToConvert[j]] = convertKeysToBoolean(changeArray[i][keysToConvert[j]], valuesToConvert);\n            }\n        }\n        if (changeArray[i]['new_link_role'] != null && changeArray[i]['new_link_role']['access'] != null) {\n            changeArray[i]['new_link_role']['access'] = convertKeysToBoolean(changeArray[i]['new_link_role']['access'], valuesToConvert);\n        }\n        if (changeArray[i]['old_link_role'] != null && changeArray[i]['old_link_role']['access'] != null) {\n            changeArray[i]['old_link_role']['access'] = convertKeysToBoolean(changeArray[i]['old_link_role']['access'], valuesToConvert);\n        }\n        changeArray[i] = convertKeysToBoolean(changeArray[i], valuesToConvert);\n    }\n}\nctx.json.action.put('changes', changeArray);\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_changes_fields_to_boolean",
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
                    .get("json.action.changes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: def relatedUser = new HashSet();\ndef relatedList = new ArrayList();\nif (ctx.related == null) {\n    ctx.related = new HashMap();\n}\nif (ctx.related.user != null) {\n    relatedUser.add(ctx.related.user);\n}\ndef changeArray = ctx.json.action.changes;\nif (changeArray instanceof List) {\n    for (int i = 0; i < changeArray.size(); i++) {\n        if (changeArray[i]['user'] != null) {\n            if (changeArray[i]['user']['display_name'] != null) {\n                relatedUser.add(changeArray[i]['user']['display_name']);\n            }\n            if (changeArray[i]['user']['email'] != null) {\n                relatedUser.add(changeArray[i]['user']['email']);\n            }\n            if (changeArray[i]['user']['id'] != null) {\n                relatedUser.add(changeArray[i]['user']['id']);\n            }\n        }\n        if (changeArray[i]['new_owner'] != null && changeArray[i]['new_owner']['email'] != null) {\n            relatedUser.add(changeArray[i]['new_owner']['email']);\n        }\n        if (changeArray[i]['recipient'] != null) {\n            relatedUser.add(changeArray[i]['recipient']);\n        }\n    }\n}\n for (def res : relatedUser) {\n    relatedList.add(res);\n }\nctx.related.put('user', relatedList);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def relatedUser = new HashSet();\ndef relatedList = new ArrayList();\nif (ctx.related == null) {\n    ctx.related = new HashMap();\n}\nif (ctx.related.user != null) {\n    relatedUser.add(ctx.related.user);\n}\ndef changeArray = ctx.json.action.changes;\nif (changeArray instanceof List) {\n    for (int i = 0; i < changeArray.size(); i++) {\n        if (changeArray[i]['user'] != null) {\n            if (changeArray[i]['user']['display_name'] != null) {\n                relatedUser.add(changeArray[i]['user']['display_name']);\n            }\n            if (changeArray[i]['user']['email'] != null) {\n                relatedUser.add(changeArray[i]['user']['email']);\n            }\n            if (changeArray[i]['user']['id'] != null) {\n                relatedUser.add(changeArray[i]['user']['id']);\n            }\n        }\n        if (changeArray[i]['new_owner'] != null && changeArray[i]['new_owner']['email'] != null) {\n            relatedUser.add(changeArray[i]['new_owner']['email']);\n        }\n        if (changeArray[i]['recipient'] != null) {\n            relatedUser.add(changeArray[i]['recipient']);\n        }\n    }\n}\n for (def res : relatedUser) {\n    relatedList.add(res);\n }\nctx.related.put('user', relatedList);\n"#
                    ),
                )?;
            }

            if event.has_value("json.action.changes") {
                event.rename("json.action.changes", "canva.audit.action.changes")?;
            }

            if event.has_value("json.action.country_code") {
                event.rename(
                    "json.action.country_code",
                    "canva.audit.action.country_code",
                )?;
            }

            if let Some(v) = event
                .get("canva.audit.action.country_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.country_iso_code", v)?;
            }

            if event.has_value("json.action.create_type") {
                event.rename("json.action.create_type", "canva.audit.action.create_type")?;
            }

            if event.has_value("json.action.default_team_id") {
                event.rename(
                    "json.action.default_team_id",
                    "canva.audit.action.default_team.id",
                )?;
            }

            if event.has_value("json.action.default_team_policy") {
                event.rename(
                    "json.action.default_team_policy",
                    "canva.audit.action.default_team.policy",
                )?;
            }

            if event.has_value("json.action.description") {
                event.rename("json.action.description", "canva.audit.action.description")?;
            }

            if event.has_value("json.action.design_type") {
                event.rename("json.action.design_type", "canva.audit.action.design_type")?;
            }

            if event.has_value("json.action.display_name") {
                event.rename(
                    "json.action.display_name",
                    "canva.audit.action.display_name",
                )?;
            }

            let _cond = { event.has_value("canva.audit.action.display_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("canva.audit.action.display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action.email") {
                event.rename("json.action.email", "canva.audit.action.email")?;
            }

            let _cond = { event.has_value("canva.audit.action.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("canva.audit.action.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action.email_verified") {
                    if let Some(val) = event.get("json.action.email_verified") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action.email_verified".into(),
                                message,
                            }
                        })?;
                        event.set("canva.audit.action.email_verified", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_email_verified_to_boolean",
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

            if event.has_value("json.action.emails") {
                event.rename("json.action.emails", "canva.audit.action.emails")?;
            }

            let _cond = {
                event
                    .get("canva.audit.action.emails")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "canva.audit.action.emails", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("json.action.end_timestamp")
                    && event.has_value("json.action.start_timestamp")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script, resolved to its runners at generation time
                    // Source: ctx.event.duration = (ctx.json.action.end_timestamp - ctx.json.action.start_timestamp) * 1000000;
                    scale_field(
                        event,
                        &ScaleField::new(
                            "json.action.start_timestamp",
                            "event.duration",
                            Factor::Long(1000000),
                        ),
                    );
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_event_duration",
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
                event.has_value("json.action.end_timestamp")
                    && event.get_str("json.action.end_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.action.end_timestamp") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("canva.audit.action.end_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.action.end_timestamp".into(),
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
                        "date_action_end_timestamp",
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
                .get("canva.audit.action.end_timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if event.has_value("json.action.first_name") {
                event.rename("json.action.first_name", "canva.audit.action.first_name")?;
            }

            if event.has_value("json.action.last_name") {
                event.rename("json.action.last_name", "canva.audit.action.last_name")?;
            }

            if event.has_value("json.action.locale") {
                event.rename("json.action.locale", "canva.audit.action.locale")?;
            }

            if event.has_value("json.action.login_type") {
                event.rename("json.action.login_type", "canva.audit.action.login_type")?;
            }

            if event.has_value("json.action.managing_entity.team.display_name") {
                event.rename(
                    "json.action.managing_entity.team.display_name",
                    "canva.audit.action.managing_entity.team.display_name",
                )?;
            }

            if event.has_value("json.action.managing_entity.team.id") {
                event.rename(
                    "json.action.managing_entity.team.id",
                    "canva.audit.action.managing_entity.team.id",
                )?;
            }

            if event.has_value("json.action.managing_entity.organization.id") {
                event.rename(
                    "json.action.managing_entity.organization.id",
                    "canva.audit.action.managing_entity.organization.id",
                )?;
            }

            if let Some(v) = event
                .get("canva.audit.action.managing_entity.organization.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            if event.has_value("json.action.managing_entity.type") {
                event.rename(
                    "json.action.managing_entity.type",
                    "canva.audit.action.managing_entity.type",
                )?;
            }

            if event.has_value("json.action.new_display_name") {
                event.rename(
                    "json.action.new_display_name",
                    "canva.audit.action.new_display_name",
                )?;
            }

            if event.has_value("json.action.new_name") {
                event.rename("json.action.new_name", "canva.audit.action.new_name")?;
            }

            if event.has_value("json.action.new_permissions") {
                event.rename(
                    "json.action.new_permissions",
                    "canva.audit.action.new_permissions",
                )?;
            }

            if event.has_value("json.action.new_role") {
                event.rename("json.action.new_role", "canva.audit.action.new_role")?;
            }

            let _cond = { event.has_value("json.action.oauth_accounts") };
            if _cond {
                foreach_array(event, "json.action.oauth_accounts", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.external_user_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.action.oauth_accounts") {
                event.rename(
                    "json.action.oauth_accounts",
                    "canva.audit.action.oauth_accounts",
                )?;
            }

            if event.has_value("json.action.oauth_platform") {
                event.rename(
                    "json.action.oauth_platform",
                    "canva.audit.action.oauth_platform",
                )?;
            }

            if event.has_value("json.action.old_display_name") {
                event.rename(
                    "json.action.old_display_name",
                    "canva.audit.action.old_display_name",
                )?;
            }

            if event.has_value("json.action.old_name") {
                event.rename("json.action.old_name", "canva.audit.action.old_name")?;
            }

            if event.has_value("json.action.old_permissions") {
                event.rename(
                    "json.action.old_permissions",
                    "canva.audit.action.old_permissions",
                )?;
            }

            if event.has_value("json.action.old_role") {
                event.rename("json.action.old_role", "canva.audit.action.old_role")?;
            }

            if event.has_value("json.action.original_design_id") {
                event.rename(
                    "json.action.original_design_id",
                    "canva.audit.action.original_design_id",
                )?;
            }

            if event.has_value("json.action.output_type") {
                event.rename("json.action.output_type", "canva.audit.action.output_type")?;
            }

            if event.has_value("json.action.permissions") {
                event.rename("json.action.permissions", "canva.audit.action.permissions")?;
            }

            if event.has_value("json.action.phone_number") {
                event.rename(
                    "json.action.phone_number",
                    "canva.audit.action.phone_number",
                )?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("canva.audit.action.phone_number")
            };
            if _cond {
                event.set("canva.audit.action.phone_number", json!("REDACTED"))?;
            }

            if event.has_value("json.action.reason.type") {
                event.rename("json.action.reason.type", "canva.audit.action.reason.type")?;
            }

            if event.has_value("json.action.report_type") {
                event.rename("json.action.report_type", "canva.audit.action.report_type")?;
            }

            if event.has_value("json.action.role") {
                event.rename("json.action.role", "canva.audit.action.role")?;
            }

            let _cond = { event.has_value("canva.audit.action.role") };
            if _cond {
                event.append_unique(
                    "user.changes.roles",
                    json!(
                        event
                            .get("canva.audit.action.role")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action.saml_accounts") {
                event.rename(
                    "json.action.saml_accounts",
                    "canva.audit.action.saml_accounts",
                )?;
            }

            if event.has_value("json.action.session_scope") {
                event.rename(
                    "json.action.session_scope",
                    "canva.audit.action.session_scope",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action.sms_mfa_enabled") {
                    if let Some(val) = event.get("json.action.sms_mfa_enabled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action.sms_mfa_enabled".into(),
                                message,
                            }
                        })?;
                        event.set("canva.audit.action.sms_mfa_enabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_sms_mfa_enabled_to_boolean",
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

            let _cond = {
                event.has_value("json.action.start_timestamp")
                    && event.get_str("json.action.start_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.action.start_timestamp") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("canva.audit.action.start_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.action.start_timestamp".into(),
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
                        "date_action_start_timestamp",
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
                .get("canva.audit.action.start_timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            if event.has_value("json.action.team.display_name") {
                event.rename(
                    "json.action.team.display_name",
                    "canva.audit.action.team.display_name",
                )?;
            }

            if event.has_value("json.action.team.id") {
                event.rename("json.action.team.id", "canva.audit.action.team.id")?;
            }

            if event.has_value("json.action.team_address.city") {
                event.rename(
                    "json.action.team_address.city",
                    "canva.audit.action.team_address.city",
                )?;
            }

            if event.has_value("json.action.team_address.country_code") {
                event.rename(
                    "json.action.team_address.country_code",
                    "canva.audit.action.team_address.country_code",
                )?;
            }

            if event.has_value("json.action.team_address.postcode") {
                if let Some(val) = event.get("json.action.team_address.postcode") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.action.team_address.postcode".into(),
                            message,
                        }
                    })?;
                    event.set("canva.audit.action.team_address.postcode", converted)?;
                }
            }

            if event.has_value("json.action.team_address.street1") {
                event.rename(
                    "json.action.team_address.street1",
                    "canva.audit.action.team_address.street1",
                )?;
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("hide_sensitive"))
                        }
                        serde_json::Value::String(s) => s.contains("hide_sensitive"),
                        _ => false,
                    })
                    && event.has_value("canva.audit.action.team_address.street1")
            };
            if _cond {
                event.set("canva.audit.action.team_address.street1", json!("REDACTED"))?;
            }

            if event.has_value("json.action.team_address.subdivision") {
                event.rename(
                    "json.action.team_address.subdivision",
                    "canva.audit.action.team_address.subdivision",
                )?;
            }

            if event.has_value("json.action.title") {
                event.rename("json.action.title", "canva.audit.action.title")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.action.totp_mfa_enabled") {
                    if let Some(val) = event.get("json.action.totp_mfa_enabled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.action.totp_mfa_enabled".into(),
                                message,
                            }
                        })?;
                        event.set("canva.audit.action.totp_mfa_enabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_action_totp_mfa_enabled_to_boolean",
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

            if event.has_value("json.action.type") {
                event.rename("json.action.type", "canva.audit.action.type")?;
            }

            if let Some(v) = event
                .get("canva.audit.action.type")
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
                        if let Some(s) = event.get_string("event.action") {
                            let mut parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("event.action", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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

            let _cond = { event.get("event.action").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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

            if event.has_value("json.action.user.display_name") {
                event.rename(
                    "json.action.user.display_name",
                    "canva.audit.action.user.display_name",
                )?;
            }

            if let Some(v) = event
                .get("canva.audit.action.user.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.changes.full_name", v)?;
            }

            let _cond = { event.has_value("user.changes.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.changes.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action.user.email") {
                event.rename("json.action.user.email", "canva.audit.action.user.email")?;
            }

            if let Some(v) = event
                .get("canva.audit.action.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.changes.email", v)?;
            }

            let _cond = { event.has_value("user.changes.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.changes.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action.user.id") {
                event.rename("json.action.user.id", "canva.audit.action.user.id")?;
            }

            if let Some(v) = event
                .get("canva.audit.action.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.changes.id", v)?;
            }

            let _cond = { event.has_value("user.changes.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.changes.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action.user_scope") {
                event.rename("json.action.user_scope", "canva.audit.action.user_scope")?;
            }

            if event.has_value("json.action.view_type") {
                event.rename("json.action.view_type", "canva.audit.action.view_type")?;
            }

            if event.has_value("json.actor.details.type") {
                event.rename("json.actor.details.type", "canva.audit.actor.details.type")?;
            }

            if event.has_value("json.actor.organization.id") {
                event.rename(
                    "json.actor.organization.id",
                    "canva.audit.actor.organization.id",
                )?;
            }

            if event.has_value("json.actor.team.display_name") {
                event.rename(
                    "json.actor.team.display_name",
                    "canva.audit.actor.team.display_name",
                )?;
            }

            if event.has_value("json.actor.team.id") {
                event.rename("json.actor.team.id", "canva.audit.actor.team.id")?;
            }

            if event.has_value("json.actor.type") {
                event.rename("json.actor.type", "canva.audit.actor.type")?;
            }

            if event.has_value("json.actor.user.display_name") {
                event.rename(
                    "json.actor.user.display_name",
                    "canva.audit.actor.user.display_name",
                )?;
            }

            if let Some(v) = event
                .get("canva.audit.actor.user.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.full_name", v)?;
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

            if event.has_value("json.actor.user.email") {
                event.rename("json.actor.user.email", "canva.audit.actor.user.email")?;
            }

            if let Some(v) = event
                .get("canva.audit.actor.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = {
                event.has_value("user.email")
                    && event.get("user.email").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
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

            if event.has_value("json.actor.user.id") {
                event.rename("json.actor.user.id", "canva.audit.actor.user.id")?;
            }

            if let Some(v) = event
                .get("canva.audit.actor.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.context.device_id") {
                event.rename("json.context.device_id", "canva.audit.context.device_id")?;
            }

            if let Some(v) = event
                .get("canva.audit.context.device_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            let _cond = {
                event.has_value("json.context.ip_address")
                    && event.get_str("json.context.ip_address") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.context.ip_address") {
                        if let Some(val) = event.get("json.context.ip_address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.context.ip_address".into(),
                                    message,
                                }
                            })?;
                            event.set("canva.audit.context.ip_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_context_ip_address_to_ip",
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
                .get("canva.audit.context.ip_address")
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

            if event.has_value("json.context.request_id") {
                event.rename("json.context.request_id", "canva.audit.context.request_id")?;
            }

            if event.has_value("json.context.session") {
                event.rename("json.context.session", "canva.audit.context.session")?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "canva.audit.id")?;
            }

            if let Some(v) = event
                .get("canva.audit.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.outcome.details.resource_id") {
                event.rename(
                    "json.outcome.details.resource_id",
                    "canva.audit.outcome.details.resource.id",
                )?;
            }

            if event.has_value("json.outcome.details.resource_type") {
                event.rename(
                    "json.outcome.details.resource_type",
                    "canva.audit.outcome.details.resource.type",
                )?;
            }

            if event.has_value("json.outcome.details.type") {
                event.rename(
                    "json.outcome.details.type",
                    "canva.audit.outcome.details.type",
                )?;
            }

            if event.has_value("json.outcome.details.user_id") {
                event.rename(
                    "json.outcome.details.user_id",
                    "canva.audit.outcome.details.user_id",
                )?;
            }

            let _cond = { event.has_value("canva.audit.outcome.details.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("canva.audit.outcome.details.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.outcome.result") {
                event.rename("json.outcome.result", "canva.audit.outcome.result")?;
            }

            let _cond = { event.has_value("canva.audit.outcome.result") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = { event.get_str("canva.audit.outcome.result") == Some("DENIED") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("canva.audit.outcome.result") == Some("PERMITTED") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            if event.has_value("json.target.id") {
                event.rename("json.target.id", "canva.audit.target.id")?;
            }

            let _cond = { event.has_value("canva.audit.target.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("canva.audit.target.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.target.name") {
                event.rename("json.target.name", "canva.audit.target.name")?;
            }

            let _cond = { event.has_value("canva.audit.target.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("canva.audit.target.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.target.organization.id") {
                event.rename(
                    "json.target.organization.id",
                    "canva.audit.target.organization.id",
                )?;
            }

            if event.has_value("json.target.owner.organization.id") {
                event.rename(
                    "json.target.owner.organization.id",
                    "canva.audit.target.owner.organization.id",
                )?;
            }

            if event.has_value("json.target.owner.team.display_name") {
                event.rename(
                    "json.target.owner.team.display_name",
                    "canva.audit.target.owner.team.display_name",
                )?;
            }

            if event.has_value("json.target.owner.team.id") {
                event.rename(
                    "json.target.owner.team.id",
                    "canva.audit.target.owner.team.id",
                )?;
            }

            if event.has_value("json.target.owner.type") {
                event.rename("json.target.owner.type", "canva.audit.target.owner.type")?;
            }

            if event.has_value("json.target.owner.user.display_name") {
                event.rename(
                    "json.target.owner.user.display_name",
                    "canva.audit.target.owner.user.display_name",
                )?;
            }

            let _cond = { event.has_value("canva.audit.target.owner.user.display_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("canva.audit.target.owner.user.display_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.target.owner.user.email") {
                event.rename(
                    "json.target.owner.user.email",
                    "canva.audit.target.owner.user.email",
                )?;
            }

            let _cond = { event.has_value("canva.audit.target.owner.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("canva.audit.target.owner.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.target.owner.user.id") {
                event.rename(
                    "json.target.owner.user.id",
                    "canva.audit.target.owner.user.id",
                )?;
            }

            let _cond = { event.has_value("canva.audit.target.owner.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("canva.audit.target.owner.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.target.resource_type") {
                event.rename(
                    "json.target.resource_type",
                    "canva.audit.target.resource_type",
                )?;
            }

            if event.has_value("json.target.target_type") {
                event.rename("json.target.target_type", "canva.audit.target.target_type")?;
            }

            if event.has_value("json.target.team.display_name") {
                event.rename(
                    "json.target.team.display_name",
                    "canva.audit.target.team.display_name",
                )?;
            }

            if let Some(v) = event
                .get("canva.audit.target.team.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.group.name", v)?;
            }

            if event.has_value("json.target.team.id") {
                event.rename("json.target.team.id", "canva.audit.target.team.id")?;
            }

            if let Some(v) = event
                .get("canva.audit.target.team.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.group.id", v)?;
            }

            if event.has_value("json.target.user.display_name") {
                event.rename(
                    "json.target.user.display_name",
                    "canva.audit.target.user.display_name",
                )?;
            }

            if let Some(v) = event
                .get("canva.audit.target.user.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.full_name", v)?;
            }

            let _cond = { event.has_value("user.target.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.target.user.email") {
                event.rename("json.target.user.email", "canva.audit.target.user.email")?;
            }

            if let Some(v) = event
                .get("canva.audit.target.user.email")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.email", v)?;
            }

            let _cond = { event.has_value("user.target.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.target.user.id") {
                event.rename("json.target.user.id", "canva.audit.target.user.id")?;
            }

            if let Some(v) = event
                .get("canva.audit.target.user.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.id", v)?;
            }

            let _cond = { event.has_value("user.target.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => event.set("canva.audit.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
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
                .get("canva.audit.timestamp")
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
                event.remove("canva.audit.action.country_code");
                event.remove("canva.audit.action.end_timestamp");
                event.remove("canva.audit.action.role");
                event.remove("canva.audit.action.start_timestamp");
                event.remove("canva.audit.action.type");
                event.remove("canva.audit.action.user.display_name");
                event.remove("canva.audit.action.user.email");
                event.remove("canva.audit.actor.user.display_name");
                event.remove("canva.audit.actor.user.email");
                event.remove("canva.audit.actor.user.id");
                event.remove("canva.audit.context.device_id");
                event.remove("canva.audit.context.ip_address");
                event.remove("canva.audit.id");
                event.remove("canva.audit.outcome.result");
                event.remove("canva.audit.action.user.id");
                event.remove("canva.audit.target.team.id");
                event.remove("canva.audit.target.team.display_name");
                event.remove("canva.audit.target.user.display_name");
                event.remove("canva.audit.target.user.email");
                event.remove("canva.audit.target.user.id");
                event.remove("canva.audit.timestamp");
                event.remove("canva.audit.action.managing_entity.organization.id");
            }

            event.remove("json");

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
