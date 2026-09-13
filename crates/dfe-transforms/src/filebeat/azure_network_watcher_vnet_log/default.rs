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
            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            event.append_unique("event.category", json!("network"))?;

            event.append_unique("event.type", json!("info"))?;

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

            let _cond = { event.has_value("json.time") && event.get_str("json.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("azure_network_watcher_vnet.log.time", parsed)?
                            }
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
                .get("azure_network_watcher_vnet.log.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.flowLogVersion") {
                if let Some(val) = event.get("json.flowLogVersion") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.flowLogVersion".into(),
                            message,
                        }
                    })?;
                    event.set("json.flowLogVersion", converted)?;
                }
            }

            if event.has_value("json.flowLogVersion") {
                event.rename(
                    "json.flowLogVersion",
                    "azure_network_watcher_vnet.log.flow_log.version",
                )?;
            }

            if event.has_value("json.flowLogGUID") {
                event.rename(
                    "json.flowLogGUID",
                    "azure_network_watcher_vnet.log.flow_log.guid",
                )?;
            }

            if event.has_value("json.flowLogResourceID") {
                event.rename(
                    "json.flowLogResourceID",
                    "azure_network_watcher_vnet.log.flow_log.resource_id",
                )?;
            }

            if event.has_value("json.category") {
                event.rename("json.category", "azure_network_watcher_vnet.log.category")?;
            }

            if event.has_value("json.targetResourceID") {
                event.rename(
                    "json.targetResourceID",
                    "azure_network_watcher_vnet.log.target_resource_id",
                )?;
            }

            if event.has_value("json.operationName") {
                event.rename(
                    "json.operationName",
                    "azure_network_watcher_vnet.log.operation_name",
                )?;
            }

            let _cond = {
                event
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.flowRecords.flows", |event| {
                    if event.has_value("_ingest._value.aclID") {
                        event.rename("_ingest._value.aclID", "_ingest._value.acl_id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event.has_value("json.macAddress") && event.get_str("json.macAddress") != Some("")
            };
            if _cond {
                if event.has_value("json.macAddress") {
                    map_strings(
                        event,
                        "json.macAddress",
                        "azure_network_watcher_vnet.log.mac_address",
                        str::to_uppercase,
                    )?;
                }
            }

            let _cond = {
                event.has_value("azure_network_watcher_vnet.log.mac_address")
                    && event.get_str("azure_network_watcher_vnet.log.mac_address") != Some("")
            };
            if _cond {
                if event.has_value("azure_network_watcher_vnet.log.mac_address") {
                    gsub_field(
                        event,
                        "azure_network_watcher_vnet.log.mac_address",
                        "azure_network_watcher_vnet.log.mac_address",
                        cached_regex!("(..)(?!$)"),
                        "$1-",
                    )?;
                }
            }

            if let Some(v) = event
                .get("azure_network_watcher_vnet.log.mac_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.mac", v)?;
            }

            let _cond = {
                event
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.flowRecords.flows", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "_ingest._value.flowGroups", |event| {
                            event.append_unique(
                                "rule.name",
                                json!(
                                    event
                                        .get("_ingest._value.rule")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def removeDuplicateValues(def list) {\n    List arrayList = new ArrayList(new HashSet(list));\n    return arrayList;\n  }\n  List flowsList = ctx.json.flowRecords.flows;\n  List destinationBytes = new ArrayList();\n  List sourceBytes = new ArrayList();\n  List destinationPackets = new ArrayList();\n  List sourcePackets = new ArrayList();\n  List srcPort = new ArrayList();\n  List destinationPort = new ArrayList();\n  List destinationIp = new ArrayList();\n  List srcIp = new ArrayList();\n  List netDirection = new ArrayList();\n  List netIana = new ArrayList();\n  for (Map flows: flowsList) {\n    List flowGroupsList = flows.containsKey('flowGroups') ? (List) flows.get('flowGroups') : Collections.emptyList();\n    for (Map flowGroups: flowGroupsList) {\n      List flowTuplesList = flowGroups.containsKey('flowTuples') ? (List) flowGroups.get('flowTuples') : Collections.emptyList();\n      List newFlowTuplesList = new ArrayList();\n      for (String flowTuple: flowTuplesList) {\n        String[] flowTupleParts = flowTuple.splitOnToken(',');\n        if (flowTupleParts.length == 13) {\n          Map flowTupleMap = new HashMap();\n          flowTupleMap.put('flow', new HashMap());\n          flowTupleMap.put('source', new HashMap());\n          flowTupleMap.put('destination', new HashMap());\n          flowTupleMap.put('bytes', new HashMap());\n          flowTupleMap.put('packets', new HashMap());\n          if (!flowTupleParts[0].isEmpty()) {\n            flowTupleMap.put('timestamp', flowTupleParts[0]);\n          }\n          if (!flowTupleParts[1].isEmpty()) {\n            srcIp.add(flowTupleParts[1]);\n            flowTupleMap.source.put('ip', flowTupleParts[1]);\n          }\n          if (!flowTupleParts[2].isEmpty()) {\n            destinationIp.add(flowTupleParts[2]);\n            flowTupleMap.destination.put('ip', flowTupleParts[2]);\n          }\n          if (!flowTupleParts[3].isEmpty()) {\n            srcPort.add(flowTupleParts[3]);\n            flowTupleMap.source.put('port', flowTupleParts[3]);\n          }\n          if (!flowTupleParts[4].isEmpty()) {\n            destinationPort.add(flowTupleParts[4]);\n            flowTupleMap.destination.put('port', flowTupleParts[4]);\n          }\n          if (!flowTupleParts[5].isEmpty()) {\n            netIana.add(flowTupleParts[5]);\n            flowTupleMap.put('protocol', flowTupleParts[5]);\n          }\n          String flowDirection = flowTupleParts[6];\n          if (flowDirection.contains('I')) {\n            flowDirection = 'Inbound';\n          } else if (flowDirection.contains('O')) {\n            flowDirection = 'Outbound';\n          }\n          if (!flowTupleParts[6].isEmpty()) {\n            netDirection.add(flowDirection);\n            flowTupleMap.flow.put('direction', flowDirection);\n          }\n          String flowState = flowTupleParts[7];\n          if (flowState.contains('B')) {\n            flowState = 'Begin';\n          } else if (flowState.contains('C')) {\n            flowState = 'Continuing';\n          } else if (flowState.contains('E')) {\n            flowState = 'End';\n          } else if (flowState.contains('D')) {\n            flowState = 'Deny';\n          }\n          if (!flowTupleParts[7].isEmpty()) {\n            flowTupleMap.flow.put('state', flowState);\n          }\n          if (!flowTupleParts[8].isEmpty()) {\n            flowTupleMap.flow.put('encryption', flowTupleParts[8]);\n          }\n          if (!flowTupleParts[9].isEmpty()) {\n            sourcePackets.add(flowTupleParts[9]);\n            flowTupleMap.packets.put('sent', flowTupleParts[9]);\n          }\n          if (!flowTupleParts[10].isEmpty()) {\n            sourceBytes.add(flowTupleParts[10]);\n            flowTupleMap.bytes.put('sent', flowTupleParts[10]);\n          }\n          if (!flowTupleParts[11].isEmpty()) {\n            destinationPackets.add(flowTupleParts[11]);\n            flowTupleMap.packets.put('received', flowTupleParts[11]);\n          }\n          if (!flowTupleParts[12].isEmpty()) {\n            destinationBytes.add(flowTupleParts[12]);\n            flowTupleMap.bytes.put('received', flowTupleParts[12]);\n          }\n          newFlowTuplesList.add(flowTupleMap);\n        }\n      }\n      flowGroups.put('tuples', newFlowTuplesList);\n      flowGroups.remove('flowTuples');\n    }\n  }\n  ctx.json.flowRecords.flows = flowsList;\n  if (ctx.destination == null) {\n    Map map = new HashMap();\n    ctx.put('destination', map);\n  }\n  if (ctx.source == null) {\n    Map map = new HashMap();\n    ctx.put('source', map);\n  }\n  if (ctx.network == null) {\n    Map map = new HashMap();\n    ctx.put('network', map);\n  }\n  if (ctx.destination?.packets == null) {\n    ctx.destination.put('packets', removeDuplicateValues(destinationPackets));\n  }\n  if (ctx.destination?.bytes == null) {\n    ctx.destination.put('bytes', removeDuplicateValues(destinationBytes));\n  }\n  if (ctx.source?.packets == null) {\n    ctx.source.put('packets', removeDuplicateValues(sourcePackets));\n  }\n  if (ctx.source?.bytes == null) {\n    ctx.source.put('bytes', removeDuplicateValues(sourceBytes));\n  }\n  if (ctx.source?.port == null) {\n    ctx.source.put('port', removeDuplicateValues(srcPort));\n  }\n  if (ctx.destination?.port == null) {\n    ctx.destination.put('port', removeDuplicateValues(destinationPort));\n  }\n  if (ctx.source?.ip == null) {\n    ctx.source.put('ip', removeDuplicateValues(srcIp));\n  }\n  if (ctx.destination?.ip == null) {\n    ctx.destination.put('ip', removeDuplicateValues(destinationIp));\n  }\n  if (ctx.network?.direction == null) {\n    ctx.network.put('direction', removeDuplicateValues(netDirection));\n  }\n  if (ctx.network?.iana_number == null) {\n    ctx.network.put('iana_number', removeDuplicateValues(netIana));\n  }\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def removeDuplicateValues(def list) {\n    List arrayList = new ArrayList(new HashSet(list));\n    return arrayList;\n  }\n  List flowsList = ctx.json.flowRecords.flows;\n  List destinationBytes = new ArrayList();\n  List sourceBytes = new ArrayList();\n  List destinationPackets = new ArrayList();\n  List sourcePackets = new ArrayList();\n  List srcPort = new ArrayList();\n  List destinationPort = new ArrayList();\n  List destinationIp = new ArrayList();\n  List srcIp = new ArrayList();\n  List netDirection = new ArrayList();\n  List netIana = new ArrayList();\n  for (Map flows: flowsList) {\n    List flowGroupsList = flows.containsKey('flowGroups') ? (List) flows.get('flowGroups') : Collections.emptyList();\n    for (Map flowGroups: flowGroupsList) {\n      List flowTuplesList = flowGroups.containsKey('flowTuples') ? (List) flowGroups.get('flowTuples') : Collections.emptyList();\n      List newFlowTuplesList = new ArrayList();\n      for (String flowTuple: flowTuplesList) {\n        String[] flowTupleParts = flowTuple.splitOnToken(',');\n        if (flowTupleParts.length == 13) {\n          Map flowTupleMap = new HashMap();\n          flowTupleMap.put('flow', new HashMap());\n          flowTupleMap.put('source', new HashMap());\n          flowTupleMap.put('destination', new HashMap());\n          flowTupleMap.put('bytes', new HashMap());\n          flowTupleMap.put('packets', new HashMap());\n          if (!flowTupleParts[0].isEmpty()) {\n            flowTupleMap.put('timestamp', flowTupleParts[0]);\n          }\n          if (!flowTupleParts[1].isEmpty()) {\n            srcIp.add(flowTupleParts[1]);\n            flowTupleMap.source.put('ip', flowTupleParts[1]);\n          }\n          if (!flowTupleParts[2].isEmpty()) {\n            destinationIp.add(flowTupleParts[2]);\n            flowTupleMap.destination.put('ip', flowTupleParts[2]);\n          }\n          if (!flowTupleParts[3].isEmpty()) {\n            srcPort.add(flowTupleParts[3]);\n            flowTupleMap.source.put('port', flowTupleParts[3]);\n          }\n          if (!flowTupleParts[4].isEmpty()) {\n            destinationPort.add(flowTupleParts[4]);\n            flowTupleMap.destination.put('port', flowTupleParts[4]);\n          }\n          if (!flowTupleParts[5].isEmpty()) {\n            netIana.add(flowTupleParts[5]);\n            flowTupleMap.put('protocol', flowTupleParts[5]);\n          }\n          String flowDirection = flowTupleParts[6];\n          if (flowDirection.contains('I')) {\n            flowDirection = 'Inbound';\n          } else if (flowDirection.contains('O')) {\n            flowDirection = 'Outbound';\n          }\n          if (!flowTupleParts[6].isEmpty()) {\n            netDirection.add(flowDirection);\n            flowTupleMap.flow.put('direction', flowDirection);\n          }\n          String flowState = flowTupleParts[7];\n          if (flowState.contains('B')) {\n            flowState = 'Begin';\n          } else if (flowState.contains('C')) {\n            flowState = 'Continuing';\n          } else if (flowState.contains('E')) {\n            flowState = 'End';\n          } else if (flowState.contains('D')) {\n            flowState = 'Deny';\n          }\n          if (!flowTupleParts[7].isEmpty()) {\n            flowTupleMap.flow.put('state', flowState);\n          }\n          if (!flowTupleParts[8].isEmpty()) {\n            flowTupleMap.flow.put('encryption', flowTupleParts[8]);\n          }\n          if (!flowTupleParts[9].isEmpty()) {\n            sourcePackets.add(flowTupleParts[9]);\n            flowTupleMap.packets.put('sent', flowTupleParts[9]);\n          }\n          if (!flowTupleParts[10].isEmpty()) {\n            sourceBytes.add(flowTupleParts[10]);\n            flowTupleMap.bytes.put('sent', flowTupleParts[10]);\n          }\n          if (!flowTupleParts[11].isEmpty()) {\n            destinationPackets.add(flowTupleParts[11]);\n            flowTupleMap.packets.put('received', flowTupleParts[11]);\n          }\n          if (!flowTupleParts[12].isEmpty()) {\n            destinationBytes.add(flowTupleParts[12]);\n            flowTupleMap.bytes.put('received', flowTupleParts[12]);\n          }\n          newFlowTuplesList.add(flowTupleMap);\n        }\n      }\n      flowGroups.put('tuples', newFlowTuplesList);\n      flowGroups.remove('flowTuples');\n    }\n  }\n  ctx.json.flowRecords.flows = flowsList;\n  if (ctx.destination == null) {\n    Map map = new HashMap();\n    ctx.put('destination', map);\n  }\n  if (ctx.source == null) {\n    Map map = new HashMap();\n    ctx.put('source', map);\n  }\n  if (ctx.network == null) {\n    Map map = new HashMap();\n    ctx.put('network', map);\n  }\n  if (ctx.destination?.packets == null) {\n    ctx.destination.put('packets', removeDuplicateValues(destinationPackets));\n  }\n  if (ctx.destination?.bytes == null) {\n    ctx.destination.put('bytes', removeDuplicateValues(destinationBytes));\n  }\n  if (ctx.source?.packets == null) {\n    ctx.source.put('packets', removeDuplicateValues(sourcePackets));\n  }\n  if (ctx.source?.bytes == null) {\n    ctx.source.put('bytes', removeDuplicateValues(sourceBytes));\n  }\n  if (ctx.source?.port == null) {\n    ctx.source.put('port', removeDuplicateValues(srcPort));\n  }\n  if (ctx.destination?.port == null) {\n    ctx.destination.put('port', removeDuplicateValues(destinationPort));\n  }\n  if (ctx.source?.ip == null) {\n    ctx.source.put('ip', removeDuplicateValues(srcIp));\n  }\n  if (ctx.destination?.ip == null) {\n    ctx.destination.put('ip', removeDuplicateValues(destinationIp));\n  }\n  if (ctx.network?.direction == null) {\n    ctx.network.put('direction', removeDuplicateValues(netDirection));\n  }\n  if (ctx.network?.iana_number == null) {\n    ctx.network.put('iana_number', removeDuplicateValues(netIana));\n  }\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_extract_flow_tupple_values",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.flowRecords.flows").cloned();
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
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.flowGroups").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.tuples").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 2 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if event.has_value("_ingest._value.destination.port") {
                            if let Some(val) = event.get("_ingest._value.destination.port") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.destination.port".into(),
                            message,
                            })?;
                            event.set("_ingest._value.destination.port", converted)?;
                            }
                            }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                                                event.set("_ingest.on_failure_processor_tag", "convert_destination_port_to_long")?;
                                                                if event.remove("_ingest._value.destination.port").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.destination.port".into() });
                            }
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.tuples",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
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
                                            "_ingest._value.flowGroups",
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
                            "json.flowRecords.flows",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.flowRecords.flows").cloned();
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
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.flowGroups").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.tuples").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 2 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if event.has_value(
                                                                    "_ingest._value.source.port",
                                                                ) {
                                                                    if let Some(val) = event.get("_ingest._value.source.port") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.source.port".into(),
                            message,
                            })?;
                            event.set("_ingest._value.source.port", converted)?;
                            }
                                                                }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                                                event.set("_ingest.on_failure_processor_tag", "convert_source_port_to_long")?;
                                                                if event.remove("_ingest._value.source.port").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.source.port".into() });
                            }
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.tuples",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
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
                                            "_ingest._value.flowGroups",
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
                            "json.flowRecords.flows",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.flowRecords.flows").cloned();
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
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.flowGroups").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.tuples").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 2 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if event.has_value(
                                                                    "_ingest._value.destination.ip",
                                                                ) {
                                                                    if let Some(val) = event.get("_ingest._value.destination.ip") {
                            let converted = convert_value(val, "ip")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.destination.ip".into(),
                            message,
                            })?;
                            event.set("_ingest._value.destination.ip", converted)?;
                            }
                                                                }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                                                event.set("_ingest.on_failure_processor_tag", "convert_destination_ip_to_ip")?;
                                                                if event.remove("_ingest._value.destination.ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.destination.ip".into() });
                            }
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.tuples",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
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
                                            "_ingest._value.flowGroups",
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
                            "json.flowRecords.flows",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.flowRecords.flows").cloned();
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
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.flowGroups").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.tuples").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 2 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if event.has_value(
                                                                    "_ingest._value.source.ip",
                                                                ) {
                                                                    if let Some(val) = event.get(
                                                                        "_ingest._value.source.ip",
                                                                    ) {
                                                                        let converted = convert_value(val, "ip")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.source.ip".into(),
                            message,
                            })?;
                                                                        event.set("_ingest._value.source.ip", converted)?;
                                                                    }
                                                                }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                                                event.set("_ingest.on_failure_processor_tag", "convert_source_ip_to_ip")?;
                                                                if event
                                                                    .remove(
                                                                        "_ingest._value.source.ip",
                                                                    )
                                                                    .is_none()
                                                                {
                                                                    return Err(TransformError::FieldNotFound { path: "_ingest._value.source.ip".into() });
                                                                }
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.tuples",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
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
                                            "_ingest._value.flowGroups",
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
                            "json.flowRecords.flows",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.flowRecords.flows").cloned();
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
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.flowGroups").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.tuples").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 2 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if event.has_value("_ingest._value.packets.received") {
                            if let Some(val) = event.get("_ingest._value.packets.received") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.packets.received".into(),
                            message,
                            })?;
                            event.set("_ingest._value.packets.received", converted)?;
                            }
                            }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                                                event.set("_ingest.on_failure_processor_tag", "convert_packets_received_to_long")?;
                                                                if event.remove("_ingest._value.packets.received").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.packets.received".into() });
                            }
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.tuples",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
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
                                            "_ingest._value.flowGroups",
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
                            "json.flowRecords.flows",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.flowRecords.flows").cloned();
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
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.flowGroups").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.tuples").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 2 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if event.has_value(
                                                                    "_ingest._value.packets.sent",
                                                                ) {
                                                                    if let Some(val) = event.get("_ingest._value.packets.sent") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.packets.sent".into(),
                            message,
                            })?;
                            event.set("_ingest._value.packets.sent", converted)?;
                            }
                                                                }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                                                event.set("_ingest.on_failure_processor_tag", "convert_packets_sent_to_long")?;
                                                                if event.remove("_ingest._value.packets.sent").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.packets.sent".into() });
                            }
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.tuples",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
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
                                            "_ingest._value.flowGroups",
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
                            "json.flowRecords.flows",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.flowRecords.flows").cloned();
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
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.flowGroups").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.tuples").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 2 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if event.has_value(
                                                                    "_ingest._value.bytes.received",
                                                                ) {
                                                                    if let Some(val) = event.get("_ingest._value.bytes.received") {
                            let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.bytes.received".into(),
                            message,
                            })?;
                            event.set("_ingest._value.bytes.received", converted)?;
                            }
                                                                }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                                                event.set("_ingest.on_failure_processor_tag", "convert_bytes_received_to_long")?;
                                                                if event.remove("_ingest._value.bytes.received").is_none() {
                            return Err(TransformError::FieldNotFound { path: "_ingest._value.bytes.received".into() });
                            }
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.tuples",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
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
                                            "_ingest._value.flowGroups",
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
                            "json.flowRecords.flows",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.flowRecords.flows").cloned();
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
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.flowGroups").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.tuples").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 2 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if event.has_value(
                                                                    "_ingest._value.bytes.sent",
                                                                ) {
                                                                    if let Some(val) = event.get(
                                                                        "_ingest._value.bytes.sent",
                                                                    ) {
                                                                        let converted = convert_value(val, "long")
                            .map_err(|message| TransformError::ParseError {
                            path: "_ingest._value.bytes.sent".into(),
                            message,
                            })?;
                                                                        event.set("_ingest._value.bytes.sent", converted)?;
                                                                    }
                                                                }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                                                event.set("_ingest.on_failure_processor_tag", "convert_bytes_sent_to_long")?;
                                                                if event
                                                                    .remove(
                                                                        "_ingest._value.bytes.sent",
                                                                    )
                                                                    .is_none()
                                                                {
                                                                    return Err(TransformError::FieldNotFound { path: "_ingest._value.bytes.sent".into() });
                                                                }
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.tuples",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
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
                                            "_ingest._value.flowGroups",
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
                            "json.flowRecords.flows",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.flowRecords.flows").cloned();
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
                                {
                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                    // binds `_ingest._key` per entry, which is what a target of
                                    // `<field>.{{{_ingest._key}}}` reads.
                                    let subject = event.get("_ingest._value.flowGroups").cloned();
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
                                                event.set(
                                                    "_ingest._key",
                                                    Value::String(key.to_string()),
                                                )?;
                                            }
                                            event.set("_ingest._value", item)?;
                                            // ignore_failure: true
                                            let _ = (|| -> Result<()> {
                                                {
                                                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                                                    // binds `_ingest._key` per entry, which is what a target of
                                                    // `<field>.{{{_ingest._key}}}` reads.
                                                    let subject =
                                                        event.get("_ingest._value.tuples").cloned();
                                                    let keyed =
                                                        matches!(subject, Some(Value::Object(_)));
                                                    let entries: Vec<(Option<String>, Value)> =
                                                        match subject {
                                                            Some(Value::Array(items)) => items
                                                                .into_iter()
                                                                .map(|v| (None, v))
                                                                .collect(),
                                                            Some(Value::Object(fields)) => fields
                                                                .into_iter()
                                                                .map(|(k, v)| (Some(k), v))
                                                                .collect(),
                                                            _ => Vec::new(),
                                                        };
                                                    if !entries.is_empty() {
                                                        // A NESTED loop borrows the same slots, so the enclosing
                                                        // entry is saved and put back afterwards.
                                                        let enclosing =
                                                            event.get("_ingest._value").cloned();
                                                        let enclosing_key =
                                                            event.get("_ingest._key").cloned();
                                                        let mut list =
                                                            Vec::with_capacity(entries.len());
                                                        let mut fields = Map::new();
                                                        for (key, item) in entries {
                                                            if let Some(key) = key.as_deref() {
                                                                event.set(
                                                                    "_ingest._key",
                                                                    Value::String(key.to_string()),
                                                                )?;
                                                            }
                                                            event.set("_ingest._value", item)?;
                                                            // on_failure: 2 handler(s)
                                                            if let Err(err) = (|| -> Result<()> {
                                                                if let Some(date_str) = event
                                                                    .get_as_string(
                                                                        "_ingest._value.timestamp",
                                                                    )
                                                                {
                                                                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("_ingest._value.timestamp", parsed)?,
                            None => {
                            return Err(TransformError::ParseError {
                            path: "_ingest._value.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                            }
                            }
                                                                }
                                                                Ok(())
                                                            })(
                                                            ) {
                                                                event.set(
                                                                    "_ingest.on_failure_message",
                                                                    err.to_string(),
                                                                )?;
                                                                event.set("_ingest.on_failure_processor_type", "date")?;
                                                                event.set("_ingest.on_failure_processor_tag", "date_timestamp")?;
                                                                if event
                                                                    .remove(
                                                                        "_ingest._value.timestamp",
                                                                    )
                                                                    .is_none()
                                                                {
                                                                    return Err(TransformError::FieldNotFound { path: "_ingest._value.timestamp".into() });
                                                                }
                                                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                                                event.remove(
                                                                    "_ingest.on_failure_message",
                                                                );
                                                                event.remove("_ingest.on_failure_processor_type");
                                                                event.remove("_ingest.on_failure_processor_tag");
                                                                if event
                                                                    .get_object("_ingest")
                                                                    .is_some_and(|m| m.is_empty())
                                                                {
                                                                    event.remove("_ingest");
                                                                }
                                                            }
                                                            let left =
                                                                event.remove("_ingest._value");
                                                            match key {
                                                                // An entry the body renamed AWAY is gone from the
                                                                // object, which is how a foreach lifts fields up.
                                                                Some(key) => {
                                                                    if let Some(value) = left {
                                                                        fields.insert(key, value);
                                                                    }
                                                                }
                                                                None => list.push(
                                                                    left.unwrap_or(Value::Null),
                                                                ),
                                                            }
                                                        }
                                                        match enclosing {
                                                            Some(previous) => {
                                                                event.set(
                                                                    "_ingest._value",
                                                                    previous,
                                                                )?;
                                                            }
                                                            None => {
                                                                event.remove("_ingest");
                                                            }
                                                        }
                                                        if let Some(previous) = enclosing_key {
                                                            event.set("_ingest._key", previous)?;
                                                        }
                                                        event.set(
                                                            "_ingest._value.tuples",
                                                            if keyed {
                                                                Value::Object(fields)
                                                            } else {
                                                                Value::Array(list)
                                                            },
                                                        )?;
                                                    }
                                                }
                                                Ok(())
                                            })(
                                            );
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
                                            "_ingest._value.flowGroups",
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
                            "json.flowRecords.flows",
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
                    .get("json.flowRecords.flows")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.flowRecords.flows", |event| {
                    if event.has_value("_ingest._value.flowGroups") {
                        event.rename("_ingest._value.flowGroups", "_ingest._value.groups")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.flowRecords.flows") {
                event.rename(
                    "json.flowRecords.flows",
                    "azure_network_watcher_vnet.log.records.flows",
                )?;
            }

            let _cond = { event.get("source.port").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("source.port").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_source_port_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
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
                            "source.port",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("destination.port").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("destination.port").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_destination_port_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
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
                            "destination.port",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("source.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("source.ip").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
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
                                    "convert_source_ip_to_ip",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
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
                            "source.ip",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("destination.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("destination.ip").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
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
                                    "convert_destination_ip_to_ip",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
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
                            "destination.ip",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("source.packets").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("source.packets").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_source_packets_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
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
                            "source.packets",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("source.bytes").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("source.bytes").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_source_bytes_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
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
                            "source.bytes",
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
                    .get("destination.packets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("destination.packets").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_destination_packets_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
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
                            "destination.packets",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("destination.bytes").is_some_and(|v| v.is_array()) };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("destination.bytes").cloned();
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
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
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
                                    "convert_destination_bytes_to_long",
                                )?;
                                if event.remove("_ingest._value").is_none() {
                                    return Err(TransformError::FieldNotFound {
                                        path: "_ingest._value".into(),
                                    });
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
                            "destination.bytes",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = { event.get("destination.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "destination.ip", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("source.ip").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "source.ip", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("network.direction").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "network.direction", |event| {
                    if event.has_value("_ingest._value") {
                        map_strings(event, "_ingest._value", "_ingest._value", str::to_lowercase)?;
                    }
                    Ok(())
                })?;
            }

            event.remove("json");

            let _cond = { event.has_value("azure_network_watcher_vnet.log.flow_log.resource_id") };
            if _cond {
                // Begin nested pipeline: "azure_shared_pipeline"
                event.set("cloud.provider", json!("azure"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) =
                        event.get_string("azure_network_watcher_vnet.log.flow_log.resource_id")
                    {
                        // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))
                        // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:.+))/NAMESPACES/(?P<azure_resource_namespace>(?:.+))/AUTHORIZATIONRULES/(?P<azure_resource_authorization_rule>(?:.+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_namespace", "azure.resource.namespace"),
                                        (
                                            "azure_resource_authorization_rule",
                                            "azure.resource.authorization_rule"
                                        )
                                    ]
                                ),
                                cached_grok_mapped!(
                                    "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:.+))/namespaces/(?P<azure_resource_namespace>(?:.+))/authorizationRules/(?P<azure_resource_authorization_rule>(?:.+))",
                                    [
                                        ("azure_subscription_id", "azure.subscription_id"),
                                        ("azure_resource_group", "azure.resource.group"),
                                        ("azure_resource_provider", "azure.resource.provider"),
                                        ("azure_resource_namespace", "azure.resource.namespace"),
                                        (
                                            "azure_resource_authorization_rule",
                                            "azure.resource.authorization_rule"
                                        )
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })();
                let _cond = { !event.has_value("azure.subscription_id") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("azure_network_watcher_vnet.log.flow_log.resource_id")
                        {
                            // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                            // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))
                            if !extract_first_match(
                                &[
                                    cached_grok_mapped!(
                                        "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_group", "azure.resource.group"),
                                            ("azure_resource_provider", "azure.resource.provider"),
                                            ("azure_resource_name", "azure.resource.name")
                                        ]
                                    ),
                                    cached_grok_mapped!(
                                        "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+/([A-Za-z])\\w+.))/(?P<azure_resource_name>(?:((?!AUTHORIZATIONRULES).)*$))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_group", "azure.resource.group"),
                                            ("azure_resource_provider", "azure.resource.provider"),
                                            ("azure_resource_name", "azure.resource.name")
                                        ]
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { !event.has_value("azure.subscription_id") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("azure_network_watcher_vnet.log.flow_log.resource_id")
                        {
                            // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                            // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))
                            if !extract_first_match(
                                &[
                                    cached_grok_mapped!(
                                        "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_group", "azure.resource.group"),
                                            ("azure_resource_provider", "azure.resource.provider"),
                                            ("azure_resource_name", "azure.resource.name")
                                        ]
                                    ),
                                    cached_grok_mapped!(
                                        "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))/(?P<azure_resource_name>(?:.+))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_group", "azure.resource.group"),
                                            ("azure_resource_provider", "azure.resource.provider"),
                                            ("azure_resource_name", "azure.resource.name")
                                        ]
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { !event.has_value("azure.subscription_id") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("azure_network_watcher_vnet.log.flow_log.resource_id")
                        {
                            // Grok pattern: /providers/(?P<azure_resource_provider>(?:.+))
                            // Grok pattern: /PROVIDERS/(?P<azure_resource_provider>(?:.+))
                            if !extract_first_match(
                                &[
                                    cached_grok_mapped!(
                                        "/providers/(?P<azure_resource_provider>(?:.+))",
                                        [("azure_resource_provider", "azure.resource.provider")]
                                    ),
                                    cached_grok_mapped!(
                                        "/PROVIDERS/(?P<azure_resource_provider>(?:.+))",
                                        [("azure_resource_provider", "azure.resource.provider")]
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { !event.has_value("azure.subscription_id") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("azure_network_watcher_vnet.log.flow_log.resource_id")
                        {
                            // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                            // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))
                            if !extract_first_match(
                                &[
                                    cached_grok_mapped!(
                                        "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/PROVIDERS/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_provider", "azure.resource.provider")
                                        ]
                                    ),
                                    cached_grok_mapped!(
                                        "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/providers/(?P<azure_resource_provider>(?:([A-Za-z])\\w+.([A-Za-z])\\w+\\/([A-Za-z][^\\/])\\w+))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_provider", "azure.resource.provider")
                                        ]
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { !event.has_value("azure.subscription_id") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("azure_network_watcher_vnet.log.flow_log.resource_id")
                        {
                            // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))
                            // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))
                            if !extract_first_match(
                                &[
                                    cached_grok_mapped!(
                                        "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/RESOURCEGROUPS/(?P<azure_resource_group>(?:.+))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_group", "azure.resource.group")
                                        ]
                                    ),
                                    cached_grok_mapped!(
                                        "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))/resourceGroups/(?P<azure_resource_group>(?:.+))",
                                        [
                                            ("azure_subscription_id", "azure.subscription_id"),
                                            ("azure_resource_group", "azure.resource.group")
                                        ]
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { !event.has_value("azure.subscription_id") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("azure_network_watcher_vnet.log.flow_log.resource_id")
                        {
                            // Grok pattern: /SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                            // Grok pattern: /subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))
                            if !extract_first_match(
                                &[
                                    cached_grok_mapped!(
                                        "/SUBSCRIPTIONS/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))",
                                        [("azure_subscription_id", "azure.subscription_id")]
                                    ),
                                    cached_grok_mapped!(
                                        "/subscriptions/(?P<azure_subscription_id>(?:(\\{){0,1}[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}(\\}){0,1}))",
                                        [("azure_subscription_id", "azure.subscription_id")]
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if let Some(v) = event
                    .get("azure_network_watcher_vnet.log.flow_log.resource_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("azure.resource.id", v)?;
                }
                // End nested pipeline: "azure_shared_pipeline"
            }

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
