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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("ecs.version", json!("8.11.0"))?;

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

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^(?P<postgresql_log_timestamp>(?:(?P<_temp__timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{ISO8601_HOUR}:?%{MINUTE}(?::?%{SECOND}))) (?P<event_timezone>(?:([a-zA-Z]{1,4})|(?:Z|[+-]%{HOUR}(?::?%{MINUTE})?)))))(?P<separator>(?:.))(?P<raw_message>(?:(.|\n|\t)*))
                if !cached_grok_mapped!("^(?P<postgresql_log_timestamp>(?:(?P<_temp__timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{ISO8601_HOUR}:?%{MINUTE}(?::?%{SECOND}))) (?P<event_timezone>(?:([a-zA-Z]{1,4})|(?:Z|[+-]%{HOUR}(?::?%{MINUTE})?)))))(?P<separator>(?:.))(?P<raw_message>(?:(.|\n|\t)*))", [("postgresql_log_timestamp", "postgresql.log.timestamp"), ("_temp__timestamp", "_temp_.timestamp"), ("event_timezone", "event.timezone")]).extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
            }

            let _cond = { event.has_value("_temp_.timestamp") };
            if _cond {
                // Painless script
                // Source: String get_timezone(def ctx) {\n  if (ctx.event?.timezone != null) {\n    if (ctx._conf?.tz_map != null) {\n      for (def item : ctx._conf.tz_map) {\n        if (item.tz_short == ctx.event?.timezone) {\n          return item.tz_long;\n        }\n      }\n    }\n    return ctx.event.timezone;\n  }\n\n  ctx.event.timezone = 'UTC';\n  return 'UTC';\n}\n\ndef event_timezone = get_timezone(ctx);\nif (!(event_timezone.contains('+')) && !(event_timezone.contains('-')) && !(event_timezone.length() > 4)) {\n  // timezone abbreviation e.g. CEST need to be put inside the timestamp\n  SimpleDateFormat sdf = new SimpleDateFormat(\"z\");\n  sdf.parse(event_timezone);\n  ctx._temp_.date_timezone = ZoneId.of(sdf.getTimeZone().getID(), ZoneId.SHORT_IDS).getId();\n  ctx?._temp_.timestamp = ctx?._temp_.timestamp + \" \" + event_timezone;\n} else {\n  // timezone is either abbreviation+-offset e.g. UTC+1 or long representation\n  // e.g. Europe/Athens needs to be put as a ZoneId and *not* inside the timestamp\n  ctx._temp_.date_timezone = event_timezone;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String get_timezone(def ctx) {\n  if (ctx.event?.timezone != null) {\n    if (ctx._conf?.tz_map != null) {\n      for (def item : ctx._conf.tz_map) {\n        if (item.tz_short == ctx.event?.timezone) {\n          return item.tz_long;\n        }\n      }\n    }\n    return ctx.event.timezone;\n  }\n\n  ctx.event.timezone = 'UTC';\n  return 'UTC';\n}\n\ndef event_timezone = get_timezone(ctx);\nif (!(event_timezone.contains('+')) && !(event_timezone.contains('-')) && !(event_timezone.length() > 4)) {\n  // timezone abbreviation e.g. CEST need to be put inside the timestamp\n  SimpleDateFormat sdf = new SimpleDateFormat(\"z\");\n  sdf.parse(event_timezone);\n  ctx._temp_.date_timezone = ZoneId.of(sdf.getTimeZone().getID(), ZoneId.SHORT_IDS).getId();\n  ctx?._temp_.timestamp = ctx?._temp_.timestamp + \" \" + event_timezone;\n} else {\n  // timezone is either abbreviation+-offset e.g. UTC+1 or long representation\n  // e.g. Europe/Athens needs to be put as a ZoneId and *not* inside the timestamp\n  ctx._temp_.date_timezone = event_timezone;\n}"#
                    ),
                )?;
            }

            let _cond = {
                event.get_str("separator") != Some(",") && event.get_str("separator") != Some(":")
            };
            if _cond {
                // Begin nested pipeline: "pipeline-log"
                if event.has_value("raw_message") {
                    if let Some(input) = event.get_string("raw_message") {
                        // Grok pattern: ^(\\[%{NUMBER:process.pid:long}(-%{BASE16FLOAT:postgresql.log.session_line_number:long})?\\]:? ?)?(\\[%{NUMBER:postgresql.log.session_line_number:long}(-%{BASE16FLOAT:postgresql.log.sequence_number:long})?\\] )?((\\[%{USERNAME:user.name}\\]@\\[(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))\\]|%{USERNAME:user.name}@(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*)) )?)?(%{DATA:_temp_.database_connection_str} ?)?%{WORD:log.level}:  ?(?:(?P<postgresql_log_sql_state_code>(?:\\b[A-Z0-9]{5}\\b))|%{SPACE})(duration: %{NUMBER:temp.duration:float} ms  (?:%{WORD:postgresql.log.query_step}(?: <unnamed>| %{WORD:postgresql.log.query_name})?): (?P<postgresql_log_query>(?:(.|\n|\t)*))|: (?P<message>(?:(.|\n|\t)*))|(?P<message>(?:(.|\n|\t)*)))
                        if !cached_grok_mapped!("^(\\[%{NUMBER:process.pid:long}(-%{BASE16FLOAT:postgresql.log.session_line_number:long})?\\]:? ?)?(\\[%{NUMBER:postgresql.log.session_line_number:long}(-%{BASE16FLOAT:postgresql.log.sequence_number:long})?\\] )?((\\[%{USERNAME:user.name}\\]@\\[(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))\\]|%{USERNAME:user.name}@(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*)) )?)?(%{DATA:_temp_.database_connection_str} ?)?%{WORD:log.level}:  ?(?:(?P<postgresql_log_sql_state_code>(?:\\b[A-Z0-9]{5}\\b))|%{SPACE})(duration: %{NUMBER:temp.duration:float} ms  (?:%{WORD:postgresql.log.query_step}(?: <unnamed>| %{WORD:postgresql.log.query_name})?): (?P<postgresql_log_query>(?:(.|\n|\t)*))|: (?P<message>(?:(.|\n|\t)*))|(?P<message>(?:(.|\n|\t)*)))", [("postgresql_log_database", "postgresql.log.database"), ("postgresql_log_database", "postgresql.log.database"), ("postgresql_log_sql_state_code", "postgresql.log.sql_state_code"), ("postgresql_log_query", "postgresql.log.query")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_temp_.database_connection_str") {
                        if let Some(kv_str) = event.get_string("_temp_.database_connection_str") {
                            let mut kv_gap = false;
                            for pair in kv_str.split(",") {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "_temp_.database_connection_str".into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    let value = value
                                        .strip_prefix(['(', '[', '<', '"', '\''])
                                        .unwrap_or(value);
                                    let value = value
                                        .strip_suffix([']', ')', '>', '"', '\''])
                                        .unwrap_or(value);
                                    if !key.is_empty() {
                                        kv_put(
                                            event,
                                            &format!("_temp_.database_connection_obj.{}", key),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has_value("_temp_.database_connection_obj.app") {
                    event.rename(
                        "_temp_.database_connection_obj.app",
                        "postgresql.log.application_name",
                    )?;
                }
                if event.has_value("_temp_.database_connection_obj.client") {
                    event.rename(
                        "_temp_.database_connection_obj.client",
                        "postgresql.log.client_addr",
                    )?;
                }
                if event.has_value("_temp_.database_connection_obj.db") {
                    event.rename(
                        "_temp_.database_connection_obj.db",
                        "postgresql.log.database",
                    )?;
                }
                if event.has_value("_temp_.database_connection_obj.user") {
                    event.rename("_temp_.database_connection_obj.user", "user.name")?;
                }
                // End nested pipeline: "pipeline-log"
            }

            let _cond = { event.get_str("separator") == Some(",") };
            if _cond {
                // Begin nested pipeline: "pipeline-csv"
                if event.has_value("raw_message") {
                    if let Some(csv_str) = event.get_string("raw_message") {
                        let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                        let mut rdr = csv::ReaderBuilder::new()
                            .delimiter(b',')
                            .quote(b'\"')
                            .has_headers(false)
                            .from_reader(csv_str.as_bytes());
                        if let Some(Ok(record)) = rdr.records().next() {
                            if let Some(val) = record.get(0) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("user.name", val)?;
                                }
                            }
                            if let Some(val) = record.get(1) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.database", val)?;
                                }
                            }
                            if let Some(val) = record.get(2) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("process.pid", val)?;
                                }
                            }
                            if let Some(val) = record.get(3) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("tempcsv.connection_from", val)?;
                                }
                            }
                            if let Some(val) = record.get(4) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.session_id", val)?;
                                }
                            }
                            if let Some(val) = record.get(5) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("tempcsv.session_line_num", val)?;
                                }
                            }
                            if let Some(val) = record.get(6) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.command_tag", val)?;
                                }
                            }
                            if let Some(val) = record.get(7) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("tempcsv.session_start_time", val)?;
                                }
                            }
                            if let Some(val) = record.get(8) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.virtual_transaction_id", val)?;
                                }
                            }
                            if let Some(val) = record.get(9) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.transaction_id", val)?;
                                }
                            }
                            if let Some(val) = record.get(10) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("log.level", val)?;
                                }
                            }
                            if let Some(val) = record.get(11) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.sql_state_code", val)?;
                                }
                            }
                            if let Some(val) = record.get(12) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("tempcsv.message", val)?;
                                }
                            }
                            if let Some(val) = record.get(13) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.detail", val)?;
                                }
                            }
                            if let Some(val) = record.get(14) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.hint", val)?;
                                }
                            }
                            if let Some(val) = record.get(15) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.internal_query", val)?;
                                }
                            }
                            if let Some(val) = record.get(16) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("tempcsv.internal_query_pos", val)?;
                                }
                            }
                            if let Some(val) = record.get(17) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.context", val)?;
                                }
                            }
                            if let Some(val) = record.get(18) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.query", val)?;
                                }
                            }
                            if let Some(val) = record.get(19) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("tempcsv.query_pos", val)?;
                                }
                            }
                            if let Some(val) = record.get(20) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.location", val)?;
                                }
                            }
                            if let Some(val) = record.get(21) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.application_name", val)?;
                                }
                            }
                            if let Some(val) = record.get(22) {
                                let val = val.trim();
                                if !val.is_empty() {
                                    event.set("postgresql.log.backend_type", val)?;
                                }
                            }
                        }
                    }
                }
                if event.has_value("tempcsv.connection_from") {
                    if let Some(input) = event.get_string("tempcsv.connection_from") {
                        // Grok pattern: ^%{DATA:postgresql.log.client_addr}(:%{NUMBER:postgresql.log.client_port:int})?$
                        if !cached_grok!("^%{DATA:postgresql.log.client_addr}(:%{NUMBER:postgresql.log.client_port:int})?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                if event.has_value("postgresql.log.session_line_num") {
                    if let Some(val) = event.get("postgresql.log.session_line_num") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "postgresql.log.session_line_num".into(),
                                message,
                            }
                        })?;
                        event.set("postgresql.log.session_line_num", converted)?;
                    }
                }
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
                if let Some(date_str) = event.get_as_string("tempcsv.session_start_time") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd HH:mm:ss.SSS zz", "yyyy-MM-dd HH:mm:ss zz"],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("postgresql.log.session_start_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "tempcsv.session_start_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                if event.has_value("postgresql.log.transaction_id") {
                    if let Some(val) = event.get("postgresql.log.transaction_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "postgresql.log.transaction_id".into(),
                                message,
                            }
                        })?;
                        event.set("postgresql.log.transaction_id", converted)?;
                    }
                }
                if event.has_value("tempcsv.message") {
                    if let Some(input) = event.get_string("tempcsv.message") {
                        // Grok pattern: ^duration: %{NUMBER:temp.duration:float} ms$
                        // Grok pattern: ^duration: %{NUMBER:temp.duration:float} ms  (?P<postgresql_log_query_step>(?:(parse|bind|statement|fastpath function call|execute|execute fetch from))) %{DATA:postgresql.log.query_name}: (?P<message>(?:(.|\n|   )*))$
                        // Grok pattern: ^duration: %{NUMBER:temp.duration:float} ms  (?P<postgresql_log_query_step>(?:(parse|bind|statement|fastpath function call|execute|execute fetch from))): (?P<message>(?:(.|\n|   )*))$
                        // Grok pattern: ^((?P<postgresql_log_query_step>(?:(parse|bind|statement|fastpath function call|execute|execute fetch from))): )?(?P<message>(?:(.|\n|   )*))$
                        if !extract_first_match(
                            &[
                                cached_grok!("^duration: %{NUMBER:temp.duration:float} ms$"),
                                cached_grok_mapped!(
                                    "^duration: %{NUMBER:temp.duration:float} ms  (?P<postgresql_log_query_step>(?:(parse|bind|statement|fastpath function call|execute|execute fetch from))) %{DATA:postgresql.log.query_name}: (?P<message>(?:(.|\n|   )*))$",
                                    [("postgresql_log_query_step", "postgresql.log.query_step")]
                                ),
                                cached_grok_mapped!(
                                    "^duration: %{NUMBER:temp.duration:float} ms  (?P<postgresql_log_query_step>(?:(parse|bind|statement|fastpath function call|execute|execute fetch from))): (?P<message>(?:(.|\n|   )*))$",
                                    [("postgresql_log_query_step", "postgresql.log.query_step")]
                                ),
                                cached_grok_mapped!(
                                    "^((?P<postgresql_log_query_step>(?:(parse|bind|statement|fastpath function call|execute|execute fetch from))): )?(?P<message>(?:(.|\n|   )*))$",
                                    [("postgresql_log_query_step", "postgresql.log.query_step")]
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                if event.has_value("tempcsv.connection_from") {
                    if let Some(input) = event.get_string("tempcsv.connection_from") {
                        // Grok pattern: ^%{DATA:postgresql.log.client_addr}(:%{NUMBER:postgresql.log.client_port:int})?$
                        if !cached_grok!("^%{DATA:postgresql.log.client_addr}(:%{NUMBER:postgresql.log.client_port:int})?$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                event.remove("tempcsv");
                // End nested pipeline: "pipeline-csv"
            }

            let _cond = { event.get_str("separator") == Some(":") };
            if _cond {
                // Begin nested pipeline: "pipeline-aws-log"
                if event.has_value("raw_message") {
                    if let Some(input) = event.get_string("raw_message") {
                        // Grok pattern: (%{DATA:postgresql.log.client_addr}\\(%{NUMBER:postgresql.log.client_port:int}\\)|\\[%{DATA:postgresql.log.client_addr}\\])?:(%{USERNAME:user.name}?@(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))?|\\[%{USERNAME:user.name}?\\]@\\[(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))?\\]):(\\[%{NUMBER:process.pid:long}\\])?:%{WORD:log.level}: ((?:%{SPACE}%{WORD:postgresql.log.query_step}): (?P<postgresql_log_query>(?:(.|\\r|\\n)*))| (?P<message>(?:(.|\\r|\\n)*))|(?P<message>(?:(.|\\r|\\n)*)))
                        if !cached_grok_mapped!("(%{DATA:postgresql.log.client_addr}\\(%{NUMBER:postgresql.log.client_port:int}\\)|\\[%{DATA:postgresql.log.client_addr}\\])?:(%{USERNAME:user.name}?@(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))?|\\[%{USERNAME:user.name}?\\]@\\[(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))?\\]):(\\[%{NUMBER:process.pid:long}\\])?:%{WORD:log.level}: ((?:%{SPACE}%{WORD:postgresql.log.query_step}): (?P<postgresql_log_query>(?:(.|\\r|\\n)*))| (?P<message>(?:(.|\\r|\\n)*))|(?P<message>(?:(.|\\r|\\n)*)))", [("postgresql_log_database", "postgresql.log.database"), ("postgresql_log_database", "postgresql.log.database"), ("postgresql_log_query", "postgresql.log.query")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                // End nested pipeline: "pipeline-aws-log"
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_temp_.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss.SSS zz",
                            "yyyy-MM-dd HH:mm:ss zz",
                            "yyyy-MM-dd HH:mm:ss.SSS",
                            "yyyy-MM-dd HH:mm:ss",
                        ],
                        event.get_str("_temp_.date_timezone"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.timestamp".into(),
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

            // on_failure: 3 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("postgresql.log.client_addr") {
                    if let Some(val) = event.get("postgresql.log.client_addr") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "postgresql.log.client_addr".into(),
                                message,
                            }
                        })?;
                        event.set("postgresql.log.client_addr", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if let Some(v) = event.get("postgresql.log.client_addr").cloned() {
                    event.set("tmp_host", v)?;
                }
                let _cond = { event.has_value("tmp_host") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("tmp_host")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("tmp_host") };
                if _cond {
                    event.set("tmp_host", json!(""))?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { !event.has_value("tmp_host") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("postgresql.log.client_addr")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("temp.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = Math.round(ctx.temp.duration * params.scale)
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.event.duration = Math.round(ctx.temp.duration * params.scale)"#
                    ),
                    cached_params!("{\"scale\":1000000}"),
                )?;
            }

            event.remove("temp.duration");

            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("database"))?;

            let _cond = {
                !event.has_value("postgresql.log.sql_state_code")
                    || (event
                        .get_str("postgresql.log.sql_state_code")
                        .is_some_and(|s| cached_regex!(r"^(?:^0[012].*)$").is_match(s)))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            let _cond = {
                event.has_value("postgresql.log.sql_state_code")
                    && !(event
                        .get_str("postgresql.log.sql_state_code")
                        .is_some_and(|s| cached_regex!(r"^(?:^0[012].*)$").is_match(s)))
            };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("error")]))?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.remove("separator").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "separator".into(),
                });
            }
            if event.remove("raw_message").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "raw_message".into(),
                });
            }

            event.remove("temp");

            event.remove("_temp_");
            event.remove("_conf");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);
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

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set(
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
