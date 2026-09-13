// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_prior_asset` pipeline.
pub struct PipelinePriorAsset;

impl Transform for PipelinePriorAsset {
    fn name(&self) -> &str {
        "pipeline_prior_asset"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.priorAsset.osInventory.osInfo.architecture") {
                    event.rename("json.priorAsset.osInventory.osInfo.architecture", "google_scc.asset.prior.os_inventory.os_info.architecture")?;
                }

                if event.has_value("json.priorAsset.osInventory.osInfo.hostname") {
                    event.rename("json.priorAsset.osInventory.osInfo.hostname", "google_scc.asset.prior.os_inventory.os_info.hostname")?;
                }

                if event.has_value("json.priorAsset.name") {
                    event.rename("json.priorAsset.name", "google_scc.asset.prior.name")?;
                }

                if event.has_value("json.priorAsset.osInventory.osInfo.shortName") {
                    event.rename("json.priorAsset.osInventory.osInfo.shortName", "google_scc.asset.prior.os_inventory.os_info.short_name")?;
                }

                if event.has_value("json.priorAsset.osInventory.osInfo.longName") {
                    event.rename("json.priorAsset.osInventory.osInfo.longName", "google_scc.asset.prior.os_inventory.os_info.long_name")?;
                }

                if event.has_value("json.priorAsset.osInventory.osInfo.kernelVersion") {
                    event.rename("json.priorAsset.osInventory.osInfo.kernelVersion", "google_scc.asset.prior.os_inventory.os_info.kernel.version")?;
                }

                if event.has_value("json.priorAsset.osInventory.osInfo.version") {
                    event.rename("json.priorAsset.osInventory.osInfo.version", "google_scc.asset.prior.os_inventory.os_info.version")?;
                }

                if event.has_value("json.priorAsset.assetType") {
                    event.rename("json.priorAsset.assetType", "google_scc.asset.prior.type")?;
                }

                if event.has_value("json.priorAsset.accessLevel.basic.combiningFunction") {
                    event.rename("json.priorAsset.accessLevel.basic.combiningFunction", "json.priorAsset.accessLevel.basic.combining_function")?;
                }

                if event.has_value("json.priorAsset.accessLevel.custom.expr.expression") {
                    event.rename("json.priorAsset.accessLevel.custom.expr.expression", "json.priorAsset.accessLevel.custom.expr.text")?;
                }

                if event.has_value("json.priorAsset.accessLevel.custom.expr") {
                    event.rename("json.priorAsset.accessLevel.custom.expr", "json.priorAsset.accessLevel.custom.expression")?;
                }

