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

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.provider", json!("firewall"))?;

            event.set("observer.vendor", json!("Cisco"))?;

            event.set("observer.product", json!("IOS"))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            if let Some(v) = event.get("message").cloned() {
                if !event.has("event.original") {
                    event.set("event.original", v)?;
                }
            }

            event.remove("message");

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^<%{NONNEGINT:log.syslog.priority:long}>Original Address=(?:%{DATA} ){7}%{GREEDYDATA:_temp_.message}$
                if !cached_grok!("^<%{NONNEGINT:log.syslog.priority:long}>Original Address=(?:%{DATA} ){7}%{GREEDYDATA:_temp_.message}$").extract_into(&input, event)? {
                        // Grok pattern: ^%{GREEDYDATA:_temp_.message}$
                        if !cached_grok!("^%{GREEDYDATA:_temp_.message}$").extract_into(&input, event)? {
                        }
                    }
            }

            if let Some(input) = event.get_string("_temp_.message") {
                // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?P<_temp__cisco_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?(?P<_temp__tz>(?:(?:Z|[+-]%{HOUR}(?::?%{MINUTE}))))?)) (?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?)) %{GREEDYDATA:_temp_.message}$
                if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?P<_temp__cisco_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?(?P<_temp__tz>(?:(?:Z|[+-]%{HOUR}(?::?%{MINUTE}))))?)) (?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?)) %{GREEDYDATA:_temp_.message}$", [("_temp__cisco_timestamp", "_temp_.cisco_timestamp"), ("log_syslog_hostname", "log.syslog.hostname"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                        // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} %{IP} (?:(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?)): )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:(?:\\d{1,4}:\\d{2}:\\d{2}|(?:(\\d+)y)?(?:(\\d+)w)?(?:(\\d+)d)?(?:(\\d+)h)?(?:(\\d+)m)?(?:(\\d+)s)?)))|(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{GREEDYDATA:_temp_.message}$
                        if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} %{IP} (?:(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?)): )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:(?:\\d{1,4}:\\d{2}:\\d{2}|(?:(\\d+)y)?(?:(\\d+)w)?(?:(\\d+)d)?(?:(\\d+)h)?(?:(\\d+)m)?(?:(\\d+)s)?)))|(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{GREEDYDATA:_temp_.message}$", [("log_syslog_hostname", "log.syslog.hostname"), ("cisco_ios_uptime", "cisco.ios.uptime"), ("_temp__cisco_timestamp", "_temp_.cisco_timestamp"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                            // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} (?:%{IP}|(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?))) %{NUMBER:cisco.ios.sequence}: (?:(?P<cisco_ios_uptime>(?:(?:\\d{1,4}:\\d{2}:\\d{2}|(?:(\\d+)y)?(?:(\\d+)w)?(?:(\\d+)d)?(?:(\\d+)h)?(?:(\\d+)m)?(?:(\\d+)s)?)))|(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{GREEDYDATA:_temp_.message}$
                            if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} (?:%{IP}|(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?))) %{NUMBER:cisco.ios.sequence}: (?:(?P<cisco_ios_uptime>(?:(?:\\d{1,4}:\\d{2}:\\d{2}|(?:(\\d+)y)?(?:(\\d+)w)?(?:(\\d+)d)?(?:(\\d+)h)?(?:(\\d+)m)?(?:(\\d+)s)?)))|(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{GREEDYDATA:_temp_.message}$", [("log_syslog_hostname", "log.syslog.hostname"), ("cisco_ios_uptime", "cisco.ios.uptime"), ("_temp__cisco_timestamp", "_temp_.cisco_timestamp"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                                // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?P<_temp__timestamp>(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{GREEDYDATA:_temp_.message}$
                                if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?P<_temp__timestamp>(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): %{GREEDYDATA:_temp_.message}$", [("_temp__timestamp", "_temp_.timestamp"), ("_temp__cisco_timestamp", "_temp_.cisco_timestamp"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                                    // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?:%{SYSLOGTIMESTAMP} )?(%{IP}|(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?))) %{DATA}:(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)%{GREEDYDATA}%%{GREEDYDATA:message}$
                                    if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?:%{SYSLOGTIMESTAMP} )?(%{IP}|(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?))) %{DATA}:(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)%{GREEDYDATA}%%{GREEDYDATA:message}$", [("log_syslog_hostname", "log.syslog.hostname"), ("_temp__cisco_timestamp", "_temp_.cisco_timestamp"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                                        // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?:(?:%{IP}|(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?)))(?:: \\*%{DATA}:|:?)? )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:(?:\\d{1,4}:\\d{2}:\\d{2}|(?:(\\d+)y)?(?:(\\d+)w)?(?:(\\d+)d)?(?:(\\d+)h)?(?:(\\d+)m)?(?:(\\d+)s)?)))|(?P<_temp__timestamp>(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?))): %{GREEDYDATA:_temp_.message}$
                                        if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?:(?:%{IP}|(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?)))(?:: \\*%{DATA}:|:?)? )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:(?:\\d{1,4}:\\d{2}:\\d{2}|(?:(\\d+)y)?(?:(\\d+)w)?(?:(\\d+)d)?(?:(\\d+)h)?(?:(\\d+)m)?(?:(\\d+)s)?)))|(?P<_temp__timestamp>(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?))): %{GREEDYDATA:_temp_.message}$", [("log_syslog_hostname", "log.syslog.hostname"), ("cisco_ios_uptime", "cisco.ios.uptime"), ("_temp__timestamp", "_temp_.timestamp"), ("_temp__cisco_timestamp", "_temp_.cisco_timestamp"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                                            // Grok pattern: ^%{SYSLOGTIMESTAMP} (?:%{IP}|%{HOSTNAME:log.syslog.hostname}) (?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?:%{NUMBER:cisco.ios.sequence}: )(?:(?:(?P<cisco_ios_uptime>(?:(?:\\d{1,4}:\\d{2}:\\d{2}|(?:(\\d+)y)?(?:(\\d+)w)?(?:(\\d+)d)?(?:(\\d+)h)?(?:(\\d+)m)?(?:(\\d+)s)?)))|(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): )?%{GREEDYDATA:_temp_.message}$
                                            if !cached_grok_mapped!("^%{SYSLOGTIMESTAMP} (?:%{IP}|%{HOSTNAME:log.syslog.hostname}) (?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?:%{NUMBER:cisco.ios.sequence}: )(?:(?:(?P<cisco_ios_uptime>(?:(?:\\d{1,4}:\\d{2}:\\d{2}|(?:(\\d+)y)?(?:(\\d+)w)?(?:(\\d+)d)?(?:(\\d+)h)?(?:(\\d+)m)?(?:(\\d+)s)?)))|(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\\d{1,2}|[+-]\\d{2}:\\d{2})?)))?)): )?%{GREEDYDATA:_temp_.message}$", [("cisco_ios_uptime", "cisco.ios.uptime"), ("_temp__cisco_timestamp", "_temp_.cisco_timestamp"), ("_temp__tz", "_temp_.tz")]).extract_into(&input, event)? {
                                                // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} (?:%{IP:log.syslog.hostname}|(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?))) %{GREEDYDATA:_temp_.message}$
                                                if !cached_grok_mapped!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} (?:%{IP:log.syslog.hostname}|(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?))) %{GREEDYDATA:_temp_.message}$", [("log_syslog_hostname", "log.syslog.hostname")]).extract_into(&input, event)? {
                                                    // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)%{GREEDYDATA:_temp_.message}$
                                                    if !cached_grok!("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)%{GREEDYDATA:_temp_.message}$").extract_into(&input, event)? {
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
            }

            if let Some(input) = event.get_string("_temp_.message") {
                // Grok pattern: ^%%{GREEDYDATA:message}$
                if !cached_grok!("^%%{GREEDYDATA:message}$").extract_into(&input, event)? {
                    // Grok pattern: ^%{GREEDYDATA}%%{GREEDYDATA:message}$
                    if !cached_grok!("^%{GREEDYDATA}%%{GREEDYDATA:message}$")
                        .extract_into(&input, event)?
                    {
                        // Grok pattern: ^%{GREEDYDATA:_temp_.generic_message}$
                        if !cached_grok!("^%{GREEDYDATA:_temp_.generic_message}$")
                            .extract_into(&input, event)?
                        {}
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("log.syslog.hostname") {
                    if let Some(input) = event.get_string("log.syslog.hostname") {
                        // Grok pattern: ^%{NUMBER:_temp_.sequence}$
                        if !cached_grok!("^%{NUMBER:_temp_.sequence}$")
                            .extract_into(&input, event)?
                        {}
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("_temp_.sequence") };
            if _cond {
                if let Some(v) = event.get("_temp_.sequence").cloned() {
                    event.set("cisco.ios.sequence", v)?;
                }
            }

            let _cond = { event.has_value("_temp_.sequence") };
            if _cond {
                event.remove("log.syslog.hostname");
            }

            let _cond = { event.has_value("_temp_.sequence") };
            if _cond {
                event.remove("_temp_.sequence");
            }

            let _cond = {
                event.has_value("log.syslog")
                    && event.get("log.syslog").is_none_or(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                event.remove("log.syslog");
            }

            let _cond = { event.has_value("cisco.ios.sequence") };
            if _cond {
                if let Some(v) = event.get("cisco.ios.sequence").cloned() {
                    event.set("event.sequence", v)?;
                }
            }

            let _cond = { event.has_value("cisco.ios.message_count") };
            if _cond {
                if let Some(val) = event.get("cisco.ios.message_count") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "cisco.ios.message_count".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "cisco.ios.message_count".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "cisco.ios.message_count".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("cisco.ios.message_count", converted)?;
                }
            }

            let _cond = {
                event.has_value("cisco.ios.message_count") && !event.has_value("event.sequence")
            };
            if _cond {
                if let Some(v) = event.get("cisco.ios.message_count").cloned() {
                    event.set("event.sequence", v)?;
                }
            }

            if event.has("_temp_.cisco_timestamp") {
                if let Some(s) = event.get_string("_temp_.cisco_timestamp") {
                    let re = cached_regex!("\\s+");
                    let replaced = re.replace_all(&s, " ").into_owned();
                    event.set("_temp_.cisco_timestamp", replaced)?;
                }
            }

            let _cond = { event.has_value("_temp_.cisco_timestamp") };
            if _cond {
                // Painless script
                // Source: String get_timezone(def ctx) {\n  if (ctx._temp_?.tz != null) {\n    if (ctx._conf?.tz_map != null) {\n      for (def item : ctx._conf.tz_map) {\n        if (item.tz_short == ctx._temp_.tz) {\n          return item.tz_long;\n        }\n      }\n    }\n    if (ctx._temp_.tz == 'Z') {\n      return '+00:00';\n    }\n    if (ctx._temp_.tz.length() <= 4) {\n      // all time zone abbreviations need to be uppercase\n      return ctx._temp_.tz.toUpperCase();\n    }\n\n    return ctx._temp_.tz;\n  }\n\n  if (ctx._conf?.tz_offset != null) {\n      ctx.event.timezone = ctx._conf.tz_offset;\n      return ctx._conf.tz_offset;\n  }\n\n  ctx.event.timezone = 'UTC';\n  return 'UTC';\n}\n\ndef event_timezone = get_timezone(ctx);\nif (!event_timezone.contains('+') && !event_timezone.contains('-') && !(event_timezone.length() > 4)) {\n  // timezone abbreviation e.g. CEST need to be put inside the timestamp\n  SimpleDateFormat sdf = new SimpleDateFormat(\"z\");\n  sdf.parse(event_timezone);\n  ctx._temp_.date_timezone = ZoneId.of(sdf.getTimeZone().getID(), ZoneId.SHORT_IDS).getId();\n  ctx?._temp_.cisco_timestamp = ctx?._temp_.cisco_timestamp + \" \" + event_timezone;\n} else {\n  // timezone is either abbreviation+-offset e.g. UTC+1 or long representation\n  // e.g. Europe/Athens needs to be put as a ZoneId and *not* inside the timestamp\n  ctx._temp_.date_timezone = event_timezone;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"String get_timezone(def ctx) {\n  if (ctx._temp_?.tz != null) {\n    if (ctx._conf?.tz_map != null) {\n      for (def item : ctx._conf.tz_map) {\n        if (item.tz_short == ctx._temp_.tz) {\n          return item.tz_long;\n        }\n      }\n    }\n    if (ctx._temp_.tz == 'Z') {\n      return '+00:00';\n    }\n    if (ctx._temp_.tz.length() <= 4) {\n      // all time zone abbreviations need to be uppercase\n      return ctx._temp_.tz.toUpperCase();\n    }\n\n    return ctx._temp_.tz;\n  }\n\n  if (ctx._conf?.tz_offset != null) {\n      ctx.event.timezone = ctx._conf.tz_offset;\n      return ctx._conf.tz_offset;\n  }\n\n  ctx.event.timezone = 'UTC';\n  return 'UTC';\n}\n\ndef event_timezone = get_timezone(ctx);\nif (!event_timezone.contains('+') && !event_timezone.contains('-') && !(event_timezone.length() > 4)) {\n  // timezone abbreviation e.g. CEST need to be put inside the timestamp\n  SimpleDateFormat sdf = new SimpleDateFormat(\"z\");\n  sdf.parse(event_timezone);\n  ctx._temp_.date_timezone = ZoneId.of(sdf.getTimeZone().getID(), ZoneId.SHORT_IDS).getId();\n  ctx?._temp_.cisco_timestamp = ctx?._temp_.cisco_timestamp + \" \" + event_timezone;\n} else {\n  // timezone is either abbreviation+-offset e.g. UTC+1 or long representation\n  // e.g. Europe/Athens needs to be put as a ZoneId and *not* inside the timestamp\n  ctx._temp_.date_timezone = event_timezone;\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.cisco_timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_temp_.cisco_timestamp") {
                    if let Some(parsed) = parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "MMM d yyyy HH:mm:ss.SSS z",
                            "MMM d yyyy HH:mm:ss.SSS",
                            "MMM d yyyy HH:mm:ss z",
                            "MMM d yyyy HH:mm:ss",
                            "MMM d HH:mm:ss.SSS z",
                            "MMM d HH:mm:ss.SSS",
                            "MMM d HH:mm:ss z",
                            "MMM d HH:mm:ss",
                            "yyyy MMM d HH:mm:ss.SSS z",
                            "yyyy MMM d HH:mm:ss.SSS",
                            "yyyy MMM d HH:mm:ss z",
                            "yyyy MMM d HH:mm:ss",
                        ],
                        event.get_str("_temp_.date_timezone"),
                        None,
                    ) {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            if event.has("message") {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}
                    if !cached_grok!("%{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}").extract_into(&input, event)? {
                        // Grok pattern: %{DATA:cisco.ios.facility}-(?P<cisco_ios_mnemonic>[A-Z])-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}
                        if !cached_grok_mapped!("%{DATA:cisco.ios.facility}-(?P<cisco_ios_mnemonic>[A-Z])-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}", [("cisco_ios_mnemonic", "cisco.ios.mnemonic")]).extract_into(&input, event)? {
                        }
                    }
                }
            }

            let _cond = {
                event.get_str("cisco.ios.facility") == Some("IOSXE")
                    && event.get_str("event.code") == Some("PLATFORM")
                    && event
                        .get_str("message")
                        .map(|s| s.find("%").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some())
            };
            if _cond {
                if event.has("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: %%{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}
                        if !cached_grok!("%%{DATA:cisco.ios.facility}-%{POSINT:event.severity}-%{DATA:event.code}:\\s+(\\w+\\d+(/\\d+)?\\:\\s+)?([a-zA-Z0-9_]+\\:\\s+)?%{GREEDYDATA:message}").extract_into(&input, event)? {
                    }
                    }
                }
            }

            let _cond = {
                event.get_str("cisco.ios.facility") == Some("FW")
                    && event.get_str("event.code") == Some("SESS_AUDIT_TRAIL")
            };
            if _cond {
                if event.has("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: initiator \\(%{IP:source.ip}:%{NUMBER:source.port:long}\\) sent %{NUMBER:source.bytes:long} bytes -- responder \\(%{IP:destination.ip}:%{NUMBER:destination.port:long}\\) sent %{NUMBER:destination.bytes:long} bytes, from %{NOTSPACE:cisco.ios.interface.name}
                        if !cached_grok!("initiator \\(%{IP:source.ip}:%{NUMBER:source.port:long}\\) sent %{NUMBER:source.bytes:long} bytes -- responder \\(%{IP:destination.ip}:%{NUMBER:destination.port:long}\\) sent %{NUMBER:destination.bytes:long} bytes, from %{NOTSPACE:cisco.ios.interface.name}").extract_into(&input, event)? {
                    }
                    }
                }
            }

            let _cond = {
                event.get_str("cisco.ios.facility") == Some("FW")
                    && event.get_str("event.code") == Some("DROP_PKT")
            };
            if _cond {
                if event.has("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^Dropping %{WORD} %{WORD} from %{NOTSPACE:cisco.ios.interface.name} %{IP:source.ip}:%{NUMBER:source.port:long} ?=> ?%{IP:destination.ip}:%{NUMBER:destination.port:long}
                        if !cached_grok!("^Dropping %{WORD} %{WORD} from %{NOTSPACE:cisco.ios.interface.name} %{IP:source.ip}:%{NUMBER:source.port:long} ?=> ?%{IP:destination.ip}:%{NUMBER:destination.port:long}").extract_into(&input, event)? {
                    }
                    }
                }
            }

            let _cond = { event.has_value("_temp_.generic_message") };
            if _cond {
                event.rename("_temp_.generic_message", "message")?;
            }

            if event.has("event.severity") {
                if let Some(val) = event.get("event.severity") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "event.severity".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "event.severity".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "event.severity".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("event.severity", converted)?;
                }
            }

            if event.has("event.sequence") {
                if let Some(val) = event.get("event.sequence") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "event.sequence".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "event.sequence".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "event.sequence".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("event.sequence", converted)?;
                }
            }

            if event.has("event.code") {
                if let Some(s) = event.get_string("event.code") {
                    let trimmed = s.trim().to_string();
                    event.set("event.code", trimmed)?;
                }
            }

            let _cond = {
                ["IPACCESSLOGP", "ACCESSLOGP", "IPV6ACCESSLOGP"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("list ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("cisco.ios.access_list", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("_temp_.event.action", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("network.transport", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("(") else {
                                break 'dissect false;
                            };
                            captured.push(("source.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("(") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(") ") else {
                                break 'dissect false;
                            };
                            captured.push(("source.port", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(") ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("(") else {
                                break 'dissect false;
                            };
                            captured.push(("destination.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("(") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("), ") else {
                                break 'dissect false;
                            };
                            captured.push(("destination.port", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("), ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" packet") else {
                                break 'dissect false;
                            };
                            captured.push(("source.packets", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" packet") else {
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

            let _cond = {
                ["IPACCESSLOGP", "ACCESSLOGP", "IPV6ACCESSLOGP"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("message") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(rest) = remaining.strip_prefix("access-list ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("cisco.ios.access_list", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("_temp_.event.action", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            captured.push(("network.transport", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("(") else {
                                break 'dissect false;
                            };
                            captured.push(("source.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("(") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(") ") else {
                                break 'dissect false;
                            };
                            captured.push(("source.port", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(") ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" ") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("(") else {
                                break 'dissect false;
                            };
                            captured.push(("destination.address", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("(") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find("), ") else {
                                break 'dissect false;
                            };
                            captured.push(("destination.port", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("), ") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            let Some(pos) = remaining.find(" packet") else {
                                break 'dissect false;
                            };
                            captured.push(("source.packets", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix(" packet") else {
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

            let _cond = {
                ["IPACCESSLOGDP", "ACCESSLOGDP"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("cisco.ios.access_list", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.event.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("/") else {
                            break 'dissect false;
                        };
                        captured.push(("icmp.type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("/") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("), ") else {
                            break 'dissect false;
                        };
                        captured.push(("icmp.code", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("), ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" packet") else {
                            break 'dissect false;
                        };
                        captured.push(("source.packets", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" packet") else {
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

            let _cond = { event.get_str("event.code") == Some("IPACCESSLOGRP") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("cisco.ios.access_list", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.event.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" packet") else {
                            break 'dissect false;
                        };
                        captured.push(("source.packets", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" packet") else {
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

            let _cond = { event.get_str("event.code") == Some("IPACCESSLOGSP") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("cisco.ios.access_list", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.event.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.transport", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" (") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" (") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("), ") else {
                            break 'dissect false;
                        };
                        captured.push(("igmp.type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("), ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" packet") else {
                            break 'dissect false;
                        };
                        captured.push(("source.packets", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" packet") else {
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

            let _cond = { event.get_str("event.code") == Some("ACCESSLOGSP") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("cisco.ios.access_list", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.event.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" packet") else {
                            break 'dissect false;
                        };
                        captured.push(("source.packets", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" packet") else {
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
                ["IPACCESSLOGNP", "ACCESSLOGNP"]
                    .contains(&event.get_str("event.code").unwrap_or(""))
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("list ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("cisco.ios.access_list", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("_temp_.event.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("network.iana_number", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", ") else {
                            break 'dissect false;
                        };
                        captured.push(("destination.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" packet") else {
                            break 'dissect false;
                        };
                        captured.push(("source.packets", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" packet") else {
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

            let _cond = { event.get_str("event.code") == Some("LOGIN_SUCCESS") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: %{DATA:cisco.ios.action} %{WORD:_temp_.event.action} \\[user: %{DATA:source.user.name}\\] \\[Source: %{DATA:source.address}\\]\\s*\\[localport: %{INT:destination.port}\\]
                    if !cached_grok!("%{DATA:cisco.ios.action} %{WORD:_temp_.event.action} \\[user: %{DATA:source.user.name}\\] \\[Source: %{DATA:source.address}\\]\\s*\\[localport: %{INT:destination.port}\\]").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("LOGOUT") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("User ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" has ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.user.name", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" has ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" ") else {
                            break 'dissect false;
                        };
                        captured.push(("cisco.ios.action", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" session ") else {
                            break 'dissect false;
                        };
                        captured.push(("cisco.ios.session.type", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" session ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("(") else {
                            break 'dissect false;
                        };
                        captured.push(("cisco.ios.session.number", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("(") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(")") else {
                            break 'dissect false;
                        };
                        captured.push(("source.address", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(")") else {
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

            let _cond = { event.get_str("event.code") == Some("INVALID_REPLAY_CTR") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find(" Invalid replay counter from client ")
                        else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) =
                            remaining.strip_prefix(" Invalid replay counter from client ")
                        else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(" - got ") else {
                            break 'dissect false;
                        };
                        captured.push(("source.mac", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(" - got ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(", expected ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(", expected ") else {
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

            let _cond = { event.get_str("event.code") == Some("BADAUTH") };
            if _cond {
                if event.has("message") {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^(?:No|Invalid) MD5 digest from %{DATA:source.address}(\\(%{INT:source.port}\\)|\\:%{INT:source.port}) to %{DATA:destination.address}(\\(%{INT:destination.port}\\)|\\:%{INT:destination.port})(?:(?: \\(RST\\))? (?:tableid - %{DATA:cisco.ios.tableid}|%{GREEDYDATA:_temp_.rst}))?$
                        if !cached_grok!("^(?:No|Invalid) MD5 digest from %{DATA:source.address}(\\(%{INT:source.port}\\)|\\:%{INT:source.port}) to %{DATA:destination.address}(\\(%{INT:destination.port}\\)|\\:%{INT:destination.port})(?:(?: \\(RST\\))? (?:tableid - %{DATA:cisco.ios.tableid}|%{GREEDYDATA:_temp_.rst}))?$").extract_into(&input, event)? {
                    }
                    }
                }
            }

            let _cond = {
                !event.has_value("observer.type")
                    && (["SEC", "FW", "IPV6", "IPV6_ACL"]
                        .contains(&event.get_str("cisco.ios.facility").unwrap_or(""))
                        || [
                            "IPACCESSLOGP",
                            "IPACCESSLOGDP",
                            "IPACCESSLOGNP",
                            "IPACCESSLOGSP",
                            "IPACCESSLOGRP",
                            "ACCESSLOGP",
                            "ACCESSLOGDP",
                            "ACCESSLOGNP",
                            "ACCESSLOGSP",
                            "IPV6ACCESSLOGP",
                        ]
                        .contains(&event.get_str("event.code").unwrap_or("")))
            };
            if _cond {
                event.set("observer.type", json!("firewall"))?;
            }

            let _cond = {
                !event.has_value("observer.type")
                    && [
                        "LINK",
                        "SPANTREE",
                        "STP",
                        "PORTSECURITY",
                        "VTP",
                        "LINEPROTO",
                    ]
                    .contains(&event.get_str("cisco.ios.facility").unwrap_or(""))
            };
            if _cond {
                event.set("observer.type", json!("switch"))?;
            }

            let _cond = { !event.has_value("observer.type") };
            if _cond {
                event.set("observer.type", json!("router"))?;
            }

            let _cond = { event.get_str("event.code") == Some("INVALID_RP_JOIN") };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: Received \\((?:(%{IP:cisco.ios.pim.source.ip}|%{DATA})), %{DATA:cisco.ios.pim.group.ip}\\) %{WORD:cisco.ios.action} from %{IP:source.address} for %{DATA:cisco.ios.outcome} %{IP:destination.address}
                    if !cached_grok!("Received \\((?:(%{IP:cisco.ios.pim.source.ip}|%{DATA})), %{DATA:cisco.ios.pim.group.ip}\\) %{WORD:cisco.ios.action} from %{IP:source.address} for %{DATA:cisco.ios.outcome} %{IP:destination.address}").extract_into(&input, event)? {
                    }
                }
            }

            let _cond = { event.get_str("event.code") == Some("INVALID_RP_JOIN") };
            if _cond {
                event.set("event.action", json!("multicast-join"))?;
            }

            let _cond = { event.get_str("event.code") == Some("INVALID_RP_JOIN") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("event.code") == Some("INVALID_RP_JOIN") };
            if _cond {
                event.set("event.reason", json!("Invalid RP"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has("destination.address") {
                    if let Some(s) = event.get_string("destination.address") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "destination.address".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("destination.ip", s)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_ip")?;
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("destination.domain", v)?;
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
                if event.has("source.address") {
                    if let Some(s) = event.get_string("source.address") {
                        // Validate IP format
                        let s = s.trim();
                        if s.parse::<std::net::IpAddr>().is_err() {
                            return Err(TransformError::ParseError {
                                path: "source.address".into(),
                                message: format!("cannot convert '{}' to IP", s),
                            });
                        }
                        event.set("source.ip", s)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_ip")?;
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("source.domain", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("cisco.ios.pim.source.ip") {
                if let Some(s) = event.get_string("cisco.ios.pim.source.ip") {
                    // Validate IP format
                    let s = s.trim();
                    if s.parse::<std::net::IpAddr>().is_err() {
                        return Err(TransformError::ParseError {
                            path: "cisco.ios.pim.source.ip".into(),
                            message: format!("cannot convert '{}' to IP", s),
                        });
                    }
                    event.set("cisco.ios.pim.source.ip", s)?;
                }
            }

            if event.has("source.port") {
                if let Some(val) = event.get("source.port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "source.port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "source.port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "source.port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.port", converted)?;
                }
            }

            if event.has("source.packets") {
                if let Some(val) = event.get("source.packets") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "source.packets".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "source.packets".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "source.packets".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("source.packets", converted)?;
                }
            }

            if event.has("destination.port") {
                if let Some(val) = event.get("destination.port") {
                    let converted = match val {
                        Value::String(s) => {
                            let s = s.trim();
                            if let Some(hex) = s.strip_prefix("0x") {
                                json!(i64::from_str_radix(hex, 16).map_err(|_| {
                                    TransformError::ParseError {
                                        path: "destination.port".into(),
                                        message: format!("cannot convert '{}' to integer", s),
                                    }
                                })?)
                            } else {
                                json!(s.parse::<i64>().map_err(|_| TransformError::ParseError {
                                    path: "destination.port".into(),
                                    message: format!("cannot convert '{}' to integer", s)
                                })?)
                            }
                        }
                        Value::Number(n) => {
                            json!(n.as_i64().unwrap_or(n.as_f64().unwrap_or(0.0) as i64))
                        }
                        Value::Bool(b) => json!(if *b { 1 } else { 0 }),
                        _ => {
                            return Err(TransformError::ParseError {
                                path: "destination.port".into(),
                                message: "cannot convert to integer".into(),
                            });
                        }
                    };
                    event.set("destination.port", converted)?;
                }
            }

            let _cond = { event.has_value("source.bytes") || event.has_value("destination.bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: long n = 0;\nif (ctx.source?.bytes != null) {\n  n += ctx.source.bytes\n}\nif (ctx.destination?.bytes != null) {\n  n += ctx.destination.bytes\n}\nif (ctx.network == null) {\n  ctx.network = new HashMap();\n}\nctx.network.bytes = n;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"long n = 0;\nif (ctx.source?.bytes != null) {\n  n += ctx.source.bytes\n}\nif (ctx.destination?.bytes != null) {\n  n += ctx.destination.bytes\n}\nif (ctx.network == null) {\n  ctx.network = new HashMap();\n}\nctx.network.bytes = n;\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.packets") };
            if _cond {
                if let Some(v) = event.get("source.packets").cloned() {
                    event.set("network.packets", v)?;
                }
            }

            let _cond = {
                event.has_value("source.ip")
                    && event.get("source.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                event.set("network.type", json!("ipv4"))?;
            }

            let _cond = { event.has_value("source.ip") && !event.has_value("network.type") };
            if _cond {
                event.set("network.type", json!("ipv6"))?;
            }

            let _cond = { event.get_str("_temp_.event.action") == Some("denied") };
            if _cond {
                event.set("event.action", json!("deny"))?;
            }

            let _cond = { event.get_str("event.action") == Some("deny") };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { event.get_str("_temp_.event.action") == Some("permitted") };
            if _cond {
                event.set("event.action", json!("allow"))?;
            }

            let _cond = { event.get_str("event.action") == Some("allow") };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(0) };
            if _cond {
                event.set("log.level", json!("emergencies"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(1) };
            if _cond {
                event.set("log.level", json!("alert"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(2) };
            if _cond {
                event.set("log.level", json!("critical"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(3) };
            if _cond {
                event.set("log.level", json!("error"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(4) };
            if _cond {
                event.set("log.level", json!("warning"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(5) };
            if _cond {
                event.set("log.level", json!("notification"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(6) };
            if _cond {
                event.set("log.level", json!("informational"))?;
            }

            let _cond = { event.get_i64("event.severity") == Some(7) };
            if _cond {
                event.set("log.level", json!("debug"))?;
            }

            if event.has("source.ip") {
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

            if event.has("destination.ip") {
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

            if event.has("source.ip") {
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

            if event.has("destination.ip") {
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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            if event.has("source.mac") {
                if let Some(s) = event.get_string("source.mac") {
                    let re = cached_regex!(":");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("source.mac", replaced)?;
                }
            }

            if event.has("source.mac") {
                if let Some(s) = event.get_string("source.mac") {
                    let uppered = s.to_uppercase();
                    event.set("source.mac", uppered)?;
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.domain") };
            if _cond {
                event.append(
                    "related.hosts",
                    json!(
                        event
                            .get("destination.domain")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("source.ip") {
                    // Community ID v1 hash
                    if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                        event.get_string("source.ip"),
                        event.get_string("destination.ip"),
                        event
                            .get_string("network.transport")
                            .or_else(|| event.get_string("network.iana_number")),
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
                }
                Ok(())
            })();

            event.remove("_temp_");
            event.remove("_conf");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.remove("_temp_");
                event.remove("_conf");
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
