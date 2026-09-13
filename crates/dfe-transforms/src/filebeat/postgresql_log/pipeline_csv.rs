// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_csv` pipeline.
pub struct PipelineCsv;

impl Transform for PipelineCsv {
    fn name(&self) -> &str {
        "pipeline_csv"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
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
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "postgresql.log.session_line_num".into(),
                        message,
                    })?;
                event.set("postgresql.log.session_line_num", converted)?;
            }
        }

        if event.has_value("process.pid") {
            if let Some(val) = event.get("process.pid") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "process.pid".into(),
                        message,
                    })?;
                event.set("process.pid", converted)?;
            }
        }

            if let Some(date_str) = event.get_as_string("tempcsv.session_start_time") {
                match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss.SSS zz", "yyyy-MM-dd HH:mm:ss zz"], None, None) {
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
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "postgresql.log.transaction_id".into(),
                        message,
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
                        cached_grok_mapped!("^duration: %{NUMBER:temp.duration:float} ms  (?P<postgresql_log_query_step>(?:(parse|bind|statement|fastpath function call|execute|execute fetch from))) %{DATA:postgresql.log.query_name}: (?P<message>(?:(.|\n|   )*))$", [("postgresql_log_query_step", "postgresql.log.query_step")]),
                        cached_grok_mapped!("^duration: %{NUMBER:temp.duration:float} ms  (?P<postgresql_log_query_step>(?:(parse|bind|statement|fastpath function call|execute|execute fetch from))): (?P<message>(?:(.|\n|   )*))$", [("postgresql_log_query_step", "postgresql.log.query_step")]),
                        cached_grok_mapped!("^((?P<postgresql_log_query_step>(?:(parse|bind|statement|fastpath function call|execute|execute fetch from))): )?(?P<message>(?:(.|\n|   )*))$", [("postgresql_log_query_step", "postgresql.log.query_step")]),
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

        Ok(TransformResult::Continue)
    }
}
