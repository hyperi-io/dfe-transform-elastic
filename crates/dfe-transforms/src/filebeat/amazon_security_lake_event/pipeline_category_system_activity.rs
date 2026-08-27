// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_category_system_activity` pipeline.
pub struct PipelineCategorySystemActivity;

impl Transform for PipelineCategorySystemActivity {
    fn name(&self) -> &str {
        "pipeline_category_system_activity"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("ocsf.access_mask") {
            if let Some(val) = event.get("ocsf.access_mask") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.access_mask".into(),
                        message,
                    })?;
                event.set("ocsf.access_mask", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_access_mask_to_long")?;
                    event.remove("ocsf.access_mask");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if event.has_value("ocsf.disposition_id") {
            if let Some(val) = event.get("ocsf.disposition_id") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.disposition_id".into(),
                        message,
                    })?;
                event.set("ocsf.disposition_id", converted)?;
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("ocsf.kernel.is_system") {
            if let Some(val) = event.get("ocsf.kernel.is_system") {
                let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.kernel.is_system".into(),
                        message,
                    })?;
                event.set("ocsf.kernel.is_system", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_kernel_is_system_to_boolean")?;
                    event.remove("ocsf.kernel.is_system");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if event.has_value("ocsf.kernel.type_id") {
            if let Some(val) = event.get("ocsf.kernel.type_id") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.kernel.type_id".into(),
                        message,
                    })?;
                event.set("ocsf.kernel.type_id", converted)?;
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("ocsf.actual_permissions") {
            if let Some(val) = event.get("ocsf.actual_permissions") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.actual_permissions".into(),
                        message,
                    })?;
                event.set("ocsf.actual_permissions", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_actual_permissions_to_long")?;
                    event.remove("ocsf.actual_permissions");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("ocsf.requested_permissions") {
            if let Some(val) = event.get("ocsf.requested_permissions") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.requested_permissions".into(),
                        message,
                    })?;
                event.set("ocsf.requested_permissions", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_requested_permissions_to_long")?;
                    event.remove("ocsf.requested_permissions");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if event.has_value("ocsf.module.load_type_id") {
            if let Some(val) = event.get("ocsf.module.load_type_id") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.module.load_type_id".into(),
                        message,
                    })?;
                event.set("ocsf.module.load_type_id", converted)?;
            }
        }

        let _cond = { event.has_value("ocsf.job.created_time") && event.get_str("ocsf.job.created_time") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("ocsf.job.created_time") {
                match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                    Some(parsed) => event.set("ocsf.job.created_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "ocsf.job.created_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_job_created_time")?;
                    event.remove("ocsf.job.created_time");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("ocsf.job.created_time_dt") && event.get_str("ocsf.job.created_time_dt") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("ocsf.job.created_time_dt") {
                match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                    Some(parsed) => event.set("ocsf.job.created_time_dt", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "ocsf.job.created_time_dt".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_job_created_time_dt")?;
                    event.remove("ocsf.job.created_time_dt");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("ocsf.job.last_run_time") && event.get_str("ocsf.job.last_run_time") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("ocsf.job.last_run_time") {
                match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                    Some(parsed) => event.set("ocsf.job.last_run_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "ocsf.job.last_run_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_job_last_run_time")?;
                    event.remove("ocsf.job.last_run_time");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("ocsf.job.last_run_time_dt") && event.get_str("ocsf.job.last_run_time_dt") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("ocsf.job.last_run_time_dt") {
                match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                    Some(parsed) => event.set("ocsf.job.last_run_time_dt", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "ocsf.job.last_run_time_dt".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_job_last_run_time_dt")?;
                    event.remove("ocsf.job.last_run_time_dt");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("ocsf.job.next_run_time") && event.get_str("ocsf.job.next_run_time") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("ocsf.job.next_run_time") {
                match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                    Some(parsed) => event.set("ocsf.job.next_run_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "ocsf.job.next_run_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_job_next_run_time")?;
                    event.remove("ocsf.job.next_run_time");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("ocsf.job.next_run_time_dt") && event.get_str("ocsf.job.next_run_time_dt") != Some("") };
        if _cond {
        // on_failure: 2 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("ocsf.job.next_run_time_dt") {
                match parse_date_out(&date_str, &["ISO8601", "UNIX_MS", "yyyy-MM-dd HH:mm:ss[.SSSSSSSSS][.SSSSSSSS][.SSSSSSS][.SSSSSS][.SSSSS][.SSSS][.SSS][.SS][.S]X"], None, None) {
                    Some(parsed) => event.set("ocsf.job.next_run_time_dt", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "ocsf.job.next_run_time_dt".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_job_next_run_time_dt")?;
                    event.remove("ocsf.job.next_run_time_dt");
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("ocsf.job.user.domain").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("user.target.domain", v)?;
        }

        if let Some(v) = event.get("ocsf.job.user.email_addr").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("user.target.email", v)?;
        }

        let _cond = { event.has_value("ocsf.job.user.email_addr") };
        if _cond {
            event.append_unique("related.user", json!(event.get("ocsf.job.user.email_addr").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("ocsf.job.user.full_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("user.target.full_name", v)?;
        }

        let _cond = { event.has_value("ocsf.job.user.full_name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("ocsf.job.user.full_name").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.get("ocsf.job.user.groups").is_some_and(|v| v.is_array()) };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            foreach_array(event, "ocsf.job.user.groups", |event| {
                event.append_unique("user.target.group.id", json!(event.get("_ingest._value.uid").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
            Ok(())
        })();
        }

        let _cond = { event.get("ocsf.job.user.groups").is_some_and(|v| v.is_array()) };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            foreach_array(event, "ocsf.job.user.groups", |event| {
                event.append_unique("user.target.group.name", json!(event.get("_ingest._value.name").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
            Ok(())
        })();
        }

        if let Some(v) = event.get("ocsf.job.user.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("user.target.id", v)?;
        }

        let _cond = { event.has_value("ocsf.job.user.uid") };
        if _cond {
            event.append_unique("related.user", json!(event.get("ocsf.job.user.uid").map_or_else(String::new, template_to_string)))?;
        }

        if let Some(v) = event.get("ocsf.job.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("user.target.name", v)?;
        }

        let _cond = { event.has_value("ocsf.job.user.name") };
        if _cond {
            event.append_unique("related.user", json!(event.get("ocsf.job.user.name").map_or_else(String::new, template_to_string)))?;
        }

        if event.has_value("ocsf.job.run_state_id") {
            if let Some(val) = event.get("ocsf.job.run_state_id") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.job.run_state_id".into(),
                        message,
                    })?;
                event.set("ocsf.job.run_state_id", converted)?;
            }
        }

        if event.has_value("ocsf.exit_code") {
            if let Some(val) = event.get("ocsf.exit_code") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.exit_code".into(),
                        message,
                    })?;
                event.set("ocsf.exit_code", converted)?;
            }
        }

        if let Some(v) = event.get("ocsf.exit_code").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("process.exit_code", v)?;
        }

        if event.has_value("ocsf.injection_type_id") {
            if let Some(val) = event.get("ocsf.injection_type_id") {
                let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                        path: "ocsf.injection_type_id".into(),
                        message,
                    })?;
                event.set("ocsf.injection_type_id", converted)?;
            }
        }

        Ok(TransformResult::Continue)
    }
}
