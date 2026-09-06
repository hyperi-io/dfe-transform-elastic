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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("message").cloned() {
                    event.set("event.original", v)?;
                }
                Ok(())
            })();

            event.set("observer.vendor", json!("Stormshield"))?;

            // Painless script
            // Source: ctx[\"stormshield\"] = new HashMap();\ndef kvStart = 0; def kvSplit = 0; def kvEnd = 0; def inQuote = false;\nfor (int i = 0, n = ctx[\"message\"].length(); i < n; ++i) {\n  char c = ctx[\"message\"].charAt(i);\n  if (c == (char)'\"') {\n    inQuote = !inQuote;\n  }\n  if (inQuote) {\n    continue;\n  }\n  \n  if (c == (char)'=') {\n    kvSplit = i;\n  }\n  if (c == (char)' ' || (i == n - 1)) {\n    if (kvStart != kvSplit) {\n      def key = ctx[\"message\"].substring(kvStart, kvSplit);\n      def end = i;\n      if (i == n - 1)  {\n          end = n;\n      }\n      def value = ctx[\"message\"].substring(kvSplit + 1, end).replace(\"\\\"\", \"\");\n      ctx[\"stormshield\"][key] = value;\n    }\n\n    kvStart = i + 1;\n    kvSplit = i + 1;\n  }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx[\"stormshield\"] = new HashMap();\ndef kvStart = 0; def kvSplit = 0; def kvEnd = 0; def inQuote = false;\nfor (int i = 0, n = ctx[\"message\"].length(); i < n; ++i) {\n  char c = ctx[\"message\"].charAt(i);\n  if (c == (char)'\"') {\n    inQuote = !inQuote;\n  }\n  if (inQuote) {\n    continue;\n  }\n  \n  if (c == (char)'=') {\n    kvSplit = i;\n  }\n  if (c == (char)' ' || (i == n - 1)) {\n    if (kvStart != kvSplit) {\n      def key = ctx[\"message\"].substring(kvStart, kvSplit);\n      def end = i;\n      if (i == n - 1)  {\n          end = n;\n      }\n      def value = ctx[\"message\"].substring(kvSplit + 1, end).replace(\"\\\"\", \"\");\n      ctx[\"stormshield\"][key] = value;\n    }\n\n    kvStart = i + 1;\n    kvSplit = i + 1;\n  }\n}"#
                ),
            )?;

            event.remove("message");

            if event.has_value("stormshield.msg") {
                event.rename("stormshield.msg", "message")?;
            }

            if let Some(input) = event.get_string("stormshield.tz") {
                // Grok pattern: (?:(?P<_temp__tz_offset>(?:[+-]?)))(?:%{HOUR:_temp_.tz_hour}):?(?:%{MINUTE:_temp_.tz_minute})
                if !cached_grok_mapped!("(?:(?P<_temp__tz_offset>(?:[+-]?)))(?:%{HOUR:_temp_.tz_hour}):?(?:%{MINUTE:_temp_.tz_minute})", [("_temp__tz_offset", "_temp_.tz_offset")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
            }

            event.remove("stormshield.tz");

            let _cond = { event.has_value("_temp_.tz_hour") };
            if _cond {
                event.set(
                    "event.timezone",
                    json!(format!(
                        "{}{}:{}",
                        event
                            .get("_temp_.tz_offset")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_temp_.tz_hour")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_temp_.tz_minute")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("stormshield.time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("stormshield.time") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd HH:mm:ss"],
                        event.get_str("event.timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "stormshield.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("stormshield.startime") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("stormshield.startime") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd HH:mm:ss"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("event.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "stormshield.startime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "format_startime")?;
                    event.remove("event.start");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("stormshield.logtype") == Some("filterstat") };
            if _cond {
                // Begin nested pipeline: "filterstat"
                // Painless script
                // Source: Pattern compositeKeyPattern = /^(\\w+)\\((\\w+)\\/(\\w+)\\)$/; Pattern compositeValuePattern = /^(\\w+)\\/(\\w+)$/;\ndef replacePairs = [:]; def move = []; ctx.stormshield.forEach((k, v) -> {\n  if (k.contains('(')) {\n    replacePairs[k] = v;\n  } else if (Character.isUpperCase(k.charAt(0))) {\n    move.add(k);\n  }\n\n  return true;\n});\nif (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} move.forEach(k -> {\n  ctx.stormshield.metadata[k] = ctx.stormshield[k];\n  ctx.stormshield.remove(k);\n  return true;\n}); replacePairs.forEach((k, v) -> {\n  def keyMatcher = compositeKeyPattern.matcher(k);\n  if (!keyMatcher.matches()) {\n    return false;\n  }\n  def valueMatcher = compositeValuePattern.matcher(v);\n  if (!valueMatcher.matches()) {\n    return false;\n  }\n  def newSubkeys = [:];\n  def newSubkey0 = keyMatcher.group(2);\n  def newSubkey1 = keyMatcher.group(3);\n  newSubkey0 = params.containsKey(newSubkey0) ? params[newSubkey0] : newSubkey0;\n  newSubkey1 = params.containsKey(newSubkey1) ? params[newSubkey1] : newSubkey1;\n  newSubkeys[newSubkey0] = Long.parseLong(valueMatcher.group(1));\n  newSubkeys[newSubkey1] = Long.parseLong(valueMatcher.group(2));\n  ctx.stormshield.metadata[keyMatcher.group(1)] = newSubkeys;\n  ctx.stormshield.remove(k);\n});
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"Pattern compositeKeyPattern = /^(\\w+)\\((\\w+)\\/(\\w+)\\)$/; Pattern compositeValuePattern = /^(\\w+)\\/(\\w+)$/;\ndef replacePairs = [:]; def move = []; ctx.stormshield.forEach((k, v) -> {\n  if (k.contains('(')) {\n    replacePairs[k] = v;\n  } else if (Character.isUpperCase(k.charAt(0))) {\n    move.add(k);\n  }\n\n  return true;\n});\nif (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} move.forEach(k -> {\n  ctx.stormshield.metadata[k] = ctx.stormshield[k];\n  ctx.stormshield.remove(k);\n  return true;\n}); replacePairs.forEach((k, v) -> {\n  def keyMatcher = compositeKeyPattern.matcher(k);\n  if (!keyMatcher.matches()) {\n    return false;\n  }\n  def valueMatcher = compositeValuePattern.matcher(v);\n  if (!valueMatcher.matches()) {\n    return false;\n  }\n  def newSubkeys = [:];\n  def newSubkey0 = keyMatcher.group(2);\n  def newSubkey1 = keyMatcher.group(3);\n  newSubkey0 = params.containsKey(newSubkey0) ? params[newSubkey0] : newSubkey0;\n  newSubkey1 = params.containsKey(newSubkey1) ? params[newSubkey1] : newSubkey1;\n  newSubkeys[newSubkey0] = Long.parseLong(valueMatcher.group(1));\n  newSubkeys[newSubkey1] = Long.parseLong(valueMatcher.group(2));\n  ctx.stormshield.metadata[keyMatcher.group(1)] = newSubkeys;\n  ctx.stormshield.remove(k);\n});"#
                    ),
                    cached_params!("{\"i\":\"in_count\",\"o\":\"out_count\"}"),
                )?;
                // End nested pipeline: "filterstat"
            }

            let _cond = { event.get_str("stormshield.logtype") == Some("monitor") };
            if _cond {
                // Begin nested pipeline: "monitor"
                // Painless script
                // Source: def deviceStats = [:]; def specialCases = [:]; ctx.stormshield.forEach((k, v) -> {\n  params.devices.forEach(d -> {\n    if (k.startsWith(d)) {\n      deviceStats[k] = [:];\n      deviceStats[k][\"value\"] = v;\n      deviceStats[k][\"stem\"] = d;\n    }\n\n    return true;\n  });\n  params.special_cases.forEach(s -> {\n    def sp_k = s[\"key\"];\n    def sp_v = s[\"value\"];\n    if (k.startsWith(sp_k)) {\n      def _map = [:];\n      def values = v.splitOnToken(',', 3);\n      for (int i = 0; i < sp_v.length; ++i) {\n        _map[sp_v[i]] = Long.parseLong(values[i]);\n      }\n      specialCases[sp_k] = _map;\n    }\n\n    return true;\n  });\n});\nif (deviceStats.isEmpty() && specialCases.isEmpty()) {\n  return;\n}\ndef result = [:]; for (entry in deviceStats.entrySet()) {\n  def entryValue = entry.getValue();\n  def stem = entryValue[\"stem\"];\n  if (!result.containsKey(stem)) {\n    result[stem] = [];\n  }\n  def values = entryValue[\"value\"].splitOnToken(',', 7);\n  def item = [:];\n  item[\"original\"] = entry.getKey();\n  for (int i = 0; i < values.length; ++i) {\n    if (i == 0) {\n      item[params.stats[i]] = values[i];\n    } else {\n      item[params.stats[i]] = Long.parseLong(values[i]);        \n    }\n  }\n  result[stem].add(item);\n  ctx.stormshield.remove(entry.getKey());\n} for (entry in result.entrySet()) {\n  result[entry.getKey()].sort((a, b) -> a[\"original\"].compareTo(b[\"original\"]));\n} if (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield[\"metadata\"] = [:];\n} ctx.stormshield.metadata.device_stats = result; for (entry in specialCases.entrySet()) {\n  ctx.stormshield.metadata.device_stats[entry.getKey()] = entry.getValue();\n  ctx.stormshield.remove(entry.getKey());\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def deviceStats = [:]; def specialCases = [:]; ctx.stormshield.forEach((k, v) -> {\n  params.devices.forEach(d -> {\n    if (k.startsWith(d)) {\n      deviceStats[k] = [:];\n      deviceStats[k][\"value\"] = v;\n      deviceStats[k][\"stem\"] = d;\n    }\n\n    return true;\n  });\n  params.special_cases.forEach(s -> {\n    def sp_k = s[\"key\"];\n    def sp_v = s[\"value\"];\n    if (k.startsWith(sp_k)) {\n      def _map = [:];\n      def values = v.splitOnToken(',', 3);\n      for (int i = 0; i < sp_v.length; ++i) {\n        _map[sp_v[i]] = Long.parseLong(values[i]);\n      }\n      specialCases[sp_k] = _map;\n    }\n\n    return true;\n  });\n});\nif (deviceStats.isEmpty() && specialCases.isEmpty()) {\n  return;\n}\ndef result = [:]; for (entry in deviceStats.entrySet()) {\n  def entryValue = entry.getValue();\n  def stem = entryValue[\"stem\"];\n  if (!result.containsKey(stem)) {\n    result[stem] = [];\n  }\n  def values = entryValue[\"value\"].splitOnToken(',', 7);\n  def item = [:];\n  item[\"original\"] = entry.getKey();\n  for (int i = 0; i < values.length; ++i) {\n    if (i == 0) {\n      item[params.stats[i]] = values[i];\n    } else {\n      item[params.stats[i]] = Long.parseLong(values[i]);        \n    }\n  }\n  result[stem].add(item);\n  ctx.stormshield.remove(entry.getKey());\n} for (entry in result.entrySet()) {\n  result[entry.getKey()].sort((a, b) -> a[\"original\"].compareTo(b[\"original\"]));\n} if (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield[\"metadata\"] = [:];\n} ctx.stormshield.metadata.device_stats = result; for (entry in specialCases.entrySet()) {\n  ctx.stormshield.metadata.device_stats[entry.getKey()] = entry.getValue();\n  ctx.stormshield.remove(entry.getKey());\n}"#
                    ),
                    cached_params!(
                        "{\"special_cases\":[{\"key\":\"CPU\",\"value\":[\"user_time\",\"kernel_time\",\"system_disruption\"]}],\"devices\":[\"agg\",\"ipsec\",\"sslvpn\",\"Wifi\",\"Qid\",\"Vlan\"],\"stats\":[\"name\",\"incoming_throughput\",\"maximum_incoming_throughput\",\"outgoing_throughput\",\"maximum_outgoing_throughput\",\"packets_accepted\",\"packets_blocked\"]}"
                    ),
                )?;
                // Painless script
                // Source: def deviceStats = [:]; ctx.stormshield.forEach((k, v) -> {\n  params.devices.forEach(d -> {\n    if (k.startsWith(d)) {\n      deviceStats[k] = [:];\n      deviceStats[k][\"value\"] = v;\n      deviceStats[k][\"stem\"] = d;\n    }\n\n    return true;\n  });\n});\nif (deviceStats.isEmpty()) {\n  return;\n}\nif (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} def ports = []; for (entry in deviceStats.entrySet()) {\n  def entryKey = entry.getKey();\n  def entryValue = entry.getValue();\n  def values = entryValue[\"value\"].splitOnToken(',', 7);\n  def item = [:];\n  ports.add(entryKey);\n  item[\"original\"] = entryKey;\n  for (int i = 0; i < values.length; ++i) {\n    if (i == 0) {\n      item[params.stats[i]] = values[i];\n    } else {\n      item[params.stats[i]] = Long.parseLong(values[i]);        \n    }\n  }\n  ctx.stormshield.remove(entryKey);\n  ctx.stormshield.metadata[entryKey] = item;\n} ports.sort((a, b) -> a.compareTo(b)); ctx.stormshield.ports = ports;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def deviceStats = [:]; ctx.stormshield.forEach((k, v) -> {\n  params.devices.forEach(d -> {\n    if (k.startsWith(d)) {\n      deviceStats[k] = [:];\n      deviceStats[k][\"value\"] = v;\n      deviceStats[k][\"stem\"] = d;\n    }\n\n    return true;\n  });\n});\nif (deviceStats.isEmpty()) {\n  return;\n}\nif (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} def ports = []; for (entry in deviceStats.entrySet()) {\n  def entryKey = entry.getKey();\n  def entryValue = entry.getValue();\n  def values = entryValue[\"value\"].splitOnToken(',', 7);\n  def item = [:];\n  ports.add(entryKey);\n  item[\"original\"] = entryKey;\n  for (int i = 0; i < values.length; ++i) {\n    if (i == 0) {\n      item[params.stats[i]] = values[i];\n    } else {\n      item[params.stats[i]] = Long.parseLong(values[i]);        \n    }\n  }\n  ctx.stormshield.remove(entryKey);\n  ctx.stormshield.metadata[entryKey] = item;\n} ports.sort((a, b) -> a.compareTo(b)); ctx.stormshield.ports = ports;"#
                    ),
                    cached_params!(
                        "{\"devices\":[\"Ethernet\"],\"stats\":[\"name\",\"incoming_throughput\",\"maximum_incoming_throughput\",\"outgoing_throughput\",\"maximum_outgoing_throughput\",\"packets_accepted\",\"packets_blocked\"]}"
                    ),
                )?;
                // End nested pipeline: "monitor"
            }

            let _cond = { event.get_str("stormshield.logtype") == Some("count") };
            if _cond {
                // Begin nested pipeline: "count"
                // Painless script
                // Source: boolean isNumeric(def s) {\n  if (!(s instanceof String) || s.isEmpty() || s.length() > 18) {\n    return false;\n  }\n  for (int i = 0; i < s.length(); i++) {\n    if (Character.isDigit(s.charAt(i)) == false) {\n      return false;\n    }\n  }\n  return true;\n} def ruleStats = []; def ruleKeys = []; def str = \"Rule\"; ctx.stormshield.forEach((k, v) -> {\n  if (k.startsWith(str) && k.indexOf(\":\") > str.length()) {\n    ruleKeys.add(k);\n    def parts = k.substring(str.length()).splitOnToken(\":\");\n    if (parts.length == 2 && isNumeric(v) && isNumeric(parts[1]) && parts[0].length() == 1 && isNumeric(parts[0])) {\n      int category = Integer.parseInt(parts[0]);\n      if (category < params.stats.size()) {\n        def item = [:];\n        item[\"original\"] = k;\n        item[\"byte_count\"] = Long.parseLong(v);\n        item[\"category\"] = params.stats[category];\n        item[\"rule_number\"] = Long.parseLong(parts[1]);\n        ruleStats.add(item);\n      }\n    }\n  }\n\n  return true;\n});\nfor (k in ruleKeys) {\n  ctx.stormshield.remove(k);\n}\nif (ruleStats.isEmpty()) {\n  return;\n}\nruleStats.sort((a, b) -> a[\"original\"].compareTo(b[\"original\"])); if (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} ctx.stormshield.metadata.rule_stats = ruleStats;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"boolean isNumeric(def s) {\n  if (!(s instanceof String) || s.isEmpty() || s.length() > 18) {\n    return false;\n  }\n  for (int i = 0; i < s.length(); i++) {\n    if (Character.isDigit(s.charAt(i)) == false) {\n      return false;\n    }\n  }\n  return true;\n} def ruleStats = []; def ruleKeys = []; def str = \"Rule\"; ctx.stormshield.forEach((k, v) -> {\n  if (k.startsWith(str) && k.indexOf(\":\") > str.length()) {\n    ruleKeys.add(k);\n    def parts = k.substring(str.length()).splitOnToken(\":\");\n    if (parts.length == 2 && isNumeric(v) && isNumeric(parts[1]) && parts[0].length() == 1 && isNumeric(parts[0])) {\n      int category = Integer.parseInt(parts[0]);\n      if (category < params.stats.size()) {\n        def item = [:];\n        item[\"original\"] = k;\n        item[\"byte_count\"] = Long.parseLong(v);\n        item[\"category\"] = params.stats[category];\n        item[\"rule_number\"] = Long.parseLong(parts[1]);\n        ruleStats.add(item);\n      }\n    }\n  }\n\n  return true;\n});\nfor (k in ruleKeys) {\n  ctx.stormshield.remove(k);\n}\nif (ruleStats.isEmpty()) {\n  return;\n}\nruleStats.sort((a, b) -> a[\"original\"].compareTo(b[\"original\"])); if (!ctx.stormshield.containsKey(\"metadata\")) {\n  ctx.stormshield.metadata = [:];\n} ctx.stormshield.metadata.rule_stats = ruleStats;"#
                    ),
                    cached_params!(
                        "{\"devices\":[\"Rule\"],\"stats\":[\"Implicit Filter Rule\",\"Global Filter Rule\",\"Local Filter Rule\",\"Implicit NAT Rule\",\"Global NAT Rule\",\"Local NAT Rule\"]}"
                    ),
                )?;
                // End nested pipeline: "count"
            }

            let _cond = { event.has_value("stormshield.fw") };
            if _cond {
                if let Some(v) = event.get("stormshield.fw").cloned() {
                    event.set("observer.name", v)?;
                }
            }

            let _cond = { event.has_value("stormshield.id") };
            if _cond {
                if let Some(v) = event.get("stormshield.id").cloned() {
                    event.set("observer.type", v)?;
                }
            }

            if let Some(v) = event
                .get("stormshield.srcif")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.ingress.interface.id", v)?;
            }

            if let Some(v) = event
                .get("stormshield.srcifname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.ingress.interface.name", v)?;
            }

            if let Some(v) = event
                .get("stormshield.dstif")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.egress.interface.id", v)?;
            }

            if let Some(v) = event
                .get("stormshield.dstifname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.egress.interface.name", v)?;
            }

            let _cond = {
                event.has_value("stormshield.modsrc")
                    && !condition_eq(
                        event.get("stormshield.modsrc"),
                        event.get("stormshield.src"),
                    )
            };
            if _cond {
                if let Some(val) = event.get("stormshield.modsrc") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "stormshield.modsrc".into(),
                            message,
                        })?;
                    event.set("source.nat.ip", converted)?;
                }
            }

            if event.has_value("stormshield.src") {
                if let Some(val) = event.get("stormshield.src") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "stormshield.src".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            event.remove("stormshield.src");

            event.remove("stormshield.modsrc");

            let _cond =
                { event.has_value("stormshield.modsrcport") && event.has_value("source.nat.ip") };
            if _cond {
                if let Some(val) = event.get("stormshield.modsrcport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stormshield.modsrcport".into(),
                            message,
                        }
                    })?;
                    event.set("source.nat.port", converted)?;
                }
            }

            if event.has_value("stormshield.srcport") {
                if let Some(val) = event.get("stormshield.srcport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stormshield.srcport".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            event.remove("stormshield.srcport");

            event.remove("stormshield.modsrcport");

            if event.has_value("stormshield.origdst") {
                if let Some(val) = event.get("stormshield.origdst") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "stormshield.origdst".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }

            let _cond =
                { !event.has_value("stormshield.origdst") && event.has_value("stormshield.dst") };
            if _cond {
                if let Some(val) = event.get("stormshield.dst") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "stormshield.dst".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }

            let _cond = {
                event.has_value("stormshield.origdst")
                    && event.has_value("stormshield.dst")
                    && !condition_eq(
                        event.get("stormshield.origdst"),
                        event.get("stormshield.dst"),
                    )
            };
            if _cond {
                if let Some(val) = event.get("stormshield.dst") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "stormshield.dst".into(),
                            message,
                        })?;
                    event.set("destination.nat.ip", converted)?;
                }
            }

            event.remove("stormshield.dst");

            event.remove("stormshield.origdst");

            let _cond =
                { !event.has_value("destination.geo") && event.has_value("destination.ip") };
            if _cond {
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

            let _cond = { event.has_value("destination.ip") };
            if _cond {
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

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = { !event.has_value("source.geo") && event.has_value("source.ip") };
            if _cond {
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

            if event.has_value("stormshield.action") {
                event.rename("stormshield.action", "event.action")?;
            }

            if event.has_value("stormshield.ipproto") {
                event.rename("stormshield.ipproto", "network.transport")?;
            }

            if event.has_value("stormshield.proto") {
                event.rename("stormshield.proto", "network.protocol")?;
            }

            if event.has_value("stormshield.ruleid") {
                event.rename("stormshield.ruleid", "rule.id")?;
            }

            if event.has_value("stormshield.rulename") {
                event.rename("stormshield.rulename", "rule.name")?;
            }

            if event.has_value("stormshield.sent") {
                if let Some(val) = event.get("stormshield.sent") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stormshield.sent".into(),
                            message,
                        }
                    })?;
                    event.set("source.bytes", converted)?;
                }
            }

            if event.has_value("stormshield.rcvd") {
                if let Some(val) = event.get("stormshield.rcvd") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stormshield.rcvd".into(),
                            message,
                        }
                    })?;
                    event.set("destination.bytes", converted)?;
                }
            }

            let _cond = { event.has_value("source.bytes") && event.has_value("destination.bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script, resolved to its runners at generation time
                    // Source: if (ctx.network == null) {\n    ctx.network = [:];\n} ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes;
                    sum_directions(event, &["bytes"]);
                    Ok(())
                })();
            }

            if event.has_value("stormshield.dstname") {
                event.rename("stormshield.dstname", "destination.domain")?;
            }

            if event.has_value("destination.ip") {
                if let Some(val) = event.get("destination.ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "destination.ip".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }

            event.remove("stormshield.dst");

            if event.has_value("stormshield.dstmac") {
                map_strings(
                    event,
                    "stormshield.dstmac",
                    "stormshield.dstmac",
                    str::to_uppercase,
                )?;
            }

            let _cond = { !event.has_value("destination.mac") };
            if _cond {
                if event.has_value("stormshield.dstmac") {
                    gsub_field(
                        event,
                        "stormshield.dstmac",
                        "destination.mac",
                        cached_regex!(":"),
                        "-",
                    )?;
                }
            }

            if event.has_value("stormshield.origdstport") {
                if let Some(val) = event.get("stormshield.origdstport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stormshield.origdstport".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            let _cond =
                { event.has_value("stormshield.dstport") && event.has_value("destination.nat.ip") };
            if _cond {
                if let Some(val) = event.get("stormshield.dstport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stormshield.dstport".into(),
                            message,
                        }
                    })?;
                    event.set("destination.nat.port", converted)?;
                }
            }

            let _cond = {
                event.has_value("stormshield.dstport") && !event.has_value("destination.nat.ip")
            };
            if _cond {
                if let Some(val) = event.get("stormshield.dstport") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stormshield.dstport".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            event.remove("stormshield.dstport");

            event.remove("stormshield.origdstport");

            let _cond = { !event.has_value("destination.geo") };
            if _cond {
                if event.has_value("stormshield.dstcountry") {
                    event.rename("stormshield.dstcountry", "destination.geo.country_iso_code")?;
                }
            }

            let _cond = { !event.has_value("source.geo") };
            if _cond {
                if event.has_value("stormshield.srccountry") {
                    event.rename("stormshield.srccountry", "source.geo.country_iso_code")?;
                }
            }

            if event.has_value("stormshield.srcmac") {
                map_strings(
                    event,
                    "stormshield.srcmac",
                    "stormshield.srcmac",
                    str::to_uppercase,
                )?;
            }

            let _cond = { !event.has_value("source.mac") };
            if _cond {
                if event.has_value("stormshield.srcmac") {
                    gsub_field(
                        event,
                        "stormshield.srcmac",
                        "source.mac",
                        cached_regex!(":"),
                        "-",
                    )?;
                }
            }

            event.remove("stormshield.srcmac");

            // Painless script
            // Source: if (ctx.stormshield?.duration != null) {\n    def duration = Float.parseFloat(ctx.stormshield.duration);\n    duration *= 1000000000; // 10^9 nanoseconds per second\n    if (!ctx.containsKey(\"event\")) {\n        ctx.event = [:];\n    }\n\n    ctx.event.duration = (int)duration;\n    ctx.stormshield.remove(\"duration\");\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.stormshield?.duration != null) {\n    def duration = Float.parseFloat(ctx.stormshield.duration);\n    duration *= 1000000000; // 10^9 nanoseconds per second\n    if (!ctx.containsKey(\"event\")) {\n        ctx.event = [:];\n    }\n\n    ctx.event.duration = (int)duration;\n    ctx.stormshield.remove(\"duration\");\n}"#
                ),
            )?;

            let _cond = { !event.has_value("source.port") };
            if _cond {
                if event.has_value("stormshield.srcport") {
                    if let Some(val) = event.get("stormshield.srcport") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "stormshield.srcport".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
            }

            event.remove("stormshield.srcport");

            let _cond = { event.get_str("source.geo.country_name") == Some("Reserved") };
            if _cond {
                if event.remove("source.geo").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.geo".into(),
                    });
                }
            }

            let _cond = { event.get_str("destination.geo.country_name") == Some("Reserved") };
            if _cond {
                if event.remove("destination.geo").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "destination.geo".into(),
                    });
                }
            }

            if event.has_value("stormshield.user") {
                event.rename("stormshield.user", "user.name")?;
            }

            let _cond = { event.has_value("stormshield.ipv") };
            if _cond {
                // Painless script
                // Source: if (ctx.stormshield.ipv == \"4\") {\n    ctx.network.type = \"ipv4\";\n    ctx.stormshield.remove(\"ipv\");\n} else if (ctx.stormshield.ipv == \"6\") {\n    ctx.network.type = \"ipv6\";\n    ctx.stormshield.remove(\"ipv\");\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.stormshield.ipv == \"4\") {\n    ctx.network.type = \"ipv4\";\n    ctx.stormshield.remove(\"ipv\");\n} else if (ctx.stormshield.ipv == \"6\") {\n    ctx.network.type = \"ipv6\";\n    ctx.stormshield.remove(\"ipv\");\n}"#
                    ),
                )?;
            }

            if let Some(v) = event
                .get("stormshield.alarmid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.code", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("stormshield.risk") {
                    if let Some(val) = event.get("stormshield.risk") {
                        let converted = convert_value(val, "float").map_err(|message| {
                            TransformError::ParseError {
                                path: "stormshield.risk".into(),
                                message,
                            }
                        })?;
                        event.set("event.risk_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_stormshield_risk_to_event_risk_score",
                )?;
                event.remove("event.risk_score");
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.get_str("stormshield.logtype") == Some("alarm")
                    && event.has_value("stormshield.pri")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("stormshield.pri") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "stormshield.pri".into(),
                                message,
                            }
                        })?;
                        event.set("event.severity", converted)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("stormshield.logtype") == Some("web")
                    && event.has_value("stormshield.arg")
            };
            if _cond {
                if let Some(v) = event
                    .get("stormshield.arg")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("url.original", v)?;
                }
            }

            let _cond = {
                event.get_str("stormshield.logtype") == Some("web")
                    && event.has_value("destination.domain")
            };
            if _cond {
                if let Some(v) = event.get("destination.domain").cloned() {
                    event.set("url.domain", v)?;
                }
            }

            let _cond = {
                event.has_value("stormshield.srcname")
                    && event.get_str("stormshield.srcname") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("stormshield.srcname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script
            // Source: if (!ctx.stormshield.containsKey(\"metadata\")) {\n    ctx.stormshield.metadata = [:];\n} params.names.forEach(k -> {\n    if (ctx.stormshield.containsKey(k)) {\n        ctx.stormshield.metadata[k] = ctx.stormshield[k];\n        ctx.stormshield.remove(k);\n    }\n    return true;\n});
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (!ctx.stormshield.containsKey(\"metadata\")) {\n    ctx.stormshield.metadata = [:];\n} params.names.forEach(k -> {\n    if (ctx.stormshield.containsKey(k)) {\n        ctx.stormshield.metadata[k] = ctx.stormshield[k];\n        ctx.stormshield.remove(k);\n    }\n    return true;\n});"#
                ),
                cached_params!(
                    "{\"names\":[\"Pvm\",\"address\",\"alarmid\",\"arg\",\"auth\",\"authcaptive\",\"authconsole\",\"authipsec\",\"authsslvpn\",\"authtotp\",\"authwebadmin\",\"cat_site\",\"class\",\"classification\",\"clientappid\",\"confid\",\"contentpolicy\",\"cookie_i\",\"cookie_r\",\"datechange\",\"domain\",\"dstcontinent\",\"dstcountry\",\"dstportname\",\"error\",\"filename\",\"filetype\",\"groupid\",\"icmpcode\",\"icmptype\",\"id\",\"ikev\",\"localnet\",\"mem\",\"method\",\"modsrc\",\"modsrcport\",\"op\",\"phase\",\"pri\",\"rcvd\",\"remoteid\",\"remotenet\",\"repeat\",\"result\",\"risk\",\"ruletype\",\"sandboxing\",\"sandboxinglevel\",\"security\",\"sensible\",\"sent\",\"serverappid\",\"sessionid\",\"side\",\"slotlevel\",\"slotname\",\"spamlevel\",\"spi_in\",\"spi_out\",\"srccontinent\",\"srccountry\",\"srcname\",\"srcportname\",\"target\",\"totp\",\"urlruleid\",\"usergroup\",\"version\",\"virus\"]}"
                ),
            )?;

            if event.has_value("stormshield.metadata.Byte.in_count") {
                if let Some(val) = event.get("stormshield.metadata.Byte.in_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stormshield.metadata.Byte.in_count".into(),
                            message,
                        }
                    })?;
                    event.set("stormshield.in_bytes", converted)?;
                }
            }

            if event.has_value("stormshield.metadata.Byte.out_count") {
                if let Some(val) = event.get("stormshield.metadata.Byte.out_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "stormshield.metadata.Byte.out_count".into(),
                            message,
                        }
                    })?;
                    event.set("stormshield.out_bytes", converted)?;
                }
            }

            let _cond =
                { event.has_value("event.action") && event.get_str("event.action") == Some("") };
            if _cond {
                if event.remove("event.action").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "event.action".into(),
                    });
                }
            }

            let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Classify network direction against the internal network ranges
                    if let (Some(src), Some(dst)) = (
                        event.get_string("source.ip"),
                        event.get_string("destination.ip"),
                    ) {
                        let networks: Vec<&str> = vec!["private"];
                        let direction = match (
                            ip_in_networks(&src, &networks),
                            ip_in_networks(&dst, &networks),
                        ) {
                            (true, false) => "outbound",
                            (false, true) => "inbound",
                            (true, true) => "internal",
                            (false, false) => "external",
                        };
                        event.set("network.direction", json!(direction))?;
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("error") };
            if _cond {
                // Painless script
                // Source: def logtype = ctx.stormshield?.logtype; def entry = params.logtypes[logtype]; if (ctx.event == null) {\n    ctx.event = [:];\n} if (entry == null) {\n    ctx.event.kind = 'event';\n} else {\n    ctx.event.kind = entry.kind;\n    ctx.event.category = new ArrayList(entry.category);\n    ctx.event.type = new ArrayList(entry.type);\n    if (params.action_logtypes.contains(logtype) && ctx.event.action instanceof String) {\n        def mapped = params.action_types[ctx.event.action.toLowerCase()];\n        if (mapped != null && !ctx.event.type.contains(mapped)) {\n            ctx.event.type.add(mapped);\n        }\n    }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def logtype = ctx.stormshield?.logtype; def entry = params.logtypes[logtype]; if (ctx.event == null) {\n    ctx.event = [:];\n} if (entry == null) {\n    ctx.event.kind = 'event';\n} else {\n    ctx.event.kind = entry.kind;\n    ctx.event.category = new ArrayList(entry.category);\n    ctx.event.type = new ArrayList(entry.type);\n    if (params.action_logtypes.contains(logtype) && ctx.event.action instanceof String) {\n        def mapped = params.action_types[ctx.event.action.toLowerCase()];\n        if (mapped != null && !ctx.event.type.contains(mapped)) {\n            ctx.event.type.add(mapped);\n        }\n    }\n}"#
                    ),
                    cached_params!(
                        "{\"logtypes\":{\"alarm\":{\"kind\":\"alert\",\"category\":[\"intrusion_detection\",\"network\"],\"type\":[\"info\"]},\"auth\":{\"kind\":\"event\",\"category\":[\"authentication\"],\"type\":[\"start\"]},\"authstat\":{\"kind\":\"metric\",\"category\":[\"authentication\"],\"type\":[\"info\"]},\"connection\":{\"kind\":\"event\",\"category\":[\"network\",\"session\"],\"type\":[\"connection\",\"end\"]},\"count\":{\"kind\":\"metric\",\"category\":[\"network\"],\"type\":[\"info\"]},\"date\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"change\"]},\"dmrouting\":{\"kind\":\"event\",\"category\":[\"network\",\"host\"],\"type\":[\"change\"]},\"filter\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"filterstat\":{\"kind\":\"metric\",\"category\":[\"network\"],\"type\":[\"info\"]},\"ftp\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"info\"]},\"ipsecstat\":{\"kind\":\"metric\",\"category\":[\"network\"],\"type\":[\"info\"]},\"monitor\":{\"kind\":\"metric\",\"category\":[\"host\"],\"type\":[\"info\"]},\"plugin\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"protocol\"]},\"pop3\":{\"kind\":\"event\",\"category\":[\"email\",\"network\"],\"type\":[\"info\"]},\"pvm\":{\"kind\":\"event\",\"category\":[\"vulnerability\"],\"type\":[\"info\"]},\"restapi\":{\"kind\":\"event\",\"category\":[\"web\",\"configuration\"],\"type\":[\"access\"]},\"routerstat\":{\"kind\":\"metric\",\"category\":[\"network\"],\"type\":[\"info\"]},\"routing\":{\"kind\":\"event\",\"category\":[\"network\",\"host\"],\"type\":[\"change\"]},\"sandboxing\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"info\"]},\"server\":{\"kind\":\"event\",\"category\":[\"configuration\"],\"type\":[\"access\"]},\"smtp\":{\"kind\":\"event\",\"category\":[\"email\",\"network\"],\"type\":[\"info\"]},\"ssl\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"connection\"]},\"system\":{\"kind\":\"event\",\"category\":[\"host\"],\"type\":[\"info\"]},\"vpn\":{\"kind\":\"event\",\"category\":[\"network\"],\"type\":[\"info\"]},\"web\":{\"kind\":\"event\",\"category\":[\"web\",\"network\"],\"type\":[\"access\"]},\"xvpn\":{\"kind\":\"event\",\"category\":[\"network\",\"session\"],\"type\":[\"connection\"]}},\"action_types\":{\"pass\":\"allowed\",\"block\":\"denied\"},\"action_logtypes\":[\"alarm\",\"connection\",\"filter\",\"ftp\",\"plugin\",\"pop3\",\"smtp\",\"ssl\",\"web\"]}"
                    ),
                )?;
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

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
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

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.name") };
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

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_temp_");
                Ok(())
            })();

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
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
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
