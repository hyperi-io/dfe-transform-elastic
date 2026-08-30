// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_log` pipeline.
pub struct PipelineLog;

impl Transform for PipelineLog {
    fn name(&self) -> &str {
        "pipeline_log"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if event.has_value("raw_message") {
            if let Some(input) = event.get_string("raw_message") {
                // Grok pattern: ^(\\[%{NUMBER:process.pid:long}(-%{BASE16FLOAT:postgresql.log.session_line_number:long})?\\]:? ?)?(\\[%{NUMBER:postgresql.log.session_line_number:long}(-%{BASE16FLOAT:postgresql.log.sequence_number:long})?\\] )?((\\[%{USERNAME:user.name}\\]@\\[(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))\\]|%{USERNAME:user.name}@(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*)) )?)?(%{DATA:_temp_.database_connection_str} ?)?%{WORD:log.level}:  ?(?:(?P<postgresql_log_sql_state_code>(?:\\b[A-Z0-9]{5}\\b))|%{SPACE})(duration: %{NUMBER:temp.duration:float} ms  (?:%{WORD:postgresql.log.query_step}(?: <unnamed>| %{WORD:postgresql.log.query_name})?): (?P<postgresql_log_query>(?:(.|\n|\t)*))|: (?P<message>(?:(.|\n|\t)*))|(?P<message>(?:(.|\n|\t)*)))
                let _ = cached_grok_mapped!("^(\\[%{NUMBER:process.pid:long}(-%{BASE16FLOAT:postgresql.log.session_line_number:long})?\\]:? ?)?(\\[%{NUMBER:postgresql.log.session_line_number:long}(-%{BASE16FLOAT:postgresql.log.sequence_number:long})?\\] )?((\\[%{USERNAME:user.name}\\]@\\[(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))\\]|%{USERNAME:user.name}@(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*)) )?)?(%{DATA:_temp_.database_connection_str} ?)?%{WORD:log.level}:  ?(?:(?P<postgresql_log_sql_state_code>(?:\\b[A-Z0-9]{5}\\b))|%{SPACE})(duration: %{NUMBER:temp.duration:float} ms  (?:%{WORD:postgresql.log.query_step}(?: <unnamed>| %{WORD:postgresql.log.query_name})?): (?P<postgresql_log_query>(?:(.|\n|\t)*))|: (?P<message>(?:(.|\n|\t)*))|(?P<message>(?:(.|\n|\t)*)))", [("postgresql_log_database", "postgresql.log.database"), ("postgresql_log_database", "postgresql.log.database"), ("postgresql_log_sql_state_code", "postgresql.log.sql_state_code"), ("postgresql_log_query", "postgresql.log.query")]).extract_into(&input, event)?;
            }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("_temp_.database_connection_str") {
            if let Some(kv_str) = event.get_string("_temp_.database_connection_str") {
                for pair in kv_str.split(",") {
                    if pair.trim().is_empty() {
                        continue;
                    }
                    let Some((key, value)) = pair.split_once("=") else {
                        return Err(TransformError::ParseError {
                            path: "_temp_.database_connection_str".into(),
                            message: format!("does not contain value_split: {pair}"),
                        });
                    };
                    {
                        let value = match (value.chars().next(), value.chars().last()) {
                            (Some('('), Some(')'))
                            | (Some('['), Some(']'))
                            | (Some('<'), Some('>'))
                            | (Some('"'), Some('"'))
                            | (Some('\''), Some('\'')) if value.chars().count() > 1 => {
                                &value[1..value.len() - 1]
                            }
                            _ => value,
                        };
                        if !key.is_empty() {
                            kv_put(event, &format!("_temp_.database_connection_obj.{}", key), value)?;
                        }
                    }
                }
            }
        }
            Ok(())
        })();

            if event.has_value("_temp_.database_connection_obj.app") {
                event.rename("_temp_.database_connection_obj.app", "postgresql.log.application_name")?;
            }

            if event.has_value("_temp_.database_connection_obj.client") {
                event.rename("_temp_.database_connection_obj.client", "postgresql.log.client_addr")?;
            }

            if event.has_value("_temp_.database_connection_obj.db") {
                event.rename("_temp_.database_connection_obj.db", "postgresql.log.database")?;
            }

            if event.has_value("_temp_.database_connection_obj.user") {
                event.rename("_temp_.database_connection_obj.user", "user.name")?;
            }

        Ok(TransformResult::Continue)
    }
}
