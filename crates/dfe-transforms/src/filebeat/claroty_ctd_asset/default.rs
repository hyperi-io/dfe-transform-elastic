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

            event.append("event.category", json!("host"))?;

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("info"))?;

            if event.has_value("json.active_queries_names") {
                event.rename(
                    "json.active_queries_names",
                    "claroty_ctd.asset.active.queries_names",
                )?;
            }

            if event.has_value("json.active_scans_names") {
                event.rename(
                    "json.active_scans_names",
                    "claroty_ctd.asset.active.scans_names",
                )?;
            }

            if event.has_value("json.active_tasks_names") {
                event.rename(
                    "json.active_tasks_names",
                    "claroty_ctd.asset.active.tasks_names",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.approved") {
                    if let Some(val) = event.get("json.approved") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.approved".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.approved", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_approved_to_boolean",
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

            if event.has_value("json.os_architecture") {
                event.rename("json.os_architecture", "claroty_ctd.asset.os.architecture")?;
            }

            if event.has_value("json.os_build") {
                event.rename("json.os_build", "claroty_ctd.asset.os.build")?;
            }

            if event.has_value("json.os_revision") {
                event.rename("json.os_revision", "claroty_ctd.asset.os.revision")?;
            }

            if event.has_value("json.os_service_pack") {
                event.rename("json.os_service_pack", "claroty_ctd.asset.os.service_pack")?;
            }

            if event.has_value("json.asset_type__") {
                event.rename("json.asset_type__", "claroty_ctd.asset.asset_types.name")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.asset_type") {
                    if let Some(val) = event.get("json.asset_type") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.asset_type".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.asset_types.number", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_asset_type_to_long",
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

            let _cond = { event.get("json.children").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def convertToLong(def value) {\n  if (value instanceof String) {\n    return Long.parseLong(value);\n  } else if (value instanceof Number) {\n    return ((long) value).longValue();\n  } else {\n    throw new Exception('Unsupported type');\n  }\n}\ndef convertToBoolean(def value) {\n  if (value instanceof String) {\n    return Boolean.parseBoolean(value);\n  } else if (value instanceof Boolean) {\n    return (Boolean) value;\n  } else {\n    throw new Exception('Unsupported type');\n  }\n}\ndef renameKeys(Map json, Map keyMap, List longFields, List stringFields, List boolFields) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap, longFields, stringFields, boolFields);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap, longFields, stringFields, boolFields);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap, longFields, stringFields, boolFields));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        if (longFields.contains(keyMap[key])) {\n          updatedJson[keyMap[key]] = convertToLong(value);\n        } else if (stringFields.contains(keyMap[key]) && value != null) {\n          updatedJson[keyMap[key]] = value.toString();\n        } else if (boolFields.contains(keyMap[key])) {\n          updatedJson[keyMap[key]] = convertToBoolean(value);\n        } else {\n          updatedJson[keyMap[key]] = value;\n        }\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\nctx.claroty_ctd.asset.put('children', new ArrayList());\nfor (child in ctx.json.children) {\n  def children = renameKeys(child, params.renamefield, params.longfield, params.stringfield, params.boolfield);\n  ctx.claroty_ctd.asset.children.add(children);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def convertToLong(def value) {\n  if (value instanceof String) {\n    return Long.parseLong(value);\n  } else if (value instanceof Number) {\n    return ((long) value).longValue();\n  } else {\n    throw new Exception('Unsupported type');\n  }\n}\ndef convertToBoolean(def value) {\n  if (value instanceof String) {\n    return Boolean.parseBoolean(value);\n  } else if (value instanceof Boolean) {\n    return (Boolean) value;\n  } else {\n    throw new Exception('Unsupported type');\n  }\n}\ndef renameKeys(Map json, Map keyMap, List longFields, List stringFields, List boolFields) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap, longFields, stringFields, boolFields);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap, longFields, stringFields, boolFields);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap, longFields, stringFields, boolFields));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        if (longFields.contains(keyMap[key])) {\n          updatedJson[keyMap[key]] = convertToLong(value);\n        } else if (stringFields.contains(keyMap[key]) && value != null) {\n          updatedJson[keyMap[key]] = value.toString();\n        } else if (boolFields.contains(keyMap[key])) {\n          updatedJson[keyMap[key]] = convertToBoolean(value);\n        } else {\n          updatedJson[keyMap[key]] = value;\n        }\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\nctx.claroty_ctd.asset.put('children', new ArrayList());\nfor (child in ctx.json.children) {\n  def children = renameKeys(child, params.renamefield, params.longfield, params.stringfield, params.boolfield);\n  ctx.claroty_ctd.asset.children.add(children);\n}\n"#
                        ),
                        cached_params!(
                            "{\"renamefield\":{\"asset_type\":\"asset_type\",\"asset_type__\":\"asset_type__\",\"class_type\":\"class_type\",\"criticality\":\"criticality\",\"criticality__\":\"criticality__\",\"special_hint\":\"special_hint\",\"special_hint__\":\"special_hint__\",\"subnet_id\":\"subnet_id\",\"edge_id\":\"edge_id\",\"network_id\":\"network_id\",\"network.id\":\"network.id\",\"network.site_id\":\"network.site_id\",\"network.resource_id\":\"network.resource_id\",\"resource_id\":\"resource_id\",\"id\":\"id\",\"site_id\":\"site_id\",\"virtual_zone_id\":\"virtual_zone_id\",\"virtual_zone_name\":\"virtual_zone_name\"},\"longfield\":[\"asset_type\",\"subnet_type\",\"risk_level\",\"custom_informations.category\",\"special_hint\",\"criticality\",\"custom_informations.priority\",\"custom_informations.type\",\"project_parsed.creation_time\",\"project_parsed.information_type\",\"project_parsed.modification_time\",\"project_parsed.priority\"],\"stringfield\":[\"subnet_id\",\"id\",\"edge_id\",\"network_id\",\"virtual_zone_id\",\"site_id\",\"network.site_id\",\"network.id\",\"network.resource_id\",\"resource_id\"],\"boolfield\":[\"approved\",\"ghost\",\"parsed\"]}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_map_fields_under_children_object",
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
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.virtual_zone_id") {
                        event.rename(
                            "_ingest._value.virtual_zone_id",
                            "_ingest._value.virtual_zone.id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.virtual_zone_name") {
                        event.rename(
                            "_ingest._value.virtual_zone_name",
                            "_ingest._value.virtual_zone.name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.special_hint__") {
                        event.rename(
                            "_ingest._value.special_hint__",
                            "_ingest._value.special_hints.name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.special_hint") {
                        event.rename(
                            "_ingest._value.special_hint",
                            "_ingest._value.special_hints.value",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.virtual_zone_id") {
                        event.rename(
                            "_ingest._value.virtual_zone_id",
                            "_ingest._value.virtual_zone.id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.os_architecture") {
                        event.rename(
                            "_ingest._value.os_architecture",
                            "_ingest._value.os.architecture",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.os_revision") {
                        event.rename("_ingest._value.os_revision", "_ingest._value.os.revision")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.os_build") {
                        event.rename("_ingest._value.os_build", "_ingest._value.os.build")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.os_service_pack") {
                        event.rename(
                            "_ingest._value.os_service_pack",
                            "_ingest._value.os.service_pack",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("claroty_ctd.asset.children.virtual_zone_id") {
                event.rename(
                    "claroty_ctd.asset.children.virtual_zone_id",
                    "claroty_ctd.asset.children.virtual_zone.id",
                )?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.asset_type__") {
                        event.rename(
                            "_ingest._value.asset_type__",
                            "_ingest._value.asset_types.name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.asset_type") {
                        event.rename(
                            "_ingest._value.asset_type",
                            "_ingest._value.asset_types.number",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.criticality__") {
                        event.rename(
                            "_ingest._value.criticality__",
                            "_ingest._value.criticalities.name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.children", |event| {
                    if event.has_value("_ingest._value.criticality") {
                        event.rename(
                            "_ingest._value.criticality",
                            "_ingest._value.criticalities.value",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.plc_slots") {
                event.rename("json.plc_slots", "claroty_ctd.asset.plc_slots")?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    if event.has_value("_ingest._value.PLCSlotInformation") {
                        event.rename(
                            "_ingest._value.PLCSlotInformation",
                            "_ingest._value.plcslotinformations",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    if event.has_value("_ingest._value.plcslotinformations.value.PLCInformation") {
                        event.rename(
                            "_ingest._value.plcslotinformations.value.PLCInformation",
                            "_ingest._value.plcslotinformations.value.plcinformation",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.plcslotinformation.information_type") {
                            if let Some(val) =
                                event.get("_ingest._value.plcslotinformation.information_type")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.plcslotinformation.information_type"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "_ingest._value.plcslotinformation.information_type",
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
                            "convert_plcslotinformation_information_type_to_long",
                        )?;
                        event.remove("_ingest._value.plcslotinformation.information_type");
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.plcslotinformation.priority") {
                            if let Some(val) =
                                event.get("_ingest._value.plcslotinformation.priority")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.plcslotinformation.priority".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("_ingest._value.plcslotinformation.priority", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_plc_slots_plcslotinformation_priority_to_long",
                        )?;
                        event.remove("_ingest._value.plcslotinformation.priority");
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.plcslotinformations.slot") {
                            if let Some(val) = event.get("_ingest._value.plcslotinformations.slot")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.plcslotinformations.slot".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.plcslotinformations.slot", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_plc_slots_plcslotinformation_priority_to_long",
                        )?;
                        event.remove("_ingest._value.plcslotinformations.slot");
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.plcslotinformations.value.plcinformation.information_type") {
                    if let Some(val) = event.get("_ingest._value.plcslotinformations.value.plcinformation.information_type") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.plcslotinformations.value.plcinformation.information_type".into(),
                    message,
                    })?;
                    event.set("_ingest._value.plcslotinformations.value.plcinformation.information_type", converted)?;
                    }
                    }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_plc_slots_plcslotinformations_value_plcinformation_information_type_to_long")?;
                        event.remove("_ingest._value.plcslotinformations.value.plcinformation.information_type");
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.plcslotinformations.information_type") {
                            if let Some(val) =
                                event.get("_ingest._value.plcslotinformations.information_type")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.plcslotinformations.information_type"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "_ingest._value.plcslotinformations.information_type",
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
                            "convert_plc_slots_plcslotinformations_information_type_to_long",
                        )?;
                        event.remove("_ingest._value.plcslotinformations.information_type");
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value(
                            "_ingest._value.plcslotinformations.value.plcinformation.priority",
                        ) {
                            if let Some(val) = event.get(
                                "_ingest._value.plcslotinformations.value.plcinformation.priority",
                            ) {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                    path: "_ingest._value.plcslotinformations.value.plcinformation.priority".into(),
                    message,
                    }
                                })?;
                                event.set("_ingest._value.plcslotinformations.value.plcinformation.priority", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_plc_slots_plcslotinformations_value_plcinformation_priority_to_long")?;
                        event.remove(
                            "_ingest._value.plcslotinformations.value.plcinformation.priority",
                        );
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.plcslotinformations.priority") {
                            if let Some(val) =
                                event.get("_ingest._value.plcslotinformations.priority")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.plcslotinformations.priority".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "_ingest._value.plcslotinformations.priority",
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
                            "convert_plc_slots_plcslotinformations_priority_to_long",
                        )?;
                        event.remove("_ingest._value.plcslotinformations.priority");
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.plc_slots")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.plc_slots", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.slot") {
                            if let Some(val) = event.get("_ingest._value.slot") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.slot".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.slot", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_plc_slots_slot_to_long",
                        )?;
                        event.remove("_ingest._value.slot");
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

            if event.has_value("json.serial_number") {
                event.rename("json.serial_number", "claroty_ctd.asset.serial_number")?;
            }

            if event.has_value("json.state") {
                event.rename("json.state", "claroty_ctd.asset.state")?;
            }

            if event.has_value("json.edge_last_run") {
                event.rename("json.edge_last_run", "claroty_ctd.asset.edge_last_run")?;
            }

            if event.has_value("json.firmware") {
                event.rename("json.firmware", "claroty_ctd.asset.firmware")?;
            }

            if event.has_value("json.model") {
                event.rename("json.model", "claroty_ctd.asset.model")?;
            }

            let _cond = {
                event.has_value("json.last_entity_seen")
                    && event.get_str("json.last_entity_seen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_entity_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("claroty_ctd.asset.last_entity_seen", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_entity_seen".into(),
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

            if event.has_value("json.ipv4") {
                event.rename("json.ipv4", "claroty_ctd.asset.ipv4")?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.ipv4")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.ipv4", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_destination_entity_ipv4_to_ip",
                        )?;
                        event.remove("_ingest._value");
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.ipv4")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("claroty_ctd.asset.ipv4").cloned();
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
                            let _cond = { event.has_value("claroty_ctd.asset.ipv4") };
                            if _cond {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                            }
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
                            "claroty_ctd.asset.ipv4",
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
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("claroty_ctd.asset.children").cloned();
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
                            let _cond = {
                                event.has_value("json.first_seen")
                                    && event.get_str("json.first_seen") != Some("")
                            };
                            if _cond {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.first_seen")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set("@timestamp", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.first_seen".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_children_first_seen",
                                    )?;
                                    event.remove("_ingest._value.first_seen");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                            }
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
                            "claroty_ctd.asset.children",
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
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("claroty_ctd.asset.children").cloned();
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
                            let _cond = {
                                event.has_value("json.last_entity_seen")
                                    && event.get_str("json.last_entity_seen") != Some("")
                            };
                            if _cond {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.last_entity_seen")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event
                                                .set("_ingest._value.last_entity_seen", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.last_entity_seen".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_children_last_entity_seen",
                                    )?;
                                    event.remove("_ingest._value.last_entity_seen");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                            }
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
                            "claroty_ctd.asset.children",
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
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("claroty_ctd.asset.children").cloned();
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
                            let _cond = {
                                event.has_value("json.last_seen")
                                    && event.get_str("json.last_seen") != Some("")
                            };
                            if _cond {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.last_seen")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.last_seen", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.last_seen".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_children_last_seen",
                                    )?;
                                    event.remove("_ingest._value.last_seen");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                            }
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
                            "claroty_ctd.asset.children",
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
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("claroty_ctd.asset.children").cloned();
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
                            let _cond = {
                                event.has_value("json.last_updated")
                                    && event.get_str("json.last_updated") != Some("")
                            };
                            if _cond {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.last_updated")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.last_updated", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.last_updated".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_children_last_updated",
                                    )?;
                                    event.remove("_ingest._value.last_updated");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                            }
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
                            "claroty_ctd.asset.children",
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
                event
                    .get("claroty_ctd.asset.children")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("claroty_ctd.asset.children").cloned();
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
                            let _cond = {
                                event.has_value("json.timestamp")
                                    && event.get_str("json.timestamp") != Some("")
                            };
                            if _cond {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.timestamp")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set("@timestamp", parsed)?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.timestamp".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_children_timestamp",
                                    )?;
                                    event.remove("_ingest._value.timestamp");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                            }
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
                            "claroty_ctd.asset.children",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.class_type") {
                event.rename("json.class_type", "claroty_ctd.asset.class_type")?;
            }

            if event.has_value("json.code_sections") {
                event.rename("json.code_sections", "claroty_ctd.asset.code_sections")?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.code_sections")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.code_sections", |event| {
                    if event.has_value("_ingest._value.rid") {
                        if let Some(val) = event.get("_ingest._value.rid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.rid".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.rid", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.criticality__") {
                event.rename("json.criticality__", "claroty_ctd.asset.criticalities.name")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.criticality") {
                    if let Some(val) = event.get("json.criticality") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.criticality".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.criticalities.value", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_criticality_number_to_long",
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
                .get("claroty_ctd.asset.criticalities.value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            if event.has_value("json.custom_attributes") {
                event.rename(
                    "json.custom_attributes",
                    "claroty_ctd.asset.custom_attributes",
                )?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.custom_attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.custom_attributes", |event| {
                    if event.has_value("_ingest._value.asset_id") {
                        if let Some(val) = event.get("_ingest._value.asset_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.asset_id".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.asset_id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.custom_attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.custom_attributes", |event| {
                    if event.has_value("_ingest._value.id") {
                        if let Some(val) = event.get("_ingest._value.id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.id".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.custom_attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.custom_attributes", |event| {
                    if event.has_value("_ingest._value.site_id") {
                        if let Some(val) = event.get("_ingest._value.site_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.site_id".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.site_id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.custom_attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.custom_attributes", |event| {
                    if event.has_value("_ingest._value.category.id") {
                        if let Some(val) = event.get("_ingest._value.category.id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.category.id".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.category.id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.custom_attributes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.custom_attributes", |event| {
                    if event.has_value("_ingest._value.category.site_id") {
                        if let Some(val) = event.get("_ingest._value.category.site_id") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_ingest._value.category.site_id".into(),
                                    message,
                                }
                            })?;
                            event.set("_ingest._value.category.site_id", converted)?;
                        }
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.custom_informations") {
                event.rename(
                    "json.custom_informations",
                    "claroty_ctd.asset.custom_informations",
                )?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.custom_informations")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.custom_informations", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.category") {
                            if let Some(val) = event.get("_ingest._value.category") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.category".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.category", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_custom_information_category_to_long",
                        )?;
                        event.remove("_ingest._value.category");
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.custom_informations")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.custom_informations", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.priority") {
                            if let Some(val) = event.get("_ingest._value.priority") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.priority".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.priority", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_custom_information_priority_to_long",
                        )?;
                        event.remove("_ingest._value.priority");
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

            let _cond = {
                event
                    .get("claroty_ctd.asset.custom_informations")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "claroty_ctd.asset.custom_informations", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.type") {
                            if let Some(val) = event.get("_ingest._value.type") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value.type".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value.type", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_custom_information_type_to_long",
                        )?;
                        event.remove("_ingest._value.type");
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

            if event.has_value("json.display_name") {
                event.rename("json.display_name", "claroty_ctd.asset.display_name")?;
            }

            let _cond = {
                event.has_value("json.first_seen") && event.get_str("json.first_seen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.first_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("claroty_ctd.asset.first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.first_seen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_first_seen")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ghost") {
                    if let Some(val) = event.get("json.ghost") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ghost".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.ghost", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_ghost_to_boolean",
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

            if event.has_value("json.hostname") {
                event.rename("json.hostname", "claroty_ctd.asset.hostname")?;
            }

            if let Some(v) = event
                .get("claroty_ctd.asset.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("claroty_ctd.asset.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("claroty_ctd.asset.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "claroty_ctd.asset.id")?;
            }

            if event.has_value("claroty_ctd.asset.id") {
                if let Some(val) = event.get("claroty_ctd.asset.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "claroty_ctd.asset.id".into(),
                            message,
                        }
                    })?;
                    event.set("claroty_ctd.asset.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("claroty_ctd.asset.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if event.has_value("json.insight_names") {
                event.rename("json.insight_names", "claroty_ctd.asset.insight_names")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.installed_programs_count") {
                    if let Some(val) = event.get("json.installed_programs_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.installed_programs_count".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.installed_programs_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_installed_programs_count_to_long",
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
                event.has_value("json.last_seen") && event.get_str("json.last_seen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_seen") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("claroty_ctd.asset.last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_seen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_last_seen")?;
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
                event.has_value("json.last_updated")
                    && event.get_str("json.last_updated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_updated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("claroty_ctd.asset.last_updated", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_updated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_last_updated")?;
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
                .get("claroty_ctd.asset.last_updated")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.project_parsed.creation_time") {
                    if let Some(val) = event.get("json.project_parsed.creation_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.project_parsed.creation_time".into(),
                                message,
                            }
                        })?;
                        event.set("json.project_parsed.creation_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_project_parsed_creation_time_to_long",
                )?;
                event.remove("json.project_parsed.creation_time");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.project_parsed.modification_time") {
                    if let Some(val) = event.get("json.project_parsed.modification_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.project_parsed.modification_time".into(),
                                message,
                            }
                        })?;
                        event.set("json.project_parsed.modification_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_project_parsed_modification_time_to_long",
                )?;
                event.remove("json.project_parsed.modification_time");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.project_parsed.priority") {
                    if let Some(val) = event.get("json.project_parsed.priority") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.project_parsed.priority".into(),
                                message,
                            }
                        })?;
                        event.set("json.project_parsed.priority", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_project_parsed_priority_to_long",
                )?;
                event.remove("json.project_parsed.priority");
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

            if event.has_value("json.mac") {
                event.rename("json.mac", "claroty_ctd.asset.mac")?;
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.mac")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("claroty_ctd.asset.mac").cloned();
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
                                let _cond = { event.has_value("claroty_ctd.asset.mac") };
                                if _cond {
                                    event.append_unique(
                                        "host.mac",
                                        json!(
                                            event
                                                .get("_ingest._value")
                                                .map_or_else(String::new, template_to_string)
                                        ),
                                    )?;
                                }
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
                                "claroty_ctd.asset.mac",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get("host.mac").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "host.mac", |event| {
                        if event.has_value("_ingest._value") {
                            gsub_field(
                                event,
                                "_ingest._value",
                                "_ingest._value",
                                cached_regex!(":"),
                                "-",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("claroty_ctd.asset.mac")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "host.mac", |event| {
                        if event.has_value("_ingest._value") {
                            map_strings(
                                event,
                                "_ingest._value",
                                "_ingest._value",
                                str::to_uppercase,
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.name") {
                event.rename("json.name", "claroty_ctd.asset.name")?;
            }

            if let Some(v) = event
                .get("claroty_ctd.asset.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if event.has_value("host.name") {
                map_strings(event, "host.name", "host.name", str::to_lowercase)?;
            }

            if event.has_value("json.network.id") {
                event.rename("json.network.id", "claroty_ctd.asset.network.id")?;
            }

            if event.has_value("claroty_ctd.asset.network.id") {
                if let Some(val) = event.get("claroty_ctd.asset.network.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "claroty_ctd.asset.network.id".into(),
                            message,
                        }
                    })?;
                    event.set("claroty_ctd.asset.network.id", converted)?;
                }
            }

            if event.has_value("json.network.name") {
                event.rename("json.network.name", "claroty_ctd.asset.network.name")?;
            }

            if let Some(v) = event
                .get("claroty_ctd.asset.network.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.name", v)?;
            }

            if event.has_value("json.network.resource_id") {
                event.rename(
                    "json.network.resource_id",
                    "claroty_ctd.asset.network.resource_id",
                )?;
            }

            if event.has_value("json.network.site_id") {
                event.rename("json.network.site_id", "claroty_ctd.asset.network.site_id")?;
            }

            if event.has_value("claroty_ctd.asset.network.site_id") {
                if let Some(val) = event.get("claroty_ctd.asset.network.site_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "claroty_ctd.asset.network.site_id".into(),
                            message,
                        }
                    })?;
                    event.set("claroty_ctd.asset.network.site_id", converted)?;
                }
            }

            if event.has_value("json.network_id") {
                event.rename("json.network_id", "claroty_ctd.asset.network_id")?;
            }

            if event.has_value("claroty_ctd.asset.network_id") {
                if let Some(val) = event.get("claroty_ctd.asset.network_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "claroty_ctd.asset.network_id".into(),
                            message,
                        }
                    })?;
                    event.set("claroty_ctd.asset.network_id", converted)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.num_alerts") {
                    if let Some(val) = event.get("json.num_alerts") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.num_alerts".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.num_alerts", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_num_alerts_to_long",
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
                if event.has_value("json.parsed") {
                    if let Some(val) = event.get("json.parsed") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.parsed".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.parsed", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_parsed_to_boolean",
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
                if event.has_value("json.patch_count") {
                    if let Some(val) = event.get("json.patch_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.patch_count".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.patch_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_patch_count_to_long",
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

            if event.has_value("json.installed_antivirus") {
                event.rename(
                    "json.installed_antivirus",
                    "claroty_ctd.asset.installed_antivirus",
                )?;
            }

            if event.has_value("json.domain_workgroup") {
                event.rename(
                    "json.domain_workgroup",
                    "claroty_ctd.asset.domain_workgroup",
                )?;
            }

            if event.has_value("json.protocol") {
                event.rename("json.protocol", "claroty_ctd.asset.protocol")?;
            }

            if let Some(v) = event
                .get("claroty_ctd.asset.protocol")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.purdue_level") {
                    if let Some(val) = event.get("json.purdue_level") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.purdue_level".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.purdue_level", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_purdue_level_to_double",
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

            if event.has_value("json.resource_id") {
                event.rename("json.resource_id", "claroty_ctd.asset.resource_id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.risk_level") {
                    if let Some(val) = event.get("json.risk_level") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.risk_level".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.risk_level", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_risk_level_to_long",
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

            if event.has_value("json.site_id") {
                event.rename("json.site_id", "claroty_ctd.asset.site_id")?;
            }

            if event.has_value("claroty_ctd.asset.site_id") {
                if let Some(val) = event.get("claroty_ctd.asset.site_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "claroty_ctd.asset.site_id".into(),
                            message,
                        }
                    })?;
                    event.set("claroty_ctd.asset.site_id", converted)?;
                }
            }

            if event.has_value("json.site_name") {
                event.rename("json.site_name", "claroty_ctd.asset.site_name")?;
            }

            if event.has_value("json.special_hint__") {
                event.rename(
                    "json.special_hint__",
                    "claroty_ctd.asset.special_hints.name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.special_hint") {
                    if let Some(val) = event.get("json.special_hint") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.special_hint".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.special_hints.value", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_special_hint_to_long",
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

            if event.has_value("json.subnet_id") {
                event.rename("json.subnet_id", "claroty_ctd.asset.subnet_id")?;
            }

            if event.has_value("json.project_parsed") {
                event.rename("json.project_parsed", "claroty_ctd.asset.project_parsed")?;
            }

            if event.has_value("claroty_ctd.asset.subnet_id") {
                if let Some(val) = event.get("claroty_ctd.asset.subnet_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "claroty_ctd.asset.subnet_id".into(),
                            message,
                        }
                    })?;
                    event.set("claroty_ctd.asset.subnet_id", converted)?;
                }
            }

            let _cond = {
                event.has_value("json.timestamp") && event.get_str("json.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("claroty_ctd.asset.timestamp", parsed)?,
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.usb_devices_count") {
                    if let Some(val) = event.get("json.usb_devices_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.usb_devices_count".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.usb_devices_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_usb_devices_count_to_long",
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
                if event.has_value("json.valid") {
                    if let Some(val) = event.get("json.valid") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.valid".into(),
                                message,
                            }
                        })?;
                        event.set("claroty_ctd.asset.valid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_valid_to_boolean",
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

            if event.has_value("json.vendor") {
                event.rename("json.vendor", "claroty_ctd.asset.vendor")?;
            }

            if let Some(v) = event
                .get("claroty_ctd.asset.vendor")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.manufacturer", v)?;
            }

            if event.has_value("json.virtual_zone_id") {
                event.rename("json.virtual_zone_id", "claroty_ctd.asset.virtual_zone.id")?;
            }

            if event.has_value("json.edge_id") {
                event.rename("json.edge_id", "claroty_ctd.asset.edge_id")?;
            }

            if event.has_value("json.default_gateway") {
                event.rename("json.default_gateway", "claroty_ctd.asset.default_gateway")?;
            }

            if event.has_value("claroty_ctd.asset.virtual_zone.id") {
                if let Some(val) = event.get("claroty_ctd.asset.virtual_zone.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "claroty_ctd.asset.virtual_zone.id".into(),
                            message,
                        }
                    })?;
                    event.set("claroty_ctd.asset.virtual_zone.id", converted)?;
                }
            }

            if event.has_value("claroty_ctd.asset.edge_id") {
                if let Some(val) = event.get("claroty_ctd.asset.edge_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "claroty_ctd.asset.edge_id".into(),
                            message,
                        }
                    })?;
                    event.set("claroty_ctd.asset.edge_id", converted)?;
                }
            }

            if event.has_value("json.virtual_zone_name") {
                event.rename(
                    "json.virtual_zone_name",
                    "claroty_ctd.asset.virtual_zone.name",
                )?;
            }

            if event.has_value("json.vlan") {
                event.rename("json.vlan", "claroty_ctd.asset.vlan")?;
            }

            if let Some(v) = event
                .get("claroty_ctd.asset.vlan")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.vlan.id", v)?;
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
                event.remove("claroty_ctd.asset.criticalities.value");
                event.remove("claroty_ctd.asset.hostname");
                event.remove("claroty_ctd.asset.id");
                event.remove("claroty_ctd.asset.last_updated");
                event.remove("claroty_ctd.asset.mac");
                event.remove("claroty_ctd.asset.name");
                event.remove("claroty_ctd.asset.network.name");
                event.remove("claroty_ctd.asset.protocol");
                event.remove("claroty_ctd.asset.vendor");
                event.remove("claroty_ctd.asset.vlan");
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
