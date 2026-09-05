// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_bigipihealthinfo` pipeline.
pub struct PipelineBigipihealthinfo;

impl Transform for PipelineBigipihealthinfo {
    fn name(&self) -> &str {
        "pipeline_bigipihealthinfo"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.append("event.category", json!("vulnerability"))?;

        event.append("event.type", json!("info"))?;

        event.set("observer.product", json!("iHealth Information"))?;

        // Painless script
        // Source: def convertKeysToString(Map versionMap, String[] keysToConvert) {\n  def convertedMap = [:];\n  versionMap.entrySet().forEach(entry -> {\n    def key = entry.getKey().toString();\n    def value = entry.getValue();\n    if (Arrays.asList(keysToConvert).contains(key)) {\n      convertedMap[key] = value.toString();\n    } else {\n      convertedMap[key] = value;\n    }\n  });\n  return convertedMap;\n}\ndef diagnosticArray = ctx.json.diagnostics;\ndef keysToConvert = new String[] {\n  'minor',\n  'major',\n  'maintenance',\n  'fix',\n  'point'\n};\nif (diagnosticArray instanceof List) {\n  for (int i = 0; i < diagnosticArray.size(); i++) {\n    def versionArray = diagnosticArray[i].version;\n    if (versionArray instanceof List) {\n      for (int j = 0; j < versionArray.size(); j++) {\n        versionArray[j] = convertKeysToString(versionArray[j], keysToConvert);\n      }\n      ctx.json.diagnostics[i].put('version', versionArray);\n    }\n  }\n}\n
        // TODO: Transpile Painless to Rust (2.2.3)
        painless_exec_plan(
            event,
            cached_painless!(
                r#"def convertKeysToString(Map versionMap, String[] keysToConvert) {\n  def convertedMap = [:];\n  versionMap.entrySet().forEach(entry -> {\n    def key = entry.getKey().toString();\n    def value = entry.getValue();\n    if (Arrays.asList(keysToConvert).contains(key)) {\n      convertedMap[key] = value.toString();\n    } else {\n      convertedMap[key] = value;\n    }\n  });\n  return convertedMap;\n}\ndef diagnosticArray = ctx.json.diagnostics;\ndef keysToConvert = new String[] {\n  'minor',\n  'major',\n  'maintenance',\n  'fix',\n  'point'\n};\nif (diagnosticArray instanceof List) {\n  for (int i = 0; i < diagnosticArray.size(); i++) {\n    def versionArray = diagnosticArray[i].version;\n    if (versionArray instanceof List) {\n      for (int j = 0; j < versionArray.size(); j++) {\n        versionArray[j] = convertKeysToString(versionArray[j], keysToConvert);\n      }\n      ctx.json.diagnostics[i].put('version', versionArray);\n    }\n  }\n}\n"#
            ),
        )?;

        // Painless script
        // Source: def diagnosticArray = ctx.json.diagnostics;\ndef cveIds = new HashSet();\ndef importance = new HashSet();\ndef header = new HashSet();\ndef summary = new HashSet();\ndef ruleName = new HashSet();\ndef ruleRef = new HashSet();\ndef threatRef = new HashSet();\nif (ctx.vulnerability == null) {\n  ctx.vulnerability = new HashMap()\n}\nif (ctx.rule == null) {\n  ctx.rule = new HashMap()\n}\nif (ctx.threat == null) {\n  ctx.threat = new HashMap()\n}\nif (ctx.threat.enrichments == null) {\n  ctx.threat.enrichments = new HashMap()\n}\nif (ctx.threat.enrichments.indicator == null) {\n  ctx.threat.enrichments.indicator = new HashMap()\n}\nif (diagnosticArray instanceof List) {\n  for (int i = 0; i < diagnosticArray.size(); i++) {\n    def cveIdsArray = diagnosticArray[i].cveIds;\n    if (cveIdsArray instanceof List) {\n      for (int j=0; j < cveIdsArray.size(); j++) {\n        cveIds.add(cveIdsArray[j]);\n      }\n      ctx.json.diagnostics[i].remove('cveIds');\n      ctx.json.diagnostics[i].put('cve_ids', cveIdsArray);\n    }\n    def solutionArray = diagnosticArray[i].solution;\n    if (solutionArray instanceof List) {\n      for (int j=0; j < solutionArray.size(); j++) {\n        if (solutionArray[j].id != null) {\n          ruleRef.add(solutionArray[j].id);\n        }\n        if (solutionArray[j].value != null) {\n          threatRef.add(solutionArray[j].value);\n        }\n      }\n      ctx.json.diagnostics[i].remove('cveIds');\n      ctx.json.diagnostics[i].put('cve_ids', cveIdsArray);\n    }\n    if (diagnosticArray[i].importance != null){\n      importance.add(diagnosticArray[i].importance);\n    }\n    if (diagnosticArray[i].header != null){\n      header.add(diagnosticArray[i].header);\n    }\n    if (diagnosticArray[i].summary != null){\n      summary.add(diagnosticArray[i].summary);\n    }\n    if (diagnosticArray[i].name != null){\n      ruleName.add(diagnosticArray[i].name);\n    }\n  }\n}\nctx.vulnerability.put('id', cveIds);\nctx.vulnerability.put('severity', importance);\nctx.vulnerability.put('description', header);\nctx.vulnerability.put('description', summary);\nctx.rule.put('name', ruleName);\nctx.rule.put('reference', ruleRef);\nctx.threat.enrichments.indicator.put('reference', threatRef);\n
        // TODO: Transpile Painless to Rust (2.2.3)
        painless_exec_plan(
            event,
            cached_painless!(
                r#"def diagnosticArray = ctx.json.diagnostics;\ndef cveIds = new HashSet();\ndef importance = new HashSet();\ndef header = new HashSet();\ndef summary = new HashSet();\ndef ruleName = new HashSet();\ndef ruleRef = new HashSet();\ndef threatRef = new HashSet();\nif (ctx.vulnerability == null) {\n  ctx.vulnerability = new HashMap()\n}\nif (ctx.rule == null) {\n  ctx.rule = new HashMap()\n}\nif (ctx.threat == null) {\n  ctx.threat = new HashMap()\n}\nif (ctx.threat.enrichments == null) {\n  ctx.threat.enrichments = new HashMap()\n}\nif (ctx.threat.enrichments.indicator == null) {\n  ctx.threat.enrichments.indicator = new HashMap()\n}\nif (diagnosticArray instanceof List) {\n  for (int i = 0; i < diagnosticArray.size(); i++) {\n    def cveIdsArray = diagnosticArray[i].cveIds;\n    if (cveIdsArray instanceof List) {\n      for (int j=0; j < cveIdsArray.size(); j++) {\n        cveIds.add(cveIdsArray[j]);\n      }\n      ctx.json.diagnostics[i].remove('cveIds');\n      ctx.json.diagnostics[i].put('cve_ids', cveIdsArray);\n    }\n    def solutionArray = diagnosticArray[i].solution;\n    if (solutionArray instanceof List) {\n      for (int j=0; j < solutionArray.size(); j++) {\n        if (solutionArray[j].id != null) {\n          ruleRef.add(solutionArray[j].id);\n        }\n        if (solutionArray[j].value != null) {\n          threatRef.add(solutionArray[j].value);\n        }\n      }\n      ctx.json.diagnostics[i].remove('cveIds');\n      ctx.json.diagnostics[i].put('cve_ids', cveIdsArray);\n    }\n    if (diagnosticArray[i].importance != null){\n      importance.add(diagnosticArray[i].importance);\n    }\n    if (diagnosticArray[i].header != null){\n      header.add(diagnosticArray[i].header);\n    }\n    if (diagnosticArray[i].summary != null){\n      summary.add(diagnosticArray[i].summary);\n    }\n    if (diagnosticArray[i].name != null){\n      ruleName.add(diagnosticArray[i].name);\n    }\n  }\n}\nctx.vulnerability.put('id', cveIds);\nctx.vulnerability.put('severity', importance);\nctx.vulnerability.put('description', header);\nctx.vulnerability.put('description', summary);\nctx.rule.put('name', ruleName);\nctx.rule.put('reference', ruleRef);\nctx.threat.enrichments.indicator.put('reference', threatRef);\n"#
            ),
        )?;

        if event.has_value("json") {
            event.rename("json", "f5_bigip.log")?;
        }

        if let Some(v) = event
            .get("f5_bigip.log.system.hostname")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("host.hostname", v)?;
        }

