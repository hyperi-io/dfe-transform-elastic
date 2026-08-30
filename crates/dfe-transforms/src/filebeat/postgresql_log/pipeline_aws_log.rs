// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_aws_log` pipeline.
pub struct PipelineAwsLog;

impl Transform for PipelineAwsLog {
    fn name(&self) -> &str {
        "pipeline_aws_log"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        if event.has_value("raw_message") {
            if let Some(input) = event.get_string("raw_message") {
                // Grok pattern: (%{DATA:postgresql.log.client_addr}\\(%{NUMBER:postgresql.log.client_port:int}\\)|\\[%{DATA:postgresql.log.client_addr}\\])?:(%{USERNAME:user.name}?@(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))?|\\[%{USERNAME:user.name}?\\]@\\[(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))?\\]):(\\[%{NUMBER:process.pid:long}\\])?:%{WORD:log.level}: ((?:%{SPACE}%{WORD:postgresql.log.query_step}): (?P<postgresql_log_query>(?:(.|\\r|\\n)*))| (?P<message>(?:(.|\\r|\\n)*))|(?P<message>(?:(.|\\r|\\n)*)))
                let _ = cached_grok_mapped!("(%{DATA:postgresql.log.client_addr}\\(%{NUMBER:postgresql.log.client_port:int}\\)|\\[%{DATA:postgresql.log.client_addr}\\])?:(%{USERNAME:user.name}?@(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))?|\\[%{USERNAME:user.name}?\\]@\\[(?P<postgresql_log_database>(?:[a-zA-Z0-9_]+[a-zA-Z0-9_\\$]*))?\\]):(\\[%{NUMBER:process.pid:long}\\])?:%{WORD:log.level}: ((?:%{SPACE}%{WORD:postgresql.log.query_step}): (?P<postgresql_log_query>(?:(.|\\r|\\n)*))| (?P<message>(?:(.|\\r|\\n)*))|(?P<message>(?:(.|\\r|\\n)*)))", [("postgresql_log_database", "postgresql.log.database"), ("postgresql_log_database", "postgresql.log.database"), ("postgresql_log_query", "postgresql.log.query")]).extract_into(&input, event)?;
            }
        }

        Ok(TransformResult::Continue)
    }
}
