// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_passed_authentications` pipeline.
pub struct PipelinePassedAuthentications;

impl Transform for PipelinePassedAuthentications {
    fn name(&self) -> &str {
        "pipeline_passed_authentications"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.get_i64("cisco_ise.log.segment.number") == Some(0) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},
                    if !cached_grok!("^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = { event.has_value("cisco_ise.log.segment.number") && event.get_i64("cisco_ise.log.segment.number").is_some_and(|n| n > 0) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{GREEDYDATA:cisco_ise.log.log_details_raw},
                    if !cached_grok!("^%{GREEDYDATA:cisco_ise.log.log_details_raw},").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSSSSS", "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date__tmp_timestamp_9ef85c6a")?;
                        event.remove("_tmp.timestamp");
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("event.timezone") && event.get_str("event.timezone") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSSSSS", "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date__tmp_timestamp_1d2a12b9")?;
                        event.remove("_tmp.timestamp");
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("cisco_ise.log.message.description") && event.get_str("cisco_ise.log.message.description") != Some("") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                    // Grok pattern: ^%{DATA:event.action}:
                    if !cached_grok!("^%{DATA:event.action}:").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5200", "5201", "5206", "5231", "5233", "5235", "5237", "5238", "5239", "5240"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5200", "5201", "5206", "5231", "5233", "5235", "5237", "5238", "5239", "5240"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("success"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5404", "5434", "5413"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("failure"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5200", "5201", "5206", "5231", "5233", "5235", "5237", "5238", "5239", "5240"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("info"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5202", "5203"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("authentication"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5202", "5203"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5202", "5203"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("allowed"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5202", "5203"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("success"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5204") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("iam"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5204") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("user"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5204") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("change"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5204") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("success"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5205") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5205") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("info"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5205") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("success"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5232", "5234", "5236"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5232", "5234", "5236"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("allowed"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && ["5232", "5234", "5236"].contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("success"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5241") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.category", json!("network"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5241") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append("event.type", json!("connection"))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.message.code") && event.get_str("cisco_ise.log.message.code") == Some("5241") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
            event.set("event.outcome", json!("success"))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                    let mut kv_gap = false;
                    for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "cisco_ise.log.log_details_raw".into(),
                                split: "=".into(),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.log_details.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details.Response") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("{") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find("}") else { break 'dissect false };
                        captured.push(("_tmp.response", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("}") else { break 'dissect false };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

                event.remove("cisco_ise.log.log_details.Response");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("_tmp.response") {
                    let mut kv_gap = false;
                    for pair in kv_str.split("; ") {
                        if pair.is_empty() {
                            kv_gap = true;
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap) else {
                            return Err(TransformError::KvValueSplit {
                                field: "_tmp.response".into(),
                                split: "=".into(),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.response.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

                if event.has_value("cisco_ise.log.log_details.AcsSessionID") {
                    event.rename("cisco_ise.log.log_details.AcsSessionID", "cisco_ise.log.acs.session.id")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.Airespace-Wlan-Id") {
                if let Some(val) = event.get("cisco_ise.log.log_details.Airespace-Wlan-Id") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.Airespace-Wlan-Id".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.airespace.wlan.id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Airespace-Wlan-Id_to_cisco_ise_log_airespace_wlan_id_86981e05")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.Airespace-Wlan-Id");

                if event.has_value("cisco_ise.log.log_details.allowEasyWiredSession") {
                    event.rename("cisco_ise.log.log_details.allowEasyWiredSession", "cisco_ise.log.allow.easy.wired.session")?;
                }

                if event.has_value("cisco_ise.log.log_details.AuthorizationPolicyMatchedRule") {
                    event.rename("cisco_ise.log.log_details.AuthorizationPolicyMatchedRule", "cisco_ise.log.auth.policy.matched.rule")?;
                }

                if event.has_value("cisco_ise.log.log_details.AuthenticationIdentityStore") {
                    event.rename("cisco_ise.log.log_details.AuthenticationIdentityStore", "cisco_ise.log.authentication.identity_store")?;
                }

                if event.has_value("cisco_ise.log.log_details.AuthenticationMethod") {
                    event.rename("cisco_ise.log.log_details.AuthenticationMethod", "cisco_ise.log.authentication.method")?;
                }

                if event.has_value("cisco_ise.log.log_details.AuthenticationStatus") {
                    event.rename("cisco_ise.log.log_details.AuthenticationStatus", "cisco_ise.log.authentication.status")?;
                }

                if event.has_value("cisco_ise.log.log_details.Calling-Station-ID") {
                    event.rename("cisco_ise.log.log_details.Calling-Station-ID", "cisco_ise.log.calling_station.id")?;
                }

                // Painless script
                // Source: if (ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\") == null) {\n  return;\n}\nif (ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\") instanceof String) {\n  ctx.cisco_ise.log.log_details[\"cisco-av-pair\"] = [ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\")];\n}\n\ndef attributes = [:];\nctx.cisco_ise.log.log_details.get(\"cisco-av-pair\")?.forEach((v) -> {\n  def firstEq = v.indexOf('=');\n  if (firstEq <= 0) {\n    return true;\n  }\n  def topKey = v.substring(0, firstEq).trim();\n  def rest = v.substring(firstEq + 1);\n\n  // Only mdm-tlv uses nested subkeys (device-platform=win, ...). Other pairs\n  // (e.g. FQSubjectName=...cn=...,ou=...) may contain '=' in the value and must stay scalar.\n  if (!\"mdm-tlv\".equals(topKey)) {\n    attributes[topKey] = rest.trim();\n    return true;\n  }\n\n  def inEscape = false;\n  def start = 0;\n  def key = \"\";\n  if (!attributes.containsKey(\"mdm-tlv\")) {\n    attributes[\"mdm-tlv\"] = [:];\n  }\n  def m = attributes[\"mdm-tlv\"];\n\n  for (def i = 0, n = rest.length(); i < n; ++i) {\n    def c = rest.charAt(i);\n    if (inEscape) {\n      inEscape = false;\n      continue;\n    }\n    if (c == (char)'\\\\') {\n      inEscape = true;\n      continue;\n    }\n\n    if (c == (char)'=') {\n      key = rest.substring(start, i).trim();\n      def nextSplit = rest.indexOf(\"=\", i + 1);\n      if (nextSplit != -1 && rest.charAt(nextSplit - 1) != (char)'\\\\') {\n        if (!m.containsKey(key)) {\n          m[key] = [:];\n        }\n        m = m[key];\n      }\n      start = i + 1;\n    }\n    if (i == n - 1) {\n      m[key] = rest.substring(start, n).trim();\n    }\n  }\n\n  return true;\n});\n\nif (attributes.size() > 0) {\n  ctx.cisco_ise.log[\"cisco_av_pair\"] = attributes;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"if (ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\") == null) {\n  return;\n}\nif (ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\") instanceof String) {\n  ctx.cisco_ise.log.log_details[\"cisco-av-pair\"] = [ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\")];\n}\n\ndef attributes = [:];\nctx.cisco_ise.log.log_details.get(\"cisco-av-pair\")?.forEach((v) -> {\n  def firstEq = v.indexOf('=');\n  if (firstEq <= 0) {\n    return true;\n  }\n  def topKey = v.substring(0, firstEq).trim();\n  def rest = v.substring(firstEq + 1);\n\n  // Only mdm-tlv uses nested subkeys (device-platform=win, ...). Other pairs\n  // (e.g. FQSubjectName=...cn=...,ou=...) may contain '=' in the value and must stay scalar.\n  if (!\"mdm-tlv\".equals(topKey)) {\n    attributes[topKey] = rest.trim();\n    return true;\n  }\n\n  def inEscape = false;\n  def start = 0;\n  def key = \"\";\n  if (!attributes.containsKey(\"mdm-tlv\")) {\n    attributes[\"mdm-tlv\"] = [:];\n  }\n  def m = attributes[\"mdm-tlv\"];\n\n  for (def i = 0, n = rest.length(); i < n; ++i) {\n    def c = rest.charAt(i);\n    if (inEscape) {\n      inEscape = false;\n      continue;\n    }\n    if (c == (char)'\\\\') {\n      inEscape = true;\n      continue;\n    }\n\n    if (c == (char)'=') {\n      key = rest.substring(start, i).trim();\n      def nextSplit = rest.indexOf(\"=\", i + 1);\n      if (nextSplit != -1 && rest.charAt(nextSplit - 1) != (char)'\\\\') {\n        if (!m.containsKey(key)) {\n          m[key] = [:];\n        }\n        m = m[key];\n      }\n      start = i + 1;\n    }\n    if (i == n - 1) {\n      m[key] = rest.substring(start, n).trim();\n    }\n  }\n\n  return true;\n});\n\nif (attributes.size() > 0) {\n  ctx.cisco_ise.log[\"cisco_av_pair\"] = attributes;\n}\n"#))?;

                event.remove("cisco_ise.log.log_details.cisco-av-pair");

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.cisco_av_pair.coa-push") {
                if let Some(val) = event.get("cisco_ise.log.cisco_av_pair.coa-push") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.cisco_av_pair.coa-push".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.cisco_av_pair.coa-push", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_cisco_av_pair_coa-push_to_cisco_ise_log_cisco_av_pair_coa-push_17c73a21")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                        if event.remove("cisco_ise.log.cisco_av_pair.coa-push").is_none() {
                            return Err(TransformError::FieldNotFound { path: "cisco_ise.log.cisco_av_pair.coa-push".into() });
                        }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.cisco-av-pair.cts-environment-version") {
                if let Some(val) = event.get("cisco_ise.log.cisco-av-pair.cts-environment-version") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.cisco-av-pair.cts-environment-version".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.cisco-av-pair.cts-environment-version", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_cisco-av-pair_cts-environment-version_to_cisco_ise_log_cisco-av-pair_cts-environment-version_0f389da9")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                        event.remove("cisco_ise.log.cisco_av_pair.cts-environment-version");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.ClientLatency") {
                if let Some(val) = event.get("cisco_ise.log.log_details.ClientLatency") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.ClientLatency".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.client.latency", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ClientLatency_to_cisco_ise_log_client_latency_108a319b")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.ClientLatency");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.CPMSessionID") {
                    event.rename("cisco_ise.log.log_details.CPMSessionID", "cisco_ise.log.cpm.session.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Device Type") {
                    event.rename("cisco_ise.log.log_details.Device Type", "cisco_ise.log.device.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.DTLSSupport") {
                    event.rename("cisco_ise.log.log_details.DTLSSupport", "cisco_ise.log.dtls_support")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.EndPointMACAddress") {
                    event.rename("cisco_ise.log.log_details.EndPointMACAddress", "cisco_ise.log.endpoint.mac.address")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if let Some(v) = event.get("cisco_ise.log.endpoint.mac.address").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("client.mac", v)?;
            }
                Ok(())
            })();

            if event.has_value("cisco_ise.log.endpoint.mac.address") {
                gsub_field(event, "cisco_ise.log.endpoint.mac.address", "cisco_ise.log.endpoint.mac.address", cached_regex!("[-:.]"), "-")?;
            }

            if event.has_value("cisco_ise.log.endpoint.mac.address") {
                map_strings(event, "cisco_ise.log.endpoint.mac.address", "cisco_ise.log.endpoint.mac.address", str::to_uppercase)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.GuestUserName") {
                    event.rename("cisco_ise.log.log_details.GuestUserName", "cisco_ise.log.guest.user.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.IdentityGroup") {
                    event.rename("cisco_ise.log.log_details.IdentityGroup", "cisco_ise.log.identity.group")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.IdentityPolicyMatchedRule") {
                    event.rename("cisco_ise.log.log_details.IdentityPolicyMatchedRule", "cisco_ise.log.identity.policy.matched.rule")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.IpAddress") {
                if let Some(val) = event.get("cisco_ise.log.log_details.IpAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.IpAddress".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_IpAddress_to_source_ip_e43c3b18")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.IpAddress");

            let _cond = { event.has_value("source.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.IPSEC") {
                    event.rename("cisco_ise.log.log_details.IPSEC", "cisco_ise.log.ipsec")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.IsThirdPartyDeviceFlow") {
                if let Some(val) = event.get("cisco_ise.log.log_details.IsThirdPartyDeviceFlow") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.IsThirdPartyDeviceFlow".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.is_third_party_device_flow", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_IsThirdPartyDeviceFlow_to_cisco_ise_log_is_third_party_device_flow_7674ad73")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.IsThirdPartyDeviceFlow");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.ISEPolicySetName") {
                    event.rename("cisco_ise.log.log_details.ISEPolicySetName", "cisco_ise.log.ise.policy.set_name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Location") {
                    event.rename("cisco_ise.log.log_details.Location", "cisco_ise.log.location")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.MisconfiguredClientFixReason") {
                    event.rename("cisco_ise.log.log_details.MisconfiguredClientFixReason", "cisco_ise.log.misconfigured.client.fix.reason")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Model Name") {
                    event.rename("cisco_ise.log.log_details.Model Name", "cisco_ise.log.model.name")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.NAS-IP-Address") {
                if let Some(val) = event.get("cisco_ise.log.log_details.NAS-IP-Address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.NAS-IP-Address".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.nas.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-IP-Address_to_cisco_ise_log_nas_ip_ae27d25e")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.NAS-IP-Address");

            let _cond = { event.has_value("cisco_ise.log.nas.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("cisco_ise.log.nas.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.NAS-Port") {
                if let Some(val) = event.get("cisco_ise.log.log_details.NAS-Port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.NAS-Port".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.nas.port.number", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-Port_to_cisco_ise_log_nas_port_number_8385ddaf")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.NAS-Port");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NAS-Port-Id") {
                    event.rename("cisco_ise.log.log_details.NAS-Port-Id", "cisco_ise.log.nas.port.id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NAS-Port-Type") {
                    event.rename("cisco_ise.log.log_details.NAS-Port-Type", "cisco_ise.log.nas.port.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceGroups") {
                    event.rename("cisco_ise.log.log_details.NetworkDeviceGroups", "cisco_ise.log.network.device.groups")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Network Device Profile") {
                    event.rename("cisco_ise.log.log_details.Network Device Profile", "cisco_ise.log.network.device.profile")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceProfileName") {
                    event.rename("cisco_ise.log.log_details.NetworkDeviceProfileName", "cisco_ise.log.network.device.profile_name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceProfileId") {
                    event.rename("cisco_ise.log.log_details.NetworkDeviceProfileId", "cisco_ise.log.network.device.profile_id")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceName") {
                    event.rename("cisco_ise.log.log_details.NetworkDeviceName", "cisco_ise.log.network.device.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.PortalName") {
                    event.rename("cisco_ise.log.log_details.PortalName", "cisco_ise.log.portal.name")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.PostureAssessmentStatus") {
                    event.rename("cisco_ise.log.log_details.PostureAssessmentStatus", "cisco_ise.log.posture.assessment.status")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.PsnHostName") {
                    event.rename("cisco_ise.log.log_details.PsnHostName", "cisco_ise.log.psn.hostname")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.RadiusFlowType") {
                    event.rename("cisco_ise.log.log_details.RadiusFlowType", "cisco_ise.log.radius.flow.type")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.RequestLatency") {
                if let Some(val) = event.get("cisco_ise.log.log_details.RequestLatency") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.RequestLatency".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.request.latency", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_RequestLatency_to_cisco_ise_log_request_latency_bafbebd7")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.RequestLatency");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.ResponseTime") {
                if let Some(val) = event.get("cisco_ise.log.log_details.ResponseTime") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.ResponseTime".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.response.time", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ResponseTime_to_cisco_ise_log_response_time_1c336c93")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.ResponseTime");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.SelectedAccessService") {
                    event.rename("cisco_ise.log.log_details.SelectedAccessService", "cisco_ise.log.selected.access.service")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.SelectedAuthenticationIdentityStores") {
                    event.rename("cisco_ise.log.log_details.SelectedAuthenticationIdentityStores", "cisco_ise.log.selected.authentication.identity_stores")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.SelectedAuthorizationProfiles") {
                    event.rename("cisco_ise.log.log_details.SelectedAuthorizationProfiles", "cisco_ise.log.selected.authorization.profiles")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.IdentitySelectionMatchedRule") {
                    event.rename("cisco_ise.log.log_details.IdentitySelectionMatchedRule", "cisco_ise.log.identity.selection.matched.rule")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Service-Type") {
                    event.rename("cisco_ise.log.log_details.Service-Type", "cisco_ise.log.service.type")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Step") {
                    event.rename("cisco_ise.log.log_details.Step", "cisco_ise.log.step")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.StepData") {
                    event.rename("cisco_ise.log.log_details.StepData", "cisco_ise.log.step_data")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.TotalAuthenLatency") {
                if let Some(val) = event.get("cisco_ise.log.log_details.TotalAuthenLatency") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.TotalAuthenLatency".into(),
                            message,
                        })?;
                    event.set("cisco_ise.log.total.authen.latency", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_TotalAuthenLatency_to_cisco_ise_log_total_authen_latency_bebbe609")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.TotalAuthenLatency");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.UseCase") {
                    event.rename("cisco_ise.log.log_details.UseCase", "cisco_ise.log.usecase")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.UserType") {
                    event.rename("cisco_ise.log.log_details.UserType", "cisco_ise.log.user.type")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("cisco_ise.log.log_details.UserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("cisco_ise.log.log_details.UserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                event.remove("cisco_ise.log.log_details.UserName");

            let _cond = { event.has_value("cisco_ise.log.log_details") && event.has_value("cisco_ise.log.log_details.User-Name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("cisco_ise.log.log_details.User-Name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.log_details") && event.has_value("cisco_ise.log.log_details.User-Name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("cisco_ise.log.log_details.User-Name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                event.remove("cisco_ise.log.log_details.User-Name");

            let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("user.name", json!(event.get("cisco_ise.log.log_details.OriginalUserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("cisco_ise.log.log_details.OriginalUserName").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

                event.remove("cisco_ise.log.log_details.OriginalUserName");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.DestinationIPAddress") {
                if let Some(val) = event.get("cisco_ise.log.log_details.DestinationIPAddress") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.DestinationIPAddress".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationIPAddress_to_destination_ip_a431dedf")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.DestinationIPAddress");

            let _cond = { event.has_value("destination.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.DestinationPort") {
                if let Some(val) = event.get("cisco_ise.log.log_details.DestinationPort") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.DestinationPort".into(),
                            message,
                        })?;
                    event.set("destination.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationPort_to_destination_port_ad144c54")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.DestinationPort");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_ise.log.log_details.Device IP Address") {
                if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_ise.log.log_details.Device IP Address".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Device_IP_Address_to_client_ip_b34586ce")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cisco_ise.log.log_details.Device IP Address");

            let _cond = { event.has_value("client.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Protocol") {
                    event.rename("cisco_ise.log.log_details.Protocol", "network.protocol")?;
                }
                Ok(())
            })();

            if event.has_value("network.protocol") {
                map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
