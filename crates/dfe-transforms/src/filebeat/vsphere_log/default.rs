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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?%{TIMESTAMP_ISO8601:_tmp.timestamp}%{SPACE}(?:(?:%{IP:host.ip}|%{HOSTNAME:host.name}))%{SPACE}%{NOTSPACE:process.name} (%{POSINT:process.pid:long}|-) - -%{SPACE}%{GREEDYDATA:message}
                // Grok pattern: ^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?%{TIMESTAMP_ISO8601:_tmp.timestamp}%{SPACE}(?:(?:%{IP:host.ip}|%{HOSTNAME:host.name}))%{SPACE}%{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?\\: %{GREEDYDATA:message}
                // Grok pattern: ^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?%{TIMESTAMP_ISO8601:_tmp.timestamp}%{SPACE}(?:(?:%{IP:host.ip}|%{HOSTNAME:host.name}))%{SPACE}%{NOTSPACE:process.name}%{SPACE}(%{POSINT:process.pid:long}|-)( -)?%{SPACE}%{GREEDYDATA:message}
                // Grok pattern: ^ \\(%{TIMESTAMP_ISO8601:_tmp.timestamp} %{GREEDYDATA:message}\\)%{GREEDYDATA:_tmp.drop}
                // Grok pattern: ^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?%{SYSLOGTIMESTAMP:_tmp.timestamp}%{SPACE}(?:(?:%{IP:host.ip}|%{HOSTNAME:host.name}))%{SPACE}%{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?\\: %{GREEDYDATA:message}
                if !extract_first_match(
                    &[
                        cached_grok!(
                            "^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?%{TIMESTAMP_ISO8601:_tmp.timestamp}%{SPACE}(?:(?:%{IP:host.ip}|%{HOSTNAME:host.name}))%{SPACE}%{NOTSPACE:process.name} (%{POSINT:process.pid:long}|-) - -%{SPACE}%{GREEDYDATA:message}"
                        ),
                        cached_grok!(
                            "^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?%{TIMESTAMP_ISO8601:_tmp.timestamp}%{SPACE}(?:(?:%{IP:host.ip}|%{HOSTNAME:host.name}))%{SPACE}%{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?\\: %{GREEDYDATA:message}"
                        ),
                        cached_grok!(
                            "^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?%{TIMESTAMP_ISO8601:_tmp.timestamp}%{SPACE}(?:(?:%{IP:host.ip}|%{HOSTNAME:host.name}))%{SPACE}%{NOTSPACE:process.name}%{SPACE}(%{POSINT:process.pid:long}|-)( -)?%{SPACE}%{GREEDYDATA:message}"
                        ),
                        cached_grok!(
                            "^ \\(%{TIMESTAMP_ISO8601:_tmp.timestamp} %{GREEDYDATA:message}\\)%{GREEDYDATA:_tmp.drop}"
                        ),
                        cached_grok!(
                            "^((?:<%{NONNEGINT:log.syslog.priority:long}>(\\d )?))?%{SYSLOGTIMESTAMP:_tmp.timestamp}%{SPACE}(?:(?:%{IP:host.ip}|%{HOSTNAME:host.name}))%{SPACE}%{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?\\: %{GREEDYDATA:message}"
                        ),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            // Painless script
            // Source: if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n"#
                ),
            )?;

            // Painless script
            // Source: if (ctx.log?.syslog?.facility?.code == null || !params.containsKey((ctx.log.syslog.facility.code).toString())) {\n  return;\n}\nctx.log.syslog.facility.name = params[(ctx.log.syslog.facility.code).toString()];
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.facility?.code == null || !params.containsKey((ctx.log.syslog.facility.code).toString())) {\n  return;\n}\nctx.log.syslog.facility.name = params[(ctx.log.syslog.facility.code).toString()];"#
                ),
                cached_params!(
                    "{\"0\":\"Kernel\",\"1\":\"User\",\"2\":\"Mail\",\"3\":\"System\",\"4\":\"Security\",\"5\":\"Syslog\",\"6\":\"Line printer\",\"7\":\"Network news\",\"8\":\"UUCP\",\"9\":\"Clock\",\"10\":\"Security\",\"11\":\"FTPd\",\"12\":\"NTPd\",\"13\":\"Log audit\",\"14\":\"Log alert\",\"15\":\"Clock daemon\",\"16\":\"Local 0\",\"17\":\"Local 1\",\"18\":\"Local 2\",\"19\":\"Local 3\",\"20\":\"Local 4\",\"21\":\"Local 5\",\"22\":\"Local 6\",\"23\":\"Local 7\"}"
                ),
            )?;

            // Painless script
            // Source: if (ctx.log?.syslog?.severity?.code == null || !params.containsKey((ctx.log.syslog.severity.code).toString())) {\n  return;\n}\nctx.log.syslog.severity.name = params[(ctx.log.syslog.severity.code).toString()];
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.severity?.code == null || !params.containsKey((ctx.log.syslog.severity.code).toString())) {\n  return;\n}\nctx.log.syslog.severity.name = params[(ctx.log.syslog.severity.code).toString()];"#
                ),
                cached_params!(
                    "{\"0\":\"Emergency\",\"1\":\"Alert\",\"2\":\"Critical\",\"3\":\"Error\",\"4\":\"Warning\",\"5\":\"Notice\",\"6\":\"Informational\",\"7\":\"Debug\"}"
                ),
            )?;

            if let Some(input) = event.get_string("message") {
                // Grok pattern: ^(?:%{TIMESTAMP_ISO8601}(( '%{NOTSPACE:event.action}' %{POSINT})|:) %{LOGLEVEL:log.level} %{GREEDYDATA:message})
                // Grok pattern: ^(?:Event \\[%{POSINT:event.id}\\] %{NOTSPACE} \\[%{TIMESTAMP_ISO8601}\\] \\[%{JAVACLASS:log.logger}\\] \\[%{LOGLEVEL:log.level}\\] %{GREEDYDATA:message})
                // Grok pattern: ^%{GREEDYDATA:message}
                if !extract_first_match(
                    &[
                        cached_grok!(
                            "^(?:%{TIMESTAMP_ISO8601}(( '%{NOTSPACE:event.action}' %{POSINT})|:) %{LOGLEVEL:log.level} %{GREEDYDATA:message})"
                        ),
                        cached_grok!(
                            "^(?:Event \\[%{POSINT:event.id}\\] %{NOTSPACE} \\[%{TIMESTAMP_ISO8601}\\] \\[%{JAVACLASS:log.logger}\\] \\[%{LOGLEVEL:log.level}\\] %{GREEDYDATA:message})"
                        ),
                        cached_grok!("^%{GREEDYDATA:message}"),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                match parse_date_out(
                    &date_str,
                    &[
                        "ISO8601",
                        "yyyy-MM-dd HH:mm:ss.SSS",
                        "yyyy-MM-dd HH:mm:ss.SSSXXX",
                        "yyyy-MM-dd HH:mm:ss.SSSZ",
                        "yyyy-MM-dd HH:mm:ss",
                        "yyyy-MM-dd HH:mm:ssXXX",
                        "yyyy-MM-dd HH:mm:ssZ",
                        "MMM dd HH:mm:ss",
                    ],
                    None,
                    None,
                ) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "_tmp.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            let _cond = { event.get_str("process.name") == Some("dnsmasq") };
            if _cond {
                // Begin nested pipeline: "dns"
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.starts_with("query["))
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("[") else {
                                break 'dissect false;
                            };
                            captured.push(("dns.op_code", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("[") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("] ") else {
                                break 'dissect false;
                            };
                            captured.push(("dns.question.type", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("] ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" from ") else {
                                break 'dissect false;
                            };
                            captured.push(("dns.question.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" from ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.starts_with("cached"))
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("cached ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" is ") else {
                                break 'dissect false;
                            };
                            captured.push(("dns.question.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" is ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_tmp.dns.resolved_ip", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.starts_with("forwarded"))
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("forwarded ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" to ") else {
                                break 'dissect false;
                            };
                            captured.push(("dns.question.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" to ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("_tmp.dns.resolved_ip", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = { event.has_value("dns.question.name") };
                if _cond {
                    if let Some(domain_str) = event.get_string("dns.question.name") {
                        let domain = domain_str.to_string();
                        event.set("dns.question.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            if let Some(registered) = rd.registered_domain {
                                event.set("dns.question.registered_domain", json!(registered))?;
                            }
                            event
                                .set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                            if let Some(sub) = rd.subdomain {
                                event.set("dns.question.subdomain", json!(sub))?;
                            }
                        }
                    }
                }
                let _cond = { event.has_value("dns.op_code") };
                if _cond {
                    map_strings(event, "dns.op_code", "dns.op_code", str::to_uppercase)?;
                }
                let _cond = { event.has_value("_tmp.dns.resolved_ip") };
                if _cond {
                    event.append(
                        "dns.resolved_ip",
                        json!(
                            event
                                .get("_tmp.dns.resolved_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("dns.resolved_ip") };
                if _cond {
                    event.set(
                        "dns.answers.data",
                        json!(
                            event
                                .get("dns.resolved_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond =
                    { event.has_value("dns.question.name") && event.has_value("dns.resolved_ip") };
                if _cond {
                    event.set(
                        "dns.answers.name",
                        json!(
                            event
                                .get("dns.question.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.append("event.category", json!("network"))?;
                event.append("event.type", json!("protocol"))?;
                event.append("event.type", json!("connection"))?;
                let _cond = { event.has_value("dns.question.domain") };
                if _cond {
                    if event.remove("dns.question.domain").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "dns.question.domain".into(),
                        });
                    }
                }
                // End nested pipeline: "dns"
            }

            let _cond = {
                (event.has_value("message")
                    && (event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("logged"))
                        }
                        serde_json::Value::String(s) => s.contains("logged"),
                        _ => false,
                    }) || event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("login"))))
                    || (event.has_value("log.logger")
                        && [
                            "vim.event.UserLogoutSessionEvent",
                            "vim.event.UserLogoutSessionEvent",
                        ]
                        .contains(&event.get_str("log.logger").unwrap_or("")))
                    || event.get_str("process.name") == Some("sshd")
            };
            if _cond {
                // Begin nested pipeline: "login"
                // SKIPPED: condition not transpiled: ctx.message?.contains('Authenticated user') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" user ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" user ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = {
                    event.has_value("message")
                        && (event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("logged in as"))
                            }
                            serde_json::Value::String(s) => s.contains("logged in as"),
                            _ => false,
                        }) && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("User "))
                            }
                            serde_json::Value::String(s) => s.contains("User "),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("User ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("User ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("client.ip", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" as ") else {
                                break 'dissect false;
                            };
                            captured.push(("_tmp.event", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" as ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user_agent.original", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = {
                    event.has_value("message")
                        && (event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("logged in as"))
                            }
                            serde_json::Value::String(s) => s.contains("logged in as"),
                            _ => false,
                        }) && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Shared secret"))
                            }
                            serde_json::Value::String(s) => s.contains("Shared secret"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" Shared secret from ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" Shared secret from ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("client.ip", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" as ") else {
                                break 'dissect false;
                            };
                            captured.push(("_tmp.event", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" as ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user_agent.original", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = {
                    event.has_value("message")
                        && (event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("logged in as"))
                            }
                            serde_json::Value::String(s) => s.contains("logged in as"),
                            _ => false,
                        }) && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("SSL thumbprint"))
                            }
                            serde_json::Value::String(s) => s.contains("SSL thumbprint"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" SSL thumbprint logged in as ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) =
                                remaining.strip_prefix(" SSL thumbprint logged in as ")
                            else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user_agent.original", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = {
                    event.has_value("message")
                        && (event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("logged in as"))
                            }
                            serde_json::Value::String(s) => s.contains("logged in as"),
                            _ => false,
                        }) && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Shared secret"))
                            }
                            serde_json::Value::String(s) => s.contains("Shared secret"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.set("user.name", json!("shared_secret_login"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && (event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("logged in as"))
                            }
                            serde_json::Value::String(s) => s.contains("logged in as"),
                            _ => false,
                        }) && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("SSL thumbprint"))
                            }
                            serde_json::Value::String(s) => s.contains("SSL thumbprint"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.set("user.name", json!("ssl_thumbprint_login"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && (event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("logged in as"))
                            }
                            serde_json::Value::String(s) => s.contains("logged in as"),
                            _ => false,
                        }) && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("Shared secret"))
                            }
                            serde_json::Value::String(s) => s.contains("Shared secret"),
                            _ => false,
                        })) && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("User "))
                            }
                            serde_json::Value::String(s) => s.contains("User "),
                            _ => false,
                        })))
                };
                if _cond {
                    event.set("_tmp.event", json!("logged in"))?;
                }
                // SKIPPED: condition not transpiled: ctx.message?.contains('logged out') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("User ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("User ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("client.ip", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" out (login time: ") else {
                                    break 'dissect false;
                                };
                                captured.push(("_tmp.event", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" out (login time: ")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(", number ") else {
                                    break 'dissect false;
                                };
                                captured.push(("event.start", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(", number ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(", ") else {
                                    break 'dissect false;
                                };
                                captured.push(("vsphere.log.api.invocations", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(", ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(": ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(": ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user_agent.original", remaining));
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
                // SKIPPED: condition not transpiled: ctx.message?.contains('logged out') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("User {Name: ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("User {Name: ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(", Domain: ") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(", Domain: ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("} ") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.domain", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("} ") else {
                                    break 'dissect false;
                                };
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
                }
                // SKIPPED: condition not transpiled: ctx.message?.contains('Failed login') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("[") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("] [] [") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("] [] [") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("] [") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("] [") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" login ") else {
                                    break 'dissect false;
                                };
                                captured.push(("event.outcome", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" login ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" from ") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" from ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push(("client.ip", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
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
                }
                // SKIPPED: condition not transpiled: ctx.message?.startsWith('Received') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("Received ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" from ") else {
                                break 'dissect false;
                            };
                            captured.push(("_tmp.status", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" from ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" port ") else {
                                break 'dissect false;
                            };
                            captured.push(("client.ip", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" port ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(":") else {
                                break 'dissect false;
                            };
                            captured.push(("client.port", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(":") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(": ") else {
                                break 'dissect false;
                            };
                            captured.push(("destination.port", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(": ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                // SKIPPED: condition not transpiled: ctx.message?.startsWith('Disconnected') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" from ") else {
                                break 'dissect false;
                            };
                            captured.push(("_tmp.status", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" from ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" port ") else {
                                break 'dissect false;
                            };
                            captured.push(("client.ip", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" port ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("client.port", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                // SKIPPED: condition not transpiled: ctx.message?.startsWith('Connection') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: Connection %{DATA:_tmp.status} by %{IPORHOST:client.ip} port %{POSINT:client.port}( %{GREEDYDATA})?$
                        if !cached_grok!("Connection %{DATA:_tmp.status} by %{IPORHOST:client.ip} port %{POSINT:client.port}( %{GREEDYDATA})?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                // SKIPPED: condition not transpiled: ctx.message?.contains('Logged in user:') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find(" Logged in user: \"") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" Logged in user: \"")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("\\") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.domain", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("\\") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("\"") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("\"") else {
                                    break 'dissect false;
                                };
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
                }
                // SKIPPED: condition not transpiled: ctx?.message?.contains('logged in successfully') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find(" User {Name: ") else {
                                    break 'dissect false;
                                };
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" User {Name: ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(", Domain: ") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(", Domain: ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find("} ") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.domain", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("} ") else {
                                    break 'dissect false;
                                };
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
                }
                // SKIPPED: condition not transpiled: ctx.user?.name?.contains('\\') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    if let Some(input) = event.get_string("user.name") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("\\") else {
                                break 'dissect false;
                            };
                            captured.push(("user.domain", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.name", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "user.name".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = {
                    !event.has_value("event.outcome")
                        && (event.has_value("user.name") || event.has_value("user.domain"))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                // SKIPPED: condition not transpiled: ctx.event?.outcome?.toLowerCase()?.startsWith('f') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    event.set("event.outcome", json!("failure"))?;
                }
                // SKIPPED: condition not transpiled: ctx.user_agent?.original != null && (ctx.user_agent.original.contains(')') || ctx.user_agent.original.contains(']'))
                #[allow(unreachable_code, unused_variables)]
                if false {
                    if let Some(input) = event.get_string("user_agent.original") {
                        // Grok pattern: %{DATA:user_agent.original}(?:\\]|\\)+)
                        if !cached_grok!("%{DATA:user_agent.original}(?:\\]|\\)+)")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = { event.has_value("user_agent.original") };
                if _cond {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
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
                let _cond = { event.has_value("event.start") };
                if _cond {
                    event.set(
                        "event.end",
                        json!(
                            event
                                .get("@timestamp")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("event.start") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("event.start") {
                        match parse_date_out(
                            &date_str,
                            &["EEEE, dd MMMM, yyyy hh:mm:ss a"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("event.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = { event.has_value("event.end") && event.has_value("event.start") };
                if _cond {
                    // Painless script
                    // Source: ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\n ZonedDateTime end = ZonedDateTime.parse(ctx.event.end);\n ctx.event.duration= ChronoUnit.NANOS.between(start, end);
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\n ZonedDateTime end = ZonedDateTime.parse(ctx.event.end);\n ctx.event.duration= ChronoUnit.NANOS.between(start, end);"#
                        ),
                    )?;
                }
                // SKIPPED: condition not transpiled: ctx.message?.toLowerCase().contains('logged in') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    event.set("event.action", json!("login"))?;
                }
                // SKIPPED: condition not transpiled: ctx.message?.toLowerCase().contains('logged out') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    event.set("event.action", json!("logout"))?;
                }
                event.append("event.type", json!("info"))?;
                event.append("event.category", json!("authentication"))?;
                let _cond = { event.has_value("client.port") };
                if _cond {
                    if let Some(val) = event.get("client.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "client.port".into(),
                                message,
                            }
                        })?;
                        event.set("client.port", converted)?;
                    }
                }
                let _cond = { event.has_value("destination.port") };
                if _cond {
                    if let Some(val) = event.get("destination.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.port".into(),
                                message,
                            }
                        })?;
                        event.set("destination.port", converted)?;
                    }
                }
                let _cond = {
                    event
                        .get("vsphere.log.api.invocations")
                        .is_some_and(|v| v.is_string())
                        && event
                            .get("vsphere.log.api.invocations")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    gsub_field(
                        event,
                        "vsphere.log.api.invocations",
                        "vsphere.log.api.invocations",
                        cached_regex!(",|\\."),
                        "",
                    )?;
                }
                let _cond = { event.has_value("vsphere.log.api.invocations") };
                if _cond {
                    if let Some(val) = event.get("vsphere.log.api.invocations") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "vsphere.log.api.invocations".into(),
                                message,
                            }
                        })?;
                        event.set("vsphere.log.api.invocations", converted)?;
                    }
                }
                let _cond = { event.has_value("client.ip") };
                if _cond {
                    if let Some(v) = event.get("client.ip").cloned() {
                        event.set("source.ip", v)?;
                    }
                }
                let _cond = { event.has_value("client.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "login"
            }

            // SKIPPED: condition not transpiled: ctx.message?.contains("'Upload' for path") ?: false
            #[allow(unreachable_code, unused_variables)]
            if false {
                // Begin nested pipeline: "file"
                // SKIPPED: condition not transpiled: ctx.message.contains('Upload') ?: false
                #[allow(unreachable_code, unused_variables)]
                if false {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find(" 'Upload' for path '") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" 'Upload' for path '") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("' ") else {
                                break 'dissect false;
                            };
                            captured.push(("vsphere.log.file.path", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("' ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" '") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" '") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("' ") else {
                                break 'dissect false;
                            };
                            captured.push(("client.ip", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("' ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" '") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" '") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("'") else {
                                break 'dissect false;
                            };
                            captured.push(("event.outcome", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("'") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "message".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                let _cond = { event.has_value("event.outcome") };
                if _cond {
                    map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
                }
                event.append("event.category", json!("file"))?;
                event.append("event.type", json!("creation"))?;
                let _cond = { event.has_value("client.ip") };
                if _cond {
                    if let Some(v) = event.get("client.ip").cloned() {
                        event.set("source.ip", v)?;
                    }
                }
                let _cond = { event.has_value("client.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "file"
            }

            event.set("event.kind", json!("event"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("_tmp").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_tmp".into(),
                    });
                }
                Ok(())
            })();

            // Painless script
            // Source: void handleMap(Map map) {\n                for (def x : map.values()) {\n                  if (x instanceof Map) {\n                    handleMap(x);\n                  } else if (x instanceof List) {\n                    handleList(x);\n                  }\n                }\n                map.values().removeIf(v -> v == null);\n              }\n              void handleList(List list) {\n                for (def x : list) {\n                  if (x instanceof Map) {\n                    handleMap(x);\n                  } else if (x instanceof List) {\n                    handleList(x);\n                  }\n                }\n              }\n              handleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n                for (def x : map.values()) {\n                  if (x instanceof Map) {\n                    handleMap(x);\n                  } else if (x instanceof List) {\n                    handleList(x);\n                  }\n                }\n                map.values().removeIf(v -> v == null);\n              }\n              void handleList(List list) {\n                for (def x : list) {\n                  if (x instanceof Map) {\n                    handleMap(x);\n                  } else if (x instanceof List) {\n                    handleList(x);\n                  }\n                }\n              }\n              handleMap(ctx);\n              "#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("_tmp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_tmp".into(),
                        });
                    }
                    Ok(())
                })();
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
