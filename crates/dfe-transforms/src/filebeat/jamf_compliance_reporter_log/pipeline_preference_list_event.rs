// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_preference_list_event` pipeline.
pub struct PipelinePreferenceListEvent;

impl Transform for PipelinePreferenceListEvent {
    fn name(&self) -> &str {
        "pipeline_preference_list_event"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("json.event_attributes.AuditEventExcludedProcesses") {
                    event.rename("json.event_attributes.AuditEventExcludedProcesses", "jamf_compliance_reporter.log.event_attributes.audit_event.excluded_processes")?;
                }

                if event.has_value("json.event_attributes.AuditEventExcludedUsers") {
                    event.rename("json.event_attributes.AuditEventExcludedUsers", "jamf_compliance_reporter.log.event_attributes.audit_event.excluded_users")?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
            if event.has_value("json.event_attributes.AuditEventLogVerboseMessages") {
                if let Some(val) = event.get("json.event_attributes.AuditEventLogVerboseMessages") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.AuditEventLogVerboseMessages".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.audit_event_log_verbose_messages", converted)?;
                }
            }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.event_attributes.AuditLevel") {
                if let Some(val) = event.get("json.event_attributes.AuditLevel") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.AuditLevel".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.audit_level", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.event_attributes.FileEventExclusionPaths") {
                    event.rename("json.event_attributes.FileEventExclusionPaths", "jamf_compliance_reporter.log.event_attributes.file_event.exclusion_paths")?;
                }

                if event.has_value("json.event_attributes.FileEventInclusionPaths") {
                    event.rename("json.event_attributes.FileEventInclusionPaths", "jamf_compliance_reporter.log.event_attributes.file_event.inclusion_paths")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.event_attributes.FileEventUseFuzzyMatch") {
                if let Some(val) = event.get("json.event_attributes.FileEventUseFuzzyMatch") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.FileEventUseFuzzyMatch".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.file_event.use_fuzzy_match", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.event_attributes.FileLicenseInfo.LicenseEmail") {
                    event.rename("json.event_attributes.FileLicenseInfo.LicenseEmail", "user.email")?;
                }

            let _cond = { event.has_value("user.email") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.user", json!(event.get("user.email").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.get_str("json.event_attributes.FileLicenseInfo.LicenseExpirationDate") != Some("0") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event_attributes.FileLicenseInfo.LicenseExpirationDate") {
                    match parse_date_out(&date_str, &["dd/MM/yyyy"], None, None) {
                        Some(parsed) => event.set("jamf_compliance_reporter.log.event_attributes.file_license_info.license_expiration_date", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event_attributes.FileLicenseInfo.LicenseExpirationDate".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.event_attributes.FileLicenseInfo.LicenseKey") {
                    event.rename("json.event_attributes.FileLicenseInfo.LicenseKey", "jamf_compliance_reporter.log.event_attributes.file_license_info.license_key")?;
                }

                if event.has_value("json.event_attributes.FileLicenseInfo.LicenseType") {
                    event.rename("json.event_attributes.FileLicenseInfo.LicenseType", "jamf_compliance_reporter.log.event_attributes.file_license_info.license_type")?;
                }

                if event.has_value("json.event_attributes.FileLicenseInfo.LicenseVersion") {
                    event.rename("json.event_attributes.FileLicenseInfo.LicenseVersion", "jamf_compliance_reporter.log.event_attributes.file_license_info.license_version")?;
                }

                if event.has_value("json.event_attributes.LogFileLocation") {
                    event.rename("json.event_attributes.LogFileLocation", "jamf_compliance_reporter.log.event_attributes.log.file.location")?;
                }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.event_attributes.LogFileMaxNumberBackups") {
                if let Some(val) = event.get("json.event_attributes.LogFileMaxNumberBackups") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.LogFileMaxNumberBackups".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.log.file.max_number_backups", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.event_attributes.LogFileMaxSizeMegaBytes") {
                if let Some(val) = event.get("json.event_attributes.LogFileMaxSizeMegaBytes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.event_attributes.LogFileMaxSizeMegaBytes".into(),
                            message,
                        })?;
                    event.set("jamf_compliance_reporter.log.event_attributes.log.file.max_size_mega_bytes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                if event.has_value("json.event_attributes.LogFileOwnership") {
                    event.rename("json.event_attributes.LogFileOwnership", "jamf_compliance_reporter.log.event_attributes.log.file.ownership")?;
                }

                if event.has_value("json.event_attributes.LogFilePermission") {
                    event.rename("json.event_attributes.LogFilePermission", "jamf_compliance_reporter.log.event_attributes.log.file.permission")?;
                }

                if event.has_value("json.event_attributes.LogRemoteEndpointEnabled") {
                    event.rename("json.event_attributes.LogRemoteEndpointEnabled", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_enabled")?;
                }

                if event.has_value("json.event_attributes.LogRemoteEndpointType") {
                    event.rename("json.event_attributes.LogRemoteEndpointType", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type")?;
                }

                if event.has_value("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.AccessKeyId") {
                    event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.AccessKeyId", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.access_key_id")?;
                }

                if event.has_value("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.Region") {
                    event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.Region", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.region")?;
                }

                if event.has_value("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.SecretKey") {
                    event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.SecretKey", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.secret_key")?;
                }

                if event.has_value("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.StreamName") {
                    event.rename("json.event_attributes.LogRemoteEndpointTypeAWSKinesis.StreamName", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_type_awskinesis.stream_name")?;
                }

                if event.has_value("json.event_attributes.LogRemoteEndpointURL") {
                    event.rename("json.event_attributes.LogRemoteEndpointURL", "jamf_compliance_reporter.log.event_attributes.log.remote_endpoint_url")?;
                }

                if event.has_value("json.event_attributes.UnifiedLogPredicates") {
                    event.rename("json.event_attributes.UnifiedLogPredicates", "jamf_compliance_reporter.log.event_attributes.unified_log_predicates")?;
                }

                if event.has_value("json.event_attributes.Version") {
                    event.rename("json.event_attributes.Version", "jamf_compliance_reporter.log.event_attributes.version")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
