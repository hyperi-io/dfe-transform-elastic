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

            event.set("event.kind", json!("event"))?;

            event.set("event.module", json!("mysql"))?;

            event.set("event.category", Value::Array(vec![json!("database")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let v = json!(
                event
                    .get("sql.metrics.string.source_user")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("user.name", v)?;
            }

            event.set("service.type", json!("mysql"))?;

            let _cond = { event.get_i64("sql.metrics.numeric.auto_position") == Some(1) };
            if _cond {
                let v = json!(true);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.numeric.auto_position", v)?;
                }
            }

            let _cond = { event.get_i64("sql.metrics.numeric.auto_position") == Some(0) };
            if _cond {
                let v = json!(false);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.numeric.auto_position", v)?;
                }
            }

            let _cond = { event.get_i64("sql.metrics.numeric.get_source_public_key") == Some(1) };
            if _cond {
                let v = json!(true);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.numeric.get_source_public_key", v)?;
                }
            }

            let _cond = { event.get_i64("sql.metrics.numeric.get_source_public_key") == Some(0) };
            if _cond {
                let v = json!(false);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.numeric.get_source_public_key", v)?;
                }
            }

            let _cond = {
                event.get_str("sql.metrics.string.source_ssl_verify_server_cert") == Some("Yes")
            };
            if _cond {
                let v = json!(true);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.string.source_ssl_verify_server_cert", v)?;
                }
            }

            let _cond =
                { event.get_str("sql.metrics.string.source_ssl_verify_server_cert") == Some("No") };
            if _cond {
                let v = json!(false);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.string.source_ssl_verify_server_cert", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sql.metrics.numeric.source_port") {
                    if let Some(val) = event.get("sql.metrics.numeric.source_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sql.metrics.numeric.source_port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if event.has_value("sql.metrics.numeric.auto_position") {
                event.rename(
                    "sql.metrics.numeric.auto_position",
                    "mysql.replica_status.is_auto_position",
                )?;
            }

            if event.has_value("sql.metrics.string.network_namespace") {
                event.rename("sql.metrics.string.network_namespace", "network.name")?;
            }

            if event.has_value("sql.metrics.string.channel_name") {
                event.rename(
                    "sql.metrics.string.channel_name",
                    "mysql.replica_status.channel.name",
                )?;
            }

            if event.has_value("sql.metrics.numeric.connect_retry") {
                event.rename(
                    "sql.metrics.numeric.connect_retry",
                    "mysql.replica_status.connection.retry.sec",
                )?;
            }

            if event.has_value("sql.metrics.numeric.skip_counter") {
                event.rename(
                    "sql.metrics.numeric.skip_counter",
                    "mysql.replica_status.event_skip.count",
                )?;
            }

            if event.has_value("sql.metrics.string.executed_gtid_set") {
                event.rename(
                    "sql.metrics.string.executed_gtid_set",
                    "mysql.replica_status.gtid.executed.set",
                )?;
            }

            if event.has_value("sql.metrics.string.retrieved_gtid_set") {
                event.rename(
                    "sql.metrics.string.retrieved_gtid_set",
                    "mysql.replica_status.gtid.retrieved.set",
                )?;
            }

            if event.has_value("sql.metrics.string.replica_io_running") {
                event.rename(
                    "sql.metrics.string.replica_io_running",
                    "mysql.replica_status.is_io_thread_running",
                )?;
            }

            if event.has_value("sql.metrics.string.replica_sql_running") {
                event.rename(
                    "sql.metrics.string.replica_sql_running",
                    "mysql.replica_status.is_sql_thread_running",
                )?;
            }

            if event.has_value("sql.metrics.string.last_io_error") {
                event.rename(
                    "sql.metrics.string.last_io_error",
                    "mysql.replica_status.last_error.io.message",
                )?;
            }

            if event.has_value("sql.metrics.numeric.last_io_errno") {
                event.rename(
                    "sql.metrics.numeric.last_io_errno",
                    "mysql.replica_status.last_error.io.number",
                )?;
            }

            let _cond = {
                event.has_value("sql.metrics.string.last_io_error_timestamp")
                    && event.get_str("sql.metrics.string.last_io_error_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("sql.metrics.string.last_io_error_timestamp")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyMMdd HH:mm:ss",
                                "yyMMdd H:m:s",
                                "yyMMdd  H:m:s",
                                "yyyy-MM-dd H:m:s",
                                "yyyy-MM-dd  H:m:s",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("mysql.replica_status.last_error.io.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "sql.metrics.string.last_io_error_timestamp".into(),
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
                        "last_io_error_timestamp",
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

            let _cond = {
                event.has_value("sql.metrics.string.last_sql_error_timestamp")
                    && event.get_str("sql.metrics.string.last_sql_error_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("sql.metrics.string.last_sql_error_timestamp")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyMMdd HH:mm:ss",
                                "yyMMdd H:m:s",
                                "yyMMdd  H:m:s",
                                "yyyy-MM-dd H:m:s",
                                "yyyy-MM-dd  H:m:s",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event
                                .set("mysql.replica_status.last_error.sql.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "sql.metrics.string.last_sql_error_timestamp".into(),
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
                        "last_sql_error_timestamp",
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

            if event.has_value("sql.metrics.string.last_error") {
                event.rename(
                    "sql.metrics.string.last_error",
                    "mysql.replica_status.last_error.message",
                )?;
            }

            if event.has_value("sql.metrics.numeric.last_errno") {
                event.rename(
                    "sql.metrics.numeric.last_errno",
                    "mysql.replica_status.last_error.number",
                )?;
            }

            if event.has_value("sql.metrics.string.last_sql_error") {
                event.rename(
                    "sql.metrics.string.last_sql_error",
                    "mysql.replica_status.last_error.sql.message",
                )?;
            }

            if event.has_value("sql.metrics.numeric.last_sql_errno") {
                event.rename(
                    "sql.metrics.numeric.last_sql_errno",
                    "mysql.replica_status.last_error.sql.number",
                )?;
            }

            if event.has_value("sql.metrics.string.relay_log_file") {
                event.rename(
                    "sql.metrics.string.relay_log_file",
                    "mysql.replica_status.relay.log_file",
                )?;
            }

            if event.has_value("sql.metrics.numeric.relay_log_pos") {
                event.rename(
                    "sql.metrics.numeric.relay_log_pos",
                    "mysql.replica_status.relay.log_position",
                )?;
            }

            if event.has_value("sql.metrics.numeric.relay_log_space") {
                event.rename(
                    "sql.metrics.numeric.relay_log_space",
                    "mysql.replica_status.relay.log_space",
                )?;
            }

            if event.has_value("sql.metrics.string.replica_io_state") {
                event.rename(
                    "sql.metrics.string.replica_io_state",
                    "mysql.replica_status.replica.io.state",
                )?;
            }

            if event.has_value("sql.metrics.string.replica_sql_running_state") {
                event.rename(
                    "sql.metrics.string.replica_sql_running_state",
                    "mysql.replica_status.replica.sql.running_state",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_rewrite_db") {
                event.rename(
                    "sql.metrics.string.replicate_rewrite_db",
                    "mysql.replica_status.replicate.rewrite_db",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_do_db") {
                event.rename(
                    "sql.metrics.string.replicate_do_db",
                    "mysql.replica_status.replicate.do_db",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_do_table") {
                event.rename(
                    "sql.metrics.string.replicate_do_table",
                    "mysql.replica_status.replicate.do_table",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_ignore_db") {
                event.rename(
                    "sql.metrics.string.replicate_ignore_db",
                    "mysql.replica_status.replicate.ignore.do_db",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_ignore_server_ids") {
                event.rename(
                    "sql.metrics.string.replicate_ignore_server_ids",
                    "mysql.replica_status.replicate.ignore.server_id",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_ignore_table") {
                event.rename(
                    "sql.metrics.string.replicate_ignore_table",
                    "mysql.replica_status.replicate.ignore.table",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_wild_ignore_table") {
                event.rename(
                    "sql.metrics.string.replicate_wild_ignore_table",
                    "mysql.replica_status.replicate.ignore.wild_table",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_wild_do_table") {
                event.rename(
                    "sql.metrics.string.replicate_wild_do_table",
                    "mysql.replica_status.replicate.wild_do_table",
                )?;
            }

            if event.has_value("sql.metrics.numeric.seconds_behind_source") {
                event.rename(
                    "sql.metrics.numeric.seconds_behind_source",
                    "mysql.replica_status.seconds_behind_source",
                )?;
            }

            if event.has_value("sql.metrics.string.source_log_file") {
                event.rename(
                    "sql.metrics.string.source_log_file",
                    "mysql.replica_status.source.binary_log_file",
                )?;
            }

            if event.has_value("sql.metrics.string.source_bind") {
                event.rename(
                    "sql.metrics.string.source_bind",
                    "mysql.replica_status.source.bind.interface.name",
                )?;
            }

            if event.has_value("sql.metrics.string.source_info_file") {
                event.rename(
                    "sql.metrics.string.source_info_file",
                    "mysql.replica_status.source.file_info",
                )?;
            }

            if event.has_value("sql.metrics.string.source_host") {
                event.rename(
                    "sql.metrics.string.source_host",
                    "mysql.replica_status.source.host.name",
                )?;
            }

            if event.has_value("sql.metrics.numeric.get_source_public_key") {
                event.rename(
                    "sql.metrics.numeric.get_source_public_key",
                    "mysql.replica_status.source.is_get_public_key",
                )?;
            }

            if event.has_value("sql.metrics.string.relay_source_log_file") {
                event.rename(
                    "sql.metrics.string.relay_source_log_file",
                    "mysql.replica_status.source.log_file.relay",
                )?;
            }

            if event.has_value("sql.metrics.numeric.exec_source_log_pos") {
                event.rename(
                    "sql.metrics.numeric.exec_source_log_pos",
                    "mysql.replica_status.source.log_position.exec",
                )?;
            }

            if event.has_value("sql.metrics.numeric.read_source_log_pos") {
                event.rename(
                    "sql.metrics.numeric.read_source_log_pos",
                    "mysql.replica_status.source.log_position.read",
                )?;
            }

            if event.has_value("sql.metrics.string.source_public_key_path") {
                event.rename(
                    "sql.metrics.string.source_public_key_path",
                    "mysql.replica_status.source.public_key_path",
                )?;
            }

            if event.has_value("sql.metrics.numeric.source_retry_count") {
                event.rename(
                    "sql.metrics.numeric.source_retry_count",
                    "mysql.replica_status.source.retry_count",
                )?;
            }

            if event.has_value("sql.metrics.numeric.source_server_id") {
                event.rename(
                    "sql.metrics.numeric.source_server_id",
                    "mysql.replica_status.source.server.id",
                )?;
            }

            if event.has_value("sql.metrics.string.source_uuid") {
                event.rename(
                    "sql.metrics.string.source_uuid",
                    "mysql.replica_status.source.server.uuid",
                )?;
            }

            if event.has_value("sql.metrics.string.source_ssl_allowed") {
                event.rename(
                    "sql.metrics.string.source_ssl_allowed",
                    "mysql.replica_status.source.ssl.allowed",
                )?;
            }

            if event.has_value("sql.metrics.string.source_ssl_verify_server_cert") {
                event.rename(
                    "sql.metrics.string.source_ssl_verify_server_cert",
                    "mysql.replica_status.source.ssl.is_verify_server_cert",
                )?;
            }

            if event.has_value("sql.metrics.string.source_ssl_ca_file") {
                event.rename(
                    "sql.metrics.string.source_ssl_ca_file",
                    "mysql.replica_status.source.ssl.ca_file",
                )?;
            }

            if event.has_value("sql.metrics.string.source_ssl_ca_path") {
                event.rename(
                    "sql.metrics.string.source_ssl_ca_path",
                    "mysql.replica_status.source.ssl.ca_path",
                )?;
            }

            if event.has_value("sql.metrics.string.source_ssl_cert") {
                event.rename(
                    "sql.metrics.string.source_ssl_cert",
                    "mysql.replica_status.source.ssl.cert",
                )?;
            }

            if event.has_value("sql.metrics.string.source_ssl_cipher") {
                event.rename(
                    "sql.metrics.string.source_ssl_cipher",
                    "mysql.replica_status.source.ssl.cipher",
                )?;
            }

            if event.has_value("sql.metrics.string.source_ssl_crl") {
                event.rename(
                    "sql.metrics.string.source_ssl_crl",
                    "mysql.replica_status.source.ssl.crl",
                )?;
            }

            if event.has_value("sql.metrics.string.source_ssl_crlpath") {
                event.rename(
                    "sql.metrics.string.source_ssl_crlpath",
                    "mysql.replica_status.source.ssl.crl_path",
                )?;
            }

            if event.has_value("sql.metrics.string.source_ssl_key") {
                event.rename(
                    "sql.metrics.string.source_ssl_key",
                    "mysql.replica_status.source.ssl.key",
                )?;
            }

            if event.has_value("sql.metrics.string.source_tls_version") {
                event.rename(
                    "sql.metrics.string.source_tls_version",
                    "mysql.replica_status.source.tls_version",
                )?;
            }

            if event.has_value("sql.metrics.numeric.sql_delay") {
                event.rename(
                    "sql.metrics.numeric.sql_delay",
                    "mysql.replica_status.thread.sql.delay.sec",
                )?;
            }

            if event.has_value("sql.metrics.string.until_condition") {
                event.rename(
                    "sql.metrics.string.until_condition",
                    "mysql.replica_status.until.condition",
                )?;
            }

            if event.has_value("sql.metrics.numeric.sql_remaining_delay") {
                event.rename(
                    "sql.metrics.numeric.sql_remaining_delay",
                    "mysql.replica_status.thread.sql.delay_remaining.sec",
                )?;
            }

            if event.has_value("sql.metrics.numeric.until_log_pos") {
                event.rename(
                    "sql.metrics.numeric.until_log_pos",
                    "mysql.replica_status.until.log_position",
                )?;
            }

            if event.has_value("sql.metrics.string.until_log_file") {
                event.rename(
                    "sql.metrics.string.until_log_file",
                    "mysql.replica_status.until.log_file",
                )?;
            }

            if event.has_value("sql.metrics.string.using_gtid") {
                event.rename(
                    "sql.metrics.string.using_gtid",
                    "mysql.replica_status.is_gtid_using",
                )?;
            }

            if event.has_value("sql.metrics.numeric.gtid_io_pos") {
                event.rename(
                    "sql.metrics.numeric.gtid_io_pos",
                    "mysql.replica_status.gtid_io_position",
                )?;
            }

            if event.has_value("sql.metrics.string.parallel_mode") {
                event.rename(
                    "sql.metrics.string.parallel_mode",
                    "mysql.replica_status.parallel_mode",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_do_domain_ids") {
                event.rename(
                    "sql.metrics.string.replicate_do_domain_ids",
                    "mysql.replica_status.replicate_do_domain_ids",
                )?;
            }

            if event.has_value("sql.metrics.string.replicate_ignore_domain_ids") {
                event.rename(
                    "sql.metrics.string.replicate_ignore_domain_ids",
                    "mysql.replica_status.replicate_ignore_domain_ids",
                )?;
            }

            if event.has_value("sql.metrics.numeric.slave_ddl_groups") {
                event.rename(
                    "sql.metrics.numeric.slave_ddl_groups",
                    "mysql.replica_status.replica.ddl_groups",
                )?;
            }

            if event.has_value("sql.metrics.numeric.slave_non_transactional_groups") {
                event.rename(
                    "sql.metrics.numeric.slave_non_transactional_groups",
                    "mysql.replica_status.replica.non_transactional_groups",
                )?;
            }

            if event.has_value("sql.metrics.numeric.slave_transactional_groups") {
                event.rename(
                    "sql.metrics.numeric.slave_transactional_groups",
                    "mysql.replica_status.replica.transactional_groups",
                )?;
            }

            let v = json!(
                event
                    .get("sql.metrics.string.master_user")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("user.name", v)?;
            }

            let _cond = { event.get_i64("sql.metrics.numeric.get_master_public_key") == Some(1) };
            if _cond {
                let v = json!(true);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.numeric.get_master_public_key", v)?;
                }
            }

            let _cond = { event.get_i64("sql.metrics.numeric.get_master_public_key") == Some(0) };
            if _cond {
                let v = json!(false);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.numeric.get_master_public_key", v)?;
                }
            }

            let _cond = {
                event.get_str("sql.metrics.string.master_ssl_verify_server_cert") == Some("Yes")
            };
            if _cond {
                let v = json!(true);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.string.master_ssl_verify_server_cert", v)?;
                }
            }

            let _cond =
                { event.get_str("sql.metrics.string.master_ssl_verify_server_cert") == Some("No") };
            if _cond {
                let v = json!(false);
                if !painless_is_empty_value(&v) {
                    event.set("sql.metrics.string.master_ssl_verify_server_cert", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sql.metrics.numeric.master_port") {
                    if let Some(val) = event.get("sql.metrics.numeric.master_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "sql.metrics.numeric.master_port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if event.has_value("sql.metrics.string.slave_io_running") {
                event.rename(
                    "sql.metrics.string.slave_io_running",
                    "mysql.replica_status.is_io_thread_running",
                )?;
            }

            if event.has_value("sql.metrics.string.slave_sql_running") {
                event.rename(
                    "sql.metrics.string.slave_sql_running",
                    "mysql.replica_status.is_sql_thread_running",
                )?;
            }

            if event.has_value("sql.metrics.string.slave_io_state") {
                event.rename(
                    "sql.metrics.string.slave_io_state",
                    "mysql.replica_status.replica.io.state",
                )?;
            }

            if event.has_value("sql.metrics.string.slave_sql_running_state") {
                event.rename(
                    "sql.metrics.string.slave_sql_running_state",
                    "mysql.replica_status.replica.sql.running_state",
                )?;
            }

            if event.has_value("sql.metrics.numeric.seconds_behind_master") {
                event.rename(
                    "sql.metrics.numeric.seconds_behind_master",
                    "mysql.replica_status.seconds_behind_source",
                )?;
            }

            if event.has_value("sql.metrics.string.master_log_file") {
                event.rename(
                    "sql.metrics.string.master_log_file",
                    "mysql.replica_status.source.binary_log_file",
                )?;
            }

            if event.has_value("sql.metrics.string.master_bind") {
                event.rename(
                    "sql.metrics.string.master_bind",
                    "mysql.replica_status.source.bind.interface.name",
                )?;
            }

            if event.has_value("sql.metrics.string.master_info_file") {
                event.rename(
                    "sql.metrics.string.master_info_file",
                    "mysql.replica_status.source.file_info",
                )?;
            }

            if event.has_value("sql.metrics.string.master_host") {
                event.rename(
                    "sql.metrics.string.master_host",
                    "mysql.replica_status.source.host.name",
                )?;
            }

            if event.has_value("sql.metrics.numeric.get_master_public_key") {
                event.rename(
                    "sql.metrics.numeric.get_master_public_key",
                    "mysql.replica_status.source.is_get_public_key",
                )?;
            }

            if event.has_value("sql.metrics.string.relay_master_log_file") {
                event.rename(
                    "sql.metrics.string.relay_master_log_file",
                    "mysql.replica_status.source.log_file.relay",
                )?;
            }

            if event.has_value("sql.metrics.numeric.exec_master_log_pos") {
                event.rename(
                    "sql.metrics.numeric.exec_master_log_pos",
                    "mysql.replica_status.source.log_position.exec",
                )?;
            }

            if event.has_value("sql.metrics.numeric.read_master_log_pos") {
                event.rename(
                    "sql.metrics.numeric.read_master_log_pos",
                    "mysql.replica_status.source.log_position.read",
                )?;
            }

            if event.has_value("sql.metrics.string.master_public_key_path") {
                event.rename(
                    "sql.metrics.string.master_public_key_path",
                    "mysql.replica_status.source.public_key_path",
                )?;
            }

            if event.has_value("sql.metrics.numeric.master_retry_count") {
                event.rename(
                    "sql.metrics.numeric.master_retry_count",
                    "mysql.replica_status.source.retry_count",
                )?;
            }

            if event.has_value("sql.metrics.numeric.master_server_id") {
                event.rename(
                    "sql.metrics.numeric.master_server_id",
                    "mysql.replica_status.source.server.id",
                )?;
            }

            if event.has_value("sql.metrics.string.master_uuid") {
                event.rename(
                    "sql.metrics.string.master_uuid",
                    "mysql.replica_status.source.server.uuid",
                )?;
            }

            if event.has_value("sql.metrics.string.master_ssl_allowed") {
                event.rename(
                    "sql.metrics.string.master_ssl_allowed",
                    "mysql.replica_status.source.ssl.allowed",
                )?;
            }

            if event.has_value("sql.metrics.string.master_ssl_ca_file") {
                event.rename(
                    "sql.metrics.string.master_ssl_ca_file",
                    "mysql.replica_status.source.ssl.ca_file",
                )?;
            }

            if event.has_value("sql.metrics.string.master_ssl_ca_path") {
                event.rename(
                    "sql.metrics.string.master_ssl_ca_path",
                    "mysql.replica_status.source.ssl.ca_path",
                )?;
            }

            if event.has_value("sql.metrics.string.master_ssl_cert") {
                event.rename(
                    "sql.metrics.string.master_ssl_cert",
                    "mysql.replica_status.source.ssl.cert",
                )?;
            }

            if event.has_value("sql.metrics.string.master_ssl_cipher") {
                event.rename(
                    "sql.metrics.string.master_ssl_cipher",
                    "mysql.replica_status.source.ssl.cipher",
                )?;
            }

            if event.has_value("sql.metrics.string.master_ssl_crl") {
                event.rename(
                    "sql.metrics.string.master_ssl_crl",
                    "mysql.replica_status.source.ssl.crl",
                )?;
            }

            if event.has_value("sql.metrics.string.master_ssl_crlpath") {
                event.rename(
                    "sql.metrics.string.master_ssl_crlpath",
                    "mysql.replica_status.source.ssl.crl_path",
                )?;
            }

            if event.has_value("sql.metrics.string.master_ssl_key") {
                event.rename(
                    "sql.metrics.string.master_ssl_key",
                    "mysql.replica_status.source.ssl.key",
                )?;
            }

            if event.has_value("sql.metrics.string.master_tls_version") {
                event.rename(
                    "sql.metrics.string.master_tls_version",
                    "mysql.replica_status.source.tls_version",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("sql.metrics.string.master_ssl_verify_server_cert") {
                    event.rename(
                        "sql.metrics.string.master_ssl_verify_server_cert",
                        "mysql.replica_status.source.ssl.is_verify_server_cert",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "rename")?;
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

            if event.remove("sql").is_none() {
                return Err(TransformError::FieldNotFound { path: "sql".into() });
            }

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == \"\" || o == \"NULL\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == \"\" || o == \"NULL\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.append_unique("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
