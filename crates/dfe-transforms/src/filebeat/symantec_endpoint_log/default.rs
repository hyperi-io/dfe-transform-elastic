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
            if let Some(v) = event.get("message").cloned() {
                if !event.has("event.original") {
                    event.set("event.original", v)?;
                }
            }

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("<"))
            };
            if _cond {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^<%{NONNEGINT:log.syslog.priority:long}>(?:%{SYSLOGTIMESTAMP:timestamp}|%{TIMESTAMP_ISO8601:timestamp})(?: %{SYSLOGFACILITY})?(?: %{SYSLOGHOST:log.syslog.hostname})?(?: (?:%{PROG:log.syslog.appname}(?:\\[%{POSINT:log.syslog.procid}\\])?):)? %{GREEDYDATA:message}
                    // Grok pattern: ^(?:(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)%{NONNEGINT:log.syslog.version} +(?:-|%{TIMESTAMP_ISO8601:timestamp}) +(?:-|%{IPORHOST:log.syslog.hostname}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.appname}) +(?:-|%{POSINT:log.syslog.procid}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.message_id}) +(?:-|%{SYSLOG5424SD:log.syslog.structured_data})?) +%{GREEDYDATA:message})
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^<%{NONNEGINT:log.syslog.priority:long}>(?:%{SYSLOGTIMESTAMP:timestamp}|%{TIMESTAMP_ISO8601:timestamp})(?: %{SYSLOGFACILITY})?(?: %{SYSLOGHOST:log.syslog.hostname})?(?: (?:%{PROG:log.syslog.appname}(?:\\[%{POSINT:log.syslog.procid}\\])?):)? %{GREEDYDATA:message}"
                            ),
                            cached_grok!(
                                "^(?:(?:(?:<%{NONNEGINT:log.syslog.priority:long}>)%{NONNEGINT:log.syslog.version} +(?:-|%{TIMESTAMP_ISO8601:timestamp}) +(?:-|%{IPORHOST:log.syslog.hostname}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.appname}) +(?:-|%{POSINT:log.syslog.procid}) +(?:-|%{SYSLOG5424PRINTASCII:log.syslog.message_id}) +(?:-|%{SYSLOG5424SD:log.syslog.structured_data})?) +%{GREEDYDATA:message})"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event
                    .get_str("event.original")
                    .is_some_and(|s| s.starts_with("20"))
                    || event
                        .get_str("event.original")
                        .is_some_and(|s| s.starts_with("19"))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: ^%{TIMESTAMP_ISO8601:timestamp},(?P<log_level>(?:(?:%{LOGLEVEL}|[Cc]ritical|CRITICAL|[Mm]ajor|MAJOR|[Mm]inor|MINOR|[Ii]nfo|INFO|[Ww]arning|WARNING|[Ee]rror|ERROR|[Ff]atal|FATAL))),%{GREEDYDATA:message}
                        if !cached_grok_mapped!("^%{TIMESTAMP_ISO8601:timestamp},(?P<log_level>(?:(?:%{LOGLEVEL}|[Cc]ritical|CRITICAL|[Mm]ajor|MAJOR|[Mm]inor|MINOR|[Ii]nfo|INFO|[Ww]arning|WARNING|[Ee]rror|ERROR|[Ff]atal|FATAL))),%{GREEDYDATA:message}", [("log_level", "log.level")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "MMM dd HH:mm:ss",
                            "MMM  d HH:mm:ss",
                            "MMM d HH:mm:ss",
                            "ISO8601",
                            "YYYY-dd-MM HH:mm:ss",
                        ],
                        event.get_str("_conf.tz_offset"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            event.remove("timestamp");

            if let Some(csv_str) = event.get_string("message") {
                let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b',')
                    .quote(b'\"')
                    .has_headers(false)
                    .from_reader(csv_str.as_bytes());
                if let Some(Ok(record)) = rdr.records().next() {
                    if let Some(val) = record.get(0) {
                        if !val.is_empty() {
                            event.set("_csv_array.00", val)?;
                        } else {
                            event.set("_csv_array.00", "")?;
                        }
                    }
                    if let Some(val) = record.get(1) {
                        if !val.is_empty() {
                            event.set("_csv_array.01", val)?;
                        } else {
                            event.set("_csv_array.01", "")?;
                        }
                    }
                    if let Some(val) = record.get(2) {
                        if !val.is_empty() {
                            event.set("_csv_array.02", val)?;
                        } else {
                            event.set("_csv_array.02", "")?;
                        }
                    }
                    if let Some(val) = record.get(3) {
                        if !val.is_empty() {
                            event.set("_csv_array.03", val)?;
                        } else {
                            event.set("_csv_array.03", "")?;
                        }
                    }
                    if let Some(val) = record.get(4) {
                        if !val.is_empty() {
                            event.set("_csv_array.04", val)?;
                        } else {
                            event.set("_csv_array.04", "")?;
                        }
                    }
                    if let Some(val) = record.get(5) {
                        if !val.is_empty() {
                            event.set("_csv_array.05", val)?;
                        } else {
                            event.set("_csv_array.05", "")?;
                        }
                    }
                    if let Some(val) = record.get(6) {
                        if !val.is_empty() {
                            event.set("_csv_array.06", val)?;
                        } else {
                            event.set("_csv_array.06", "")?;
                        }
                    }
                    if let Some(val) = record.get(7) {
                        if !val.is_empty() {
                            event.set("_csv_array.07", val)?;
                        } else {
                            event.set("_csv_array.07", "")?;
                        }
                    }
                    if let Some(val) = record.get(8) {
                        if !val.is_empty() {
                            event.set("_csv_array.08", val)?;
                        } else {
                            event.set("_csv_array.08", "")?;
                        }
                    }
                    if let Some(val) = record.get(9) {
                        if !val.is_empty() {
                            event.set("_csv_array.09", val)?;
                        } else {
                            event.set("_csv_array.09", "")?;
                        }
                    }
                    if let Some(val) = record.get(10) {
                        if !val.is_empty() {
                            event.set("_csv_array.10", val)?;
                        } else {
                            event.set("_csv_array.10", "")?;
                        }
                    }
                    if let Some(val) = record.get(11) {
                        if !val.is_empty() {
                            event.set("_csv_array.11", val)?;
                        } else {
                            event.set("_csv_array.11", "")?;
                        }
                    }
                    if let Some(val) = record.get(12) {
                        if !val.is_empty() {
                            event.set("_csv_array.12", val)?;
                        } else {
                            event.set("_csv_array.12", "")?;
                        }
                    }
                    if let Some(val) = record.get(13) {
                        if !val.is_empty() {
                            event.set("_csv_array.13", val)?;
                        } else {
                            event.set("_csv_array.13", "")?;
                        }
                    }
                    if let Some(val) = record.get(14) {
                        if !val.is_empty() {
                            event.set("_csv_array.14", val)?;
                        } else {
                            event.set("_csv_array.14", "")?;
                        }
                    }
                    if let Some(val) = record.get(15) {
                        if !val.is_empty() {
                            event.set("_csv_array.15", val)?;
                        } else {
                            event.set("_csv_array.15", "")?;
                        }
                    }
                    if let Some(val) = record.get(16) {
                        if !val.is_empty() {
                            event.set("_csv_array.16", val)?;
                        } else {
                            event.set("_csv_array.16", "")?;
                        }
                    }
                    if let Some(val) = record.get(17) {
                        if !val.is_empty() {
                            event.set("_csv_array.17", val)?;
                        } else {
                            event.set("_csv_array.17", "")?;
                        }
                    }
                    if let Some(val) = record.get(18) {
                        if !val.is_empty() {
                            event.set("_csv_array.18", val)?;
                        } else {
                            event.set("_csv_array.18", "")?;
                        }
                    }
                    if let Some(val) = record.get(19) {
                        if !val.is_empty() {
                            event.set("_csv_array.19", val)?;
                        } else {
                            event.set("_csv_array.19", "")?;
                        }
                    }
                    if let Some(val) = record.get(20) {
                        if !val.is_empty() {
                            event.set("_csv_array.20", val)?;
                        } else {
                            event.set("_csv_array.20", "")?;
                        }
                    }
                    if let Some(val) = record.get(21) {
                        if !val.is_empty() {
                            event.set("_csv_array.21", val)?;
                        } else {
                            event.set("_csv_array.21", "")?;
                        }
                    }
                    if let Some(val) = record.get(22) {
                        if !val.is_empty() {
                            event.set("_csv_array.22", val)?;
                        } else {
                            event.set("_csv_array.22", "")?;
                        }
                    }
                    if let Some(val) = record.get(23) {
                        if !val.is_empty() {
                            event.set("_csv_array.23", val)?;
                        } else {
                            event.set("_csv_array.23", "")?;
                        }
                    }
                    if let Some(val) = record.get(24) {
                        if !val.is_empty() {
                            event.set("_csv_array.24", val)?;
                        } else {
                            event.set("_csv_array.24", "")?;
                        }
                    }
                    if let Some(val) = record.get(25) {
                        if !val.is_empty() {
                            event.set("_csv_array.25", val)?;
                        } else {
                            event.set("_csv_array.25", "")?;
                        }
                    }
                    if let Some(val) = record.get(26) {
                        if !val.is_empty() {
                            event.set("_csv_array.26", val)?;
                        } else {
                            event.set("_csv_array.26", "")?;
                        }
                    }
                    if let Some(val) = record.get(27) {
                        if !val.is_empty() {
                            event.set("_csv_array.27", val)?;
                        } else {
                            event.set("_csv_array.27", "")?;
                        }
                    }
                    if let Some(val) = record.get(28) {
                        if !val.is_empty() {
                            event.set("_csv_array.28", val)?;
                        } else {
                            event.set("_csv_array.28", "")?;
                        }
                    }
                    if let Some(val) = record.get(29) {
                        if !val.is_empty() {
                            event.set("_csv_array.29", val)?;
                        } else {
                            event.set("_csv_array.29", "")?;
                        }
                    }
                    if let Some(val) = record.get(30) {
                        if !val.is_empty() {
                            event.set("_csv_array.30", val)?;
                        } else {
                            event.set("_csv_array.30", "")?;
                        }
                    }
                    if let Some(val) = record.get(31) {
                        if !val.is_empty() {
                            event.set("_csv_array.31", val)?;
                        } else {
                            event.set("_csv_array.31", "")?;
                        }
                    }
                    if let Some(val) = record.get(32) {
                        if !val.is_empty() {
                            event.set("_csv_array.32", val)?;
                        } else {
                            event.set("_csv_array.32", "")?;
                        }
                    }
                    if let Some(val) = record.get(33) {
                        if !val.is_empty() {
                            event.set("_csv_array.33", val)?;
                        } else {
                            event.set("_csv_array.33", "")?;
                        }
                    }
                    if let Some(val) = record.get(34) {
                        if !val.is_empty() {
                            event.set("_csv_array.34", val)?;
                        } else {
                            event.set("_csv_array.34", "")?;
                        }
                    }
                    if let Some(val) = record.get(35) {
                        if !val.is_empty() {
                            event.set("_csv_array.35", val)?;
                        } else {
                            event.set("_csv_array.35", "")?;
                        }
                    }
                    if let Some(val) = record.get(36) {
                        if !val.is_empty() {
                            event.set("_csv_array.36", val)?;
                        } else {
                            event.set("_csv_array.36", "")?;
                        }
                    }
                    if let Some(val) = record.get(37) {
                        if !val.is_empty() {
                            event.set("_csv_array.37", val)?;
                        } else {
                            event.set("_csv_array.37", "")?;
                        }
                    }
                    if let Some(val) = record.get(38) {
                        if !val.is_empty() {
                            event.set("_csv_array.38", val)?;
                        } else {
                            event.set("_csv_array.38", "")?;
                        }
                    }
                    if let Some(val) = record.get(39) {
                        if !val.is_empty() {
                            event.set("_csv_array.39", val)?;
                        } else {
                            event.set("_csv_array.39", "")?;
                        }
                    }
                    if let Some(val) = record.get(40) {
                        if !val.is_empty() {
                            event.set("_csv_array.40", val)?;
                        } else {
                            event.set("_csv_array.40", "")?;
                        }
                    }
                    if let Some(val) = record.get(41) {
                        if !val.is_empty() {
                            event.set("_csv_array.41", val)?;
                        } else {
                            event.set("_csv_array.41", "")?;
                        }
                    }
                    if let Some(val) = record.get(42) {
                        if !val.is_empty() {
                            event.set("_csv_array.42", val)?;
                        } else {
                            event.set("_csv_array.42", "")?;
                        }
                    }
                    if let Some(val) = record.get(43) {
                        if !val.is_empty() {
                            event.set("_csv_array.43", val)?;
                        } else {
                            event.set("_csv_array.43", "")?;
                        }
                    }
                    if let Some(val) = record.get(44) {
                        if !val.is_empty() {
                            event.set("_csv_array.44", val)?;
                        } else {
                            event.set("_csv_array.44", "")?;
                        }
                    }
                    if let Some(val) = record.get(45) {
                        if !val.is_empty() {
                            event.set("_csv_array.45", val)?;
                        } else {
                            event.set("_csv_array.45", "")?;
                        }
                    }
                    if let Some(val) = record.get(46) {
                        if !val.is_empty() {
                            event.set("_csv_array.46", val)?;
                        } else {
                            event.set("_csv_array.46", "")?;
                        }
                    }
                    if let Some(val) = record.get(47) {
                        if !val.is_empty() {
                            event.set("_csv_array.47", val)?;
                        } else {
                            event.set("_csv_array.47", "")?;
                        }
                    }
                    if let Some(val) = record.get(48) {
                        if !val.is_empty() {
                            event.set("_csv_array.48", val)?;
                        } else {
                            event.set("_csv_array.48", "")?;
                        }
                    }
                    if let Some(val) = record.get(49) {
                        if !val.is_empty() {
                            event.set("_csv_array.49", val)?;
                        } else {
                            event.set("_csv_array.49", "")?;
                        }
                    }
                    if let Some(val) = record.get(50) {
                        if !val.is_empty() {
                            event.set("_csv_array.50", val)?;
                        } else {
                            event.set("_csv_array.50", "")?;
                        }
                    }
                }
            }

            // Painless script
            // Source: def columnArray = [];\ndef sortedMap = new TreeMap();\nsortedMap.putAll(ctx._csv_array);\nsortedMap.forEach((key, value) -> {\n  def v = value;\n  if (v.startsWith(\"'\") && v.endsWith(\"'\"))\n  {\n    v = v.substring(1, v.length() - 1);\n  }\n  columnArray.add(v);\n});\nctx['_csv_array'] = columnArray;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def columnArray = [];\ndef sortedMap = new TreeMap();\nsortedMap.putAll(ctx._csv_array);\nsortedMap.forEach((key, value) -> {\n  def v = value;\n  if (v.startsWith(\"'\") && v.endsWith(\"'\"))\n  {\n    v = v.substring(1, v.length() - 1);\n  }\n  columnArray.add(v);\n});\nctx['_csv_array'] = columnArray;\n"#
                ),
            )?;

            // Painless script
            // Source: def aliases = Collections.unmodifiableMap([\n  'computer': 'computer_name',\n  'domain': 'domain_name',\n  'end_time': 'end',\n  'group_name': 'group',\n  'local': 'local_host_ip',\n  'local_host': 'local_host_ip',\n  'server_name': 'server',\n  'user': 'user_name'\n]);\n\ndef keyPattern = /^([a-zA-Z][a-zA-Z0-9 \\(\\)-]{0,28}):(?:\\s(.+)|\\s)?/;\ndef keyValue = [:];\ndef fingerprint = [];\nctx._csv_array.forEach(v -> {\n    def m = keyPattern.matcher(v);\n    def key = 'NONE';\n    if (m.matches()) {\n      key = m.group(1).toLowerCase().replace(' ', '_');\n      key = /[\\(\\)]+/.matcher(key).replaceAll('');\n\n      def tmp = aliases[key];\n      if (tmp != null) {\n        key = tmp;\n      }\n\n\n      def value = m.group(2);\n      if (value != null && !value.trim().isEmpty()) {\n        keyValue[key] = value.trim();\n      }\n    }\n\n    fingerprint.add(key);\n    return true;\n});\nif (!keyValue.isEmpty()) {\n  ctx['_csv_map'] = keyValue;\n}\nctx['_fingerprint'] = String.join(\"|\", fingerprint);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def aliases = Collections.unmodifiableMap([\n  'computer': 'computer_name',\n  'domain': 'domain_name',\n  'end_time': 'end',\n  'group_name': 'group',\n  'local': 'local_host_ip',\n  'local_host': 'local_host_ip',\n  'server_name': 'server',\n  'user': 'user_name'\n]);\n\ndef keyPattern = /^([a-zA-Z][a-zA-Z0-9 \\(\\)-]{0,28}):(?:\\s(.+)|\\s)?/;\ndef keyValue = [:];\ndef fingerprint = [];\nctx._csv_array.forEach(v -> {\n    def m = keyPattern.matcher(v);\n    def key = 'NONE';\n    if (m.matches()) {\n      key = m.group(1).toLowerCase().replace(' ', '_');\n      key = /[\\(\\)]+/.matcher(key).replaceAll('');\n\n      def tmp = aliases[key];\n      if (tmp != null) {\n        key = tmp;\n      }\n\n\n      def value = m.group(2);\n      if (value != null && !value.trim().isEmpty()) {\n        keyValue[key] = value.trim();\n      }\n    }\n\n    fingerprint.add(key);\n    return true;\n});\nif (!keyValue.isEmpty()) {\n  ctx['_csv_map'] = keyValue;\n}\nctx['_fingerprint'] = String.join(\"|\", fingerprint);\n"#
                ),
            )?;

            event.remove("message");

            // Painless script
            // Source: // Assume first column is always the host.hostname.\ndef hostname = ctx._csv_array.get(0);\nif (/[\\.a-zA-Z0-9_-]+/.matcher(hostname).matches()) {\n  if (ctx?.host == null) {\n    ctx['host'] = [:];\n  }\n  ctx['host']['hostname'] = hostname;\n}\n\ndef provider = null;\nfor (def p: params.providers) {\n  if (p.fingerprint == ctx._fingerprint || (p.fingerprint instanceof Collection && p.fingerprint.contains(ctx._fingerprint))) {\n    provider = p;\n    break;\n  }\n}\nif (provider == null) { return; }\n\nctx['event']['provider'] = provider.name;\nif (provider?.event_category != null) {\n  ctx['event']['category'] = new ArrayList(provider.event_category);\n}\nif (provider?.event_type!= null) {\n  ctx['event']['type'] = new ArrayList(provider.event_type);\n}\nfor (def c : provider.columns) {\n  def v = ctx._csv_array.get(c.index).trim();\n  if (!v.isEmpty()) {\n    ctx._csv_map[c.name] = v;\n  }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"// Assume first column is always the host.hostname.\ndef hostname = ctx._csv_array.get(0);\nif (/[\\.a-zA-Z0-9_-]+/.matcher(hostname).matches()) {\n  if (ctx?.host == null) {\n    ctx['host'] = [:];\n  }\n  ctx['host']['hostname'] = hostname;\n}\n\ndef provider = null;\nfor (def p: params.providers) {\n  if (p.fingerprint == ctx._fingerprint || (p.fingerprint instanceof Collection && p.fingerprint.contains(ctx._fingerprint))) {\n    provider = p;\n    break;\n  }\n}\nif (provider == null) { return; }\n\nctx['event']['provider'] = provider.name;\nif (provider?.event_category != null) {\n  ctx['event']['category'] = new ArrayList(provider.event_category);\n}\nif (provider?.event_type!= null) {\n  ctx['event']['type'] = new ArrayList(provider.event_type);\n}\nfor (def c : provider.columns) {\n  def v = ctx._csv_array.get(c.index).trim();\n  if (!v.isEmpty()) {\n    ctx._csv_map[c.name] = v;\n  }\n}\n"#
                ),
                cached_params!(
                    "{\"providers\":[{\"name\":\"Agent Behavior Log\",\"fingerprint\":\"NONE|NONE|NONE|NONE|NONE|begin|end|rule|NONE|NONE|NONE|NONE|NONE|user_name|domain_name|action_type|file_size_bytes|device_id\",\"event_category\":[\"intrusion_detection\",\"process\"],\"columns\":[{\"index\":1,\"name\":\"local_host_ip\"},{\"index\":2,\"name\":\"action\"},{\"index\":3,\"name\":\"event_description\"},{\"index\":4,\"name\":\"api_name\"},{\"index\":8,\"name\":\"caller_process_id\"},{\"index\":9,\"name\":\"caller_process_name\"},{\"index\":10,\"name\":\"caller_return_address\"},{\"index\":11,\"name\":\"caller_return_module_name\"},{\"index\":12,\"name\":\"parameters\"}]},{\"name\":\"Agent Security Log\",\"fingerprint\":[\"NONE|event_description|local_host_ip|local_host_mac|remote_host_name|remote_host_ip|remote_host_mac|NONE|NONE|intrusion_id|begin|end|occurrences|application|location|user_name|domain_name|local_port|remote_port|cids_signature_id|cids_signature_string|cids_signature_subid|intrusion_url|intrusion_payload_url|sha-256|md-5\",\"NONE|event_description|local_host_ip|local_host_mac|remote_host_name|remote_host_ip|remote_host_mac|NONE|NONE|NONE|begin|end|occurrences|application|location|user_name|domain_name|local_port|remote_port|cids_signature_id|cids_signature_string|cids_signature_subid|intrusion_url|intrusion_payload_url|sha-256|md-5\"],\"event_category\":[\"intrusion_detection\",\"network\",\"process\"],\"event_type\":[\"connection\"],\"columns\":[{\"index\":7,\"name\":\"traffic_direction\"},{\"index\":8,\"name\":\"network_protocol\"}]},{\"name\":\"Agent Traffic Log\",\"fingerprint\":\"NONE|local_host_ip|local_port|local_host_mac|remote_host_ip|remote_host_name|remote_port|remote_host_mac|NONE|NONE|begin|end|occurrences|application|rule|location|user_name|domain_name|action|sha-256|md-5\",\"event_category\":[\"intrusion_detection\",\"network\",\"process\"],\"event_type\":[\"connection\"],\"columns\":[{\"index\":9,\"name\":\"traffic_direction\"},{\"index\":8,\"name\":\"network_protocol\"}]},{\"name\":\"Agent Activity Log\",\"fingerprint\":\"site|server|domain_name|NONE|NONE|NONE|NONE\",\"columns\":[{\"index\":3,\"name\":\"event_description\"},{\"index\":4,\"name\":\"local_host_name\"},{\"index\":5,\"name\":\"user_name\"},{\"index\":6,\"name\":\"domain_name\"}]},{\"name\":\"Agent Packet Log\",\"fingerprint\":[\"NONE|local_host_ip|local_port|remote_host_ip|remote_host_name|remote_port|NONE|application|action\"],\"event_category\":[\"intrusion_detection\",\"network\",\"process\"],\"event_type\":[\"connection\"],\"columns\":[{\"index\":6,\"name\":\"traffic_direction\"}]},{\"name\":\"Agent System Log\",\"fingerprint\":[\"NONE|category|NONE|NONE|event_time\"],\"columns\":[{\"index\":2,\"name\":\"event_source\"},{\"index\":3,\"name\":\"event_description\"}]},{\"name\":\"Administrative Log\",\"fingerprint\":\"site|server|domain_name|admin|NONE\",\"columns\":[{\"index\":4,\"name\":\"event_description\"}]},{\"name\":\"System Log\",\"fingerprint\":\"site|server|NONE\",\"columns\":[{\"index\":2,\"name\":\"event_description\"}]},{\"name\":\"Agent Proactive Detection Log\",\"fingerprint\":\"NONE|computer_name|detection_type|first_seen|application_name|application_type|application_version|hash_type|application_hash|company_name|file_size_bytes|sensitivity|detection_score|coh_engine_version|NONE|permitted_application_reason|disposition|download_site|web_domain|downloaded_by|prevalence|confidence|url_tracking_status|risk_level|detection_source|source|risk_name|occurrences|NONE|NONE|actual_action|requested_action|secondary_action|event_time|inserted|end|domain_name|group|server|user_name|source_computer|source_ip\",\"columns\":[{\"index\":0,\"name\":\"event_description\"},{\"index\":16,\"name\":\"submission_recommended\"},{\"index\":28,\"name\":\"file_path\"},{\"index\":29,\"name\":\"description\"}]},{\"name\":\"Agent Proactive Detection Log\",\"fingerprint\":\"NONE|computer_name|ip_address|detection_type|first_seen|application_name|application_type|application_version|hash_type|application_hash|company_name|file_size_bytes|sensitivity|detection_score|coh_engine_version|NONE|permitted_application_reason|disposition|download_site|web_domain|downloaded_by|prevalence|confidence|url_tracking_status|risk_level|risk_type|source|risk_name|occurrences|NONE|NONE|actual_action|requested_action|secondary_action|event_time|inserted|end|domain_name|group|server|user_name|source_computer|source_ip|intensive_protection_level|certificate_issuer|certificate_signer|certificate_thumbprint|signing_timestamp|certificate_serial_number\",\"columns\":[{\"index\":0,\"name\":\"event_description\"},{\"index\":17,\"name\":\"submission_recommended\"},{\"index\":29,\"name\":\"file_path\"},{\"index\":30,\"name\":\"description\"}]},{\"name\":\"Policy Log\",\"fingerprint\":\"site|server|domain_name|admin|event_description|NONE\",\"columns\":[{\"index\":5,\"name\":\"policy_name\"}]},{\"name\":\"Agent Scan Log\",\"fingerprint\":\"scan_id|begin|end|NONE|duration_seconds|user1|user2|NONE|scan_complete|command|threats|infected|total_files|omitted|computer_name|ip_address|domain_name|group|server\",\"columns\":[{\"index\":3,\"name\":\"action\"},{\"index\":7,\"name\":\"event_description\"}]},{\"name\":\"Agent Risk Log\",\"fingerprint\":\"NONE|ip_address|computer_name|source|risk_name|occurrences|NONE|NONE|actual_action|requested_action|secondary_action|event_time|inserted|end|last_update_time|domain_name|group|server|user_name|source_computer|source_ip|disposition|download_site|web_domain|downloaded_by|prevalence|confidence|url_tracking_status|first_seen|sensitivity|permitted_application_reason|application_hash|hash_type|company_name|application_name|application_version|application_type|file_size_bytes|category_set|category_type|location|intensive_protection_level|certificate_issuer|certificate_signer|certificate_thumbprint|signing_timestamp|certificate_serial_number\",\"columns\":[{\"index\":0,\"name\":\"event_description\"},{\"index\":6,\"name\":\"file_path\"}]}]}"
                ),
            )?;

            if event.has_value("_csv_map") {
                event.rename("_csv_map", "symantec_endpoint.log")?;
            }

            if event.has_value("symantec_endpoint.log.action") {
                map_strings(
                    event,
                    "symantec_endpoint.log.action",
                    "symantec_endpoint.log.action",
                    str::to_lowercase,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.action").cloned() {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("event.action") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("symantec_endpoint.log.actual_action").cloned() {
                        event.set("event.action", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.admin").cloned() {
                    event.set("user.name", v)?;
                }
                Ok(())
            })();

            let _cond = { !event.has_value("process.executable") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("symantec_endpoint.log.application").cloned() {
                        event.set("process.executable", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.application_name").cloned() {
                    event.set("file.pe.product", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("symantec_endpoint.log.application_version")
                    .cloned()
                {
                    event.set("file.pe.file_version", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("symantec_endpoint.log.begin") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd HH:mm:ss"],
                        event.get_str("_conf.tz_offset"),
                        None,
                    ) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "symantec_endpoint.log.begin".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("symantec_endpoint.log.event_description") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" Caller MD5=") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" Caller MD5=") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("process.hash.md5", remaining));
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("symantec_endpoint.log.caller_process_id") {
                    if let Some(val) = event.get("symantec_endpoint.log.caller_process_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.caller_process_id".into(),
                                message,
                            }
                        })?;
                        event.set("process.pid", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event
                    .remove("symantec_endpoint.log.caller_process_id")
                    .is_none()
                {
                    return Err(TransformError::FieldNotFound {
                        path: "symantec_endpoint.log.caller_process_id".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { !event.has_value("process.executable") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("symantec_endpoint.log.caller_process_name")
                        .cloned()
                    {
                        event.set("process.executable", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("symantec_endpoint.log.certificate_issuer") };
            if _cond {
                event.append(
                    "file.x509.issuer.common_name",
                    json!(
                        event
                            .get("symantec_endpoint.log.certificate_issuer")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("symantec_endpoint.log.certificate_serial_number")
                    .cloned()
                {
                    event.set("file.x509.serial_number", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("symantec_endpoint.log.certificate_signer") };
            if _cond {
                event.append(
                    "file.x509.issuer.common_name",
                    json!(
                        event
                            .get("symantec_endpoint.log.certificate_signer")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("symantec_endpoint.log.certificate_thumbprint")
                    && event
                        .get_as_string("symantec_endpoint.log.certificate_thumbprint")
                        .is_some_and(|s| s.len() == 40)
            };
            if _cond {
                map_strings(
                    event,
                    "symantec_endpoint.log.certificate_thumbprint",
                    "file.hash.sha1",
                    str::to_lowercase,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.company_name").cloned() {
                    event.set("file.pe.company", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.computer_name").cloned() {
                    if !event.has("host.hostname") {
                        event.set("host.hostname", v)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("user.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("symantec_endpoint.log.domain_name").cloned() {
                        event.set("user.domain", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("process.executable") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("symantec_endpoint.log.downloaded_by").cloned() {
                        event.set("process.executable", v)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(
                    event,
                    "symantec_endpoint.log.download_site",
                    "url",
                    true,
                    false,
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("symantec_endpoint.log.duration_seconds") {
                    if let Some(val) = event.get("symantec_endpoint.log.duration_seconds") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.duration_seconds".into(),
                                message,
                            }
                        })?;
                        event.set("event.duration", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("event.duration") };
            if _cond {
                // Painless script, resolved to its runners at generation time
                // Source: ctx.event['duration'] = ctx.event.duration * 1e9;\n
                scale_field(
                    event,
                    &ScaleField::new(
                        "event.duration",
                        "event.duration",
                        Factor::Double(1000000000.0),
                    ),
                );
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("symantec_endpoint.log.end") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd HH:mm:ss"],
                        event.get_str("_conf.tz_offset"),
                        None,
                    ) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "symantec_endpoint.log.end".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("symantec_endpoint.log.event_description")
                    .cloned()
                {
                    event.set("message", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("symantec_endpoint.log.event_time") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("symantec_endpoint.log.event_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss"],
                            event.get_str("_conf.tz_offset"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("symantec_endpoint.log.event_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "symantec_endpoint.log.event_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("symantec_endpoint.log.event_time") };
            if _cond {
                if let Some(v) = event.get("symantec_endpoint.log.event_time").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.file_path").cloned() {
                    event.set("file.path", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("symantec_endpoint.log.file_size_bytes") {
                    if let Some(val) = event.get("symantec_endpoint.log.file_size_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.file_size_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("file.size", converted)?;
                    }
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("symantec_endpoint.log.infected") {
                    if let Some(val) = event.get("symantec_endpoint.log.infected") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.infected".into(),
                                message,
                            }
                        })?;
                        event.set("symantec_endpoint.log.infected", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("symantec_endpoint.log.infected").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "symantec_endpoint.log.infected".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("symantec_endpoint.log.inserted") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("symantec_endpoint.log.inserted") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss"],
                            event.get_str("_conf.tz_offset"),
                            None,
                        ) {
                            Some(parsed) => event.set("symantec_endpoint.log.inserted", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "symantec_endpoint.log.inserted".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.intrusion_id").cloned() {
                    event.set("rule.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.intrusion_url").cloned() {
                    event.set("url.original", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("symantec_endpoint.log.ip_address") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("symantec_endpoint.log.ip_address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.ip_address".into(),
                                message,
                            }
                        })?;
                        event.set("_temp.ip", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_temp.ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("_temp.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("_temp.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("symantec_endpoint.log.last_update_time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("symantec_endpoint.log.last_update_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss"],
                            event.get_str("_conf.tz_offset"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("symantec_endpoint.log.last_update_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "symantec_endpoint.log.last_update_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("symantec_endpoint.log.last_update_time")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "symantec_endpoint.log.last_update_time".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("symantec_endpoint.log.local_host_ip")
                    && event.get_str("symantec_endpoint.log.local_host_ip") != Some("0.0.0.0")
            };
            if _cond {
                if let Some(v) = event.get("symantec_endpoint.log.local_host_ip").cloned() {
                    event.set("source.address", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.local_host_mac").cloned() {
                    event.set("source.mac", v)?;
                }
                Ok(())
            })();

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("[-:.]"),
                    "",
                )?;
            }

            let _cond = { event.get_str("source.mac") == Some("000000000000") };
            if _cond {
                if event.remove("source.mac").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.mac".into(),
                    });
                }
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            let _cond = { event.get_str("symantec_endpoint.log.local_host_name") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("symantec_endpoint.log.local_host_name").cloned() {
                        event.set("source.domain", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("symantec_endpoint.log.local_port") != Some("0") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("symantec_endpoint.log.local_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.local_port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.location").cloned() {
                    event.set("source.geo.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.md-5").cloned() {
                    event.set("process.hash.md5", v)?;
                }
                Ok(())
            })();

            if event.has_value("process.hash.md5") {
                map_strings(
                    event,
                    "process.hash.md5",
                    "process.hash.md5",
                    str::to_lowercase,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.network_protocol").cloned() {
                    event.set("network.transport", v)?;
                }
                Ok(())
            })();

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("symantec_endpoint.log.occurrences") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "symantec_endpoint.log.occurrences".into(),
                            message,
                        }
                    })?;
                    event.set("event.count", converted)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("symantec_endpoint.log.omitted") {
                    if let Some(val) = event.get("symantec_endpoint.log.omitted") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.omitted".into(),
                                message,
                            }
                        })?;
                        event.set("symantec_endpoint.log.omitted", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("symantec_endpoint.log.omitted").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "symantec_endpoint.log.omitted".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("symantec_endpoint.log.remote_host_ip")
                    && event.get_str("symantec_endpoint.log.remote_host_ip") != Some("0.0.0.0")
            };
            if _cond {
                if let Some(v) = event.get("symantec_endpoint.log.remote_host_ip").cloned() {
                    event.set("destination.address", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.remote_host_mac").cloned() {
                    event.set("destination.mac", v)?;
                }
                Ok(())
            })();

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("[-:.]"),
                    "",
                )?;
            }

            let _cond = { event.get_str("destination.mac") == Some("000000000000") };
            if _cond {
                if event.remove("destination.mac").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.mac".into(),
                    });
                }
            }

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("(..)(?!$)"),
                    "$1-",
                )?;
            }

            if event.has_value("destination.mac") {
                map_strings(
                    event,
                    "destination.mac",
                    "destination.mac",
                    str::to_uppercase,
                )?;
            }

            let _cond = { event.get_str("symantec_endpoint.log.remote_host_name") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("symantec_endpoint.log.remote_host_name").cloned() {
                        event.set("destination.domain", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("symantec_endpoint.log.remote_port") != Some("0") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("symantec_endpoint.log.remote_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.remote_port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.port", converted)?;
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.rule").cloned() {
                    event.set("rule.name", v)?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("symantec_endpoint.log.sensitivity") {
                    if let Some(val) = event.get("symantec_endpoint.log.sensitivity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.sensitivity".into(),
                                message,
                            }
                        })?;
                        event.set("symantec_endpoint.log.sensitivity", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("symantec_endpoint.log.sensitivity").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "symantec_endpoint.log.sensitivity".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.sha-256").cloned() {
                    event.set("process.hash.sha256", v)?;
                }
                Ok(())
            })();

            if event.has_value("process.hash.sha256") {
                map_strings(
                    event,
                    "process.hash.sha256",
                    "process.hash.sha256",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.has_value("symantec_endpoint.log.signing_timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("symantec_endpoint.log.signing_timestamp")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("symantec_endpoint.log.signing_timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "symantec_endpoint.log.signing_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    if event
                        .remove("symantec_endpoint.log.signing_timestamp")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "symantec_endpoint.log.signing_timestamp".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("symantec_endpoint.log.signing_timestamp")
                    .cloned()
                {
                    event.set("file.x509.not_before", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.source_computer").cloned() {
                    event.set("source.domain", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.source_ip").cloned() {
                    event.set("source.address", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("symantec_endpoint.log.submission_recommended")
                    && event
                        .get_str("symantec_endpoint.log.submission_recommended")
                        .is_some_and(|s| s.to_lowercase().contains("yes"))
            };
            if _cond {
                event.set("symantec_endpoint.log.submission_recommended", json!(true))?;
            }

            let _cond = {
                event.has_value("symantec_endpoint.log.submission_recommended")
                    && !(event
                        .get_str("symantec_endpoint.log.submission_recommended")
                        .is_some_and(|s| s.to_lowercase().contains("yes")))
            };
            if _cond {
                event.set("symantec_endpoint.log.submission_recommended", json!(false))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("symantec_endpoint.log.traffic_direction")
                    .cloned()
                {
                    event.set("network.direction", v)?;
                }
                Ok(())
            })();

            if event.has_value("network.direction") {
                map_strings(
                    event,
                    "network.direction",
                    "network.direction",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("network.direction") == Some("inbound") };
            if _cond {
                event.set("network.direction", json!("ingress"))?;
            }

            let _cond = { event.get_str("network.direction") == Some("outbound") };
            if _cond {
                event.set("network.direction", json!("egress"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("symantec_endpoint.log.threats") {
                    if let Some(val) = event.get("symantec_endpoint.log.threats") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.threats".into(),
                                message,
                            }
                        })?;
                        event.set("symantec_endpoint.log.threats", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("symantec_endpoint.log.threats").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "symantec_endpoint.log.threats".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("symantec_endpoint.log.total_files") {
                    if let Some(val) = event.get("symantec_endpoint.log.total_files") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "symantec_endpoint.log.total_files".into(),
                                message,
                            }
                        })?;
                        event.set("symantec_endpoint.log.total_files", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("symantec_endpoint.log.total_files").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "symantec_endpoint.log.total_files".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("symantec_endpoint.log.user_name").cloned() {
                    event.set("user.name", v)?;
                }
                Ok(())
            })();

            let _cond =
                { event.has_value("symantec_endpoint.log.user1") && !event.has_value("user.name") };
            if _cond {
                if let Some(v) = event.get("symantec_endpoint.log.user1").cloned() {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.get_bool("_conf.remove_mapped_fields") == Some(true) };
            if _cond {
                event.remove("symantec_endpoint.log.action");
                event.remove("symantec_endpoint.log.actual_action");
                event.remove("symantec_endpoint.log.admin");
                event.remove("symantec_endpoint.log.application");
                event.remove("symantec_endpoint.log.application_name");
                event.remove("symantec_endpoint.log.application_version");
                event.remove("symantec_endpoint.log.begin");
                event.remove("symantec_endpoint.log.caller_process_id");
                event.remove("symantec_endpoint.log.caller_process_name");
                event.remove("symantec_endpoint.log.certificate_serial_number");
                event.remove("symantec_endpoint.log.certificate_thumbprint");
                event.remove("symantec_endpoint.log.company_name");
                event.remove("symantec_endpoint.log.domain_name");
                event.remove("symantec_endpoint.log.download_site");
                event.remove("symantec_endpoint.log.downloaded_by");
                event.remove("symantec_endpoint.log.duration_seconds");
                event.remove("symantec_endpoint.log.end");
                event.remove("symantec_endpoint.log.event_description");
                event.remove("symantec_endpoint.log.event_time");
                event.remove("symantec_endpoint.log.file_path");
                event.remove("symantec_endpoint.log.file_size_bytes");
                event.remove("symantec_endpoint.log.inserted");
                event.remove("symantec_endpoint.log.intrusion_id");
                event.remove("symantec_endpoint.log.intrusion_url");
                event.remove("symantec_endpoint.log.last_update_time");
                event.remove("symantec_endpoint.log.local_host_ip");
                event.remove("symantec_endpoint.log.local_host_mac");
                event.remove("symantec_endpoint.log.local_host_name");
                event.remove("symantec_endpoint.log.local_port");
                event.remove("symantec_endpoint.log.location");
                event.remove("symantec_endpoint.log.md-5");
                event.remove("symantec_endpoint.log.network_protocol");
                event.remove("symantec_endpoint.log.occurrences");
                event.remove("symantec_endpoint.log.remote_host_ip");
                event.remove("symantec_endpoint.log.remote_host_mac");
                event.remove("symantec_endpoint.log.remote_host_name");
                event.remove("symantec_endpoint.log.remote_port");
                event.remove("symantec_endpoint.log.rule");
                event.remove("symantec_endpoint.log.sha-256");
                event.remove("symantec_endpoint.log.signing_timestamp");
                event.remove("symantec_endpoint.log.source_computer");
                event.remove("symantec_endpoint.log.source_ip");
                event.remove("symantec_endpoint.log.submission_recommended");
                event.remove("symantec_endpoint.log.traffic_direction");
                event.remove("symantec_endpoint.log.user1");
                event.remove("symantec_endpoint.log.user_name");
            }

            let _cond = {
                event.has_value("symantec_endpoint.log")
                    && event.get("symantec_endpoint.log").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                event.remove("symantec_endpoint");
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.get_str("event.action") == Some("blocked")
                    || (event.has_value("message")
                        && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("not blocked"))
                            }
                            serde_json::Value::String(s) => s.contains("not blocked"),
                            _ => false,
                        }))
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("blocked"))
                            }
                            serde_json::Value::String(s) => s.contains("blocked"),
                            _ => false,
                        }))
            };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = {
                event.get_str("event.action") == Some("not blocked")
                    || (event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("not blocked"))
                            }
                            serde_json::Value::String(s) => s.contains("not blocked"),
                            _ => false,
                        }))
            };
            if _cond {
                event.append_unique("event.type", json!("allowed"))?;
            }

            let _cond = {
                event.get_str("event.provider") == Some("Administrative Log")
                    && event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("log on"))
                        }
                        serde_json::Value::String(s) => s.contains("log on"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.get_str("event.provider") == Some("Administrative Log")
                    && event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("log on"))
                        }
                        serde_json::Value::String(s) => s.contains("log on"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.get_str("event.provider") == Some("Administrative Log")
                    && event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("log on failed"))
                        }
                        serde_json::Value::String(s) => s.contains("log on failed"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event.get_str("event.provider") == Some("Administrative Log")
                    && event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("log on succeeded"))
                        }
                        serde_json::Value::String(s) => s.contains("log on succeeded"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("destination.address") {
                    if let Some(val) = event.get("destination.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.address".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("source.ip")
                    && !(event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.mac") };
            if _cond {
                event.append_unique(
                    "host.mac",
                    json!(
                        event
                            .get("source.mac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("source.domain").cloned() {
                    if !event.has("host.hostname") {
                        event.set("host.hostname", v)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("host.hostname").cloned() {
                    if !event.has("host.name") {
                        event.set("host.name", v)?;
                    }
                }
                Ok(())
            })();

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

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha1") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.hash.md5") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("process.hash.sha256") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("process.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("source.ip"),
                    event.get_string("destination.ip"),
                    event
                        .get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("source.port", "destination.port")
                    };
                    let src_port =
                        u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port =
                        u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("network.community_id", cid)?,
                        Err(message) => {
                            return Err(TransformError::ParseError {
                                path: "network.community_id".into(),
                                message,
                            });
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("source.geo") };
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

            let _cond = { !event.has_value("destination.geo") };
            if _cond {
                if event.has_value("destination.ip") {
                    if let Some(ip_str) = event.get_string("destination.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("destination.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("destination.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("destination.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("destination.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("destination.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("destination.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("destination.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("destination.geo.location", v.clone())?;
                            }
                        }
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

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
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

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = {
                event.get_str("network.direction") == Some("ingress")
                    && event.has_value("source")
                    && event.has_value("destination")
            };
            if _cond {
                // Painless script
                // Source: def tmp = ctx.source;\nctx.source = ctx.destination;\nctx.destination = tmp;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def tmp = ctx.source;\nctx.source = ctx.destination;\nctx.destination = tmp;\n"#
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("debug"))
                        }
                        serde_json::Value::String(s) => s.contains("debug"),
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("_conf");
                event.remove("_csv_array");
                event.remove("_fingerprint");
                event.remove("_temp");
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("log.syslog.procid") {
                    if let Some(val) = event.get("log.syslog.procid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "log.syslog.procid".into(),
                                message,
                            }
                        })?;
                        event.set("log.syslog.process.pid", converted)?;
                    }
                }
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "processor {}: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                let _cond = {
                    !event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("debug"))
                            }
                            serde_json::Value::String(s) => s.contains("debug"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.remove("_conf");
                    event.remove("_csv_array");
                    event.remove("_csv_map");
                    event.remove("_fingerprint");
                    event.remove("_temp");
                }
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
