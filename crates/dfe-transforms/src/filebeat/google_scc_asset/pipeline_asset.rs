// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_asset` pipeline.
pub struct PipelineAsset;

impl Transform for PipelineAsset {
    fn name(&self) -> &str {
        "pipeline_asset"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.asset.osInventory.osInfo.architecture") {
                    event.rename("json.asset.osInventory.osInfo.architecture", "google_scc.asset.os_inventory.os_info.architecture")?;
                }

                if event.has_value("json.asset.osInventory.osInfo.hostname") {
                    event.rename("json.asset.osInventory.osInfo.hostname", "google_scc.asset.os_inventory.os_info.hostname")?;
                }

                if event.has_value("json.asset.name") {
                    event.rename("json.asset.name", "google_scc.asset.name")?;
                }

                if event.has_value("json.asset.osInventory.osInfo.shortName") {
                    event.rename("json.asset.osInventory.osInfo.shortName", "google_scc.asset.os_inventory.os_info.short_name")?;
                }

                if event.has_value("json.asset.osInventory.osInfo.longName") {
                    event.rename("json.asset.osInventory.osInfo.longName", "google_scc.asset.os_inventory.os_info.long_name")?;
                }

                if event.has_value("json.asset.osInventory.osInfo.kernelVersion") {
                    event.rename("json.asset.osInventory.osInfo.kernelVersion", "google_scc.asset.os_inventory.os_info.kernel.version")?;
                }

                if event.has_value("json.asset.osInventory.osInfo.version") {
                    event.rename("json.asset.osInventory.osInfo.version", "google_scc.asset.os_inventory.os_info.version")?;
                }

                if event.has_value("json.asset.assetType") {
                    event.rename("json.asset.assetType", "google_scc.asset.type")?;
                }

                if event.has_value("json.asset.accessLevel.basic.combiningFunction") {
                    event.rename("json.asset.accessLevel.basic.combiningFunction", "json.asset.accessLevel.basic.combining_function")?;
                }

                if event.has_value("json.asset.accessLevel.custom.expr.expression") {
                    event.rename("json.asset.accessLevel.custom.expr.expression", "json.asset.accessLevel.custom.expr.text")?;
                }

                if event.has_value("json.asset.accessLevel.custom.expr") {
                    event.rename("json.asset.accessLevel.custom.expr", "json.asset.accessLevel.custom.expression")?;
                }

