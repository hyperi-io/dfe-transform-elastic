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

            let _cond = { event.has_value("log.file.path") };
            if _cond {
                event.set(
                    "_tmp.filepath",
                    json!(
                        event
                            .get("log.file.path")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("log.file.path") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_tmp.filepath") {
                        // Grok pattern: ^%{DATA}[\\\\/]%{WORD:cisco_secure_email_gateway.log.category.name}(?:\\.%{HOSTNAME:cisco_secure_email_gateway.log.host})?\\.@%{GREEDYDATA}\\.s$
                        let _ = cached_grok!("^%{DATA}[\\\\/]%{WORD:cisco_secure_email_gateway.log.category.name}(?:\\.%{HOSTNAME:cisco_secure_email_gateway.log.host})?\\.@%{GREEDYDATA}\\.s$").extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:%{HOSTNAME:cisco_secure_email_gateway.log.host} )?%{NOTSPACE:cisco_secure_email_gateway.log.category.name}: %{WORD:log.level}: %{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                // Grok pattern: ^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:%{HOSTNAME:cisco_secure_email_gateway.log.host} )?%{NOTSPACE:cisco_secure_email_gateway.log.category.name}: %{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                // Grok pattern: ^%{DATA:_tmp.timestamp} %{WORD:log.level}: %{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                let _ = extract_first_match(
                    &[
                        cached_grok!(
                            "^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:%{HOSTNAME:cisco_secure_email_gateway.log.host} )?%{NOTSPACE:cisco_secure_email_gateway.log.category.name}: %{WORD:log.level}: %{GREEDYDATA:cisco_secure_email_gateway.log.message}$"
                        ),
                        cached_grok!(
                            "^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP:_tmp.timestamp} (?:%{HOSTNAME:cisco_secure_email_gateway.log.host} )?%{NOTSPACE:cisco_secure_email_gateway.log.category.name}: %{GREEDYDATA:cisco_secure_email_gateway.log.message}$"
                        ),
                        cached_grok!(
                            "^%{DATA:_tmp.timestamp} %{WORD:log.level}: %{GREEDYDATA:cisco_secure_email_gateway.log.message}$"
                        ),
                        cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                    ],
                    &input,
                    event,
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(
                    event,
                    "cisco_secure_email_gateway.log.message",
                    "cisco_secure_email_gateway.log.message",
                    |s| s.trim().to_string(),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "log.level", "log.level", str::to_lowercase)?;
                Ok(())
            })();

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                if let Some(v) = event.get("_conf.tz_offset").cloned() {
                    event.set("_tmp.tz", v)?;
                }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(v) = event.get("event.timezone").cloned() {
                    if !event.has("_tmp.tz") {
                        event.set("_tmp.tz", v)?;
                    }
                }
            }

            if !event.has("_tmp.tz") {
                event.set("_tmp.tz", json!("UTC"))?;
            }

            if let Some(v) = event.get("_tmp.tz").cloned() {
                event.set("event.timezone", v)?;
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "MMM  d HH:mm:ss",
                                "MMM dd HH:mm:ss",
                                "MMM d HH:mm:ss",
                            ],
                            event.get_str("event.timezone"),
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
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date__tmp_timestamp_11c130e5",
                    )?;
                    event.remove("event.timezone");
                    let _cond = { event.has_value("_tmp.timestamp") };
                    if _cond {
                        if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "E MMM dd HH:mm:ss yyyy",
                                    "E MMM  d HH:mm:ss yyyy",
                                    "E MMM d HH:mm:ss yyyy",
                                    "MMM  d HH:mm:ss",
                                    "MMM dd HH:mm:ss",
                                    "MMM d HH:mm:ss",
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
                event.get_str("cisco_secure_email_gateway.log.category.name")
                    == Some("authentication")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_authentication"
                event.set("event.kind", json!("event"))?;
                event.set(
                    "event.category",
                    Value::Array(vec![json!("authentication")]),
                )?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^GUI: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from session %{GREEDYDATA:cisco_secure_email_gateway.log.session} because of inactivity timeout$
                    // Grok pattern: ^CLI: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from %{GREEDYDATA:cisco_secure_email_gateway.log.session} because of inactivity timeout$
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.action}:%{IP:host.ip} user:%{USERNAME:user.name} session:%{WORD:cisco_secure_email_gateway.log.session}$
                    // Grok pattern: ^User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} of %{WORD:network.protocol} session %{IP:host.ip}$
                    // Grok pattern: ^An authentication attempt by the user %{USERNAME:user.name} from %{IP:host.ip} %{WORD:cisco_secure_email_gateway.log.outcome} using an %{WORD:network.protocol} connection\\.$
                    // Grok pattern: ^The user %{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.outcome} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from %{IP:host.ip} with privilege %{DATA:cisco_secure_email_gateway.log.privilege} using an %{WORD:network.protocol} connection\\.$
                    // Grok pattern: ^User %{USERNAME:user.name} was %{WORD:cisco_secure_email_gateway.log.action} %{WORD:cisco_secure_email_gateway.log.outcome}\\.$
                    // Grok pattern: ^User %{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.outcome} %{WORD:cisco_secure_email_gateway.log.action}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^GUI: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from session %{GREEDYDATA:cisco_secure_email_gateway.log.session} because of inactivity timeout$"
                            ),
                            cached_grok!(
                                "^CLI: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from %{GREEDYDATA:cisco_secure_email_gateway.log.session} because of inactivity timeout$"
                            ),
                            cached_grok!(
                                "^%{WORD:cisco_secure_email_gateway.log.action}:%{IP:host.ip} user:%{USERNAME:user.name} session:%{WORD:cisco_secure_email_gateway.log.session}$"
                            ),
                            cached_grok!(
                                "^User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} of %{WORD:network.protocol} session %{IP:host.ip}$"
                            ),
                            cached_grok!(
                                "^An authentication attempt by the user %{USERNAME:user.name} from %{IP:host.ip} %{WORD:cisco_secure_email_gateway.log.outcome} using an %{WORD:network.protocol} connection\\.$"
                            ),
                            cached_grok!(
                                "^The user %{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.outcome} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from %{IP:host.ip} with privilege %{DATA:cisco_secure_email_gateway.log.privilege} using an %{WORD:network.protocol} connection\\.$"
                            ),
                            cached_grok!(
                                "^User %{USERNAME:user.name} was %{WORD:cisco_secure_email_gateway.log.action} %{WORD:cisco_secure_email_gateway.log.outcome}\\.$"
                            ),
                            cached_grok!(
                                "^User %{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.outcome} %{WORD:cisco_secure_email_gateway.log.action}$"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                    Ok(())
                })();
                let _cond =
                    { event.get_str("cisco_secure_email_gateway.log.outcome") == Some("failed") };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("cisco_secure_email_gateway.log.outcome") == Some("successfully")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("cisco_secure_email_gateway.log.action") == Some("logged on")
                        || event.get_str("cisco_secure_email_gateway.log.action")
                            == Some("authenticated")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = {
                    event.get_str("cisco_secure_email_gateway.log.action") == Some("logged out")
                        || event.get_str("cisco_secure_email_gateway.log.action") == Some("logout")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("host.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("host.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_authentication"
            }

            let _cond = {
                event.get_str("cisco_secure_email_gateway.log.category.name") == Some("gui_logs")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_gui_logs"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^req:%{DATA:client.ip:IP} user:%{DATA:user.name} id:%{DATA:event.id} %{NUMBER:http.response.status_code:long} %{WORD:http.request.method} %{DATA:url.path} HTTP/%{NUMBER:http.version} %{GREEDYDATA:user_agent.original}$
                    // Grok pattern: ^Action: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from session %{GREEDYDATA:cisco_secure_email_gateway.log.session} beacuse of inactivity timeout$
                    // Grok pattern: ^Session %{DATA:cisco_secure_email_gateway.log.session} user:%{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.result}$
                    // Grok pattern: ^Session %{DATA:cisco_secure_email_gateway.log.session} from %{IP:host.ip} not found Destination:%{GREEDYDATA:cisco_secure_email_gateway.log.destination}$
                    // Grok pattern: ^SourceIP:%{IP:host.ip} Destination:%{GREEDYDATA:cisco_secure_email_gateway.log.destination} Username:%{USERNAME:user.name} Privilege:%{DATA:cisco_secure_email_gateway.log.privilege} session:%{DATA:cisco_secure_email_gateway.log.session} Action: %{GREEDYDATA:cisco_secure_email_gateway.log.action}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} %{IP:source.ip}:%{NUMBER:source.port:long} - \\(%{GREEDYDATA:cisco_secure_email_gateway.log.description}\\)$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} %{IP:source.ip} port %{NUMBER:source.port:long} - %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^%{DATA:cisco_secure_email_gateway.log.object} has been %{DATA:cisco_secure_email_gateway.log.action} for user %{USERNAME:user.name}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^req:%{DATA:client.ip:IP} user:%{DATA:user.name} id:%{DATA:event.id} %{NUMBER:http.response.status_code:long} %{WORD:http.request.method} %{DATA:url.path} HTTP/%{NUMBER:http.version} %{GREEDYDATA:user_agent.original}$"
                            ),
                            cached_grok!(
                                "^Action: User %{USERNAME:user.name} %{GREEDYDATA:cisco_secure_email_gateway.log.action} from session %{GREEDYDATA:cisco_secure_email_gateway.log.session} beacuse of inactivity timeout$"
                            ),
                            cached_grok!(
                                "^Session %{DATA:cisco_secure_email_gateway.log.session} user:%{USERNAME:user.name} %{WORD:cisco_secure_email_gateway.log.result}$"
                            ),
                            cached_grok!(
                                "^Session %{DATA:cisco_secure_email_gateway.log.session} from %{IP:host.ip} not found Destination:%{GREEDYDATA:cisco_secure_email_gateway.log.destination}$"
                            ),
                            cached_grok!(
                                "^SourceIP:%{IP:host.ip} Destination:%{GREEDYDATA:cisco_secure_email_gateway.log.destination} Username:%{USERNAME:user.name} Privilege:%{DATA:cisco_secure_email_gateway.log.privilege} session:%{DATA:cisco_secure_email_gateway.log.session} Action: %{GREEDYDATA:cisco_secure_email_gateway.log.action}$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} %{IP:source.ip}:%{NUMBER:source.port:long} - \\(%{GREEDYDATA:cisco_secure_email_gateway.log.description}\\)$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} %{IP:source.ip} port %{NUMBER:source.port:long} - %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cisco_secure_email_gateway.log.object} has been %{DATA:cisco_secure_email_gateway.log.action} for user %{USERNAME:user.name}$"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                let _cond = { event.get_str("user_agent.original") != Some("-") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
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
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("http.request.method") };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("web")]))?;
                }
                let _cond = { event.has_value("http.request.method") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("access")]))?;
                }
                let _cond = {
                    event.get_str("cisco_secure_email_gateway.log.result") == Some("expired")
                        || event.get_str("cisco_secure_email_gateway.log.action")
                            == Some("logged out")
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("session")]))?;
                }
                let _cond = {
                    event.get_str("cisco_secure_email_gateway.log.result") == Some("expired")
                        || event.get_str("cisco_secure_email_gateway.log.action")
                            == Some("logged out")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = { event.get_str("user.name") == Some("-") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.remove("user.name").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "user.name".into(),
                            });
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("user_agent.original") == Some("-") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.remove("user_agent.original").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "user_agent.original".into(),
                            });
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("client.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("client.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("host.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("host.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("source.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond =
                    { event.has_value("user.name") && event.get_str("user.name") != Some("-") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_gui_logs"
            }

            let _cond = {
                event.get_str("cisco_secure_email_gateway.log.category.name") == Some("antispam")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_anti_spam"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^case %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(%{NUMBER:cisco_secure_email_gateway.log.case_id}\\) : case-daemon: all %{DATA:cisco_secure_email_gateway.log.object} killed, %{GREEDYDATA:cisco_secure_email_gateway.log.result}$
                    // Grok pattern: ^case %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(%{NUMBER:cisco_secure_email_gateway.log.case_id}\\) : case-daemon: %{DATA:cisco_secure_email_gateway.log.object} killed by %{DATA:cisco_secure_email_gateway.log.command}, %{GREEDYDATA:cisco_secure_email_gateway.log.result}$
                    // Grok pattern: ^case %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(%{NUMBER:cisco_secure_email_gateway.log.case_id}\\) : case-daemon: %{GREEDYDATA:cisco_secure_email_gateway.log.result}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^case %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(%{NUMBER:cisco_secure_email_gateway.log.case_id}\\) : case-daemon: all %{DATA:cisco_secure_email_gateway.log.object} killed, %{GREEDYDATA:cisco_secure_email_gateway.log.result}$"
                            ),
                            cached_grok!(
                                "^case %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(%{NUMBER:cisco_secure_email_gateway.log.case_id}\\) : case-daemon: %{DATA:cisco_secure_email_gateway.log.object} killed by %{DATA:cisco_secure_email_gateway.log.command}, %{GREEDYDATA:cisco_secure_email_gateway.log.result}$"
                            ),
                            cached_grok!(
                                "^case %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(%{NUMBER:cisco_secure_email_gateway.log.case_id}\\) : case-daemon: %{GREEDYDATA:cisco_secure_email_gateway.log.result}$"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                // End nested pipeline: "pipeline_anti_spam"
            }

            let _cond = {
                event.get_str("cisco_secure_email_gateway.log.category.name") == Some("error_logs")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_error_logs"
                event.set("event.kind", json!("event"))?;
                event.set("event.type", Value::Array(vec![json!("error")]))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message")
                    {
                        // Grok pattern: ^Internal %{DATA:network.protocol} giving up on message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$
                        // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.description}: Unable to send System/Warning %{DATA:event.kind} to %{DATA:email.to.address} with subject \"%{GREEDYDATA:email.subject}\"\\.$
                        // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.alert_category}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                        // Grok pattern: ^Internal %{DATA:network.protocol} system attempting to send a message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$
                        // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                        let _ = extract_first_match(
                            &[
                                cached_grok!(
                                    "^Internal %{DATA:network.protocol} giving up on message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:cisco_secure_email_gateway.log.description}: Unable to send System/Warning %{DATA:event.kind} to %{DATA:email.to.address} with subject \"%{GREEDYDATA:email.subject}\"\\.$"
                                ),
                                cached_grok!(
                                    "^%{WORD:cisco_secure_email_gateway.log.alert_category}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"
                                ),
                                cached_grok!(
                                    "^Internal %{DATA:network.protocol} system attempting to send a message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                    Ok(())
                })();
                // End nested pipeline: "pipeline_error_logs"
            }

            let _cond = {
                event.get_str("cisco_secure_email_gateway.log.category.name") == Some("mail_logs")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_text_mail_logs"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^graymail \\[CONFIG\\] %{WORD:cisco_secure_email_gateway.log.vendor_action} %{GREEDYDATA:cisco_secure_email_gateway.log.object}$
                    // Grok pattern: ^URL_REP_CLIENT: %{WORD:cisco_secure_email_gateway.log.object_attr} %{DATA:cisco_secure_email_gateway.log.type}. Triggering %{WORD:cisco_secure_email_gateway.log.vendor_action} of %{GREEDYDATA:cisco_secure_email_gateway.log.object}\\.$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.subject}. Severity: %{WORD:cisco_secure_email_gateway.log.severity} \\(Risk Factor: %{NUMBER:cisco_secure_email_gateway.log.risk_factor:long}\\). %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^A System/Warning %{DATA:event.kind} was sent to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.connection_status} %{WORD:network.protocol} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} interface Management \\(%{IP:cisco_secure_email_gateway.log.interface}\\) address %{IP:cisco_secure_email_gateway.log.address} reverse dns host %{DATA:dns.question.name} verified %{WORD:cisco_secure_email_gateway.log.verified}$
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.connection_status} MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} From: <%{DATA:email.from.address}>$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} RID %{DATA:cisco_secure_email_gateway.log.recipient_id} To: <%{DATA:email.to.address}>$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} ready %{NUMBER:cisco_secure_email_gateway.log.read_bytes:long} bytes from <%{DATA:email.from.address}>$
                    // Grok pattern: ^ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} %{WORD:cisco_secure_email_gateway.log.connection_status}$
                    // Grok pattern: ^%{DATA:cisco_secure_email_gateway.log.message_status} %{WORD:network.protocol} DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} interface %{IP:cisco_secure_email_gateway.log.interface} address %{IP:cisco_secure_email_gateway.log.address}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message_status} DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} MID %{NUMBER:email.message_id} to RID \\[%{DATA:cisco_secure_email_gateway.log.recipient_id}\\](\\s\\[%{DATA:cisco_secure_email_gateway.log.email_participants}\\])?$
                    // Grok pattern: ^DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} %{WORD:cisco_secure_email_gateway.log.connection_status}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.description}: Unable to send System/Warning %{DATA:event.kind} to %{DATA:email.to.address} with subject \"%{GREEDYDATA:email.subject}\"\\.$
                    // Grok pattern: ^Internal %{DATA:network.protocol} system successfully sent a message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$
                    // Grok pattern: ^Internal %{DATA:network.protocol} giving up on message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$
                    // Grok pattern: ^Internal %{DATA:network.protocol} Error: %{GREEDYDATA:cisco_secure_email_gateway.log.description} to host %{IP:destination.ip}:%{NUMBER:destination.port:long} for recipient %{DATA:email.to.address}: %{GREEDYDATA:cisco_secure_email_gateway.log.subject}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} RID \\[%{DATA:cisco_secure_email_gateway.log.recipient_id}\\] Response %{GREEDYDATA:cisco_secure_email_gateway.log.response}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} Subject \"%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\"$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.message_status} for delivery$
                    // Grok pattern: ^Message %{GREEDYDATA:cisco_secure_email_gateway.log.message_status} MID %{NUMBER:email.message_id} done$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} interim verdict using engine: %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} interim AV verdict using %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} using engine: %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.subject}: verdict %{GREEDYDATA:cisco_secure_email_gateway.log.verdict_scale}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} Message-ID '\\<%{GREEDYDATA:cisco_secure_email_gateway.log.email}\\>'$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: Verification %{GREEDYDATA:cisco_secure_email_gateway.log.verified}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: mailfrom identity %{DATA:email.from.address} %{GREEDYDATA:cisco_secure_email_gateway.log.verified} \\(v\\=spf1\\)$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} matched all recipients for per-recipient policy %{WORD:cisco_secure_email_gateway.log.policy} in the %{DATA:email.direction} table$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SDR: Tracker Header : %{GREEDYDATA:cisco_secure_email_gateway.log.email_tracker_header}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SDR: Domains for which SDR is requested: reverse DNS host: %{DATA:dns.question.name}, helo: %{DATA:cisco_secure_email_gateway.log.helo}, env-from: %{DATA:cisco_secure_email_gateway.log.env}, header-from: %{DATA:email.from.address}, reply-to: %{DATA:email.to.address}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SDR: Consolidated Sender Threat Level: %{DATA:cisco_secure_email_gateway.log.threat_level}, Threat Category: %{DATA:cisco_secure_email_gateway.log.threat_category}, Suspected Domain\\(s\\) : %{GREEDYDATA:cisco_secure_email_gateway.log.suspected_domains} \\(other reasons for verdict\\)\\. Sender Maturity: %{GREEDYDATA:cisco_secure_email_gateway.log.maturity} for domain: %{DATA:cisco_secure_email_gateway.log.domain}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\\. %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: Message from domain %{DATA:email.from.address}, DMARC %{DATA:cisco_secure_email_gateway.log.verified} \\(SPF aligned %{DATA:cisco_secure_email_gateway.log.spf_aligned}, DKIM aligned %{DATA:cisco_secure_email_gateway.log.dkim_aligned}\\)$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: pass signature %{DATA:cisco_secure_email_gateway.log.verified} \\(%{GREEDYDATA:cisco_secure_email_gateway.log.details}\\)$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SHA %{BASE16NUM:file.hash.sha256} filename %{GREEDYDATA:file.name}\\.%{WORD:file.extension} %{GREEDYDATA:cisco_secure_email_gateway.log.details}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SHA %{BASE16NUM:file.hash.sha256} filename %{DATA:file.name} %{GREEDYDATA:cisco_secure_email_gateway.log.details}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:file.extension} file %{GREEDYDATA:cisco_secure_email_gateway.log.details}$
                    // Grok pattern: \\bMID %{NUMBER:email.message_id}(?: ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id})?
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^graymail \\[CONFIG\\] %{WORD:cisco_secure_email_gateway.log.vendor_action} %{GREEDYDATA:cisco_secure_email_gateway.log.object}$"
                            ),
                            cached_grok!(
                                "^URL_REP_CLIENT: %{WORD:cisco_secure_email_gateway.log.object_attr} %{DATA:cisco_secure_email_gateway.log.type}. Triggering %{WORD:cisco_secure_email_gateway.log.vendor_action} of %{GREEDYDATA:cisco_secure_email_gateway.log.object}\\.$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.subject}. Severity: %{WORD:cisco_secure_email_gateway.log.severity} \\(Risk Factor: %{NUMBER:cisco_secure_email_gateway.log.risk_factor:long}\\). %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"
                            ),
                            cached_grok!(
                                "^A System/Warning %{DATA:event.kind} was sent to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$"
                            ),
                            cached_grok!(
                                "^%{WORD:cisco_secure_email_gateway.log.connection_status} %{WORD:network.protocol} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} interface Management \\(%{IP:cisco_secure_email_gateway.log.interface}\\) address %{IP:cisco_secure_email_gateway.log.address} reverse dns host %{DATA:dns.question.name} verified %{WORD:cisco_secure_email_gateway.log.verified}$"
                            ),
                            cached_grok!(
                                "^%{WORD:cisco_secure_email_gateway.log.connection_status} MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} From: <%{DATA:email.from.address}>$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} RID %{DATA:cisco_secure_email_gateway.log.recipient_id} To: <%{DATA:email.to.address}>$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} ready %{NUMBER:cisco_secure_email_gateway.log.read_bytes:long} bytes from <%{DATA:email.from.address}>$"
                            ),
                            cached_grok!(
                                "^ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} %{WORD:cisco_secure_email_gateway.log.connection_status}$"
                            ),
                            cached_grok!(
                                "^%{DATA:cisco_secure_email_gateway.log.message_status} %{WORD:network.protocol} DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} interface %{IP:cisco_secure_email_gateway.log.interface} address %{IP:cisco_secure_email_gateway.log.address}$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.message_status} DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} MID %{NUMBER:email.message_id} to RID \\[%{DATA:cisco_secure_email_gateway.log.recipient_id}\\](\\s\\[%{DATA:cisco_secure_email_gateway.log.email_participants}\\])?$"
                            ),
                            cached_grok!(
                                "^DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} %{WORD:cisco_secure_email_gateway.log.connection_status}$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.description}: Unable to send System/Warning %{DATA:event.kind} to %{DATA:email.to.address} with subject \"%{GREEDYDATA:email.subject}\"\\.$"
                            ),
                            cached_grok!(
                                "^Internal %{DATA:network.protocol} system successfully sent a message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$"
                            ),
                            cached_grok!(
                                "^Internal %{DATA:network.protocol} giving up on message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$"
                            ),
                            cached_grok!(
                                "^Internal %{DATA:network.protocol} Error: %{GREEDYDATA:cisco_secure_email_gateway.log.description} to host %{IP:destination.ip}:%{NUMBER:destination.port:long} for recipient %{DATA:email.to.address}: %{GREEDYDATA:cisco_secure_email_gateway.log.subject}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} RID \\[%{DATA:cisco_secure_email_gateway.log.recipient_id}\\] Response %{GREEDYDATA:cisco_secure_email_gateway.log.response}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} Subject \"%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\"$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.message_status} for delivery$"
                            ),
                            cached_grok!(
                                "^Message %{GREEDYDATA:cisco_secure_email_gateway.log.message_status} MID %{NUMBER:email.message_id} done$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} interim verdict using engine: %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} interim AV verdict using %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} using engine: %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.subject}: verdict %{GREEDYDATA:cisco_secure_email_gateway.log.verdict_scale}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} Message-ID '\\<%{GREEDYDATA:cisco_secure_email_gateway.log.email}\\>'$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: Verification %{GREEDYDATA:cisco_secure_email_gateway.log.verified}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: mailfrom identity %{DATA:email.from.address} %{GREEDYDATA:cisco_secure_email_gateway.log.verified} \\(v\\=spf1\\)$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} matched all recipients for per-recipient policy %{WORD:cisco_secure_email_gateway.log.policy} in the %{DATA:email.direction} table$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} SDR: Tracker Header : %{GREEDYDATA:cisco_secure_email_gateway.log.email_tracker_header}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} SDR: Domains for which SDR is requested: reverse DNS host: %{DATA:dns.question.name}, helo: %{DATA:cisco_secure_email_gateway.log.helo}, env-from: %{DATA:cisco_secure_email_gateway.log.env}, header-from: %{DATA:email.from.address}, reply-to: %{DATA:email.to.address}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} SDR: Consolidated Sender Threat Level: %{DATA:cisco_secure_email_gateway.log.threat_level}, Threat Category: %{DATA:cisco_secure_email_gateway.log.threat_category}, Suspected Domain\\(s\\) : %{GREEDYDATA:cisco_secure_email_gateway.log.suspected_domains} \\(other reasons for verdict\\)\\. Sender Maturity: %{GREEDYDATA:cisco_secure_email_gateway.log.maturity} for domain: %{DATA:cisco_secure_email_gateway.log.domain}$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\\. %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: Message from domain %{DATA:email.from.address}, DMARC %{DATA:cisco_secure_email_gateway.log.verified} \\(SPF aligned %{DATA:cisco_secure_email_gateway.log.spf_aligned}, DKIM aligned %{DATA:cisco_secure_email_gateway.log.dkim_aligned}\\)$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: pass signature %{DATA:cisco_secure_email_gateway.log.verified} \\(%{GREEDYDATA:cisco_secure_email_gateway.log.details}\\)$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} SHA %{BASE16NUM:file.hash.sha256} filename %{GREEDYDATA:file.name}\\.%{WORD:file.extension} %{GREEDYDATA:cisco_secure_email_gateway.log.details}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} SHA %{BASE16NUM:file.hash.sha256} filename %{DATA:file.name} %{GREEDYDATA:cisco_secure_email_gateway.log.details}$"
                            ),
                            cached_grok!(
                                "^MID %{NUMBER:email.message_id} %{DATA:file.extension} file %{GREEDYDATA:cisco_secure_email_gateway.log.details}$"
                            ),
                            cached_grok!(
                                "\\bMID %{NUMBER:email.message_id}(?: ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id})?"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                    Ok(())
                })();
                let _cond =
                    { event.get_str("cisco_secure_email_gateway.log.type") == Some("changed") };
                if _cond {
                    event.set("event.type", json!("change"))?;
                }
                let _cond =
                    { event.get_str("cisco_secure_email_gateway.log.spf_aligned") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cisco_secure_email_gateway.log.spf_aligned") {
                            map_strings(
                                event,
                                "cisco_secure_email_gateway.log.spf_aligned",
                                "cisco_secure_email_gateway.log.spf_aligned",
                                str::to_lowercase,
                            )?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "lowercase")?;
                        event.set("_ingest.on_failure_processor_tag", "lowercase_spf_aligned")?;
                        event.append("error.message", json!(format!("Processor {} with tag fail-{} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond =
                    { event.get_str("cisco_secure_email_gateway.log.dkim_aligned") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cisco_secure_email_gateway.log.dkim_aligned") {
                            map_strings(
                                event,
                                "cisco_secure_email_gateway.log.dkim_aligned",
                                "cisco_secure_email_gateway.log.dkim_aligned",
                                str::to_lowercase,
                            )?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "lowercase")?;
                        event.set("_ingest.on_failure_processor_tag", "lowercase_dkim_aligned")?;
                        event.append("error.message", json!(format!("Processor {} with tag fail-{} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond =
                    { event.get_str("cisco_secure_email_gateway.log.spf_aligned") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cisco_secure_email_gateway.log.spf_aligned") {
                            if let Some(val) =
                                event.get("cisco_secure_email_gateway.log.spf_aligned")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "cisco_secure_email_gateway.log.spf_aligned"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event
                                    .set("cisco_secure_email_gateway.log.spf_aligned", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_cisco_secure_email_gateway_log_spf_aligned",
                        )?;
                        event.append("error.message", json!(format!("Processor {} with tag fail-{} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond =
                    { event.get_str("cisco_secure_email_gateway.log.dkim_aligned") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cisco_secure_email_gateway.log.dkim_aligned") {
                            if let Some(val) =
                                event.get("cisco_secure_email_gateway.log.dkim_aligned")
                            {
                                let converted =
                                    convert_value(val, "boolean").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "cisco_secure_email_gateway.log.dkim_aligned"
                                                .into(),
                                            message,
                                        }
                                    })?;
                                event.set(
                                    "cisco_secure_email_gateway.log.dkim_aligned",
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
                            "convert_cisco_secure_email_gateway_log_dkim_aligned",
                        )?;
                        event.append("error.message", json!(format!("Processor {} with tag fail-{} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("cisco_secure_email_gateway.log.interface") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_secure_email_gateway.log.interface")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_secure_email_gateway.log.address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_secure_email_gateway.log.address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("destination.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("destination.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_text_mail_logs"
            }

            let _cond = {
                event.get_str("cisco_secure_email_gateway.log.category.name")
                    == Some("content_scanner")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_content_scanner"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^PF: %{WORD:cisco_secure_email_gateway.log.vendor_action} %{WORD:cisco_secure_email_gateway.log.object} %{GREEDYDATA:cisco_secure_email_gateway.log.object_category}$
                    // Grok pattern: ^PF: %{WORD:cisco_secure_email_gateway.log.vendor_action} %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(pid=%{NUMBER:process.pid:long}\\)$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^PF: %{WORD:cisco_secure_email_gateway.log.vendor_action} %{WORD:cisco_secure_email_gateway.log.object} %{GREEDYDATA:cisco_secure_email_gateway.log.object_category}$"
                            ),
                            cached_grok!(
                                "^PF: %{WORD:cisco_secure_email_gateway.log.vendor_action} %{GREEDYDATA:cisco_secure_email_gateway.log.object_category} \\(pid=%{NUMBER:process.pid:long}\\)$"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                // End nested pipeline: "pipeline_content_scanner"
            }

            let _cond =
                { event.get_str("cisco_secure_email_gateway.log.category.name") == Some("system") };
            if _cond {
                // Begin nested pipeline: "pipeline_system"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^PID %{NUMBER:process.pid:long}: User %{USERNAME:user.name} commit changes:%{GREEDYDATA:cisco_secure_email_gateway.log.commit_changes}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.name}: qname:%{DATA:cisco_secure_email_gateway.log.qname} ns_name:%{DATA:cisco_secure_email_gateway.log.ns_name} zone:%{DATA:cisco_secure_email_gateway.log.zone} ref_zone:%{DATA:cisco_secure_email_gateway.log.ref_zone} referrals:%{GREEDYDATA:cisco_secure_email_gateway.log.referrals}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\\. %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} to %{GREEDYDATA:cisco_secure_email_gateway.log.description} ' '$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^PID %{NUMBER:process.pid:long}: User %{USERNAME:user.name} commit changes:%{GREEDYDATA:cisco_secure_email_gateway.log.commit_changes}$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.name}: qname:%{DATA:cisco_secure_email_gateway.log.qname} ns_name:%{DATA:cisco_secure_email_gateway.log.ns_name} zone:%{DATA:cisco_secure_email_gateway.log.zone} ref_zone:%{DATA:cisco_secure_email_gateway.log.ref_zone} referrals:%{GREEDYDATA:cisco_secure_email_gateway.log.referrals}$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\\. %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.subject} to %{GREEDYDATA:cisco_secure_email_gateway.log.description} ' '$"
                            ),
                            cached_grok!(
                                "^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}: %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_system"
            }

            let _cond = {
                event.get_str("cisco_secure_email_gateway.log.category.name") == Some("bounces")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_bounce"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.bounce_type}: DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} MID %{NUMBER:email.message_id} From:<%{GREEDYDATA:email.from.address}> To:<%{GREEDYDATA:email.to.address}> RID %{NUMBER:cisco_secure_email_gateway.log.recipient_id} - %{DATA:cisco_secure_email_gateway.log.error_code} - %{GREEDYDATA:event.reason} \\(%{GREEDYDATA:cisco_secure_email_gateway.log.response}\\)$
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.bounce_type}: %{NUMBER:email.message_id}:%{NUMBER:cisco_secure_email_gateway.log.recipient_id} From:<%{GREEDYDATA:email.from.address}> To:<%{GREEDYDATA:email.to.address}>$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^%{WORD:cisco_secure_email_gateway.log.bounce_type}: DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} MID %{NUMBER:email.message_id} From:<%{GREEDYDATA:email.from.address}> To:<%{GREEDYDATA:email.to.address}> RID %{NUMBER:cisco_secure_email_gateway.log.recipient_id} - %{DATA:cisco_secure_email_gateway.log.error_code} - %{GREEDYDATA:event.reason} \\(%{GREEDYDATA:cisco_secure_email_gateway.log.response}\\)$"
                            ),
                            cached_grok!(
                                "^%{WORD:cisco_secure_email_gateway.log.bounce_type}: %{NUMBER:email.message_id}:%{NUMBER:cisco_secure_email_gateway.log.recipient_id} From:<%{GREEDYDATA:email.from.address}> To:<%{GREEDYDATA:email.to.address}>$"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                // End nested pipeline: "pipeline_bounce"
            }

            let _cond =
                { event.get_str("cisco_secure_email_gateway.log.category.name") == Some("status") };
            if _cond {
                // Begin nested pipeline: "pipeline_status"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^Status: CPULd %{NUMBER:cisco_secure_email_gateway.log.cpu.utilization:long} DskIO %{NUMBER:cisco_secure_email_gateway.log.disk_io:long} RAMUtil %{NUMBER:cisco_secure_email_gateway.log.ram.utilization:long} QKUsd %{NUMBER:cisco_secure_email_gateway.log.queue_kilobytes_usd:long} QKFre %{NUMBER:cisco_secure_email_gateway.log.queue_kilobytes_free:long} CrtMID %{NUMBER:email.message_id} CrtICID %{NUMBER:cisco_secure_email_gateway.log.crt.injection_connection_id} CrtDCID %{NUMBER:cisco_secure_email_gateway.log.crt.delivery_connection_id} InjMsg %{NUMBER:cisco_secure_email_gateway.log.injected.messages:long} InjRcp %{NUMBER:cisco_secure_email_gateway.log.injected.recipients:long} GenBncRcp %{NUMBER:cisco_secure_email_gateway.log.generated_bounce_recipients:long} RejRcp %{NUMBER:cisco_secure_email_gateway.log.rejected_recipients:long} DrpMsg %{NUMBER:cisco_secure_email_gateway.log.dropped_messages:long} SftBncEvnt %{NUMBER:cisco_secure_email_gateway.log.soft_bounced_events:long} CmpRcp %{NUMBER:cisco_secure_email_gateway.log.completed_recipients:long} HrdBncRcp %{NUMBER:cisco_secure_email_gateway.log.hard_bounce_recipients:long} DnsHrdBnc %{NUMBER:cisco_secure_email_gateway.log.dns.hard_bounces:long} 5XXHrdBnc %{NUMBER:cisco_secure_email_gateway.log.5xx_hard_bounces:long} FltrHrdBnc %{NUMBER:cisco_secure_email_gateway.log.filter_hard_bounces:long} ExpHrdBnc %{NUMBER:cisco_secure_email_gateway.log.expired_hard_bounces:long} OtrHrdBnc %{NUMBER:cisco_secure_email_gateway.log.other_hard_bounces:long} DlvRcp %{NUMBER:cisco_secure_email_gateway.log.delivered_recipients:long} DelRcp %{NUMBER:cisco_secure_email_gateway.log.deleted_recipients:long} GlbUnsbHt %{NUMBER:cisco_secure_email_gateway.log.global_unsubscribe_hits:long} ActvRcp %{NUMBER:cisco_secure_email_gateway.log.active_recipients:long} UnatmptRcp %{NUMBER:cisco_secure_email_gateway.log.unattempted_recipients:long} AtmptRcp %{NUMBER:cisco_secure_email_gateway.log.attempted_recipients:long} CrtCncIn %{NUMBER:cisco_secure_email_gateway.log.current.inbound_connections:long} CrtCncOut %{NUMBER:cisco_secure_email_gateway.log.current.outbound_connections:long} DnsReq %{NUMBER:cisco_secure_email_gateway.log.dns.requests:long} NetReq %{NUMBER:cisco_secure_email_gateway.log.network_requests:long} CchHit %{NUMBER:cisco_secure_email_gateway.log.cache.hits:long} CchMis %{NUMBER:cisco_secure_email_gateway.log.cache.misses:long} CchEct %{NUMBER:cisco_secure_email_gateway.log.cache.exceptions:long} CchExp %{NUMBER:cisco_secure_email_gateway.log.cache.expired:long} CPUTTm %{NUMBER:cisco_secure_email_gateway.log.cpu.total_time:long} CPUETm %{NUMBER:cisco_secure_email_gateway.log.cpu.elapsed_time:long} MaxIO %{NUMBER:cisco_secure_email_gateway.log.max_io:long} RAMUsd %{NUMBER:cisco_secure_email_gateway.log.ram.used:long} MMLen %{NUMBER:cisco_secure_email_gateway.log.messages_length:long} DstInMem %{NUMBER:cisco_secure_email_gateway.log.destination_memory:long} ResCon %{NUMBER:cisco_secure_email_gateway.log.resource_conservation:long} WorkQ %{NUMBER:cisco_secure_email_gateway.log.work_queue:long} QuarMsgs %{NUMBER:cisco_secure_email_gateway.log.quarantine.messages:long} QuarQKUsd %{NUMBER:cisco_secure_email_gateway.log.quarantine.queue_kilobytes_used:long} LogUsd %{NUMBER:cisco_secure_email_gateway.log.log_used:long} SophLd %{NUMBER:cisco_secure_email_gateway.log.sophos_ld:long} BMLd %{NUMBER:cisco_secure_email_gateway.log.bmld:long} CASELd %{NUMBER:cisco_secure_email_gateway.log.case_ld:long} TotalLd %{NUMBER:cisco_secure_email_gateway.log.total_ld:long} LogAvail %{DATA:cisco_secure_email_gateway.log.log_available} EuQ %{NUMBER:cisco_secure_email_gateway.log.estimated.quarantine:long} EuqRls %{NUMBER:cisco_secure_email_gateway.log.estimated.quarantine_release_queue:long} CmrkLd %{NUMBER:cisco_secure_email_gateway.log.cmrkld:long} McafLd %{NUMBER:cisco_secure_email_gateway.log.mcafee_ld:long} SwIn %{NUMBER:cisco_secure_email_gateway.log.swapped.in:long} SwOut %{NUMBER:cisco_secure_email_gateway.log.swapped.out:long} SwPgIn %{NUMBER:cisco_secure_email_gateway.log.swapped.page.in:long} SwPgOut %{NUMBER:cisco_secure_email_gateway.log.swapped.page.out:long} SwapUsage %{DATA:cisco_secure_email_gateway.log.swap_usage} RptLd %{NUMBER:cisco_secure_email_gateway.log.reporting_load:long} QtnLd %{NUMBER:cisco_secure_email_gateway.log.quarantine.load:long} EncrQ %{NUMBER:cisco_secure_email_gateway.log.encryption_queue:long} InjBytes %{NUMBER:cisco_secure_email_gateway.log.injected.bytes:long}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^Status: CPULd %{NUMBER:cisco_secure_email_gateway.log.cpu.utilization:long} DskIO %{NUMBER:cisco_secure_email_gateway.log.disk_io:long} RAMUtil %{NUMBER:cisco_secure_email_gateway.log.ram.utilization:long} QKUsd %{NUMBER:cisco_secure_email_gateway.log.queue_kilobytes_usd:long} QKFre %{NUMBER:cisco_secure_email_gateway.log.queue_kilobytes_free:long} CrtMID %{NUMBER:email.message_id} CrtICID %{NUMBER:cisco_secure_email_gateway.log.crt.injection_connection_id} CrtDCID %{NUMBER:cisco_secure_email_gateway.log.crt.delivery_connection_id} InjMsg %{NUMBER:cisco_secure_email_gateway.log.injected.messages:long} InjRcp %{NUMBER:cisco_secure_email_gateway.log.injected.recipients:long} GenBncRcp %{NUMBER:cisco_secure_email_gateway.log.generated_bounce_recipients:long} RejRcp %{NUMBER:cisco_secure_email_gateway.log.rejected_recipients:long} DrpMsg %{NUMBER:cisco_secure_email_gateway.log.dropped_messages:long} SftBncEvnt %{NUMBER:cisco_secure_email_gateway.log.soft_bounced_events:long} CmpRcp %{NUMBER:cisco_secure_email_gateway.log.completed_recipients:long} HrdBncRcp %{NUMBER:cisco_secure_email_gateway.log.hard_bounce_recipients:long} DnsHrdBnc %{NUMBER:cisco_secure_email_gateway.log.dns.hard_bounces:long} 5XXHrdBnc %{NUMBER:cisco_secure_email_gateway.log.5xx_hard_bounces:long} FltrHrdBnc %{NUMBER:cisco_secure_email_gateway.log.filter_hard_bounces:long} ExpHrdBnc %{NUMBER:cisco_secure_email_gateway.log.expired_hard_bounces:long} OtrHrdBnc %{NUMBER:cisco_secure_email_gateway.log.other_hard_bounces:long} DlvRcp %{NUMBER:cisco_secure_email_gateway.log.delivered_recipients:long} DelRcp %{NUMBER:cisco_secure_email_gateway.log.deleted_recipients:long} GlbUnsbHt %{NUMBER:cisco_secure_email_gateway.log.global_unsubscribe_hits:long} ActvRcp %{NUMBER:cisco_secure_email_gateway.log.active_recipients:long} UnatmptRcp %{NUMBER:cisco_secure_email_gateway.log.unattempted_recipients:long} AtmptRcp %{NUMBER:cisco_secure_email_gateway.log.attempted_recipients:long} CrtCncIn %{NUMBER:cisco_secure_email_gateway.log.current.inbound_connections:long} CrtCncOut %{NUMBER:cisco_secure_email_gateway.log.current.outbound_connections:long} DnsReq %{NUMBER:cisco_secure_email_gateway.log.dns.requests:long} NetReq %{NUMBER:cisco_secure_email_gateway.log.network_requests:long} CchHit %{NUMBER:cisco_secure_email_gateway.log.cache.hits:long} CchMis %{NUMBER:cisco_secure_email_gateway.log.cache.misses:long} CchEct %{NUMBER:cisco_secure_email_gateway.log.cache.exceptions:long} CchExp %{NUMBER:cisco_secure_email_gateway.log.cache.expired:long} CPUTTm %{NUMBER:cisco_secure_email_gateway.log.cpu.total_time:long} CPUETm %{NUMBER:cisco_secure_email_gateway.log.cpu.elapsed_time:long} MaxIO %{NUMBER:cisco_secure_email_gateway.log.max_io:long} RAMUsd %{NUMBER:cisco_secure_email_gateway.log.ram.used:long} MMLen %{NUMBER:cisco_secure_email_gateway.log.messages_length:long} DstInMem %{NUMBER:cisco_secure_email_gateway.log.destination_memory:long} ResCon %{NUMBER:cisco_secure_email_gateway.log.resource_conservation:long} WorkQ %{NUMBER:cisco_secure_email_gateway.log.work_queue:long} QuarMsgs %{NUMBER:cisco_secure_email_gateway.log.quarantine.messages:long} QuarQKUsd %{NUMBER:cisco_secure_email_gateway.log.quarantine.queue_kilobytes_used:long} LogUsd %{NUMBER:cisco_secure_email_gateway.log.log_used:long} SophLd %{NUMBER:cisco_secure_email_gateway.log.sophos_ld:long} BMLd %{NUMBER:cisco_secure_email_gateway.log.bmld:long} CASELd %{NUMBER:cisco_secure_email_gateway.log.case_ld:long} TotalLd %{NUMBER:cisco_secure_email_gateway.log.total_ld:long} LogAvail %{DATA:cisco_secure_email_gateway.log.log_available} EuQ %{NUMBER:cisco_secure_email_gateway.log.estimated.quarantine:long} EuqRls %{NUMBER:cisco_secure_email_gateway.log.estimated.quarantine_release_queue:long} CmrkLd %{NUMBER:cisco_secure_email_gateway.log.cmrkld:long} McafLd %{NUMBER:cisco_secure_email_gateway.log.mcafee_ld:long} SwIn %{NUMBER:cisco_secure_email_gateway.log.swapped.in:long} SwOut %{NUMBER:cisco_secure_email_gateway.log.swapped.out:long} SwPgIn %{NUMBER:cisco_secure_email_gateway.log.swapped.page.in:long} SwPgOut %{NUMBER:cisco_secure_email_gateway.log.swapped.page.out:long} SwapUsage %{DATA:cisco_secure_email_gateway.log.swap_usage} RptLd %{NUMBER:cisco_secure_email_gateway.log.reporting_load:long} QtnLd %{NUMBER:cisco_secure_email_gateway.log.quarantine.load:long} EncrQ %{NUMBER:cisco_secure_email_gateway.log.encryption_queue:long} InjBytes %{NUMBER:cisco_secure_email_gateway.log.injected.bytes:long}$"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                // End nested pipeline: "pipeline_status"
            }

            let _cond =
                { event.get_str("cisco_secure_email_gateway.log.category.name") == Some("amp") };
            if _cond {
                // Begin nested pipeline: "pipeline_amp"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^File reputation query initiating. %{GREEDYDATA:_tmp.new_message}$
                    // Grok pattern: ^Response received for file reputation query from (Cloud|Cache). %{GREEDYDATA:_tmp.new_message}$
                    // Grok pattern: ^File Analysis complete. SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256}, Submit Timestamp: %{GREEDYDATA:_tmp.submit.timestamp}, Update Timestamp: %{GREEDYDATA:_tmp.update.timestamp}, Disposition: %{DATA:cisco_secure_email_gateway.log.disposition} Score: %{NUMBER:cisco_secure_email_gateway.log.score:long}, run_id: %{NUMBER:cisco_secure_email_gateway.log.run_id} Details: %{DATA:cisco_secure_email_gateway.log.details} Spyname:\\[%{GREEDYDATA:cisco_secure_email_gateway.log.spy_name}\\]$
                    // Grok pattern: ^(?i)File not uploaded for analysis.\\s+MID = %{NUMBER:email.message_id},? File SHA256\\[%{GREEDYDATA:email.attachments.file.hash.sha256}\\],? File mime\\[%{GREEDYDATA:email.attachments.file.mime_type}\\],? Reason: %{GREEDYDATA:event.reason}$
                    // Grok pattern: ^File analysis upload skipped. SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256},Timestamp\\[%{GREEDYDATA:_tmp.submit.timestamp}\\] details\\[%{GREEDYDATA:_tmp.cisco_secure_email_gateway.log.remaining_details}\\]$
                    // Grok pattern: ^SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256},Timestamp\\[%{GREEDYDATA:_tmp.submit.timestamp}\\] details\\[%{GREEDYDATA:cisco_secure_email_gateway.log.server_error_details}\\]$
                    // Grok pattern: ^Retrospective verdict received. %{GREEDYDATA:_tmp.new_message}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match_traced(
                        &[
                            cached_grok!(
                                "^File reputation query initiating. %{GREEDYDATA:_tmp.new_message}$"
                            ),
                            cached_grok!(
                                "^Response received for file reputation query from (Cloud|Cache). %{GREEDYDATA:_tmp.new_message}$"
                            ),
                            cached_grok!(
                                "^File Analysis complete. SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256}, Submit Timestamp: %{GREEDYDATA:_tmp.submit.timestamp}, Update Timestamp: %{GREEDYDATA:_tmp.update.timestamp}, Disposition: %{DATA:cisco_secure_email_gateway.log.disposition} Score: %{NUMBER:cisco_secure_email_gateway.log.score:long}, run_id: %{NUMBER:cisco_secure_email_gateway.log.run_id} Details: %{DATA:cisco_secure_email_gateway.log.details} Spyname:\\[%{GREEDYDATA:cisco_secure_email_gateway.log.spy_name}\\]$"
                            ),
                            cached_grok!(
                                "^(?i)File not uploaded for analysis.\\s+MID = %{NUMBER:email.message_id},? File SHA256\\[%{GREEDYDATA:email.attachments.file.hash.sha256}\\],? File mime\\[%{GREEDYDATA:email.attachments.file.mime_type}\\],? Reason: %{GREEDYDATA:event.reason}$"
                            ),
                            cached_grok!(
                                "^File analysis upload skipped. SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256},Timestamp\\[%{GREEDYDATA:_tmp.submit.timestamp}\\] details\\[%{GREEDYDATA:_tmp.cisco_secure_email_gateway.log.remaining_details}\\]$"
                            ),
                            cached_grok!(
                                "^SHA256: %{GREEDYDATA:email.attachments.file.hash.sha256},Timestamp\\[%{GREEDYDATA:_tmp.submit.timestamp}\\] details\\[%{GREEDYDATA:cisco_secure_email_gateway.log.server_error_details}\\]$"
                            ),
                            cached_grok!(
                                "^Retrospective verdict received. %{GREEDYDATA:_tmp.new_message}$"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                event.set(
                    "_tmp.grok_match_index",
                    json!(
                        event
                            .get("_ingest._grok_match_index")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                let _cond = {
                    event.has_value("_tmp.new_message")
                        && event.get_str("_tmp.grok_match_index") != Some("1")
                };
                if _cond {
                    if let Some(kv_str) = event.get_string("_tmp.new_message") {
                        for pair in cached_regex!(",\\s*(?=(File Name|FileName|File Size|File Type|Reputation Score|Analysis Score|verdict_source|upload_action|Disposition|Malware|Verdict|Spyname|Timestamp|SHA256|sha256|MID)\\s*[=:])").split(&kv_str).into_iter() {
                if pair.trim().is_empty() {
                continue;
                }
                let Some((key, value)) = ({ let parts = cached_regex!("\\s*=\\s*|:\\s*").splitn(&pair, 2); match (parts.first(), parts.get(1)) { (Some(k), Some(v)) => Some((k.clone(), v.clone())), _ => None } }) else {
                return Err(TransformError::ParseError {
                path: "_tmp.new_message".into(),
                message: format!("does not contain value_split: {pair}"),
                });
                };
                {
                let key = &key[..];
                if !key.is_empty() {
                kv_put(event, key, value)?;
                }
                }
                }
                    }
                }
                let _cond = {
                    event.has_value("_tmp.new_message")
                        && event.get_str("_tmp.grok_match_index") == Some("1")
                };
                if _cond {
                    if let Some(kv_str) = event.get_string("_tmp.new_message") {
                        for pair in cached_regex!(",\\s*(?=(File Name|FileName|File Size|File Type|Reputation Score|Analysis Score|verdict_source|upload_action|Disposition|Malware|Verdict|Spyname|Timestamp|SHA256|sha256|MID)\\s*=)").split(&kv_str).into_iter() {
                if pair.trim().is_empty() {
                continue;
                }
                let Some((key, value)) = ({ let parts = cached_regex!("\\s*=\\s*").splitn(&pair, 2); match (parts.first(), parts.get(1)) { (Some(k), Some(v)) => Some((k.clone(), v.clone())), _ => None } }) else {
                return Err(TransformError::ParseError {
                path: "_tmp.new_message".into(),
                message: format!("does not contain value_split: {pair}"),
                });
                };
                {
                let key = &key[..];
                if !key.is_empty() {
                kv_put(event, key, value)?;
                }
                }
                }
                    }
                }
                let _cond =
                    { event.has_value("_tmp.cisco_secure_email_gateway.log.remaining_details") };
                if _cond {
                    if let Some(input) =
                        event.get_string("_tmp.cisco_secure_email_gateway.log.remaining_details")
                    {
                        // Grok pattern: ^File SHA256\\[%{GREEDYDATA:email.attachments.file.hash.sha256}\\] file mime\\[%{GREEDYDATA:email.attachments.file.mime_type}\\], upload priority\\[%{GREEDYDATA:cisco_secure_email_gateway.log.upload.priority}\\] not uploaded, re-tries\\[%{GREEDYDATA:cisco_secure_email_gateway.log.retries:long}\\], backoff\\[%{GREEDYDATA:cisco_secure_email_gateway.log.backoff:long}\\] %{GREEDYDATA:cisco_secure_email_gateway.log.details}$
                        let _ = cached_grok!("^File SHA256\\[%{GREEDYDATA:email.attachments.file.hash.sha256}\\] file mime\\[%{GREEDYDATA:email.attachments.file.mime_type}\\], upload priority\\[%{GREEDYDATA:cisco_secure_email_gateway.log.upload.priority}\\] not uploaded, re-tries\\[%{GREEDYDATA:cisco_secure_email_gateway.log.retries:long}\\], backoff\\[%{GREEDYDATA:cisco_secure_email_gateway.log.backoff:long}\\] %{GREEDYDATA:cisco_secure_email_gateway.log.details}$").extract_into(&input, event)?;
                    }
                }
                if event.has_value("Timestamp") {
                    event.rename("Timestamp", "_tmp.submit.timestamp")?;
                }
                let _cond = {
                    event.has_value("_tmp.submit.timestamp")
                        && event.get_str("cisco_secure_email_gateway.log._tmp.submit.timestamp")
                            != Some("0")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_tmp.submit.timestamp") {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => event.set(
                                    "cisco_secure_email_gateway.log.submit.timestamp",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_tmp.submit.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date__tmp_submit_timestamp_to_cisco_secure_email_gateway_log_submit_timestamp_dd83046a")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                    event.has_value("_tmp.update.timestamp")
                        && event.get_str("cisco_secure_email_gateway.log._tmp.update.timestamp")
                            != Some("0")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_tmp.update.timestamp") {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => event.set(
                                    "cisco_secure_email_gateway.log.update.timestamp",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_tmp.update.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date__tmp_update_timestamp_to_cisco_secure_email_gateway_log_update_timestamp_5d6d9d79")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("File Name") {
                    event.rename("File Name", "email.attachments.file.name")?;
                }
                if event.has_value("MID") {
                    event.rename("MID", "email.message_id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    gsub_field(
                        event,
                        "File Size",
                        "File Size",
                        cached_regex!("\\ bytes"),
                        "",
                    )?;
                    Ok(())
                })();
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("File Size") {
                        if let Some(val) = event.get("File Size") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "File Size".into(),
                                    message,
                                }
                            })?;
                            event.set("email.attachments.file.size", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_File_Size_to_email_attachments_file_size_1e170071",
                    )?;
                    if event.remove("File Size").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "File Size".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                if event.has_value("File Type") {
                    event.rename("File Type", "email.content_type")?;
                }
                if event.has_value("FileName") {
                    event.rename("FileName", "email.attachments.file.name")?;
                }
                if event.has_value("Malware") {
                    event.rename("Malware", "cisco_secure_email_gateway.log.malware")?;
                }
                if event.has_value("Disposition") {
                    event.rename("Disposition", "cisco_secure_email_gateway.log.disposition")?;
                }
                if event.has_value("Analysis Score") {
                    if let Some(val) = event.get("Analysis Score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "Analysis Score".into(),
                                message,
                            }
                        })?;
                        event.set("Analysis Score", converted)?;
                    }
                }
                if event.has_value("Analysis Score") {
                    event.rename("Analysis Score", "cisco_secure_email_gateway.log.score")?;
                }
                if event.has_value("sha256") {
                    event.rename("sha256", "email.attachments.file.hash.sha256")?;
                }
                if event.has_value("upload_action") {
                    event.rename(
                        "upload_action",
                        "cisco_secure_email_gateway.log.upload.action",
                    )?;
                }
                if event.has_value("Reputation Score") {
                    event.rename(
                        "Reputation Score",
                        "cisco_secure_email_gateway.log.reputation_score",
                    )?;
                }
                if event.has_value("SHA256") {
                    event.rename("SHA256", "email.attachments.file.hash.sha256")?;
                }
                if event.has_value("Spyname") {
                    event.rename("Spyname", "cisco_secure_email_gateway.log.spy_name")?;
                }
                if event.has_value("Verdict") {
                    event.rename("Verdict", "cisco_secure_email_gateway.log.verdict")?;
                }
                if event.has_value("verdict_source") {
                    event.rename(
                        "verdict_source",
                        "cisco_secure_email_gateway.log.verdict_source",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    gsub_field(
                        event,
                        "email.attachments.file.name",
                        "email.attachments.file.name",
                        cached_regex!("\\'"),
                        "",
                    )?;
                    Ok(())
                })();
                let _cond = { event.has_value("email.attachments.file.hash.sha256") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("email.attachments.file.hash.sha256")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("_tmp");
                event.remove("File Size");
                // End nested pipeline: "pipeline_amp"
            }

            let _cond = {
                ["consolidated_event", "SplunkGIS"].contains(
                    &event
                        .get_str("cisco_secure_email_gateway.log.category.name")
                        .unwrap_or(""),
                )
            };
            if _cond {
                // Begin nested pipeline: "pipeline_consolidated_event"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^(?:%{DATA:_tmp.timestamp} )?CEF:%{NUMBER:cisco_secure_email_gateway.log.cef_format_version}\\|%{WORD:cisco_secure_email_gateway.log.appliance.vendor}\\|%{DATA:cisco_secure_email_gateway.log.appliance.product}\\|%{DATA:cisco_secure_email_gateway.log.appliance.version}\\|%{DATA:cisco_secure_email_gateway.log.event_class_id}\\|%{DATA:cisco_secure_email_gateway.log.event.name}\\|%{WORD:event.severity}\\|%{GREEDYDATA:_tmp.details}$
                    let _ = cached_grok!("^(?:%{DATA:_tmp.timestamp} )?CEF:%{NUMBER:cisco_secure_email_gateway.log.cef_format_version}\\|%{WORD:cisco_secure_email_gateway.log.appliance.vendor}\\|%{DATA:cisco_secure_email_gateway.log.appliance.product}\\|%{DATA:cisco_secure_email_gateway.log.appliance.version}\\|%{DATA:cisco_secure_email_gateway.log.event_class_id}\\|%{DATA:cisco_secure_email_gateway.log.event.name}\\|%{WORD:event.severity}\\|%{GREEDYDATA:_tmp.details}$").extract_into(&input, event)?;
                }
                let _cond = { event.has_value("_tmp.details") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_tmp.details") {
                            if let Some(kv_str) = event.get_string("_tmp.details") {
                                for pair in cached_regex!("(?:((\\s+)?$|\\s+(?=\\w+=)))")
                                    .split(&kv_str)
                                    .into_iter()
                                {
                                    if pair.trim().is_empty() {
                                        continue;
                                    }
                                    let Some((key, value)) = pair.split_once("=") else {
                                        return Err(TransformError::ParseError {
                                            path: "_tmp.details".into(),
                                            message: format!(
                                                "does not contain value_split: {pair}"
                                            ),
                                        });
                                    };
                                    {
                                        if !key.is_empty() {
                                            kv_put(event, &format!("_tmp.fields.{}", key), value)?;
                                        }
                                    }
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAAttachmentDetails") {
                        event.rename(
                            "_tmp.fields.ESAAttachmentDetails",
                            "cisco_secure_email_gateway.log.esa.attachment_details",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("_tmp.fields.ESADHASource") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_tmp.fields.ESADHASource") {
                            if let Some(val) = event.get("_tmp.fields.ESADHASource") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_tmp.fields.ESADHASource".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "cisco_secure_email_gateway.log.esa.dha_source",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESADKIMVerdict") {
                        event.rename(
                            "_tmp.fields.ESADKIMVerdict",
                            "cisco_secure_email_gateway.log.esa.dkim_verdict",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESADLPVerdict") {
                        event.rename(
                            "_tmp.fields.ESADLPVerdict",
                            "cisco_secure_email_gateway.log.esa.dlp_verdict",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESADMARCVerdict") {
                        event.rename(
                            "_tmp.fields.ESADMARCVerdict",
                            "cisco_secure_email_gateway.log.esa.dmarc_verdict",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESADaneHost") {
                        event.rename(
                            "_tmp.fields.ESADaneHost",
                            "cisco_secure_email_gateway.log.esa.dane.host",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESADaneStatus") {
                        event.rename(
                            "_tmp.fields.ESADaneStatus",
                            "cisco_secure_email_gateway.log.esa.dane.status",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAFinalActionDetails") {
                        event.rename(
                            "_tmp.fields.ESAFinalActionDetails",
                            "cisco_secure_email_gateway.log.esa.final_action_details",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAFriendlyFrom") {
                        event.rename(
                            "_tmp.fields.ESAFriendlyFrom",
                            "cisco_secure_email_gateway.log.esa.friendly_from",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAGMVerdict") {
                        event.rename(
                            "_tmp.fields.ESAGMVerdict",
                            "cisco_secure_email_gateway.log.esa.graymail_verdict",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAHeloDomain") {
                        if let Some(s) = event.get_string("_tmp.fields.ESAHeloDomain") {
                            match url_decode(&s) {
                                Some(decoded) => event.set(
                                    "cisco_secure_email_gateway.log.esa.helo.domain",
                                    json!(decoded),
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_tmp.fields.ESAHeloDomain".into(),
                                        message: format!("cannot url-decode '{s}'"),
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("_tmp.fields.ESAHeloIP") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_tmp.fields.ESAHeloIP") {
                            if let Some(val) = event.get("_tmp.fields.ESAHeloIP") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_tmp.fields.ESAHeloIP".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("cisco_secure_email_gateway.log.esa.helo.ip", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAMARAction") {
                        event.rename(
                            "_tmp.fields.ESAMARAction",
                            "cisco_secure_email_gateway.log.esa.mail_auto_remediation_action",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAMFVerdict") {
                        event.rename(
                            "_tmp.fields.ESAMFVerdict",
                            "cisco_secure_email_gateway.log.esa.mf_verdict",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAMailFlowPolicy") {
                        event.rename(
                            "_tmp.fields.ESAMailFlowPolicy",
                            "cisco_secure_email_gateway.log.esa.mail_flow_policy",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAMsgSize") {
                        if let Some(val) = event.get("_tmp.fields.ESAMsgSize") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_tmp.fields.ESAMsgSize".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_secure_email_gateway.log.esa.msg_size", converted)?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAMsgTooBigFromSender") {
                        event.rename(
                            "_tmp.fields.ESAMsgTooBigFromSender",
                            "cisco_secure_email_gateway.log.esa.msg_too_big_from_sender",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAOFVerdict") {
                        event.rename(
                            "_tmp.fields.ESAOFVerdict",
                            "cisco_secure_email_gateway.log.esa.outbreak_filter_verdict",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESARateLimitedIP") {
                        event.rename(
                            "_tmp.fields.ESARateLimitedIP",
                            "cisco_secure_email_gateway.log.esa.rate_limited_ip",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAReplyTo") {
                        event.rename(
                            "_tmp.fields.ESAReplyTo",
                            "cisco_secure_email_gateway.log.esa.reply_to",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESASDRDomainAge") {
                        event.rename(
                            "_tmp.fields.ESASDRDomainAge",
                            "cisco_secure_email_gateway.log.esa.sdr_consolidated_domain_age",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESASPFVerdict") {
                        event.rename(
                            "_tmp.fields.ESASPFVerdict",
                            "cisco_secure_email_gateway.log.esa.spf_verdict",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESASenderGroup") {
                        event.rename(
                            "_tmp.fields.ESASenderGroup",
                            "cisco_secure_email_gateway.log.esa.sender_group",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESATLSInCipher") {
                        event.rename(
                            "_tmp.fields.ESATLSInCipher",
                            "cisco_secure_email_gateway.log.esa.tls.in.cipher",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESATLSInConnStatus") {
                        event.rename(
                            "_tmp.fields.ESATLSInConnStatus",
                            "cisco_secure_email_gateway.log.esa.tls.in.connection_status",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESATLSInProtocol") {
                        event.rename(
                            "_tmp.fields.ESATLSInProtocol",
                            "cisco_secure_email_gateway.log.esa.tls.in.protocol",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESATLSOutCipher") {
                        event.rename(
                            "_tmp.fields.ESATLSOutCipher",
                            "cisco_secure_email_gateway.log.esa.tls.out.cipher",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESATLSOutConnStatus") {
                        event.rename(
                            "_tmp.fields.ESATLSOutConnStatus",
                            "cisco_secure_email_gateway.log.esa.tls.out.connection_status",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESATLSOutProtocol") {
                        event.rename(
                            "_tmp.fields.ESATLSOutProtocol",
                            "cisco_secure_email_gateway.log.esa.tls.out.protocol",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAURLDetails") {
                        event.rename(
                            "_tmp.fields.ESAURLDetails",
                            "cisco_secure_email_gateway.log.esa.url_details",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.act") {
                        event.rename("_tmp.fields.act", "cisco_secure_email_gateway.log.act")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cfp1Label") {
                        event.rename(
                            "_tmp.fields.cfp1Label",
                            "cisco_secure_email_gateway.log.cfp1_label",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = {
                    !(["None", "not enabled", "rfc1918"]
                        .contains(&event.get_str("_tmp.fields.cfp1").unwrap_or("")))
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_tmp.fields.cfp1") {
                            if let Some(val) = event.get("_tmp.fields.cfp1") {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_tmp.fields.cfp1".into(),
                                            message,
                                        }
                                    })?;
                                event.set("cisco_secure_email_gateway.log.cfp1", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert__tmp_fields_cfp1_to_cisco_secure_email_gateway_log_cfp1_736a7f37")?;
                        if event.remove("_tmp.fields.cfp1").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "_tmp.fields.cfp1".into(),
                            });
                        }
                        event.append(
                            "error.message",
                            json!(format!(
                                "Failed to convert cfp1 field: {}",
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs1Label") {
                        event.rename(
                            "_tmp.fields.cs1Label",
                            "cisco_secure_email_gateway.log.cs1_label",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs1") {
                        event.rename("_tmp.fields.cs1", "cisco_secure_email_gateway.log.cs1")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs2Label") {
                        event.rename(
                            "_tmp.fields.cs2Label",
                            "cisco_secure_email_gateway.log.cs2_label",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs2") {
                        event.rename("_tmp.fields.cs2", "cisco_secure_email_gateway.log.cs2")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs3Label") {
                        event.rename(
                            "_tmp.fields.cs3Label",
                            "cisco_secure_email_gateway.log.cs3_label",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs3") {
                        event.rename("_tmp.fields.cs3", "cisco_secure_email_gateway.log.cs3")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs4Label") {
                        event.rename(
                            "_tmp.fields.cs4Label",
                            "cisco_secure_email_gateway.log.cs4_label",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("_tmp.fields.cs4")
                        && event.get_str("_tmp.fields.cs4") != Some("''")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_tmp.fields.cs4") {
                            gsub_field(
                                event,
                                "_tmp.fields.cs4",
                                "cisco_secure_email_gateway.log.cs4",
                                cached_regex!("^'<|>'$"),
                                "",
                            )?;
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs5Label") {
                        event.rename(
                            "_tmp.fields.cs5Label",
                            "cisco_secure_email_gateway.log.cs5_label",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs5") {
                        event.rename("_tmp.fields.cs5", "cisco_secure_email_gateway.log.cs5")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs6Label") {
                        event.rename(
                            "_tmp.fields.cs6Label",
                            "cisco_secure_email_gateway.log.cs6_label",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.cs6") {
                        event.rename("_tmp.fields.cs6", "cisco_secure_email_gateway.log.cs6")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.deviceDirection") {
                        event.rename(
                            "_tmp.fields.deviceDirection",
                            "cisco_secure_email_gateway.log.device_direction",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.deviceInboundInterface") {
                        event.rename(
                            "_tmp.fields.deviceInboundInterface",
                            "cisco_secure_email_gateway.log.listener.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.deviceOutboundInterface") {
                        event.rename(
                            "_tmp.fields.deviceOutboundInterface",
                            "cisco_secure_email_gateway.log.listener.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.duser") {
                        event.rename("_tmp.fields.duser", "email.to.address")?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("_tmp.fields.dvc") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_tmp.fields.dvc") {
                            if let Some(val) = event.get("_tmp.fields.dvc") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_tmp.fields.dvc".into(),
                                        message,
                                    }
                                })?;
                                event.set("cisco_secure_email_gateway.log.data.ip", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.endTime") {
                        event.rename("_tmp.fields.endTime", "event.end")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.end") {
                        event.rename("_tmp.fields.end", "event.end")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.sourceHostName") {
                        if let Some(s) = event.get_string("_tmp.fields.sourceHostName") {
                            match url_decode(&s) {
                                Some(decoded) => event.set("source.domain", json!(decoded))?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_tmp.fields.sourceHostName".into(),
                                        message: format!("cannot url-decode '{s}'"),
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.shost") {
                        if let Some(s) = event.get_string("_tmp.fields.shost") {
                            match url_decode(&s) {
                                Some(decoded) => event.set("source.domain", json!(decoded))?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "_tmp.fields.shost".into(),
                                        message: format!("cannot url-decode '{s}'"),
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("_tmp.fields.sourceAddress") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_tmp.fields.sourceAddress") {
                            if let Some(val) = event.get("_tmp.fields.sourceAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_tmp.fields.sourceAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("_tmp.fields.src") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_tmp.fields.src") {
                            if let Some(val) = event.get("_tmp.fields.src") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_tmp.fields.src".into(),
                                        message,
                                    }
                                })?;
                                event.set("source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("_tmp.fields.msg") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_tmp.fields.msg") {
                            gsub_field(
                                event,
                                "_tmp.fields.msg",
                                "email.subject",
                                cached_regex!("(?:^['\"]|['\"]$)"),
                                "",
                            )?;
                        }
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.suser") {
                        event.rename("_tmp.fields.suser", "email.from.address")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.startTime") {
                        event.rename("_tmp.fields.startTime", "event.start")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.start") {
                        event.rename("_tmp.fields.start", "event.start")?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("cisco_secure_email_gateway.log.esa.helo.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_secure_email_gateway.log.esa.helo.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_secure_email_gateway.log.data.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_secure_email_gateway.log.data.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("source.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("cisco_secure_email_gateway.log.listener.name")
                        == Some("Incomingmail")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("email.direction", json!("inbound"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("cisco_secure_email_gateway.log.listener.name")
                        == Some("Outcomingmail")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("email.direction", json!("outbound"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("cisco_secure_email_gateway.log.device_direction") == Some("0")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set(
                            "cisco_secure_email_gateway.log.device_direction",
                            json!("incoming"),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("cisco_secure_email_gateway.log.device_direction") == Some("1")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set(
                            "cisco_secure_email_gateway.log.device_direction",
                            json!("outgoing"),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("_tmp.fields.deviceExternalId") {
                    event.rename("_tmp.fields.deviceExternalId", "host.id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.fields.ESAMID") {
                        event.rename("_tmp.fields.ESAMID", "email.message_id")?;
                    }
                    Ok(())
                })();
                if event.has_value("_tmp.fields.ESAICID") {
                    event.rename(
                        "_tmp.fields.ESAICID",
                        "cisco_secure_email_gateway.log.esa.injection_connection_id",
                    )?;
                }
                if event.has_value("_tmp.fields.ESADCID") {
                    event.rename(
                        "_tmp.fields.ESADCID",
                        "cisco_secure_email_gateway.log.esa.delivery_connection_id",
                    )?;
                }
                if event.has_value("_tmp.fields.ESAAMPVerdict") {
                    event.rename(
                        "_tmp.fields.ESAAMPVerdict",
                        "cisco_secure_email_gateway.log.esa.amp_verdict",
                    )?;
                }
                if event.has_value("_tmp.fields.ESAASVerdict") {
                    event.rename(
                        "_tmp.fields.ESAASVerdict",
                        "cisco_secure_email_gateway.log.esa.as_verdict",
                    )?;
                }
                if event.has_value("_tmp.fields.ESAAVVerdict") {
                    event.rename(
                        "_tmp.fields.ESAAVVerdict",
                        "cisco_secure_email_gateway.log.esa.av_verdict",
                    )?;
                }
                if event.has_value("_tmp.fields.ESACFVerdict") {
                    event.rename(
                        "_tmp.fields.ESACFVerdict",
                        "cisco_secure_email_gateway.log.esa.content_filter_verdict",
                    )?;
                }
                let _cond = { event.has_value("_tmp.timestamp") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "E MMM dd HH:mm:ss yyyy",
                                    "E MMM  d HH:mm:ss yyyy",
                                    "E MMM d HH:mm:ss yyyy",
                                    "MMM  d HH:mm:ss",
                                    "MMM dd HH:mm:ss",
                                    "MMM d HH:mm:ss",
                                ],
                                event.get_str("event.timezone"),
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
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date__tmp_timestamp_d7741cfc",
                        )?;
                        event.remove("event.timezone");
                        let _cond = { event.has_value("_tmp.timestamp") };
                        if _cond {
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "E MMM dd HH:mm:ss yyyy",
                                            "E MMM  d HH:mm:ss yyyy",
                                            "E MMM d HH:mm:ss yyyy",
                                            "MMM  d HH:mm:ss",
                                            "MMM dd HH:mm:ss",
                                            "MMM d HH:mm:ss",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set("@timestamp", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_tmp.timestamp".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date__tmp_timestamp_5d9cfc58",
                                )?;
                                event.append(
                                    "error.message",
                                    json!(
                                        event
                                            .get("_ingest.on_failure_message")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("event.start") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("event.start") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "E MMM dd HH:mm:ss yyyy",
                                    "E MMM  d HH:mm:ss yyyy",
                                    "E MMM d HH:mm:ss yyyy",
                                    "MMM  d HH:mm:ss",
                                    "MMM dd HH:mm:ss",
                                    "MMM d HH:mm:ss",
                                ],
                                event.get_str("event.timezone"),
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
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date event.start")?;
                        event.remove("event.timezone");
                        let _cond = { event.has_value("event.start") };
                        if _cond {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("event.start") {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "E MMM dd HH:mm:ss yyyy",
                                            "E MMM  d HH:mm:ss yyyy",
                                            "E MMM d HH:mm:ss yyyy",
                                            "MMM  d HH:mm:ss",
                                            "MMM dd HH:mm:ss",
                                            "MMM d HH:mm:ss",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set("event.start", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "event.start".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date re-parse event.start",
                                )?;
                                event.remove("event.start");
                                event.append(
                                    "error.message",
                                    json!(
                                        event
                                            .get("_ingest.on_failure_message")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("event.end") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("event.end") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "E MMM dd HH:mm:ss yyyy",
                                    "E MMM  d HH:mm:ss yyyy",
                                    "E MMM d HH:mm:ss yyyy",
                                    "MMM  d HH:mm:ss",
                                    "MMM dd HH:mm:ss",
                                    "MMM d HH:mm:ss",
                                ],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => event.set("event.end", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "event.end".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date event.end")?;
                        event.remove("event.timezone");
                        let _cond = { event.has_value("event.end") };
                        if _cond {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) = event.get_as_string("event.end") {
                                    match parse_date_out(
                                        &date_str,
                                        &[
                                            "E MMM dd HH:mm:ss yyyy",
                                            "E MMM  d HH:mm:ss yyyy",
                                            "E MMM d HH:mm:ss yyyy",
                                            "MMM  d HH:mm:ss",
                                            "MMM dd HH:mm:ss",
                                            "MMM d HH:mm:ss",
                                        ],
                                        None,
                                        None,
                                    ) {
                                        Some(parsed) => event.set("event.end", parsed)?,
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "event.end".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
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
                                    "date re-parse event.end",
                                )?;
                                event.remove("event.end");
                                event.append(
                                    "error.message",
                                    json!(
                                        event
                                            .get("_ingest.on_failure_message")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
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
                    event.remove("_tmp");
                    Ok(())
                })();
                // End nested pipeline: "pipeline_consolidated_event"
            }

            let _cond = {
                event.get_str("cisco_secure_email_gateway.log.category.name") == Some("antivirus")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_antivirus"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} - %{WORD:cisco_secure_email_gateway.log.type} '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' \\(\\)$
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} - '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' '%{GREEDYDATA:cisco_secure_email_gateway.log.encrypted_hash}'$
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' 'body.scan\\/%{GREEDYDATA:file.name}' 1 0$
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} - '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}'$
                    // Grok pattern: ^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} - %{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} - %{WORD:cisco_secure_email_gateway.log.type} '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' \\(\\)$"
                            ),
                            cached_grok!(
                                "^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} - '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' '%{GREEDYDATA:cisco_secure_email_gateway.log.encrypted_hash}'$"
                            ),
                            cached_grok!(
                                "^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}' 'body.scan\\/%{GREEDYDATA:file.name}' 1 0$"
                            ),
                            cached_grok!(
                                "^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} %{NUMBER:cisco_secure_email_gateway.log.rank:long} - %{WORD:cisco_secure_email_gateway.log.type} - '%{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}'$"
                            ),
                            cached_grok!(
                                "^%{WORD:observer.vendor}  antivirus - MID %{NUMBER:email.message_id} - %{GREEDYDATA:cisco_secure_email_gateway.log.antivirus_result}$"
                            ),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
                let _cond =
                    { event.get_str("cisco_secure_email_gateway.log.type") == Some("Error") };
                if _cond {
                    event.set("event.type", json!("error"))?;
                }
                // End nested pipeline: "pipeline_antivirus"
            }

            let _cond = {
                event
                    .get("email.from.address")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                event.set(
                    "email.from.address",
                    Value::Array(vec![json!(
                        event
                            .get("email.from.address")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = { event.get("email.to.address").is_some_and(|v| v.is_string()) };
            if _cond {
                event.set(
                    "email.to.address",
                    Value::Array(vec![json!(
                        event
                            .get("email.to.address")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event.get_str("cisco_secure_email_gateway.log.category.name") == Some("antivirus")
            };
            if _cond {
                event.set("event.category", json!("vulnerability"))?;
            }

            event.remove("_tmp");
            event.remove("_conf");

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        event.remove("_ingest._grok_match_index");
        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
            event.remove("_ingest");
        }
        Ok(TransformResult::Continue)
    }
}
