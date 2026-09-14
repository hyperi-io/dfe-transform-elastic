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
            let _cond = { !event.has_value("event.original") && event.has_value("message") };
            if _cond {
                if let Some(v) = event.get("message").cloned() {
                    event.set("event.original", v)?;
                }
            }

            let _cond = { event.get_str("input.type") == Some("journald") };
            if _cond {
                // Begin nested pipeline: "journald"
                if event.has_value("journald.process.name") {
                    event.rename("journald.process.name", "process.name")?;
                }
                let _cond = { event.has_value("log.syslog.procid") };
                if _cond {
                    if event.has_value("log.syslog.procid") {
                        if let Some(val) = event.get("log.syslog.procid") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "log.syslog.procid".into(),
                                    message,
                                }
                            })?;
                            event.set("log.syslog.procid", converted)?;
                        }
                    }
                }
                event.remove("journald");
                event.remove("process.thread");
                event.remove("syslog");
                event.remove("systemd");
                event.remove("message_id");
                // Begin nested pipeline: "message"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        if let Some(input) = event.get_string("message") {
                            // Grok pattern: ^%{DATA:system.auth.ssh.event} %{DATA:system.auth.ssh.method} for (invalid user)?%{DATA:user.name} from %{IPORHOST:source.address} port %{NUMBER:source.port:long} ssh2(: %{GREEDYDATA:system.auth.ssh.signature})?
                            // Grok pattern: ^%{DATA:system.auth.ssh.event} user %{DATA:user.name} from %{IPORHOST:source.address}
                            // Grok pattern: ^Did not receive identification string from %{IPORHOST:system.auth.ssh.dropped_ip}
                            // Grok pattern: ^%{DATA:user.name} :(?: ((?!TTY=)%{DATA:system.auth.sudo.error}) ;)?(?: TTY=%{DATA:system.auth.sudo.tty} ;)? PWD=%{DATA:system.auth.sudo.pwd} ; USER=%{DATA:system.auth.sudo.user} ; COMMAND=%{GREEDYDATA:system.auth.sudo.command}
                            // Grok pattern: ^new group: name=%{DATA:group.name}, GID=%{NUMBER:group.id}
                            // Grok pattern: ^new user: name=%{DATA:user.target.name}, UID=%{NUMBER:user.target.id}, GID=%{NUMBER:group.id}, home=%{DATA:system.auth.useradd.home}, shell=%{DATA:system.auth.useradd.shell}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{DATA:system.auth.ssh.event} %{DATA:system.auth.ssh.method} for (invalid user)?%{DATA:user.name} from %{IPORHOST:source.address} port %{NUMBER:source.port:long} ssh2(: %{GREEDYDATA:system.auth.ssh.signature})?"
                                    ),
                                    cached_grok!(
                                        "^%{DATA:system.auth.ssh.event} user %{DATA:user.name} from %{IPORHOST:source.address}"
                                    ),
                                    cached_grok!(
                                        "^Did not receive identification string from %{IPORHOST:system.auth.ssh.dropped_ip}"
                                    ),
                                    cached_grok!(
                                        "^%{DATA:user.name} :(?: ((?!TTY=)%{DATA:system.auth.sudo.error}) ;)?(?: TTY=%{DATA:system.auth.sudo.tty} ;)? PWD=%{DATA:system.auth.sudo.pwd} ; USER=%{DATA:system.auth.sudo.user} ; COMMAND=%{GREEDYDATA:system.auth.sudo.command}"
                                    ),
                                    cached_grok!(
                                        "^new group: name=%{DATA:group.name}, GID=%{NUMBER:group.id}"
                                    ),
                                    cached_grok!(
                                        "^new user: name=%{DATA:user.target.name}, UID=%{NUMBER:user.target.id}, GID=%{NUMBER:group.id}, home=%{DATA:system.auth.useradd.home}, shell=%{DATA:system.auth.useradd.shell}$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("user.target.name") };
                if _cond {
                    if let Some(v) = event
                        .get("user.target.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = { event.has_value("user.target.id") };
                if _cond {
                    if let Some(v) = event
                        .get("user.target.id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = { event.has_value("system.auth.sudo.command") };
                if _cond {
                    event.append_unique("event.category", json!("process"))?;
                }
                let _cond = { event.has_value("system.auth.sudo.command") };
                if _cond {
                    event.append_unique("event.type", json!("start"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: for user %{USERNAME:_temp.pam_user}
                                if !cached_grok!("for user %{USERNAME:_temp.pam_user}")
                                    .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: for user \\(?%{USERNAME:_temp.pam_user}\\)?
                                if !cached_grok!("for user \\(?%{USERNAME:_temp.pam_user}\\)?")
                                    .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                if let Some(v) = event
                    .get("log.syslog.appname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("process.name") {
                        event.set("process.name", v)?;
                    }
                }
                let _cond = { event.get_str("process.name") == Some("gpasswd") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: user %{USERNAME:_temp.gpasswd_target} %{WORD:_temp.gpasswd_action} by %{USERNAME:_temp.pam_user} (?:to|from) group %{DATA:group.name}$
                                if !cached_grok!("user %{USERNAME:_temp.gpasswd_target} %{WORD:_temp.gpasswd_action} by %{USERNAME:_temp.pam_user} (?:to|from) group %{DATA:group.name}$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("process.name") == Some("usermod") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: ^add '%{USERNAME:_temp.usermod_user}' to (?:shadow )?group '%{DATA:_temp.usermod_group}'
                                // Grok pattern: user '%{USERNAME:_temp.usermod_user}'
                                if !extract_first_match(
                                    &[
                                        cached_grok!(
                                            "^add '%{USERNAME:_temp.usermod_user}' to (?:shadow )?group '%{DATA:_temp.usermod_group}'"
                                        ),
                                        cached_grok!("user '%{USERNAME:_temp.usermod_user}'"),
                                    ],
                                    &input,
                                    event,
                                )? {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("process.name") == Some("userdel") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: ^delete '%{USERNAME:_temp.userdel_user}' from (?:shadow )?group '%{DATA:_temp.userdel_group}'$
                                // Grok pattern: ^delete user '%{USERNAME:_temp.userdel_user}'$
                                // Grok pattern: ^removed (?:shadow )?group '%{DATA:_temp.userdel_group}'
                                if !extract_first_match(
                                    &[
                                        cached_grok!(
                                            "^delete '%{USERNAME:_temp.userdel_user}' from (?:shadow )?group '%{DATA:_temp.userdel_group}'$"
                                        ),
                                        cached_grok!(
                                            "^delete user '%{USERNAME:_temp.userdel_user}'$"
                                        ),
                                        cached_grok!(
                                            "^removed (?:shadow )?group '%{DATA:_temp.userdel_group}'"
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("message")
                        && event.get_str("message") != Some("")
                        && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        }))
                        && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        }))
                        && (!event.has_value("process.name")
                            || !(["usermod", "userdel"]
                                .contains(&event.get_str("process.name").unwrap_or(""))))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: for user (?:['\"])?%{DATA:_temp.foruser}(?:['\"])? by (?:['\"])?%{DATA:_temp.byuser}(?:['\"])?(?:\\(uid=%{NUMBER:_temp.byuid}\\))?$
                                // Grok pattern: for user (?:['\"])?%{DATA:_temp.foruser}(?:['\"])?$
                                // Grok pattern: by user (?:['\"])?%{DATA:_temp.byuser}(?:['\"])?$
                                // Grok pattern: (?:(?<! )) user (?:['\"])%{DATA:_temp.user}(?:['\"])
                                if !extract_first_match(
                                    &[
                                        cached_grok!(
                                            "for user (?:['\"])?%{DATA:_temp.foruser}(?:['\"])? by (?:['\"])?%{DATA:_temp.byuser}(?:['\"])?(?:\\(uid=%{NUMBER:_temp.byuid}\\))?$"
                                        ),
                                        cached_grok!(
                                            "for user (?:['\"])?%{DATA:_temp.foruser}(?:['\"])?$"
                                        ),
                                        cached_grok!(
                                            "by user (?:['\"])?%{DATA:_temp.byuser}(?:['\"])?$"
                                        ),
                                        cached_grok!(
                                            "(?:(?<! )) user (?:['\"])%{DATA:_temp.user}(?:['\"])"
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_unix("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_unix("),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: ^pam_unix(%{DATA}:%{WORD:_temp.category})
                                if !cached_grok!("^pam_unix(%{DATA}:%{WORD:_temp.category})")
                                    .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("_temp.category") == Some("chauthtok") };
                if _cond {
                    event.append_unique("event.category", json!("iam"))?;
                }
                let _cond = { event.get_str("_temp.category") == Some("chauthtok") };
                if _cond {
                    event.append_unique("event.type", json!("change"))?;
                }
                let _cond = {
                    event.get_str("_temp.category") == Some("chauthtok")
                        && event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("password changed"))
                            }
                            serde_json::Value::String(s) => s.contains("password changed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("_temp.category") == Some("chauthtok")
                        && event.has_value("message")
                        && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("password changed"))
                            }
                            serde_json::Value::String(s) => s.contains("password changed"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = { event.get_str("_temp.category") == Some("chauthtok") };
                if _cond {
                    event.set("event.action", json!("password-changed"))?;
                }
                let _cond = { event.has_value("_temp.gpasswd_target") };
                if _cond {
                    event.append_unique("event.category", json!("iam"))?;
                }
                let _cond = { event.has_value("_temp.gpasswd_target") };
                if _cond {
                    event.append_unique("event.type", json!("change"))?;
                }
                let _cond = { event.has_value("_temp.gpasswd_target") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("_temp.gpasswd_action") == Some("added") };
                if _cond {
                    event.set("event.action", json!("group-user-added"))?;
                }
                let _cond = { event.get_str("_temp.gpasswd_action") == Some("removed") };
                if _cond {
                    event.set("event.action", json!("group-user-removed"))?;
                }
                let _cond = { event.has_value("_temp.usermod_user") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.usermod_user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.target.name", v)?;
                    }
                }
                let _cond = { event.has_value("_temp.usermod_user") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.usermod_user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = { event.has_value("_temp.usermod_group") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.usermod_group")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("group.name", v)?;
                    }
                }
                let _cond = { event.has_value("_temp.usermod_group") };
                if _cond {
                    event.set("event.action", json!("group-member-added"))?;
                }
                let _cond = { event.has_value("_temp.userdel_user") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.userdel_user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.target.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("_temp.userdel_user")
                        && event.has_value("message")
                        && event
                            .get_str("message")
                            .is_some_and(|s| s.starts_with("delete user '"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.userdel_user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = { event.has_value("_temp.userdel_group") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.userdel_group")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("group.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("_temp.userdel_user") && event.has_value("_temp.userdel_group")
                };
                if _cond {
                    event.set("event.action", json!("group-member-removed"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("rhost="))
                            }
                            serde_json::Value::String(s) => s.contains("rhost="),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: rhost=%{IPORHOST:_temp.pam_rhost}%{SPACE}(?:user=%{DATA:_temp.pam_user})?$
                                // Grok pattern: rhost=%{SPACE}user=%{DATA:_temp.pam_user}$
                                if !extract_first_match(
                                    &[
                                        cached_grok!(
                                            "rhost=%{IPORHOST:_temp.pam_rhost}%{SPACE}(?:user=%{DATA:_temp.pam_user})?$"
                                        ),
                                        cached_grok!("rhost=%{SPACE}user=%{DATA:_temp.pam_user}$"),
                                    ],
                                    &input,
                                    event,
                                )? {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("password changed for"))
                            }
                            serde_json::Value::String(s) => s.contains("password changed for"),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: password changed for %{USERNAME:_temp.pam_target}$
                                if !cached_grok!(
                                    "password changed for %{USERNAME:_temp.pam_target}$"
                                )
                                .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("syslog5424_sd") && event.get_str("syslog5424_sd") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("syslog5424_sd") {
                            let mut kv_gap = false;
                            for pair in cached_regex!("(?<=\"); ").split(&kv_str).into_iter() {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = ({
                                    let parts = cached_regex!("(?i)(?<=[a-z]):{1,2}(?=\")")
                                        .splitn(&pair, 2);
                                    match (parts.first(), parts.get(1)) {
                                        (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                        _ => None,
                                    }
                                })
                                .filter(|_| !kv_gap) else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "syslog5424_sd".into(),
                                        split: "(?i)(?<=[a-z]):{1,2}(?=\")".into(),
                                    });
                                };
                                {
                                    let key = &key[..];
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
                                    let value = value
                                        .as_str()
                                        .strip_prefix(['(', '[', '<', '"', '\''])
                                        .unwrap_or(value.as_str());
                                    let value = value
                                        .strip_suffix([']', ')', '>', '"', '\''])
                                        .unwrap_or(value);
                                    let value = value.trim_matches(|c: char| matches!(c, ' '));
                                    if !key.is_empty() {
                                        kv_put(event, &format!("system.auth.{}", key), value)?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "kv")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "kv_syslog_structured_semicolon_colon",
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
                let _cond = { !event.has_value("system.auth") && event.has_value("syslog5424_sd") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) = event.get_string("syslog5424_sd") {
                            // Grok pattern: (?:%{NOTSPACE} +)?%{GREEDYDATA:syslog5424_sd}
                            if !cached_grok!("(?:%{NOTSPACE} +)?%{GREEDYDATA:syslog5424_sd}")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "grok")?;
                        event.set("_ingest.on_failure_processor_tag", "grok_syslog5424_sd")?;
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
                let _cond = { !event.has_value("system.auth") && event.has_value("syslog5424_sd") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("syslog5424_sd") {
                            let mut kv_gap = false;
                            for pair in cached_regex!("(?<=\") ").split(&kv_str).into_iter() {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = ({
                                    let parts =
                                        cached_regex!("(?i)(?<=[a-z])=(?=\")").splitn(&pair, 2);
                                    match (parts.first(), parts.get(1)) {
                                        (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                        _ => None,
                                    }
                                })
                                .filter(|_| !kv_gap) else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "syslog5424_sd".into(),
                                        split: "(?i)(?<=[a-z])=(?=\")".into(),
                                    });
                                };
                                {
                                    let key = &key[..];
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
                                    let value = value
                                        .as_str()
                                        .strip_prefix(['(', '[', '<', '"', '\''])
                                        .unwrap_or(value.as_str());
                                    let value = value
                                        .strip_suffix([']', ')', '>', '"', '\''])
                                        .unwrap_or(value);
                                    let value = value.trim_matches(|c: char| matches!(c, ' '));
                                    if !key.is_empty() {
                                        kv_put(event, &format!("system.auth.{}", key), value)?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "kv")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "kv_syslog_structured_space_equals",
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
                let _cond = { event.has_value("system.auth") };
                if _cond {
                    if event.has_value("system.auth") {
                        foreach_array(event, "system.auth", |event| {
                            map_strings(event, "_ingest._key", "_ingest._key", str::to_lowercase)?;
                            Ok(())
                        })?;
                    }
                }
                let _cond = { event.get_str("_temp.category") == Some("session") };
                if _cond {
                    event.append_unique("event.category", json!("session"))?;
                }
                let _cond = { event.get_str("_temp.category") == Some("auth") };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.get_str("_temp.category") == Some("auth")
                        && event.has_value("message")
                        && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("authentication_failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.type", json!("info"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("account-locked"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.type", json!("info"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.type", json!("info"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("authentication_failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("session opened"))
                            }
                            serde_json::Value::String(s) => s.contains("session opened"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("logged-on"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("session closed"))
                            }
                            serde_json::Value::String(s) => s.contains("session closed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("logged-off"))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_temp.byuser") {
                        event.rename("_temp.byuser", "user.name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_temp.byuid") {
                        event.rename("_temp.byuid", "user.id")?;
                    }
                    Ok(())
                })();
                let _cond =
                    { !event.has_value("user.name") || event.get_str("user.name") == Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.foruser") {
                            event.rename("_temp.foruser", "user.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond =
                    { !event.has_value("user.name") || event.get_str("user.name") == Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.user") {
                            event.rename("_temp.user", "user.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.foruser") {
                            event.rename("_temp.foruser", "user.effective.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond =
                    { event.has_value("_temp.pam_rhost") && !event.has_value("source.address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.pam_rhost") {
                            event.rename("_temp.pam_rhost", "source.address")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("_temp.pam_user")
                        && event.get_str("_temp.pam_user") != Some("")
                        && (!event.has_value("user.name") || event.get_str("user.name") == Some(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.pam_user") {
                            event.rename("_temp.pam_user", "user.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("_temp.pam_target")
                        && event.get_str("_temp.pam_target") != Some("")
                        && (!event.has_value("user.name") || event.get_str("user.name") == Some(""))
                };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.pam_target")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = {
                    event.get_str("_temp.category") == Some("chauthtok")
                        && event.has_value("user.name")
                        && event.get_str("user.name") != Some("")
                };
                if _cond {
                    if let Some(v) = event
                        .get("user.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.target.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("_temp.pam_target")
                        && event.get_str("_temp.pam_target") != Some("")
                        && (!event.has_value("user.target.name")
                            || event.get_str("user.target.name") == Some(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.pam_target") {
                            event.rename("_temp.pam_target", "user.target.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("_temp.gpasswd_target")
                        && event.get_str("_temp.gpasswd_target") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.gpasswd_target") {
                            event.rename("_temp.gpasswd_target", "user.target.name")?;
                        }
                        Ok(())
                    })();
                }
                event.remove("_temp");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
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
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_source-address")?;
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event.get("source.address").cloned() {
                            event.set("source.domain", v)?;
                        }
                        Ok(())
                    })();
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = { event.has_value("system.auth.sudo.user") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(val) = event.get("system.auth.sudo.user") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "system.auth.sudo.user".into(),
                                    message,
                                }
                            })?;
                            event.set("user.effective.name", converted)?;
                        }
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("system.auth.ssh.dropped_ip") {
                        if let Some(val) = event.get("system.auth.ssh.dropped_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "system.auth.ssh.dropped_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dropped-ip")?;
                    if event.remove("system.auth.ssh.dropped_ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "system.auth.ssh.dropped_ip".into(),
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
                    !event.has_value("event.timezone") && event.has_value("system.auth.timestamp")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("system.auth.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "system.auth.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_timestamp_notz")?;
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
                    event.has_value("event.timezone") && event.has_value("system.auth.timestamp")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("system.auth.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "ISO8601"],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "system.auth.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_timestamp_tz")?;
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
                event.remove("system.auth.timestamp");
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
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("system.auth.ssh.event") };
                if _cond {
                    // Painless script
                    // Source: if (ctx.system.auth.ssh.event == \"Accepted\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\", \"session\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"success\";\n} else if (ctx.system.auth.ssh.event == \"Invalid\" || ctx.system.auth.ssh.event == \"Failed\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"failure\";\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.system.auth.ssh.event == \"Accepted\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\", \"session\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"success\";\n} else if (ctx.system.auth.ssh.event == \"Invalid\" || ctx.system.auth.ssh.event == \"Failed\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"failure\";\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && [
                            "groupadd", "groupdel", "groupmod", "useradd", "userdel", "usermod",
                        ]
                        .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("iam"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && (!event.has_value("message")
                            || !(event.get("message").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("fail"))
                                }
                                serde_json::Value::String(s) => s.contains("fail"),
                                _ => false,
                            })))
                        && [
                            "groupadd", "groupdel", "groupmod", "useradd", "userdel", "usermod",
                        ]
                        .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && (event.has_value("message")
                            && event.get("message").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("fail"))
                                }
                                serde_json::Value::String(s) => s.contains("fail"),
                                _ => false,
                            }))
                        && [
                            "groupadd", "groupdel", "groupmod", "useradd", "userdel", "usermod",
                        ]
                        .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["useradd", "userdel", "usermod"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("user"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["groupadd", "groupdel", "groupmod"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("group"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["useradd", "groupadd"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["userdel", "groupdel"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["usermod", "groupmod"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond =
                    { event.has_value("user.name") && event.get_str("user.name") != Some("") };
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
                    event.has_value("user.target.name")
                        && event.get_str("user.target.name") != Some("")
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
                let _cond =
                    { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
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
                let _cond = {
                    event.has_value("host.hostname") && event.get_str("host.hostname") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.set("ecs.version", json!("8.11.0"))?;
                // End nested pipeline: "message"
                // End nested pipeline: "journald"
            }

            let _cond = { event.get_str("input.type") == Some("log") };
            if _cond {
                // Begin nested pipeline: "log"
                let _cond = { !event.has_value("log.syslog") };
                if _cond {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: ^<%{NONNEGINT:log.syslog.priority:int}>(?:%{NONNEGINT:system.auth.syslog.version} )?+(?:-|(?P<system_auth_timestamp>(?:(?:%{TIMESTAMP_ISO8601}|%{SYSLOGTIMESTAMP})))) +(?:-|%{IPORHOST:host.hostname}) +(?:-|%{SYSLOG5424PRINTASCII:process.name}) +(?:-|%{POSINT:process.pid:long}) +(?:-|%{SYSLOG5424PRINTASCII:event.code}) +(?:-|%{SYSLOG5424SD:syslog5424_sd})? +%{GREEDYDATA:message}$
                        // Grok pattern: ^(?P<system_auth_timestamp>(?:(?:%{TIMESTAMP_ISO8601}|%{SYSLOGTIMESTAMP}))) %{SYSLOGHOST:host.hostname}? %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?:%{SPACE}(?P<message>(?:(.|\\n)*))$
                        // Grok pattern: ^<%{NONNEGINT:log.syslog.priority:int}>(?:%{NONNEGINT:system.auth.syslog.version} )?(?P<system_auth_timestamp>(?:(?:%{TIMESTAMP_ISO8601}|%{SYSLOGTIMESTAMP}))) %{SYSLOGHOST:host.hostname}? %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?:%{SPACE}(?P<message>(?:(.|\\n)*))$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^<%{NONNEGINT:log.syslog.priority:int}>(?:%{NONNEGINT:system.auth.syslog.version} )?+(?:-|(?P<system_auth_timestamp>(?:(?:%{TIMESTAMP_ISO8601}|%{SYSLOGTIMESTAMP})))) +(?:-|%{IPORHOST:host.hostname}) +(?:-|%{SYSLOG5424PRINTASCII:process.name}) +(?:-|%{POSINT:process.pid:long}) +(?:-|%{SYSLOG5424PRINTASCII:event.code}) +(?:-|%{SYSLOG5424SD:syslog5424_sd})? +%{GREEDYDATA:message}$",
                                    [("system_auth_timestamp", "system.auth.timestamp")]
                                ),
                                cached_grok_mapped!(
                                    "^(?P<system_auth_timestamp>(?:(?:%{TIMESTAMP_ISO8601}|%{SYSLOGTIMESTAMP}))) %{SYSLOGHOST:host.hostname}? %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?:%{SPACE}(?P<message>(?:(.|\\n)*))$",
                                    [("system_auth_timestamp", "system.auth.timestamp")]
                                ),
                                cached_grok_mapped!(
                                    "^<%{NONNEGINT:log.syslog.priority:int}>(?:%{NONNEGINT:system.auth.syslog.version} )?(?P<system_auth_timestamp>(?:(?:%{TIMESTAMP_ISO8601}|%{SYSLOGTIMESTAMP}))) %{SYSLOGHOST:host.hostname}? %{DATA:process.name}(?:\\[%{POSINT:process.pid:long}\\])?:%{SPACE}(?P<message>(?:(.|\\n)*))$",
                                    [("system_auth_timestamp", "system.auth.timestamp")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                // Begin nested pipeline: "message"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        if let Some(input) = event.get_string("message") {
                            // Grok pattern: ^%{DATA:system.auth.ssh.event} %{DATA:system.auth.ssh.method} for (invalid user)?%{DATA:user.name} from %{IPORHOST:source.address} port %{NUMBER:source.port:long} ssh2(: %{GREEDYDATA:system.auth.ssh.signature})?
                            // Grok pattern: ^%{DATA:system.auth.ssh.event} user %{DATA:user.name} from %{IPORHOST:source.address}
                            // Grok pattern: ^Did not receive identification string from %{IPORHOST:system.auth.ssh.dropped_ip}
                            // Grok pattern: ^%{DATA:user.name} :(?: ((?!TTY=)%{DATA:system.auth.sudo.error}) ;)?(?: TTY=%{DATA:system.auth.sudo.tty} ;)? PWD=%{DATA:system.auth.sudo.pwd} ; USER=%{DATA:system.auth.sudo.user} ; COMMAND=%{GREEDYDATA:system.auth.sudo.command}
                            // Grok pattern: ^new group: name=%{DATA:group.name}, GID=%{NUMBER:group.id}
                            // Grok pattern: ^new user: name=%{DATA:user.target.name}, UID=%{NUMBER:user.target.id}, GID=%{NUMBER:group.id}, home=%{DATA:system.auth.useradd.home}, shell=%{DATA:system.auth.useradd.shell}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{DATA:system.auth.ssh.event} %{DATA:system.auth.ssh.method} for (invalid user)?%{DATA:user.name} from %{IPORHOST:source.address} port %{NUMBER:source.port:long} ssh2(: %{GREEDYDATA:system.auth.ssh.signature})?"
                                    ),
                                    cached_grok!(
                                        "^%{DATA:system.auth.ssh.event} user %{DATA:user.name} from %{IPORHOST:source.address}"
                                    ),
                                    cached_grok!(
                                        "^Did not receive identification string from %{IPORHOST:system.auth.ssh.dropped_ip}"
                                    ),
                                    cached_grok!(
                                        "^%{DATA:user.name} :(?: ((?!TTY=)%{DATA:system.auth.sudo.error}) ;)?(?: TTY=%{DATA:system.auth.sudo.tty} ;)? PWD=%{DATA:system.auth.sudo.pwd} ; USER=%{DATA:system.auth.sudo.user} ; COMMAND=%{GREEDYDATA:system.auth.sudo.command}"
                                    ),
                                    cached_grok!(
                                        "^new group: name=%{DATA:group.name}, GID=%{NUMBER:group.id}"
                                    ),
                                    cached_grok!(
                                        "^new user: name=%{DATA:user.target.name}, UID=%{NUMBER:user.target.id}, GID=%{NUMBER:group.id}, home=%{DATA:system.auth.useradd.home}, shell=%{DATA:system.auth.useradd.shell}$"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("user.target.name") };
                if _cond {
                    if let Some(v) = event
                        .get("user.target.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = { event.has_value("user.target.id") };
                if _cond {
                    if let Some(v) = event
                        .get("user.target.id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = { event.has_value("system.auth.sudo.command") };
                if _cond {
                    event.append_unique("event.category", json!("process"))?;
                }
                let _cond = { event.has_value("system.auth.sudo.command") };
                if _cond {
                    event.append_unique("event.type", json!("start"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: for user %{USERNAME:_temp.pam_user}
                                if !cached_grok!("for user %{USERNAME:_temp.pam_user}")
                                    .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: for user \\(?%{USERNAME:_temp.pam_user}\\)?
                                if !cached_grok!("for user \\(?%{USERNAME:_temp.pam_user}\\)?")
                                    .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                if let Some(v) = event
                    .get("log.syslog.appname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("process.name") {
                        event.set("process.name", v)?;
                    }
                }
                let _cond = { event.get_str("process.name") == Some("gpasswd") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: user %{USERNAME:_temp.gpasswd_target} %{WORD:_temp.gpasswd_action} by %{USERNAME:_temp.pam_user} (?:to|from) group %{DATA:group.name}$
                                if !cached_grok!("user %{USERNAME:_temp.gpasswd_target} %{WORD:_temp.gpasswd_action} by %{USERNAME:_temp.pam_user} (?:to|from) group %{DATA:group.name}$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("process.name") == Some("usermod") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: ^add '%{USERNAME:_temp.usermod_user}' to (?:shadow )?group '%{DATA:_temp.usermod_group}'
                                // Grok pattern: user '%{USERNAME:_temp.usermod_user}'
                                if !extract_first_match(
                                    &[
                                        cached_grok!(
                                            "^add '%{USERNAME:_temp.usermod_user}' to (?:shadow )?group '%{DATA:_temp.usermod_group}'"
                                        ),
                                        cached_grok!("user '%{USERNAME:_temp.usermod_user}'"),
                                    ],
                                    &input,
                                    event,
                                )? {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("process.name") == Some("userdel") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: ^delete '%{USERNAME:_temp.userdel_user}' from (?:shadow )?group '%{DATA:_temp.userdel_group}'$
                                // Grok pattern: ^delete user '%{USERNAME:_temp.userdel_user}'$
                                // Grok pattern: ^removed (?:shadow )?group '%{DATA:_temp.userdel_group}'
                                if !extract_first_match(
                                    &[
                                        cached_grok!(
                                            "^delete '%{USERNAME:_temp.userdel_user}' from (?:shadow )?group '%{DATA:_temp.userdel_group}'$"
                                        ),
                                        cached_grok!(
                                            "^delete user '%{USERNAME:_temp.userdel_user}'$"
                                        ),
                                        cached_grok!(
                                            "^removed (?:shadow )?group '%{DATA:_temp.userdel_group}'"
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("message")
                        && event.get_str("message") != Some("")
                        && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        }))
                        && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        }))
                        && (!event.has_value("process.name")
                            || !(["usermod", "userdel"]
                                .contains(&event.get_str("process.name").unwrap_or(""))))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: for user (?:['\"])?%{DATA:_temp.foruser}(?:['\"])? by (?:['\"])?%{DATA:_temp.byuser}(?:['\"])?(?:\\(uid=%{NUMBER:_temp.byuid}\\))?$
                                // Grok pattern: for user (?:['\"])?%{DATA:_temp.foruser}(?:['\"])?$
                                // Grok pattern: by user (?:['\"])?%{DATA:_temp.byuser}(?:['\"])?$
                                // Grok pattern: (?:(?<! )) user (?:['\"])%{DATA:_temp.user}(?:['\"])
                                if !extract_first_match(
                                    &[
                                        cached_grok!(
                                            "for user (?:['\"])?%{DATA:_temp.foruser}(?:['\"])? by (?:['\"])?%{DATA:_temp.byuser}(?:['\"])?(?:\\(uid=%{NUMBER:_temp.byuid}\\))?$"
                                        ),
                                        cached_grok!(
                                            "for user (?:['\"])?%{DATA:_temp.foruser}(?:['\"])?$"
                                        ),
                                        cached_grok!(
                                            "by user (?:['\"])?%{DATA:_temp.byuser}(?:['\"])?$"
                                        ),
                                        cached_grok!(
                                            "(?:(?<! )) user (?:['\"])%{DATA:_temp.user}(?:['\"])"
                                        ),
                                    ],
                                    &input,
                                    event,
                                )? {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_unix("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_unix("),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: ^pam_unix(%{DATA}:%{WORD:_temp.category})
                                if !cached_grok!("^pam_unix(%{DATA}:%{WORD:_temp.category})")
                                    .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("_temp.category") == Some("chauthtok") };
                if _cond {
                    event.append_unique("event.category", json!("iam"))?;
                }
                let _cond = { event.get_str("_temp.category") == Some("chauthtok") };
                if _cond {
                    event.append_unique("event.type", json!("change"))?;
                }
                let _cond = {
                    event.get_str("_temp.category") == Some("chauthtok")
                        && event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("password changed"))
                            }
                            serde_json::Value::String(s) => s.contains("password changed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("_temp.category") == Some("chauthtok")
                        && event.has_value("message")
                        && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("password changed"))
                            }
                            serde_json::Value::String(s) => s.contains("password changed"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = { event.get_str("_temp.category") == Some("chauthtok") };
                if _cond {
                    event.set("event.action", json!("password-changed"))?;
                }
                let _cond = { event.has_value("_temp.gpasswd_target") };
                if _cond {
                    event.append_unique("event.category", json!("iam"))?;
                }
                let _cond = { event.has_value("_temp.gpasswd_target") };
                if _cond {
                    event.append_unique("event.type", json!("change"))?;
                }
                let _cond = { event.has_value("_temp.gpasswd_target") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("_temp.gpasswd_action") == Some("added") };
                if _cond {
                    event.set("event.action", json!("group-user-added"))?;
                }
                let _cond = { event.get_str("_temp.gpasswd_action") == Some("removed") };
                if _cond {
                    event.set("event.action", json!("group-user-removed"))?;
                }
                let _cond = { event.has_value("_temp.usermod_user") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.usermod_user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.target.name", v)?;
                    }
                }
                let _cond = { event.has_value("_temp.usermod_user") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.usermod_user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = { event.has_value("_temp.usermod_group") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.usermod_group")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("group.name", v)?;
                    }
                }
                let _cond = { event.has_value("_temp.usermod_group") };
                if _cond {
                    event.set("event.action", json!("group-member-added"))?;
                }
                let _cond = { event.has_value("_temp.userdel_user") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.userdel_user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.target.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("_temp.userdel_user")
                        && event.has_value("message")
                        && event
                            .get_str("message")
                            .is_some_and(|s| s.starts_with("delete user '"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.userdel_user")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = { event.has_value("_temp.userdel_group") };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.userdel_group")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("group.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("_temp.userdel_user") && event.has_value("_temp.userdel_group")
                };
                if _cond {
                    event.set("event.action", json!("group-member-removed"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("rhost="))
                            }
                            serde_json::Value::String(s) => s.contains("rhost="),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: rhost=%{IPORHOST:_temp.pam_rhost}%{SPACE}(?:user=%{DATA:_temp.pam_user})?$
                                // Grok pattern: rhost=%{SPACE}user=%{DATA:_temp.pam_user}$
                                if !extract_first_match(
                                    &[
                                        cached_grok!(
                                            "rhost=%{IPORHOST:_temp.pam_rhost}%{SPACE}(?:user=%{DATA:_temp.pam_user})?$"
                                        ),
                                        cached_grok!("rhost=%{SPACE}user=%{DATA:_temp.pam_user}$"),
                                    ],
                                    &input,
                                    event,
                                )? {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("password changed for"))
                            }
                            serde_json::Value::String(s) => s.contains("password changed for"),
                            _ => false,
                        })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: password changed for %{USERNAME:_temp.pam_target}$
                                if !cached_grok!(
                                    "password changed for %{USERNAME:_temp.pam_target}$"
                                )
                                .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("syslog5424_sd") && event.get_str("syslog5424_sd") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("syslog5424_sd") {
                            let mut kv_gap = false;
                            for pair in cached_regex!("(?<=\"); ").split(&kv_str).into_iter() {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = ({
                                    let parts = cached_regex!("(?i)(?<=[a-z]):{1,2}(?=\")")
                                        .splitn(&pair, 2);
                                    match (parts.first(), parts.get(1)) {
                                        (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                        _ => None,
                                    }
                                })
                                .filter(|_| !kv_gap) else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "syslog5424_sd".into(),
                                        split: "(?i)(?<=[a-z]):{1,2}(?=\")".into(),
                                    });
                                };
                                {
                                    let key = &key[..];
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
                                    let value = value
                                        .as_str()
                                        .strip_prefix(['(', '[', '<', '"', '\''])
                                        .unwrap_or(value.as_str());
                                    let value = value
                                        .strip_suffix([']', ')', '>', '"', '\''])
                                        .unwrap_or(value);
                                    let value = value.trim_matches(|c: char| matches!(c, ' '));
                                    if !key.is_empty() {
                                        kv_put(event, &format!("system.auth.{}", key), value)?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "kv")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "kv_syslog_structured_semicolon_colon",
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
                let _cond = { !event.has_value("system.auth") && event.has_value("syslog5424_sd") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) = event.get_string("syslog5424_sd") {
                            // Grok pattern: (?:%{NOTSPACE} +)?%{GREEDYDATA:syslog5424_sd}
                            if !cached_grok!("(?:%{NOTSPACE} +)?%{GREEDYDATA:syslog5424_sd}")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "grok")?;
                        event.set("_ingest.on_failure_processor_tag", "grok_syslog5424_sd")?;
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
                let _cond = { !event.has_value("system.auth") && event.has_value("syslog5424_sd") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("syslog5424_sd") {
                            let mut kv_gap = false;
                            for pair in cached_regex!("(?<=\") ").split(&kv_str).into_iter() {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = ({
                                    let parts =
                                        cached_regex!("(?i)(?<=[a-z])=(?=\")").splitn(&pair, 2);
                                    match (parts.first(), parts.get(1)) {
                                        (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                        _ => None,
                                    }
                                })
                                .filter(|_| !kv_gap) else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "syslog5424_sd".into(),
                                        split: "(?i)(?<=[a-z])=(?=\")".into(),
                                    });
                                };
                                {
                                    let key = &key[..];
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
                                    let value = value
                                        .as_str()
                                        .strip_prefix(['(', '[', '<', '"', '\''])
                                        .unwrap_or(value.as_str());
                                    let value = value
                                        .strip_suffix([']', ')', '>', '"', '\''])
                                        .unwrap_or(value);
                                    let value = value.trim_matches(|c: char| matches!(c, ' '));
                                    if !key.is_empty() {
                                        kv_put(event, &format!("system.auth.{}", key), value)?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "kv")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "kv_syslog_structured_space_equals",
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
                let _cond = { event.has_value("system.auth") };
                if _cond {
                    if event.has_value("system.auth") {
                        foreach_array(event, "system.auth", |event| {
                            map_strings(event, "_ingest._key", "_ingest._key", str::to_lowercase)?;
                            Ok(())
                        })?;
                    }
                }
                let _cond = { event.get_str("_temp.category") == Some("session") };
                if _cond {
                    event.append_unique("event.category", json!("session"))?;
                }
                let _cond = { event.get_str("_temp.category") == Some("auth") };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.get_str("_temp.category") == Some("auth")
                        && event.has_value("message")
                        && !(event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("authentication_failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("authentication failure")),
                            serde_json::Value::String(s) => s.contains("authentication failure"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.type", json!("info"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("account-locked"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("pam_faillock("))
                            }
                            serde_json::Value::String(s) => s.contains("pam_faillock("),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.type", json!("info"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append_unique("event.type", json!("info"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("password check failed")),
                            serde_json::Value::String(s) => s.contains("password check failed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("authentication_failure"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("session opened"))
                            }
                            serde_json::Value::String(s) => s.contains("session opened"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("logged-on"))?;
                }
                let _cond = {
                    event.has_value("message")
                        && event.get("message").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("session closed"))
                            }
                            serde_json::Value::String(s) => s.contains("session closed"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("logged-off"))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_temp.byuser") {
                        event.rename("_temp.byuser", "user.name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_temp.byuid") {
                        event.rename("_temp.byuid", "user.id")?;
                    }
                    Ok(())
                })();
                let _cond =
                    { !event.has_value("user.name") || event.get_str("user.name") == Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.foruser") {
                            event.rename("_temp.foruser", "user.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond =
                    { !event.has_value("user.name") || event.get_str("user.name") == Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.user") {
                            event.rename("_temp.user", "user.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.foruser") {
                            event.rename("_temp.foruser", "user.effective.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond =
                    { event.has_value("_temp.pam_rhost") && !event.has_value("source.address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.pam_rhost") {
                            event.rename("_temp.pam_rhost", "source.address")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("_temp.pam_user")
                        && event.get_str("_temp.pam_user") != Some("")
                        && (!event.has_value("user.name") || event.get_str("user.name") == Some(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.pam_user") {
                            event.rename("_temp.pam_user", "user.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("_temp.pam_target")
                        && event.get_str("_temp.pam_target") != Some("")
                        && (!event.has_value("user.name") || event.get_str("user.name") == Some(""))
                };
                if _cond {
                    if let Some(v) = event
                        .get("_temp.pam_target")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = {
                    event.get_str("_temp.category") == Some("chauthtok")
                        && event.has_value("user.name")
                        && event.get_str("user.name") != Some("")
                };
                if _cond {
                    if let Some(v) = event
                        .get("user.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.target.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("_temp.pam_target")
                        && event.get_str("_temp.pam_target") != Some("")
                        && (!event.has_value("user.target.name")
                            || event.get_str("user.target.name") == Some(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.pam_target") {
                            event.rename("_temp.pam_target", "user.target.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("_temp.gpasswd_target")
                        && event.get_str("_temp.gpasswd_target") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("_temp.gpasswd_target") {
                            event.rename("_temp.gpasswd_target", "user.target.name")?;
                        }
                        Ok(())
                    })();
                }
                event.remove("_temp");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
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
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_source-address")?;
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event.get("source.address").cloned() {
                            event.set("source.domain", v)?;
                        }
                        Ok(())
                    })();
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = { event.has_value("system.auth.sudo.user") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(val) = event.get("system.auth.sudo.user") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "system.auth.sudo.user".into(),
                                    message,
                                }
                            })?;
                            event.set("user.effective.name", converted)?;
                        }
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("system.auth.ssh.dropped_ip") {
                        if let Some(val) = event.get("system.auth.ssh.dropped_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "system.auth.ssh.dropped_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_dropped-ip")?;
                    if event.remove("system.auth.ssh.dropped_ip").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "system.auth.ssh.dropped_ip".into(),
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
                    !event.has_value("event.timezone") && event.has_value("system.auth.timestamp")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("system.auth.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "ISO8601"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "system.auth.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_timestamp_notz")?;
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
                    event.has_value("event.timezone") && event.has_value("system.auth.timestamp")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("system.auth.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss", "ISO8601"],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "system.auth.timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_timestamp_tz")?;
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
                event.remove("system.auth.timestamp");
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
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("system.auth.ssh.event") };
                if _cond {
                    // Painless script
                    // Source: if (ctx.system.auth.ssh.event == \"Accepted\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\", \"session\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"success\";\n} else if (ctx.system.auth.ssh.event == \"Invalid\" || ctx.system.auth.ssh.event == \"Failed\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"failure\";\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.system.auth.ssh.event == \"Accepted\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\", \"session\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"success\";\n} else if (ctx.system.auth.ssh.event == \"Invalid\" || ctx.system.auth.ssh.event == \"Failed\") {\n  ctx.event.type = [\"info\"];\n  ctx.event.category = [\"authentication\"];\n  ctx.event.action = \"ssh_login\";\n  ctx.event.outcome = \"failure\";\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && [
                            "groupadd", "groupdel", "groupmod", "useradd", "userdel", "usermod",
                        ]
                        .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.category", json!("iam"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && (!event.has_value("message")
                            || !(event.get("message").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("fail"))
                                }
                                serde_json::Value::String(s) => s.contains("fail"),
                                _ => false,
                            })))
                        && [
                            "groupadd", "groupdel", "groupmod", "useradd", "userdel", "usermod",
                        ]
                        .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && (event.has_value("message")
                            && event.get("message").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("fail"))
                                }
                                serde_json::Value::String(s) => s.contains("fail"),
                                _ => false,
                            }))
                        && [
                            "groupadd", "groupdel", "groupmod", "useradd", "userdel", "usermod",
                        ]
                        .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["useradd", "userdel", "usermod"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("user"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["groupadd", "groupdel", "groupmod"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("group"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["useradd", "groupadd"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["userdel", "groupdel"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = {
                    event.has_value("process.name")
                        && ["usermod", "groupmod"]
                            .contains(&event.get_str("process.name").unwrap_or(""))
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond =
                    { event.has_value("user.name") && event.get_str("user.name") != Some("") };
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
                    event.has_value("user.target.name")
                        && event.get_str("user.target.name") != Some("")
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
                let _cond =
                    { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
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
                let _cond = {
                    event.has_value("host.hostname") && event.get_str("host.hostname") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                event.set("ecs.version", json!("8.11.0"))?;
                // End nested pipeline: "message"
                // End nested pipeline: "log"
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.set(
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