            let _cond = { event.get("json.priorAsset.accessLevel.basic.conditions").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.priorAsset.accessLevel.basic.conditions").cloned();
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
                        event.set("json.priorAsset.accessLevel.basic.conditions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.priorAsset.accessLevel.basic.conditions").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.priorAsset.accessLevel.basic.conditions").cloned();
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
                        event.set("json.priorAsset.accessLevel.basic.conditions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.priorAsset.accessLevel.basic.conditions").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.priorAsset.accessLevel.basic.conditions").cloned();
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
                        event.set("json.priorAsset.accessLevel.basic.conditions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.priorAsset.accessLevel.basic.conditions").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.priorAsset.accessLevel.basic.conditions").cloned();
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
                        event.set("json.priorAsset.accessLevel.basic.conditions", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.priorAsset.accessLevel.basic.conditions") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef conditions = new ArrayList();\nfor(entity in ctx.json.priorAsset.accessLevel.basic.conditions){\n  conditions.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.accessLevel.basic.remove('conditions');\nctx.json.priorAsset.accessLevel.basic.put('conditions',conditions);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef conditions = new ArrayList();\nfor(entity in ctx.json.priorAsset.accessLevel.basic.conditions){\n  conditions.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.accessLevel.basic.remove('conditions');\nctx.json.priorAsset.accessLevel.basic.put('conditions',conditions);\n"#), cached_params!("{\"conditions\":\"conditions\",\"requiredAccessLevels\":\"required_access_levels\",\"devicePolicy\":\"device_policy\",\"requireScreenlock\":\"require_screenlock\",\"allowedEncryptionStatuses\":\"allowed_encryption_statuses\",\"osConstraints\":\"os_constraints\",\"osType\":\"os_type\",\"minimumVersion\":\"minimum_version\",\"requireVerifiedChromeOs\":\"require_verified_chrome_os\",\"allowedDeviceManagementLevels\":\"allowed_device_management_levels\",\"requireAdminApproval\":\"require_admin_approval\",\"requireCorpOwned\":\"require_corp_owned\",\"ipSubnetworks\":\"sub_networks\"}"))?;
            }

                if event.has_value("json.priorAsset.accessLevel") {
                    event.rename("json.priorAsset.accessLevel", "google_scc.asset.prior.access_level")?;
                }

                if event.has_value("json.priorAsset.accessPolicy") {
                    event.rename("json.priorAsset.accessPolicy", "google_scc.asset.prior.access_policy")?;
                }

                if event.has_value("json.priorAsset.ancestors") {
                    event.rename("json.priorAsset.ancestors", "google_scc.asset.prior.ancestors")?;
                }

            let _cond = { event.has_value("json.priorAsset.iamPolicy.auditConfigs") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef audit_configs = new ArrayList();\nfor(entity in ctx.json.priorAsset.iamPolicy.auditConfigs){\n  audit_configs.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.iamPolicy.remove('auditConfigs');\nctx.json.priorAsset.iamPolicy.put('audit_configs',audit_configs);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef audit_configs = new ArrayList();\nfor(entity in ctx.json.priorAsset.iamPolicy.auditConfigs){\n  audit_configs.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.iamPolicy.remove('auditConfigs');\nctx.json.priorAsset.iamPolicy.put('audit_configs',audit_configs);\n"#), cached_params!("{\"auditConfigs\":\"audit_configs\",\"auditLogConfigs\":\"audit_log_configs\",\"logType\":\"log_type\",\"exemptedMembers\":\"exemted_members\"}"))?;
            }

            let _cond = { event.get_str("json.priorAsset.iamPolicy.version") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.priorAsset.iamPolicy.version") {
                if let Some(val) = event.get("json.priorAsset.iamPolicy.version") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.priorAsset.iamPolicy.version".into(),
                            message,
                        })?;
                    event.set("json.priorAsset.iamPolicy.version", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_priorAsset_iamPolicy_version")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.priorAsset.iamPolicy") {
                    event.rename("json.priorAsset.iamPolicy", "google_scc.asset.prior.iam_policy")?;
                }

            let _cond = { event.get("json.priorAsset.orgPolicy").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.priorAsset.orgPolicy").cloned();
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
                            event.set("_ingest.on_failure_processor_tag", "convert_priorAsset_orgPolicy_version")?;
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
                        event.set("json.priorAsset.orgPolicy", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.priorAsset.orgPolicy").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.priorAsset.orgPolicy").cloned();
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
                        event.set("json.priorAsset.orgPolicy", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.priorAsset.orgPolicy").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.priorAsset.orgPolicy").cloned();
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
                        event.set("json.priorAsset.orgPolicy", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.get("json.priorAsset.orgPolicy").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.priorAsset.orgPolicy").cloned();
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
                        event.set("json.priorAsset.orgPolicy", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
                Ok(())
            })();
            }

            let _cond = { event.has_value("json.priorAsset.orgPolicy") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef organization_policy = new ArrayList();\nfor(entity in ctx.json.priorAsset.orgPolicy){\n  organization_policy.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.remove('orgPolicy');\nctx.google_scc.asset.prior.put('organization_policy',organization_policy);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef organization_policy = new ArrayList();\nfor(entity in ctx.json.priorAsset.orgPolicy){\n  organization_policy.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.remove('orgPolicy');\nctx.google_scc.asset.prior.put('organization_policy',organization_policy);\n"#), cached_params!("{\"orgPolicy\":\"organization_policy\",\"listPolicy\":\"list_policy\",\"allowedValues\":\"allowed_values\",\"deniedValues\":\"denied_values\",\"allValues\":\"all_values\",\"suggestedValue\":\"suggested_value\",\"restoreDefault\":\"restore_default\",\"updateTime\":\"update_time\",\"inheritFromParent\":\"inherit_from_parent\",\"booleanPolicy\":\"boolean_policy\"}"))?;
            }

                if event.has_value("json.priorAsset.osInventory.name") {
                    event.rename("json.priorAsset.osInventory.name", "google_scc.asset.prior.os_inventory.name")?;
                }

                if event.has_value("json.priorAsset.osInventory.osInfo.kernelRelease") {
                    event.rename("json.priorAsset.osInventory.osInfo.kernelRelease", "google_scc.asset.prior.os_inventory.os_info.kernel.release")?;
                }

                if event.has_value("json.priorAsset.osInventory.osInfo.osconfigAgentVersion") {
                    event.rename("json.priorAsset.osInventory.osInfo.osconfigAgentVersion", "google_scc.asset.prior.os_inventory.os_info.os_config_agent_version")?;
                }

            let _cond = { event.has_value("json.priorAsset.osInventory.updateTime") && event.get_str("json.priorAsset.osInventory.updateTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.priorAsset.osInventory.updateTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("google_scc.asset.prior.os_inventory.update_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.priorAsset.osInventory.updateTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_priorAsset_osInventory_updateTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.priorAsset.relatedAsset.ancestors") {
                    event.rename("json.priorAsset.relatedAsset.ancestors", "google_scc.asset.prior.related_asset.ancestors")?;
                }

                if event.has_value("json.priorAsset.relatedAsset.asset") {
                    event.rename("json.priorAsset.relatedAsset.asset", "google_scc.asset.prior.related_asset.name")?;
                }

                if event.has_value("json.priorAsset.relatedAsset.relationshipType") {
                    event.rename("json.priorAsset.relatedAsset.relationshipType", "google_scc.asset.prior.related_asset.relationship_type")?;
                }

                if event.has_value("json.priorAsset.relatedAsset.assetType") {
                    event.rename("json.priorAsset.relatedAsset.assetType", "google_scc.asset.prior.related_asset.type")?;
                }

                if event.has_value("json.priorAsset.relatedAssets.relationshipAttributes.action") {
                    event.rename("json.priorAsset.relatedAssets.relationshipAttributes.action", "json.priorAsset.relatedAssets.relationship_attributes.action")?;
                }

                if event.has_value("json.priorAsset.relatedAssets.relationshipAttributes.sourceResourceType") {
                    event.rename("json.priorAsset.relatedAssets.relationshipAttributes.sourceResourceType", "json.priorAsset.relatedAssets.relationship_attributes.source_resource_type")?;
                }

                if event.has_value("json.priorAsset.relatedAssets.relationshipAttributes.targetResourceType") {
                    event.rename("json.priorAsset.relatedAssets.relationshipAttributes.targetResourceType", "json.priorAsset.relatedAssets.relationship_attributes.target_resource_type")?;
                }

                if event.has_value("json.priorAsset.relatedAssets.relationshipAttributes.type") {
                    event.rename("json.priorAsset.relatedAssets.relationshipAttributes.type", "json.priorAsset.relatedAssets.relationship_attributes.type")?;
                }

            let _cond = { event.has_value("json.priorAsset.relatedAssets.assets") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef assets = new ArrayList();\nfor(entity in ctx.json.priorAsset.relatedAssets.assets){\n  assets.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.relatedAssets.put('assets',assets);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef assets = new ArrayList();\nfor(entity in ctx.json.priorAsset.relatedAssets.assets){\n  assets.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.relatedAssets.put('assets',assets);\n"#), cached_params!("{\"assetType\":\"type\",\"asset\":\"name\",\"relationshipType\":\"relationship_type\"}"))?;
            }

                if event.has_value("json.priorAsset.relatedAssets") {
                    event.rename("json.priorAsset.relatedAssets", "google_scc.asset.prior.related_assets")?;
                }

                if event.has_value("json.priorAsset.resource.discoveryDocumentUri") {
                    event.rename("json.priorAsset.resource.discoveryDocumentUri", "google_scc.asset.prior.resource.discovery.document_uri")?;
                }

                if event.has_value("json.priorAsset.resource.discoveryName") {
                    event.rename("json.priorAsset.resource.discoveryName", "google_scc.asset.prior.resource.discovery.name")?;
                }

                if event.has_value("json.priorAsset.resource.location") {
                    event.rename("json.priorAsset.resource.location", "google_scc.asset.prior.resource.location")?;
                }

                if event.has_value("json.priorAsset.resource.parent") {
                    event.rename("json.priorAsset.resource.parent", "google_scc.asset.prior.resource.parent")?;
                }

                if event.has_value("json.priorAsset.resource.resourceUrl") {
                    event.rename("json.priorAsset.resource.resourceUrl", "google_scc.asset.prior.resource.url")?;
                }

                if event.has_value("json.priorAsset.resource.version") {
                    event.rename("json.priorAsset.resource.version", "google_scc.asset.prior.resource.version")?;
                }

                if event.has_value("json.priorAsset.resource.data") {
                    event.rename("json.priorAsset.resource.data", "google_scc.asset.prior.resource.data")?;
                }

                if event.has_value("json.priorAsset.servicePerimeter.spec.accessLevels") {
                    event.rename("json.priorAsset.servicePerimeter.spec.accessLevels", "json.priorAsset.servicePerimeter.spec.access_levels")?;
                }

                if event.has_value("json.priorAsset.servicePerimeter.spec.restrictedServices") {
                    event.rename("json.priorAsset.servicePerimeter.spec.restrictedServices", "json.priorAsset.servicePerimeter.spec.restricted_services")?;
                }

                if event.has_value("json.priorAsset.servicePerimeter.spec.vpcAccessibleServices.allowedServices") {
                    event.rename("json.priorAsset.servicePerimeter.spec.vpcAccessibleServices.allowedServices", "json.priorAsset.servicePerimeter.spec.vpc_accessible_services.allowed_services")?;
                }

            let _cond = { event.get_str("json.priorAsset.servicePerimeter.spec.vpcAccessibleServices.enableRestriction") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.priorAsset.servicePerimeter.spec.vpcAccessibleServices.enableRestriction") {
                if let Some(val) = event.get("json.priorAsset.servicePerimeter.spec.vpcAccessibleServices.enableRestriction") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.priorAsset.servicePerimeter.spec.vpcAccessibleServices.enableRestriction".into(),
                            message,
                        })?;
                    event.set("json.priorAsset.servicePerimeter.spec.vpc_accessible_services.enable_restriction", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_priorAsset_servicePerimeter_spec_vpcAccessibleServices_enableRestriction_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.priorAsset.servicePerimeter.status.restrictedServices") {
                    event.rename("json.priorAsset.servicePerimeter.status.restrictedServices", "json.priorAsset.servicePerimeter.status.restricted_services")?;
                }

                if event.has_value("json.priorAsset.servicePerimeter.status.vpcAccessibleServices.allowedServices") {
                    event.rename("json.priorAsset.servicePerimeter.status.vpcAccessibleServices.allowedServices", "json.priorAsset.servicePerimeter.status.vpc_accessible_services.allowed_services")?;
                }

            let _cond = { event.get_str("json.priorAsset.servicePerimeter.status.vpcAccessibleServices.enableRestriction") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.priorAsset.servicePerimeter.status.vpcAccessibleServices.enableRestriction") {
                if let Some(val) = event.get("json.priorAsset.servicePerimeter.status.vpcAccessibleServices.enableRestriction") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.priorAsset.servicePerimeter.status.vpcAccessibleServices.enableRestriction".into(),
                            message,
                        })?;
                    event.set("json.priorAsset.servicePerimeter.status.vpc_accessible_services.enable_restriction", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_priorAsset_servicePerimeter_status_vpcAccessibleServices_enableRestriction_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.priorAsset.servicePerimeter.status.accessLevels") {
                    event.rename("json.priorAsset.servicePerimeter.status.accessLevels", "json.priorAsset.servicePerimeter.status.access_levels")?;
                }

                if event.has_value("json.priorAsset.servicePerimeter.perimeterType") {
                    event.rename("json.priorAsset.servicePerimeter.perimeterType", "json.priorAsset.servicePerimeter.type")?;
                }

            let _cond = { event.get_str("json.priorAsset.servicePerimeter.useExplicitDryRunSpec") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.priorAsset.servicePerimeter.useExplicitDryRunSpec") {
                if let Some(val) = event.get("json.priorAsset.servicePerimeter.useExplicitDryRunSpec") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.priorAsset.servicePerimeter.useExplicitDryRunSpec".into(),
                            message,
                        })?;
                    event.set("json.priorAsset.servicePerimeter.use_explicit_dry_run_spec", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_priorAsset_servicePerimeter_useExplicitDryRunSpec_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("json.priorAsset.servicePerimeter.status.egressPolicies") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef egress_policies = new ArrayList();\nfor(entity in ctx.json.priorAsset.servicePerimeter.status.egressPolicies){\n  egress_policies.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.servicePerimeter.status.remove('egressPolicies');\nctx.json.priorAsset.servicePerimeter.status.put('egress_policies',egress_policies);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef egress_policies = new ArrayList();\nfor(entity in ctx.json.priorAsset.servicePerimeter.status.egressPolicies){\n  egress_policies.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.servicePerimeter.status.remove('egressPolicies');\nctx.json.priorAsset.servicePerimeter.status.put('egress_policies',egress_policies);\n"#), cached_params!("{\"egressPolicies\":\"egress_policies\",\"egressFrom\":\"egress_from\",\"accessLevel\":\"access_level\",\"identityType\":\"identity_type\",\"egressTo\":\"egress_to\",\"methodSelectors\":\"method_selectors\",\"serviceName\":\"service_name\"}"))?;
            }

            let _cond = { event.has_value("json.priorAsset.servicePerimeter.spec.egressPolicies") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef egress_policies = new ArrayList();\nfor(entity in ctx.json.priorAsset.servicePerimeter.spec.egressPolicies){\n  egress_policies.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.servicePerimeter.spec.remove('egressPolicies');\nctx.json.priorAsset.servicePerimeter.spec.put('egress_policies',egress_policies);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef egress_policies = new ArrayList();\nfor(entity in ctx.json.priorAsset.servicePerimeter.spec.egressPolicies){\n  egress_policies.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.servicePerimeter.spec.remove('egressPolicies');\nctx.json.priorAsset.servicePerimeter.spec.put('egress_policies',egress_policies);\n"#), cached_params!("{\"egressPolicies\":\"egress_policies\",\"egressFrom\":\"egress_from\",\"accessLevel\":\"access_level\",\"identityType\":\"identity_type\",\"egressTo\":\"egress_to\",\"methodSelectors\":\"method_selectors\",\"serviceName\":\"service_name\"}"))?;
            }

            let _cond = { event.has_value("json.priorAsset.servicePerimeter.spec.ingressPolicies") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef ingress_policies = new ArrayList();\nfor(entity in ctx.json.priorAsset.servicePerimeter.spec.ingressPolicies){\n  ingress_policies.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.servicePerimeter.spec.remove('ingressPolicies');\nctx.json.priorAsset.servicePerimeter.spec.put('ingress_policies',ingress_policies);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef ingress_policies = new ArrayList();\nfor(entity in ctx.json.priorAsset.servicePerimeter.spec.ingressPolicies){\n  ingress_policies.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.servicePerimeter.spec.remove('ingressPolicies');\nctx.json.priorAsset.servicePerimeter.spec.put('ingress_policies',ingress_policies);\n"#), cached_params!("{\"ingressPolicies\":\"ingress_policies\",\"ingressFrom\":\"ingress_from\",\"accessLevel\":\"access_level\",\"identityType\":\"identity_type\",\"ingressTo\":\"ingress_to\",\"methodSelectors\":\"method_selectors\",\"serviceName\":\"service_name\"}"))?;
            }

            let _cond = { event.has_value("json.priorAsset.servicePerimeter.status.ingressPolicies") };
            if _cond {
                // Painless script
                // Source: def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef ingress_policies = new ArrayList();\nfor(entity in ctx.json.priorAsset.servicePerimeter.status.ingressPolicies){\n  ingress_policies.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.servicePerimeter.status.remove('ingressPolicies');\nctx.json.priorAsset.servicePerimeter.status.put('ingress_policies',ingress_policies);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def renameKeys(Map json, Map keyMap) {\n  def updatedJson = new HashMap();\n  for (def entry: json.entrySet()) {\n    def key = entry.getKey();\n    def value = entry.getValue();\n    if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        updatedJson[key] = renameKeys(value, keyMap);\n      }\n    } else if (value instanceof List) {\n      def updatedList = [];\n      for (def item: value) {\n        if (item instanceof Map) {\n          updatedList.add(renameKeys(item, keyMap));\n        } else {\n          updatedList.add(item);\n        }\n      }\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = updatedList;\n      } else {\n        updatedJson[key] = updatedList;\n      }\n    } else {\n      if (keyMap.containsKey(key)) {\n        updatedJson[keyMap[key]] = value;\n      } else {\n        updatedJson[key] = value;\n      }\n    }\n  }\n  return updatedJson;\n}\n\ndef ingress_policies = new ArrayList();\nfor(entity in ctx.json.priorAsset.servicePerimeter.status.ingressPolicies){\n  ingress_policies.add(renameKeys(entity, params));\n}\nctx.json.priorAsset.servicePerimeter.status.remove('ingressPolicies');\nctx.json.priorAsset.servicePerimeter.status.put('ingress_policies',ingress_policies);\n"#), cached_params!("{\"ingressPolicies\":\"ingress_policies\",\"ingressFrom\":\"ingress_from\",\"accessLevel\":\"access_level\",\"identityType\":\"identity_type\",\"ingressTo\":\"ingress_to\",\"methodSelectors\":\"method_selectors\",\"serviceName\":\"service_name\"}"))?;
            }

                if event.has_value("json.priorAsset.servicePerimeter") {
                    event.rename("json.priorAsset.servicePerimeter", "google_scc.asset.prior.service_perimeter")?;
                }

            let _cond = { event.has_value("json.priorAsset.updateTime") && event.get_str("json.priorAsset.updateTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.priorAsset.updateTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("google_scc.asset.prior.update_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.priorAsset.updateTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_priorAsset_updateTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                event.remove("google_scc.asset.prior.service_perimeter.spec.vpcAccessibleServices.enableRestriction");
                event.remove("google_scc.asset.prior.service_perimeter.status.vpcAccessibleServices.enableRestriction");
                event.remove("google_scc.asset.prior.service_perimeter.useExplicitDryRunSpec");

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
