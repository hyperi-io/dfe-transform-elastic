// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_dns` pipeline.
pub struct PipelineDns;

impl Transform for PipelineDns {
    fn name(&self) -> &str {
        "pipeline_dns"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("network.protocol", json!("dns"))?;

                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^zone %{DATA:dns.question.name}/%{DATA:dns.question.class}: notify from %{IP:client.ip}#%{NUMBER:client.port:long}:? %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^transfer of '%{DATA:dns.question.name}/%{DATA:dns.question.class}' from %{IP:client.ip}#%{NUMBER:client.port:long}:? %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^validating %{DATA:dns.question.name}/%{WORD:dns.question.type}: %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) updating zone '%{DATA:dns.question.name}/%{DATA:dns.question.class}': %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): (?:view %{DATA:infoblox_nios.log.view}: )?query failed %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA:infoblox_nios.log.dns.before_query}\\): rewriting query name %{DATA} to '%{DATA:infoblox_nios.log.dns.after_query}', type %{DATA:dns.question.type}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} %{DATA:infoblox_nios.log.dns.header_flags} \\(%{IP:server.ip}\\)$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): transfer of '%{DATA:dns.question.name}/%{DATA:dns.question.class}': %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*CEF:0\\|Infoblox\\|NIOS\\|%{GREEDYDATA:infoblox_nios.log.dns.version}\\|RPZ-%{DATA:dns.answers.type}\\|%{DATA:infoblox_nios.log.dns.answers_policy}\\|\\d+\\|app=DNS dst=%{IP:server.ip} src=%{IP:client.ip} spt=%{NUMBER:client.port:long} view=%{DATA:infoblox_nios.log.dns.view_name} qtype=%{WORD:dns.question.type} msg=%{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*%{GREEDYDATA:_tmp.timestamp} (?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags} %{GREEDYDATA:repeat_message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*%{GREEDYDATA:_tmp.timestamp} (?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dns.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!("^zone %{DATA:dns.question.name}/%{DATA:dns.question.class}: notify from %{IP:client.ip}#%{NUMBER:client.port:long}:? %{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                            cached_grok!("^transfer of '%{DATA:dns.question.name}/%{DATA:dns.question.class}' from %{IP:client.ip}#%{NUMBER:client.port:long}:? %{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                            cached_grok!("^validating %{DATA:dns.question.name}/%{WORD:dns.question.type}: %{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) updating zone '%{DATA:dns.question.name}/%{DATA:dns.question.class}': %{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): (?:view %{DATA:infoblox_nios.log.view}: )?query failed %{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA:infoblox_nios.log.dns.before_query}\\): rewriting query name %{DATA} to '%{DATA:infoblox_nios.log.dns.after_query}', type %{DATA:dns.question.type}$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} %{DATA:infoblox_nios.log.dns.header_flags} \\(%{IP:server.ip}\\)$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags}$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): transfer of '%{DATA:dns.question.name}/%{DATA:dns.question.class}': %{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*CEF:0\\|Infoblox\\|NIOS\\|%{GREEDYDATA:infoblox_nios.log.dns.version}\\|RPZ-%{DATA:dns.answers.type}\\|%{DATA:infoblox_nios.log.dns.answers_policy}\\|\\d+\\|app=DNS dst=%{IP:server.ip} src=%{IP:client.ip} spt=%{NUMBER:client.port:long} view=%{DATA:infoblox_nios.log.dns.view_name} qtype=%{WORD:dns.question.type} msg=%{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*%{GREEDYDATA:_tmp.timestamp} (?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags} %{GREEDYDATA:repeat_message}$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*%{GREEDYDATA:_tmp.timestamp} (?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags}$"),
                            cached_grok!("^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }

            let _cond = { event.has_value("_tmp.timestamp") && event.has_value("event.timezone") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["dd-MMM-yyyy HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSS'Z'"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
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
                event.set("_ingest.on_failure_processor_tag", "date_tmp_timestamp_tz")?;
                        event.remove("_tmp.timestamp");
                        event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("_tmp.timestamp") && !event.has_value("event.timezone") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["dd-MMM-yyyy HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSS'Z'"], None, None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
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
                event.set("_ingest.on_failure_processor_tag", "date_tmp_timestamp_notz")?;
                        event.remove("_tmp.timestamp");
                        event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("repeat_message") };
            if _cond {
                // Painless script
                // Source: def splitUnquoted(String input, String sep) {\n  def tokens = [];\n  def startPosition = 0;\n  def isInQuotes = false;\n  char quote = (char)\"\\\"\";\n  for (def currentPosition = 0; currentPosition < input.length(); currentPosition++) {\n      if (input.charAt(currentPosition) == quote) {\n          isInQuotes = !isInQuotes;\n      }\n      else if (input.charAt(currentPosition) == (char)sep && !isInQuotes) {\n          def token = input.substring(startPosition, currentPosition).trim();\n          if (!token.equals(\"\")) {\n            tokens.add(token);\n          }\n          startPosition = currentPosition + 1;\n      }\n  }\n\n  def lastToken = input.substring(startPosition);\n  if (!lastToken.equals(sep) && !lastToken.equals(\"\")) {\n      tokens.add(lastToken.trim());\n  }\n  return tokens;\n}\n\ndef arr = splitUnquoted(ctx.repeat_message, \";\");\nctx.repeat_message = arr;\nMap map = new HashMap();\nmap.put('name', new ArrayList());\nmap.put('ttl', new ArrayList());\nmap.put('class', new ArrayList());\nmap.put('type', new ArrayList());\nmap.put('data', new ArrayList());\n\nfor (def i = 0; i < arr.length; i++) {\n  def response = splitUnquoted(arr[i], \" \");\n  if (response.size() >= 4) {\n    map['name'].add(response[0]);\n    map['ttl'].add(response[1]);\n    map['class'].add(response[2]);\n    map['type'].add(response[3]);\n    map['data'].addAll(response.subList(4, response.length));\n  }\n}\nctx.dns.answers = map;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def splitUnquoted(String input, String sep) {\n  def tokens = [];\n  def startPosition = 0;\n  def isInQuotes = false;\n  char quote = (char)\"\\\"\";\n  for (def currentPosition = 0; currentPosition < input.length(); currentPosition++) {\n      if (input.charAt(currentPosition) == quote) {\n          isInQuotes = !isInQuotes;\n      }\n      else if (input.charAt(currentPosition) == (char)sep && !isInQuotes) {\n          def token = input.substring(startPosition, currentPosition).trim();\n          if (!token.equals(\"\")) {\n            tokens.add(token);\n          }\n          startPosition = currentPosition + 1;\n      }\n  }\n\n  def lastToken = input.substring(startPosition);\n  if (!lastToken.equals(sep) && !lastToken.equals(\"\")) {\n      tokens.add(lastToken.trim());\n  }\n  return tokens;\n}\n\ndef arr = splitUnquoted(ctx.repeat_message, \";\");\nctx.repeat_message = arr;\nMap map = new HashMap();\nmap.put('name', new ArrayList());\nmap.put('ttl', new ArrayList());\nmap.put('class', new ArrayList());\nmap.put('type', new ArrayList());\nmap.put('data', new ArrayList());\n\nfor (def i = 0; i < arr.length; i++) {\n  def response = splitUnquoted(arr[i], \" \");\n  if (response.size() >= 4) {\n    map['name'].add(response[0]);\n    map['ttl'].add(response[1]);\n    map['class'].add(response[2]);\n    map['type'].add(response[3]);\n    map['data'].addAll(response.subList(4, response.length));\n  }\n}\nctx.dns.answers = map;\n"#))?;
            }

            let _cond = { event.has_value("infoblox_nios.log.dns.message") };
            if _cond {
                gsub_field(event, "infoblox_nios.log.dns.message", "infoblox_nios.log.dns.message", cached_regex!("\""), "")?;
            }

            let _cond = { event.has_value("infoblox_nios.log.dns.message") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("infoblox_nios.log.dns.message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("rpz ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("infoblox_nios.log.dns.rpz.rule_type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("infoblox_nios.log.dns.rpz.query_class", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("infoblox_nios.log.dns.rpz.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else { break 'dissect false };
                        captured.push(("infoblox_nios.log.dns.rpz.domain", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" via ") else { break 'dissect false };
                        captured.push(("infoblox_nios.log.dns.rpz.query_class_rewrite", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" via ") else { break 'dissect false };
                        remaining = rest;
                        let Some(pos) = remaining.find(" CAT=") else { break 'dissect false };
                        captured.push(("infoblox_nios.log.dns.rpz.domain_rewrite", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" CAT=") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("infoblox_nios.log.dns.rpz.type", remaining));
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
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("dns.answers.ttl") {
                if let Some(val) = event.get("dns.answers.ttl") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "dns.answers.ttl".into(),
                            message,
                        })?;
                    event.set("dns.answers.ttl", converted)?;
                }
            }
                Ok(())
            })();

            let _cond = { event.get("dns.answers.data").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def hash = new ArrayList();\nfor(data in ctx.dns.answers.data){\n  def n = data.length();\n  if(data.charAt(n-1).toString() == '.'){\n    def data_substring = data.substring(0,n-1) + data.substring(n);\n    hash.add(data_substring);\n  }\n  else{\n    hash.add(data);\n  }\n}\nctx.dns.answers.data = hash;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def hash = new ArrayList();\nfor(data in ctx.dns.answers.data){\n  def n = data.length();\n  if(data.charAt(n-1).toString() == '.'){\n    def data_substring = data.substring(0,n-1) + data.substring(n);\n    hash.add(data_substring);\n  }\n  else{\n    hash.add(data);\n  }\n}\nctx.dns.answers.data = hash;\n"#))?;
            }

            let _cond = { event.get("dns.answers.name").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def hash = new ArrayList();\nfor(name in ctx.dns.answers.name){\n  def n = name.length();\n  if(name.charAt(n-1).toString() == '.'){\n    def name_substring = name.substring(0,n-1) + name.substring(n);\n    hash.add(name_substring);\n  }\n  else{\n    hash.add(name);\n  }\n}\nctx.dns.answers.name = hash;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def hash = new ArrayList();\nfor(name in ctx.dns.answers.name){\n  def n = name.length();\n  if(name.charAt(n-1).toString() == '.'){\n    def name_substring = name.substring(0,n-1) + name.substring(n);\n    hash.add(name_substring);\n  }\n  else{\n    hash.add(name);\n  }\n}\nctx.dns.answers.name = hash;\n"#))?;
            }

            let _cond = { event.has_value("dns.answers.data") };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("dns.answers.data").cloned();
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
                            if let Some(input) = event.get_string("_ingest._value") {
                            // Grok pattern: ^%{IP:related.ip}$
                            // Grok pattern: ^%{HOSTNAME:related.hosts}$
                            if !extract_first_match(
                            &[
                            cached_grok!("^%{IP:related.ip}$"),
                            cached_grok!("^%{HOSTNAME:related.hosts}$"),
                            ],
                            &input,
                            event,
                            )? {
                            return Err(TransformError::GrokNoMatch { value: input });
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
                        event.set("dns.answers.data", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                    }
                }
            }

            let _cond = { event.has_value("client.ip") && event.get_str("client.ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("client.ip") {
                if let Some(val) = event.get("client.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "client.ip".into(),
                            message,
                        })?;
                    event.set("client.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("client.ip");
                        event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("server.ip") && event.get_str("server.ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("server.ip") {
                if let Some(val) = event.get("server.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "server.ip".into(),
                            message,
                        })?;
                    event.set("server.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("server.ip");
                        event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("server.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("server.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("dns.answers.name") };
            if _cond {
                foreach_array(event, "dns.answers.name", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                    event.append_unique("related.hosts", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("dns.question.name") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.hosts", json!(event.get("dns.question.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("infoblox_nios.log.dns.header_flags") && event.get_str("infoblox_nios.log.dns.header_flags") != Some("") };
            if _cond {
                // Painless script
                // Source: ArrayList hf = new ArrayList();\nfor (entry in params.entrySet()) {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains(entry.getKey())) {\n    hf.add(entry.getValue());\n  }\n}\nif (ctx.dns?.response_code != null && ctx.dns.response_code != '') {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) {\n    hf.add('RA')\n  }\n} else {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) {\n    hf.add('RD')\n  }\n}\nif (hf.length == 0) {\n  return;\n}\nif (ctx.dns == null) {\n  HashMap hm = new HashMap();\n  ctx.put('dns', hm);\n}\nctx.dns.put('header_flags', hf);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"ArrayList hf = new ArrayList();\nfor (entry in params.entrySet()) {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains(entry.getKey())) {\n    hf.add(entry.getValue());\n  }\n}\nif (ctx.dns?.response_code != null && ctx.dns.response_code != '') {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) {\n    hf.add('RA')\n  }\n} else {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) {\n    hf.add('RD')\n  }\n}\nif (hf.length == 0) {\n  return;\n}\nif (ctx.dns == null) {\n  HashMap hm = new HashMap();\n  ctx.put('dns', hm);\n}\nctx.dns.put('header_flags', hf);\n"#), cached_params!("{\"A\":\"AA\",\"t\":\"TC\",\"C\":\"CD\",\"D\":\"DO\"}"))?;
            }

            let _cond = { event.has_value("dns.question") };
            if _cond {
                if let Some(domain) = event.get_string("dns.question.name") {
                    // Public suffix list lookup for registered domain extraction.
                    // A failed lookup writes NO target field, which is what
                    // Elasticsearch does.
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        event.set("dns.question.domain", json!(domain))?;
                        if let Some(registered) = rd.registered_domain {
                            event.set("dns.question.registered_domain", json!(registered))?;
                        }
                        event.set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("dns.question.subdomain", json!(sub))?;
                        }
                    }
                }
            }

                event.remove("repeat_message");
                event.remove("dns.question.domain");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