        if event.has_value("f5_bigip.log.system.ihealthLink") {
            event.rename(
                "f5_bigip.log.system.ihealthLink",
                "f5_bigip.log.system.ihealth_link",
            )?;
        }

        if let Some(v) = event
            .get("f5_bigip.log.system.ihealth_link")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("url.full", v)?;
        }

        if event.has_value("f5_bigip.log.system.qkviewNumber") {
            event.rename(
                "f5_bigip.log.system.qkviewNumber",
                "f5_bigip.log.system.qkview_number",
            )?;
        }

        let _cond = {
            event.has_value("f5_bigip.log.telemetryServiceInfo.cycleEnd")
                && event.get_str("f5_bigip.log.telemetryServiceInfo.cycleEnd") != Some("")
        };
        if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("f5_bigip.log.telemetryServiceInfo.cycleEnd")
                {
                    match parse_date_out(
                        &date_str,
                        &["EEE, dd MMM yyyy HH:mm:ss z", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("f5_bigip.log.telemetry_service_info.cycle_end", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "f5_bigip.log.telemetryServiceInfo.cycleEnd".into(),
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
                    "date_cycle_end_conversion",
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
            .get("f5_bigip.log.telemetry_service_info.cycle_end")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("event.end", v)?;
        }

        let _cond = {
            event.has_value("f5_bigip.log.telemetryServiceInfo.cycleStart")
                && event.get_str("f5_bigip.log.telemetryServiceInfo.cycleStart") != Some("")
        };
        if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("f5_bigip.log.telemetryServiceInfo.cycleStart")
                {
                    match parse_date_out(
                        &date_str,
                        &["EEE, dd MMM yyyy HH:mm:ss z", "ISO8601"],
                        None,
                        None,
                    ) {
                        Some(parsed) => {
                            event.set("f5_bigip.log.telemetry_service_info.cycle_start", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "f5_bigip.log.telemetryServiceInfo.cycleStart".into(),
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
                    "date_cycle_start_conversion",
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
            .get("f5_bigip.log.telemetry_service_info.cycle_start")
            .filter(|v| !painless_is_empty_value(v))
            .cloned()
        {
            event.set("event.start", v)?;
        }

        if event.has_value("f5_bigip.log.telemetryEventCategory") {
            event.rename(
                "f5_bigip.log.telemetryEventCategory",
                "f5_bigip.log.telemetry.event.category",
            )?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.remove("f5_bigip.log.telemetryServiceInfo.cycleEnd");
            event.remove("f5_bigip.log.telemetryServiceInfo.cycleStart");
            Ok(())
        })();

        let _cond = {
            event
                .get("f5_bigip.log.diagnostics")
                .is_some_and(|v| v.is_array())
        };
        if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "f5_bigip.log.diagnostics", |event| {
                    let _cond = {
                        !event.has_value("tags")
                            || !(event.get("tags").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => a.iter().any(|x| {
                                    x.as_str() == Some("preserve_duplicate_custom_fields")
                                }),
                                serde_json::Value::String(s) => {
                                    s.contains("preserve_duplicate_custom_fields")
                                }
                                _ => false,
                            }))
                    };
                    if _cond {
                        event.remove("_ingest._value.cve_ids");
                        event.remove("_ingest._value.header");
                        event.remove("_ingest._value.importance");
                        event.remove("_ingest._value.name");
                        event.remove("_ingest._value.solution");
                        event.remove("_ingest._value.summary");
                    }
                    Ok(())
                })?;
                Ok(())
            })();
        }

        let _cond = {
            !event.has_value("tags")
                || !(event.get("tags").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a
                        .iter()
                        .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                    serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"),
                    _ => false,
                }))
        };
        if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("f5_bigip.log.system.hostname");
                event.remove("f5_bigip.log.system.ihealth_link");
                event.remove("f5_bigip.log.telemetry_service_info.cycle_end");
                event.remove("f5_bigip.log.telemetry_service_info.cycle_start");
                Ok(())
            })();
        }

        Ok(TransformResult::Continue)
    }
}