            let _cond = { event.get("json.asset.accessLevel.basic.conditions").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.asset.accessLevel.basic.conditions").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject = event.get("_ingest._value.devicePolicy.osConstraints").cloned();
                            let keyed = matches!(subject, Some(Value::Object(_)));
                            let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                            Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.requireVerifiedChromeOs") {
                            if let Some(val) = event.get("_ingest._value.requireVerifiedChromeOs") {
                            let converted = convert_value(val, "boolean")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.requireVerifiedChromeOs".into(),
                            message,
                            })?;
                            event.set("_ingest._value.requireVerifiedChromeOs", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_requireVerifiedChromeOs_to_boolean")?;
                            if event.remove("_ingest._value.devicePolicy.requireCorpOwned").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.devicePolicy.requireCorpOwned".into() });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                            // An entry the body renamed AWAY is gone from the
                            // object, which is how a foreach lifts fields up.
                            Some(key) => {
                            if let Some(value) = left { fields.insert(key, value); }
                            }
                            None => list.push(left.unwrap_or(Value::Null)),
                            }
                            }
                            match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                            }
                            if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                            }
                            event.set("_ingest._value.devicePolicy.osConstraints", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                            }
                            }
                            Ok(())
                            })();
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.asset.accessLevel.basic.conditions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.asset.accessLevel.basic.conditions").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.asset.accessLevel.basic.conditions").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.devicePolicy.requireScreenlock") {
                            if let Some(val) = event.get("_ingest._value.devicePolicy.requireScreenlock") {
                            let converted = convert_value(val, "boolean")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.devicePolicy.requireScreenlock".into(),
                            message,
                            })?;
                            event.set("_ingest._value.devicePolicy.requireScreenlock", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_devicePolicy_requireScreenlock_to_boolean")?;
                            if event.remove("_ingest._value.devicePolicy.requireScreenlock").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.devicePolicy.requireScreenlock".into() });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.asset.accessLevel.basic.conditions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.asset.accessLevel.basic.conditions").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.asset.accessLevel.basic.conditions").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.devicePolicy.requireAdminApproval") {
                            if let Some(val) = event.get("_ingest._value.devicePolicy.requireAdminApproval") {
                            let converted = convert_value(val, "boolean")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.devicePolicy.requireAdminApproval".into(),
                            message,
                            })?;
                            event.set("_ingest._value.devicePolicy.requireAdminApproval", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_devicePolicy_requireAdminApproval_to_boolean")?;
                            if event.remove("_ingest._value.devicePolicy.requireAdminApproval").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.devicePolicy.requireAdminApproval".into() });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.asset.accessLevel.basic.conditions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.asset.accessLevel.basic.conditions").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.asset.accessLevel.basic.conditions").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.negate") {
                            if let Some(val) = event.get("_ingest._value.negate") {
                            let converted = convert_value(val, "boolean")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.negate".into(),
                            message,
                            })?;
                            event.set("_ingest._value.negate", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_negate_to_boolean")?;
                            if event.remove("_ingest._value.negate").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.negate".into() });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.asset.accessLevel.basic.conditions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.asset.accessLevel.basic.conditions") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef conditions = new ArrayList();\nfor(entity in ctx.json.asset.accessLevel.basic.conditions){\n  conditions.add(renameKeys(entity, params));\n}\nctx.json.asset.accessLevel.basic.remove('conditions');\nctx.json.asset.accessLevel.basic.put('conditions',conditions);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef conditions = new ArrayList();\nfor(entity in ctx.json.asset.accessLevel.basic.conditions){\n  conditions.add(renameKeys(entity, params));\n}\nctx.json.asset.accessLevel.basic.remove('conditions');\nctx.json.asset.accessLevel.basic.put('conditions',conditions);\n"#), cached_params!("{\"conditions\":\"conditions\",\"requiredAccessLevels\":\"required_access_levels\",\"devicePolicy\":\"device_policy\",\"requireScreenlock\":\"require_screenlock\",\"allowedEncryptionStatuses\":\"allowed_encryption_statuses\",\"osConstraints\":\"os_constraints\",\"osType\":\"os_type\",\"minimumVersion\":\"minimum_version\",\"requireVerifiedChromeOs\":\"require_verified_chrome_os\",\"allowedDeviceManagementLevels\":\"allowed_device_management_levels\",\"requireAdminApproval\":\"require_admin_approval\",\"requireCorpOwned\":\"require_corp_owned\",\"ipSubnetworks\":\"sub_networks\"}"))?;
            }

                if event.has_value("json.asset.accessLevel") {
                    event.rename("json.asset.accessLevel", "google_scc.asset.access_level")?;
                }

                if event.has_value("json.asset.accessPolicy") {
                    event.rename("json.asset.accessPolicy", "google_scc.asset.access_policy")?;
                }

                if event.has_value("json.asset.ancestors") {
                    event.rename("json.asset.ancestors", "google_scc.asset.ancestors")?;
                }

            let _cond = { event.has_value("json.asset.iamPolicy.auditConfigs") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef audit_configs = new ArrayList();\nfor(entity in ctx.json.asset.iamPolicy.auditConfigs){\n  audit_configs.add(renameKeys(entity, params));\n}\nctx.json.asset.iamPolicy.remove('auditConfigs');\nctx.json.asset.iamPolicy.put('audit_configs',audit_configs);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef audit_configs = new ArrayList();\nfor(entity in ctx.json.asset.iamPolicy.auditConfigs){\n  audit_configs.add(renameKeys(entity, params));\n}\nctx.json.asset.iamPolicy.remove('auditConfigs');\nctx.json.asset.iamPolicy.put('audit_configs',audit_configs);\n"#), cached_params!("{\"auditConfigs\":\"audit_configs\",\"auditLogConfigs\":\"audit_log_configs\",\"logType\":\"log_type\",\"exemptedMembers\":\"exempted_members\"}"))?;
            }

            let _cond = { event.get_str("json.asset.iamPolicy.version") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.asset.iamPolicy.version") {
                if let Some(val) = event.get("json.asset.iamPolicy.version") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.asset.iamPolicy.version".into(),
                            message,
                        })?;
                    event.set("json.asset.iamPolicy.version", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_asset_iamPolicy_version")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.asset.iamPolicy") {
                    event.rename("json.asset.iamPolicy", "google_scc.asset.iam_policy")?;
                }

            let _cond = { event.get("json.asset.orgPolicy").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.asset.orgPolicy").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.version") {
                            if let Some(val) = event.get("_ingest._value.version") {
                            let converted = convert_value(val, "string")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.version".into(),
                            message,
                            })?;
                            event.set("_ingest._value.version", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_asset_orgPolicy_version")?;
                            if event.remove("_ingest._value.version").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.version".into() });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.asset.orgPolicy", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.asset.orgPolicy").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.asset.orgPolicy").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if let Some(date_str) = event.get_as_string("_ingest._value.updateTime") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("_ingest._value.updateTime", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.updateTime".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "date")?;
                            event.set("_ingest.on_failure_processor_tag", "date_updateTime")?;
                            event.remove("_ingest._value.updateTime");
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.asset.orgPolicy", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.asset.orgPolicy").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.asset.orgPolicy").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.listPolicy.inheritFromParent") {
                            if let Some(val) = event.get("_ingest._value.listPolicy.inheritFromParent") {
                            let converted = convert_value(val, "boolean")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.listPolicy.inheritFromParent".into(),
                            message,
                            })?;
                            event.set("_ingest._value.listPolicy.inheritFromParent", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_orgPolicy_listPolicy_inheritFromParent_to_boolean")?;
                            if event.remove("_ingest._value.listPolicy.inheritFromParent").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.listPolicy.inheritFromParent".into() });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.asset.orgPolicy", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.asset.orgPolicy").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.asset.orgPolicy").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.booleanPolicy.enforced") {
                            if let Some(val) = event.get("_ingest._value.booleanPolicy.enforced") {
                            let converted = convert_value(val, "boolean")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.booleanPolicy.enforced".into(),
                            message,
                            })?;
                            event.set("_ingest._value.booleanPolicy.enforced", converted)?;
                            }
                            }
                            Ok(())
                            })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_orgPolicy_booleanPolicy_enforced_to_boolean")?;
                            if event.remove("_ingest._value.inbooleanPolicy.enforcedheritFromParent").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.inbooleanPolicy.enforcedheritFromParent".into() });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                            }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left { fields.insert(key, value); }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => { event.set("_ingest._value", previous)?; }
                            None => { event.remove("_ingest"); }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set("json.asset.orgPolicy", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.asset.orgPolicy") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef organization_policy = new ArrayList();\nfor(entity in ctx.json.asset.orgPolicy){\n  organization_policy.add(renameKeys(entity, params));\n}\nctx.json.asset.remove('orgPolicy');\nctx.google_scc.asset.put('organization_policy',organization_policy);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef organization_policy = new ArrayList();\nfor(entity in ctx.json.asset.orgPolicy){\n  organization_policy.add(renameKeys(entity, params));\n}\nctx.json.asset.remove('orgPolicy');\nctx.google_scc.asset.put('organization_policy',organization_policy);\n"#), cached_params!("{\"orgPolicy\":\"organization_policy\",\"listPolicy\":\"list_policy\",\"allowedValues\":\"allowed_values\",\"deniedValues\":\"denied_values\",\"allValues\":\"all_values\",\"suggestedValue\":\"suggested_value\",\"restoreDefault\":\"restore_default\",\"updateTime\":\"update_time\",\"inheritFromParent\":\"inherit_from_parent\",\"booleanPolicy\":\"boolean_policy\"}"))?;
            }

                if event.has_value("json.asset.osInventory.name") {
                    event.rename("json.asset.osInventory.name", "google_scc.asset.os_inventory.name")?;
                }

                if event.has_value("json.asset.osInventory.osInfo.kernelRelease") {
                    event.rename("json.asset.osInventory.osInfo.kernelRelease", "google_scc.asset.os_inventory.os_info.kernel.release")?;
                }

                if event.has_value("json.asset.osInventory.osInfo.osconfigAgentVersion") {
                    event.rename("json.asset.osInventory.osInfo.osconfigAgentVersion", "google_scc.asset.os_inventory.os_info.os_config_agent_version")?;
                }

            let _cond = { event.has_value("json.asset.osInventory.updateTime") && event.get_str("json.asset.osInventory.updateTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.asset.osInventory.updateTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("google_scc.asset.os_inventory.update_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.asset.osInventory.updateTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_asset_osInventory_updateTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.asset.relatedAsset.ancestors") {
                    event.rename("json.asset.relatedAsset.ancestors", "google_scc.asset.related_asset.ancestors")?;
                }

                if event.has_value("json.asset.relatedAsset.asset") {
                    event.rename("json.asset.relatedAsset.asset", "google_scc.asset.related_asset.name")?;
                }

                if event.has_value("json.asset.relatedAsset.relationshipType") {
                    event.rename("json.asset.relatedAsset.relationshipType", "google_scc.asset.related_asset.relationship_type")?;
                }

                if event.has_value("json.asset.relatedAsset.assetType") {
                    event.rename("json.asset.relatedAsset.assetType", "google_scc.asset.related_asset.type")?;
                }

                if event.has_value("json.asset.relatedAssets.relationshipAttributes.action") {
                    event.rename("json.asset.relatedAssets.relationshipAttributes.action", "json.asset.relatedAssets.relationship_attributes.action")?;
                }

                if event.has_value("json.asset.relatedAssets.relationshipAttributes.sourceResourceType") {
                    event.rename("json.asset.relatedAssets.relationshipAttributes.sourceResourceType", "json.asset.relatedAssets.relationship_attributes.source_resource_type")?;
                }

                if event.has_value("json.asset.relatedAssets.relationshipAttributes.targetResourceType") {
                    event.rename("json.asset.relatedAssets.relationshipAttributes.targetResourceType", "json.asset.relatedAssets.relationship_attributes.target_resource_type")?;
                }

                if event.has_value("json.asset.relatedAssets.relationshipAttributes.type") {
                    event.rename("json.asset.relatedAssets.relationshipAttributes.type", "json.asset.relatedAssets.relationship_attributes.type")?;
                }

            let _cond = { event.has_value("json.asset.relatedAssets.assets") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef assets = new ArrayList();\nfor(entity in ctx.json.asset.relatedAssets.assets){\n  assets.add(renameKeys(entity, params));\n}\nctx.json.asset.relatedAssets.put('assets',assets);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef assets = new ArrayList();\nfor(entity in ctx.json.asset.relatedAssets.assets){\n  assets.add(renameKeys(entity, params));\n}\nctx.json.asset.relatedAssets.put('assets',assets);\n"#), cached_params!("{\"assetType\":\"type\",\"asset\":\"name\",\"relationshipType\":\"relationship_type\"}"))?;
            }

                if event.has_value("json.asset.relatedAssets") {
                    event.rename("json.asset.relatedAssets", "google_scc.asset.related_assets")?;
                }

                if event.has_value("json.asset.resource.discoveryDocumentUri") {
                    event.rename("json.asset.resource.discoveryDocumentUri", "google_scc.asset.resource.discovery.document_uri")?;
                }

                if event.has_value("json.asset.resource.discoveryName") {
                    event.rename("json.asset.resource.discoveryName", "google_scc.asset.resource.discovery.name")?;
                }

                if event.has_value("json.asset.resource.location") {
                    event.rename("json.asset.resource.location", "google_scc.asset.resource.location")?;
                }

                if event.has_value("json.asset.resource.parent") {
                    event.rename("json.asset.resource.parent", "google_scc.asset.resource.parent")?;
                }

                if event.has_value("json.asset.resource.resourceUrl") {
                    event.rename("json.asset.resource.resourceUrl", "google_scc.asset.resource.url")?;
                }

                if event.has_value("json.asset.resource.version") {
                    event.rename("json.asset.resource.version", "google_scc.asset.resource.version")?;
                }

                if event.has_value("json.asset.resource.data") {
                    event.rename("json.asset.resource.data", "google_scc.asset.resource.data")?;
                }

                if event.has_value("json.asset.servicePerimeter.spec.accessLevels") {
                    event.rename("json.asset.servicePerimeter.spec.accessLevels", "json.asset.servicePerimeter.spec.access_levels")?;
                }

                if event.has_value("json.asset.servicePerimeter.spec.restrictedServices") {
                    event.rename("json.asset.servicePerimeter.spec.restrictedServices", "json.asset.servicePerimeter.spec.restricted_services")?;
                }

                if event.has_value("json.asset.servicePerimeter.spec.vpcAccessibleServices.allowedServices") {
                    event.rename("json.asset.servicePerimeter.spec.vpcAccessibleServices.allowedServices", "json.asset.servicePerimeter.spec.vpc_accessible_services.allowed_services")?;
                }

            let _cond = { event.get_str("json.asset.servicePerimeter.spec.vpcAccessibleServices.enableRestriction") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.asset.servicePerimeter.spec.vpcAccessibleServices.enableRestriction") {
                if let Some(val) = event.get("json.asset.servicePerimeter.spec.vpcAccessibleServices.enableRestriction") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.asset.servicePerimeter.spec.vpcAccessibleServices.enableRestriction".into(),
                            message,
                        })?;
                    event.set("json.asset.servicePerimeter.spec.vpc_accessible_services.enable_restriction", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_asset_servicePerimeter_spec_vpcAccessibleServices_enableRestriction_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.asset.servicePerimeter.status.restrictedServices") {
                    event.rename("json.asset.servicePerimeter.status.restrictedServices", "json.asset.servicePerimeter.status.restricted_services")?;
                }

                if event.has_value("json.asset.servicePerimeter.status.vpcAccessibleServices.allowedServices") {
                    event.rename("json.asset.servicePerimeter.status.vpcAccessibleServices.allowedServices", "json.asset.servicePerimeter.status.vpc_accessible_services.allowed_services")?;
                }

            let _cond = { event.get_str("json.asset.servicePerimeter.status.vpcAccessibleServices.enableRestriction") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.asset.servicePerimeter.status.vpcAccessibleServices.enableRestriction") {
                if let Some(val) = event.get("json.asset.servicePerimeter.status.vpcAccessibleServices.enableRestriction") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.asset.servicePerimeter.status.vpcAccessibleServices.enableRestriction".into(),
                            message,
                        })?;
                    event.set("json.asset.servicePerimeter.status.vpc_accessible_services.enable_restriction", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_asset_servicePerimeter_status_vpcAccessibleServices_enableRestriction_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.asset.servicePerimeter.status.accessLevels") {
                    event.rename("json.asset.servicePerimeter.status.accessLevels", "json.asset.servicePerimeter.status.access_levels")?;
                }

                if event.has_value("json.asset.servicePerimeter.perimeterType") {
                    event.rename("json.asset.servicePerimeter.perimeterType", "json.asset.servicePerimeter.type")?;
                }

            let _cond = { event.get_str("json.asset.servicePerimeter.useExplicitDryRunSpec") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.asset.servicePerimeter.useExplicitDryRunSpec") {
                if let Some(val) = event.get("json.asset.servicePerimeter.useExplicitDryRunSpec") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.asset.servicePerimeter.useExplicitDryRunSpec".into(),
                            message,
                        })?;
                    event.set("json.asset.servicePerimeter.use_explicit_dry_run_spec", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_asset_servicePerimeter_useExplicitDryRunSpec_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("json.asset.servicePerimeter.status.egressPolicies") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef egress_policies = new ArrayList();\nfor(entity in ctx.json.asset.servicePerimeter.status.egressPolicies){\n  egress_policies.add(renameKeys(entity, params));\n}\nctx.json.asset.servicePerimeter.status.remove('egressPolicies');\nctx.json.asset.servicePerimeter.status.put('egress_policies',egress_policies);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef egress_policies = new ArrayList();\nfor(entity in ctx.json.asset.servicePerimeter.status.egressPolicies){\n  egress_policies.add(renameKeys(entity, params));\n}\nctx.json.asset.servicePerimeter.status.remove('egressPolicies');\nctx.json.asset.servicePerimeter.status.put('egress_policies',egress_policies);\n"#), cached_params!("{\"egressPolicies\":\"egress_policies\",\"egressFrom\":\"egress_from\",\"accessLevel\":\"access_level\",\"identityType\":\"identity_type\",\"egressTo\":\"egress_to\",\"methodSelectors\":\"method_selectors\",\"serviceName\":\"service_name\",\"externalResources\":\"external_resources\"}"))?;
            }

            let _cond = { event.has_value("json.asset.servicePerimeter.spec.egressPolicies") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef egress_policies = new ArrayList();\nfor(entity in ctx.json.asset.servicePerimeter.spec.egressPolicies){\n  egress_policies.add(renameKeys(entity, params));\n}\nctx.json.asset.servicePerimeter.spec.remove('egressPolicies');\nctx.json.asset.servicePerimeter.spec.put('egress_policies',egress_policies);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef egress_policies = new ArrayList();\nfor(entity in ctx.json.asset.servicePerimeter.spec.egressPolicies){\n  egress_policies.add(renameKeys(entity, params));\n}\nctx.json.asset.servicePerimeter.spec.remove('egressPolicies');\nctx.json.asset.servicePerimeter.spec.put('egress_policies',egress_policies);\n"#), cached_params!("{\"egressPolicies\":\"egress_policies\",\"egressFrom\":\"egress_from\",\"accessLevel\":\"access_level\",\"identityType\":\"identity_type\",\"egressTo\":\"egress_to\",\"methodSelectors\":\"method_selectors\",\"serviceName\":\"service_name\",\"externalResources\":\"external_resources\"}"))?;
            }

            let _cond = { event.has_value("json.asset.servicePerimeter.spec.ingressPolicies") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef ingress_policies = new ArrayList();\nfor(entity in ctx.json.asset.servicePerimeter.spec.ingressPolicies){\n  ingress_policies.add(renameKeys(entity, params));\n}\nctx.json.asset.servicePerimeter.spec.remove('ingressPolicies');\nctx.json.asset.servicePerimeter.spec.put('ingress_policies',ingress_policies);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef ingress_policies = new ArrayList();\nfor(entity in ctx.json.asset.servicePerimeter.spec.ingressPolicies){\n  ingress_policies.add(renameKeys(entity, params));\n}\nctx.json.asset.servicePerimeter.spec.remove('ingressPolicies');\nctx.json.asset.servicePerimeter.spec.put('ingress_policies',ingress_policies);\n"#), cached_params!("{\"ingressPolicies\":\"ingress_policies\",\"ingressFrom\":\"ingress_from\",\"accessLevel\":\"access_level\",\"identityType\":\"identity_type\",\"ingressTo\":\"ingress_to\",\"methodSelectors\":\"method_selectors\",\"serviceName\":\"service_name\"}"))?;
            }

            let _cond = { event.has_value("json.asset.servicePerimeter.status.ingressPolicies") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef ingress_policies = new ArrayList();\nfor(entity in ctx.json.asset.servicePerimeter.status.ingressPolicies){\n  ingress_policies.add(renameKeys(entity, params));\n}\nctx.json.asset.servicePerimeter.status.remove('ingressPolicies');\nctx.json.asset.servicePerimeter.status.put('ingress_policies',ingress_policies);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef ingress_policies = new ArrayList();\nfor(entity in ctx.json.asset.servicePerimeter.status.ingressPolicies){\n  ingress_policies.add(renameKeys(entity, params));\n}\nctx.json.asset.servicePerimeter.status.remove('ingressPolicies');\nctx.json.asset.servicePerimeter.status.put('ingress_policies',ingress_policies);\n"#), cached_params!("{\"ingressPolicies\":\"ingress_policies\",\"ingressFrom\":\"ingress_from\",\"accessLevel\":\"access_level\",\"identityType\":\"identity_type\",\"ingressTo\":\"ingress_to\",\"methodSelectors\":\"method_selectors\",\"serviceName\":\"service_name\"}"))?;
            }

                if event.has_value("json.asset.servicePerimeter") {
                    event.rename("json.asset.servicePerimeter", "google_scc.asset.service_perimeter")?;
                }

            let _cond = { event.has_value("json.asset.updateTime") && event.get_str("json.asset.updateTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.asset.updateTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("google_scc.asset.update_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.asset.updateTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_asset_updateTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("google_scc.asset.service_perimeter.spec.vpcAccessibleServices.enableRestriction");
                event.remove("google_scc.asset.service_perimeter.status.vpcAccessibleServices.enableRestriction");
                event.remove("google_scc.asset.service_perimeter.useExplicitDryRunSpec");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
