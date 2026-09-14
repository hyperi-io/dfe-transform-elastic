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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        event.rename("message", "event.original")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("auditd.log.record_type") };
            if _cond {
                // Begin nested pipeline: "raw"
                let _cond = {
                    !event.has_value("auditd.log.record_type")
                        && event.has_value("event.original")
                        && event.get("event.original").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(" old auid="))
                            }
                            serde_json::Value::String(s) => s.contains(" old auid="),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("event.original") {
                            // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?P<auditd_log_kv>.*?)(?= old auid=) old auid=%{NUMBER:auditd.log.old_auid} new auid=%{NUMBER:auditd.log.new_auid} old ses=%{NUMBER:auditd.log.old_ses} new ses=%{NUMBER:auditd.log.new_ses}$
                            if !cached_grok_mapped!("(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?P<auditd_log_kv>.*?)(?= old auid=) old auid=%{NUMBER:auditd.log.old_auid} new auid=%{NUMBER:auditd.log.new_auid} old ses=%{NUMBER:auditd.log.old_ses} new ses=%{NUMBER:auditd.log.new_ses}$", [("auditd_log_kv", "auditd.log.kv")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    !event.has_value("auditd.log.record_type")
                        && event.has_value("event.original")
                        && (event.get("event.original").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(" msg='"))
                            }
                            serde_json::Value::String(s) => s.contains(" msg='"),
                            _ => false,
                        }) || event.get("event.original").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(" msg=\""))
                            }
                            serde_json::Value::String(s) => s.contains(" msg=\""),
                            _ => false,
                        }))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("event.original") {
                            // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?:user )?(?P<auditd_log_kv>.*?)(?= msg=) msg=(?:(?:'(?:(?:avc:%{SPACE}%{WORD:auditd.log.avc.action}%{SPACE}{%{SPACE}%{WORD:auditd.log.avc.request}%{SPACE}}%{SPACE}for%{SPACE})|[^=]*\\s)?(?P<auditd_log_sub_kv>(?:[^']*))'|\"([^=]*\\s)?(?P<auditd_log_sub_kv>(?:[^\"]*))\"))(?:(?:\\x1d)(?P<auditd_log_sub_kv_enriched>.*))?$
                            if !cached_grok_mapped!("(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?:user )?(?P<auditd_log_kv>.*?)(?= msg=) msg=(?:(?:'(?:(?:avc:%{SPACE}%{WORD:auditd.log.avc.action}%{SPACE}{%{SPACE}%{WORD:auditd.log.avc.request}%{SPACE}}%{SPACE}for%{SPACE})|[^=]*\\s)?(?P<auditd_log_sub_kv>(?:[^']*))'|\"([^=]*\\s)?(?P<auditd_log_sub_kv>(?:[^\"]*))\"))(?:(?:\\x1d)(?P<auditd_log_sub_kv_enriched>.*))?$", [("auditd_log_sub_kv", "auditd.log.sub_kv"), ("auditd_log_sub_kv", "auditd.log.sub_kv"), ("auditd_log_kv", "auditd.log.kv"), ("auditd_log_sub_kv_enriched", "auditd.log.sub_kv_enriched")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    !event.has_value("auditd.log.record_type")
                        && event.has_value("event.original")
                        && event.get("event.original").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("={ "))
                            }
                            serde_json::Value::String(s) => s.contains("={ "),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("event.original") {
                            // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?P<auditd_log_kv>.*?)(?=\\x1d%{WORD}=\\{ )\\x1d%{WORD:auditd.log.original_field}={ (?P<auditd_log_sub_kv>.*) }$
                            if !cached_grok_mapped!("(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?P<auditd_log_kv>.*?)(?=\\x1d%{WORD}=\\{ )\\x1d%{WORD:auditd.log.original_field}={ (?P<auditd_log_sub_kv>.*) }$", [("auditd_log_kv", "auditd.log.kv"), ("auditd_log_sub_kv", "auditd.log.sub_kv")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = { !event.has_value("auditd.log.record_type") };
                if _cond {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?:user )?(?:avc:%{SPACE}%{WORD:auditd.log.avc.action}%{SPACE}{%{SPACE}%{WORD:auditd.log.avc.request}%{SPACE}}%{SPACE}for%{SPACE})?(?:(?:.*?)(?P<auditd_log_kv>[A-Za-z0-9_]+=[\\s\\S]*))
                        // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*.*$
                        // Grok pattern: (?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*$
                        // Grok pattern: (?:type=%{NOTSPACE:auditd.log.record_type}) (?:(?:.*?)(?P<auditd_log_kv>[A-Za-z0-9_]+=[\\s\\S]*))
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*(?:user )?(?:avc:%{SPACE}%{WORD:auditd.log.avc.action}%{SPACE}{%{SPACE}%{WORD:auditd.log.avc.request}%{SPACE}}%{SPACE}for%{SPACE})?(?:(?:.*?)(?P<auditd_log_kv>[A-Za-z0-9_]+=[\\s\\S]*))",
                                    [("auditd_log_kv", "auditd.log.kv")]
                                ),
                                cached_grok!(
                                    "(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*.*$"
                                ),
                                cached_grok!(
                                    "(?:^(?:(?:node=%{IPORHOST:auditd.log.node} ))?(?:type=%{NOTSPACE:auditd.log.record_type}) msg=audit\\(%{NUMBER:auditd.log.epoch}:%{NUMBER:auditd.log.sequence}\\):)\\s*$"
                                ),
                                cached_grok_mapped!(
                                    "(?:type=%{NOTSPACE:auditd.log.record_type}) (?:(?:.*?)(?P<auditd_log_kv>[A-Za-z0-9_]+=[\\s\\S]*))",
                                    [("auditd_log_kv", "auditd.log.kv")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("auditd.log.sub_kv") {
                        gsub_field(
                            event,
                            "auditd.log.sub_kv",
                            "auditd.log.sub_kv",
                            cached_regex!("(res=[a-z]+)([A-Z][A-Za-z_]+=)"),
                            "$1 $2",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("auditd.log.kv") {
                        gsub_field(
                            event,
                            "auditd.log.kv",
                            "auditd.log.kv",
                            cached_regex!("(res=[a-z]+)([A-Z][A-Za-z_]+=)"),
                            "$1 $2",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("auditd.log.kv") };
                if _cond {
                    if let Some(kv_str) = event.get_string("auditd.log.kv") {
                        let mut kv_gap = false;
                        for pair in cached_regex!("(?:\\s+|\\x1d)(?=[^\\s\\x1d]+=)")
                            .split(&kv_str)
                            .into_iter()
                        {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = ({
                                let parts = cached_regex!("(?<!\\\\)=").splitn(&pair, 2);
                                match (parts.first(), parts.get(1)) {
                                    (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                    _ => None,
                                }
                            })
                            .filter(|_| !kv_gap) else {
                                return Err(TransformError::KvValueSplit {
                                    field: "auditd.log.kv".into(),
                                    split: "(?<!\\\\)=".into(),
                                });
                            };
                            {
                                let key = &key[..];
                                if !key.is_empty() {
                                    kv_put(event, &format!("auditd.log.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("auditd.log.sub_kv") {
                    if let Some(kv_str) = event.get_string("auditd.log.sub_kv") {
                        let mut kv_gap = false;
                        for pair in cached_regex!("\\s+(?=[^\\s]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "auditd.log.sub_kv".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("auditd.log.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("auditd.log.sub_kv_enriched") {
                    if let Some(kv_str) = event.get_string("auditd.log.sub_kv_enriched") {
                        let mut kv_gap = false;
                        for pair in cached_regex!("\\s+(?=[^\\s]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "auditd.log.sub_kv_enriched".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("auditd.log.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("auditd.log.epoch") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "auditd.log.epoch".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has_value("auditd.log.original_field") {
                    map_strings(
                        event,
                        "auditd.log.original_field",
                        "auditd.log.original_field",
                        str::to_lowercase,
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("auditd.log.old-auid", "auditd.log.old_auid")?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("auditd.log.old-ses", "auditd.log.old_ses")?;
                    Ok(())
                })();
                // End nested pipeline: "raw"
            }

            // Painless script
            // Source: String trimQuotes(def singleQuote, def doubleQuote, def v) {\n    if (v.startsWith(singleQuote) || v.startsWith(doubleQuote)) {\n        v = v.substring(1, v.length());\n    }\n    if (v.endsWith(singleQuote) || v.endsWith(doubleQuote)) {\n        v = v.substring(0, v.length()-1);\n    }\n    return v;\n}\n\nboolean isHexAscii(String v) {\n    def len = v.length();\n\n    if (len == 0 || len % 2 != 0) {\n        return false;\n    }\n\n    for (int i = 0 ; i < len ; i++) {\n        if (Character.digit(v.charAt(i), 16) == -1) {\n            return false;\n        }\n    }\n    return true;\n}\n\nString convertHexToString(String hex) {\n    StringBuilder sb = new StringBuilder();\n    boolean needed_encoding = false;\n\n    for (int i=0; i < hex.length() - 1; i+=2) {\n        int cp = Integer.parseInt(hex.substring(i, (i +2)), 16);\n        if (cp < 33 || cp == 34 || cp == 127) {\n            needed_encoding = true;\n        }\n        if (cp < 32 || cp == 127) {\n            sb.append('^');\n            cp ^= 64;\n        }\n        sb.append((char)cp);\n    }\n    if (needed_encoding) {\n        return sb.toString();\n    }\n    return hex;\n}\n\nBoolean convertStringToBoolean(String value) {\n    value = value.toLowerCase();\n    return value == \"yes\" || value == \"true\" || value == \"1\";\n}\n\n// processFieldValue processes the value for the field, and returns\n// the processed value if it is valid, otherwise null.\ndef processFieldValue(String k, def v, def possibleHexKeys, def possibleBooleanKeys) {\n    // Remove entries whose value is ?\n    if (v == \"?\" || v == \"(null)\" || v == \"\") {\n        return null;\n    }\n\n    // Convert hex values to ASCII.\n    if (possibleHexKeys.contains(k) && isHexAscii(v)) {\n        v = convertHexToString(v);\n    }\n\n    // Convert string values to boolean.\n    if (possibleBooleanKeys.contains(k) && v instanceof String) {\n        v = convertStringToBoolean(v);\n    }\n\n    // Trim quotes.\n    if (v instanceof String) {\n        v = trimQuotes(\"'\", \"\\\"\", v);\n    }\n\n    // Convert arch.\n    if (k == \"arch\" && v == \"c000003e\") {\n        v = \"x86_64\";\n    }\n\n    return v;\n}\n\ndef audit = ctx.auditd.get(\"log\");\nIterator entries = audit.entrySet().iterator();\n\nwhile (entries.hasNext()) {\n    def e = entries.next();\n    def k = e.getKey();\n    def v = e.getValue();\n\n    if (v instanceof List) {\n        int j = 0;\n        for (int i = 0; i < v.length; i++) {\n            v[j] = processFieldValue(k, v[i], params.possibleHexKeys, params.possibleBooleanKeys);\n            if (v[j] != null) {\n                j++;\n            }\n        }\n        if (j < v.length) {\n            if (j == 0) {\n                entries.remove();\n                continue;\n            }\n            audit.put(k, v.subList(0, j));\n        }\n        continue;\n    }\n\n    v = processFieldValue(k, v, params.possibleHexKeys, params.possibleBooleanKeys);\n    if (v == null) {\n        entries.remove();\n    } else {\n        audit.put(k, v);\n    }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"String trimQuotes(def singleQuote, def doubleQuote, def v) {\n    if (v.startsWith(singleQuote) || v.startsWith(doubleQuote)) {\n        v = v.substring(1, v.length());\n    }\n    if (v.endsWith(singleQuote) || v.endsWith(doubleQuote)) {\n        v = v.substring(0, v.length()-1);\n    }\n    return v;\n}\n\nboolean isHexAscii(String v) {\n    def len = v.length();\n\n    if (len == 0 || len % 2 != 0) {\n        return false;\n    }\n\n    for (int i = 0 ; i < len ; i++) {\n        if (Character.digit(v.charAt(i), 16) == -1) {\n            return false;\n        }\n    }\n    return true;\n}\n\nString convertHexToString(String hex) {\n    StringBuilder sb = new StringBuilder();\n    boolean needed_encoding = false;\n\n    for (int i=0; i < hex.length() - 1; i+=2) {\n        int cp = Integer.parseInt(hex.substring(i, (i +2)), 16);\n        if (cp < 33 || cp == 34 || cp == 127) {\n            needed_encoding = true;\n        }\n        if (cp < 32 || cp == 127) {\n            sb.append('^');\n            cp ^= 64;\n        }\n        sb.append((char)cp);\n    }\n    if (needed_encoding) {\n        return sb.toString();\n    }\n    return hex;\n}\n\nBoolean convertStringToBoolean(String value) {\n    value = value.toLowerCase();\n    return value == \"yes\" || value == \"true\" || value == \"1\";\n}\n\n// processFieldValue processes the value for the field, and returns\n// the processed value if it is valid, otherwise null.\ndef processFieldValue(String k, def v, def possibleHexKeys, def possibleBooleanKeys) {\n    // Remove entries whose value is ?\n    if (v == \"?\" || v == \"(null)\" || v == \"\") {\n        return null;\n    }\n\n    // Convert hex values to ASCII.\n    if (possibleHexKeys.contains(k) && isHexAscii(v)) {\n        v = convertHexToString(v);\n    }\n\n    // Convert string values to boolean.\n    if (possibleBooleanKeys.contains(k) && v instanceof String) {\n        v = convertStringToBoolean(v);\n    }\n\n    // Trim quotes.\n    if (v instanceof String) {\n        v = trimQuotes(\"'\", \"\\\"\", v);\n    }\n\n    // Convert arch.\n    if (k == \"arch\" && v == \"c000003e\") {\n        v = \"x86_64\";\n    }\n\n    return v;\n}\n\ndef audit = ctx.auditd.get(\"log\");\nIterator entries = audit.entrySet().iterator();\n\nwhile (entries.hasNext()) {\n    def e = entries.next();\n    def k = e.getKey();\n    def v = e.getValue();\n\n    if (v instanceof List) {\n        int j = 0;\n        for (int i = 0; i < v.length; i++) {\n            v[j] = processFieldValue(k, v[i], params.possibleHexKeys, params.possibleBooleanKeys);\n            if (v[j] != null) {\n                j++;\n            }\n        }\n        if (j < v.length) {\n            if (j == 0) {\n                entries.remove();\n                continue;\n            }\n            audit.put(k, v.subList(0, j));\n        }\n        continue;\n    }\n\n    v = processFieldValue(k, v, params.possibleHexKeys, params.possibleBooleanKeys);\n    if (v == null) {\n        entries.remove();\n    } else {\n        audit.put(k, v);\n    }\n}\n"#
                ),
                cached_params!(
                    "{\"possibleHexKeys\":[\"exe\",\"cmd\",\"data\",\"path\",\"comm\",\"file\",\"name\",\"watch\",\"cwd\",\"acct\",\"dir\",\"vm\",\"old-chardev\",\"new-chardev\",\"old-disk\",\"new-disk\",\"old-fs\",\"new-fs\",\"old-net\",\"new-net\",\"device\",\"cgroup\",\"apparmor\",\"operation\",\"denied_mask\",\"info\",\"profile\",\"requested_mask\",\"old-rng\",\"new-rng\",\"ocomm\",\"grp\",\"new_group\",\"invalid_context\",\"sw\",\"root_dir\",\"proctitle\"],\"possibleBooleanKeys\":[\"success\",\"key_enforce\"]}"
                ),
            )?;

            if event.has_value("auditd.log.sequence") {
                if let Some(val) = event.get("auditd.log.sequence") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.log.sequence".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.log.sequence", converted)?;
                }
            }

            if event.has_value("auditd.log.lport") {
                if let Some(val) = event.get("auditd.log.lport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.log.lport".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.log.lport", converted)?;
                }
            }

            if event.has_value("auditd.log.rport") {
                if let Some(val) = event.get("auditd.log.rport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.log.rport".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.log.rport", converted)?;
                }
            }

            if event.has_value("auditd.log.entries") {
                if let Some(val) = event.get("auditd.log.entries") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.log.entries".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.log.entries", converted)?;
                }
            }

            if event.has_value("auditd.log.dst_prefixlen") {
                if let Some(val) = event.get("auditd.log.dst_prefixlen") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.log.dst_prefixlen".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.log.dst_prefixlen", converted)?;
                }
            }

            if event.has_value("auditd.log.ksize") {
                if let Some(val) = event.get("auditd.log.ksize") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.log.ksize".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.log.ksize", converted)?;
                }
            }

            if event.has_value("auditd.log.size") {
                if let Some(val) = event.get("auditd.log.size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.log.size".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.log.size", converted)?;
                }
            }

            if event.has_value("auditd.log.src_prefixlen") {
                if let Some(val) = event.get("auditd.log.src_prefixlen") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "auditd.log.src_prefixlen".into(),
                            message,
                        }
                    })?;
                    event.set("auditd.log.src_prefixlen", converted)?;
                }
            }

            event.set("event.kind", json!("event"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: boolean hasFields(HashMap base, def list) {\n  if (list == null) return true;\n  for (int i=0; i<list.length; i++)\n    if (base[list[i]] == null) return false;\n  return true;\n} if (ctx?.auditd?.log?.record_type == null) {\n  return;\n} HashMap base = ctx.auditd.log; def acts = params.types.get(base.record_type); if (acts == null && base.syscall != null) {\n  acts = params.syscalls.get(base?.syscall);\n  if (acts == null) acts = params.syscalls.get('*');\n} if (acts == null) return; def act = null; for (int i=0; act == null && i<acts.length; i++) {\n  if (hasFields(base, acts[i][\"has_fields\"])) act = acts[i];\n}\nif (act?.event != null) {\n  def hm = new HashMap(act.event);\n  hm.forEach((k, v) -> ctx.event[k] = v);\n} if (act?.copy != null) {\n  List lst = new ArrayList();\n  for(int i=0; i<act.copy.length; i++) {\n    def value;\n    def srcList = act.copy[i][\"from\"];\n    for (int j=0; value == null && j<srcList.length; j++) {\n      value = base[srcList[j]];\n    }\n    if (value != null && value instanceof String && value != 'unset' && value != '?') {\n      String suffix = value ==~ /[0-9]+/? \".id\" : \".name\";\n      lst.add([\n        \"target\": act.copy[i][\"to\"] + suffix,\n        \"value\": value\n      ]);\n    }\n  }\n  if (lst.size() > 0) {\n    ctx.auditd.log[\"copy\"] = lst;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"boolean hasFields(HashMap base, def list) {\n  if (list == null) return true;\n  for (int i=0; i<list.length; i++)\n    if (base[list[i]] == null) return false;\n  return true;\n} if (ctx?.auditd?.log?.record_type == null) {\n  return;\n} HashMap base = ctx.auditd.log; def acts = params.types.get(base.record_type); if (acts == null && base.syscall != null) {\n  acts = params.syscalls.get(base?.syscall);\n  if (acts == null) acts = params.syscalls.get('*');\n} if (acts == null) return; def act = null; for (int i=0; act == null && i<acts.length; i++) {\n  if (hasFields(base, acts[i][\"has_fields\"])) act = acts[i];\n}\nif (act?.event != null) {\n  def hm = new HashMap(act.event);\n  hm.forEach((k, v) -> ctx.event[k] = v);\n} if (act?.copy != null) {\n  List lst = new ArrayList();\n  for(int i=0; i<act.copy.length; i++) {\n    def value;\n    def srcList = act.copy[i][\"from\"];\n    for (int j=0; value == null && j<srcList.length; j++) {\n      value = base[srcList[j]];\n    }\n    if (value != null && value instanceof String && value != 'unset' && value != '?') {\n      String suffix = value ==~ /[0-9]+/? \".id\" : \".name\";\n      lst.add([\n        \"target\": act.copy[i][\"to\"] + suffix,\n        \"value\": value\n      ]);\n    }\n  }\n  if (lst.size() > 0) {\n    ctx.auditd.log[\"copy\"] = lst;\n  }\n}"#
                    ),
                    cached_params!(
                        "{\"syscalls\":{\"*\":[{\"event\":{\"category\":[\"process\"],\"type\":[\"info\"]}}],\"accept\":[{\"event\":{\"action\":[\"accepted-connection-from\"],\"category\":[\"network\"],\"type\":[\"connection\",\"start\"]}}],\"accept4\":[{\"event\":{\"action\":[\"accepted-connection-from\"],\"category\":[\"network\"],\"type\":[\"connection\",\"start\"]}}],\"access\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"adjtimex\":[{\"event\":{\"action\":[\"changed-system-time\"],\"category\":[\"host\"],\"type\":[\"change\"]}}],\"bind\":[{\"event\":{\"action\":[\"bound-socket\"],\"category\":[\"network\"],\"type\":[\"start\"]}}],\"brk\":[{\"event\":{\"action\":[\"allocated-memory\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"chmod\":[{\"event\":{\"action\":[\"changed-file-permissions-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"chown\":[{\"event\":{\"action\":[\"changed-file-ownership-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"clock_settime\":[{\"event\":{\"action\":[\"changed-system-time\"],\"category\":[\"host\"],\"type\":[\"change\"]}}],\"connect\":[{\"event\":{\"action\":[\"connected-to\"],\"category\":[\"network\"],\"type\":[\"connection\",\"start\"]}}],\"creat\":[{\"event\":{\"action\":[\"opened-file\"],\"category\":[\"file\"],\"type\":[\"creation\"]}}],\"delete_module\":[{\"event\":{\"action\":[\"unloaded-kernel-module\"],\"category\":[\"driver\"],\"type\":[\"end\"]}}],\"execve\":[{\"event\":{\"action\":[\"executed\"],\"category\":[\"process\"],\"type\":[\"start\"]}}],\"execveat\":[{\"event\":{\"action\":[\"executed\"],\"category\":[\"process\"],\"type\":[\"start\"]}}],\"faccessat\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"fallocate\":[{\"event\":{\"action\":[\"opened-file\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"fchmod\":[{\"event\":{\"action\":[\"changed-file-permissions-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"fchmodat\":[{\"event\":{\"action\":[\"changed-file-permissions-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"fchown\":[{\"event\":{\"action\":[\"changed-file-ownership-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"fchownat\":[{\"event\":{\"action\":[\"changed-file-ownership-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"fgetxattr\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"finit_module\":[{\"event\":{\"action\":[\"loaded-kernel-module\"],\"category\":[\"driver\"],\"type\":[\"start\"]}}],\"fremovexattr\":[{\"event\":{\"action\":[\"changed-file-attributes-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"fsetxattr\":[{\"event\":{\"action\":[\"changed-file-attributes-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"fstat\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"fstatat\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"fstatfs\":[{\"event\":{\"action\":[\"checked-filesystem-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"ftruncate\":[{\"event\":{\"action\":[\"opened-file\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"futimens\":[{\"event\":{\"action\":[\"changed-timestamp-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"futimesat\":[{\"event\":{\"action\":[\"changed-timestamp-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"getxattr\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"init_module\":[{\"event\":{\"action\":[\"loaded-kernel-module\"],\"category\":[\"driver\"],\"type\":[\"start\"]}}],\"kill\":[{\"event\":{\"action\":[\"killed-pid\"],\"category\":[\"process\"],\"type\":[\"end\"]}}],\"lchown\":[{\"event\":{\"action\":[\"changed-file-ownership-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"lgetxattr\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"listen\":[{\"event\":{\"action\":[\"listen-for-connections\"],\"category\":[\"network\"],\"type\":[\"start\"]}}],\"lremovexattr\":[{\"event\":{\"action\":[\"changed-file-attributes-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"lsetxattr\":[{\"event\":{\"action\":[\"changed-file-attributes-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"lstat\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"mkdir\":[{\"event\":{\"action\":[\"created-directory\"],\"category\":[\"file\"],\"type\":[\"creation\"]}}],\"mkdirat\":[{\"event\":{\"action\":[\"created-directory\"],\"category\":[\"file\"],\"type\":[\"creation\"]}}],\"mknod\":[{\"event\":{\"action\":[\"make-device\"],\"category\":[\"file\"],\"type\":[\"creation\"]}}],\"mknodat\":[{\"event\":{\"action\":[\"make-device\"],\"category\":[\"file\"],\"type\":[\"creation\"]}}],\"mmap\":[{\"event\":{\"action\":[\"allocated-memory\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"mmap2\":[{\"event\":{\"action\":[\"allocated-memory\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"mount\":[{\"event\":{\"action\":[\"mounted\"],\"category\":[\"file\"],\"type\":[\"creation\"]}}],\"newfstatat\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"open\":[{\"event\":{\"action\":[\"opened-file\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"openat\":[{\"event\":{\"action\":[\"opened-file\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"read\":[{\"event\":{\"action\":[\"read-file\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"readlink\":[{\"event\":{\"action\":[\"opened-file\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"readlinkat\":[{\"event\":{\"action\":[\"opened-file\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"recv\":[{\"event\":{\"action\":[\"received-from\"],\"category\":[\"network\"],\"type\":[\"connection\",\"info\"]}}],\"recvfrom\":[{\"event\":{\"action\":[\"received-from\"],\"category\":[\"network\"],\"type\":[\"connection\",\"info\"]}}],\"recvmmsg\":[{\"event\":{\"action\":[\"received-from\"],\"category\":[\"network\"],\"type\":[\"connection\",\"info\"]}}],\"recvmsg\":[{\"event\":{\"action\":[\"received-from\"],\"category\":[\"network\"],\"type\":[\"connection\",\"info\"]}}],\"removexattr\":[{\"event\":{\"action\":[\"changed-file-attributes-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"rename\":[{\"event\":{\"action\":[\"renamed\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"renameat\":[{\"event\":{\"action\":[\"renamed\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"renameat2\":[{\"event\":{\"action\":[\"renamed\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"rmdir\":[{\"event\":{\"action\":[\"deleted\"],\"category\":[\"file\"],\"type\":[\"deletion\"]}}],\"sched_setattr\":[{\"event\":{\"action\":[\"adjusted-scheduling-policy-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"sched_setparam\":[{\"event\":{\"action\":[\"adjusted-scheduling-policy-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"sched_setscheduler\":[{\"event\":{\"action\":[\"adjusted-scheduling-policy-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"send\":[{\"event\":{\"action\":[\"sent-to\"],\"category\":[\"network\"],\"type\":[\"connection\",\"info\"]}}],\"sendmmsg\":[{\"event\":{\"action\":[\"sent-to\"],\"category\":[\"network\"],\"type\":[\"connection\",\"info\"]}}],\"sendmsg\":[{\"event\":{\"action\":[\"sent-to\"],\"category\":[\"network\"],\"type\":[\"connection\",\"info\"]}}],\"sendto\":[{\"event\":{\"action\":[\"sent-to\"],\"category\":[\"network\"],\"type\":[\"connection\",\"info\"]}}],\"setdomainname\":[{\"event\":{\"action\":[\"changed-system-name\"],\"category\":[\"host\"],\"type\":[\"change\"]}}],\"setegid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"seteuid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"setfsgid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"setfsuid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"setgid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"sethostname\":[{\"event\":{\"action\":[\"changed-system-name\"],\"category\":[\"host\"],\"type\":[\"change\"]}}],\"setregid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"setresgid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"setresuid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"setreuid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"settimeofday\":[{\"event\":{\"action\":[\"changed-system-time\"],\"category\":[\"host\"],\"type\":[\"change\"]}}],\"setuid\":[{\"event\":{\"action\":[\"changed-identity-of\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"setxattr\":[{\"event\":{\"action\":[\"changed-file-attributes-of\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"stat\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"stat64\":[{\"event\":{\"action\":[\"checked-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"statfs\":[{\"event\":{\"action\":[\"checked-filesystem-metadata-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"stime\":[{\"event\":{\"action\":[\"changed-system-time\"],\"category\":[\"host\"],\"type\":[\"change\"]}}],\"symlink\":[{\"event\":{\"action\":[\"symlinked\"],\"category\":[\"file\"],\"type\":[\"creation\"]}}],\"symlinkat\":[{\"event\":{\"action\":[\"symlinked\"],\"category\":[\"file\"],\"type\":[\"creation\"]}}],\"tgkill\":[{\"event\":{\"action\":[\"killed-pid\"],\"category\":[\"process\"],\"type\":[\"end\"]}}],\"tkill\":[{\"event\":{\"action\":[\"killed-pid\"],\"category\":[\"process\"],\"type\":[\"end\"]}}],\"truncate\":[{\"event\":{\"action\":[\"opened-file\"],\"category\":[\"file\"],\"type\":[\"change\"]}}],\"umount\":[{\"event\":{\"action\":[\"unmounted\"],\"category\":[\"file\"],\"type\":[\"deletion\"]}}],\"umount2\":[{\"event\":{\"action\":[\"unmounted\"],\"category\":[\"file\"],\"type\":[\"deletion\"]}}],\"unlink\":[{\"event\":{\"action\":[\"deleted\"],\"category\":[\"file\"],\"type\":[\"deletion\"]}}],\"unlinkat\":[{\"event\":{\"action\":[\"deleted\"],\"category\":[\"file\"],\"type\":[\"deletion\"]}}],\"utime\":[{\"event\":{\"action\":[\"changed-timestamp-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"utimensat\":[{\"event\":{\"action\":[\"changed-timestamp-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"utimes\":[{\"event\":{\"action\":[\"changed-timestamp-of\"],\"category\":[\"file\"],\"type\":[\"info\"]}}],\"write\":[{\"event\":{\"action\":[\"wrote-to-file\"],\"category\":[\"file\"],\"type\":[\"change\"]}}]},\"types\":{\"ACCT_LOCK\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"AUID\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"UID\"],\"to\":\"user.effective\"},{\"from\":[\"id\",\"acct\"],\"to\":\"user.target\"},{\"from\":[\"ID\"],\"to\":\"user.target\"}],\"event\":{\"action\":[\"locked-account\"],\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]}}],\"ACCT_UNLOCK\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"AUID\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"UID\"],\"to\":\"user.effective\"},{\"from\":[\"id\",\"acct\"],\"to\":\"user.target\"},{\"from\":[\"ID\"],\"to\":\"user.target\"}],\"event\":{\"action\":[\"unlocked-account\"],\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]}}],\"ADD_GROUP\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"AUID\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"UID\"],\"to\":\"user.effective\"},{\"from\":[\"id\",\"acct\"],\"to\":\"group\"},{\"from\":[\"ID\"],\"to\":\"group\"}],\"event\":{\"action\":[\"added-group-account-to\"],\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]}}],\"ADD_USER\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"AUID\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"UID\"],\"to\":\"user.effective\"},{\"from\":[\"id\",\"acct\"],\"to\":\"user.target\"},{\"from\":[\"ID\"],\"to\":\"user.target\"}],\"event\":{\"action\":[\"added-user-account\"],\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]}}],\"ANOM_ABEND\":[{\"event\":{\"action\":[\"crashed-program\"],\"category\":[\"process\"],\"type\":[\"end\"]}}],\"ANOM_EXEC\":[{\"event\":{\"action\":[\"attempted-execution-of-forbidden-program\"],\"category\":[\"process\"],\"type\":[\"start\"]}}],\"ANOM_LINK\":[{\"event\":{\"action\":[\"used-suspicious-link\"]}}],\"ANOM_LOGIN_FAILURES\":[{\"event\":{\"action\":[\"failed-log-in-too-many-times-to\"]}}],\"ANOM_LOGIN_LOCATION\":[{\"event\":{\"action\":[\"attempted-log-in-from-unusual-place-to\"]}}],\"ANOM_LOGIN_SESSIONS\":[{\"event\":{\"action\":[\"opened-too-many-sessions-to\"]}}],\"ANOM_LOGIN_TIME\":[{\"event\":{\"action\":[\"attempted-log-in-during-unusual-hour-to\"]}}],\"ANOM_PROMISCUOUS\":[{\"event\":{\"action\":[\"changed-promiscuous-mode-on-device\"]}}],\"ANOM_RBAC_INTEGRITY_FAIL\":[{\"event\":{\"action\":[\"tested-file-system-integrity-of\"]}}],\"AVC\":[{\"event\":{\"action\":[\"violated-selinux-policy\"]},\"has_fields\":[\"seresult\"]},{\"event\":{\"action\":[\"violated-apparmor-policy\"]},\"has_fields\":[\"apparmor\"]}],\"CHGRP_ID\":[{\"event\":{\"action\":[\"changed-group\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"CHUSER_ID\":[{\"event\":{\"action\":[\"changed-user-id\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"CONFIG_CHANGE\":[{\"event\":{\"action\":[\"changed-audit-configuration\"],\"category\":[\"process\",\"configuration\"],\"type\":[\"change\"]}}],\"CRED_ACQ\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"acquired-credentials\"],\"category\":[\"authentication\"],\"type\":[\"info\"]}}],\"CRED_DISP\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"disposed-credentials\"],\"category\":[\"authentication\"],\"type\":[\"info\"]}}],\"CRED_REFR\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"refreshed-credentials\"],\"category\":[\"authentication\"],\"type\":[\"info\"]}}],\"CRYPTO_KEY_USER\":[{\"event\":{\"action\":[\"negotiated-crypto-key\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"CRYPTO_LOGIN\":[{\"event\":{\"action\":[\"crypto-officer-logged-in\"]}}],\"CRYPTO_LOGOUT\":[{\"event\":{\"action\":[\"crypto-officer-logged-out\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"CRYPTO_SESSION\":[{\"event\":{\"action\":[\"started-crypto-session\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"DAC_CHECK\":[{\"event\":{\"action\":[\"access-result\"]}}],\"DAEMON_ABORT\":[{\"event\":{\"action\":[\"aborted-auditd-startup\"],\"category\":[\"process\"],\"type\":[\"end\"]}}],\"DAEMON_ACCEPT\":[{\"event\":{\"action\":[\"remote-audit-connected\"],\"category\":[\"network\"],\"type\":[\"connection\",\"start\"]}}],\"DAEMON_CLOSE\":[{\"event\":{\"action\":[\"remote-audit-disconnected\"],\"category\":[\"network\"],\"type\":[\"connection\",\"start\"]}}],\"DAEMON_CONFIG\":[{\"event\":{\"action\":[\"changed-auditd-configuration\"],\"category\":[\"process\",\"configuration\"],\"type\":[\"change\"]}}],\"DAEMON_END\":[{\"event\":{\"action\":[\"shutdown-audit\"],\"category\":[\"process\"],\"type\":[\"end\"]}}],\"DAEMON_ERR\":[{\"event\":{\"action\":[\"audit-error\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"DAEMON_RECONFIG\":[{\"event\":{\"action\":[\"reconfigured-auditd\"],\"category\":[\"process\",\"configuration\"],\"type\":[\"info\"]}}],\"DAEMON_RESUME\":[{\"event\":{\"action\":[\"resumed-audit-logging\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"DAEMON_ROTATE\":[{\"event\":{\"action\":[\"rotated-audit-logs\"],\"category\":[\"process\"],\"type\":[\"change\"]}}],\"DAEMON_START\":[{\"event\":{\"action\":[\"started-audit\"],\"category\":[\"process\"],\"type\":[\"start\"]}}],\"DEL_GROUP\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"AUID\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"UID\"],\"to\":\"user.effective\"},{\"from\":[\"id\",\"acct\"],\"to\":\"group\"},{\"from\":[\"ID\"],\"to\":\"group\"}],\"event\":{\"action\":[\"deleted-group-account-from\"],\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]}}],\"DEL_USER\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"AUID\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"UID\"],\"to\":\"user.effective\"},{\"from\":[\"id\",\"acct\"],\"to\":\"user.target\"},{\"from\":[\"ID\"],\"to\":\"user.target\"}],\"event\":{\"action\":[\"deleted-user-account\"],\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]}}],\"FEATURE_CHANGE\":[{\"event\":{\"action\":[\"changed-audit-feature\"],\"category\":[\"configuration\"],\"type\":[\"change\"]}}],\"FS_RELABEL\":[{\"event\":{\"action\":[\"relabeled-filesystem\"]}}],\"GRP_AUTH\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"authenticated-to-group\"],\"category\":[\"authentication\"],\"type\":[\"info\"]}}],\"GRP_CHAUTHTOK\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"group\"}],\"event\":{\"action\":[\"changed-group-password\"],\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]}}],\"GRP_MGMT\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"group\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"modified-group-account\"],\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]}}],\"KERNEL\":[{\"event\":{\"action\":[\"initialized-audit-subsystem\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"KERN_MODULE\":[{\"event\":{\"action\":[\"loaded-kernel-module\"],\"category\":[\"driver\"],\"type\":[\"start\"]}}],\"LABEL_LEVEL_CHANGE\":[{\"event\":{\"action\":[\"modified-level-of\"]}}],\"LABEL_OVERRIDE\":[{\"event\":{\"action\":[\"overrode-label-of\"]}}],\"LOGIN\":[{\"copy\":[{\"from\":[\"old_auid\",\"old-auid\"],\"to\":\"user\"},{\"from\":[\"new-auid\",\"new_auid\",\"auid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"changed-login-id-to\"],\"category\":[\"authentication\"],\"type\":[\"start\"]}}],\"MAC_CHECK\":[{\"event\":{\"action\":[\"mac-permission\"]}}],\"MAC_CONFIG_CHANGE\":[{\"event\":{\"action\":[\"changed-selinux-boolean\"],\"category\":[\"configuration\"],\"type\":[\"change\"]}}],\"MAC_POLICY_LOAD\":[{\"event\":{\"action\":[\"loaded-selinux-policy\"],\"category\":[\"configuration\"],\"type\":[\"access\"]}}],\"MAC_STATUS\":[{\"event\":{\"action\":[\"changed-selinux-enforcement\"],\"category\":[\"configuration\"],\"type\":[\"change\"]}}],\"NETFILTER_CFG\":[{\"event\":{\"action\":[\"loaded-firewall-rule-to\"],\"category\":[\"configuration\"],\"type\":[\"change\"]}}],\"ROLE_ASSIGN\":[{\"event\":{\"action\":[\"assigned-user-role-to\"],\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]}}],\"ROLE_MODIFY\":[{\"event\":{\"action\":[\"modified-role\"],\"category\":[\"iam\"],\"type\":[\"change\"]}}],\"ROLE_REMOVE\":[{\"event\":{\"action\":[\"removed-user-role-from\"],\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]}}],\"SECCOMP\":[{\"event\":{\"action\":[\"violated-seccomp-policy\"]}}],\"SELINUX_ERR\":[{\"event\":{\"action\":[\"caused-mac-policy-error\"]}}],\"SERVICE_START\":[{\"event\":{\"action\":[\"started-service\"],\"category\":[\"process\"],\"type\":[\"start\"]}}],\"SERVICE_STOP\":[{\"event\":{\"action\":[\"stopped-service\"],\"category\":[\"process\"],\"type\":[\"end\"]}}],\"SOFTWARE_UPDATE\":[{\"event\":{\"action\":[\"package-updated\"],\"category\":[\"package\"],\"type\":[\"info\"]}}],\"SYSTEM_BOOT\":[{\"event\":{\"action\":[\"booted-system\"],\"category\":[\"host\"],\"type\":[\"start\"]}}],\"SYSTEM_RUNLEVEL\":[{\"event\":{\"action\":[\"changed-to-runlevel\"],\"category\":[\"host\"],\"type\":[\"change\"]}}],\"SYSTEM_SHUTDOWN\":[{\"event\":{\"action\":[\"shutdown-system\"],\"category\":[\"host\"],\"type\":[\"end\"]}}],\"TEST\":[{\"event\":{\"action\":[\"sent-test\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"TRUSTED_APP\":[{\"event\":{\"action\":[\"unknown\"],\"category\":[\"process\"],\"type\":[\"info\"]}}],\"TTY\":[{\"event\":{\"action\":[\"typed\"]}}],\"USER\":[{\"event\":{\"action\":[\"sent-message\"]}}],\"USER_ACCT\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"was-authorized\"],\"category\":[\"authentication\"],\"type\":[\"info\"]}}],\"USER_AUTH\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"authenticated\"],\"category\":[\"authentication\"],\"type\":[\"info\"]}}],\"USER_AVC\":[{\"event\":{\"action\":[\"access-permission\"]}}],\"USER_CHAUTHTOK\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"AUID\"],\"to\":\"user\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"UID\"],\"to\":\"user.effective\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.target\"},{\"from\":[\"ID\"],\"to\":\"user.target\"}],\"event\":{\"action\":[\"changed-password\"],\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]}}],\"USER_CMD\":[{\"event\":{\"action\":[\"ran-command\"],\"category\":[\"process\"],\"type\":[\"start\"]}}],\"USER_END\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"ended-session\"],\"category\":[\"session\"],\"type\":[\"end\"]}}],\"USER_ERR\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"error\"],\"category\":[\"authentication\"],\"type\":[\"info\"]}}],\"USER_LOGIN\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"logged-in\"],\"category\":[\"authentication\"],\"type\":[\"start\"]}}],\"USER_LOGOUT\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"AUID\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"UID\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"logged-out\"],\"category\":[\"authentication\"],\"type\":[\"end\"]}}],\"USER_MAC_CONFIG_CHANGE\":[{\"event\":{\"action\":[\"changed-mac-configuration\"],\"category\":[\"configuration\"],\"type\":[\"change\"]}}],\"USER_MAC_POLICY_LOAD\":[{\"event\":{\"action\":[\"loaded-mac-policy\"],\"category\":[\"configuration\"],\"type\":[\"access\"]}}],\"USER_MGMT\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.target\"},{\"from\":[\"uid\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"modified-user-account\"],\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]}}],\"USER_ROLE_CHANGE\":[{\"event\":{\"action\":[\"changed-role-to\"]}}],\"USER_SELINUX_ERR\":[{\"event\":{\"action\":[\"access-error\"]}}],\"USER_START\":[{\"copy\":[{\"from\":[\"auid\"],\"to\":\"user\"},{\"from\":[\"AUID\"],\"to\":\"user\"},{\"from\":[\"acct\",\"id\",\"uid\"],\"to\":\"user.effective\"},{\"from\":[\"UID\"],\"to\":\"user.effective\"}],\"event\":{\"action\":[\"started-session\"],\"category\":[\"session\"],\"type\":[\"start\"]}}],\"USER_TTY\":[{\"event\":{\"action\":[\"typed\"]}}],\"USYS_CONFIG\":[{\"event\":{\"action\":[\"changed-configuration\"],\"category\":[\"configuration\"],\"type\":[\"change\"]}}],\"VIRT_CONTROL\":[{\"event\":{\"action\":[\"issued-vm-control\"],\"category\":[\"host\"],\"type\":[\"info\"]}}],\"VIRT_CREATE\":[{\"event\":{\"action\":[\"created-vm-image\"],\"category\":[\"host\"],\"type\":[\"info\"]}}],\"VIRT_DESTROY\":[{\"event\":{\"action\":[\"deleted-vm-image\"],\"category\":[\"host\"],\"type\":[\"info\"]}}],\"VIRT_INTEGRITY_CHECK\":[{\"event\":{\"action\":[\"checked-integrity-of\"],\"category\":[\"host\"],\"type\":[\"info\"]}}],\"VIRT_MACHINE_ID\":[{\"event\":{\"action\":[\"assigned-vm-id\"],\"category\":[\"host\"],\"type\":[\"info\"]}}],\"VIRT_MIGRATE_IN\":[{\"event\":{\"action\":[\"migrated-vm-from\"],\"category\":[\"host\"],\"type\":[\"info\"]}}],\"VIRT_MIGRATE_OUT\":[{\"event\":{\"action\":[\"migrated-vm-to\"],\"category\":[\"host\"],\"type\":[\"info\"]}}],\"VIRT_RESOURCE\":[{\"event\":{\"action\":[\"assigned-vm-resource\"],\"category\":[\"host\"],\"type\":[\"info\"]}}]}}"
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("auditd.log.copy") {
                foreach_array(event, "auditd.log.copy", |event| {
                    set_templated(
                        event,
                        "{{{_ingest._value.target}}}",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("SYSTEM_BOOT")
                    || event.get_str("auditd.log.record_type") == Some("SYSTEM_SHUTDOWN")
            };
            if _cond {
                event.set("event.category", json!("host"))?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("SYSTEM_BOOT")
                    || event.get_str("auditd.log.record_type") == Some("SYSTEM_SHUTDOWN")
            };
            if _cond {
                event.set("event.type", json!("info"))?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("SYSCALL")
                    && event.get_str("auditd.log.syscall") == Some("execve")
            };
            if _cond {
                event.set("event.category", json!("process"))?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("SYSCALL")
                    && event.get_str("auditd.log.syscall") == Some("execve")
            };
            if _cond {
                event.set("event.type", json!("info"))?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("VIRT_CONTROL")
                    || event.get_str("auditd.log.record_type") == Some("VIRT_MACHINE_ID")
            };
            if _cond {
                event.set("event.category", json!("host"))?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("VIRT_CONTROL")
                    && event.get_str("auditd.log.op") == Some("start")
            };
            if _cond {
                event.set("event.type", json!("start"))?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("VIRT_CONTROL")
                    && event.get_str("auditd.log.op") == Some("stop")
            };
            if _cond {
                event.set("event.type", json!("end"))?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("VIRT_CONTROL")
                    && event.get_str("auditd.log.op") == Some("create")
            };
            if _cond {
                event.set("event.type", json!("creation"))?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("VIRT_CONTROL")
                    && event.get_str("auditd.log.op") == Some("delete")
            };
            if _cond {
                event.set("event.type", json!("deletion"))?;
            }

            let _cond = { event.get_str("auditd.log.record_type") == Some("VIRT_MACHINE_ID") };
            if _cond {
                event.set("event.type", json!("creation"))?;
            }

            let _cond = { event.get_str("auditd.log.record_type") == Some("VIRT_MACHINE_ID") };
            if _cond {
                let v = json!(
                    event
                        .get("auditd.log.vm")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("container.name", v)?;
                }
            }

            let _cond = { event.get_str("auditd.log.record_type") == Some("VIRT_MACHINE_ID") };
            if _cond {
                let v = json!(
                    event
                        .get("auditd.log.virt")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("container.runtime", v)?;
                }
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("SYSCALL")
                    && (event.get_str("auditd.log.syscall") == Some("accept")
                        || event.get_str("auditd.log.syscall") == Some("43")
                        || event.get_str("auditd.log.syscall") == Some("recvfrom")
                        || event.get_str("auditd.log.syscall") == Some("45")
                        || event.get_str("auditd.log.syscall") == Some("recvmsg")
                        || event.get_str("auditd.log.syscall") == Some("47")
                        || event.get_str("auditd.log.syscall") == Some("accept4")
                        || event.get_str("auditd.log.syscall") == Some("288"))
            };
            if _cond {
                event.set("network.direction", json!("ingress"))?;
            }

            let _cond = {
                event.get_str("auditd.log.record_type") == Some("SYSCALL")
                    && (event.get_str("auditd.log.syscall") == Some("connect")
                        || event.get_str("auditd.log.syscall") == Some("42")
                        || event.get_str("auditd.log.syscall") == Some("sendto")
                        || event.get_str("auditd.log.syscall") == Some("44")
                        || event.get_str("auditd.log.syscall") == Some("sendmsg")
                        || event.get_str("auditd.log.syscall") == Some("46"))
            };
            if _cond {
                event.set("network.direction", json!("egress"))?;
            }

            let _cond = { event.has_value("auditd.log.arch") };
            if _cond {
                if let Some(v) = event.get("auditd.log.arch").cloned() {
                    event.set("host.architecture", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.acct", "user.name")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.user", "user.name")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.uid", "user.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.gid", "user.group.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.agid", "user.audit.group.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.auid", "user.audit.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.fsgid", "user.filesystem.group.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.fsuid", "user.filesystem.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.egid", "user.effective.group.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.euid", "user.effective.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.sgid", "user.saved.group.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.suid", "user.saved.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.ogid", "user.owner.group.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.ouid", "user.owner.id")?;
                Ok(())
            })();

            let _cond = {
                event.has_value("auditd.log.UID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.UID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.UID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.name") {
                        event.set("user.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.GID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.GID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.GID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.group.name") {
                        event.set("user.group.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.AUID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.AUID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.AUID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.audit.name") {
                        event.set("user.audit.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.AGID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.AGID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.AGID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.audit.group.name") {
                        event.set("user.audit.group.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.EUID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.EUID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.EUID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.effective.name") {
                        event.set("user.effective.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.EGID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.EGID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.EGID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.effective.group.name") {
                        event.set("user.effective.group.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.FSUID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.FSUID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.FSUID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.filesystem.name") {
                        event.set("user.filesystem.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.FSGID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.FSGID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.FSGID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.filesystem.group.name") {
                        event.set("user.filesystem.group.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.SUID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.SUID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.SUID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.saved.name") {
                        event.set("user.saved.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.SGID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.SGID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.SGID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.saved.group.name") {
                        event.set("user.saved.group.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.OUID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.OUID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.OUID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.owner.name") {
                        event.set("user.owner.name", v)?;
                    }
                }
            }

            let _cond = {
                event.has_value("auditd.log.OGID")
                    && !(["?", "unset"].contains(&event.get_str("auditd.log.OGID").unwrap_or("")))
            };
            if _cond {
                if let Some(v) = event
                    .get("auditd.log.OGID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.owner.group.name") {
                        event.set("user.owner.group.name", v)?;
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.comm", "process.name")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.exe", "process.executable")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.pid", "process.pid")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.ppid", "process.parent.pid")?;
                Ok(())
            })();

            if event.has_value("process.pid") {
                if let Some(val) = event.get("process.pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.pid".into(),
                            message,
                        }
                    })?;
                    event.set("process.pid", converted)?;
                }
            }

            if event.has_value("process.parent.pid") {
                if let Some(val) = event.get("process.parent.pid") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.parent.pid".into(),
                            message,
                        }
                    })?;
                    event.set("process.parent.pid", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.cmd", "process.args")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(s) = event.get_string("process.args") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("process.args", Value::Array(parts))?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.argc", "process.args_count")?;
                Ok(())
            })();

            let _cond = { event.has_value("process.args") };
            if _cond {
                // Painless script
                // Source: if (ctx.process.args instanceof List) {\n  ctx.process.args_count = ctx.process.args.length;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.process.args instanceof List) {\n  ctx.process.args_count = ctx.process.args.length;\n}"#
                    ),
                )?;
            }

            if event.has_value("process.args_count") {
                if let Some(val) = event.get("process.args_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.args_count".into(),
                            message,
                        }
                    })?;
                    event.set("process.args_count", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.exit", "process.exit_code")?;
                Ok(())
            })();

            if event.has_value("process.exit_code") {
                if let Some(val) = event.get("process.exit_code") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "process.exit_code".into(),
                            message,
                        }
                    })?;
                    event.set("process.exit_code", converted)?;
                }
            }

            if event.has_value("auditd.log.cwd") {
                event.rename("auditd.log.cwd", "process.working_directory")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.terminal", "user.terminal")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.msg", "message")?;
                Ok(())
            })();

            let _cond = {
                event.has_value("auditd.log.res")
                    && ["1", "success"].contains(&event.get_str("auditd.log.res").unwrap_or(""))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("success"))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("auditd.log.res")
                    && ["0", "failed"].contains(&event.get_str("auditd.log.res").unwrap_or(""))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("failure"))?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("auditd.log.res")
                    && !(["0", "1", "success", "failed"]
                        .contains(&event.get_str("auditd.log.res").unwrap_or("")))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("unknown"))?;
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.has_value("auditd.log.result")
                    && ["1", "success"].contains(&event.get_str("auditd.log.result").unwrap_or(""))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("success"))?;
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.has_value("auditd.log.result")
                    && ["0", "failed"].contains(&event.get_str("auditd.log.result").unwrap_or(""))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("failure"))?;
                    Ok(())
                })();
            }

            let _cond = {
                !event.has_value("event.outcome")
                    && event.has_value("auditd.log.result")
                    && !(["0", "1", "success", "failed"]
                        .contains(&event.get_str("auditd.log.result").unwrap_or("")))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("unknown"))?;
                    Ok(())
                })();
            }

            let _cond =
                { event.has_value("auditd.log.avc.action") && !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = { event.get_str("auditd.log.avc.action") == Some("granted") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("auditd.log.avc.action") == Some("denied") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("auditd.log.record_type") == Some("EXECVE") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: /* Want to capture all aNN fields, including aN_len and aN[x] */ Pattern argRegex = /^a([0-9]+)(.*)$/;\nList keys = ctx.auditd.log.keySet().stream()\n            /* From List of keys to list of matchers */\n            .map(x -> argRegex.matcher(x))\n            /* Drop elements that didn't match the regex */\n            .filter(x -> x.matches())\n            /* Must save to a list because it needs to remove keys in auditd.log,\n               which cannot be done while streaming from this source */\n            .collect(Collectors.toList());\n\nList args = keys.stream()\n            /* List<Matcher> to List<[Matcher, Value for given key]>\n               with side effect of removing the key */\n            .map(x -> [x, ctx.auditd.log.remove(x.group(0))])\n            /* Drop elements that end in _len, just wanted to remove them */\n            .filter(x -> x[0].group(2) != \"_len\")\n            /* List<Matcher, Value> to List<[Int, Value]>\n               where the Int is the argument index */\n            .map(x -> [Integer.parseInt(x[0].group(1)), x[1]])\n            /* Sort by numeric argument index */\n            .sorted((lhs, rhs) -> lhs[0].compareTo(rhs[0]))\n            /* Save as List<[Index, Value]> */\n            .collect(Collectors.toList());\n\nif (args.isEmpty()) return; if (ctx.process == null) ctx.process = new HashMap(); ctx.process.args = args.stream().map(x -> x[1]).collect(Collectors.toList()); def firstIndex = args[0][0]; if (firstIndex == 0) {\n  ctx.process.executable = ctx.process.args[0];\n} else {\n  ctx.process.args.add(0, \"[... \" + firstIndex + \" truncated arguments ...]\");\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"/* Want to capture all aNN fields, including aN_len and aN[x] */ Pattern argRegex = /^a([0-9]+)(.*)$/;\nList keys = ctx.auditd.log.keySet().stream()\n            /* From List of keys to list of matchers */\n            .map(x -> argRegex.matcher(x))\n            /* Drop elements that didn't match the regex */\n            .filter(x -> x.matches())\n            /* Must save to a list because it needs to remove keys in auditd.log,\n               which cannot be done while streaming from this source */\n            .collect(Collectors.toList());\n\nList args = keys.stream()\n            /* List<Matcher> to List<[Matcher, Value for given key]>\n               with side effect of removing the key */\n            .map(x -> [x, ctx.auditd.log.remove(x.group(0))])\n            /* Drop elements that end in _len, just wanted to remove them */\n            .filter(x -> x[0].group(2) != \"_len\")\n            /* List<Matcher, Value> to List<[Int, Value]>\n               where the Int is the argument index */\n            .map(x -> [Integer.parseInt(x[0].group(1)), x[1]])\n            /* Sort by numeric argument index */\n            .sorted((lhs, rhs) -> lhs[0].compareTo(rhs[0]))\n            /* Save as List<[Index, Value]> */\n            .collect(Collectors.toList());\n\nif (args.isEmpty()) return; if (ctx.process == null) ctx.process = new HashMap(); ctx.process.args = args.stream().map(x -> x[1]).collect(Collectors.toList()); def firstIndex = args[0][0]; if (firstIndex == 0) {\n  ctx.process.executable = ctx.process.args[0];\n} else {\n  ctx.process.args.add(0, \"[... \" + firstIndex + \" truncated arguments ...]\");\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_execve_process_args",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "failed extracting process arguments: {}",
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

            if let Some(v) = event.get("auditd.log.record_type").cloned() {
                if !event.has("event.action") {
                    event.set("event.action", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.src", "source.address")?;
                Ok(())
            })();

            let _cond = { !event.has_value("source.address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("auditd.log.addr", "source.address")?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("auditd.log.dst", "destination.address")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^%{IP:source.ip}$
                    if !cached_grok!("^%{IP:source.ip}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })();

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

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
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

            let _cond = {
                event.has_value("user.audit.name") && event.get_str("user.audit.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.audit.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("user.effective.name")
                    && event.get_str("user.effective.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.effective.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("user.filesystem.name")
                    && event.get_str("user.filesystem.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.filesystem.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("user.saved.name") && event.get_str("user.saved.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.saved.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("user.owner.name") && event.get_str("user.owner.name") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.owner.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("user.target.name") && event.get_str("user.target.name") != Some("")
            };
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("auditd.log.kv");
                event.remove("auditd.log.sub_kv");
                event.remove("auditd.log.sub_kv_enriched");
                event.remove("auditd.log.epoch");
                event.remove("auditd.log.copy");
                event.remove("auditd.log.arch");
                event.remove("auditd.log.res");
                event.remove("auditd.log.result");
                Ok(())
            })();

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor \"{}\" with tag \"{}\" failed with message \"{}\"",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
