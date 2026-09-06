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
            event.set("ecs.version", json!("9.3.0"))?;

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

            parse_json_field(event, "event.original", "json")?;

            let _cond = {
                event.has_value("json.items")
                    && event.get("json.items").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.eventTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.request") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.requests") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.rt_utc") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.uuid") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json") };
            if _cond {
                // Painless script
                // Source: String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.detection = convertToSnakeCase(ctx.json);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String camelToSnake(String str) {\n  def result = \"\";\n  def lastCharWasUpperCase = false;\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0 && !lastCharWasUpperCase) {\n        result += \"_\";\n      }\n      result += Character.toLowerCase(c);\n      lastCharWasUpperCase = true;\n    } else {\n      result += c;\n      lastCharWasUpperCase = false;\n    }\n  }\n  return result;\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      newObj[camelToSnake(entry.getKey())] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    if (obj == \"\") {\n      return null;\n    }\n    return obj;\n  }\n}\nctx.trend_micro_vision_one = ctx.trend_micro_vision_one ?: [:];\nctx.trend_micro_vision_one.detection = convertToSnakeCase(ctx.json);\n"#
                    ),
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.endpoint_mac_address") {
                gsub_field(
                    event,
                    "trend_micro_vision_one.detection.endpoint_mac_address",
                    "trend_micro_vision_one.detection.endpoint_mac_address",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.endpoint_mac_address") {
                map_strings(
                    event,
                    "trend_micro_vision_one.detection.endpoint_mac_address",
                    "trend_micro_vision_one.detection.endpoint_mac_address",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.device_mac_address") {
                gsub_field(
                    event,
                    "trend_micro_vision_one.detection.device_mac_address",
                    "trend_micro_vision_one.detection.device_mac_address",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.device_mac_address") {
                map_strings(
                    event,
                    "trend_micro_vision_one.detection.device_mac_address",
                    "trend_micro_vision_one.detection.device_mac_address",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.interested_mac_address") {
                gsub_field(
                    event,
                    "trend_micro_vision_one.detection.interested_mac_address",
                    "trend_micro_vision_one.detection.interested_mac_address",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.interested_mac_address") {
                map_strings(
                    event,
                    "trend_micro_vision_one.detection.interested_mac_address",
                    "trend_micro_vision_one.detection.interested_mac_address",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.smac") {
                gsub_field(
                    event,
                    "trend_micro_vision_one.detection.smac",
                    "trend_micro_vision_one.detection.smac",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.smac") {
                map_strings(
                    event,
                    "trend_micro_vision_one.detection.smac",
                    "trend_micro_vision_one.detection.smac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.dmac") {
                gsub_field(
                    event,
                    "trend_micro_vision_one.detection.dmac",
                    "trend_micro_vision_one.detection.dmac",
                    cached_regex!("[-:.]"),
                    "-",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.dmac") {
                map_strings(
                    event,
                    "trend_micro_vision_one.detection.dmac",
                    "trend_micro_vision_one.detection.dmac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.dst") {
                event.rename(
                    "trend_micro_vision_one.detection.dst",
                    "trend_micro_vision_one.detection.destination.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.dpt") {
                event.rename(
                    "trend_micro_vision_one.detection.dpt",
                    "trend_micro_vision_one.detection.destination.port",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.dst_group") {
                event.rename(
                    "trend_micro_vision_one.detection.dst_group",
                    "trend_micro_vision_one.detection.destination.ip_group",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.dmac") {
                event.rename(
                    "trend_micro_vision_one.detection.dmac",
                    "trend_micro_vision_one.detection.destination.mac",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.dst_zone") {
                event.rename(
                    "trend_micro_vision_one.detection.dst_zone",
                    "trend_micro_vision_one.detection.destination.zone",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.duser") {
                event.rename(
                    "trend_micro_vision_one.detection.duser",
                    "trend_micro_vision_one.detection.destination.user",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.suser") {
                event.rename(
                    "trend_micro_vision_one.detection.suser",
                    "trend_micro_vision_one.detection.source.user",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.shost") {
                event.rename(
                    "trend_micro_vision_one.detection.shost",
                    "trend_micro_vision_one.detection.source.host",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.src_zone") {
                event.rename(
                    "trend_micro_vision_one.detection.src_zone",
                    "trend_micro_vision_one.detection.source.zone",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.smac") {
                event.rename(
                    "trend_micro_vision_one.detection.smac",
                    "trend_micro_vision_one.detection.source.mac",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.src") {
                event.rename(
                    "trend_micro_vision_one.detection.src",
                    "trend_micro_vision_one.detection.source.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.spt") {
                event.rename(
                    "trend_micro_vision_one.detection.spt",
                    "trend_micro_vision_one.detection.source.port",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.src_group") {
                event.rename(
                    "trend_micro_vision_one.detection.src_group",
                    "trend_micro_vision_one.detection.source.group",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.dhost") {
                event.rename(
                    "trend_micro_vision_one.detection.dhost",
                    "trend_micro_vision_one.detection.device.host",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.device_mac_address") {
                event.rename(
                    "trend_micro_vision_one.detection.device_mac_address",
                    "trend_micro_vision_one.detection.device.mac",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.m_device_guid") {
                event.rename(
                    "trend_micro_vision_one.detection.m_device_guid",
                    "trend_micro_vision_one.detection.device.guid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.device_guid") {
                event.rename(
                    "trend_micro_vision_one.detection.device_guid",
                    "trend_micro_vision_one.detection.device.id",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.m_device") {
                event.rename(
                    "trend_micro_vision_one.detection.m_device",
                    "trend_micro_vision_one.detection.device.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.device_process_name") {
                event.rename(
                    "trend_micro_vision_one.detection.device_process_name",
                    "trend_micro_vision_one.detection.device.process_name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.device_direction") {
                event.rename(
                    "trend_micro_vision_one.detection.device_direction",
                    "trend_micro_vision_one.detection.device.direction",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.device_payload_id") {
                event.rename(
                    "trend_micro_vision_one.detection.device_payload_id",
                    "trend_micro_vision_one.detection.device.payload_id",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.device_risk_confidence_level") {
                event.rename(
                    "trend_micro_vision_one.detection.device_risk_confidence_level",
                    "trend_micro_vision_one.detection.device.risk_confidence_level",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.endpoint_guid") {
                event.rename(
                    "trend_micro_vision_one.detection.endpoint_guid",
                    "trend_micro_vision_one.detection.endpoint.guid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.endpoint_ip") {
                event.rename(
                    "trend_micro_vision_one.detection.endpoint_ip",
                    "trend_micro_vision_one.detection.endpoint.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.endpoint_mac_address") {
                event.rename(
                    "trend_micro_vision_one.detection.endpoint_mac_address",
                    "trend_micro_vision_one.detection.endpoint.mac",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.endpoint_host_name") {
                event.rename(
                    "trend_micro_vision_one.detection.endpoint_host_name",
                    "trend_micro_vision_one.detection.endpoint.hostname",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.interested_host") {
                event.rename(
                    "trend_micro_vision_one.detection.interested_host",
                    "trend_micro_vision_one.detection.interested.host",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.interested_ip") {
                event.rename(
                    "trend_micro_vision_one.detection.interested_ip",
                    "trend_micro_vision_one.detection.interested.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.interested_mac_address") {
                event.rename(
                    "trend_micro_vision_one.detection.interested_mac_address",
                    "trend_micro_vision_one.detection.interested.mac",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.interested_group") {
                event.rename(
                    "trend_micro_vision_one.detection.interested_group",
                    "trend_micro_vision_one.detection.interested.group",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.peer_host") {
                event.rename(
                    "trend_micro_vision_one.detection.peer_host",
                    "trend_micro_vision_one.detection.peer.host",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.peer_ip") {
                event.rename(
                    "trend_micro_vision_one.detection.peer_ip",
                    "trend_micro_vision_one.detection.peer.ip",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.peer_group") {
                event.rename(
                    "trend_micro_vision_one.detection.peer_group",
                    "trend_micro_vision_one.detection.peer.group",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.object_file_hash_md5") {
                event.rename(
                    "trend_micro_vision_one.detection.object_file_hash_md5",
                    "trend_micro_vision_one.detection.object.file.hash.md5",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.object_file_hash_sha1") {
                event.rename(
                    "trend_micro_vision_one.detection.object_file_hash_sha1",
                    "trend_micro_vision_one.detection.object.file.hash.sha1",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.object_file_hash_sha256") {
                event.rename(
                    "trend_micro_vision_one.detection.object_file_hash_sha256",
                    "trend_micro_vision_one.detection.object.file.hash.sha256",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.object_file_name") {
                event.rename(
                    "trend_micro_vision_one.detection.object_file_name",
                    "trend_micro_vision_one.detection.object.file.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.object_file_path") {
                event.rename(
                    "trend_micro_vision_one.detection.object_file_path",
                    "trend_micro_vision_one.detection.object.file.path",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.object_cmd") {
                event.rename(
                    "trend_micro_vision_one.detection.object_cmd",
                    "trend_micro_vision_one.detection.object.cmd",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.object_name") {
                event.rename(
                    "trend_micro_vision_one.detection.object_name",
                    "trend_micro_vision_one.detection.object.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.object_pid") {
                event.rename(
                    "trend_micro_vision_one.detection.object_pid",
                    "trend_micro_vision_one.detection.object.pid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.object_signer") {
                event.rename(
                    "trend_micro_vision_one.detection.object_signer",
                    "trend_micro_vision_one.detection.object.signer",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.parent_cmd") {
                event.rename(
                    "trend_micro_vision_one.detection.parent_cmd",
                    "trend_micro_vision_one.detection.parent.cmd",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.parent_file_hash_sha1") {
                event.rename(
                    "trend_micro_vision_one.detection.parent_file_hash_sha1",
                    "trend_micro_vision_one.detection.parent.file.hash.sha1",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.parent_file_hash_sha256") {
                event.rename(
                    "trend_micro_vision_one.detection.parent_file_hash_sha256",
                    "trend_micro_vision_one.detection.parent.file.hash.sha256",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.parent_file_path") {
                event.rename(
                    "trend_micro_vision_one.detection.parent_file_path",
                    "trend_micro_vision_one.detection.parent.file.path",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.process_cmd") {
                event.rename(
                    "trend_micro_vision_one.detection.process_cmd",
                    "trend_micro_vision_one.detection.process.cmd",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.process_name") {
                event.rename(
                    "trend_micro_vision_one.detection.process_name",
                    "trend_micro_vision_one.detection.process.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.process_pid") {
                event.rename(
                    "trend_micro_vision_one.detection.process_pid",
                    "trend_micro_vision_one.detection.process.pid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.process_file_hash_md5") {
                event.rename(
                    "trend_micro_vision_one.detection.process_file_hash_md5",
                    "trend_micro_vision_one.detection.process.file.hash.md5",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.process_file_hash_sha1") {
                event.rename(
                    "trend_micro_vision_one.detection.process_file_hash_sha1",
                    "trend_micro_vision_one.detection.process.file.hash.sha1",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.process_file_hash_sha256") {
                event.rename(
                    "trend_micro_vision_one.detection.process_file_hash_sha256",
                    "trend_micro_vision_one.detection.process.file.hash.sha256",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.process_file_path") {
                event.rename(
                    "trend_micro_vision_one.detection.process_file_path",
                    "trend_micro_vision_one.detection.process.file.path",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.process_signer") {
                event.rename(
                    "trend_micro_vision_one.detection.process_signer",
                    "trend_micro_vision_one.detection.process.signer",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.product_code") {
                event.rename(
                    "trend_micro_vision_one.detection.product_code",
                    "trend_micro_vision_one.detection.product.code",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.pname") {
                event.rename(
                    "trend_micro_vision_one.detection.pname",
                    "trend_micro_vision_one.detection.product.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.pver") {
                event.rename(
                    "trend_micro_vision_one.detection.pver",
                    "trend_micro_vision_one.detection.product.version",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.mpname") {
                event.rename(
                    "trend_micro_vision_one.detection.mpname",
                    "trend_micro_vision_one.detection.mproduct.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.mpver") {
                event.rename(
                    "trend_micro_vision_one.detection.mpver",
                    "trend_micro_vision_one.detection.mproduct.version",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.policy_name") {
                event.rename(
                    "trend_micro_vision_one.detection.policy_name",
                    "trend_micro_vision_one.detection.policy.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.policy_uuid") {
                event.rename(
                    "trend_micro_vision_one.detection.policy_uuid",
                    "trend_micro_vision_one.detection.policy.uuid",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.log_key") {
                event.rename(
                    "trend_micro_vision_one.detection.log_key",
                    "trend_micro_vision_one.detection.policy.logkey",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.domain_name") {
                event.rename(
                    "trend_micro_vision_one.detection.domain_name",
                    "trend_micro_vision_one.detection.domain.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.os_name") {
                event.rename(
                    "trend_micro_vision_one.detection.os_name",
                    "trend_micro_vision_one.detection.os.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.user_domain") {
                event.rename(
                    "trend_micro_vision_one.detection.user_domain",
                    "trend_micro_vision_one.detection.user.domain",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.mitre_mapping") {
                event.rename(
                    "trend_micro_vision_one.detection.mitre_mapping",
                    "trend_micro_vision_one.detection.security_analytics.engine.name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.mitre_version") {
                event.rename(
                    "trend_micro_vision_one.detection.mitre_version",
                    "trend_micro_vision_one.detection.security_analytics.engine.version",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.act") {
                event.rename(
                    "trend_micro_vision_one.detection.act",
                    "trend_micro_vision_one.detection.action",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.act_result") {
                event.rename(
                    "trend_micro_vision_one.detection.act_result",
                    "trend_micro_vision_one.detection.action_result",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.host_name") {
                event.rename(
                    "trend_micro_vision_one.detection.host_name",
                    "trend_micro_vision_one.detection.hostname",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.cat") {
                event.rename(
                    "trend_micro_vision_one.detection.cat",
                    "trend_micro_vision_one.detection.severity_level",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.app") {
                event.rename(
                    "trend_micro_vision_one.detection.app",
                    "trend_micro_vision_one.detection.protocol",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.app_group") {
                event.rename(
                    "trend_micro_vision_one.detection.app_group",
                    "trend_micro_vision_one.detection.protocol_group",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.behavior_cat") {
                event.rename(
                    "trend_micro_vision_one.detection.behavior_cat",
                    "trend_micro_vision_one.detection.behavior_category",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.blocking") {
                event.rename(
                    "trend_micro_vision_one.detection.blocking",
                    "trend_micro_vision_one.detection.block",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.component") {
                event.rename(
                    "trend_micro_vision_one.detection.component",
                    "trend_micro_vision_one.detection.component_version",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.ccca_detection") {
                event.rename(
                    "trend_micro_vision_one.detection.ccca_detection",
                    "trend_micro_vision_one.detection.detection",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.ccca_detection_source") {
                event.rename(
                    "trend_micro_vision_one.detection.ccca_detection_source",
                    "trend_micro_vision_one.detection.detection_source",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.ccca_risk_level") {
                event.rename(
                    "trend_micro_vision_one.detection.ccca_risk_level",
                    "trend_micro_vision_one.detection.risk_level",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.end") {
                event.rename(
                    "trend_micro_vision_one.detection.end",
                    "trend_micro_vision_one.detection.end_time",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.eng_type") {
                event.rename(
                    "trend_micro_vision_one.detection.eng_type",
                    "trend_micro_vision_one.detection.engine_type",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.eng_ver") {
                event.rename(
                    "trend_micro_vision_one.detection.eng_ver",
                    "trend_micro_vision_one.detection.engine_version",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.first_act") {
                event.rename(
                    "trend_micro_vision_one.detection.first_act",
                    "trend_micro_vision_one.detection.first_action",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.first_act_result") {
                event.rename(
                    "trend_micro_vision_one.detection.first_act_result",
                    "trend_micro_vision_one.detection.first_action_result",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.mal_name") {
                event.rename(
                    "trend_micro_vision_one.detection.mal_name",
                    "trend_micro_vision_one.detection.malware_name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.mal_type") {
                event.rename(
                    "trend_micro_vision_one.detection.mal_type",
                    "trend_micro_vision_one.detection.malware_type",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.mal_type_group") {
                event.rename(
                    "trend_micro_vision_one.detection.mal_type_group",
                    "trend_micro_vision_one.detection.malware_type_group",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.search_dl") {
                event.rename(
                    "trend_micro_vision_one.detection.search_dl",
                    "trend_micro_vision_one.detection.search_data_lake",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.event_sub_name") {
                event.rename(
                    "trend_micro_vision_one.detection.event_sub_name",
                    "trend_micro_vision_one.detection.sub_name",
                )?;
            }

            if event.has_value("trend_micro_vision_one.detection.cnt") {
                event.rename(
                    "trend_micro_vision_one.detection.cnt",
                    "trend_micro_vision_one.detection.total_count",
                )?;
            }

            let _cond = { !event.has_value("trend_micro_vision_one.detection.request") };
            if _cond {
                if event.has_value("trend_micro_vision_one.detection.requests") {
                    event.rename(
                        "trend_micro_vision_one.detection.requests",
                        "trend_micro_vision_one.detection.request",
                    )?;
                }
            }

            let _cond =
                { event.get_str("trend_micro_vision_one.detection.apt_related") == Some("0") };
            if _cond {
                event.set("trend_micro_vision_one.detection.related_apt", json!(false))?;
            }

            let _cond =
                { event.get_str("trend_micro_vision_one.detection.apt_related") == Some("1") };
            if _cond {
                event.set("trend_micro_vision_one.detection.related_apt", json!(true))?;
            }

            let _cond = {
                event.has_value("trend_micro_vision_one.detection.event_time")
                    && event.get_str("trend_micro_vision_one.detection.event_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.detection.event_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.detection.event_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.detection.event_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_event_time")?;
                    event.remove("trend_micro_vision_one.detection.event_time");
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
                event.has_value("trend_micro_vision_one.detection.event_time_dt")
                    && event.get_str("trend_micro_vision_one.detection.event_time_dt") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.detection.event_time_dt")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => event
                                .set("trend_micro_vision_one.detection.event_time_dt", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.detection.event_time_dt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_event_time_dt")?;
                    event.remove("trend_micro_vision_one.detection.event_time_dt");
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
                event.has_value("trend_micro_vision_one.detection.end_time")
                    && event.get_str("trend_micro_vision_one.detection.end_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.detection.end_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.detection.end_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.detection.end_time".into(),
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
                    event.remove("trend_micro_vision_one.detection.end_time");
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
                event.has_value("trend_micro_vision_one.detection.rt")
                    && event.get_str("trend_micro_vision_one.detection.rt") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.detection.rt")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.detection.rt", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.detection.rt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_rt")?;
                    event.remove("trend_micro_vision_one.detection.rt");
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
                event.has_value("trend_micro_vision_one.detection.rt_utc")
                    && event.get_str("trend_micro_vision_one.detection.rt_utc") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.detection.rt_utc")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.detection.rt_utc", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.detection.rt_utc".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_rt_utc")?;
                    event.remove("trend_micro_vision_one.detection.rt_utc");
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
                event.has_value("trend_micro_vision_one.detection.rt_date")
                    && event.get_str("trend_micro_vision_one.detection.rt_date") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.detection.rt_date")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("trend_micro_vision_one.detection.rt_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.detection.rt_date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_rt_date")?;
                    event.remove("trend_micro_vision_one.detection.rt_date");
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
                event.has_value("trend_micro_vision_one.detection.log_received_time")
                    && event.get_str("trend_micro_vision_one.detection.log_received_time")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("trend_micro_vision_one.detection.log_received_time")
                    {
                        match parse_date_out(&date_str, &["ISO8601", "UNIX_MS"], None, None) {
                            Some(parsed) => event.set(
                                "trend_micro_vision_one.detection.log_received_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "trend_micro_vision_one.detection.log_received_time"
                                        .into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_log_received_time")?;
                    event.remove("trend_micro_vision_one.detection.log_received_time");
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
                if event.has_value("trend_micro_vision_one.detection.endpoint.ip") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.endpoint.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.endpoint.ip".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.endpoint.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_endpoint_ip_to_ip",
                )?;
                event.remove("trend_micro_vision_one.detection.endpoint.ip");
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
                if event.has_value("trend_micro_vision_one.detection.destination.ip") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.destination.ip")
                    {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.destination.ip".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.destination.ip", converted)?;
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
                event.remove("trend_micro_vision_one.detection.destination.ip");
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
                if event.has_value("trend_micro_vision_one.detection.destination.port") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.detection.destination.port")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.destination.port".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.detection.destination.port",
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
                    "convert_destination_port_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.destination.port");
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
                if event.has_value("trend_micro_vision_one.detection.source.ip") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.source.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.source.ip".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.source.ip", converted)?;
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
                event.remove("trend_micro_vision_one.detection.source.ip");
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
                if event.has_value("trend_micro_vision_one.detection.source.port") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.source.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.source.port".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.source.port", converted)?;
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
                event.remove("trend_micro_vision_one.detection.source.port");
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
                if event.has_value("trend_micro_vision_one.detection.file_size") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.file_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.file_size".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.file_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_file_size_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.file_size");
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
                if event.has_value("trend_micro_vision_one.detection.compressed_file_size") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.detection.compressed_file_size")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.compressed_file_size"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.detection.compressed_file_size",
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
                    "convert_compressed_file_size_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.compressed_file_size");
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
                if event.has_value("trend_micro_vision_one.detection.process.pid") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.process.pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.process.pid".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.process.pid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_process_pid_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.process.pid");
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
                if event.has_value("trend_micro_vision_one.detection.object.pid") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.object.pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.object.pid".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.object.pid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_object_pid_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.object.pid");
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
                if event.has_value("trend_micro_vision_one.detection.device.ip") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.device.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.device.ip".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.device.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_device_ip_to_ip",
                )?;
                event.remove("trend_micro_vision_one.detection.device.ip");
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
                if event.has_value("trend_micro_vision_one.detection.interested.ip") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.interested.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.interested.ip".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.interested.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_interested_ip_to_ip",
                )?;
                event.remove("trend_micro_vision_one.detection.interested.ip");
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
                if event.has_value("trend_micro_vision_one.detection.peer.ip") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.peer.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.peer.ip".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.peer.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_peer_ip_to_ip")?;
                event.remove("trend_micro_vision_one.detection.peer.ip");
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
                if event.has_value("trend_micro_vision_one.detection.client_ip") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.client_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.client_ip".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.client_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_client_ip_to_ip",
                )?;
                event.remove("trend_micro_vision_one.detection.client_ip");
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
                if event.has_value("trend_micro_vision_one.detection.risk_level") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.risk_level") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.risk_level".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.risk_level", converted)?;
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
                event.remove("trend_micro_vision_one.detection.risk_level");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.risk_level") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def cccaRiskLevel = ctx.trend_micro_vision_one.detection.risk_level;\nif (params.cccaRiskLevel.containsKey(cccaRiskLevel.toString())) {\n  ctx.trend_micro_vision_one.detection.risk_level_value = params.cccaRiskLevel[cccaRiskLevel.toString()];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def cccaRiskLevel = ctx.trend_micro_vision_one.detection.risk_level;\nif (params.cccaRiskLevel.containsKey(cccaRiskLevel.toString())) {\n  ctx.trend_micro_vision_one.detection.risk_level_value = params.cccaRiskLevel[cccaRiskLevel.toString()];\n}"#
                        ),
                        cached_params!(
                            "{\"cccaRiskLevel\":{\"0\":\"SLF_CCCA_RISKLEVEL_UNKNOWN\",\"1\":\"SLF_CCCA_RISKLEVEL_LOW\",\"2\":\"SLF_CCCA_RISKLEVEL_MEDIUM\",\"3\":\"SLF_CCCA_RISKLEVEL_HIGH\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_resolve_risk_level_to_text_value",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("trend_micro_vision_one.detection.total_count") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.total_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.total_count".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.total_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_total_count_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.total_count");
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
                if event.has_value("trend_micro_vision_one.detection.aggregated_count") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.detection.aggregated_count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.aggregated_count".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.detection.aggregated_count",
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
                    "convert_aggregated_count_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.aggregated_count");
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
                if event.has_value("trend_micro_vision_one.detection.severity") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.severity".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.severity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_severity_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.severity");
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
                if event.has_value("trend_micro_vision_one.detection.severity_level") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.severity_level")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.severity_level".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.severity_level", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_severity_level_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.severity_level");
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
                if event.has_value("trend_micro_vision_one.detection.rt_hour") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.rt_hour") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.rt_hour".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.rt_hour", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_rt_hour_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.rt_hour");
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
                if event.has_value("trend_micro_vision_one.detection.potential_risk") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.potential_risk")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.potential_risk".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.potential_risk", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_potential_risk_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.potential_risk");
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
                if event.has_value("trend_micro_vision_one.detection.device.risk_confidence_level")
                {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.detection.device.risk_confidence_level")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "trend_micro_vision_one.detection.device.risk_confidence_level"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.detection.device.risk_confidence_level",
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
                    "convert_device_risk_confidence_level_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.device.risk_confidence_level");
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
                if event.has_value("trend_micro_vision_one.detection.mail_msg_direction") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.detection.mail_msg_direction")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.mail_msg_direction".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.detection.mail_msg_direction",
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
                    "convert_mail_msg_direction_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.mail_msg_direction");
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
                event
                    .get("trend_micro_vision_one.detection.attachment")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.attachment",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.attachment_file_size") {
                                if let Some(val) = event.get("_ingest._value.attachment_file_size")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.attachment_file_size".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.attachment_file_size", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_attachment_file_size_to_long",
                            )?;
                            event.remove("_ingest._value.attachment_file_size");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("trend_micro_vision_one.detection.attachment_file_sizes") {
                    if let Some(val) =
                        event.get("trend_micro_vision_one.detection.attachment_file_sizes")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.attachment_file_sizes"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "trend_micro_vision_one.detection.attachment_file_sizes",
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
                    "convert_attachment_file_sizes_to_long",
                )?;
                event.remove("trend_micro_vision_one.detection.attachment_file_sizes");
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

            if event.has_value("trend_micro_vision_one.detection.v_lanid") {
                if let Some(val) = event.get("trend_micro_vision_one.detection.v_lanid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.detection.v_lanid".into(),
                            message,
                        }
                    })?;
                    event.set("trend_micro_vision_one.detection.v_lanid", converted)?;
                }
            }

            if event.has_value("trend_micro_vision_one.detection.threat_type") {
                if let Some(val) = event.get("trend_micro_vision_one.detection.threat_type") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.detection.threat_type".into(),
                            message,
                        }
                    })?;
                    event.set("trend_micro_vision_one.detection.threat_type", converted)?;
                }
            }

            if event.has_value("trend_micro_vision_one.detection.source.zone") {
                if let Some(val) = event.get("trend_micro_vision_one.detection.source.zone") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.detection.source.zone".into(),
                            message,
                        }
                    })?;
                    event.set("trend_micro_vision_one.detection.source.zone", converted)?;
                }
            }

            if event.has_value("trend_micro_vision_one.detection.rule_id") {
                if let Some(val) = event.get("trend_micro_vision_one.detection.rule_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.detection.rule_id".into(),
                            message,
                        }
                    })?;
                    event.set("trend_micro_vision_one.detection.rule_id", converted)?;
                }
            }

            if event.has_value("trend_micro_vision_one.detection.event_source_type") {
                if let Some(val) = event.get("trend_micro_vision_one.detection.event_source_type") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.detection.event_source_type".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "trend_micro_vision_one.detection.event_source_type",
                        converted,
                    )?;
                }
            }

            if event.has_value("trend_micro_vision_one.detection.destination.zone") {
                if let Some(val) = event.get("trend_micro_vision_one.detection.destination.zone") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.detection.destination.zone".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "trend_micro_vision_one.detection.destination.zone",
                        converted,
                    )?;
                }
            }

            if event.has_value("trend_micro_vision_one.detection.dce_hash1") {
                if let Some(val) = event.get("trend_micro_vision_one.detection.dce_hash1") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.detection.dce_hash1".into(),
                            message,
                        }
                    })?;
                    event.set("trend_micro_vision_one.detection.dce_hash1", converted)?;
                }
            }

            if event.has_value("trend_micro_vision_one.detection.dce_hash2") {
                if let Some(val) = event.get("trend_micro_vision_one.detection.dce_hash2") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.detection.dce_hash2".into(),
                            message,
                        }
                    })?;
                    event.set("trend_micro_vision_one.detection.dce_hash2", converted)?;
                }
            }

            if event.has_value("trend_micro_vision_one.detection.data_type") {
                if let Some(val) = event.get("trend_micro_vision_one.detection.data_type") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "trend_micro_vision_one.detection.data_type".into(),
                            message,
                        }
                    })?;
                    event.set("trend_micro_vision_one.detection.data_type", converted)?;
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("trend_micro_vision_one.detection.sender_ip") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.sender_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.sender_ip".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.sender_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_sender_ip_to_ip",
                )?;
                event.remove("trend_micro_vision_one.detection.sender_ip");
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
                if event.has_value("trend_micro_vision_one.detection.dvc") {
                    if let Some(val) = event.get("trend_micro_vision_one.detection.dvc") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "trend_micro_vision_one.detection.dvc".into(),
                                message,
                            }
                        })?;
                        event.set("trend_micro_vision_one.detection.dvc", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dvc_to_ip")?;
                event.remove("trend_micro_vision_one.detection.dvc");
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("trend_micro_vision_one.detection.request") {
                    uri_parts(
                        event,
                        "trend_micro_vision_one.detection.request",
                        "url",
                        true,
                        false,
                    )?;
                }
                Ok(())
            })();

            if event.has_value("trend_micro_vision_one.detection.request_client_application") {
                if let Some(ua_str) =
                    event.get_string("trend_micro_vision_one.detection.request_client_application")
                {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
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

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.event_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("intrusion_detection"))?;

            event.append("event.type", json!("info"))?;

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.action")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.event_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.domain.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.destination.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.destination.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.destination.mac")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.mac", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.destination.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.source.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.source.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.source.mac")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.mac", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.source.host")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.source.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.client_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.ip", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.org_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.endpoint.guid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.endpoint.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.ip", v)?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.endpoint.mac") };
            if _cond {
                event.append_unique(
                    "host.mac",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.endpoint.mac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.endpoint.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.os.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.device.host")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.device.mac") };
            if _cond {
                event.append_unique(
                    "observer.mac",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.device.mac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.msg_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.message_id", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.msg_uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.local_id", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.mail_msg_subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.http_referer")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.referrer", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.device.direction")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.direction", v)?;
            }

            if event.has_value("network.direction") {
                map_strings(
                    event,
                    "network.direction",
                    "network.direction",
                    str::to_lowercase,
                )?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.protocol")
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
                .get("trend_micro_vision_one.detection.object.file.hash.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.object.file.hash.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.object.file.hash.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.file_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.file_path_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.file_size")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.size", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.file_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.type", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.process.cmd")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.process.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.process.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.rule_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.tactic_id")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.tactic_id",
                    |event| {
                        event.append_unique(
                            "threat.tactic.id",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = { event.has_value("threat.tactic.id") };
            if _cond {
                event.set("threat.framework", json!("MITRE ATT&CK"))?;
            }

            if let Some(v) = event
                .get("trend_micro_vision_one.detection.user.domain")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.detection.object.file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.object.file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.detection.object.file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.object.file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.detection.object.file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.object.file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.file_hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.file_hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.detection.parent.file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.parent.file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.detection.parent.file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.parent.file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.detection.process.file.hash.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.process.file.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.detection.process.file.hash.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.process.file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("trend_micro_vision_one.detection.process.file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.process.file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.endpoint.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.endpoint.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.device.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.device.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.source.host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.source.host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.dvchost") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.dvchost")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("trend_micro_vision_one.detection.sam_user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("trend_micro_vision_one.detection.sam_user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.source.user")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.source.user",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.destination.user")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.destination.user",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.attachment")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.attachment",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.attachment_file_hash")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.attachment")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.attachment",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.attachment_file_tlsh")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.attachment_file_hashes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.attachment_file_hashes",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.attachment_file_hashs")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.attachment_file_hashs",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.attachment_file_tlshes")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.attachment_file_tlshes",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.attachment_file_tlshs")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.attachment_file_tlshs",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.sender_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.sender_ip",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.dvc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "trend_micro_vision_one.detection.dvc", |event| {
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

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.destination.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.destination.ip",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.source.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.source.ip",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.device.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.device.ip",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.interested.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.interested.ip",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.peer.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "trend_micro_vision_one.detection.peer.ip", |event| {
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

            let _cond = {
                event
                    .get("trend_micro_vision_one.detection.client_ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "trend_micro_vision_one.detection.client_ip",
                    |event| {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            event.remove("json");
            event.remove("trend_micro_vision_one.detection.apt_related");

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
                event.remove("trend_micro_vision_one.detection.event_time");
                event.remove("trend_micro_vision_one.detection.domain.name");
                event.remove("trend_micro_vision_one.detection.destination.ip");
                event.remove("trend_micro_vision_one.detection.destination.port");
                event.remove("trend_micro_vision_one.detection.action");
                event.remove("trend_micro_vision_one.detection.event_id");
                event.remove("trend_micro_vision_one.detection.object.file.hash.md5");
                event.remove("trend_micro_vision_one.detection.object.file.hash.sha1");
                event.remove("trend_micro_vision_one.detection.object.file.hash.sha256");
                event.remove("trend_micro_vision_one.detection.file_name");
                event.remove("trend_micro_vision_one.detection.file_path_name");
                event.remove("trend_micro_vision_one.detection.file_size");
                event.remove("trend_micro_vision_one.detection.hostname");
                event.remove("trend_micro_vision_one.detection.endpoint.guid");
                event.remove("trend_micro_vision_one.detection.endpoint.ip");
                event.remove("trend_micro_vision_one.detection.endpoint.mac");
                event.remove("trend_micro_vision_one.detection.endpoint.hostname");
                event.remove("trend_micro_vision_one.detection.http_referer");
                event.remove("trend_micro_vision_one.detection.device.direction");
                event.remove("trend_micro_vision_one.detection.protocol");
                event.remove("trend_micro_vision_one.detection.device.host");
                event.remove("trend_micro_vision_one.detection.device.mac");
                event.remove("trend_micro_vision_one.detection.process.cmd");
                event.remove("trend_micro_vision_one.detection.process.name");
                event.remove("trend_micro_vision_one.detection.process.pid");
                event.remove("trend_micro_vision_one.detection.source.ip");
                event.remove("trend_micro_vision_one.detection.source.port");
                event.remove("trend_micro_vision_one.detection.source.mac");
                event.remove("trend_micro_vision_one.detection.source.host");
                event.remove("trend_micro_vision_one.detection.source.user");
                event.remove("trend_micro_vision_one.detection.rule_id");
                event.remove("trend_micro_vision_one.detection.rule_name");
                event.remove("trend_micro_vision_one.detection.destination.mac");
                event.remove("trend_micro_vision_one.detection.destination.user");
                event.remove("trend_micro_vision_one.detection.org_id");
                event.remove("trend_micro_vision_one.detection.msg_id");
                event.remove("trend_micro_vision_one.detection.msg_uuid");
                event.remove("trend_micro_vision_one.detection.mail_msg_subject");
                event.remove("trend_micro_vision_one.detection.tactic_id");
                event.remove("trend_micro_vision_one.detection.file_type");
                event.remove("trend_micro_vision_one.detection.os.name");
                event.remove("trend_micro_vision_one.detection.user.domain");
                event.remove("trend_micro_vision_one.detection.client_ip");
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
